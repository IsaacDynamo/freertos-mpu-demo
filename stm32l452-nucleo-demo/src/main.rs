#![no_main]
#![no_std]
#![warn(unsafe_op_in_unsafe_fn)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

mod wrapper;
use wrapper::*;

use cortex_m_rt::{entry, exception, ExceptionFrame};
use hal::gpio::{Output, Pin, PushPull, L8};
use panic_rtt_target as _;
use rtt_target::{rprint, rprintln};
use stm32l4xx_hal as hal;
use stm32l4xx_hal::prelude::*;
use critical_section::Mutex;
use core::cell::Cell;
use core::concat;
use core::default::Default;
use core::format_args;
use core::mem::MaybeUninit;
use core::ptr::null;
use freertos_bindgen::*;
use static_cell::StaticCell;

static LED: Mutex<Cell<Option<Pin<Output<PushPull>, L8, 'A', 5>>>> = Mutex::new(Cell::new(None));

static QUEUE: Mutex<Cell<Option<QueueHandle<Command>>>> = Mutex::new(Cell::new(None));

#[derive(Debug, Clone, Copy)]
enum Command {
    PutChar(u8),
    SetLed(bool),
}

#[entry]
fn entry() -> ! {
    rtt_target::rtt_init_print!();

    let p = hal::stm32::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let mut pwr = p.PWR.constrain(&mut rcc.apb1r1);

    let _clocks = rcc
        .cfgr
        .sysclk(80.MHz())
        .pclk1(80.MHz())
        .pclk2(80.MHz())
        .freeze(&mut flash.acr, &mut pwr);

    let mut gpioa = p.GPIOA.split(&mut rcc.ahb2);
    let led = gpioa.pa5.into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
    critical_section::with(|cs| {
        LED.borrow(cs).replace(Some(led))
    });

    extern "C" {
        static __FLASH_segment_start__: u32;
        static __FLASH_segment_end__: u32;
        static __SRAM_segment_start__: u32;
        static __SRAM_segment_end__: u32;
        static __privileged_functions_start__: u32;
        static __privileged_functions_end__: u32;
        static __privileged_data_start__: u32;
        static __privileged_data_end__: u32;
        static __syscalls_flash_start__: u32;
        static __syscalls_flash_end__: u32;

        static __sbss: u32;
        static __ebss: u32;
        static __sdata: u32;
        static __edata: u32;
        static __sidata: u32;
    }

    rprintln!("flash {:08x?}-{:08x?}", unsafe { &__FLASH_segment_start__ as *const u32 }, unsafe { &__FLASH_segment_end__ as *const u32 });
    rprintln!("sram  {:08x?}-{:08x?}", unsafe { &__SRAM_segment_start__ as *const u32 }, unsafe { &__SRAM_segment_end__ as *const u32 });
    rprintln!("func  {:08x?}-{:08x?}", unsafe { &__privileged_functions_start__ as *const u32 }, unsafe { &__privileged_functions_end__ as *const u32 });
    rprintln!("data  {:08x?}-{:08x?}", unsafe { &__privileged_data_start__ as *const u32 }, unsafe { &__privileged_data_end__ as *const u32 });
    rprintln!("scall {:08x?}-{:08x?}", unsafe { &__syscalls_flash_start__ as *const u32 }, unsafe { &__syscalls_flash_end__ as *const u32 });

    rprintln!("bss   {:08x?}-{:08x?}", unsafe { &__sbss as *const u32 }, unsafe { &__ebss as *const u32 });
    rprintln!("data  {:08x?}-{:08x?}", unsafe { &__sdata as *const u32 }, unsafe { &__edata as *const u32 });
    rprintln!("idata {:08x?}", unsafe { &__sidata as *const u32 });

    fn idletaskmem() -> (u32, u32, u32) {
        let mut task = core::ptr::null_mut::<freertos_bindgen::StaticTask_t>();
        let mut stack = core::ptr::null_mut::<freertos_bindgen::StackType_t>();
        let mut size = 0;
        unsafe { vApplicationGetIdleTaskMemory(&mut task, &mut stack, &mut size) };
        (task as u32, stack as u32, size)
    }

    fn timertaskmem() -> (u32, u32, u32) {
        let mut task = core::ptr::null_mut::<freertos_bindgen::StaticTask_t>();
        let mut stack = core::ptr::null_mut::<freertos_bindgen::StackType_t>();
        let mut size = 0;
        unsafe { vApplicationGetTimerTaskMemory(&mut task, &mut stack, &mut size) };
        (task as u32, stack as u32, size)
    }

    rprintln!("idle  {:x?}", idletaskmem());
    rprintln!("timer {:x?}", timertaskmem());
    rprintln!("main:");

    #[link_section = "privileged_data"]
    static mut tickTaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut tickTaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    #[link_section = "privileged_data"]
    static mut exampleTaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut exampleTaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    #[link_section = "privileged_data"]
    static mut queue_static_storage: StaticQueue_t =
        unsafe { MaybeUninit::zeroed().assume_init() };

    static QUEUE_BUFFER: StaticCell<[Command; 10]> = StaticCell::new();
    let queue_buffer = QUEUE_BUFFER.init_with(|| [Command::PutChar(0); 10]);

    let queue = queue_create_static(unsafe{&mut queue_static_storage}, queue_buffer).unwrap();
    critical_section::with(|cs| {
        QUEUE.borrow(cs).set(Some(queue));
    });

    let privileged_task = unsafe{xTaskCreateStatic(
        Some(privileged_fn),
        b"privileged_task\0".as_ptr().cast(),
        configMINIMAL_STACK_SIZE,
        core::ptr::null_mut(),
        gen::configMAX_PRIORITIES - 1,
        &mut exampleTaskStack.0[0],
        &mut exampleTaskTCB,
    )};
    assert!(!privileged_task.is_null());

    let def = TaskParameters_t {
        pvTaskCode: Some(unprivileged_fn),
        pcName: b"unprivileged_task\0".as_ptr().cast(),
        usStackDepth: configMINIMAL_STACK_SIZE as usize,
        pvParameters: unsafe{core::mem::transmute(queue)},
        uxPriority: 1,
        puxStackBuffer: unsafe{&mut tickTaskStack.0[0]},
        xRegions: [MemoryRegion_t::default(); 3],
        pxTaskBuffer: unsafe{&mut tickTaskTCB},
    };

    let mut unprivileged_task: TaskHandle_t = unsafe{MaybeUninit::zeroed().assume_init()};
    let rc = unsafe{xTaskCreateRestrictedStatic(&def, &mut unprivileged_task)};
    assert!(rc == pdPASS);

    queue.grant_access(privileged_task);
    queue.grant_access(unprivileged_task);

    start_scheduler();
    loop {}
}

unsafe extern "C" fn unprivileged_fn(arg: *mut core::ffi::c_void) {
    // TODO: add some decent way to inject arguments.
    // Side channel via QUEUE doesn't work because QUEUE is not in this threads accessible memory
    // let queue = critical_section::with(|cs| QUEUE.borrow(cs).get()).unwrap();
    let queue: QueueHandle<Command> = unsafe{core::mem::transmute(arg)};

    loop {
        delay(1000);
        queue.send(Command::SetLed(true)).unwrap();
        for b in "Hello world!\n".bytes() {
            queue.send(Command::PutChar(b)).unwrap();
        }
        queue.send(Command::SetLed(false)).unwrap();
    }
}

unsafe extern "C" fn privileged_fn(_arg: *mut ::core::ffi::c_void) {
    let (mut led, queue) = critical_section::with(|cs| {(
        LED.borrow(cs).take().unwrap(),
        QUEUE.borrow(cs).get().unwrap()
    )});

    loop {
        let cmd = queue.receive().unwrap();
        match cmd {
            Command::PutChar(b) => {
                let b = [b];
                let c = core::str::from_utf8(&b).unwrap();
                rprint!("{}", c);
            },
            Command::SetLed(false) => led.set_low(),
            Command::SetLed(true) => led.set_high(),
        }
    }
}

// Bring symbols in from FreeRTOS, so the will populate the interrupt vector table.
#[link(name = "freertos_kernel_port")]
extern "C" {
    pub fn SVCall();
    pub fn PendSV();
    pub fn SysTick();
}

#[repr(C, align(512))]
struct MinStack([StackType_t; configMINIMAL_STACK_SIZE as usize]);

#[no_mangle]
pub unsafe extern "C" fn vApplicationStackOverflowHook(
    _xTask: TaskHandle_t,
    _pcTaskName: *const core::ffi::c_char,
) {
    panic!("vApplicationStackOverflowHook");
}

#[no_mangle]
pub unsafe extern "C" fn vApplicationGetIdleTaskMemory(
    ppxIdleTaskTCBBuffer: *mut *mut StaticTask_t,
    ppxIdleTaskStackBuffer: *mut *mut StackType_t,
    pulIdleTaskStackSize: *mut u32,
) {
    #[link_section = "privileged_data"]
    static mut TaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut TaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    unsafe {
        *ppxIdleTaskTCBBuffer = &mut TaskTCB as *mut StaticTask_t;
        *ppxIdleTaskStackBuffer = &mut TaskStack.0[0] as *mut StackType_t;
        *pulIdleTaskStackSize = configMINIMAL_STACK_SIZE;
    }
}

#[no_mangle]
pub unsafe extern "C" fn vApplicationGetTimerTaskMemory(
    ppxTimerTaskTCBBuffer: *mut *mut StaticTask_t,
    ppxTimerTaskStackBuffer: *mut *mut StackType_t,
    pulTimerTaskStackSize: *mut u32,
) {
    #[link_section = "privileged_data"]
    static mut TaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut TaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    unsafe {
        *ppxTimerTaskTCBBuffer = &mut TaskTCB as *mut StaticTask_t;
        *ppxTimerTaskStackBuffer = &mut TaskStack.0[0] as *mut StackType_t;
        *pulTimerTaskStackSize = configMINIMAL_STACK_SIZE;
    }
}

#[exception]
unsafe fn HardFault(frame: &ExceptionFrame) -> ! {
    rprintln!("HardFault");
    rprintln!("{:?}", frame);
    loop {}
}

#[exception]
unsafe fn MemoryManagement() -> ! {
    rprintln!("MemoryManagement");
    loop {}
}

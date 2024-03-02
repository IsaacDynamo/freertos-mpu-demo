#![no_main]
#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use cortex_m_rt::{entry, exception, ExceptionFrame};
use stm32l4xx_hal as hal;
use stm32l4xx_hal::prelude::*;

use cortex_m_semihosting::hprintln;
use core::format_args;
use core::concat;
use core::mem::MaybeUninit;
use core::ptr::null;
use core::default::Default;

use panic_semihosting as _;

use freertos_bindgen::gen::QueueHandle_t;
use freertos_bindgen::*;

#[entry]
fn entry() -> ! {
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

    hprintln!("flash {:08x?}-{:08x?}", unsafe { &__FLASH_segment_start__ as *const u32 }, unsafe { &__FLASH_segment_end__ as *const u32 });
    hprintln!("sram  {:08x?}-{:08x?}", unsafe { &__SRAM_segment_start__ as *const u32 }, unsafe { &__SRAM_segment_end__ as *const u32 });
    hprintln!("func  {:08x?}-{:08x?}", unsafe { &__privileged_functions_start__ as *const u32 }, unsafe { &__privileged_functions_end__ as *const u32 });
    hprintln!("data  {:08x?}-{:08x?}", unsafe { &__privileged_data_start__ as *const u32 }, unsafe { &__privileged_data_end__ as *const u32 });
    hprintln!("scall {:08x?}-{:08x?}", unsafe { &__syscalls_flash_start__ as *const u32 }, unsafe { &__syscalls_flash_end__ as *const u32 });

    hprintln!("bss   {:08x?}-{:08x?}", unsafe { &__sbss as *const u32 }, unsafe { &__ebss as *const u32 });
    hprintln!("data  {:08x?}-{:08x?}", unsafe { &__sdata as *const u32 }, unsafe { &__edata as *const u32 });
    hprintln!("idata {:08x?}", unsafe { &__sidata as *const u32 });

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

    hprintln!("idle  {:x?}", idletaskmem());
    hprintln!("timer {:x?}", timertaskmem());
    unsafe {

        #[link_section = "privileged_data"]
        static mut xSemaphoreBuffer: StaticSemaphore_t = unsafe { MaybeUninit::zeroed().assume_init() };
        //static mut xSemaphoreBuffer2: StaticSemaphore_t = unsafe { MaybeUninit::zeroed().assume_init() };

        #[link_section = "privileged_data"]
        static mut tickTaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
        static mut tickTaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

        #[link_section = "privileged_data"]
        static mut exampleTaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
        static mut exampleTaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);


        let xSemaphore = xSemaphoreCreateBinaryStatic( &mut xSemaphoreBuffer );
        assert!(!xSemaphore.is_null());


        unsafe extern "C" fn pong(arg: *mut ::core::ffi::c_void) {
            let xSemaphore = arg as QueueHandle_t;
            loop {
                let rc = xSemaphoreTake( xSemaphore, 0xFFFFFFFF );
                assert!(rc == 1 /* pdPASS */);
                hprintln!("!");
            }
        }

        let task = xTaskCreateStatic( Some(pong), //exampleTask,
                        null(), // "example",
                        configMINIMAL_STACK_SIZE,
                        xSemaphore as *mut core::ffi::c_void,
                        gen::configMAX_PRIORITIES - 1,
                        &mut exampleTaskStack.0[0],
                        &mut exampleTaskTCB);

        assert!(!task.is_null());

        vGrantAccessToSemaphore(task, xSemaphore);


        unsafe extern "C" fn ping(arg: *mut core::ffi::c_void) {
            let xSemaphore = arg as QueueHandle_t;
            loop {
                vTaskDelay( 1000 ); /* delay in ticks */
                let rc = xSemaphoreGive( xSemaphore );
                assert!(rc == 1 /* pdPASS */);
                //hprintln!("?");
            }
        }

        let def = TaskParameters_t {
            pvTaskCode:  Some(ping), //tickTask,
            pcName: null(), // "tick",
            usStackDepth: configMINIMAL_STACK_SIZE as usize,
            pvParameters: xSemaphore as *mut core::ffi::c_void,
            uxPriority: 1,
            puxStackBuffer: &mut tickTaskStack.0[0],
            xRegions: [MemoryRegion_t::default(); 3],
            pxTaskBuffer: &mut tickTaskTCB,
        };

        let mut task2: TaskHandle_t = MaybeUninit::zeroed().assume_init();
        let rc = xTaskCreateRestrictedStatic(&def, &mut task2);

        assert!(rc == 1 /* pdPASS */);

        vGrantAccessToSemaphore(task2, xSemaphore);

        /* Start the scheduler. */
        vTaskStartScheduler();

    }

    loop { }
}

#[link(name = "freertos_kernel_port")]
extern "C" {
    pub fn SVCall();
    pub fn PendSV();
    pub fn SysTick();
}

#[repr(C, align(512))]
struct MinStack([StackType_t; configMINIMAL_STACK_SIZE as usize]);

#[no_mangle]
pub unsafe extern "C" fn vApplicationStackOverflowHook( _xTask: TaskHandle_t, _pcTaskName: *const core::ffi::c_char ) {
    panic!("vApplicationStackOverflowHook");
}

#[no_mangle]
pub unsafe extern "C" fn vApplicationGetIdleTaskMemory(
    ppxIdleTaskTCBBuffer: *mut *mut StaticTask_t,
    ppxIdleTaskStackBuffer: *mut *mut StackType_t,
    pulIdleTaskStackSize: *mut u32
){
    #[link_section = "privileged_data"]
    static mut TaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut TaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    *ppxIdleTaskTCBBuffer = &mut TaskTCB as *mut StaticTask_t;
    *ppxIdleTaskStackBuffer = &mut TaskStack.0[0] as *mut StackType_t;
    *pulIdleTaskStackSize = configMINIMAL_STACK_SIZE;
}

#[no_mangle]
pub unsafe extern "C" fn vApplicationGetTimerTaskMemory(
    ppxTimerTaskTCBBuffer: *mut *mut StaticTask_t,
    ppxTimerTaskStackBuffer: *mut *mut StackType_t,
    pulTimerTaskStackSize: *mut u32
){
    #[link_section = "privileged_data"]
    static mut TaskTCB: StaticTask_t = unsafe { MaybeUninit::zeroed().assume_init() };
    static mut TaskStack: MinStack = MinStack([0; configMINIMAL_STACK_SIZE as usize]);

    *ppxTimerTaskTCBBuffer = &mut TaskTCB as *mut StaticTask_t;
    *ppxTimerTaskStackBuffer = &mut TaskStack.0[0] as *mut StackType_t;
    *pulTimerTaskStackSize = configMINIMAL_STACK_SIZE;
}




#[exception]
unsafe fn HardFault(frame: &ExceptionFrame) -> ! {
    hprintln!("HardFault");
    hprintln!("{:?}", frame);
    loop {}
}

#[exception]
unsafe fn MemoryManagement() -> ! {
    hprintln!("MemoryManagement");
    loop {}
}

use freertos_bindgen::gen::QueueHandle_t;
use freertos_bindgen::*;

use core::marker::PhantomData;
use core::mem::{size_of, MaybeUninit};

#[derive(Debug, Clone, Copy)]
pub struct QueueHandle<T> {
    handle: QueueHandle_t,
    t: PhantomData<T>,
}

unsafe impl<T: Send> Send for QueueHandle<T> {}

fn as_void<T>(ptr: &T) -> *const core::ffi::c_void {
    ptr as *const T as *const core::ffi::c_void
}

fn as_void_mut<T>(ptr: &mut T) -> *mut core::ffi::c_void {
    ptr as *mut T as *mut core::ffi::c_void
}

pub fn queue_create_static<T, const N: usize>(
    static_storage: &'static mut StaticQueue_t,
    buffer: &'static mut [T; N],
) -> Option<QueueHandle<T>> {
    let queue = unsafe {
        xQueueCreateStatic(
            N as u32,
            size_of::<T>() as u32,
            buffer as *mut [T; N] as *mut u8,
            static_storage,
        )
    };
    (!queue.is_null()).then_some(QueueHandle {
        handle: queue,
        t: PhantomData,
    })
}

impl<T> QueueHandle<T> {
    pub fn receive(self) -> Option<T> {
        let mut item = MaybeUninit::<T>::uninit();
        let rc = unsafe { xQueueReceive(self.handle, as_void_mut(&mut item), 0xFFFFFFFF) };
        (rc == pdPASS).then(|| unsafe { item.assume_init() })
    }

    pub fn send(self, data: T) -> Option<()> {
        let rc = unsafe { xQueueSend(self.handle, as_void(&data), 0xFFFFFFFF) };
        (rc == pdPASS).then_some(())
    }

    pub fn grant_access(self, task: TaskHandle_t) {
        unsafe { vGrantAccessToQueue(task, self.handle) }
    }
}

pub fn start_scheduler() {
    unsafe { vTaskStartScheduler() };
}

pub fn delay(ticks: u32) {
    unsafe { vTaskDelay(ticks) };
}

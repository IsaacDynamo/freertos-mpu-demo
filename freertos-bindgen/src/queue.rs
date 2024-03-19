// Missing defines from queue.h
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(unused)]

use crate::gen::BaseType_t;
use crate::pdFALSE;
use crate::gen::*;
use core::ffi::c_void;

// For internal use only.
pub(crate) const queueSEND_TO_BACK: BaseType_t = 0;
pub(crate) const queueSEND_TO_FRONT: BaseType_t = 1;
pub(crate) const queueOVERWRITE: BaseType_t = 2;

// For internal use only.  These definitions *must* match those in queue.c.
pub(crate) const queueQUEUE_TYPE_BASE: u8 = 0;
pub(crate) const queueQUEUE_TYPE_SET: u8 = 0;
pub(crate) const queueQUEUE_TYPE_MUTEX: u8 = 1;
pub(crate) const queueQUEUE_TYPE_COUNTING_SEMAPHORE: u8 = 2;
pub(crate) const queueQUEUE_TYPE_BINARY_SEMAPHORE: u8 = 3;
pub(crate) const queueQUEUE_TYPE_RECURSIVE_MUTEX: u8 = 4;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION)]
#[inline]
pub unsafe fn xQueueCreate(uxQueueLength: UBaseType_t, uxItemSize: UBaseType_t) -> QueueHandle_t {
    xQueueGenericCreate(uxQueueLength, uxItemSize, queueQUEUE_TYPE_BASE)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION)]
#[inline]
pub unsafe fn xQueueCreateStatic(
    uxQueueLength: UBaseType_t,
    uxItemSize: UBaseType_t,
    pucQueueStorage: *mut u8,
    pxQueueBuffer: &mut StaticQueue_t,
) -> QueueHandle_t {
    xQueueGenericCreateStatic(
        uxQueueLength,
        uxItemSize,
        pucQueueStorage,
        pxQueueBuffer,
        queueQUEUE_TYPE_BASE,
    )
}

#[cfg(configSUPPORT_STATIC_ALLOCATION)]
#[inline]
pub unsafe fn xQueueGetStaticBuffers(
    xQueue: QueueHandle_t,
    ppucQueueStorage: *mut *mut u8,
    ppxStaticQueue: *mut *mut StaticQueue_t,
) {
    xQueueGenericGetStaticBuffers(xQueue, ppucQueueStorage, ppxStaticQueue);
}

#[inline]
pub unsafe fn xQueueSendToFront(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_FRONT)
}

#[inline]
pub unsafe fn xQueueSendToBack(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
}

#[inline]
pub unsafe fn xQueueSend(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, xTicksToWait, queueSEND_TO_BACK)
}

#[inline]
pub unsafe fn xQueueOverwrite(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
) -> BaseType_t {
    xQueueGenericSend(xQueue, pvItemToQueue, 0, queueOVERWRITE)
}

#[inline]
pub unsafe fn xQueueSendToFrontFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken, queueSEND_TO_FRONT)
}

#[inline]
pub unsafe fn xQueueSendToBackFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken, queueSEND_TO_BACK)
}

#[inline]
pub unsafe fn xQueueSendFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken, queueSEND_TO_BACK)
}

#[inline]
pub unsafe fn xQueueOverwriteFromISR(
    xQueue: QueueHandle_t,
    pvItemToQueue: *const c_void,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xQueueGenericSendFromISR(xQueue, pvItemToQueue, pxHigherPriorityTaskWoken, queueOVERWRITE)
}


#[inline]
pub unsafe fn xQueueReset(xQueue: QueueHandle_t) -> BaseType_t {
    xQueueGenericReset(xQueue, pdFALSE)
}

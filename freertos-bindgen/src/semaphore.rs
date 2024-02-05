// Missing defines and macros from semphr.h
// Macros have been converted with LLM assistance to inline functions
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::gen::*;
use crate::queue::*;
use core::ptr::{null, null_mut};

// vSemaphoreCreateBinary() is not implemented use xSemaphoreCreateBinary() for new designs

const semBINARY_SEMAPHORE_QUEUE_LENGTH: UBaseType_t = 1;
const semSEMAPHORE_QUEUE_ITEM_LENGTH: UBaseType_t = 0;
const semGIVE_BLOCK_TIME: TickType_t = 0;

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION)]
#[inline]
pub unsafe fn xSemaphoreCreateBinary() -> QueueHandle_t {
    xQueueGenericCreate(semBINARY_SEMAPHORE_QUEUE_LENGTH, semSEMAPHORE_QUEUE_ITEM_LENGTH, queueQUEUE_TYPE_BINARY_SEMAPHORE)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION)]
#[inline]
pub unsafe fn xSemaphoreCreateBinaryStatic(pxStaticSemaphore: &mut StaticSemaphore_t) -> QueueHandle_t {
    xQueueGenericCreateStatic(semBINARY_SEMAPHORE_QUEUE_LENGTH, semSEMAPHORE_QUEUE_ITEM_LENGTH, null_mut(), pxStaticSemaphore, queueQUEUE_TYPE_BINARY_SEMAPHORE)
}

#[inline]
pub unsafe fn xSemaphoreTake(xSemaphore: QueueHandle_t, xBlockTime: TickType_t) -> BaseType_t {
    xQueueSemaphoreTake(xSemaphore, xBlockTime)
}

#[cfg(configUSE_RECURSIVE_MUTEXES)]
#[inline]
pub unsafe fn xSemaphoreTakeRecursive(xMutex: QueueHandle_t, xBlockTime: TickType_t) -> BaseType_t {
    xQueueTakeMutexRecursive(xMutex, xBlockTime)
}

#[inline]
pub unsafe fn xSemaphoreGive(xSemaphore: QueueHandle_t) -> BaseType_t {
    xQueueGenericSend(xSemaphore, null(), semGIVE_BLOCK_TIME, queueSEND_TO_BACK)
}

#[cfg(configUSE_RECURSIVE_MUTEXES)]
#[inline]
pub unsafe fn xSemaphoreGiveRecursive(xMutex: QueueHandle_t) -> BaseType_t {
    xQueueGiveMutexRecursive(xMutex)
}

#[inline]
pub unsafe fn xSemaphoreGiveFromISR(xSemaphore: QueueHandle_t, pxHigherPriorityTaskWoken: &mut BaseType_t) -> BaseType_t {
    xQueueGiveFromISR(xSemaphore, pxHigherPriorityTaskWoken)
}

#[inline]
pub unsafe fn xSemaphoreTakeFromISR(xSemaphore: QueueHandle_t, pxHigherPriorityTaskWoken: &mut BaseType_t) -> BaseType_t {
    xQueueReceiveFromISR(xSemaphore, null_mut(), pxHigherPriorityTaskWoken)
}

#[cfg(all(configSUPPORT_DYNAMIC_ALLOCATION, configUSE_MUTEXES))]
#[inline]
pub unsafe fn xSemaphoreCreateMutex() -> QueueHandle_t {
    xQueueCreateMutex(queueQUEUE_TYPE_MUTEX)
}

#[cfg(all(configSUPPORT_STATIC_ALLOCATION, configUSE_MUTEXES))]
#[inline]
pub unsafe fn xSemaphoreCreateMutexStatic(pxMutexBuffer: &mut StaticSemaphore_t) -> QueueHandle_t {
    xQueueCreateMutexStatic(queueQUEUE_TYPE_MUTEX, pxMutexBuffer)
}

#[cfg(all(configSUPPORT_DYNAMIC_ALLOCATION, configUSE_RECURSIVE_MUTEXES))]
#[inline]
pub unsafe fn xSemaphoreCreateRecursiveMutex() -> QueueHandle_t {
    xQueueCreateMutex(queueQUEUE_TYPE_RECURSIVE_MUTEX)
}

#[cfg(all(configSUPPORT_STATIC_ALLOCATION, configUSE_RECURSIVE_MUTEXES))]
#[inline]
pub unsafe fn xSemaphoreCreateRecursiveMutexStatic(pxStaticSemaphore: &mut StaticSemaphore_t) -> QueueHandle_t {
    xQueueCreateMutexStatic(queueQUEUE_TYPE_RECURSIVE_MUTEX, pxStaticSemaphore)
}

#[cfg(configSUPPORT_DYNAMIC_ALLOCATION)]
#[inline]
pub unsafe fn xSemaphoreCreateCounting(uxMaxCount: UBaseType_t, uxInitialCount: UBaseType_t) -> QueueHandle_t {
    xQueueCreateCountingSemaphore(uxMaxCount, uxInitialCount)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION)]
#[inline]
pub unsafe fn xSemaphoreCreateCountingStatic(
    uxMaxCount: UBaseType_t,
    uxInitialCount: UBaseType_t,
    pxSemaphoreBuffer: &mut StaticSemaphore_t,
) -> QueueHandle_t {
    xQueueCreateCountingSemaphoreStatic(uxMaxCount, uxInitialCount, pxSemaphoreBuffer)
}

#[inline]
pub unsafe fn vSemaphoreDelete(xSemaphore: QueueHandle_t) {
    vQueueDelete(xSemaphore)
}

#[cfg(all(configUSE_MUTEXES, INCLUDE_xSemaphoreGetMutexHolder))]
#[inline]
pub unsafe fn xSemaphoreGetMutexHolder(xSemaphore: QueueHandle_t) -> TaskHandle_t {
    xQueueGetMutexHolder(xSemaphore)
}

#[cfg(all(configUSE_MUTEXES, INCLUDE_xSemaphoreGetMutexHolder))]
#[inline]
pub unsafe fn xSemaphoreGetMutexHolderFromISR(xSemaphore: QueueHandle_t) -> TaskHandle_t {
    xQueueGetMutexHolderFromISR(xSemaphore)
}

#[inline]
pub unsafe fn uxSemaphoreGetCount(xSemaphore: QueueHandle_t) -> UBaseType_t {
    uxQueueMessagesWaiting(xSemaphore)
}

#[inline]
pub unsafe fn uxSemaphoreGetCountFromISR(xSemaphore: QueueHandle_t) -> UBaseType_t {
    uxQueueMessagesWaitingFromISR(xSemaphore)
}

#[cfg(configSUPPORT_STATIC_ALLOCATION)]
#[inline]
pub unsafe fn xSemaphoreGetStaticBuffer(xSemaphore: QueueHandle_t, ppxSemaphoreBuffer: *mut *mut StaticSemaphore_t) -> BaseType_t {
    xQueueGenericGetStaticBuffers(xSemaphore, null_mut(), ppxSemaphoreBuffer)
}

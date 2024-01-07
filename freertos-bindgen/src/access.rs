#![allow(non_snake_case)]

use crate::bindings::*;

#[inline]
pub unsafe fn vGrantAccessToTask(xTask: TaskHandle_t, xTaskToGrantAccess: TaskHandle_t) {
    vGrantAccessToKernelObject(xTask, xTaskToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToTask(xTask: TaskHandle_t, xTaskToGrantAccess: TaskHandle_t) {
    vRevokeAccessToKernelObject(xTask, xTaskToGrantAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToSemaphore(
    xTask: TaskHandle_t,
    xSemaphoreToGrantAccess: SemaphoreHandle_t,
) {
    vGrantAccessToKernelObject(xTask, xSemaphoreToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToSemaphore(
    xTask: TaskHandle_t,
    xSemaphoreToRevokeAccess: SemaphoreHandle_t,
) {
    vRevokeAccessToKernelObject(xTask, xSemaphoreToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToQueue(xTask: TaskHandle_t, xQueueToGrantAccess: QueueHandle_t) {
    vGrantAccessToKernelObject(xTask, xQueueToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToQueue(xTask: TaskHandle_t, xQueueToRevokeAccess: QueueHandle_t) {
    vRevokeAccessToKernelObject(xTask, xQueueToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToQueueSet(
    xTask: TaskHandle_t,
    xQueueSetToGrantAccess: QueueSetHandle_t,
) {
    vGrantAccessToKernelObject(xTask, xQueueSetToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToQueueSet(
    xTask: TaskHandle_t,
    xQueueSetToRevokeAccess: QueueSetHandle_t,
) {
    vRevokeAccessToKernelObject(xTask, xQueueSetToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToEventGroup(
    xTask: TaskHandle_t,
    xEventGroupToGrantAccess: EventGroupHandle_t,
) {
    vGrantAccessToKernelObject(xTask, xEventGroupToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToEventGroup(
    xTask: TaskHandle_t,
    xEventGroupToRevokeAccess: EventGroupHandle_t,
) {
    vRevokeAccessToKernelObject(xTask, xEventGroupToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToStreamBuffer(
    xTask: TaskHandle_t,
    xStreamBufferToGrantAccess: StreamBufferHandle_t,
) {
    vGrantAccessToKernelObject(xTask, xStreamBufferToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToStreamBuffer(
    xTask: TaskHandle_t,
    xStreamBufferToRevokeAccess: StreamBufferHandle_t,
) {
    vRevokeAccessToKernelObject(xTask, xStreamBufferToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToMessageBuffer(
    xTask: TaskHandle_t,
    xMessageBufferToGrantAccess: MessageBufferHandle_t,
) {
    vGrantAccessToKernelObject(xTask, xMessageBufferToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToMessageBuffer(
    xTask: TaskHandle_t,
    xMessageBufferToRevokeAccess: MessageBufferHandle_t,
) {
    vRevokeAccessToKernelObject(xTask, xMessageBufferToRevokeAccess as i32)
}

#[inline]
pub unsafe fn vGrantAccessToTimer(xTask: TaskHandle_t, xTimerToGrantAccess: TimerHandle_t) {
    vGrantAccessToKernelObject(xTask, xTimerToGrantAccess as i32)
}

#[inline]
pub unsafe fn vRevokeAccessToTimer(xTask: TaskHandle_t, xTimerToRevokeAccess: TimerHandle_t) {
    vRevokeAccessToKernelObject(xTask, xTimerToRevokeAccess as i32)
}

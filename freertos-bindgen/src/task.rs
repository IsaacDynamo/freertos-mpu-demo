#![allow(non_snake_case)]

use crate::bindings::*;

#[inline]
pub unsafe fn xTaskNotify(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        ulValue,
        eAction,
        core::ptr::null_mut(),
    )
}

#[inline]
pub unsafe fn xTaskNotifyIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        core::ptr::null_mut(),
    )
}

#[inline]
pub unsafe fn xTaskNotifyAndQuery(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotifyValue: *mut u32,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        ulValue,
        eAction,
        pulPreviousNotifyValue,
    )
}

#[inline]
pub unsafe fn xTaskNotifyAndQueryIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotifyValue: *mut u32,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        pulPreviousNotifyValue,
    )
}

#[inline]
pub unsafe fn xTaskNotifyFromISR(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        ulValue,
        eAction,
        core::ptr::null_mut(),
        pxHigherPriorityTaskWoken,
    )
}

#[inline]
pub unsafe fn xTaskNotifyIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        core::ptr::null_mut(),
        pxHigherPriorityTaskWoken,
    )
}

#[inline]
pub unsafe fn xTaskNotifyAndQueryIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotificationValue: *mut u32,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        uxIndexToNotify,
        ulValue,
        eAction,
        pulPreviousNotificationValue,
        pxHigherPriorityTaskWoken,
    )
}

#[inline]
pub unsafe fn xTaskNotifyAndQueryFromISR(
    xTaskToNotify: TaskHandle_t,
    ulValue: u32,
    eAction: eNotifyAction,
    pulPreviousNotificationValue: *mut u32,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        ulValue,
        eAction,
        pulPreviousNotificationValue,
        pxHigherPriorityTaskWoken,
    )
}

#[inline]
pub unsafe fn xTaskNotifyWait(
    ulBitsToClearOnEntry: u32,
    ulBitsToClearOnExit: u32,
    pulNotificationValue: *mut u32,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTaskGenericNotifyWait(
        tskDEFAULT_INDEX_TO_NOTIFY,
        ulBitsToClearOnEntry,
        ulBitsToClearOnExit,
        pulNotificationValue,
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTaskNotifyWaitIndexed(
    uxIndexToWaitOn: UBaseType_t,
    ulBitsToClearOnEntry: u32,
    ulBitsToClearOnExit: u32,
    pulNotificationValue: *mut u32,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTaskGenericNotifyWait(
        uxIndexToWaitOn,
        ulBitsToClearOnEntry,
        ulBitsToClearOnExit,
        pulNotificationValue,
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTaskNotifyGive(xTaskToNotify: TaskHandle_t) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        0,
        eNotifyAction_eIncrement,
        core::ptr::null_mut(),
    )
}

#[inline]
pub unsafe fn xTaskNotifyGiveIndexed(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
) -> BaseType_t {
    xTaskGenericNotify(
        xTaskToNotify,
        uxIndexToNotify,
        0,
        eNotifyAction_eIncrement,
        core::ptr::null_mut(),
    )
}

#[inline]
pub unsafe fn vTaskNotifyGiveFromISR(
    xTaskToNotify: TaskHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) {
    vTaskGenericNotifyGiveFromISR(
        xTaskToNotify,
        tskDEFAULT_INDEX_TO_NOTIFY,
        pxHigherPriorityTaskWoken,
    )
}

#[inline]
pub unsafe fn vTaskNotifyGiveIndexedFromISR(
    xTaskToNotify: TaskHandle_t,
    uxIndexToNotify: UBaseType_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) {
    vTaskGenericNotifyGiveFromISR(xTaskToNotify, uxIndexToNotify, pxHigherPriorityTaskWoken)
}

#[inline]
pub unsafe fn ulTaskNotifyTake(xClearCountOnExit: BaseType_t, xTicksToWait: TickType_t) -> u32 {
    ulTaskGenericNotifyTake(tskDEFAULT_INDEX_TO_NOTIFY, xClearCountOnExit, xTicksToWait)
}

#[inline]
pub unsafe fn ulTaskNotifyTakeIndexed(
    uxIndexToWaitOn: UBaseType_t,
    xClearCountOnExit: BaseType_t,
    xTicksToWait: TickType_t,
) -> u32 {
    ulTaskGenericNotifyTake(uxIndexToWaitOn, xClearCountOnExit, xTicksToWait)
}

#[inline]
pub unsafe fn xTaskNotifyStateClear(xTask: TaskHandle_t) -> BaseType_t {
    xTaskGenericNotifyStateClear(xTask, tskDEFAULT_INDEX_TO_NOTIFY)
}

#[inline]
pub unsafe fn xTaskNotifyStateClearIndexed(
    xTask: TaskHandle_t,
    uxIndexToClear: UBaseType_t,
) -> BaseType_t {
    xTaskGenericNotifyStateClear(xTask, uxIndexToClear)
}

#[inline]
pub unsafe fn ulTaskNotifyValueClear(xTask: TaskHandle_t, ulBitsToClear: u32) -> u32 {
    ulTaskGenericNotifyValueClear(xTask, tskDEFAULT_INDEX_TO_NOTIFY, ulBitsToClear)
}

#[inline]
pub unsafe fn ulTaskNotifyValueClearIndexed(
    xTask: TaskHandle_t,
    uxIndexToClear: UBaseType_t,
    ulBitsToClear: u32,
) -> u32 {
    ulTaskGenericNotifyValueClear(xTask, uxIndexToClear, ulBitsToClear)
}

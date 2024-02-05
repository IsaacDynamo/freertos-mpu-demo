// Missing defines and macros from timers.h
// Macros have been converted with LLM assistance to inline functions
#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use crate::gen::*;

const tmrCOMMAND_EXECUTE_CALLBACK_FROM_ISR: BaseType_t = -2;
const tmrCOMMAND_EXECUTE_CALLBACK: BaseType_t = -1;
const tmrCOMMAND_START_DONT_TRACE: BaseType_t = 0;
const tmrCOMMAND_START: BaseType_t = 1;
const tmrCOMMAND_RESET: BaseType_t = 2;
const tmrCOMMAND_STOP: BaseType_t = 3;
const tmrCOMMAND_CHANGE_PERIOD: BaseType_t = 4;
const tmrCOMMAND_DELETE: BaseType_t = 5;
const tmrFIRST_FROM_ISR_COMMAND: BaseType_t = 6;
const tmrCOMMAND_START_FROM_ISR: BaseType_t = 6;
const tmrCOMMAND_RESET_FROM_ISR: BaseType_t = 7;
const tmrCOMMAND_STOP_FROM_ISR: BaseType_t = 8;
const tmrCOMMAND_CHANGE_PERIOD_FROM_ISR: BaseType_t = 9;

#[inline]
unsafe fn xTimerGenericCommand(
    xTimer: TimerHandle_t,
    xCommandID: BaseType_t,
    xOptionalValue: TickType_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    if xCommandID < tmrFIRST_FROM_ISR_COMMAND {
        xTimerGenericCommandFromTask(
            xTimer,
            xCommandID,
            xOptionalValue,
            pxHigherPriorityTaskWoken,
            xTicksToWait,
        )
    } else {
        xTimerGenericCommandFromISR(
            xTimer,
            xCommandID,
            xOptionalValue,
            pxHigherPriorityTaskWoken,
            xTicksToWait,
        )
    }
}

#[inline]
pub unsafe fn xTimerStart(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_START,
        xTaskGetTickCount(),
        core::ptr::null_mut(),
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTimerStop(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_STOP,
        0,
        core::ptr::null_mut(),
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTimerChangePeriod(
    xTimer: TimerHandle_t,
    xNewPeriod: TickType_t,
    xTicksToWait: TickType_t,
) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_CHANGE_PERIOD,
        xNewPeriod,
        core::ptr::null_mut(),
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTimerDelete(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_DELETE,
        0,
        core::ptr::null_mut(),
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTimerReset(xTimer: TimerHandle_t, xTicksToWait: TickType_t) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_RESET,
        xTaskGetTickCount(),
        core::ptr::null_mut(),
        xTicksToWait,
    )
}

#[inline]
pub unsafe fn xTimerStartFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_START_FROM_ISR,
        xTaskGetTickCountFromISR(),
        pxHigherPriorityTaskWoken,
        0,
    )
}

#[inline]
pub unsafe fn xTimerStopFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_STOP_FROM_ISR,
        0,
        pxHigherPriorityTaskWoken,
        0,
    )
}

#[inline]
pub unsafe fn xTimerChangePeriodFromISR(
    xTimer: TimerHandle_t,
    xNewPeriod: TickType_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_CHANGE_PERIOD_FROM_ISR,
        xNewPeriod,
        pxHigherPriorityTaskWoken,
        0,
    )
}

#[inline]
pub unsafe fn xTimerResetFromISR(
    xTimer: TimerHandle_t,
    pxHigherPriorityTaskWoken: *mut BaseType_t,
) -> BaseType_t {
    xTimerGenericCommand(
        xTimer,
        tmrCOMMAND_RESET_FROM_ISR,
        xTaskGetTickCountFromISR(),
        pxHigherPriorityTaskWoken,
        0,
    )
}

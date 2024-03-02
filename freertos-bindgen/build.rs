use bindgen::callbacks::{ItemInfo, ParseCallbacks};
use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, sync::Mutex};

#[derive(Debug, Default)]
struct Config {
    kernel_object_accessors: bool,
    public_api: HashMap<String, bool>,
}

#[derive(Debug)]
struct Callbacks(Arc<Mutex<Config>>);

impl ParseCallbacks for Callbacks {
    fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        item_info.name.strip_prefix("MPU_").map(|x| x.to_string())
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        let mut config = self.0.lock().unwrap();

        if let Some(found) = config.public_api.get_mut(original_item_name) {
            *found = true;
        }

        if original_item_name == "vGrantAccessToKernelObject" {
            config.kernel_object_accessors = true;
        }
        None
    }
}

const HEADERS: &[&str] = &[
    "FreeRTOSConfig.h",
    "FreeRTOS.h",
    "task.h",
    "queue.h",
    "stream_buffer.h",
    "message_buffer.h",
    "semphr.h",
    "timers.h",
    "event_groups.h",
];

const PUBLIC: &[&str] = &[
    "configMINIMAL_STACK_SIZE",


    "UBaseType_t",
    "StackType_t",
    "TaskHandle_t",
    "TaskFunction_t",
    "StaticTask_t",
    "TaskParameters_t",
    "MemoryRegion_t",
    "StaticQueue_t",
    "StaticSemaphore_t",
    "TickType_t",

    // Task Creation
    "xTaskCreate",
    "xTaskCreateStatic",
    "vTaskDelete",
    "xTaskGetStaticBuffers",

    // Task Control
    "vTaskDelay",
    "vTaskDelayUntil",
    "xTaskDelayUntil",
    "uxTaskPriorityGet",
    "uxTaskPriorityGetFromISR",
    "uxTaskBasePriorityGet",
    "uxTaskBasePriorityGetFromISR",
    "vTaskPrioritySet",
    "vTaskSuspend",
    "vTaskResume",
    "xTaskResumeFromISR",
    "xTaskAbortDelay",

    // Task Utilities
    "uxTaskGetSystemState",
    "vTaskGetInfo",
    "xTaskGetCurrentTaskHandle",
    "xTaskGetIdleTaskHandle",
    "uxTaskGetStackHighWaterMark",
    "eTaskGetState",
    "pcTaskGetName",
    "xTaskGetHandle",
    "xTaskGetTickCount",
    "xTaskGetTickCountFromISR",
    "xTaskGetSchedulerState",
    "uxTaskGetNumberOfTasks",
    "vTaskList",
    "vTaskListTasks",
    "vTaskStartTrace",
    "ulTaskEndTrace",
    "vTaskGetRunTimeStats",
    "vTaskGetRunTimeStatistics",
    "vTaskGetIdleRunTimeCounter",
    "ulTaskGetRunTimeCounter",
    "ulTaskGetRunTimePercent",
    "ulTaskGetIdleRunTimeCounter",
    "ulTaskGetIdleRunTimePercent",
    "vTaskSetApplicationTaskTag",
    "xTaskGetApplicationTaskTag",
    "xTaskCallApplicationTaskHook",
    "pvTaskGetThreadLocalStoragePointer",
    "vTaskSetThreadLocalStoragePointer",
    "vTaskSetTimeOutState",
    "xTaskCheckForTimeOut",

    // Kernel Control
    "taskYIELD",
    "taskENTER_CRITICAL",
    "taskEXIT_CRITICAL",
    "taskENTER_CRITICAL_FROM_ISR",
    "taskEXIT_CRITICAL_FROM_ISR",
    "taskDISABLE_INTERRUPTS",
    "taskENABLE_INTERRUPTS",
    "vTaskStartScheduler",
    "vTaskEndScheduler",
    "vTaskSuspendAll",
    "xTaskResumeAll",
    "vTaskStepTick",

    // Task Notifications
    // Provided by task.rs
    // "xTaskNotifyGive()", "xTaskNotifyGiveIndexed()",
    // "vTaskNotifyGiveFromISR()", "vTaskNotifyGiveIndexedFromISR()",
    // "ulTaskNotifyTake()", "ulTaskNotifyTakeIndexed()",
    // "xTaskNotify()", "xTaskNotifyIndexed()",
    // "xTaskNotifyAndQuery()", "xTaskNotifyAndQueryIndexed()",
    // "xTaskNotifyAndQueryFromISR", "xTaskNotifyAndQueryFromISRIndexed()",
    // "xTaskNotifyFromISR()", "xTaskNotifyFromISRIndexed()",
    // "xTaskNotifyWait()", "xTaskNotifyWaitIndexed()",
    // "xTaskNotifyStateClear()", "xTaskNotifyStateClearIndexed()",
    // "ulTasknotifyValueClear()", "ulTasknotifyValueClearIndexed()",

    // Queue Management
    "xQueueCreate",
    "xQueueCreateStatic",
    "vQueueDelete",
    "xQueueSend",
    "xQueueSendFromISR",
    "xQueueSendToBack",
    "xQueueSendToBackFromISR",
    "xQueueSendToFront",
    "xQueueSendToFrontFromISR",
    "xQueueReceive",
    "xQueueReceiveFromISR",
    "uxQueueMessagesWaiting",
    "uxQueueMessagesWaitingFromISR",
    "uxQueueSpacesAvailable",
    "xQueueReset",
    "xQueuePeek",
    "xQueuePeekFromISR",
    "vQueueAddToRegistry",
    "pcQueueGetName",
    "vQueueUnregisterQueue",
    "xQueueIsQueueEmptyFromISR",
    "xQueueIsQueueFullFromISR",
    "xQueueOverwrite",
    "xQueueOverwriteFromISR",
    "xQueueGetStaticBuffers",

    // Queue Set
    "xQueueCreateSet",
    "xQueueAddToSet",
    "xQueueRemoveFromSet",
    "xQueueSelectFromSet",
    "xQueueSelectFromSetFromISR",

    // Stream Buffers
    "xStreamBufferCreate",
    "xStreamBufferCreateStatic",
    "xStreamBufferSend",
    "xStreamBufferSendFromISR",
    "xStreamBufferReceive",
    "xStreamBufferReceiveFromISR",
    "vStreamBufferDelete",
    "xStreamBufferBytesAvailable",
    "xStreamBufferSpacesAvailable",
    "xStreamBufferSetTriggerLevel",
    "xStreamBufferReset",
    "xStreamBufferIsEmpty",
    "xStreamBufferIsFull",
    "xStreamBufferGetStaticBuffers",

    // Message Buffers
    "xMessageBufferCreate",
    "xMessageBufferCreateStatic",
    "xMessageBufferSend",
    "xMessageBufferSendFromISR",
    "xMessageBufferReceive",
    "xMessageBufferReceiveFromISR",
    "vMessageBufferDelete",
    "xMessageBufferSpacesAvailable",
    "xMessageBufferReset",
    "xMessageBufferIsEmpty",
    "xMessageBufferIsFull",
    "xMessageBufferGetStaticBuffers",

    // Semaphores
    // Implemented in semaphore.rs
    // "xSemaphoreCreateBinary",
    // "xSemaphoreCreateBinaryStatic",
    // "vSemaphoreCreateBinary",
    // "xSemaphoreCreateCounting",
    // "xSemaphoreCreateCountingStatic",
    // "xSemaphoreCreateMutex",
    // "xSemaphoreCreateMutexStatic",
    // "xSemaphoreCreateRecursiveMutex",
    // "xSemaphoreCreateRecursiveMutexStatic",
    // "vSemaphoreDelete",
    // "xSemaphoreGetMutexHolder",
    // "xSemaphoreGetMutexHolderFromISR",
    // "xSemaphoreTake",
    // "xSemaphoreTakeFromISR",
    // "xSemaphoreTakeRecursive",
    // "xSemaphoreGive",
    // "xSemaphoreGiveRecursive",
    // "xSemaphoreGiveFromISR",
    // "uxSemaphoreGetCount",
    // "uxSemaphoreGetCountFromISR",
    // "xSemaphoreGetStaticBuffer",

    // Software Timer
    "xTimerCreate",
    "xTimerCreateStatic",
    "xTimerIsTimerActive",
    "pvTimerGetTimerID",
    "pcTimerGetName",
    "vTimerSetReloadMode",
    // Implemented in timers.rs
    // "xTimerStart",
    // "xTimerStop",
    // "xTimerChangePeriod",
    // "xTimerDelete",
    // "xTimerReset",
    // "xTimerStartFromISR",
    // "xTimerStopFromISR",
    // "xTimerChangePeriodFromISR",
    // "xTimerResetFromISR",
    "pvTimerGetTimerID",
    "vTimerSetTimerID",
    "xTimerGetTimerDaemonTaskHandle",
    "xTimerPendFunctionCall",
    "xTimerPendFunctionCallFromISR",
    "pcTimerGetName",
    "xTimerGetPeriod",
    "xTimerGetExpiryTime",
    "xTimerGetReloadMode",

    // Event Groups and Event Bits
    "vEventGroupDelete",
    "xEventGroupClearBits",
    "xEventGroupClearBitsFromISR",
    "xEventGroupCreate",
    "xEventGroupCreateStatic",
    "xEventGroupGetBits",
    "xEventGroupGetBitsFromISR",
    "xEventGroupGetStaticBuffer",
    "xEventGroupSetBits",
    "xEventGroupSetBitsFromISR",
    "xEventGroupSync",
    "xEventGroupWaitBits",

    // Co-routine specific API is not supported

    // MPU Specific Functions
    "xTaskCreateRestricted",
    "xTaskCreateRestrictedStatic",
    "vTaskAllocateMPURegions",
    "portSWITCH_TO_USER_MODE",
];

fn main() {
    let all_headers = HEADERS.iter().fold(String::new(), |mut acc, &header| {
        acc.write_fmt(format_args!("#include <{}>\n", header))
            .unwrap();
        acc
    });

    let builder = bindgen::Builder::default().header_contents("all_headers.h", &all_headers);

    // let builder = HEADERS.iter().fold(builder, |builder, &header| {
    //     builder.allowlist_file(format!(".*{}", header))
    // });

    let config = Config {
        public_api: HashMap::from_iter(PUBLIC.iter().map(|x| (x.to_string(), false))),
        ..Default::default()
    };

    let config = Arc::new(Mutex::new(config));

    let bindings = builder
        .clang_arg(format!("-I.."))
        .clang_arg(format!("-I../FreeRTOS-Kernel/include"))
        .clang_arg(format!("-I../FreeRTOS-Kernel/portable/GCC/ARM_CM4_MPU"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(Callbacks(config.clone())))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("gen.rs"))
        .expect("Couldn't write gen.rs");

    let config = config.lock().unwrap();

    let exports = config
        .public_api
        .iter()
        .filter_map(|x| x.1.then(|| format!("pub use gen::{};\n", x.0)))
        .collect::<String>();

    std::fs::write(out_path.join("public.rs"), exports.as_bytes())
        .expect("Couldn't write public.rs");


    for (name, &found) in config.public_api.iter() {
        if !found {
            eprintln!("warning: {} not found", name);
        }
    }

    if config.kernel_object_accessors {
        println!("cargo:rustc-cfg=kernel_object_accessors")
    }

    println!("cargo:rustc-cfg=configSUPPORT_STATIC_ALLOCATION");
}

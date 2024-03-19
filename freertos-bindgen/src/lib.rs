#![no_std]
#![allow(non_upper_case_globals)]

pub mod gen;

mod task;
mod timers;
mod semaphore;
mod queue;
mod default;

// ( portUSING_MPU_WRAPPERS == 1 ) && ( configUSE_MPU_WRAPPERS_V1 == 0 ) && ( configENABLE_ACCESS_CONTROL_LIST == 1 )
#[cfg(kernel_object_accessors)]
mod access;

pub use task::*;
pub use timers::*;
pub use semaphore::*;
pub use default::*;
pub use queue::*;

#[cfg(kernel_object_accessors)]
pub use access::*;

// Defines
pub const pdFALSE: BaseType_t = 0;
pub const pdTRUE: BaseType_t = 1;

pub const pdPASS: BaseType_t = pdFALSE;
pub const pdFAIL: BaseType_t = pdTRUE;

// Include build.rs generated public re-exports
include!(concat!(env!("OUT_DIR"), "/public.rs"));

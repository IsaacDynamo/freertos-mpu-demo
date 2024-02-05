// Zero-init default constructor for C structs

use crate::gen::{StaticSemaphore_t, StaticTask_t, MemoryRegion_t};
use core::mem::MaybeUninit;

impl Default for StaticSemaphore_t {
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Default for StaticTask_t {
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

impl Default for MemoryRegion_t {
    fn default() -> Self {
        unsafe { MaybeUninit::zeroed().assume_init() }
    }
}

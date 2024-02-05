// Missing defines from queue.h
#![allow(non_upper_case_globals)]

// TODO impl Queue macros as inline functions, until than allow unused const.
#![allow(unused)]

use crate::gen::BaseType_t;

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



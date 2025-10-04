# Notes

export PATH=$PATH:/opt/gcc-arm-none-eabi-10-2020-q4-major/bin/

gdb-multiarch -f target/thumbv7m-none-eabi/release/stm32l452-nucleo-demo -ex "target remote host.docker.internal:3333" -ex "monitor arm semihosting enable"

// RUST_BACKTRACE=1 cargo test --target x86_64-unknown-linux-gnu  -- --nocapture

// Prevent drop by leaking the guard.
// https://github.com/rust-lang/rust/issues/24292


// https://github.com/espressif/freertos-gdb
// (gdb) python import freertos_gdb
// (gdb) freertos task
# Partial FreeRTOS bindings
**Currently this crate only exposes an API subset needed in the demo.**

FreeRTOS is highly configurable so generating the bindings would be preferred. So this crate uses bindgen to expose the function in the public API. However the FreeRTOS codebase also has a lot of macros in its public interface which are not fully supported by bindgen. So additional function are provided that try to mimic the macros. This is not ideal because the conversion is error prone and incomplete. Some macros will require C function shims to make them callable from Rust. This is not yet implemented.

## The ecosystem
There exists a lot of FreeRTOS crates already. But unfortunately there isn't a single crate that is flexible enough to supports multiple use-cases.

If I had time to crate bindings for FreeRTOS I would try to support the following uses-case and features.

### Use-cases
- Pure Rust project. Cargo builds app, optional builds FreeRTOS and links.
- A task in Rust. Cargo crates static library, external build system builds FreeRTOS and links the final executable.
- Consumable by a crate that wants to provide a safe Rust API.

### Features
- `no_std`
- Optional `alloc`
- Configuration of FreeRTOS path, config, port and heap, with defaults where possible.
- Optional build and linking of FreeRTOS and shims
- Configuration of cross compiler tools might be needed
- Exposed API should respect FreeRTOSConfig.h
- Only use a shim when needed

## Other crates
### [freertos.rs](https://github.com/hashmismatch/freertos.rs)
Assumes `alloc`, uses shims for everything.

### [FreeRTOS-rust](https://github.com/lobaro/FreeRTOS-rust)
Assumes `alloc`, uses shims for everything. Always links FreeRTOS.

### [freertos-sys2](https://github.com/junelife/freertos-sys2)
Not using bindgen yet. Looks promising.

### [libfreertos-sys](https://github.com/axos88/libfreertos-sys)
C code compilation as a library and bindings for FreeRTOS v10.2.1

### [freertos-esp32-sys](https://github.com/N3xed/freertos-esp32-sys)
This library is based on FreeRTOS-rust.
The bindings were generated using bindgen.
ESP32 specific.

### [esp-idf-sys](https://github.com/esp-rs/esp-idf-sys)
Contains bindings for FreeRTOS, but ESP only.

### [freertos-sys](https://github.com/tstellanova/freertos-sys)
Builds and links FreeRTOS but with CMSIS-RTOS2 bindings.

### [freertos-std](https://github.com/sheref-sidarous/freertos-std)
Manual bindings.




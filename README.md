# FreeRTOS MPU demo in Rust
This is a proof of concept to see what it would take to use FreeRTOS together with Rust, in order to run memory protected tasks on a ARM Cortex with MPU.

# Structure
## stm32l452-nucleo-demo
Demo with memory protected tasks

## FreeRTOS-Kernel
FreeRTOS v11.0.1

## freertos-bindgen
Partial FreeRTOS bindings

# Build FreeRTOS
Building FreeRTOS requires `make`, `cmake` and `arm-none-eabi-gcc`.
```
mkdir build
cd build
cmake ..
make
```

# Build demo
```
cd stm32l452-nucleo-demo
cargo build --release
```


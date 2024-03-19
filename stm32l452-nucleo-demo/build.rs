fn main() {
    println!("cargo:rustc-link-search=native=../build/FreeRTOS-Kernel");
    println!("cargo:rustc-link-lib=freertos_kernel");
    println!("cargo:rustc-link-search=native=../build/FreeRTOS-Kernel/portable");
    println!("cargo:rustc-link-lib=freertos_kernel_port");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../build/FreeRTOS-Kernel/libfreertos_kernel.a");
    println!("cargo:rerun-if-changed=../build/FreeRTOS-Kernel/portable/libfreertos_kernel_port.a");
}

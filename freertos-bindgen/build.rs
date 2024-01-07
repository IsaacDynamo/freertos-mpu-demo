use bindgen::callbacks::{ItemInfo, ParseCallbacks};
use std::fmt::Write;
use std::path::PathBuf;
use std::sync::Arc;
use std::{env, sync::Mutex};

#[derive(Debug, Default)]
struct Config {
    kernel_object_accessors: bool,
}

#[derive(Debug)]
struct Callbacks(Arc<Mutex<Config>>);

impl ParseCallbacks for Callbacks {
    fn generated_name_override(&self, item_info: ItemInfo<'_>) -> Option<String> {
        item_info.name.strip_prefix("MPU_").map(|x| x.to_string())
    }

    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name == "vGrantAccessToKernelObject" {
            self.0.lock().as_mut().unwrap().kernel_object_accessors = true;
        }
        None
    }
}

fn main() {
    let headers = [
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

    let all_headers = headers.iter().fold(String::new(), |mut acc, &header| {
        acc.write_fmt(format_args!("#include <{}>\n", header))
            .unwrap();
        acc
    });

    let builder = bindgen::Builder::default().header_contents("all_headers.h", &all_headers);

    let builder = headers.iter().fold(builder, |builder, &header| {
        builder.allowlist_file(format!(".*{}", header))
    });

    let config = Arc::new(Mutex::new(Config::default()));

    let bindings = builder
        .clang_arg(format!("-I../../freertos-main"))
        .clang_arg(format!("-I../FreeRTOS-Kernel/include"))
        .clang_arg(format!("-I../FreeRTOS-Kernel/portable/GCC/ARM_CM3_MPU"))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(Callbacks(config.clone())))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    if config.lock().unwrap().kernel_object_accessors {
        println!("cargo:rustc-cfg=kernel_object_accessors")
    }
}

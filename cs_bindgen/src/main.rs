// lib.rs, simple FFI code
#[no_mangle]
pub extern "C" fn my_add(x: i32, y: i32) -> i32 {
    x + y
}

fn main() {
    csbindgen::Builder::default()
        .input_extern_file("../clam_ffi/src/lib.rs")
        .csharp_dll_name("clam_ffi")
        .csharp_namespace("ClamFFI")
        .csharp_class_name("Clam")
        .csharp_class_accessibility("public")
        .csharp_class_accessibility("public")
        .generate_csharp_file("../../unity/Assets/Plugins/ClamFFI.cs")
        .unwrap();
}

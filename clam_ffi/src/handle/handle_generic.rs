pub struct HandleGeneric<T,U,I> {
    value1: T,
    value2: U,
    value3: I,
}

impl<T,U,I> HandleGeneric<T,U,I> {
    pub fn new(value1: T, value2 : U, value3: I) -> Self {
        Self { value1,value2,value3 }
    }
}

#[no_mangle]
pub extern "C" fn create_handle_f32_i32_u8(ptr: Option<&mut *mut HandleGeneric<f32,i32,u8>>, value1: f32,value2: i32,value3: u8) {
    if let Some(out_handle) = ptr {
        let handle = HandleGeneric::new(value1,value2,value3);
        *out_handle = Box::into_raw(Box::new(handle));
    }
}

#[no_mangle]
pub extern "C" fn create_handle_u8_f64_i32(ptr: Option<&mut *mut HandleGeneric<u8,f64,i32>>, value1 : u8, value2: f64, value3: i32) {
    if let Some(out_handle) = ptr {
        let handle = HandleGeneric::new(value1, value2, value3);
        *out_handle = Box::into_raw(Box::new(handle));
    }
}
#[no_mangle]
pub extern "C" fn get_f32_i32_u8_value1(ptr: Option<&mut HandleGeneric<f32,i32,u8>>) -> f32{
    if let Some(handle) = ptr{
        return handle.value1;
    }
    return -1.0;
}

#[no_mangle]
pub extern "C" fn get_u8_f64_i32_value1(ptr: Option<&mut HandleGeneric<u8,f64,i32>>) -> u8{
    if let Some(handle) = ptr{
        return handle.value1;
    }
    return 1;
}

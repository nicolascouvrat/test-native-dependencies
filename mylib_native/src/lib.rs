use mylib_rs::a_rust_endpoint;

#[no_mangle]
pub extern "C" fn mylib_call() {
    a_rust_endpoint();
}

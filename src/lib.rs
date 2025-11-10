

#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut ffi::c_void) {
    if reason == DLL_PROCESS_ATTACH {
        
    }
}

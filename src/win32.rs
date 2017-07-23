extern crate winapi;
extern crate kernel32;

pub use self::winapi::minwindef::{HMODULE};
use std;

// cstring helper becuase the ffi::CString is horrible to type
pub trait AsCStr {
	fn as_cstr(&self) -> std::ffi::CString;
}

impl AsCStr for str {
	fn as_cstr(&self) -> std::ffi::CString {
		std::ffi::CString::new(self).unwrap()
	}
}

// some winapi wrappers
pub fn get_module_handle(s: &str) -> Result<HMODULE, &str> {
	let result = unsafe{kernel32::GetModuleHandleA(s.as_cstr().as_ptr())};

	if result.is_null() {
		return Err("module null");
	}

	return Ok(result);
}

pub fn get_proc_address<T>(h: HMODULE, s: &str) -> Result<T, &str> {
	let fnptr = unsafe{(kernel32::GetProcAddress(h, s.as_cstr().as_ptr()))};

	if fnptr.is_null() {
		return Err(("fnptr null"));
	}

	let func: T = unsafe{std::mem::transmute_copy(&fnptr)};

	return Ok(func);
}

pub fn alloc_console() {
	unsafe {kernel32::AllocConsole()};
}

pub fn free_library_and_exit_thread(h: HMODULE, ret: u32) {
	unsafe{kernel32::FreeLibraryAndExitThread(h, ret)}
}
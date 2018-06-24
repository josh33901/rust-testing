use win32::*;
use std;

// use std::fmt to format before passing here
// rust "doesnt support" varargs functions out of the box

pub fn console(s: &str) {
	let result: fn (*const i8) = get_proc_address(get_module_handle("tier0.dll").unwrap(), "Warning").unwrap();

	//println!("Warning @ {:p}", result);
	println!("{}", s);

	result(s.as_cstr().as_ptr());
}

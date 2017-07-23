
use win32::*;

pub type CreateInterfaceFn<T> = fn(*const i8, error: *mut u32) -> T;

pub fn get_interface_from_module<T>(module: &str, interface: &str) -> Result<T, &'static str> {

	let create_interface_fn: CreateInterfaceFn <T> = get_proc_address(get_module_handle(module).unwrap(), "CreateInterface").unwrap();

	let mut status: u32 = 0;
	let result = create_interface_fn(interface.as_cstr().as_ptr(), &mut status);

	if status != 0 {
		return Err(("Non-Zero status from CreateInterface"));
	} else {
		return Ok(result);
	}
}
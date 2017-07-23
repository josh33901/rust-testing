#![feature(abi_thiscall)]

extern crate libc;

mod sdk;
use sdk::*;

mod win32;
use win32::AsCStr;



#[no_mangle]
#[allow(unused_variables)]
#[allow(non_snake_case)]
/// Entry point.
pub extern "stdcall" fn DllMain(module: win32::HMODULE, reason_for_call: u32, reserved: *mut libc::c_void) {
	match reason_for_call {
		1 => { // DLL_PROCESS_ATTACH
			// start the runtime, and away we go!
			std::thread::Builder::new().name("init".to_string()).spawn(init).unwrap();
		},
		_ => () // who gives a shit
	}
}

extern "thiscall" fn hooked_play_sound(instance: *const u32, sound: *const i8) {
	unsafe{log::console(&std::ffi::CStr::from_ptr(sound).to_string_lossy().into_owned())};
}
 
fn init() {
	win32::alloc_console();


	println!("hello!");

	log::console("\n\n\n\n\n\n\n\n\n\nhello !\n\n\n\n\n\n\n\n\n\n\n\n\n");


	log::console("get surface\n");
	let surface: &Surface = appsystem::get_interface_from_module("vguimatsurface.dll", "VGUI_Surface030").unwrap();

	log::console("play\n");
	surface.play_sound("sound\\vo\\heavy_award01.wav");
	log::console("we did it\n");

	let mut surface_hook = VMTHook::<Surface>::new(surface, None);
	(*surface_hook).hook_method(&hooked_play_sound, 78);
	(*surface_hook).rehook();

	log::console("play 2.0\n");
	surface.play_sound("sound\\vo\\heavy_award01\n.wav");
	log::console("double did it!\n");

	win32::free_library_and_exit_thread(win32::get_module_handle("f1_rust.dll").unwrap(), 0);
}
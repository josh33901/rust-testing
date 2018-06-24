use sdk::vtable::*;

use win32::AsCStr;

use log;

pub struct Surface {

}

impl VTable for Surface {}

impl Surface {
	pub fn play_sound(&self, s: &str) {
		let original: extern "thiscall" fn(*const u32, *const i8) = unsafe{self.get_function(78, None)};
		return original(unsafe{self.get_this()}, s.as_cstr().as_ptr());
	}
}
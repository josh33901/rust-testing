use std;


// From DS2015
// inline void **&getvtable (void *inst, size_t offset = 0)
// {
// 	return *reinterpret_cast<void ***> ((size_t)inst + offset);
// }
// inline const void **getvtable (const void *inst, size_t offset = 0)
// {
// 	return *reinterpret_cast<const void ***> ((size_t)inst + offset);
// }
// template <typename Fn>
// inline Fn getvfunc (void *inst, size_t index, size_t offset = 0)
// {
// 	return reinterpret_cast<Fn> (getvtable (inst, offset)[index]);
// }

pub trait VTable {
	unsafe fn get_this(&self) -> *const u32{
		let this: *const u32 = std::mem::transmute(self as *const _ as *const u32);

		println!("this @ {:p}", this);

		return this;
	}
	unsafe fn get_table(&self, offset: Option<isize>) -> *const *const u32 {
		let this = self.get_this();

		let table_address = *(this.offset(offset.unwrap_or(0) as isize));

		println!("table @ {:x}", table_address);

		return std::mem::transmute(table_address);
	}

	unsafe fn get_function<T>(&self, index: u32, offset: Option<isize>) -> T {
		let table = self.get_table(offset);

		let func_address = table.offset(index as isize);

		println!("func address @ {:p} --> {:p} off: {:x}", table, func_address, (func_address as u32 - table as u32) / 4);

		let func: T = std::mem::transmute_copy(&(*func_address));

		return func;
	}
}

pub struct VMTHook<T> {
	this: *const T,
	old_vtable: *const u32, // pointer to data section
	real_vtable: *const *const u32,
	array: Vec<u32>,
}
impl <T: VTable> VMTHook<T>  {
	unsafe fn count_funcs(vmt: *const *const u32) -> isize {
		// -1 for the rtti pointer
		let mut i: isize = -1;

		// do while hack
		while{
			vmt.offset(i).is_null() == false
		} { 
			i += 1
		}

		assert!(i >= 0);

		return i;
	}

	pub fn new(instance: &T, offset: Option<isize>) -> Box<VMTHook<T>> {
	    let this = unsafe{instance.get_this()};
	    let real_vtable = unsafe{instance.get_table(offset)};

	    let mut ret: Box<VMTHook<T>> = Box::new(VMTHook{
	    	this: instance,
	    	old_vtable: unsafe{*real_vtable as *const u32},
	    	real_vtable: real_vtable,
	    	array: Vec::new(),

	    });

	    let self_address = &ret as *const Box<VMTHook<T>> as u32;

		ret.array.push(self_address);
	    ret.array.push(unsafe{std::mem::transmute(0xFAC0D775u32)});

	    // create a backup of funcs
	    for i in -1..unsafe{VMTHook::<T>::count_funcs(real_vtable)} {
	    	ret.array.push(unsafe{std::mem::transmute(real_vtable.offset(i as isize))});
	    }
	    
	    return ret;
	}

	pub fn hook_method<F>(&self, newfunc: &F, index: isize) {
		assert!(index < unsafe{VMTHook::<T>::count_funcs(self.real_vtable)});

		let address: *mut u32 = unsafe{self.array.as_ptr().offset(index + 3)} as *mut u32;

		unsafe{*address = newfunc as *const F as u32};
	}

	pub fn rehook(&mut self) {
		self.real_vtable = unsafe{&self.array.as_ptr().offset(3) as *const _};
	}
}
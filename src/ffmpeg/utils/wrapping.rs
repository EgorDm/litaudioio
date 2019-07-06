pub trait FFWrapper<T> {
	fn as_ptr(&self) -> *const T;

	fn as_mut_ptr(&self) -> *mut T;

	fn as_ref(&self) -> &T {
		unsafe { self.as_ptr().as_ref().unwrap() }
	}

	fn as_mut_ref(&mut self) -> &mut T {
		unsafe { self.as_mut_ptr().as_mut().unwrap() }
	}
}

macro_rules! ff_wrap_struct (
	($Name: ident, $Type: ident) => {
		pub struct $Name {
			ptr: *mut $Type,
		}

		impl $Name {
			pub fn new(ptr: *mut $Type) -> Option<$Name> {
				if ptr.is_null() {
					None
				} else {
					Some(Self { ptr })
				}
			}
		}
	}
);

macro_rules! ff_wrap (
	($Name: ident, $Type: ident) => {
		impl FFWrapper<$Type> for $Name {
			fn as_ptr(&self) -> *const $Type { self.ptr }

			fn as_mut_ptr(&self) -> *mut $Type { self.ptr }
		}
	}
);


macro_rules! wrap_ff_wrap_field (
	($Name: ident, $Type: ident, $field: ident, $field_mut: ident) => {
		impl $Name {
			pub fn $field(&self) -> &$Type { &self.$field }

			pub fn $field_mut(&mut self) -> &mut $Type { &mut self.$field }
		}
	}
);

macro_rules! wrap_ff_wrap (
	($Name: ident, $Type: ident, $FFType: ident, $field: ident, $field_mut: ident) => {
		wrap_ff_wrap_field!($Name, $Type, $field, $field_mut);

		impl FFWrapper<$FFType> for $Name {
			fn as_ptr(&self) -> *const $FFType { self.$field.as_ptr() }

			fn as_mut_ptr(&self) -> *mut $FFType { self.$field.as_mut_ptr() }
		}
	}
);


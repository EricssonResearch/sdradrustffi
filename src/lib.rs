//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

#![feature(allocator_api)]
#![feature(alloc_layout_extra)]

extern crate bitflags;
extern crate core;
extern crate libc;
extern crate paste;

#[cfg(feature = "abomonation_v1")]
extern crate abomonation_v1 as abomonation;

#[cfg(feature = "abomonation_v2")]
extern crate abomonation_v2 as abomonation;

#[cfg(feature = "bincode_v2")]
extern crate bincode_v2 as bincode;

#[cfg(feature = "bincode_v1")]
extern crate bincode_v1 as bincode;

#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
pub use abomonation::{decode, encode, measure, Abomonation};

#[macro_use]
#[cfg(any(feature = "abomonation_v1", feature = "abomonation_v2"))]
pub mod sdrad_abomonation;

#[macro_use]
#[cfg(feature = "bincode_v1")]
pub mod sdrad_bincode;

#[cfg(feature = "bincode_v1")]
pub use bincode::{deserialize, serialize, serialize_into};

#[macro_use]
#[cfg(feature = "bincode_v2")]
pub mod sdrad_bincode_v2;

#[cfg(feature = "bincode_v2")]
pub use bincode::config;

#[cfg(feature = "bincode_v2")]
pub use bincode::{decode_from_slice, decode_from_std_read, encode_into_std_write};

#[cfg(feature = "bincode_v2")]
pub use bincode::serde::decode_borrowed_from_slice;

pub use bitflags::bitflags;
pub use paste::paste;
pub use std::mem::{size_of, zeroed};

pub use crate::sdrad_api_import::*;
pub use crate::sdrad_stack_write_read::*;
pub use crate::sdrad_vec_allocator::*;

pub mod sdrad_api_import;
pub mod sdrad_stack_write_read;
pub mod sdrad_vec_allocator;

pub static SDRAD_FFI_DEFAULT_UDI: u64 = 1;
pub static SDRAD_SUCCESSFUL_RETURNED: i32 = -1;

#[macro_export]
macro_rules! sdrad_define__real_func {
	(fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block) => {
		paste! {
			fn [<__real_$f>]($($x)*) $(->$rettype)? {
				unsafe{
					$body
				}
			}
		}
	}
}

#[macro_export]
macro_rules! sdrad_define__wrap_func {
	($udi:expr, fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block ) => {
		paste! {
			#[inline(never)]
			pub fn [<__wrap_$f>]() {
				unsafe{
					sdrad_pull_args_run!($udi, fn $f($($x)*) $(->$rettype)?);
				}
			}
		}
	};
}

#[macro_export]
macro_rules! sdrad_global_create_function {
	($udi:expr,  fn $f:ident() $body:block, $rewind:expr) => {
		unsafe {
			sdrad_init($udi, Flags::SDRAD_EXECUTION_DOMAIN | Flags::SDRAD_NONISOLATED_DOMAIN | Flags::SDRAD_RETURN_TO_CURRENT);
			sdrad_enter($udi);
			paste!{[<__wrap_$f>]()};
			sdrad_exit();
			sdrad_deinit($udi, 0);
		}
	};
	($udi:expr, fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block, $rewind:expr) => {
		unsafe {
			let mut ret : i32 = sdrad_init($udi, Flags::SDRAD_EXECUTION_DOMAIN | Flags::SDRAD_NONISOLATED_DOMAIN | Flags::SDRAD_RETURN_TO_CURRENT);
			if  ret == SDRAD_SUCCESSFUL_RETURNED || ret == SDRAD_WARNING_SAVE_EC {
				let mut rsp_ptr : *mut i64 = std::ptr::null_mut();
				sdrad_push_args!($udi, rsp_ptr, fn $f($($x)*));
				sdrad_enter($udi);
				paste!{[<__wrap_$f>]()};
				sdrad_exit();
				sdrad_deinit($udi, 0);
				sdrad_collect_ret_try!($udi, rsp_ptr, fn $f($($x)*) $(->$rettype)? )

			} else {
			 	panic!("Domain Violation");
			}
		};
	};
}

#[macro_export]
macro_rules! sandbox {
	/* Without UDI - Without Rewind Return Value */
	(fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body);
		fn $f($($x)*) $(->$rettype)? {
			sdrad_global_create_function!(SDRAD_FFI_DEFAULT_UDI,  fn $f($($x)*) $(->$rettype)? $body, Err("domain_violation"))
		}
	};

	(pub fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)?  $body);
		pub fn $f($($x)*) $(->$rettype)? {
			sdrad_global_create_function!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body, Err("domain_violation"))
		}
	};

	/* With UDI Without Rewind Return Value*/
	($udi:expr, fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!($udi, fn $f($($x)*) $(->$rettype)? $body);
		fn $f($($x)*) $(->$rettype)? {
			sdrad_global_create_function!($udi, fn $f($($x)*) $(->$rettype)? $body, Err("domain_violation"))
		}
	};

	($udi:expr, pub fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!($udi, fn $f($($x)*) $(->$rettype)? $body);
		pub fn $f($($x)*) $(->$rettype)? {
			sdrad_global_create_function!($udi, fn $f($($x)*) $(->$rettype)? $body, Err("domain_violation"))
		}
	};


	/* With UDI and With Return Value*/
	($udi:expr, fn $f:ident($($x:tt)*) $(->$rettype:ty)? $body:block, $rewind_ret:expr) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!($udi, $f($($x)*) $(->$rettype)? $body);
		fn $f($($x)*) $(->$rettype)? {
			sdrad_global_create_function!($udi, $f($($x)*) $(->$rettype)? $body, $rewind_ret)
		}
	};

	($udi:expr,  pub fn $f:ident($($x:tt)*)  $(->$rettype:ty)? $body:block, $rewind_ret:expr) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!($udi, fn $f($($x)*) $(->$rettype)? $body);
		pub fn $f($($x)*) $(->$rettype)?  {
			sdrad_global_create_function!($udi, fn $f($($x)*) $(->$rettype)? $body,  $rewind_ret)
		}
	};

	(pub fn $f:ident($($x:tt)*)  $(->$rettype:ty)? $body:block, $rewind_ret:expr) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body);
		pub fn $f($($x)*) $(->$rettype)?  {
			sdrad_global_create_function!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body,  $rewind_ret)
		}
	};

	(fn $f:ident($($x:tt)*)  $(->$rettype:ty)? $body:block, $rewind_ret:expr) => {
		sdrad_define__real_func!(fn $f($($x)*) $(->$rettype)? $body);
		sdrad_define__wrap_func!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body);
		pub fn $f($($x)*) $(->$rettype)?  {
			sdrad_global_create_function!(SDRAD_FFI_DEFAULT_UDI, fn $f($($x)*) $(->$rettype)? $body,  $rewind_ret)
		}
	};
}

#[macro_export]
macro_rules! sdrad_wrap_global {
	(#[$link_flag:meta] extern { $(static mut $name:ident: $var_type:ty;)+ }) => {
		// re-gengerate the extern block
		#[$link_flag]
		extern {
			$(
			static mut $name: $var_type;
			)+
		}

		fn sdrad_push_global(rsp_ptr: i64) {
			unsafe {
				$(
					sdrad_put_var(rsp_ptr as *mut &i32, &$name);
				)+
			}
		}

		fn sdrad_pull_global(rsp_ptr: i64) {
			$(
				unsafe{
					$name = sdrad_restore_var(rsp_ptr as *mut i32);
				}
			)+
		}
	}
}

#[macro_export]
macro_rules! sdrad_strip_types {
	(($head:ident : &mut $var_type:ty, $($tail:tt)+) -> ($f:ident($($body:tt)*))) => (sdrad_strip_types!(($($tail)+) -> ($f($($body)* &mut $head,))));
	(($head:ident : &mut $var_type:ty) -> ($f:ident($($body:tt)*))) => ($f($($body)* &mut $head));

	(($head:ident : &$var_type:ty, $($tail:tt)+) -> ($f:ident($($body:tt)*))) => (sdrad_strip_types!(($($tail)+) -> ($f($($body)* &$head,))));
	(($head:ident : &$var_type:ty) -> ($f:ident($($body:tt)*))) => ($f($($body)*  &$head));

	((mut $head:ident : $var_type:ty, $($tail:tt)+) -> ($f:ident($($body:tt)*))) => (sdrad_strip_types!(($($tail)+) -> ($f($($body)* mut $head,))));
	((mut $head:ident : $var_type:ty) -> ($f:ident($($body:tt)*))) => ($f($($body)* $head));

	(($head:ident : $var_type:ty, $($tail:tt)+) -> ($f:ident($($body:tt)*))) => (sdrad_strip_types!(($($tail)+) -> ($f($($body)* $head,))));
	(($head:ident : $var_type:ty) -> ($f:ident($($body:tt)*))) => ($f($($body)* $head));

	($f:ident($($tail:tt)+)) => (sdrad_strip_types!(($($tail)+) -> ($f())));
	($f:ident()) => ($f());
}

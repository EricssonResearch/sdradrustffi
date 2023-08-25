//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

use std::fmt;
use std::mem::size_of;
use std::ptr;

use libc::c_void;

#[derive(Debug, Clone, PartialEq)]
pub struct StackBufError {
    base: *const c_void,
}

impl fmt::Display for StackBufError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid base address for stack buffer {:?}", self.base)
    }
}

#[derive(Debug, PartialEq)]
pub struct StackBufReader {
    ptr: *mut c_void,
}

#[derive(Debug, PartialEq)]
pub struct StackBufWriter {
    base: *const c_void,
    ptr: *mut c_void,
}

impl StackBufReader {
    pub fn new(base: *mut c_void) -> Result<StackBufReader, StackBufError> {
        if base as *const c_void == std::ptr::null() {
            Err(StackBufError { base })
        } else {
            Ok(StackBufReader { ptr: base })
        }
    }

    pub unsafe fn retrieve<T>(&mut self) -> T
    where
        T: Sized,
    {
        self.ptr = self.ptr.offset(-(size_of::<T>() as isize));
        ptr::read_volatile(self.ptr as *const T) //TODO
    }
}

impl StackBufWriter {
    pub fn new(base: *mut c_void) -> Result<StackBufWriter, StackBufError> {
        if base as *const c_void == std::ptr::null() {
            Err(StackBufError { base })
        } else {
            Ok(StackBufWriter { base, ptr: base })
        }
    }

    pub unsafe fn put<T>(&mut self, arg: T) -> *const c_void
    where
        T: Sized,
    {
        self.ptr = self.ptr.offset(-(size_of::<T>() as isize));
        ptr::write_volatile(self.ptr as *mut T, arg); //TODO
        self.ptr
    }

    pub fn get_size(&mut self) -> usize {
        self.base as usize - self.ptr as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::{alloc, dealloc, Layout};

    #[test]
    fn test_init_writer_with_null_base_pointer() {
        let result = StackBufWriter::new(std::ptr::null::<i8>() as *mut c_void);
        let expected = Err(StackBufError {
            base: std::ptr::null(),
        });
        assert_eq!(expected, result);
    }

    #[test]
    fn test_init_reader_with_null_base_pointer() {
        let result = StackBufReader::new(std::ptr::null::<i8>() as *mut c_void);
        let expected = Err(StackBufError {
            base: std::ptr::null(),
        });
        assert_eq!(expected, result);
    }

    //     #[test]
    //     fn test_writing_and_reading_with_uint() {
    //         unsafe {
    //             let layout = Layout::from_size_align(62, 1).expect("layout if valid");
    //             let ptr = alloc(layout);

    //             let base = ptr.offset(layout.size().try_into().unwrap());

    //             let mut sw = StackBufWriter::new(base as *mut c_void).unwrap();

    //             assert!(*(sw.put(u8::MIN) as *const u8) == u8::MIN);
    //             assert!(sw.get_size() == 1);
    //             assert!(*(sw.put(u8::MAX) as *const u8) == u8::MAX);
    //             assert!(sw.get_size() == 2);
    //             assert!(*(sw.put(u16::MIN) as *const u16) == u16::MIN);
    //             assert!(sw.get_size() == 4);
    //             assert!(*(sw.put(u16::MAX) as *const u16) == u16::MAX);
    //             assert!(sw.get_size() == 6);
    //             assert!(*(sw.put(u32::MIN) as *const u32) == u32::MIN);
    //             assert!(sw.get_size() == 10);
    //             assert!(*(sw.put(u32::MAX) as *const u32) == u32::MAX);
    //             assert!(sw.get_size() == 14);
    //             assert!(*(sw.put(u64::MIN) as *const u64) == u64::MIN);
    //             assert!(sw.get_size() == 22);
    //             assert!(*(sw.put(u64::MAX) as *const u64) == u64::MAX);
    //             assert!(sw.get_size() == 30);
    //             assert!(*(sw.put(u128::MIN) as *const u128) == u128::MIN);
    //             assert!(sw.get_size() == 46);
    //             assert!(*(sw.put(u128::MAX) as *const u128) == u128::MAX);
    //             assert!(sw.get_size() == 62);

    //             let mut sr = StackBufReader::new(base as *mut c_void).unwrap();

    //             let u8_min: u8 = sr.retrieve::<u8>();
    //             let u8_max: u8 = sr.retrieve::<u8>();
    //             let u16_min: u16 = sr.retrieve::<u16>();
    //             let u16_max: u16 = sr.retrieve::<u16>();
    //             let u32_min: u32 = sr.retrieve::<u32>();
    //             let u32_max: u32 = sr.retrieve::<u32>();
    //             let u64_min: u64 = sr.retrieve::<u64>();
    //             let u64_max: u64 = sr.retrieve::<u64>();
    //             let u128_min: u128 = sr.retrieve::<u128>();
    //             let u128_max: u128 = sr.retrieve::<u128>();

    //             assert!(u8_min == u8::MIN);
    //             assert!(u8_max == u8::MAX);
    //             assert!(u16_min == u16::MIN);
    //             assert!(u16_max == u16::MAX);
    //             assert!(u32_min == u32::MIN);
    //             assert!(u32_max == u32::MAX);
    //             assert!(u64_min == u64::MIN);
    //             assert!(u64_max == u64::MAX);
    //             assert!(u128_min == u128::MIN);
    //             assert!(u128_max == u128::MAX);

    //             dealloc(ptr, layout);
    //         }
    //    }
}

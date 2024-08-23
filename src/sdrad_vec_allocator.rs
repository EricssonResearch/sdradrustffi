//  @author Merve Gulmez (merve.gulmez@ericsson.com)
//  @version 0.1
//  @date 2023-08-31
//  @copyright © Ericsson AB 2023
//  SPDX-License-Identifier: BSD 3-Clause

use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};
use libc::c_void;

use crate::sdrad_api_import::*;

pub struct SdradAllocator {
    pub data_domain_id: u64,
}

unsafe impl Allocator for SdradAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
            // SAFETY: `layout` is non-zero in size,
            size => unsafe {
                let raw_ptr = sdrad_malloc(self.data_domain_id, layout.size());

                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            },
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        sdrad_free(self.data_domain_id, _ptr.as_ptr() as *mut c_void);
    }

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        match new_layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(old_layout.dangling(), 0)),
            // SAFETY: `layout` is non-zero in size,
            size => unsafe {
                let raw_ptr = sdrad_realloc(
                    self.data_domain_id,
                    ptr.as_ptr() as *mut c_void,
                    new_layout.size() as isize,
                );

                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            },
        }
    }
}

pub struct SdradAllocatorFake {
    pub data_domain_id: u64,
}

unsafe impl Allocator for SdradAllocatorFake {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        match layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(layout.dangling(), 0)),
            // SAFETY: `layout` is non-zero in size,
            size => unsafe {
                let raw_ptr = sdrad_malloc(self.data_domain_id, layout.size());

                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            },
        }
    }

    unsafe fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {}

    unsafe fn grow(
        &self,
        ptr: NonNull<u8>,
        old_layout: Layout,
        new_layout: Layout,
    ) -> Result<NonNull<[u8]>, AllocError> {
        match new_layout.size() {
            0 => Ok(NonNull::slice_from_raw_parts(old_layout.dangling(), 0)),
            // SAFETY: `layout` is non-zero in size,
            size => unsafe {
                let raw_ptr = sdrad_realloc(
                    self.data_domain_id,
                    ptr.as_ptr() as *mut c_void,
                    new_layout.size() as isize,
                );

                let ptr = NonNull::new(raw_ptr).ok_or(AllocError)?;
                Ok(NonNull::slice_from_raw_parts(ptr, size))
            },
        }
    }
}

extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct wordexp_t {
    pub we_wordc: libc::size_t,
    pub we_wordv: *const *const libc::c_char,
    pub we_offs: libc::size_t,
}

impl std::ops::Drop for wordexp_t {
    fn drop(&mut self)
    {
        unsafe { wordfree(self); }
    }
}

extern "C" {
    pub fn wordexp(_: *const libc::c_char, _: &mut wordexp_t, _: i32) -> i32;
    pub fn wordfree(_: &mut wordexp_t);
}

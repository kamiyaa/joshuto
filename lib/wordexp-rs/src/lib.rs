extern crate libc;

use std::ffi::CString;
use std::ffi::CStr;

mod ll;

trait ToCStr {
    fn to_c_str(&self) -> CString;
}

impl <'a>ToCStr for &'a str {
    fn to_c_str(&self) -> CString {
        CString::new(*self).unwrap()
    }
}

pub struct Wordexp {
    pub we_wordc: libc::size_t,
    pub we_wordv: *const *const libc::c_char,
    pub we_offs: libc::size_t,
    wordexp_ref: ll::wordexp_t,
}

impl std::ops::Drop for Wordexp {
    fn drop(&mut self) {}
}

pub fn wordexp<'a>(s: &str, flags: i32) -> Vec<String>
{

    let mut wordexp = ll::wordexp_t {
        we_wordc: 0,
        we_wordv: libc::PT_NULL as *const *const libc::c_char,
        we_offs: 0,
        };

    unsafe {
        ll::wordexp(s.to_c_str().as_ptr(), &mut wordexp, flags);
        let we_wordc: usize = wordexp.we_wordc as usize;
        let mut we_wordv: Vec<String> = Vec::with_capacity(we_wordc);

        let ptr: *const *const libc::c_char = wordexp.we_wordv;

        for i in 0..we_wordc {
            let cstr = CStr::from_ptr(*ptr.offset(i as isize));
            if let Ok(cstr) = cstr.to_str() {
                we_wordv.push(String::from(cstr));
            }
        }
        we_wordv
    }
}

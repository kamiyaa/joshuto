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

#[allow(dead_code)]
pub struct Wordexp<'a> {
    we_wordv: Vec<&'a str>,
    counter: usize,
    wordexp_ref: ll::wordexp_t,
}

impl<'a> Wordexp<'a> {

    pub fn new(wordexp_ref: ll::wordexp_t) -> Self
    {
        let we_wordc: usize = wordexp_ref.we_wordc as usize;
        let mut we_wordv: Vec<&str> = Vec::with_capacity(we_wordc);
        unsafe {
            let ptr: *const *const libc::c_char = wordexp_ref.we_wordv;

            for i in 0..we_wordc {
                let cstr = CStr::from_ptr(*ptr.offset(i as isize));
                if let Ok(s) = cstr.to_str() {
                    we_wordv.push(s);
                }
            }
        }

        Wordexp {
            we_wordv,
            counter: 0,
            wordexp_ref,
        }
    }
}

impl<'a> std::ops::Drop for Wordexp<'a> {
    fn drop(&mut self) {
        drop(&self.wordexp_ref);
    }
}

impl<'a> std::iter::Iterator for Wordexp<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str>
    {
        if self.counter >= self.we_wordv.len() {
            self.counter = 0;
            None
        } else {
            let item = self.we_wordv[self.counter];
            self.counter = self.counter + 1;
            Some(item)
        }
    }
}

pub fn wordexp<'a>(s: &str, flags: i32) -> Wordexp
{
    let mut wordexp = ll::wordexp_t {
        we_wordc: 0,
        we_wordv: libc::PT_NULL as *const *const libc::c_char,
        we_offs: 0,
        };

    unsafe {
        ll::wordexp(s.to_c_str().as_ptr(), &mut wordexp, flags);
    }

    Wordexp::new(wordexp)
}

extern crate libc;

#[repr(C)]
#[derive(Clone, Debug)]
pub struct wordexp_t {
    pub we_wordc: libc::size_t,
    pub we_wordv: *const *const libc::c_char,
    pub we_offs: libc::size_t,
}

impl std::ops::Drop for wordexp_t {
    fn drop(&mut self) {
        unsafe {
            wordfree(self);
        }
    }
}

extern "C" {
    /*
        pub static WRDE_APPEND: i32;
        pub static WRDE_DOOFFS: i32;
        pub static WRDE_NOCMD: i32;
        pub static WRDE_REUSE: i32;
        pub static WRDE_SHOWERR: i32;
        pub static WRDE_UNDEF: i32;

        pub static WRDE_BADCHAR: i32;
        pub static WRDE_BADVAL: i32;
        pub static WRDE_CMDSUB: i32;
        pub static WRDE_NOSPACE: i32;
        pub static WRDE_SYNTAX: i32;
    */

    pub fn wordexp(_: *const libc::c_char, _: &mut wordexp_t, _: i32) -> i32;
    pub fn wordfree(_: &mut wordexp_t);
}

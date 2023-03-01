extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn memrev16(mut p: *mut libc::c_void) {
    let mut x: *mut libc::c_uchar = p as *mut libc::c_uchar;
    let mut t: libc::c_uchar = 0;
    t = *x.offset(0 as libc::c_int as isize);
    *x.offset(0 as libc::c_int as isize) = *x.offset(1 as libc::c_int as isize);
    *x.offset(1 as libc::c_int as isize) = t;
}
#[no_mangle]
pub unsafe extern "C" fn memrev32(mut p: *mut libc::c_void) {
    let mut x: *mut libc::c_uchar = p as *mut libc::c_uchar;
    let mut t: libc::c_uchar = 0;
    t = *x.offset(0 as libc::c_int as isize);
    *x.offset(0 as libc::c_int as isize) = *x.offset(3 as libc::c_int as isize);
    *x.offset(3 as libc::c_int as isize) = t;
    t = *x.offset(1 as libc::c_int as isize);
    *x.offset(1 as libc::c_int as isize) = *x.offset(2 as libc::c_int as isize);
    *x.offset(2 as libc::c_int as isize) = t;
}
#[no_mangle]
pub unsafe extern "C" fn memrev64(mut p: *mut libc::c_void) {
    let mut x: *mut libc::c_uchar = p as *mut libc::c_uchar;
    let mut t: libc::c_uchar = 0;
    t = *x.offset(0 as libc::c_int as isize);
    *x.offset(0 as libc::c_int as isize) = *x.offset(7 as libc::c_int as isize);
    *x.offset(7 as libc::c_int as isize) = t;
    t = *x.offset(1 as libc::c_int as isize);
    *x.offset(1 as libc::c_int as isize) = *x.offset(6 as libc::c_int as isize);
    *x.offset(6 as libc::c_int as isize) = t;
    t = *x.offset(2 as libc::c_int as isize);
    *x.offset(2 as libc::c_int as isize) = *x.offset(5 as libc::c_int as isize);
    *x.offset(5 as libc::c_int as isize) = t;
    t = *x.offset(3 as libc::c_int as isize);
    *x.offset(3 as libc::c_int as isize) = *x.offset(4 as libc::c_int as isize);
    *x.offset(4 as libc::c_int as isize) = t;
}
#[no_mangle]
pub unsafe extern "C" fn intrev16(mut v: uint16_t) -> uint16_t {
    memrev16(&mut v as *mut uint16_t as *mut libc::c_void);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn intrev32(mut v: uint32_t) -> uint32_t {
    memrev32(&mut v as *mut uint32_t as *mut libc::c_void);
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn intrev64(mut v: uint64_t) -> uint64_t {
    memrev64(&mut v as *mut uint64_t as *mut libc::c_void);
    return v;
}

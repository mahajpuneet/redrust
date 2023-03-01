extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn crc64(crc: uint64_t, s: *const libc::c_uchar, l: uint64_t) -> uint64_t;
}
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C" fn redisGitSHA1() -> *mut libc::c_char {
    return b"00000000\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn redisGitDirty() -> *mut libc::c_char {
    return b"0\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn redisBuildId() -> uint64_t {
    let mut buildid: *mut libc::c_char = b"7.0.8debian-1675670694000000000\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    return crc64(
        0 as libc::c_int as uint64_t,
        buildid as *mut libc::c_uchar,
        strlen(buildid),
    );
}
#[no_mangle]
pub unsafe extern "C" fn redisBuildIdString() -> *mut libc::c_char {
    static mut buf: [libc::c_char; 32] = [0; 32];
    static mut cached: libc::c_int = 0 as libc::c_int;
    if cached == 0 {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"%llx\0" as *const u8 as *const libc::c_char,
            redisBuildId() as libc::c_ulonglong,
        );
        cached = 1 as libc::c_int;
    }
    return buf.as_mut_ptr();
}

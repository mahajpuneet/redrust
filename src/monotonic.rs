extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
}
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type monotime = uint64_t;
pub type monotonic_clock_type = libc::c_uint;
pub const MONOTONIC_CLOCK_HW: monotonic_clock_type = 1;
pub const MONOTONIC_CLOCK_POSIX: monotonic_clock_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
#[no_mangle]
pub static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime> = None;
static mut monotonic_info_string: [libc::c_char; 32] = [0; 32];
unsafe extern "C" fn getMonotonicUs_posix() -> monotime {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    clock_gettime(1 as libc::c_int, &mut ts);
    return (ts.tv_sec as uint64_t)
        .wrapping_mul(1000000 as libc::c_int as libc::c_ulong)
        .wrapping_add(
            (ts.tv_nsec / 1000 as libc::c_int as libc::c_long) as libc::c_ulong,
        );
}
unsafe extern "C" fn monotonicInit_posix() {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut rc: libc::c_int = clock_gettime(1 as libc::c_int, &mut ts);
    if rc == 0 as libc::c_int {} else {
        __assert_fail(
            b"rc == 0\0" as *const u8 as *const libc::c_char,
            b"monotonic.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 27],
                &[libc::c_char; 27],
            >(b"void monotonicInit_posix()\0"))
                .as_ptr(),
        );
    };
    snprintf(
        monotonic_info_string.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        b"POSIX clock_gettime\0" as *const u8 as *const libc::c_char,
    );
    getMonotonicUs = core::mem::transmute::<
        Option::<unsafe extern "C" fn() -> monotime>,
        Option::<unsafe extern "C" fn() -> monotime>,
    >(
        Some(
            core::mem::transmute::<
                unsafe extern "C" fn() -> monotime,
                unsafe extern "C" fn() -> monotime,
            >(getMonotonicUs_posix),
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn monotonicInit() -> *const libc::c_char {
    if getMonotonicUs.is_none() {
        monotonicInit_posix();
    }
    return monotonic_info_string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn monotonicInfoString() -> *const libc::c_char {
    return monotonic_info_string.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn monotonicGetType() -> monotonic_clock_type {
    if getMonotonicUs
        == core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> monotime>,
            Option::<unsafe extern "C" fn() -> monotime>,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> monotime,
                    unsafe extern "C" fn() -> monotime,
                >(getMonotonicUs_posix),
            ),
        )
    {
        return MONOTONIC_CLOCK_POSIX;
    }
    return MONOTONIC_CLOCK_HW;
}

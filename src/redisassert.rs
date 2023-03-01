extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn abort() -> !;
    fn raise(__sig: libc::c_int) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
/* #[no_mangle]
pub unsafe extern "C" fn _serverAssert(
    mut estr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    fprintf(stderr, b"=== ASSERTION FAILED ===\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"==> %s:%d '%s' is not true\0" as *const u8 as *const libc::c_char,
        file,
        line,
        estr,
    );
    raise(11 as libc::c_int);
} 
#[no_mangle]
pub unsafe extern "C" fn _serverPanic(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    fprintf(
        stderr,
        b"------------------------------------------------\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"!!! Software Failure. Press left mouse button to continue\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Guru Meditation: %s #%s:%d\0" as *const u8 as *const libc::c_char,
        msg,
        file,
        line,
    );
    abort();
}
*/
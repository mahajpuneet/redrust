extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtold(_: *const libc::c_char, _: *mut *mut libc::c_char) -> f64;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn rand() -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn llrint(_: libc::c_double) -> libc::c_longlong;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn rmdir(__path: *const libc::c_char) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    static mut timezone: libc::c_long;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sha256_init(ctx: *mut SHA256_CTX);
    fn sha256_update(ctx: *mut SHA256_CTX, data: *const BYTE, len: size_t);
    fn sha256_final(ctx: *mut SHA256_CTX, hash: *mut BYTE);
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino64_t,
    pub d_off: __off64_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
pub type ld2string_mode = libc::c_uint;
pub const LD_STR_HEX: ld2string_mode = 2;
pub const LD_STR_HUMAN: ld2string_mode = 1;
pub const LD_STR_AUTO: ld2string_mode = 0;
pub type BYTE = uint8_t;
pub type WORD = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256_CTX {
    pub data: [BYTE; 64],
    pub datalen: WORD,
    pub bitlen: libc::c_ulonglong,
    pub state: [WORD; 8],
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(0 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn stringmatchlen(
    mut pattern: *const libc::c_char,
    mut patternLen: libc::c_int,
    mut string: *const libc::c_char,
    mut stringLen: libc::c_int,
    mut nocase: libc::c_int,
) -> libc::c_int {
    while patternLen != 0 && stringLen != 0 {
        let mut current_block_74: u64;
        match *pattern.offset(0 as libc::c_int as isize) as libc::c_int {
            42 => {
                while patternLen != 0
                    && *pattern.offset(1 as libc::c_int as isize) as libc::c_int
                        == '*' as i32
                {
                    pattern = pattern.offset(1);
                    patternLen -= 1;
                }
                if patternLen == 1 as libc::c_int {
                    return 1 as libc::c_int;
                }
                while stringLen != 0 {
                    if stringmatchlen(
                        pattern.offset(1 as libc::c_int as isize),
                        patternLen - 1 as libc::c_int,
                        string,
                        stringLen,
                        nocase,
                    ) != 0
                    {
                        return 1 as libc::c_int;
                    }
                    string = string.offset(1);
                    stringLen -= 1;
                }
                return 0 as libc::c_int;
            }
            63 => {
                string = string.offset(1);
                stringLen -= 1;
                current_block_74 = 5028470053297453708;
            }
            91 => {
                let mut not: libc::c_int = 0;
                let mut match_0: libc::c_int = 0;
                pattern = pattern.offset(1);
                patternLen -= 1;
                not = (*pattern.offset(0 as libc::c_int as isize) as libc::c_int
                    == '^' as i32) as libc::c_int;
                if not != 0 {
                    pattern = pattern.offset(1);
                    patternLen -= 1;
                }
                match_0 = 0 as libc::c_int;
                loop {
                    if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                        == '\\' as i32 && patternLen >= 2 as libc::c_int
                    {
                        pattern = pattern.offset(1);
                        patternLen -= 1;
                        if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                            == *string.offset(0 as libc::c_int as isize) as libc::c_int
                        {
                            match_0 = 1 as libc::c_int;
                        }
                    } else {
                        if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                            == ']' as i32
                        {
                            break;
                        }
                        if patternLen == 0 as libc::c_int {
                            pattern = pattern.offset(-1);
                            patternLen += 1;
                            break;
                        } else if patternLen >= 3 as libc::c_int
                            && *pattern.offset(1 as libc::c_int as isize) as libc::c_int
                                == '-' as i32
                        {
                            let mut start: libc::c_int = *pattern
                                .offset(0 as libc::c_int as isize) as libc::c_int;
                            let mut end: libc::c_int = *pattern
                                .offset(2 as libc::c_int as isize) as libc::c_int;
                            let mut c: libc::c_int = *string
                                .offset(0 as libc::c_int as isize) as libc::c_int;
                            if start > end {
                                let mut t: libc::c_int = start;
                                start = end;
                                end = t;
                            }
                            if nocase != 0 {
                                start = ({
                                    let mut __res: libc::c_int = 0;
                                    if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = start;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_tolower_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            __res = tolower(start);
                                        }
                                    } else {
                                        __res = *(*__ctype_tolower_loc()).offset(start as isize);
                                    }
                                    __res
                                });
                                end = ({
                                    let mut __res: libc::c_int = 0;
                                    if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = end;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_tolower_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            __res = tolower(end);
                                        }
                                    } else {
                                        __res = *(*__ctype_tolower_loc()).offset(end as isize);
                                    }
                                    __res
                                });
                                c = ({
                                    let mut __res: libc::c_int = 0;
                                    if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                        > 1 as libc::c_int as libc::c_ulong
                                    {
                                        if 0 != 0 {
                                            let mut __c: libc::c_int = c;
                                            __res = if __c < -(128 as libc::c_int)
                                                || __c > 255 as libc::c_int
                                            {
                                                __c
                                            } else {
                                                *(*__ctype_tolower_loc()).offset(__c as isize)
                                            };
                                        } else {
                                            __res = tolower(c);
                                        }
                                    } else {
                                        __res = *(*__ctype_tolower_loc()).offset(c as isize);
                                    }
                                    __res
                                });
                            }
                            pattern = pattern.offset(2 as libc::c_int as isize);
                            patternLen -= 2 as libc::c_int;
                            if c >= start && c <= end {
                                match_0 = 1 as libc::c_int;
                            }
                        } else if nocase == 0 {
                            if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                                == *string.offset(0 as libc::c_int as isize) as libc::c_int
                            {
                                match_0 = 1 as libc::c_int;
                            }
                        } else if ({
                            let mut __res: libc::c_int = 0;
                            if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                > 1 as libc::c_int as libc::c_ulong
                            {
                                if 0 != 0 {
                                    let mut __c: libc::c_int = *pattern
                                        .offset(0 as libc::c_int as isize) as libc::c_int;
                                    __res = (if __c < -(128 as libc::c_int)
                                        || __c > 255 as libc::c_int
                                    {
                                        __c
                                    } else {
                                        *(*__ctype_tolower_loc()).offset(__c as isize)
                                    });
                                } else {
                                    __res = tolower(
                                        *pattern.offset(0 as libc::c_int as isize) as libc::c_int,
                                    );
                                }
                            } else {
                                __res = *(*__ctype_tolower_loc())
                                    .offset(
                                        *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                                            as isize,
                                    );
                            }
                            __res
                        })
                            == ({
                                let mut __res: libc::c_int = 0;
                                if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                                    > 1 as libc::c_int as libc::c_ulong
                                {
                                    if 0 != 0 {
                                        let mut __c: libc::c_int = *string
                                            .offset(0 as libc::c_int as isize) as libc::c_int;
                                        __res = (if __c < -(128 as libc::c_int)
                                            || __c > 255 as libc::c_int
                                        {
                                            __c
                                        } else {
                                            *(*__ctype_tolower_loc()).offset(__c as isize)
                                        });
                                    } else {
                                        __res = tolower(
                                            *string.offset(0 as libc::c_int as isize) as libc::c_int,
                                        );
                                    }
                                } else {
                                    __res = *(*__ctype_tolower_loc())
                                        .offset(
                                            *string.offset(0 as libc::c_int as isize) as libc::c_int
                                                as isize,
                                        );
                                }
                                __res
                            })
                        {
                            match_0 = 1 as libc::c_int;
                        }
                    }
                    pattern = pattern.offset(1);
                    patternLen -= 1;
                }
                if not != 0 {
                    match_0 = (match_0 == 0) as libc::c_int;
                }
                if match_0 == 0 {
                    return 0 as libc::c_int;
                }
                string = string.offset(1);
                stringLen -= 1;
                current_block_74 = 5028470053297453708;
            }
            92 => {
                if patternLen >= 2 as libc::c_int {
                    pattern = pattern.offset(1);
                    patternLen -= 1;
                }
                current_block_74 = 3929285555421377241;
            }
            _ => {
                current_block_74 = 3929285555421377241;
            }
        }
        match current_block_74 {
            3929285555421377241 => {
                if nocase == 0 {
                    if *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                        != *string.offset(0 as libc::c_int as isize) as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                } else if ({
                    let mut __res: libc::c_int = 0;
                    if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int = *pattern
                                .offset(0 as libc::c_int as isize) as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int)
                                || __c > 255 as libc::c_int
                            {
                                __c
                            } else {
                                *(*__ctype_tolower_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = tolower(
                                *pattern.offset(0 as libc::c_int as isize) as libc::c_int,
                            );
                        }
                    } else {
                        __res = *(*__ctype_tolower_loc())
                            .offset(
                                *pattern.offset(0 as libc::c_int as isize) as libc::c_int
                                    as isize,
                            );
                    }
                    __res
                })
                    != ({
                        let mut __res: libc::c_int = 0;
                        if core::mem::size_of::<libc::c_int>() as libc::c_ulong
                            > 1 as libc::c_int as libc::c_ulong
                        {
                            if 0 != 0 {
                                let mut __c: libc::c_int = *string
                                    .offset(0 as libc::c_int as isize) as libc::c_int;
                                __res = (if __c < -(128 as libc::c_int)
                                    || __c > 255 as libc::c_int
                                {
                                    __c
                                } else {
                                    *(*__ctype_tolower_loc()).offset(__c as isize)
                                });
                            } else {
                                __res = tolower(
                                    *string.offset(0 as libc::c_int as isize) as libc::c_int,
                                );
                            }
                        } else {
                            __res = *(*__ctype_tolower_loc())
                                .offset(
                                    *string.offset(0 as libc::c_int as isize) as libc::c_int
                                        as isize,
                                );
                        }
                        __res
                    })
                {
                    return 0 as libc::c_int
                }
                string = string.offset(1);
                stringLen -= 1;
            }
            _ => {}
        }
        pattern = pattern.offset(1);
        patternLen -= 1;
        if !(stringLen == 0 as libc::c_int) {
            continue;
        }
        while *pattern as libc::c_int == '*' as i32 {
            pattern = pattern.offset(1);
            patternLen -= 1;
        }
        break;
    }
    if patternLen == 0 as libc::c_int && stringLen == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stringmatch(
    mut pattern: *const libc::c_char,
    mut string: *const libc::c_char,
    mut nocase: libc::c_int,
) -> libc::c_int {
    return stringmatchlen(
        pattern,
        strlen(pattern) as libc::c_int,
        string,
        strlen(string) as libc::c_int,
        nocase,
    );
}
#[no_mangle]
pub unsafe extern "C" fn stringmatchlen_fuzz_test() -> libc::c_int {
    let mut str: [libc::c_char; 32] = [0; 32];
    let mut pat: [libc::c_char; 32] = [0; 32];
    let mut cycles: libc::c_int = 10000000 as libc::c_int;
    let mut total_matches: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = cycles;
        cycles = cycles - 1;
        if !(fresh0 != 0) {
            break;
        }
        let mut strlen_0: libc::c_int = (rand() as libc::c_ulong)
            .wrapping_rem(core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            as libc::c_int;
        let mut patlen: libc::c_int = (rand() as libc::c_ulong)
            .wrapping_rem(core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
            as libc::c_int;
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < strlen_0 {
            str[j as usize] = (rand() % 128 as libc::c_int) as libc::c_char;
            j += 1;
        }
        let mut j_0: libc::c_int = 0 as libc::c_int;
        while j_0 < patlen {
            pat[j_0 as usize] = (rand() % 128 as libc::c_int) as libc::c_char;
            j_0 += 1;
        }
        total_matches
            += stringmatchlen(
                pat.as_mut_ptr(),
                patlen,
                str.as_mut_ptr(),
                strlen_0,
                0 as libc::c_int,
            );
    }
    return total_matches;
}
#[no_mangle]
pub unsafe extern "C" fn memtoull(
    mut p: *const libc::c_char,
    mut err: *mut libc::c_int,
) -> libc::c_ulonglong {
    let mut u: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut mul: libc::c_long = 0;
    let mut val: libc::c_ulonglong = 0;
    let mut digits: libc::c_uint = 0;
    if !err.is_null() {
        *err = 0 as libc::c_int;
    }
    u = p;
    if *u as libc::c_int == '-' as i32 {
        if !err.is_null() {
            *err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    while *u as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*u as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        u = u.offset(1);
    }
    if *u as libc::c_int == '\0' as i32
        || strcasecmp(u, b"b\0" as *const u8 as *const libc::c_char) == 0
    {
        mul = 1 as libc::c_int as libc::c_long;
    } else if strcasecmp(u, b"k\0" as *const u8 as *const libc::c_char) == 0 {
        mul = 1000 as libc::c_int as libc::c_long;
    } else if strcasecmp(u, b"kb\0" as *const u8 as *const libc::c_char) == 0 {
        mul = 1024 as libc::c_int as libc::c_long;
    } else if strcasecmp(u, b"m\0" as *const u8 as *const libc::c_char) == 0 {
        mul = (1000 as libc::c_int * 1000 as libc::c_int) as libc::c_long;
    } else if strcasecmp(u, b"mb\0" as *const u8 as *const libc::c_char) == 0 {
        mul = (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_long;
    } else if strcasecmp(u, b"g\0" as *const u8 as *const libc::c_char) == 0 {
        mul = 1000 as libc::c_long * 1000 as libc::c_int as libc::c_long
            * 1000 as libc::c_int as libc::c_long;
    } else if strcasecmp(u, b"gb\0" as *const u8 as *const libc::c_char) == 0 {
        mul = 1024 as libc::c_long * 1024 as libc::c_int as libc::c_long
            * 1024 as libc::c_int as libc::c_long;
    } else {
        if !err.is_null() {
            *err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    digits = u.offset_from(p) as libc::c_long as libc::c_uint;
    if digits as libc::c_ulong
        >= core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
    {
        if !err.is_null() {
            *err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        p as *const libc::c_void,
        digits as libc::c_ulong,
    );
    buf[digits as usize] = '\0' as i32 as libc::c_char;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *__errno_location() = 0 as libc::c_int;
    val = strtoull(buf.as_mut_ptr(), &mut endptr, 10 as libc::c_int);
    if val == 0 as libc::c_int as libc::c_ulonglong
        && *__errno_location() == 22 as libc::c_int
        || *endptr as libc::c_int != '\0' as i32
    {
        if !err.is_null() {
            *err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    return val.wrapping_mul(mul as libc::c_ulonglong);
}
#[no_mangle]
pub unsafe extern "C" fn mempbrk(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut chars: *const libc::c_char,
    mut charslen: size_t,
) -> *const libc::c_char {
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        let mut n: size_t = 0 as libc::c_int as size_t;
        while n < charslen {
            if *s.offset(j as isize) as libc::c_int
                == *chars.offset(n as isize) as libc::c_int
            {
                return &*s.offset(j as isize) as *const libc::c_char;
            }
            n = n.wrapping_add(1);
        }
        j = j.wrapping_add(1);
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn memmapchars(
    mut s: *mut libc::c_char,
    mut len: size_t,
    mut from: *const libc::c_char,
    mut to: *const libc::c_char,
    mut setlen: size_t,
) -> *mut libc::c_char {
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < len {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < setlen {
            if *s.offset(j as isize) as libc::c_int
                == *from.offset(i as isize) as libc::c_int
            {
                *s.offset(j as isize) = *to.offset(i as isize);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
        j = j.wrapping_add(1);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn digits10(mut v: uint64_t) -> uint32_t {
    if v < 10 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int as uint32_t;
    }
    if v < 100 as libc::c_int as libc::c_ulong {
        return 2 as libc::c_int as uint32_t;
    }
    if v < 1000 as libc::c_int as libc::c_ulong {
        return 3 as libc::c_int as uint32_t;
    }
    if v < 1000000000000 as libc::c_ulong {
        if v < 100000000 as libc::c_ulong {
            if v < 1000000 as libc::c_int as libc::c_ulong {
                if v < 10000 as libc::c_int as libc::c_ulong {
                    return 4 as libc::c_int as uint32_t;
                }
                return (5 as libc::c_int
                    + (v >= 100000 as libc::c_int as libc::c_ulong) as libc::c_int)
                    as uint32_t;
            }
            return (7 as libc::c_int + (v >= 10000000 as libc::c_ulong) as libc::c_int)
                as uint32_t;
        }
        if v < 10000000000 as libc::c_ulong {
            return (9 as libc::c_int + (v >= 1000000000 as libc::c_ulong) as libc::c_int)
                as uint32_t;
        }
        return (11 as libc::c_int + (v >= 100000000000 as libc::c_ulong) as libc::c_int)
            as uint32_t;
    }
    return (12 as libc::c_int as libc::c_uint)
        .wrapping_add(digits10(v.wrapping_div(1000000000000 as libc::c_ulong)));
}
#[no_mangle]
pub unsafe extern "C" fn sdigits10(mut v: int64_t) -> uint32_t {
    if v < 0 as libc::c_int as libc::c_long {
        let mut uv: uint64_t = if v as libc::c_longlong
            != -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
        {
            -v as uint64_t
        } else {
            (9223372036854775807 as libc::c_longlong as uint64_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        };
        return (digits10(uv)).wrapping_add(1 as libc::c_int as libc::c_uint);
    } else {
        return digits10(v as uint64_t)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ll2string(
    mut dst: *mut libc::c_char,
    mut dstlen: size_t,
    mut svalue: libc::c_longlong,
) -> libc::c_int {
    let mut value: libc::c_ulonglong = 0;
    let mut negative: libc::c_int = 0 as libc::c_int;
    if svalue < 0 as libc::c_int as libc::c_longlong {
        if svalue != -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong {
            value = -svalue as libc::c_ulonglong;
        } else {
            value = (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong);
        }
        if dstlen < 2 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        negative = 1 as libc::c_int;
        *dst.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
        dst = dst.offset(1);
        dstlen = dstlen.wrapping_sub(1);
    } else {
        value = svalue as libc::c_ulonglong;
    }
    let mut length: libc::c_int = ull2string(dst, dstlen, value);
    if length == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return length + negative;
}
#[no_mangle]
pub unsafe extern "C" fn ull2string(
    mut dst: *mut libc::c_char,
    mut dstlen: size_t,
    mut value: libc::c_ulonglong,
) -> libc::c_int {
    static mut digits: [libc::c_char; 201] = unsafe {
        *core::mem::transmute::<
            &[u8; 201],
            &[libc::c_char; 201],
        >(
            b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0",
        )
    };
    let mut length: uint32_t = digits10(value as uint64_t);
    if length as libc::c_ulong >= dstlen {
        return 0 as libc::c_int;
    }
    let mut next: uint32_t = length.wrapping_sub(1 as libc::c_int as libc::c_uint);
    *dst
        .offset(
            next.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
        ) = '\0' as i32 as libc::c_char;
    while value >= 100 as libc::c_int as libc::c_ulonglong {
        let i: libc::c_int = value
            .wrapping_rem(100 as libc::c_int as libc::c_ulonglong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulonglong) as libc::c_int;
        value = value.wrapping_div(100 as libc::c_int as libc::c_ulonglong);
        *dst.offset(next as isize) = digits[(i + 1 as libc::c_int) as usize];
        *dst
            .offset(
                next.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = digits[i as usize];
        next = (next as libc::c_uint).wrapping_sub(2 as libc::c_int as libc::c_uint)
            as uint32_t as uint32_t;
    }
    if value < 10 as libc::c_int as libc::c_ulonglong {
        *dst
            .offset(
                next as isize,
            ) = ('0' as i32 as libc::c_uint).wrapping_add(value as uint32_t)
            as libc::c_char;
    } else {
        let mut i_0: libc::c_int = (value as uint32_t)
            .wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_int;
        *dst.offset(next as isize) = digits[(i_0 + 1 as libc::c_int) as usize];
        *dst
            .offset(
                next.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
            ) = digits[i_0 as usize];
    }
    return length as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string2ll(
    mut s: *const libc::c_char,
    mut slen: size_t,
    mut value: *mut libc::c_longlong,
) -> libc::c_int {
    let mut p: *const libc::c_char = s;
    let mut plen: size_t = 0 as libc::c_int as size_t;
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_ulonglong = 0;
    if plen == slen || slen >= 21 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if slen == 1 as libc::c_int as libc::c_ulong
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        if !value.is_null() {
            *value = 0 as libc::c_int as libc::c_longlong;
        }
        return 1 as libc::c_int;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32 {
        negative = 1 as libc::c_int;
        p = p.offset(1);
        plen = plen.wrapping_add(1);
        if plen == slen {
            return 0 as libc::c_int;
        }
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int >= '1' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        v = (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
            as libc::c_ulonglong;
        p = p.offset(1);
        plen = plen.wrapping_add(1);
    } else {
        return 0 as libc::c_int
    }
    while plen < slen
        && *p.offset(0 as libc::c_int as isize) as libc::c_int >= '0' as i32
        && *p.offset(0 as libc::c_int as isize) as libc::c_int <= '9' as i32
    {
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_div(10 as libc::c_int as libc::c_ulonglong)
        {
            return 0 as libc::c_int;
        }
        v = v.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
        if v
            > (9223372036854775807 as libc::c_longlong as libc::c_ulonglong)
                .wrapping_mul(2 as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_ulonglong)
                .wrapping_sub(
                    (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                        as libc::c_ulonglong,
                )
        {
            return 0 as libc::c_int;
        }
        v = v
            .wrapping_add(
                (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as libc::c_ulonglong,
            );
        p = p.offset(1);
        plen = plen.wrapping_add(1);
    }
    if plen < slen {
        return 0 as libc::c_int;
    }
    if negative != 0 {
        if v
            > (-(-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                + 1 as libc::c_int as libc::c_longlong) as libc::c_ulonglong)
                .wrapping_add(1 as libc::c_int as libc::c_ulonglong)
        {
            return 0 as libc::c_int;
        }
        if !value.is_null() {
            *value = v.wrapping_neg() as libc::c_longlong;
        }
    } else {
        if v > 9223372036854775807 as libc::c_longlong as libc::c_ulonglong {
            return 0 as libc::c_int;
        }
        if !value.is_null() {
            *value = v as libc::c_longlong;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string2ull(
    mut s: *const libc::c_char,
    mut value: *mut libc::c_ulonglong,
) -> libc::c_int {
    let mut ll: libc::c_longlong = 0;
    if string2ll(s, strlen(s), &mut ll) != 0 {
        if ll < 0 as libc::c_int as libc::c_longlong {
            return 0 as libc::c_int;
        }
        *value = ll as libc::c_ulonglong;
        return 1 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    *value = strtoull(s, &mut endptr, 10 as libc::c_int);
    if *__errno_location() == 22 as libc::c_int
        || *__errno_location() == 34 as libc::c_int
        || !(*s as libc::c_int != '\0' as i32 && *endptr as libc::c_int == '\0' as i32)
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn string2l(
    mut s: *const libc::c_char,
    mut slen: size_t,
    mut lval: *mut libc::c_long,
) -> libc::c_int {
    let mut llval: libc::c_longlong = 0;
    if string2ll(s, slen, &mut llval) == 0 {
        return 0 as libc::c_int;
    }
    if llval
        < (-(9223372036854775807 as libc::c_long) - 1 as libc::c_long)
            as libc::c_longlong
        || llval > 9223372036854775807 as libc::c_long as libc::c_longlong
    {
        return 0 as libc::c_int;
    }
    *lval = llval as libc::c_long;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn double2ll(
    mut d: libc::c_double,
    mut out: *mut libc::c_longlong,
) -> libc::c_int {
    if d
        < (-(9223372036854775807 as libc::c_longlong)
            / 2 as libc::c_int as libc::c_longlong) as libc::c_double
        || d
            > (9223372036854775807 as libc::c_longlong
                / 2 as libc::c_int as libc::c_longlong) as libc::c_double
    {
        return 0 as libc::c_int;
    }
    let mut ll: libc::c_longlong = d as libc::c_longlong;
    if ll as libc::c_double == d {
        *out = ll;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn d2string(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut value: libc::c_double,
) -> libc::c_int {
    if value.is_nan() as i32 != 0 {
        len = snprintf(buf, len, b"nan\0" as *const u8 as *const libc::c_char) as size_t;
    } else if if value.is_infinite() {
        if value.is_sign_positive() { 1 } else { -1 }
    } else {
        0
    } != 0
    {
        if value < 0 as libc::c_int as libc::c_double {
            len = snprintf(buf, len, b"-inf\0" as *const u8 as *const libc::c_char)
                as size_t;
        } else {
            len = snprintf(buf, len, b"inf\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
    } else if value == 0 as libc::c_int as libc::c_double {
        if 1.0f64 / value < 0 as libc::c_int as libc::c_double {
            len = snprintf(buf, len, b"-0\0" as *const u8 as *const libc::c_char)
                as size_t;
        } else {
            len = snprintf(buf, len, b"0\0" as *const u8 as *const libc::c_char)
                as size_t;
        }
    } else {
        let mut lvalue: libc::c_longlong = 0;
        if double2ll(value, &mut lvalue) != 0 {
            len = ll2string(buf, len, lvalue) as size_t;
        } else {
            len = snprintf(
                buf,
                len,
                b"%.17g\0" as *const u8 as *const libc::c_char,
                value,
            ) as size_t;
        }
    }
    return len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fixedpoint_d2string(
    mut dst: *mut libc::c_char,
    mut dstlen: size_t,
    mut dvalue: libc::c_double,
    mut fractional_digits: libc::c_int,
) -> libc::c_int {
    static mut powers_of_ten: [libc::c_double; 18] = [
        1.0f64,
        10.0f64,
        100.0f64,
        1000.0f64,
        10000.0f64,
        100000.0f64,
        1000000.0f64,
        10000000.0f64,
        100000000.0f64,
        1000000000.0f64,
        10000000000.0f64,
        100000000000.0f64,
        1000000000000.0f64,
        10000000000000.0f64,
        100000000000000.0f64,
        1000000000000000.0f64,
        10000000000000000.0f64,
        100000000000000000.0f64,
    ];
    let mut svalue: libc::c_longlong = 0;
    let mut value: libc::c_ulonglong = 0;
    let mut negative: libc::c_int = 0;
    static mut digitsd: [libc::c_char; 201] = unsafe {
        *core::mem::transmute::<
            &[u8; 201],
            &[libc::c_char; 201],
        >(
            b"00010203040506070809101112131415161718192021222324252627282930313233343536373839404142434445464748495051525354555657585960616263646566676869707172737475767778798081828384858687888990919293949596979899\0",
        )
    };
    let mut ndigits: uint32_t = 0;
    let mut integer_digits: libc::c_int = 0;
    let mut size: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut current_block: u64;
    if !(fractional_digits < 1 as libc::c_int || fractional_digits > 17 as libc::c_int) {
        if !((dstlen as libc::c_int) < fractional_digits + 3 as libc::c_int) {
            if dvalue == 0 as libc::c_int as libc::c_double {
                *dst.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
                *dst.offset(1 as libc::c_int as isize) = '.' as i32 as libc::c_char;
                memset(
                    dst.offset(2 as libc::c_int as isize) as *mut libc::c_void,
                    '0' as i32,
                    fractional_digits as libc::c_ulong,
                );
                *dst
                    .offset(
                        (fractional_digits + 2 as libc::c_int) as isize,
                    ) = '\0' as i32 as libc::c_char;
                return fractional_digits + 2 as libc::c_int;
            }
            svalue = llrint(dvalue * powers_of_ten[fractional_digits as usize]);
            value = 0;
            negative = 0 as libc::c_int;
            if svalue < 0 as libc::c_int as libc::c_longlong {
                if svalue
                    != -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
                {
                    value = -svalue as libc::c_ulonglong;
                } else {
                    value = (9223372036854775807 as libc::c_longlong
                        as libc::c_ulonglong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulonglong);
                }
                if dstlen < 2 as libc::c_int as libc::c_ulong {
                    current_block = 2556275553776289657;
                } else {
                    negative = 1 as libc::c_int;
                    *dst.offset(0 as libc::c_int as isize) = '-' as i32 as libc::c_char;
                    dst = dst.offset(1);
                    dstlen = dstlen.wrapping_sub(1);
                    current_block = 5689001924483802034;
                }
            } else {
                value = svalue as libc::c_ulonglong;
                current_block = 5689001924483802034;
            }
            match current_block {
                2556275553776289657 => {}
                _ => {
                    ndigits = digits10(value as uint64_t);
                    if !(ndigits as libc::c_ulong >= dstlen) {
                        integer_digits = ndigits
                            .wrapping_sub(fractional_digits as libc::c_uint)
                            as libc::c_int;
                        if integer_digits < 1 as libc::c_int {
                            *dst
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '0' as i32 as libc::c_char;
                            integer_digits = 1 as libc::c_int;
                        }
                        *dst
                            .offset(
                                integer_digits as isize,
                            ) = '.' as i32 as libc::c_char;
                        size = integer_digits + 1 as libc::c_int + fractional_digits;
                        memset(
                            dst
                                .offset(integer_digits as isize)
                                .offset(1 as libc::c_int as isize) as *mut libc::c_void,
                            '0' as i32,
                            fractional_digits as libc::c_ulong,
                        );
                        next = size - 1 as libc::c_int;
                        while value >= 100 as libc::c_int as libc::c_ulonglong {
                            let i: libc::c_int = value
                                .wrapping_rem(100 as libc::c_int as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulonglong)
                                as libc::c_int;
                            value = value
                                .wrapping_div(100 as libc::c_int as libc::c_ulonglong);
                            *dst
                                .offset(
                                    next as isize,
                                ) = digitsd[(i + 1 as libc::c_int) as usize];
                            *dst
                                .offset(
                                    (next - 1 as libc::c_int) as isize,
                                ) = digitsd[i as usize];
                            next -= 2 as libc::c_int;
                            if next == integer_digits {
                                next -= 1;
                            }
                        }
                        if value < 10 as libc::c_int as libc::c_ulonglong {
                            *dst
                                .offset(
                                    next as isize,
                                ) = ('0' as i32 as libc::c_uint)
                                .wrapping_add(value as uint32_t) as libc::c_char;
                        } else {
                            let mut i_0: libc::c_int = (value as uint32_t)
                                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                                as libc::c_int;
                            *dst
                                .offset(
                                    next as isize,
                                ) = digitsd[(i_0 + 1 as libc::c_int) as usize];
                            *dst
                                .offset(
                                    (next - 1 as libc::c_int) as isize,
                                ) = digitsd[i_0 as usize];
                        }
                        *dst.offset(size as isize) = '\0' as i32 as libc::c_char;
                        return size + negative;
                    }
                }
            }
        }
    }
    if dstlen > 0 as libc::c_int as libc::c_ulong {
        *dst.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trimDoubleString(
    mut buf: *mut libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if !(strchr(buf, '.' as i32)).is_null() {
        let mut p: *mut libc::c_char = buf
            .offset(len as isize)
            .offset(-(1 as libc::c_int as isize));
        while *p as libc::c_int == '0' as i32 {
            p = p.offset(-1);
            len = len.wrapping_sub(1);
        }
        if *p as libc::c_int == '.' as i32 {
            len = len.wrapping_sub(1);
        }
    }
    *buf.offset(len as isize) = '\0' as i32 as libc::c_char;
    return len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ld2string(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut value: f64,
    mut mode: ld2string_mode,
) -> libc::c_int {
    let mut l: size_t = 0 as libc::c_int as size_t;
    if if value.is_infinite() {
        if value.is_sign_positive() { 1 } else { -1 }
    } else {
        0
    } != 0
    {
        if len < 5 as libc::c_int as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if value > (0 as libc::c_int) as f64 {
            memcpy(
                buf as *mut libc::c_void,
                b"inf\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
            l = 3 as libc::c_int as size_t;
        } else {
            memcpy(
                buf as *mut libc::c_void,
                b"-inf\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                4 as libc::c_int as libc::c_ulong,
            );
            l = 4 as libc::c_int as size_t;
        }
    } else {
        match mode as libc::c_uint {
            0 => {
                l = snprintf(
                    buf,
                    len,
                    b"%.17Lg\0" as *const u8 as *const libc::c_char,
                    value,
                ) as size_t;
                if l.wrapping_add(1 as libc::c_int as libc::c_ulong) > len {
                    return 0 as libc::c_int;
                }
            }
            2 => {
                l = snprintf(
                    buf,
                    len,
                    b"%La\0" as *const u8 as *const libc::c_char,
                    value,
                ) as size_t;
                if l.wrapping_add(1 as libc::c_int as libc::c_ulong) > len {
                    return 0 as libc::c_int;
                }
            }
            1 => {
                l = snprintf(
                    buf,
                    len,
                    b"%.17Lf\0" as *const u8 as *const libc::c_char,
                    value,
                ) as size_t;
                if l.wrapping_add(1 as libc::c_int as libc::c_ulong) > len {
                    return 0 as libc::c_int;
                }
                if !(strchr(buf, '.' as i32)).is_null() {
                    let mut p: *mut libc::c_char = buf
                        .offset(l as isize)
                        .offset(-(1 as libc::c_int as isize));
                    while *p as libc::c_int == '0' as i32 {
                        p = p.offset(-1);
                        l = l.wrapping_sub(1);
                    }
                    if *p as libc::c_int == '.' as i32 {
                        l = l.wrapping_sub(1);
                    }
                }
                if l == 2 as libc::c_int as libc::c_ulong
                    && *buf.offset(0 as libc::c_int as isize) as libc::c_int
                        == '-' as i32
                    && *buf.offset(1 as libc::c_int as isize) as libc::c_int
                        == '0' as i32
                {
                    *buf.offset(0 as libc::c_int as isize) = '0' as i32 as libc::c_char;
                    l = 1 as libc::c_int as size_t;
                }
            }
            _ => return 0 as libc::c_int,
        }
    }
    *buf.offset(l as isize) = '\0' as i32 as libc::c_char;
    return l as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getRandomBytes(mut p: *mut libc::c_uchar, mut len: size_t) {
    static mut seed_initialized: libc::c_int = 0 as libc::c_int;
    static mut seed: [libc::c_uchar; 64] = [0; 64];
    static mut counter: uint64_t = 0 as libc::c_int as uint64_t;
    if seed_initialized == 0 {
        let mut fp: *mut FILE = fopen(
            b"/dev/urandom\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
        if fp.is_null()
            || fread(
                seed.as_mut_ptr() as *mut libc::c_void,
                core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                fp,
            ) != 1 as libc::c_int as libc::c_ulong
        {
            let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
            while (j as libc::c_ulong)
                < core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong
            {
                let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
                gettimeofday(&mut tv, 0 as *mut libc::c_void);
                let mut pid: pid_t = getpid();
                seed[j
                    as usize] = (tv.tv_sec ^ tv.tv_usec ^ pid as libc::c_long
                    ^ fp as libc::c_long) as libc::c_uchar;
                j = j.wrapping_add(1);
            }
        } else {
            seed_initialized = 1 as libc::c_int;
        }
        if !fp.is_null() {
            fclose(fp);
        }
    }
    while len != 0 {
        let mut digest: [libc::c_uchar; 32] = [0; 32];
        let mut kxor: [libc::c_uchar; 64] = [0; 64];
        let mut copylen: libc::c_uint = (if len > 32 as libc::c_int as libc::c_ulong {
            32 as libc::c_int as libc::c_ulong
        } else {
            len
        }) as libc::c_uint;
        memcpy(
            kxor.as_mut_ptr() as *mut libc::c_void,
            seed.as_mut_ptr() as *const libc::c_void,
            core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong
        {
            kxor[i
                as usize] = (kxor[i as usize] as libc::c_int ^ 0x36 as libc::c_int)
                as libc::c_uchar;
            i = i.wrapping_add(1);
        }
        let mut ctx: SHA256_CTX = SHA256_CTX {
            data: [0; 64],
            datalen: 0,
            bitlen: 0,
            state: [0; 8],
        };
        sha256_init(&mut ctx);
        sha256_update(
            &mut ctx,
            kxor.as_mut_ptr() as *const BYTE,
            core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
        sha256_update(
            &mut ctx,
            &mut counter as *mut uint64_t as *mut libc::c_uchar as *const BYTE,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        );
        sha256_final(&mut ctx, digest.as_mut_ptr());
        memcpy(
            kxor.as_mut_ptr() as *mut libc::c_void,
            seed.as_mut_ptr() as *const libc::c_void,
            core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
        let mut i_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i_0 as libc::c_ulong)
            < core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong
        {
            kxor[i_0
                as usize] = (kxor[i_0 as usize] as libc::c_int ^ 0x5c as libc::c_int)
                as libc::c_uchar;
            i_0 = i_0.wrapping_add(1);
        }
        sha256_init(&mut ctx);
        sha256_update(
            &mut ctx,
            kxor.as_mut_ptr() as *const BYTE,
            core::mem::size_of::<[libc::c_uchar; 64]>() as libc::c_ulong,
        );
        sha256_update(
            &mut ctx,
            digest.as_mut_ptr() as *const BYTE,
            32 as libc::c_int as size_t,
        );
        sha256_final(&mut ctx, digest.as_mut_ptr());
        counter = counter.wrapping_add(1);
        memcpy(
            p as *mut libc::c_void,
            digest.as_mut_ptr() as *const libc::c_void,
            copylen as libc::c_ulong,
        );
        len = (len as libc::c_ulong).wrapping_sub(copylen as libc::c_ulong) as size_t
            as size_t;
        p = p.offset(copylen as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getRandomHexChars(mut p: *mut libc::c_char, mut len: size_t) {
    let mut charset: *mut libc::c_char = b"0123456789abcdef\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    let mut j: size_t = 0;
    getRandomBytes(p as *mut libc::c_uchar, len);
    j = 0 as libc::c_int as size_t;
    while j < len {
        *p
            .offset(
                j as isize,
            ) = *charset
            .offset(
                (*p.offset(j as isize) as libc::c_int & 0xf as libc::c_int) as isize,
            );
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn getAbsolutePath(mut filename: *mut libc::c_char) -> sds {
    let mut cwd: [libc::c_char; 1024] = [0; 1024];
    let mut abspath: sds = 0 as *mut libc::c_char;
    let mut relpath: sds = sdsnew(filename);
    relpath = sdstrim(relpath, b" \r\n\t\0" as *const u8 as *const libc::c_char);
    if *relpath.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32 {
        return relpath;
    }
    if (getcwd(
        cwd.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    ))
        .is_null()
    {
        sdsfree(relpath);
        return 0 as sds;
    }
    abspath = sdsnew(cwd.as_mut_ptr());
    if sdslen(abspath) != 0
        && *abspath
            .offset(
                (sdslen(abspath)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                    as isize,
            ) as libc::c_int != '/' as i32
    {
        abspath = sdscat(abspath, b"/\0" as *const u8 as *const libc::c_char);
    }
    while sdslen(relpath) >= 3 as libc::c_int as libc::c_ulong
        && *relpath.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *relpath.offset(1 as libc::c_int as isize) as libc::c_int == '.' as i32
        && *relpath.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
    {
        sdsrange(relpath, 3 as libc::c_int as ssize_t, -(1 as libc::c_int) as ssize_t);
        if sdslen(abspath) > 1 as libc::c_int as libc::c_ulong {
            let mut p: *mut libc::c_char = abspath
                .offset(sdslen(abspath) as isize)
                .offset(-(2 as libc::c_int as isize));
            let mut trimlen: libc::c_int = 1 as libc::c_int;
            while *p as libc::c_int != '/' as i32 {
                p = p.offset(-1);
                trimlen += 1;
            }
            sdsrange(
                abspath,
                0 as libc::c_int as ssize_t,
                -(trimlen + 1 as libc::c_int) as ssize_t,
            );
        }
    }
    abspath = sdscatsds(abspath, relpath);
    sdsfree(relpath);
    return abspath;
}
#[no_mangle]
pub unsafe extern "C" fn getTimeZone() -> libc::c_long {
    return timezone;
}
#[no_mangle]
pub unsafe extern "C" fn pathIsBaseName(mut path: *mut libc::c_char) -> libc::c_int {
    return ((strchr(path, '/' as i32)).is_null()
        && (strchr(path, '\\' as i32)).is_null()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fileExist(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    return (stat(filename, &mut statbuf) == 0 as libc::c_int
        && statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dirExists(mut dname: *mut libc::c_char) -> libc::c_int {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    return (stat(dname, &mut statbuf) == 0 as libc::c_int
        && statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dirCreateIfMissing(
    mut dname: *mut libc::c_char,
) -> libc::c_int {
    if mkdir(dname, 0o755 as libc::c_int as __mode_t) != 0 as libc::c_int {
        if *__errno_location() != 17 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if dirExists(dname) == 0 {
                *__errno_location() = 20 as libc::c_int;
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dirRemove(mut dname: *mut libc::c_char) -> libc::c_int {
    let mut dir: *mut DIR = 0 as *mut DIR;
    let mut stat_entry: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut full_path: [libc::c_char; 4097] = [0; 4097];
    dir = opendir(dname);
    if dir.is_null() {
        return -(1 as libc::c_int);
    }
    loop {
        entry = readdir(dir);
        if entry.is_null() {
            break;
        }
        if strcmp(
            ((*entry).d_name).as_mut_ptr(),
            b".\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                ((*entry).d_name).as_mut_ptr(),
                b"..\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            continue;
        }
        snprintf(
            full_path.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 4097]>() as libc::c_ulong,
            b"%s/%s\0" as *const u8 as *const libc::c_char,
            dname,
            ((*entry).d_name).as_mut_ptr(),
        );
        let mut fd: libc::c_int = open(
            full_path.as_mut_ptr(),
            0 as libc::c_int | 0o4000 as libc::c_int,
        );
        if fd == -(1 as libc::c_int) {
            closedir(dir);
            return -(1 as libc::c_int);
        }
        if fstat(fd, &mut stat_entry) == -(1 as libc::c_int) {
            close(fd);
            closedir(dir);
            return -(1 as libc::c_int);
        }
        close(fd);
        if (stat_entry.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o40000 as libc::c_int as libc::c_uint) as libc::c_int != 0 as libc::c_int
        {
            if dirRemove(full_path.as_mut_ptr()) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        } else if unlink(full_path.as_mut_ptr()) != 0 as libc::c_int {
            closedir(dir);
            return -(1 as libc::c_int);
        }
    }
    if rmdir(dname) != 0 as libc::c_int {
        closedir(dir);
        return -(1 as libc::c_int);
    }
    closedir(dir);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn makePath(
    mut path: *mut libc::c_char,
    mut filename: *mut libc::c_char,
) -> sds {
    return sdscatfmt(
        sdsempty(),
        b"%s/%s\0" as *const u8 as *const libc::c_char,
        path,
        filename,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fsyncFileDir(mut filename: *const libc::c_char) -> libc::c_int {
    let mut temp_filename: [libc::c_char; 4097] = [0; 4097];
    let mut dname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir_fd: libc::c_int = 0;
    if strlen(filename) > 4096 as libc::c_int as libc::c_ulong {
        *__errno_location() = 36 as libc::c_int;
        return -(1 as libc::c_int);
    }
    memcpy(
        temp_filename.as_mut_ptr() as *mut libc::c_void,
        filename as *const libc::c_void,
        (strlen(filename)).wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    dname = dirname(temp_filename.as_mut_ptr());
    dir_fd = open(dname, 0 as libc::c_int);
    if dir_fd == -(1 as libc::c_int) {
        if *__errno_location() == 21 as libc::c_int {
            return 0 as libc::c_int;
        }
        return -(1 as libc::c_int);
    }
    if fdatasync(dir_fd) == -(1 as libc::c_int)
        && !(*__errno_location() == 9 as libc::c_int
            || *__errno_location() == 22 as libc::c_int)
    {
        let mut save_errno: libc::c_int = *__errno_location();
        close(dir_fd);
        *__errno_location() = save_errno;
        return -(1 as libc::c_int);
    }
    close(dir_fd);
    return 0 as libc::c_int;
}

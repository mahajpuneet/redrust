extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listpackEntry {
    pub sval: *mut libc::c_uchar,
    pub slen: uint32_t,
    pub lval: libc::c_longlong,
}
pub type listpackValidateEntryCB = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rand_pick {
    pub index: libc::c_uint,
    pub order: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn lpSafeToAdd(
    mut lp: *mut libc::c_uchar,
    mut add: size_t,
) -> libc::c_int {
    let mut len: size_t = (if !lp.is_null() {
        (*lp.offset(0 as libc::c_int as isize) as uint32_t) << 0 as libc::c_int
            | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
            | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
            | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
    } else {
        0 as libc::c_int as libc::c_uint
    }) as size_t;
    if len.wrapping_add(add) > ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lpStringToInt64(
    mut s: *const libc::c_char,
    mut slen: libc::c_ulong,
    mut value: *mut int64_t,
) -> libc::c_int {
    let mut p: *const libc::c_char = s;
    let mut plen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut negative: libc::c_int = 0 as libc::c_int;
    let mut v: uint64_t = 0;
    if slen == 0 as libc::c_int as libc::c_ulong
        || slen >= 21 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if slen == 1 as libc::c_int as libc::c_ulong
        && *p.offset(0 as libc::c_int as isize) as libc::c_int == '0' as i32
    {
        if !value.is_null() {
            *value = 0 as libc::c_int as int64_t;
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
            as uint64_t;
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
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_div(10 as libc::c_int as libc::c_ulong)
        {
            return 0 as libc::c_int;
        }
        v = (v as libc::c_ulong).wrapping_mul(10 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        if v
            > (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(
                    (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                        as libc::c_ulong,
                )
        {
            return 0 as libc::c_int;
        }
        v = (v as libc::c_ulong)
            .wrapping_add(
                (*p.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32)
                    as libc::c_ulong,
            ) as uint64_t as uint64_t;
        p = p.offset(1);
        plen = plen.wrapping_add(1);
    }
    if plen < slen {
        return 0 as libc::c_int;
    }
    if negative != 0 {
        if v
            > (-(-(9223372036854775807 as libc::c_long)
                - 1 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long)
                as uint64_t)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            return 0 as libc::c_int;
        }
        if !value.is_null() {
            *value = v.wrapping_neg() as int64_t;
        }
    } else {
        if v > 9223372036854775807 as libc::c_long as libc::c_ulong {
            return 0 as libc::c_int;
        }
        if !value.is_null() {
            *value = v as int64_t;
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lpNew(mut capacity: size_t) -> *mut libc::c_uchar {
    let mut lp: *mut libc::c_uchar = zmalloc(
        if capacity > (6 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
            capacity
        } else {
            (6 as libc::c_int + 1 as libc::c_int) as libc::c_ulong
        },
    ) as *mut libc::c_uchar;
    if lp.is_null() {
        return 0 as *mut libc::c_uchar;
    }
    *lp
        .offset(
            0 as libc::c_int as isize,
        ) = (6 as libc::c_int + 1 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *lp
        .offset(
            1 as libc::c_int as isize,
        ) = (6 as libc::c_int + 1 as libc::c_int >> 8 as libc::c_int
        & 0xff as libc::c_int) as libc::c_uchar;
    *lp
        .offset(
            2 as libc::c_int as isize,
        ) = (6 as libc::c_int + 1 as libc::c_int >> 16 as libc::c_int
        & 0xff as libc::c_int) as libc::c_uchar;
    *lp
        .offset(
            3 as libc::c_int as isize,
        ) = (6 as libc::c_int + 1 as libc::c_int >> 24 as libc::c_int
        & 0xff as libc::c_int) as libc::c_uchar;
    *lp
        .offset(
            4 as libc::c_int as isize,
        ) = (0 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    *lp
        .offset(
            5 as libc::c_int as isize,
        ) = (0 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    *lp.offset(6 as libc::c_int as isize) = 0xff as libc::c_int as libc::c_uchar;
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn lpFree(mut lp: *mut libc::c_uchar) {
    zfree(lp as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lpShrinkToFit(
    mut lp: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut size: size_t = ((*lp.offset(0 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as size_t;
    if size < je_malloc_usable_size(lp as *mut libc::c_void) {
        return zrealloc(lp as *mut libc::c_void, size) as *mut libc::c_uchar
    } else {
        return lp
    };
}
#[inline]
unsafe extern "C" fn lpEncodeIntegerGetType(
    mut v: int64_t,
    mut intenc: *mut libc::c_uchar,
    mut enclen: *mut uint64_t,
) {
    if v >= 0 as libc::c_int as libc::c_long && v <= 127 as libc::c_int as libc::c_long {
        *intenc.offset(0 as libc::c_int as isize) = v as libc::c_uchar;
        *enclen = 1 as libc::c_int as uint64_t;
    } else if v >= -(4096 as libc::c_int) as libc::c_long
        && v <= 4095 as libc::c_int as libc::c_long
    {
        if v < 0 as libc::c_int as libc::c_long {
            v = ((1 as libc::c_int as int64_t) << 13 as libc::c_int) + v;
        }
        *intenc
            .offset(
                0 as libc::c_int as isize,
            ) = (v >> 8 as libc::c_int | 0xc0 as libc::c_int as libc::c_long)
            as libc::c_uchar;
        *intenc
            .offset(
                1 as libc::c_int as isize,
            ) = (v & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *enclen = 2 as libc::c_int as uint64_t;
    } else if v >= -(32768 as libc::c_int) as libc::c_long
        && v <= 32767 as libc::c_int as libc::c_long
    {
        if v < 0 as libc::c_int as libc::c_long {
            v = ((1 as libc::c_int as int64_t) << 16 as libc::c_int) + v;
        }
        *intenc.offset(0 as libc::c_int as isize) = 0xf1 as libc::c_int as libc::c_uchar;
        *intenc
            .offset(
                1 as libc::c_int as isize,
            ) = (v & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *intenc
            .offset(
                2 as libc::c_int as isize,
            ) = (v >> 8 as libc::c_int) as libc::c_uchar;
        *enclen = 3 as libc::c_int as uint64_t;
    } else if v >= -(8388608 as libc::c_int) as libc::c_long
        && v <= 8388607 as libc::c_int as libc::c_long
    {
        if v < 0 as libc::c_int as libc::c_long {
            v = ((1 as libc::c_int as int64_t) << 24 as libc::c_int) + v;
        }
        *intenc.offset(0 as libc::c_int as isize) = 0xf2 as libc::c_int as libc::c_uchar;
        *intenc
            .offset(
                1 as libc::c_int as isize,
            ) = (v & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *intenc
            .offset(
                2 as libc::c_int as isize,
            ) = (v >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as libc::c_uchar;
        *intenc
            .offset(
                3 as libc::c_int as isize,
            ) = (v >> 16 as libc::c_int) as libc::c_uchar;
        *enclen = 4 as libc::c_int as uint64_t;
    } else if v >= -(2147483648 as libc::c_long)
        && v <= 2147483647 as libc::c_int as libc::c_long
    {
        if v < 0 as libc::c_int as libc::c_long {
            v = ((1 as libc::c_int as int64_t) << 32 as libc::c_int) + v;
        }
        *intenc.offset(0 as libc::c_int as isize) = 0xf3 as libc::c_int as libc::c_uchar;
        *intenc
            .offset(
                1 as libc::c_int as isize,
            ) = (v & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *intenc
            .offset(
                2 as libc::c_int as isize,
            ) = (v >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as libc::c_uchar;
        *intenc
            .offset(
                3 as libc::c_int as isize,
            ) = (v >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as libc::c_uchar;
        *intenc
            .offset(
                4 as libc::c_int as isize,
            ) = (v >> 24 as libc::c_int) as libc::c_uchar;
        *enclen = 5 as libc::c_int as uint64_t;
    } else {
        let mut uv: uint64_t = v as uint64_t;
        *intenc.offset(0 as libc::c_int as isize) = 0xf4 as libc::c_int as libc::c_uchar;
        *intenc
            .offset(
                1 as libc::c_int as isize,
            ) = (uv & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *intenc
            .offset(
                2 as libc::c_int as isize,
            ) = (uv >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                3 as libc::c_int as isize,
            ) = (uv >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                4 as libc::c_int as isize,
            ) = (uv >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                5 as libc::c_int as isize,
            ) = (uv >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                6 as libc::c_int as isize,
            ) = (uv >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                7 as libc::c_int as isize,
            ) = (uv >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as libc::c_uchar;
        *intenc
            .offset(
                8 as libc::c_int as isize,
            ) = (uv >> 56 as libc::c_int) as libc::c_uchar;
        *enclen = 9 as libc::c_int as uint64_t;
    };
}
#[inline]
unsafe extern "C" fn lpEncodeGetType(
    mut ele: *mut libc::c_uchar,
    mut size: uint32_t,
    mut intenc: *mut libc::c_uchar,
    mut enclen: *mut uint64_t,
) -> libc::c_int {
    let mut v: int64_t = 0;
    if lpStringToInt64(ele as *const libc::c_char, size as libc::c_ulong, &mut v) != 0 {
        lpEncodeIntegerGetType(v, intenc, enclen);
        return 0 as libc::c_int;
    } else {
        if size < 64 as libc::c_int as libc::c_uint {
            *enclen = (1 as libc::c_int as libc::c_uint).wrapping_add(size) as uint64_t;
        } else if size < 4096 as libc::c_int as libc::c_uint {
            *enclen = (2 as libc::c_int as libc::c_uint).wrapping_add(size) as uint64_t;
        } else {
            *enclen = (5 as libc::c_int as libc::c_ulong).wrapping_add(size as uint64_t);
        }
        return 1 as libc::c_int;
    };
}
#[inline]
unsafe extern "C" fn lpEncodeBacklen(
    mut buf: *mut libc::c_uchar,
    mut l: uint64_t,
) -> libc::c_ulong {
    if l <= 127 as libc::c_int as libc::c_ulong {
        if !buf.is_null() {
            *buf.offset(0 as libc::c_int as isize) = l as libc::c_uchar;
        }
        return 1 as libc::c_int as libc::c_ulong;
    } else if l < 16383 as libc::c_int as libc::c_ulong {
        if !buf.is_null() {
            *buf
                .offset(
                    0 as libc::c_int as isize,
                ) = (l >> 7 as libc::c_int) as libc::c_uchar;
            *buf
                .offset(
                    1 as libc::c_int as isize,
                ) = (l & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        }
        return 2 as libc::c_int as libc::c_ulong;
    } else if l < 2097151 as libc::c_int as libc::c_ulong {
        if !buf.is_null() {
            *buf
                .offset(
                    0 as libc::c_int as isize,
                ) = (l >> 14 as libc::c_int) as libc::c_uchar;
            *buf
                .offset(
                    1 as libc::c_int as isize,
                ) = (l >> 7 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    2 as libc::c_int as isize,
                ) = (l & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        }
        return 3 as libc::c_int as libc::c_ulong;
    } else if l < 268435455 as libc::c_int as libc::c_ulong {
        if !buf.is_null() {
            *buf
                .offset(
                    0 as libc::c_int as isize,
                ) = (l >> 21 as libc::c_int) as libc::c_uchar;
            *buf
                .offset(
                    1 as libc::c_int as isize,
                ) = (l >> 14 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    2 as libc::c_int as isize,
                ) = (l >> 7 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    3 as libc::c_int as isize,
                ) = (l & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        }
        return 4 as libc::c_int as libc::c_ulong;
    } else {
        if !buf.is_null() {
            *buf
                .offset(
                    0 as libc::c_int as isize,
                ) = (l >> 28 as libc::c_int) as libc::c_uchar;
            *buf
                .offset(
                    1 as libc::c_int as isize,
                ) = (l >> 21 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    2 as libc::c_int as isize,
                ) = (l >> 14 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    3 as libc::c_int as isize,
                ) = (l >> 7 as libc::c_int & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
            *buf
                .offset(
                    4 as libc::c_int as isize,
                ) = (l & 127 as libc::c_int as libc::c_ulong
                | 128 as libc::c_int as libc::c_ulong) as libc::c_uchar;
        }
        return 5 as libc::c_int as libc::c_ulong;
    };
}
#[inline]
unsafe extern "C" fn lpDecodeBacklen(mut p: *mut libc::c_uchar) -> uint64_t {
    let mut val: uint64_t = 0 as libc::c_int as uint64_t;
    let mut shift: uint64_t = 0 as libc::c_int as uint64_t;
    loop {
        val
            |= ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                & 127 as libc::c_int) as uint64_t) << shift;
        if *p.offset(0 as libc::c_int as isize) as libc::c_int & 128 as libc::c_int == 0
        {
            break;
        }
        shift = (shift as libc::c_ulong).wrapping_add(7 as libc::c_int as libc::c_ulong)
            as uint64_t as uint64_t;
        p = p.offset(-1);
        if shift > 28 as libc::c_int as libc::c_ulong {
            return 18446744073709551615 as libc::c_ulong;
        }
    }
    return val;
}
#[inline]
unsafe extern "C" fn lpEncodeString(
    mut buf: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut len: uint32_t,
) {
    if len < 64 as libc::c_int as libc::c_uint {
        *buf
            .offset(
                0 as libc::c_int as isize,
            ) = (len | 0x80 as libc::c_int as libc::c_uint) as libc::c_uchar;
        memcpy(
            buf.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else if len < 4096 as libc::c_int as libc::c_uint {
        *buf
            .offset(
                0 as libc::c_int as isize,
            ) = (len >> 8 as libc::c_int | 0xe0 as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *buf
            .offset(
                1 as libc::c_int as isize,
            ) = (len & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        memcpy(
            buf.offset(2 as libc::c_int as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len as libc::c_ulong,
        );
    } else {
        *buf.offset(0 as libc::c_int as isize) = 0xf0 as libc::c_int as libc::c_uchar;
        *buf
            .offset(
                1 as libc::c_int as isize,
            ) = (len & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *buf
            .offset(
                2 as libc::c_int as isize,
            ) = (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *buf
            .offset(
                3 as libc::c_int as isize,
            ) = (len >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        *buf
            .offset(
                4 as libc::c_int as isize,
            ) = (len >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
        memcpy(
            buf.offset(5 as libc::c_int as isize) as *mut libc::c_void,
            s as *const libc::c_void,
            len as libc::c_ulong,
        );
    };
}
#[inline]
unsafe extern "C" fn lpCurrentEncodedSizeUnsafe(mut p: *mut libc::c_uchar) -> uint32_t {
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
        == 0 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
        == 0x80 as libc::c_int
    {
        return (1 as libc::c_int
            + (*p.offset(0 as libc::c_int as isize) as libc::c_int
                & 0x3f as libc::c_int)) as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int
        == 0xc0 as libc::c_int
    {
        return 2 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf1 as libc::c_int
    {
        return 3 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf2 as libc::c_int
    {
        return 4 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf3 as libc::c_int
    {
        return 5 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf4 as libc::c_int
    {
        return 9 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xf0 as libc::c_int
        == 0xe0 as libc::c_int
    {
        return (2 as libc::c_int
            + ((*p.offset(0 as libc::c_int as isize) as libc::c_int & 0xf as libc::c_int)
                << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int)) as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf0 as libc::c_int
    {
        return (5 as libc::c_int as libc::c_uint)
            .wrapping_add(
                (*p.offset(1 as libc::c_int as isize) as uint32_t) << 0 as libc::c_int
                    | (*p.offset(2 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int
                    | (*p.offset(3 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int
                    | (*p.offset(4 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int,
            );
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int as uint32_t;
    }
    return 0 as libc::c_int as uint32_t;
}
#[inline]
unsafe extern "C" fn lpCurrentEncodedSizeBytes(mut p: *mut libc::c_uchar) -> uint32_t {
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
        == 0 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
        == 0x80 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int
        == 0xc0 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf1 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf2 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf3 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf4 as libc::c_int
    {
        return 1 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xf0 as libc::c_int
        == 0xe0 as libc::c_int
    {
        return 2 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
        == 0xf0 as libc::c_int
    {
        return 5 as libc::c_int as uint32_t;
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 1 as libc::c_int as uint32_t;
    }
    return 0 as libc::c_int as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn lpSkip(mut p: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut entrylen: libc::c_ulong = lpCurrentEncodedSizeUnsafe(p) as libc::c_ulong;
    entrylen = entrylen.wrapping_add(lpEncodeBacklen(0 as *mut libc::c_uchar, entrylen));
    p = p.offset(entrylen as isize);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn lpNext(
    mut lp: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"p\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            475 as libc::c_int,
        );
        unreachable!();
    };
    p = lpSkip(p);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    lpAssertValidEntry(lp, lpBytes(lp), p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn lpPrev(
    mut lp: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"p\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            486 as libc::c_int,
        );
        unreachable!();
    };
    if p.offset_from(lp) as libc::c_long == 6 as libc::c_int as libc::c_long {
        return 0 as *mut libc::c_uchar;
    }
    p = p.offset(-1);
    let mut prevlen: uint64_t = lpDecodeBacklen(p);
    prevlen = (prevlen as libc::c_ulong)
        .wrapping_add(lpEncodeBacklen(0 as *mut libc::c_uchar, prevlen)) as uint64_t
        as uint64_t;
    p = p.offset(-(prevlen.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize));
    lpAssertValidEntry(lp, lpBytes(lp), p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn lpFirst(mut lp: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = lp.offset(6 as libc::c_int as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    lpAssertValidEntry(lp, lpBytes(lp), p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn lpLast(mut lp: *mut libc::c_uchar) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = lp
        .offset(
            ((*lp.offset(0 as libc::c_int as isize) as uint32_t) << 0 as libc::c_int
                | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | (*lp.offset(2 as libc::c_int as isize) as uint32_t)
                    << 16 as libc::c_int
                | (*lp.offset(3 as libc::c_int as isize) as uint32_t)
                    << 24 as libc::c_int) as isize,
        )
        .offset(-(1 as libc::c_int as isize));
    return lpPrev(lp, p);
}
#[no_mangle]
pub unsafe extern "C" fn lpLength(mut lp: *mut libc::c_uchar) -> libc::c_ulong {
    let mut numele: uint32_t = (*lp.offset(4 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
    if numele != 65535 as libc::c_int as libc::c_uint {
        return numele as libc::c_ulong;
    }
    let mut count: uint32_t = 0 as libc::c_int as uint32_t;
    let mut p: *mut libc::c_uchar = lpFirst(lp);
    while !p.is_null() {
        count = count.wrapping_add(1);
        p = lpNext(lp, p);
    }
    if count < 65535 as libc::c_int as libc::c_uint {
        *lp
            .offset(
                4 as libc::c_int as isize,
            ) = (count & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
        *lp
            .offset(
                5 as libc::c_int as isize,
            ) = (count >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
            as libc::c_uchar;
    }
    return count as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn lpGetWithSize(
    mut p: *mut libc::c_uchar,
    mut count: *mut int64_t,
    mut intbuf: *mut libc::c_uchar,
    mut entry_size: *mut uint64_t,
) -> *mut libc::c_uchar {
    let mut val: int64_t = 0;
    let mut uval: uint64_t = 0;
    let mut negstart: uint64_t = 0;
    let mut negmax: uint64_t = 0;
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"p\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            576 as libc::c_int,
        );
        unreachable!();
    };
    if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
        == 0 as libc::c_int
    {
        negstart = 18446744073709551615 as libc::c_ulong;
        negmax = 0 as libc::c_int as uint64_t;
        uval = (*p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x7f as libc::c_int) as uint64_t;
        if !entry_size.is_null() {
            *entry_size = 2 as libc::c_int as uint64_t;
        }
    } else if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xc0 as libc::c_int
        == 0x80 as libc::c_int
    {
        *count = (*p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0x3f as libc::c_int) as int64_t;
        if !entry_size.is_null() {
            *entry_size = ((1 as libc::c_int as libc::c_long + *count) as libc::c_ulong)
                .wrapping_add(
                    lpEncodeBacklen(
                        0 as *mut libc::c_uchar,
                        (*count + 1 as libc::c_int as libc::c_long) as uint64_t,
                    ),
                );
        }
        return p.offset(1 as libc::c_int as isize);
    } else {
        if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xe0 as libc::c_int
            == 0xc0 as libc::c_int
        {
            uval = ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                & 0x1f as libc::c_int) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int) as uint64_t;
            negstart = (1 as libc::c_int as uint64_t) << 12 as libc::c_int;
            negmax = 8191 as libc::c_int as uint64_t;
            if !entry_size.is_null() {
                *entry_size = 3 as libc::c_int as uint64_t;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int == 0xf1 as libc::c_int
        {
            uval = *p.offset(1 as libc::c_int as isize) as uint64_t
                | (*p.offset(2 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int;
            negstart = (1 as libc::c_int as uint64_t) << 15 as libc::c_int;
            negmax = 65535 as libc::c_int as uint64_t;
            if !entry_size.is_null() {
                *entry_size = 4 as libc::c_int as uint64_t;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int == 0xf2 as libc::c_int
        {
            uval = *p.offset(1 as libc::c_int as isize) as uint64_t
                | (*p.offset(2 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as uint64_t)
                    << 16 as libc::c_int;
            negstart = (1 as libc::c_int as uint64_t) << 23 as libc::c_int;
            negmax = (4294967295 as libc::c_uint >> 8 as libc::c_int) as uint64_t;
            if !entry_size.is_null() {
                *entry_size = 5 as libc::c_int as uint64_t;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int == 0xf3 as libc::c_int
        {
            uval = *p.offset(1 as libc::c_int as isize) as uint64_t
                | (*p.offset(2 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
                | (*p.offset(4 as libc::c_int as isize) as uint64_t)
                    << 24 as libc::c_int;
            negstart = (1 as libc::c_int as uint64_t) << 31 as libc::c_int;
            negmax = 4294967295 as libc::c_uint as uint64_t;
            if !entry_size.is_null() {
                *entry_size = 6 as libc::c_int as uint64_t;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xff as libc::c_int == 0xf4 as libc::c_int
        {
            uval = *p.offset(1 as libc::c_int as isize) as uint64_t
                | (*p.offset(2 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
                | (*p.offset(4 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
                | (*p.offset(5 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
                | (*p.offset(6 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
                | (*p.offset(7 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
                | (*p.offset(8 as libc::c_int as isize) as uint64_t)
                    << 56 as libc::c_int;
            negstart = (1 as libc::c_int as uint64_t) << 63 as libc::c_int;
            negmax = 18446744073709551615 as libc::c_ulong;
            if !entry_size.is_null() {
                *entry_size = 10 as libc::c_int as uint64_t;
            }
        } else if *p.offset(0 as libc::c_int as isize) as libc::c_int
            & 0xf0 as libc::c_int == 0xe0 as libc::c_int
        {
            *count = ((*p.offset(0 as libc::c_int as isize) as libc::c_int
                & 0xf as libc::c_int) << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int) as int64_t;
            if !entry_size.is_null() {
                *entry_size = ((2 as libc::c_int as libc::c_long + *count)
                    as libc::c_ulong)
                    .wrapping_add(
                        lpEncodeBacklen(
                            0 as *mut libc::c_uchar,
                            (*count + 2 as libc::c_int as libc::c_long) as uint64_t,
                        ),
                    );
            }
            return p.offset(2 as libc::c_int as isize);
        } else {
            if *p.offset(0 as libc::c_int as isize) as libc::c_int & 0xff as libc::c_int
                == 0xf0 as libc::c_int
            {
                *count = ((*p.offset(1 as libc::c_int as isize) as uint32_t)
                    << 0 as libc::c_int
                    | (*p.offset(2 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int
                    | (*p.offset(3 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int
                    | (*p.offset(4 as libc::c_int as isize) as uint32_t)
                        << 24 as libc::c_int) as int64_t;
                if !entry_size.is_null() {
                    *entry_size = ((5 as libc::c_int as libc::c_long + *count)
                        as libc::c_ulong)
                        .wrapping_add(
                            lpEncodeBacklen(
                                0 as *mut libc::c_uchar,
                                (*count + 5 as libc::c_int as libc::c_long) as uint64_t,
                            ),
                        );
                }
                return p.offset(5 as libc::c_int as isize);
            } else {
                uval = (12345678900000000 as libc::c_ulonglong)
                    .wrapping_add(
                        *p.offset(0 as libc::c_int as isize) as libc::c_ulonglong,
                    ) as uint64_t;
                negstart = 18446744073709551615 as libc::c_ulong;
                negmax = 0 as libc::c_int as uint64_t;
            }
        }
    }
    if uval >= negstart {
        uval = negmax.wrapping_sub(uval);
        val = uval as int64_t;
        val = -val - 1 as libc::c_int as libc::c_long;
    } else {
        val = uval as int64_t;
    }
    if !intbuf.is_null() {
        *count = ll2string(
            intbuf as *mut libc::c_char,
            21 as libc::c_int as size_t,
            val as libc::c_longlong,
        ) as int64_t;
        return intbuf;
    } else {
        *count = val;
        return 0 as *mut libc::c_uchar;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lpGet(
    mut p: *mut libc::c_uchar,
    mut count: *mut int64_t,
    mut intbuf: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    return lpGetWithSize(p, count, intbuf, 0 as *mut uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn lpGetValue(
    mut p: *mut libc::c_uchar,
    mut slen: *mut libc::c_uint,
    mut lval: *mut libc::c_longlong,
) -> *mut libc::c_uchar {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ele_len: int64_t = 0;
    vstr = lpGet(p, &mut ele_len, 0 as *mut libc::c_uchar);
    if !vstr.is_null() {
        *slen = ele_len as libc::c_uint;
    } else {
        *lval = ele_len as libc::c_longlong;
    }
    return vstr;
}
#[no_mangle]
pub unsafe extern "C" fn lpFind(
    mut lp: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: uint32_t,
    mut skip: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut skipcnt: libc::c_int = 0 as libc::c_int;
    let mut vencoding: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut ll: int64_t = 0;
    let mut vll: int64_t = 0;
    let mut entry_size: uint64_t = 123456789 as libc::c_int as uint64_t;
    let mut lp_bytes: uint32_t = lpBytes(lp) as uint32_t;
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"p\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            695 as libc::c_int,
        );
        unreachable!();
    };
    while !p.is_null() {
        if skipcnt == 0 as libc::c_int {
            value = lpGetWithSize(p, &mut ll, 0 as *mut libc::c_uchar, &mut entry_size);
            if !value.is_null() {
                if (p >= lp.offset(6 as libc::c_int as isize)
                    && p.offset(entry_size as isize) < lp.offset(lp_bytes as isize))
                    as libc::c_int as libc::c_long != 0
                {} else {
                    _serverAssert(
                        b"p >= lp + LP_HDR_SIZE && p + entry_size < lp + lp_bytes\0"
                            as *const u8 as *const libc::c_char,
                        b"listpack.c\0" as *const u8 as *const libc::c_char,
                        701 as libc::c_int,
                    );
                    unreachable!();
                };
                if slen as libc::c_long == ll
                    && memcmp(
                        value as *const libc::c_void,
                        s as *const libc::c_void,
                        slen as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    return p;
                }
            } else {
                if vencoding as libc::c_int == 0 as libc::c_int {
                    if slen >= 32 as libc::c_int as libc::c_uint
                        || slen == 0 as libc::c_int as libc::c_uint
                        || lpStringToInt64(
                            s as *const libc::c_char,
                            slen as libc::c_ulong,
                            &mut vll,
                        ) == 0
                    {
                        vencoding = (127 as libc::c_int * 2 as libc::c_int
                            + 1 as libc::c_int) as libc::c_uchar;
                    } else {
                        vencoding = 1 as libc::c_int as libc::c_uchar;
                    }
                }
                if vencoding as libc::c_int
                    != 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                    && ll == vll
                {
                    return p;
                }
            }
            skipcnt = skip as libc::c_int;
            p = p.offset(entry_size as isize);
        } else {
            skipcnt -= 1;
            p = lpSkip(p);
        }
        if p.offset(8 as libc::c_int as isize) >= lp.offset(lp_bytes as isize) {
            lpAssertValidEntry(lp, lp_bytes as size_t, p);
        } else {
            if (p >= lp.offset(6 as libc::c_int as isize)
                && p < lp.offset(lp_bytes as isize)) as libc::c_int as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"p >= lp + LP_HDR_SIZE && p < lp + lp_bytes\0" as *const u8
                        as *const libc::c_char,
                    b"listpack.c\0" as *const u8 as *const libc::c_char,
                    745 as libc::c_int,
                );
                unreachable!();
            };
        }
        if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
            break;
        }
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn lpInsert(
    mut lp: *mut libc::c_uchar,
    mut elestr: *mut libc::c_uchar,
    mut eleint: *mut libc::c_uchar,
    mut size: uint32_t,
    mut p: *mut libc::c_uchar,
    mut where_0: libc::c_int,
    mut newp: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut intenc: [libc::c_uchar; 9] = [0; 9];
    let mut backlen: [libc::c_uchar; 5] = [0; 5];
    let mut enclen: uint64_t = 0;
    let mut delete: libc::c_int = (elestr.is_null() && eleint.is_null()) as libc::c_int;
    if delete != 0 {
        where_0 = 2 as libc::c_int;
    }
    if where_0 == 1 as libc::c_int {
        p = lpSkip(p);
        where_0 = 0 as libc::c_int;
        if (p >= lp.offset(6 as libc::c_int as isize)
            && p
                < lp
                    .offset(
                        ((*lp.offset(0 as libc::c_int as isize) as uint32_t)
                            << 0 as libc::c_int
                            | (*lp.offset(1 as libc::c_int as isize) as uint32_t)
                                << 8 as libc::c_int
                            | (*lp.offset(2 as libc::c_int as isize) as uint32_t)
                                << 16 as libc::c_int
                            | (*lp.offset(3 as libc::c_int as isize) as uint32_t)
                                << 24 as libc::c_int) as isize,
                    )) as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"(p) >= (lp)+LP_HDR_SIZE && (p) < (lp)+lpGetTotalBytes((lp))\0"
                    as *const u8 as *const libc::c_char,
                b"listpack.c\0" as *const u8 as *const libc::c_char,
                801 as libc::c_int,
            );
            unreachable!();
        };
    }
    let mut poff: libc::c_ulong = p.offset_from(lp) as libc::c_long as libc::c_ulong;
    let mut enctype: libc::c_int = 0;
    if !elestr.is_null() {
        enctype = lpEncodeGetType(elestr, size, intenc.as_mut_ptr(), &mut enclen);
        if enctype == 0 as libc::c_int {
            eleint = intenc.as_mut_ptr();
        }
    } else if !eleint.is_null() {
        enctype = 0 as libc::c_int;
        enclen = size as uint64_t;
    } else {
        enctype = -(1 as libc::c_int);
        enclen = 0 as libc::c_int as uint64_t;
    }
    let mut backlen_size: libc::c_ulong = if delete == 0 {
        lpEncodeBacklen(backlen.as_mut_ptr(), enclen)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    let mut old_listpack_bytes: uint64_t = ((*lp.offset(0 as libc::c_int as isize)
        as uint32_t) << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as uint64_t;
    let mut replaced_len: uint32_t = 0 as libc::c_int as uint32_t;
    if where_0 == 2 as libc::c_int {
        replaced_len = lpCurrentEncodedSizeUnsafe(p);
        replaced_len = (replaced_len as libc::c_ulong)
            .wrapping_add(
                lpEncodeBacklen(0 as *mut libc::c_uchar, replaced_len as uint64_t),
            ) as uint32_t as uint32_t;
        if (p >= lp.offset(6 as libc::c_int as isize)
            && p.offset(replaced_len as isize)
                < lp
                    .offset(
                        ((*lp.offset(0 as libc::c_int as isize) as uint32_t)
                            << 0 as libc::c_int
                            | (*lp.offset(1 as libc::c_int as isize) as uint32_t)
                                << 8 as libc::c_int
                            | (*lp.offset(2 as libc::c_int as isize) as uint32_t)
                                << 16 as libc::c_int
                            | (*lp.offset(3 as libc::c_int as isize) as uint32_t)
                                << 24 as libc::c_int) as isize,
                    )) as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"(p) >= (lp)+LP_HDR_SIZE && (p)+(replaced_len) < (lp)+lpGetTotalBytes((lp))\0"
                    as *const u8 as *const libc::c_char,
                b"listpack.c\0" as *const u8 as *const libc::c_char,
                837 as libc::c_int,
            );
            unreachable!();
        };
    }
    let mut new_listpack_bytes: uint64_t = old_listpack_bytes
        .wrapping_add(enclen)
        .wrapping_add(backlen_size)
        .wrapping_sub(replaced_len as libc::c_ulong);
    if new_listpack_bytes > 4294967295 as libc::c_uint as libc::c_ulong {
        return 0 as *mut libc::c_uchar;
    }
    let mut dst: *mut libc::c_uchar = lp.offset(poff as isize);
    if new_listpack_bytes > old_listpack_bytes
        && new_listpack_bytes > je_malloc_usable_size(lp as *mut libc::c_void)
    {
        lp = zrealloc(lp as *mut libc::c_void, new_listpack_bytes) as *mut libc::c_uchar;
        if lp.is_null() {
            return 0 as *mut libc::c_uchar;
        }
        dst = lp.offset(poff as isize);
    }
    if where_0 == 0 as libc::c_int {
        memmove(
            dst.offset(enclen as isize).offset(backlen_size as isize)
                as *mut libc::c_void,
            dst as *const libc::c_void,
            old_listpack_bytes.wrapping_sub(poff),
        );
    } else {
        let mut lendiff: libc::c_long = enclen
            .wrapping_add(backlen_size)
            .wrapping_sub(replaced_len as libc::c_ulong) as libc::c_long;
        memmove(
            dst.offset(replaced_len as isize).offset(lendiff as isize)
                as *mut libc::c_void,
            dst.offset(replaced_len as isize) as *const libc::c_void,
            old_listpack_bytes
                .wrapping_sub(poff)
                .wrapping_sub(replaced_len as libc::c_ulong),
        );
    }
    if new_listpack_bytes < old_listpack_bytes {
        lp = zrealloc(lp as *mut libc::c_void, new_listpack_bytes) as *mut libc::c_uchar;
        if lp.is_null() {
            return 0 as *mut libc::c_uchar;
        }
        dst = lp.offset(poff as isize);
    }
    if !newp.is_null() {
        *newp = dst;
        if delete != 0
            && *dst.offset(0 as libc::c_int as isize) as libc::c_int
                == 0xff as libc::c_int
        {
            *newp = 0 as *mut libc::c_uchar;
        }
    }
    if delete == 0 {
        if enctype == 0 as libc::c_int {
            memcpy(dst as *mut libc::c_void, eleint as *const libc::c_void, enclen);
        } else {
            lpEncodeString(dst, elestr, size);
        }
        dst = dst.offset(enclen as isize);
        memcpy(
            dst as *mut libc::c_void,
            backlen.as_mut_ptr() as *const libc::c_void,
            backlen_size,
        );
        dst = dst.offset(backlen_size as isize);
    }
    if where_0 != 2 as libc::c_int || delete != 0 {
        let mut num_elements: uint32_t = (*lp.offset(4 as libc::c_int as isize)
            as uint32_t) << 0 as libc::c_int
            | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
        if num_elements != 65535 as libc::c_int as libc::c_uint {
            if delete == 0 {
                *lp
                    .offset(
                        4 as libc::c_int as isize,
                    ) = (num_elements.wrapping_add(1 as libc::c_int as libc::c_uint)
                    & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
                *lp
                    .offset(
                        5 as libc::c_int as isize,
                    ) = (num_elements.wrapping_add(1 as libc::c_int as libc::c_uint)
                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_uchar;
            } else {
                *lp
                    .offset(
                        4 as libc::c_int as isize,
                    ) = (num_elements.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
                *lp
                    .offset(
                        5 as libc::c_int as isize,
                    ) = (num_elements.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
                    as libc::c_uchar;
            }
        }
    }
    *lp
        .offset(
            0 as libc::c_int as isize,
        ) = (new_listpack_bytes & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            1 as libc::c_int as isize,
        ) = (new_listpack_bytes >> 8 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            2 as libc::c_int as isize,
        ) = (new_listpack_bytes >> 16 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            3 as libc::c_int as isize,
        ) = (new_listpack_bytes >> 24 as libc::c_int
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn lpInsertString(
    mut lp: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: uint32_t,
    mut p: *mut libc::c_uchar,
    mut where_0: libc::c_int,
    mut newp: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    return lpInsert(lp, s, 0 as *mut libc::c_uchar, slen, p, where_0, newp);
}
#[no_mangle]
pub unsafe extern "C" fn lpInsertInteger(
    mut lp: *mut libc::c_uchar,
    mut lval: libc::c_longlong,
    mut p: *mut libc::c_uchar,
    mut where_0: libc::c_int,
    mut newp: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut enclen: uint64_t = 0;
    let mut intenc: [libc::c_uchar; 9] = [0; 9];
    lpEncodeIntegerGetType(lval as int64_t, intenc.as_mut_ptr(), &mut enclen);
    return lpInsert(
        lp,
        0 as *mut libc::c_uchar,
        intenc.as_mut_ptr(),
        enclen as uint32_t,
        p,
        where_0,
        newp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lpPrepend(
    mut lp: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: uint32_t,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = lpFirst(lp);
    if p.is_null() {
        return lpAppend(lp, s, slen);
    }
    return lpInsert(
        lp,
        s,
        0 as *mut libc::c_uchar,
        slen,
        p,
        0 as libc::c_int,
        0 as *mut *mut libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lpPrependInteger(
    mut lp: *mut libc::c_uchar,
    mut lval: libc::c_longlong,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = lpFirst(lp);
    if p.is_null() {
        return lpAppendInteger(lp, lval);
    }
    return lpInsertInteger(lp, lval, p, 0 as libc::c_int, 0 as *mut *mut libc::c_uchar);
}
#[no_mangle]
pub unsafe extern "C" fn lpAppend(
    mut lp: *mut libc::c_uchar,
    mut ele: *mut libc::c_uchar,
    mut size: uint32_t,
) -> *mut libc::c_uchar {
    let mut listpack_bytes: uint64_t = ((*lp.offset(0 as libc::c_int as isize)
        as uint32_t) << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as uint64_t;
    let mut eofptr: *mut libc::c_uchar = lp
        .offset(listpack_bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    return lpInsert(
        lp,
        ele,
        0 as *mut libc::c_uchar,
        size,
        eofptr,
        0 as libc::c_int,
        0 as *mut *mut libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lpAppendInteger(
    mut lp: *mut libc::c_uchar,
    mut lval: libc::c_longlong,
) -> *mut libc::c_uchar {
    let mut listpack_bytes: uint64_t = ((*lp.offset(0 as libc::c_int as isize)
        as uint32_t) << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as uint64_t;
    let mut eofptr: *mut libc::c_uchar = lp
        .offset(listpack_bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    return lpInsertInteger(
        lp,
        lval,
        eofptr,
        0 as libc::c_int,
        0 as *mut *mut libc::c_uchar,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lpReplace(
    mut lp: *mut libc::c_uchar,
    mut p: *mut *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: uint32_t,
) -> *mut libc::c_uchar {
    return lpInsert(lp, s, 0 as *mut libc::c_uchar, slen, *p, 2 as libc::c_int, p);
}
#[no_mangle]
pub unsafe extern "C" fn lpReplaceInteger(
    mut lp: *mut libc::c_uchar,
    mut p: *mut *mut libc::c_uchar,
    mut lval: libc::c_longlong,
) -> *mut libc::c_uchar {
    return lpInsertInteger(lp, lval, *p, 2 as libc::c_int, p);
}
#[no_mangle]
pub unsafe extern "C" fn lpDelete(
    mut lp: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut newp: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    return lpInsert(
        lp,
        0 as *mut libc::c_uchar,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as uint32_t,
        p,
        2 as libc::c_int,
        newp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lpDeleteRangeWithEntry(
    mut lp: *mut libc::c_uchar,
    mut p: *mut *mut libc::c_uchar,
    mut num: libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut bytes: size_t = lpBytes(lp);
    let mut deleted: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut eofptr: *mut libc::c_uchar = lp
        .offset(bytes as isize)
        .offset(-(1 as libc::c_int as isize));
    let mut first: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tail: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    tail = *p;
    first = tail;
    if num == 0 as libc::c_int as libc::c_ulong {
        return lp;
    }
    loop {
        let fresh0 = num;
        num = num.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        deleted = deleted.wrapping_add(1);
        tail = lpSkip(tail);
        if *tail.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int
        {
            break;
        }
        lpAssertValidEntry(lp, bytes, tail);
    }
    let mut poff: libc::c_ulong = first.offset_from(lp) as libc::c_long as libc::c_ulong;
    memmove(
        first as *mut libc::c_void,
        tail as *const libc::c_void,
        (eofptr.offset_from(tail) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_ulong,
    );
    *lp
        .offset(
            0 as libc::c_int as isize,
        ) = (bytes.wrapping_sub(tail.offset_from(first) as libc::c_long as libc::c_ulong)
        & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            1 as libc::c_int as isize,
        ) = (bytes.wrapping_sub(tail.offset_from(first) as libc::c_long as libc::c_ulong)
        >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            2 as libc::c_int as isize,
        ) = (bytes.wrapping_sub(tail.offset_from(first) as libc::c_long as libc::c_ulong)
        >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *lp
        .offset(
            3 as libc::c_int as isize,
        ) = (bytes.wrapping_sub(tail.offset_from(first) as libc::c_long as libc::c_ulong)
        >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    let mut numele: uint32_t = (*lp.offset(4 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
    if numele != 65535 as libc::c_int as libc::c_uint {
        *lp
            .offset(
                4 as libc::c_int as isize,
            ) = ((numele as libc::c_ulong).wrapping_sub(deleted)
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        *lp
            .offset(
                5 as libc::c_int as isize,
            ) = ((numele as libc::c_ulong).wrapping_sub(deleted) >> 8 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    }
    lp = lpShrinkToFit(lp);
    *p = lp.offset(poff as isize);
    if *(*p).offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        *p = 0 as *mut libc::c_uchar;
    }
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn lpDeleteRange(
    mut lp: *mut libc::c_uchar,
    mut index: libc::c_long,
    mut num: libc::c_ulong,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut numele: uint32_t = (*lp.offset(4 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
    if num == 0 as libc::c_int as libc::c_ulong {
        return lp;
    }
    p = lpSeek(lp, index);
    if p.is_null() {
        return lp;
    }
    if numele != 65535 as libc::c_int as libc::c_uint
        && index < 0 as libc::c_int as libc::c_long
    {
        index = numele as libc::c_long + index;
    }
    if numele != 65535 as libc::c_int as libc::c_uint
        && (numele as libc::c_ulong).wrapping_sub(index as libc::c_ulong) <= num
    {
        *p.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as libc::c_uchar;
        *lp
            .offset(
                0 as libc::c_int as isize,
            ) = (p.offset_from(lp) as libc::c_long + 1 as libc::c_int as libc::c_long
            & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *lp
            .offset(
                1 as libc::c_int as isize,
            ) = (p.offset_from(lp) as libc::c_long + 1 as libc::c_int as libc::c_long
            >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *lp
            .offset(
                2 as libc::c_int as isize,
            ) = (p.offset_from(lp) as libc::c_long + 1 as libc::c_int as libc::c_long
            >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *lp
            .offset(
                3 as libc::c_int as isize,
            ) = (p.offset_from(lp) as libc::c_long + 1 as libc::c_int as libc::c_long
            >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *lp
            .offset(
                4 as libc::c_int as isize,
            ) = (index & 0xff as libc::c_int as libc::c_long) as libc::c_uchar;
        *lp
            .offset(
                5 as libc::c_int as isize,
            ) = (index >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_long)
            as libc::c_uchar;
        lp = lpShrinkToFit(lp);
    } else {
        lp = lpDeleteRangeWithEntry(lp, &mut p, num);
    }
    return lp;
}
#[no_mangle]
pub unsafe extern "C" fn lpMerge(
    mut first: *mut *mut libc::c_uchar,
    mut second: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    if first.is_null() || (*first).is_null() || second.is_null() || (*second).is_null() {
        return 0 as *mut libc::c_uchar;
    }
    if *first == *second {
        return 0 as *mut libc::c_uchar;
    }
    let mut first_bytes: size_t = lpBytes(*first);
    let mut first_len: libc::c_ulong = lpLength(*first);
    let mut second_bytes: size_t = lpBytes(*second);
    let mut second_len: libc::c_ulong = lpLength(*second);
    let mut append: libc::c_int = 0;
    let mut source: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut target: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut target_bytes: size_t = 0;
    let mut source_bytes: size_t = 0;
    if first_bytes >= second_bytes {
        target = *first;
        target_bytes = first_bytes;
        source = *second;
        source_bytes = second_bytes;
        append = 1 as libc::c_int;
    } else {
        target = *second;
        target_bytes = second_bytes;
        source = *first;
        source_bytes = first_bytes;
        append = 0 as libc::c_int;
    }
    let mut lpbytes: libc::c_ulonglong = (first_bytes as libc::c_ulonglong)
        .wrapping_add(second_bytes as libc::c_ulonglong)
        .wrapping_sub(6 as libc::c_int as libc::c_ulonglong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulonglong);
    if (lpbytes < 4294967295 as libc::c_uint as libc::c_ulonglong) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"lpbytes < UINT32_MAX\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1119 as libc::c_int,
        );
        unreachable!();
    };
    let mut lplength: libc::c_ulong = first_len.wrapping_add(second_len);
    lplength = if lplength < 65535 as libc::c_int as libc::c_ulong {
        lplength
    } else {
        65535 as libc::c_int as libc::c_ulong
    };
    target = zrealloc(target as *mut libc::c_void, lpbytes as size_t)
        as *mut libc::c_uchar;
    if append != 0 {
        memcpy(
            target.offset(target_bytes as isize).offset(-(1 as libc::c_int as isize))
                as *mut libc::c_void,
            source.offset(6 as libc::c_int as isize) as *const libc::c_void,
            source_bytes.wrapping_sub(6 as libc::c_int as libc::c_ulong),
        );
    } else {
        memmove(
            target.offset(source_bytes as isize).offset(-(1 as libc::c_int as isize))
                as *mut libc::c_void,
            target.offset(6 as libc::c_int as isize) as *const libc::c_void,
            target_bytes.wrapping_sub(6 as libc::c_int as libc::c_ulong),
        );
        memcpy(
            target as *mut libc::c_void,
            source as *const libc::c_void,
            source_bytes.wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
    }
    *target
        .offset(
            4 as libc::c_int as isize,
        ) = (lplength & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
    *target
        .offset(
            5 as libc::c_int as isize,
        ) = (lplength >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
        as libc::c_uchar;
    *target
        .offset(
            0 as libc::c_int as isize,
        ) = (lpbytes & 0xff as libc::c_int as libc::c_ulonglong) as libc::c_uchar;
    *target
        .offset(
            1 as libc::c_int as isize,
        ) = (lpbytes >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulonglong)
        as libc::c_uchar;
    *target
        .offset(
            2 as libc::c_int as isize,
        ) = (lpbytes >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulonglong)
        as libc::c_uchar;
    *target
        .offset(
            3 as libc::c_int as isize,
        ) = (lpbytes >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulonglong)
        as libc::c_uchar;
    if append != 0 {
        zfree(*second as *mut libc::c_void);
        *second = 0 as *mut libc::c_uchar;
        *first = target;
    } else {
        zfree(*first as *mut libc::c_void);
        *first = 0 as *mut libc::c_uchar;
        *second = target;
    }
    return target;
}
#[no_mangle]
pub unsafe extern "C" fn lpBytes(mut lp: *mut libc::c_uchar) -> size_t {
    return ((*lp.offset(0 as libc::c_int as isize) as uint32_t) << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn lpSeek(
    mut lp: *mut libc::c_uchar,
    mut index: libc::c_long,
) -> *mut libc::c_uchar {
    let mut forward: libc::c_int = 1 as libc::c_int;
    let mut numele: uint32_t = (*lp.offset(4 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
    if numele != 65535 as libc::c_int as libc::c_uint {
        if index < 0 as libc::c_int as libc::c_long {
            index = numele as libc::c_long + index;
        }
        if index < 0 as libc::c_int as libc::c_long {
            return 0 as *mut libc::c_uchar;
        }
        if index >= numele as libc::c_long {
            return 0 as *mut libc::c_uchar;
        }
        if index > numele as libc::c_long / 2 as libc::c_int as libc::c_long {
            forward = 0 as libc::c_int;
            index -= numele as libc::c_long;
        }
    } else if index < 0 as libc::c_int as libc::c_long {
        forward = 0 as libc::c_int;
    }
    if forward != 0 {
        let mut ele: *mut libc::c_uchar = lpFirst(lp);
        while index > 0 as libc::c_int as libc::c_long && !ele.is_null() {
            ele = lpNext(lp, ele);
            index -= 1;
        }
        return ele;
    } else {
        let mut ele_0: *mut libc::c_uchar = lpLast(lp);
        while index < -(1 as libc::c_int) as libc::c_long && !ele_0.is_null() {
            ele_0 = lpPrev(lp, ele_0);
            index += 1;
        }
        return ele_0;
    };
}
#[no_mangle]
pub unsafe extern "C" fn lpValidateFirst(
    mut lp: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = lp.offset(6 as libc::c_int as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn lpValidateNext(
    mut lp: *mut libc::c_uchar,
    mut pp: *mut *mut libc::c_uchar,
    mut lpbytes: size_t,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = *pp;
    if p.is_null() {
        return 0 as libc::c_int;
    }
    if p < lp.offset(6 as libc::c_int as isize)
        || p > lp.offset(lpbytes as isize).offset(-(1 as libc::c_int as isize))
    {
        return 0 as libc::c_int;
    }
    if *p as libc::c_int == 0xff as libc::c_int {
        *pp = 0 as *mut libc::c_uchar;
        return 1 as libc::c_int;
    }
    let mut lenbytes: uint32_t = lpCurrentEncodedSizeBytes(p);
    if lenbytes == 0 {
        return 0 as libc::c_int;
    }
    if p.offset(lenbytes as isize) < lp.offset(6 as libc::c_int as isize)
        || p.offset(lenbytes as isize)
            > lp.offset(lpbytes as isize).offset(-(1 as libc::c_int as isize))
    {
        return 0 as libc::c_int;
    }
    let mut entrylen: libc::c_ulong = lpCurrentEncodedSizeUnsafe(p) as libc::c_ulong;
    let mut encodedBacklen: libc::c_ulong = lpEncodeBacklen(
        0 as *mut libc::c_uchar,
        entrylen,
    );
    entrylen = entrylen.wrapping_add(encodedBacklen);
    if p.offset(entrylen as isize) < lp.offset(6 as libc::c_int as isize)
        || p.offset(entrylen as isize)
            > lp.offset(lpbytes as isize).offset(-(1 as libc::c_int as isize))
    {
        return 0 as libc::c_int;
    }
    p = p.offset(entrylen as isize);
    let mut prevlen: uint64_t = lpDecodeBacklen(p.offset(-(1 as libc::c_int as isize)));
    if prevlen.wrapping_add(encodedBacklen) != entrylen {
        return 0 as libc::c_int;
    }
    *pp = p;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn lpAssertValidEntry(
    mut lp: *mut libc::c_uchar,
    mut lpbytes: size_t,
    mut p: *mut libc::c_uchar,
) {
    if (lpValidateNext(lp, &mut p, lpbytes) != 0) as libc::c_int as libc::c_long != 0
    {} else {
        _serverAssert(
            b"lpValidateNext(lp, &p, lpbytes)\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1276 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn lpValidateIntegrity(
    mut lp: *mut libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
    mut entry_cb: listpackValidateEntryCB,
    mut cb_userdata: *mut libc::c_void,
) -> libc::c_int {
    if size < (6 as libc::c_int + 1 as libc::c_int) as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut bytes: size_t = ((*lp.offset(0 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(1 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
        | (*lp.offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
        | (*lp.offset(3 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int)
        as size_t;
    if bytes != size {
        return 0 as libc::c_int;
    }
    if *lp.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int != 0xff as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if deep == 0 {
        return 1 as libc::c_int;
    }
    let mut count: uint32_t = 0 as libc::c_int as uint32_t;
    let mut numele: uint32_t = (*lp.offset(4 as libc::c_int as isize) as uint32_t)
        << 0 as libc::c_int
        | (*lp.offset(5 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int;
    let mut p: *mut libc::c_uchar = lp.offset(6 as libc::c_int as isize);
    while !p.is_null()
        && *p.offset(0 as libc::c_int as isize) as libc::c_int != 0xff as libc::c_int
    {
        let mut prev: *mut libc::c_uchar = p;
        if lpValidateNext(lp, &mut p, bytes) == 0 {
            return 0 as libc::c_int;
        }
        if entry_cb.is_some()
            && entry_cb.expect("non-null function pointer")(prev, numele, cb_userdata)
                == 0
        {
            return 0 as libc::c_int;
        }
        count = count.wrapping_add(1);
    }
    if p != lp.offset(size as isize).offset(-(1 as libc::c_int as isize)) {
        return 0 as libc::c_int;
    }
    if numele != 65535 as libc::c_int as libc::c_uint && numele != count {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lpCompare(
    mut p: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: uint32_t,
) -> libc::c_uint {
    let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut sz: int64_t = 0;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 0xff as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    value = lpGet(p, &mut sz, 0 as *mut libc::c_uchar);
    if !value.is_null() {
        return (slen as libc::c_long == sz
            && memcmp(
                value as *const libc::c_void,
                s as *const libc::c_void,
                slen as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int as libc::c_uint
    } else {
        let mut sval: int64_t = 0;
        if lpStringToInt64(s as *const libc::c_char, slen as libc::c_ulong, &mut sval)
            != 0
        {
            return (sz == sval) as libc::c_int as libc::c_uint;
        }
    }
    return 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn uintCompare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (*(a as *mut libc::c_uint)).wrapping_sub(*(b as *mut libc::c_uint))
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn lpSaveValue(
    mut val: *mut libc::c_uchar,
    mut len: libc::c_uint,
    mut lval: int64_t,
    mut dest: *mut listpackEntry,
) {
    (*dest).sval = val;
    (*dest).slen = len;
    (*dest).lval = lval as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn lpRandomPair(
    mut lp: *mut libc::c_uchar,
    mut total_count: libc::c_ulong,
    mut key: *mut listpackEntry,
    mut val: *mut listpackEntry,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (total_count != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"total_count\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1372 as libc::c_int,
        );
        unreachable!();
    };
    let mut r: libc::c_int = (rand() as libc::c_ulong)
        .wrapping_rem(total_count)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    p = lpSeek(lp, r as libc::c_long);
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"(p = lpSeek(lp, r))\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1376 as libc::c_int,
        );
        unreachable!();
    };
    (*key).sval = lpGetValue(p, &mut (*key).slen, &mut (*key).lval);
    if val.is_null() {
        return;
    }
    p = lpNext(lp, p);
    if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"(p = lpNext(lp, p))\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1381 as libc::c_int,
        );
        unreachable!();
    };
    (*val).sval = lpGetValue(p, &mut (*val).slen, &mut (*val).lval);
}
#[no_mangle]
pub unsafe extern "C" fn lpRandomPairs(
    mut lp: *mut libc::c_uchar,
    mut count: libc::c_uint,
    mut keys: *mut listpackEntry,
    mut vals: *mut listpackEntry,
) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut klen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut vlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut klval: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut vlval: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut picks: *mut rand_pick = zmalloc(
        (core::mem::size_of::<rand_pick>() as libc::c_ulong)
            .wrapping_mul(count as libc::c_ulong),
    ) as *mut rand_pick;
    let mut total_size: libc::c_uint = (lpLength(lp))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_uint;
    if (total_size != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"total_size\0" as *const u8 as *const libc::c_char,
            b"listpack.c\0" as *const u8 as *const libc::c_char,
            1403 as libc::c_int,
        );
        unreachable!();
    };
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while i < count {
        (*picks.offset(i as isize))
            .index = (rand() as libc::c_uint)
            .wrapping_rem(total_size)
            .wrapping_mul(2 as libc::c_int as libc::c_uint);
        (*picks.offset(i as isize)).order = i;
        i = i.wrapping_add(1);
    }
    qsort(
        picks as *mut libc::c_void,
        count as size_t,
        core::mem::size_of::<rand_pick>() as libc::c_ulong,
        Some(
            uintCompare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    );
    let mut lpindex: libc::c_uint = (*picks.offset(0 as libc::c_int as isize)).index;
    let mut pickindex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    p = lpSeek(lp, lpindex as libc::c_long);
    while !p.is_null() && pickindex < count {
        key = lpGetValue(p, &mut klen, &mut klval);
        p = lpNext(lp, p);
        if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
            _serverAssert(
                b"(p = lpNext(lp, p))\0" as *const u8 as *const libc::c_char,
                b"listpack.c\0" as *const u8 as *const libc::c_char,
                1420 as libc::c_int,
            );
            unreachable!();
        };
        value = lpGetValue(p, &mut vlen, &mut vlval);
        while pickindex < count && lpindex == (*picks.offset(pickindex as isize)).index {
            let mut storeorder: libc::c_int = (*picks.offset(pickindex as isize)).order
                as libc::c_int;
            lpSaveValue(
                key,
                klen,
                klval as int64_t,
                &mut *keys.offset(storeorder as isize),
            );
            if !vals.is_null() {
                lpSaveValue(
                    value,
                    vlen,
                    vlval as int64_t,
                    &mut *vals.offset(storeorder as isize),
                );
            }
            pickindex = pickindex.wrapping_add(1);
        }
        lpindex = lpindex.wrapping_add(2 as libc::c_int as libc::c_uint);
        p = lpNext(lp, p);
    }
    zfree(picks as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn lpRandomPairsUnique(
    mut lp: *mut libc::c_uchar,
    mut count: libc::c_uint,
    mut keys: *mut listpackEntry,
    mut vals: *mut listpackEntry,
) -> libc::c_uint {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut klen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut klval: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut total_size: libc::c_uint = (lpLength(lp))
        .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_uint;
    let mut index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if count > total_size {
        count = total_size;
    }
    p = lpFirst(lp);
    let mut picked: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut remaining: libc::c_uint = count;
    while picked < count && !p.is_null() {
        let mut randomDouble: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double;
        let mut threshold: libc::c_double = remaining as libc::c_double
            / total_size.wrapping_sub(index) as libc::c_double;
        if randomDouble <= threshold {
            key = lpGetValue(p, &mut klen, &mut klval);
            lpSaveValue(key, klen, klval as int64_t, &mut *keys.offset(picked as isize));
            p = lpNext(lp, p);
            if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
                _serverAssert(
                    b"(p = lpNext(lp, p))\0" as *const u8 as *const libc::c_char,
                    b"listpack.c\0" as *const u8 as *const libc::c_char,
                    1463 as libc::c_int,
                );
                unreachable!();
            };
            if !vals.is_null() {
                key = lpGetValue(p, &mut klen, &mut klval);
                lpSaveValue(
                    key,
                    klen,
                    klval as int64_t,
                    &mut *vals.offset(picked as isize),
                );
            }
            remaining = remaining.wrapping_sub(1);
            picked = picked.wrapping_add(1);
        } else {
            p = lpNext(lp, p);
            if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
                _serverAssert(
                    b"(p = lpNext(lp, p))\0" as *const u8 as *const libc::c_char,
                    b"listpack.c\0" as *const u8 as *const libc::c_char,
                    1471 as libc::c_int,
                );
                unreachable!();
            };
        }
        p = lpNext(lp, p);
        index = index.wrapping_add(1);
    }
    return picked;
}
#[no_mangle]
pub unsafe extern "C" fn lpRepr(mut lp: *mut libc::c_uchar) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: int64_t = 0;
    let mut intbuf: [libc::c_uchar; 21] = [0; 21];
    let mut index: libc::c_int = 0 as libc::c_int;
    printf(
        b"{total bytes %zu} {num entries %lu}\n\0" as *const u8 as *const libc::c_char,
        lpBytes(lp),
        lpLength(lp),
    );
    p = lpFirst(lp);
    while !p.is_null() {
        let mut encoded_size_bytes: uint32_t = lpCurrentEncodedSizeBytes(p);
        let mut encoded_size: uint32_t = lpCurrentEncodedSizeUnsafe(p);
        let mut back_len: libc::c_ulong = lpEncodeBacklen(
            0 as *mut libc::c_uchar,
            encoded_size as uint64_t,
        );
        printf(
            b"{\n\taddr: 0x%08lx,\n\tindex: %2d,\n\toffset: %1lu,\n\thdr+entrylen+backlen: %2lu,\n\thdrlen: %3u,\n\tbacklen: %2lu,\n\tpayload: %1u\n\0"
                as *const u8 as *const libc::c_char,
            p as libc::c_ulong,
            index,
            p.offset_from(lp) as libc::c_long as libc::c_ulong,
            (encoded_size as libc::c_ulong).wrapping_add(back_len),
            encoded_size_bytes,
            back_len,
            encoded_size.wrapping_sub(encoded_size_bytes),
        );
        printf(b"\tbytes: \0" as *const u8 as *const libc::c_char);
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while (i as libc::c_ulong)
            < (encoded_size as libc::c_ulong).wrapping_add(back_len)
        {
            printf(
                b"%02x|\0" as *const u8 as *const libc::c_char,
                *p.offset(i as isize) as libc::c_int,
            );
            i = i.wrapping_add(1);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        vstr = lpGet(p, &mut vlen, intbuf.as_mut_ptr());
        printf(b"\t[str]\0" as *const u8 as *const libc::c_char);
        if vlen > 40 as libc::c_int as libc::c_long {
            if fwrite(
                vstr as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                stdout,
            ) == 0 as libc::c_int as libc::c_ulong
            {
                perror(b"fwrite\0" as *const u8 as *const libc::c_char);
            }
            printf(b"...\0" as *const u8 as *const libc::c_char);
        } else if fwrite(
            vstr as *const libc::c_void,
            vlen as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            stdout,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            perror(b"fwrite\0" as *const u8 as *const libc::c_char);
        }
        printf(b"\n}\n\0" as *const u8 as *const libc::c_char);
        index += 1;
        p = lpNext(lp, p);
    }
    printf(b"{end}\n\n\0" as *const u8 as *const libc::c_char);
}

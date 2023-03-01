extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
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
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ziplistEntry {
    pub sval: *mut libc::c_uchar,
    pub slen: libc::c_uint,
    pub lval: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zlentry {
    pub prevrawlensize: libc::c_uint,
    pub prevrawlen: libc::c_uint,
    pub lensize: libc::c_uint,
    pub len: libc::c_uint,
    pub headersize: libc::c_uint,
    pub encoding: libc::c_uchar,
    pub p: *mut libc::c_uchar,
}
pub type ziplistValidateEntryCB = Option::<
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
pub unsafe extern "C" fn ziplistSafeToAdd(
    mut zl: *mut libc::c_uchar,
    mut add: size_t,
) -> libc::c_int {
    let mut len: size_t = if !zl.is_null() {
        ziplistBlobLen(zl)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if len.wrapping_add(add) > ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zipEncodingLenSize(mut encoding: libc::c_uchar) -> libc::c_uint {
    if encoding as libc::c_int
        == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        || encoding as libc::c_int
            == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        || encoding as libc::c_int
            == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
        || encoding as libc::c_int
            == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
        || encoding as libc::c_int == 0xfe as libc::c_int
    {
        return 1 as libc::c_int as libc::c_uint;
    }
    if encoding as libc::c_int >= 0xf1 as libc::c_int
        && encoding as libc::c_int <= 0xfd as libc::c_int
    {
        return 1 as libc::c_int as libc::c_uint;
    }
    if encoding as libc::c_int == (0 as libc::c_int) << 6 as libc::c_int {
        return 1 as libc::c_int as libc::c_uint;
    }
    if encoding as libc::c_int == (1 as libc::c_int) << 6 as libc::c_int {
        return 2 as libc::c_int as libc::c_uint;
    }
    if encoding as libc::c_int == (2 as libc::c_int) << 6 as libc::c_int {
        return 5 as libc::c_int as libc::c_uint;
    }
    return 0xff as libc::c_int as libc::c_uint;
}
#[inline]
unsafe extern "C" fn zipIntSize(mut encoding: libc::c_uchar) -> libc::c_uint {
    match encoding as libc::c_int {
        254 => return 1 as libc::c_int as libc::c_uint,
        192 => return 2 as libc::c_int as libc::c_uint,
        240 => return 3 as libc::c_int as libc::c_uint,
        208 => return 4 as libc::c_int as libc::c_uint,
        224 => return 8 as libc::c_int as libc::c_uint,
        _ => {}
    }
    if encoding as libc::c_int >= 0xf1 as libc::c_int
        && encoding as libc::c_int <= 0xfd as libc::c_int
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    unreachable!();
}
#[no_mangle]
pub unsafe extern "C" fn zipStoreEntryEncoding(
    mut p: *mut libc::c_uchar,
    mut encoding: libc::c_uchar,
    mut rawlen: libc::c_uint,
) -> libc::c_uint {
    let mut len: libc::c_uchar = 1 as libc::c_int as libc::c_uchar;
    let mut buf: [libc::c_uchar; 5] = [0; 5];
    if (encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
        if rawlen <= 0x3f as libc::c_int as libc::c_uint {
            if p.is_null() {
                return len as libc::c_uint;
            }
            buf[0 as libc::c_int
                as usize] = (((0 as libc::c_int) << 6 as libc::c_int) as libc::c_uint
                | rawlen) as libc::c_uchar;
        } else if rawlen <= 0x3fff as libc::c_int as libc::c_uint {
            len = (len as libc::c_int + 1 as libc::c_int) as libc::c_uchar;
            if p.is_null() {
                return len as libc::c_uint;
            }
            buf[0 as libc::c_int
                as usize] = (((1 as libc::c_int) << 6 as libc::c_int) as libc::c_uint
                | rawlen >> 8 as libc::c_int & 0x3f as libc::c_int as libc::c_uint)
                as libc::c_uchar;
            buf[1 as libc::c_int
                as usize] = (rawlen & 0xff as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        } else {
            len = (len as libc::c_int + 4 as libc::c_int) as libc::c_uchar;
            if p.is_null() {
                return len as libc::c_uint;
            }
            buf[0 as libc::c_int
                as usize] = ((2 as libc::c_int) << 6 as libc::c_int) as libc::c_uchar;
            buf[1 as libc::c_int
                as usize] = (rawlen >> 24 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            buf[2 as libc::c_int
                as usize] = (rawlen >> 16 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            buf[3 as libc::c_int
                as usize] = (rawlen >> 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
            buf[4 as libc::c_int
                as usize] = (rawlen & 0xff as libc::c_int as libc::c_uint)
                as libc::c_uchar;
        }
    } else {
        if p.is_null() {
            return len as libc::c_uint;
        }
        buf[0 as libc::c_int as usize] = encoding;
    }
    memcpy(
        p as *mut libc::c_void,
        buf.as_mut_ptr() as *const libc::c_void,
        len as libc::c_ulong,
    );
    return len as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn zipStorePrevEntryLengthLarge(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut u32: uint32_t = 0;
    if !p.is_null() {
        *p.offset(0 as libc::c_int as isize) = 254 as libc::c_int as libc::c_uchar;
        u32 = len;
        memcpy(
            p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            &mut u32 as *mut uint32_t as *const libc::c_void,
            core::mem::size_of::<uint32_t>() as libc::c_ulong,
        );
    }
    return (1 as libc::c_int as libc::c_ulong)
        .wrapping_add(core::mem::size_of::<uint32_t>() as libc::c_ulong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zipStorePrevEntryLength(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_uint {
    if p.is_null() {
        return (if len < 254 as libc::c_int as libc::c_uint {
            1 as libc::c_int as libc::c_ulong
        } else {
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        }) as libc::c_uint
    } else if len < 254 as libc::c_int as libc::c_uint {
        *p.offset(0 as libc::c_int as isize) = len as libc::c_uchar;
        return 1 as libc::c_int as libc::c_uint;
    } else {
        return zipStorePrevEntryLengthLarge(p, len) as libc::c_uint
    };
}
#[no_mangle]
pub unsafe extern "C" fn zipPrevLenByteDiff(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut prevlensize: libc::c_uint = 0;
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
        prevlensize = 1 as libc::c_int as libc::c_uint;
    } else {
        prevlensize = 5 as libc::c_int as libc::c_uint;
    }
    return (zipStorePrevEntryLength(0 as *mut libc::c_uchar, len))
        .wrapping_sub(prevlensize) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zipTryEncoding(
    mut entry: *mut libc::c_uchar,
    mut entrylen: libc::c_uint,
    mut v: *mut libc::c_longlong,
    mut encoding: *mut libc::c_uchar,
) -> libc::c_int {
    let mut value: libc::c_longlong = 0;
    if entrylen >= 32 as libc::c_int as libc::c_uint
        || entrylen == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    if string2ll(entry as *mut libc::c_char, entrylen as size_t, &mut value) != 0 {
        if value >= 0 as libc::c_int as libc::c_longlong
            && value <= 12 as libc::c_int as libc::c_longlong
        {
            *encoding = (0xf1 as libc::c_int as libc::c_longlong + value)
                as libc::c_uchar;
        } else if value >= -(128 as libc::c_int) as libc::c_longlong
            && value <= 127 as libc::c_int as libc::c_longlong
        {
            *encoding = 0xfe as libc::c_int as libc::c_uchar;
        } else if value
            >= (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
            && value <= 32767 as libc::c_int as libc::c_longlong
        {
            *encoding = (0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int)
                as libc::c_uchar;
        } else if value
            >= (-(0x7fffff as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
            && value <= 0x7fffff as libc::c_int as libc::c_longlong
        {
            *encoding = (0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int)
                as libc::c_uchar;
        } else if value
            >= (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_longlong
            && value <= 2147483647 as libc::c_int as libc::c_longlong
        {
            *encoding = (0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int)
                as libc::c_uchar;
        } else {
            *encoding = (0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int)
                as libc::c_uchar;
        }
        *v = value;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zipSaveInteger(
    mut p: *mut libc::c_uchar,
    mut value: int64_t,
    mut encoding: libc::c_uchar,
) {
    let mut i16: int16_t = 0;
    let mut i32: int32_t = 0;
    let mut i64: int64_t = 0;
    if encoding as libc::c_int == 0xfe as libc::c_int {
        *(p as *mut int8_t).offset(0 as libc::c_int as isize) = value as int8_t;
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        i16 = value as int16_t;
        memcpy(
            p as *mut libc::c_void,
            &mut i16 as *mut int16_t as *const libc::c_void,
            core::mem::size_of::<int16_t>() as libc::c_ulong,
        );
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
    {
        i32 = ((value as uint64_t) << 8 as libc::c_int) as int32_t;
        memcpy(
            p as *mut libc::c_void,
            (&mut i32 as *mut int32_t as *mut uint8_t).offset(1 as libc::c_int as isize)
                as *const libc::c_void,
            (core::mem::size_of::<int32_t>() as libc::c_ulong)
                .wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        i32 = value as int32_t;
        memcpy(
            p as *mut libc::c_void,
            &mut i32 as *mut int32_t as *const libc::c_void,
            core::mem::size_of::<int32_t>() as libc::c_ulong,
        );
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
    {
        i64 = value;
        memcpy(
            p as *mut libc::c_void,
            &mut i64 as *mut int64_t as *const libc::c_void,
            core::mem::size_of::<int64_t>() as libc::c_ulong,
        );
    } else if !(encoding as libc::c_int >= 0xf1 as libc::c_int
        && encoding as libc::c_int <= 0xfd as libc::c_int)
    {
        if !(0 as *mut libc::c_void).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"NULL\0" as *const u8 as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                574 as libc::c_int,
            );
            unreachable!();
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn zipLoadInteger(
    mut p: *mut libc::c_uchar,
    mut encoding: libc::c_uchar,
) -> int64_t {
    let mut i16: int16_t = 0;
    let mut i32: int32_t = 0;
    let mut i64: int64_t = 0;
    let mut ret: int64_t = 0 as libc::c_int as int64_t;
    if encoding as libc::c_int == 0xfe as libc::c_int {
        ret = *(p as *mut int8_t).offset(0 as libc::c_int as isize) as int64_t;
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
    {
        memcpy(
            &mut i16 as *mut int16_t as *mut libc::c_void,
            p as *const libc::c_void,
            core::mem::size_of::<int16_t>() as libc::c_ulong,
        );
        ret = i16 as int64_t;
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
    {
        memcpy(
            &mut i32 as *mut int32_t as *mut libc::c_void,
            p as *const libc::c_void,
            core::mem::size_of::<int32_t>() as libc::c_ulong,
        );
        ret = i32 as int64_t;
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
    {
        i32 = 0 as libc::c_int;
        memcpy(
            (&mut i32 as *mut int32_t as *mut uint8_t).offset(1 as libc::c_int as isize)
                as *mut libc::c_void,
            p as *const libc::c_void,
            (core::mem::size_of::<int32_t>() as libc::c_ulong)
                .wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
        ret = (i32 >> 8 as libc::c_int) as int64_t;
    } else if encoding as libc::c_int
        == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
    {
        memcpy(
            &mut i64 as *mut int64_t as *mut libc::c_void,
            p as *const libc::c_void,
            core::mem::size_of::<int64_t>() as libc::c_ulong,
        );
        ret = i64;
    } else if encoding as libc::c_int >= 0xf1 as libc::c_int
        && encoding as libc::c_int <= 0xfd as libc::c_int
    {
        ret = ((encoding as libc::c_int & 0xf as libc::c_int) - 1 as libc::c_int)
            as int64_t;
    } else {
        if !(0 as *mut libc::c_void).is_null() as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"NULL\0" as *const u8 as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                605 as libc::c_int,
            );
            unreachable!();
        };
    }
    return ret;
}
#[inline]
unsafe extern "C" fn zipEntry(mut p: *mut libc::c_uchar, mut e: *mut zlentry) {
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
        (*e).prevrawlensize = 1 as libc::c_int as libc::c_uint;
    } else {
        (*e).prevrawlensize = 5 as libc::c_int as libc::c_uint;
    }
    if (*e).prevrawlensize == 1 as libc::c_int as libc::c_uint {
        (*e).prevrawlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    } else {
        (*e)
            .prevrawlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
            << 24 as libc::c_int
            | (*p.offset(3 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
            | (*p.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | *p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
    }
    (*e)
        .encoding = *p
        .offset((*e).prevrawlensize as isize)
        .offset(0 as libc::c_int as isize);
    if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
        (*e)
            .encoding = ((*e).encoding as libc::c_int & 0xc0 as libc::c_int)
            as libc::c_uchar;
    }
    if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
        if (*e).encoding as libc::c_int == (0 as libc::c_int) << 6 as libc::c_int {
            (*e).lensize = 1 as libc::c_int as libc::c_uint;
            (*e)
                .len = (*p
                .offset((*e).prevrawlensize as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                as libc::c_uint;
        } else if (*e).encoding as libc::c_int == (1 as libc::c_int) << 6 as libc::c_int
        {
            (*e).lensize = 2 as libc::c_int as libc::c_uint;
            (*e)
                .len = ((*p
                .offset((*e).prevrawlensize as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                << 8 as libc::c_int
                | *p
                    .offset((*e).prevrawlensize as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        } else if (*e).encoding as libc::c_int == (2 as libc::c_int) << 6 as libc::c_int
        {
            (*e).lensize = 5 as libc::c_int as libc::c_uint;
            (*e)
                .len = (*p
                .offset((*e).prevrawlensize as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                | (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(3 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *p
                    .offset((*e).prevrawlensize as isize)
                    .offset(4 as libc::c_int as isize) as uint32_t;
        } else {
            (*e).lensize = 0 as libc::c_int as libc::c_uint;
            (*e).len = 0 as libc::c_int as libc::c_uint;
        }
    } else {
        (*e).lensize = 1 as libc::c_int as libc::c_uint;
        if (*e).encoding as libc::c_int == 0xfe as libc::c_int {
            (*e).len = 1 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 2 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 3 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 4 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 8 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int >= 0xf1 as libc::c_int
            && (*e).encoding as libc::c_int <= 0xfd as libc::c_int
        {
            (*e).len = 0 as libc::c_int as libc::c_uint;
        } else {
            (*e).len = 0 as libc::c_int as libc::c_uint;
            (*e).lensize = (*e).len;
        }
    }
    if ((*e).lensize != 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
        != 0
    {} else {
        _serverAssert(
            b"e->lensize != 0\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            620 as libc::c_int,
        );
        unreachable!();
    };
    (*e).headersize = ((*e).prevrawlensize).wrapping_add((*e).lensize);
    (*e).p = p;
}
#[inline]
unsafe extern "C" fn zipEntrySafe(
    mut zl: *mut libc::c_uchar,
    mut zlbytes: size_t,
    mut p: *mut libc::c_uchar,
    mut e: *mut zlentry,
    mut validate_prevlen: libc::c_int,
) -> libc::c_int {
    let mut zlfirst: *mut libc::c_uchar = zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                as isize,
        );
    let mut zllast: *mut libc::c_uchar = zl
        .offset(zlbytes as isize)
        .offset(-(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize));
    if p >= zlfirst && p.offset(10 as libc::c_int as isize) < zllast {
        if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
            (*e).prevrawlensize = 1 as libc::c_int as libc::c_uint;
        } else {
            (*e).prevrawlensize = 5 as libc::c_int as libc::c_uint;
        }
        if (*e).prevrawlensize == 1 as libc::c_int as libc::c_uint {
            (*e).prevrawlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
        } else {
            (*e)
                .prevrawlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
                << 24 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                | (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        }
        (*e)
            .encoding = *p
            .offset((*e).prevrawlensize as isize)
            .offset(0 as libc::c_int as isize);
        if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
            (*e)
                .encoding = ((*e).encoding as libc::c_int & 0xc0 as libc::c_int)
                as libc::c_uchar;
        }
        if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
            if (*e).encoding as libc::c_int == (0 as libc::c_int) << 6 as libc::c_int {
                (*e).lensize = 1 as libc::c_int as libc::c_uint;
                (*e)
                    .len = (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x3f as libc::c_int) as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == (1 as libc::c_int) << 6 as libc::c_int
            {
                (*e).lensize = 2 as libc::c_int as libc::c_uint;
                (*e)
                    .len = ((*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(0 as libc::c_int as isize) as libc::c_int
                    & 0x3f as libc::c_int) << 8 as libc::c_int
                    | *p
                        .offset((*e).prevrawlensize as isize)
                        .offset(1 as libc::c_int as isize) as libc::c_int)
                    as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == (2 as libc::c_int) << 6 as libc::c_int
            {
                (*e).lensize = 5 as libc::c_int as libc::c_uint;
                (*e)
                    .len = (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(1 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                    | (*p
                        .offset((*e).prevrawlensize as isize)
                        .offset(2 as libc::c_int as isize) as uint32_t)
                        << 16 as libc::c_int
                    | (*p
                        .offset((*e).prevrawlensize as isize)
                        .offset(3 as libc::c_int as isize) as uint32_t)
                        << 8 as libc::c_int
                    | *p
                        .offset((*e).prevrawlensize as isize)
                        .offset(4 as libc::c_int as isize) as uint32_t;
            } else {
                (*e).lensize = 0 as libc::c_int as libc::c_uint;
                (*e).len = 0 as libc::c_int as libc::c_uint;
            }
        } else {
            (*e).lensize = 1 as libc::c_int as libc::c_uint;
            if (*e).encoding as libc::c_int == 0xfe as libc::c_int {
                (*e).len = 1 as libc::c_int as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
            {
                (*e).len = 2 as libc::c_int as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
            {
                (*e).len = 3 as libc::c_int as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
            {
                (*e).len = 4 as libc::c_int as libc::c_uint;
            } else if (*e).encoding as libc::c_int
                == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
            {
                (*e).len = 8 as libc::c_int as libc::c_uint;
            } else if (*e).encoding as libc::c_int >= 0xf1 as libc::c_int
                && (*e).encoding as libc::c_int <= 0xfd as libc::c_int
            {
                (*e).len = 0 as libc::c_int as libc::c_uint;
            } else {
                (*e).len = 0 as libc::c_int as libc::c_uint;
                (*e).lensize = (*e).len;
            }
        }
        (*e).headersize = ((*e).prevrawlensize).wrapping_add((*e).lensize);
        (*e).p = p;
        if ((*e).lensize == 0 as libc::c_int as libc::c_uint) as libc::c_int
            as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        if (p.offset((*e).headersize as isize).offset((*e).len as isize) < zlfirst
            || p.offset((*e).headersize as isize).offset((*e).len as isize) > zllast)
            as libc::c_int as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        if validate_prevlen != 0
            && (p.offset(-((*e).prevrawlen as isize)) < zlfirst
                || p.offset(-((*e).prevrawlen as isize)) > zllast) as libc::c_int
                as libc::c_long != 0
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if (p < zlfirst || p > zllast) as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int;
    }
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
        (*e).prevrawlensize = 1 as libc::c_int as libc::c_uint;
    } else {
        (*e).prevrawlensize = 5 as libc::c_int as libc::c_uint;
    }
    if (p.offset((*e).prevrawlensize as isize) < zlfirst
        || p.offset((*e).prevrawlensize as isize) > zllast) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    (*e)
        .encoding = *p
        .offset((*e).prevrawlensize as isize)
        .offset(0 as libc::c_int as isize);
    if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
        (*e)
            .encoding = ((*e).encoding as libc::c_int & 0xc0 as libc::c_int)
            as libc::c_uchar;
    }
    (*e).lensize = zipEncodingLenSize((*e).encoding);
    if ((*e).lensize == 0xff as libc::c_int as libc::c_uint) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    if (p.offset((*e).prevrawlensize as isize).offset((*e).lensize as isize) < zlfirst
        || p.offset((*e).prevrawlensize as isize).offset((*e).lensize as isize) > zllast)
        as libc::c_int as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
        (*e).prevrawlensize = 1 as libc::c_int as libc::c_uint;
    } else {
        (*e).prevrawlensize = 5 as libc::c_int as libc::c_uint;
    }
    if (*e).prevrawlensize == 1 as libc::c_int as libc::c_uint {
        (*e).prevrawlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
    } else {
        (*e)
            .prevrawlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
            << 24 as libc::c_int
            | (*p.offset(3 as libc::c_int as isize) as libc::c_int) << 16 as libc::c_int
            | (*p.offset(2 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
            | *p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
    }
    if ((*e).encoding as libc::c_int) < 0xc0 as libc::c_int {
        if (*e).encoding as libc::c_int == (0 as libc::c_int) << 6 as libc::c_int {
            (*e).lensize = 1 as libc::c_int as libc::c_uint;
            (*e)
                .len = (*p
                .offset((*e).prevrawlensize as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                as libc::c_uint;
        } else if (*e).encoding as libc::c_int == (1 as libc::c_int) << 6 as libc::c_int
        {
            (*e).lensize = 2 as libc::c_int as libc::c_uint;
            (*e)
                .len = ((*p
                .offset((*e).prevrawlensize as isize)
                .offset(0 as libc::c_int as isize) as libc::c_int & 0x3f as libc::c_int)
                << 8 as libc::c_int
                | *p
                    .offset((*e).prevrawlensize as isize)
                    .offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        } else if (*e).encoding as libc::c_int == (2 as libc::c_int) << 6 as libc::c_int
        {
            (*e).lensize = 5 as libc::c_int as libc::c_uint;
            (*e)
                .len = (*p
                .offset((*e).prevrawlensize as isize)
                .offset(1 as libc::c_int as isize) as uint32_t) << 24 as libc::c_int
                | (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(2 as libc::c_int as isize) as uint32_t) << 16 as libc::c_int
                | (*p
                    .offset((*e).prevrawlensize as isize)
                    .offset(3 as libc::c_int as isize) as uint32_t) << 8 as libc::c_int
                | *p
                    .offset((*e).prevrawlensize as isize)
                    .offset(4 as libc::c_int as isize) as uint32_t;
        } else {
            (*e).lensize = 0 as libc::c_int as libc::c_uint;
            (*e).len = 0 as libc::c_int as libc::c_uint;
        }
    } else {
        (*e).lensize = 1 as libc::c_int as libc::c_uint;
        if (*e).encoding as libc::c_int == 0xfe as libc::c_int {
            (*e).len = 1 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (0 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 2 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (3 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 3 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 4 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int
            == 0xc0 as libc::c_int | (2 as libc::c_int) << 4 as libc::c_int
        {
            (*e).len = 8 as libc::c_int as libc::c_uint;
        } else if (*e).encoding as libc::c_int >= 0xf1 as libc::c_int
            && (*e).encoding as libc::c_int <= 0xfd as libc::c_int
        {
            (*e).len = 0 as libc::c_int as libc::c_uint;
        } else {
            (*e).len = 0 as libc::c_int as libc::c_uint;
            (*e).lensize = (*e).len;
        }
    }
    (*e).headersize = ((*e).prevrawlensize).wrapping_add((*e).lensize);
    if (p.offset((*e).headersize as isize).offset((*e).len as isize) < zlfirst
        || p.offset((*e).headersize as isize).offset((*e).len as isize) > zllast)
        as libc::c_int as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    if validate_prevlen != 0
        && (p.offset(-((*e).prevrawlen as isize)) < zlfirst
            || p.offset(-((*e).prevrawlen as isize)) > zllast) as libc::c_int
            as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    (*e).p = p;
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn zipRawEntryLengthSafe(
    mut zl: *mut libc::c_uchar,
    mut zlbytes: size_t,
    mut p: *mut libc::c_uchar,
) -> libc::c_uint {
    let mut e: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    if (zipEntrySafe(zl, zlbytes, p, &mut e, 0 as libc::c_int) != 0) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"zipEntrySafe(zl, zlbytes, p, &e, 0)\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            694 as libc::c_int,
        );
        unreachable!();
    };
    return (e.headersize).wrapping_add(e.len);
}
#[inline]
unsafe extern "C" fn zipRawEntryLength(mut p: *mut libc::c_uchar) -> libc::c_uint {
    let mut e: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    zipEntry(p, &mut e);
    return (e.headersize).wrapping_add(e.len);
}
#[inline]
unsafe extern "C" fn zipAssertValidEntry(
    mut zl: *mut libc::c_uchar,
    mut zlbytes: size_t,
    mut p: *mut libc::c_uchar,
) {
    let mut e: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    if (zipEntrySafe(zl, zlbytes, p, &mut e, 1 as libc::c_int) != 0) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"zipEntrySafe(zl, zlbytes, p, &e, 1)\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            708 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn ziplistNew() -> *mut libc::c_uchar {
    let mut bytes: libc::c_uint = (core::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
        .wrapping_add(core::mem::size_of::<uint8_t>() as libc::c_ulong)
        as libc::c_uint;
    let mut zl: *mut libc::c_uchar = zmalloc(bytes as size_t) as *mut libc::c_uchar;
    *(zl as *mut uint32_t) = bytes;
    *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
        as *mut uint32_t) = (core::mem::size_of::<uint32_t>() as libc::c_ulong)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
        .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong) as uint32_t;
    *(zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) = 0 as libc::c_int as uint16_t;
    *zl
        .offset(
            bytes.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = 255 as libc::c_int as libc::c_uchar;
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistResize(
    mut zl: *mut libc::c_uchar,
    mut len: size_t,
) -> *mut libc::c_uchar {
    if (len < 4294967295 as libc::c_uint as libc::c_ulong) as libc::c_int as libc::c_long
        != 0
    {} else {
        _serverAssert(
            b"len < UINT32_MAX\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            724 as libc::c_int,
        );
        unreachable!();
    };
    zl = zrealloc(zl as *mut libc::c_void, len) as *mut libc::c_uchar;
    *(zl as *mut uint32_t) = len as uint32_t;
    *zl
        .offset(
            len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = 255 as libc::c_int as libc::c_uchar;
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn __ziplistCascadeUpdate(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut cur: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    let mut prevlen: size_t = 0;
    let mut prevlensize: size_t = 0;
    let mut prevoffset: size_t = 0;
    let mut firstentrylen: size_t = 0;
    let mut rawlen: size_t = 0;
    let mut curlen: size_t = *(zl as *mut uint32_t) as size_t;
    let mut extra: size_t = 0 as libc::c_int as size_t;
    let mut cnt: size_t = 0 as libc::c_int as size_t;
    let mut offset: size_t = 0;
    let mut delta: size_t = 4 as libc::c_int as size_t;
    let mut tail: *mut libc::c_uchar = zl
        .offset(
            *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t) as isize,
        );
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        return zl;
    }
    zipEntry(p, &mut cur);
    prevlen = (cur.headersize).wrapping_add(cur.len) as size_t;
    firstentrylen = prevlen;
    prevlensize = zipStorePrevEntryLength(
        0 as *mut libc::c_uchar,
        prevlen as libc::c_uint,
    ) as size_t;
    prevoffset = p.offset_from(zl) as libc::c_long as size_t;
    p = p.offset(prevlen as isize);
    while *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
        if (zipEntrySafe(zl, curlen, p, &mut cur, 0 as libc::c_int) != 0) as libc::c_int
            as libc::c_long != 0
        {} else {
            _serverAssert(
                b"zipEntrySafe(zl, curlen, p, &cur, 0)\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                771 as libc::c_int,
            );
            unreachable!();
        };
        if cur.prevrawlen as libc::c_ulong == prevlen {
            break;
        }
        if cur.prevrawlensize as libc::c_ulong >= prevlensize {
            if cur.prevrawlensize as libc::c_ulong == prevlensize {
                zipStorePrevEntryLength(p, prevlen as libc::c_uint);
            } else {
                zipStorePrevEntryLengthLarge(p, prevlen as libc::c_uint);
            }
            break;
        } else {
            if (cur.prevrawlen == 0 as libc::c_int as libc::c_uint
                || (cur.prevrawlen as libc::c_ulong).wrapping_add(delta) == prevlen)
                as libc::c_int as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"cur.prevrawlen == 0 || cur.prevrawlen + delta == prevlen\0"
                        as *const u8 as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    789 as libc::c_int,
                );
                unreachable!();
            };
            rawlen = (cur.headersize).wrapping_add(cur.len) as size_t;
            prevlen = rawlen.wrapping_add(delta);
            prevlensize = zipStorePrevEntryLength(
                0 as *mut libc::c_uchar,
                prevlen as libc::c_uint,
            ) as size_t;
            prevoffset = p.offset_from(zl) as libc::c_long as size_t;
            p = p.offset(rawlen as isize);
            extra = (extra as libc::c_ulong).wrapping_add(delta) as size_t as size_t;
            cnt = cnt.wrapping_add(1);
        }
    }
    if extra == 0 as libc::c_int as libc::c_ulong {
        return zl;
    }
    if tail == zl.offset(prevoffset as isize) {
        if extra.wrapping_sub(delta) != 0 as libc::c_int as libc::c_ulong {
            *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t) = (*(zl
                .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t) as libc::c_ulong)
                .wrapping_add(extra)
                .wrapping_sub(delta) as uint32_t;
        }
    } else {
        *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) = (*(zl
            .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) as libc::c_ulong)
            .wrapping_add(extra) as uint32_t;
    }
    offset = p.offset_from(zl) as libc::c_long as size_t;
    zl = ziplistResize(zl, curlen.wrapping_add(extra));
    p = zl.offset(offset as isize);
    memmove(
        p.offset(extra as isize) as *mut libc::c_void,
        p as *const libc::c_void,
        curlen.wrapping_sub(offset).wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    p = p.offset(extra as isize);
    while cnt != 0 {
        zipEntry(zl.offset(prevoffset as isize), &mut cur);
        rawlen = (cur.headersize).wrapping_add(cur.len) as size_t;
        memmove(
            p
                .offset(
                    -(rawlen.wrapping_sub(cur.prevrawlensize as libc::c_ulong) as isize),
                ) as *mut libc::c_void,
            zl.offset(prevoffset as isize).offset(cur.prevrawlensize as isize)
                as *const libc::c_void,
            rawlen.wrapping_sub(cur.prevrawlensize as libc::c_ulong),
        );
        p = p.offset(-(rawlen.wrapping_add(delta) as isize));
        if cur.prevrawlen == 0 as libc::c_int as libc::c_uint {
            zipStorePrevEntryLength(p, firstentrylen as libc::c_uint);
        } else {
            zipStorePrevEntryLength(
                p,
                (cur.prevrawlen as libc::c_ulong).wrapping_add(delta) as libc::c_uint,
            );
        }
        prevoffset = (prevoffset as libc::c_ulong)
            .wrapping_sub(cur.prevrawlen as libc::c_ulong) as size_t as size_t;
        cnt = cnt.wrapping_sub(1);
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn __ziplistDelete(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut num: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut i: libc::c_uint = 0;
    let mut totlen: libc::c_uint = 0;
    let mut deleted: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut offset: size_t = 0;
    let mut nextdiff: libc::c_int = 0 as libc::c_int;
    let mut first: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    let mut tail: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    let mut zlbytes: size_t = *(zl as *mut uint32_t) as size_t;
    zipEntry(p, &mut first);
    i = 0 as libc::c_int as libc::c_uint;
    while *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int
        && i < num
    {
        p = p.offset(zipRawEntryLengthSafe(zl, zlbytes, p) as isize);
        deleted = deleted.wrapping_add(1);
        i = i.wrapping_add(1);
    }
    if (p >= first.p) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"p >= first.p\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            863 as libc::c_int,
        );
        unreachable!();
    };
    totlen = p.offset_from(first.p) as libc::c_long as libc::c_uint;
    if totlen > 0 as libc::c_int as libc::c_uint {
        let mut set_tail: uint32_t = 0;
        if *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
            nextdiff = zipPrevLenByteDiff(p, first.prevrawlen);
            p = p.offset(-(nextdiff as isize));
            if (p >= first.p
                && p < zl.offset(zlbytes as isize).offset(-(1 as libc::c_int as isize)))
                as libc::c_int as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"p >= first.p && p<zl+zlbytes-1\0" as *const u8
                        as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    879 as libc::c_int,
                );
                unreachable!();
            };
            zipStorePrevEntryLength(p, first.prevrawlen);
            set_tail = (*(zl
                .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t))
                .wrapping_sub(totlen);
            if (zipEntrySafe(zl, zlbytes, p, &mut tail, 1 as libc::c_int) != 0)
                as libc::c_int as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"zipEntrySafe(zl, zlbytes, p, &tail, 1)\0" as *const u8
                        as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    888 as libc::c_int,
                );
                unreachable!();
            };
            if *p.offset((tail.headersize).wrapping_add(tail.len) as isize)
                as libc::c_int != 255 as libc::c_int
            {
                set_tail = set_tail.wrapping_add(nextdiff as libc::c_uint);
            }
            let mut bytes_to_move: size_t = zlbytes
                .wrapping_sub(p.offset_from(zl) as libc::c_long as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong);
            memmove(
                first.p as *mut libc::c_void,
                p as *const libc::c_void,
                bytes_to_move,
            );
        } else {
            set_tail = ((first.p).offset_from(zl) as libc::c_long
                - first.prevrawlen as libc::c_long) as uint32_t;
        }
        offset = (first.p).offset_from(zl) as libc::c_long as size_t;
        zlbytes = (zlbytes as libc::c_ulong)
            .wrapping_sub(totlen.wrapping_sub(nextdiff as libc::c_uint) as libc::c_ulong)
            as size_t as size_t;
        zl = ziplistResize(zl, zlbytes);
        p = zl.offset(offset as isize);
        if (*(zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut uint16_t) as libc::c_int) < 65535 as libc::c_int
        {
            *(zl
                .offset(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                )
                as *mut uint16_t) = (*(zl
                .offset(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                ) as *mut uint16_t) as libc::c_uint)
                .wrapping_add(deleted.wrapping_neg()) as uint16_t;
        }
        if (set_tail as libc::c_ulong
            <= zlbytes.wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong))
            as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"set_tail <= zlbytes - ZIPLIST_END_SIZE\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                914 as libc::c_int,
            );
            unreachable!();
        };
        *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) = set_tail;
        if nextdiff != 0 as libc::c_int {
            zl = __ziplistCascadeUpdate(zl, p);
        }
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn __ziplistInsert(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut curlen: size_t = *(zl as *mut uint32_t) as size_t;
    let mut reqlen: size_t = 0;
    let mut newlen: size_t = 0;
    let mut prevlensize: libc::c_uint = 0;
    let mut prevlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut offset: size_t = 0;
    let mut nextdiff: libc::c_int = 0 as libc::c_int;
    let mut encoding: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut value: libc::c_longlong = 123456789 as libc::c_int as libc::c_longlong;
    let mut tail: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
        if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
            prevlensize = 1 as libc::c_int as libc::c_uint;
        } else {
            prevlensize = 5 as libc::c_int as libc::c_uint;
        }
        if prevlensize == 1 as libc::c_int as libc::c_uint {
            prevlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
        } else {
            prevlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
                << 24 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                | (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        }
    } else {
        let mut ptail: *mut libc::c_uchar = zl
            .offset(
                *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                    as *mut uint32_t) as isize,
            );
        if *ptail.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int
        {
            prevlen = zipRawEntryLengthSafe(zl, curlen, ptail);
        }
    }
    if zipTryEncoding(s, slen, &mut value, &mut encoding) != 0 {
        reqlen = zipIntSize(encoding) as size_t;
    } else {
        reqlen = slen as size_t;
    }
    reqlen = (reqlen as libc::c_ulong)
        .wrapping_add(
            zipStorePrevEntryLength(0 as *mut libc::c_uchar, prevlen) as libc::c_ulong,
        ) as size_t as size_t;
    reqlen = (reqlen as libc::c_ulong)
        .wrapping_add(
            zipStoreEntryEncoding(0 as *mut libc::c_uchar, encoding, slen)
                as libc::c_ulong,
        ) as size_t as size_t;
    let mut forcelarge: libc::c_int = 0 as libc::c_int;
    nextdiff = if *p.offset(0 as libc::c_int as isize) as libc::c_int
        != 255 as libc::c_int
    {
        zipPrevLenByteDiff(p, reqlen as libc::c_uint)
    } else {
        0 as libc::c_int
    };
    if nextdiff == -(4 as libc::c_int) && reqlen < 4 as libc::c_int as libc::c_ulong {
        nextdiff = 0 as libc::c_int;
        forcelarge = 1 as libc::c_int;
    }
    offset = p.offset_from(zl) as libc::c_long as size_t;
    newlen = curlen.wrapping_add(reqlen).wrapping_add(nextdiff as libc::c_ulong);
    zl = ziplistResize(zl, newlen);
    p = zl.offset(offset as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
        memmove(
            p.offset(reqlen as isize) as *mut libc::c_void,
            p.offset(-(nextdiff as isize)) as *const libc::c_void,
            curlen
                .wrapping_sub(offset)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(nextdiff as libc::c_ulong),
        );
        if forcelarge != 0 {
            zipStorePrevEntryLengthLarge(
                p.offset(reqlen as isize),
                reqlen as libc::c_uint,
            );
        } else {
            zipStorePrevEntryLength(p.offset(reqlen as isize), reqlen as libc::c_uint);
        }
        *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) = (*(zl
            .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) as libc::c_ulong)
            .wrapping_add(reqlen) as uint32_t;
        if (zipEntrySafe(
            zl,
            newlen,
            p.offset(reqlen as isize),
            &mut tail,
            1 as libc::c_int,
        ) != 0) as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"zipEntrySafe(zl, newlen, p+reqlen, &tail, 1)\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                995 as libc::c_int,
            );
            unreachable!();
        };
        if *p
            .offset(
                reqlen
                    .wrapping_add(tail.headersize as libc::c_ulong)
                    .wrapping_add(tail.len as libc::c_ulong) as isize,
            ) as libc::c_int != 255 as libc::c_int
        {
            *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t) = (*(zl
                .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                as *mut uint32_t))
                .wrapping_add(nextdiff as libc::c_uint);
        }
    } else {
        *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t) = p.offset_from(zl) as libc::c_long as uint32_t;
    }
    if nextdiff != 0 as libc::c_int {
        offset = p.offset_from(zl) as libc::c_long as size_t;
        zl = __ziplistCascadeUpdate(zl, p.offset(reqlen as isize));
        p = zl.offset(offset as isize);
    }
    p = p.offset(zipStorePrevEntryLength(p, prevlen) as isize);
    p = p.offset(zipStoreEntryEncoding(p, encoding, slen) as isize);
    if (encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
        memcpy(p as *mut libc::c_void, s as *const libc::c_void, slen as libc::c_ulong);
    } else {
        zipSaveInteger(p, value as int64_t, encoding);
    }
    if (*(zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) as libc::c_int) < 65535 as libc::c_int
    {
        *(zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
            )
            as *mut uint16_t) = (*(zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut uint16_t) as libc::c_int + 1 as libc::c_int) as uint16_t;
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistMerge(
    mut first: *mut *mut libc::c_uchar,
    mut second: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    if first.is_null() || (*first).is_null() || second.is_null() || (*second).is_null() {
        return 0 as *mut libc::c_uchar;
    }
    if *first == *second {
        return 0 as *mut libc::c_uchar;
    }
    let mut first_bytes: size_t = *(*first as *mut uint32_t) as size_t;
    let mut first_len: size_t = *((*first)
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) as size_t;
    let mut second_bytes: size_t = *(*second as *mut uint32_t) as size_t;
    let mut second_len: size_t = *((*second)
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) as size_t;
    let mut append: libc::c_int = 0;
    let mut source: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut target: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut target_bytes: size_t = 0;
    let mut source_bytes: size_t = 0;
    if first_len >= second_len {
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
    let mut zlbytes: size_t = first_bytes
        .wrapping_add(second_bytes)
        .wrapping_sub(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong),
        )
        .wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong);
    let mut zllength: size_t = first_len.wrapping_add(second_len);
    zllength = if zllength < 65535 as libc::c_int as libc::c_ulong {
        zllength
    } else {
        65535 as libc::c_int as libc::c_ulong
    };
    if (zlbytes < 4294967295 as libc::c_uint as libc::c_ulong) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"zlbytes < UINT32_MAX\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            1086 as libc::c_int,
        );
        unreachable!();
    };
    let mut first_offset: size_t = *((*first)
        .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
        as *mut uint32_t) as size_t;
    let mut second_offset: size_t = *((*second)
        .offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
        as *mut uint32_t) as size_t;
    target = zrealloc(target as *mut libc::c_void, zlbytes) as *mut libc::c_uchar;
    if append != 0 {
        memcpy(
            target
                .offset(target_bytes as isize)
                .offset(-(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize))
                as *mut libc::c_void,
            source
                .offset(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        ) as isize,
                ) as *const libc::c_void,
            source_bytes
                .wrapping_sub(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        ),
                ),
        );
    } else {
        memmove(
            target
                .offset(source_bytes as isize)
                .offset(-(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize))
                as *mut libc::c_void,
            target
                .offset(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        ) as isize,
                ) as *const libc::c_void,
            target_bytes
                .wrapping_sub(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        ),
                ),
        );
        memcpy(
            target as *mut libc::c_void,
            source as *const libc::c_void,
            source_bytes.wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong),
        );
    }
    *(target as *mut uint32_t) = zlbytes as uint32_t;
    *(target
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) = zllength as uint16_t;
    *(target.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
        as *mut uint32_t) = first_bytes
        .wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong)
        .wrapping_add(
            second_offset
                .wrapping_sub(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                        .wrapping_add(
                            core::mem::size_of::<uint16_t>() as libc::c_ulong,
                        ),
                ),
        ) as uint32_t;
    target = __ziplistCascadeUpdate(target, target.offset(first_offset as isize));
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
pub unsafe extern "C" fn ziplistPush(
    mut zl: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: libc::c_uint,
    mut where_0: libc::c_int,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = if where_0 == 0 as libc::c_int {
        zl.offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                as isize,
        )
    } else {
        zl.offset(*(zl as *mut uint32_t) as isize)
            .offset(-(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize))
    };
    return __ziplistInsert(zl, p, s, slen);
}
#[no_mangle]
pub unsafe extern "C" fn ziplistIndex(
    mut zl: *mut libc::c_uchar,
    mut index: libc::c_int,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prevlensize: libc::c_uint = 0;
    let mut prevlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut zlbytes: size_t = *(zl as *mut uint32_t) as size_t;
    if index < 0 as libc::c_int {
        index = -index - 1 as libc::c_int;
        p = zl
            .offset(
                *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                    as *mut uint32_t) as isize,
            );
        if *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
            if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int
            {
                prevlensize = 1 as libc::c_int as libc::c_uint;
            } else {
                prevlensize = 5 as libc::c_int as libc::c_uint;
            }
            if (p.offset(prevlensize as isize)
                < zl
                    .offset(zlbytes as isize)
                    .offset(
                        -(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize),
                    )) as libc::c_int as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"p + prevlensize < zl + zlbytes - ZIPLIST_END_SIZE\0" as *const u8
                        as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    1164 as libc::c_int,
                );
                unreachable!();
            };
            if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int
            {
                prevlensize = 1 as libc::c_int as libc::c_uint;
            } else {
                prevlensize = 5 as libc::c_int as libc::c_uint;
            }
            if prevlensize == 1 as libc::c_int as libc::c_uint {
                prevlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
            } else {
                prevlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
                    << 24 as libc::c_int
                    | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                        << 16 as libc::c_int
                    | (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                        << 8 as libc::c_int
                    | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                    as libc::c_uint;
            }
            while prevlen > 0 as libc::c_int as libc::c_uint
                && {
                    let fresh0 = index;
                    index = index - 1;
                    fresh0 != 0
                }
            {
                p = p.offset(-(prevlen as isize));
                if (p
                    >= zl
                        .offset(
                            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                .wrapping_add(
                                    core::mem::size_of::<uint16_t>() as libc::c_ulong,
                                ) as isize,
                        )
                    && p
                        < zl
                            .offset(zlbytes as isize)
                            .offset(
                                -(core::mem::size_of::<uint8_t>() as libc::c_ulong
                                    as isize),
                            )) as libc::c_int as libc::c_long != 0
                {} else {
                    _serverAssert(
                        b"p >= zl + ZIPLIST_HEADER_SIZE && p < zl + zlbytes - ZIPLIST_END_SIZE\0"
                            as *const u8 as *const libc::c_char,
                        b"ziplist.c\0" as *const u8 as *const libc::c_char,
                        1168 as libc::c_int,
                    );
                    unreachable!();
                };
                if (*p.offset(0 as libc::c_int as isize) as libc::c_int)
                    < 254 as libc::c_int
                {
                    prevlensize = 1 as libc::c_int as libc::c_uint;
                } else {
                    prevlensize = 5 as libc::c_int as libc::c_uint;
                }
                if prevlensize == 1 as libc::c_int as libc::c_uint {
                    prevlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
                } else {
                    prevlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
                        << 24 as libc::c_int
                        | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                            << 16 as libc::c_int
                        | (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                            << 8 as libc::c_int
                        | *p.offset(1 as libc::c_int as isize) as libc::c_int)
                        as libc::c_uint;
                }
            }
        }
    } else {
        p = zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            );
        loop {
            let fresh1 = index;
            index = index - 1;
            if !(fresh1 != 0) {
                break;
            }
            p = p.offset(zipRawEntryLengthSafe(zl, zlbytes, p) as isize);
            if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int
            {
                break;
            }
        }
    }
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int
        || index > 0 as libc::c_int
    {
        return 0 as *mut libc::c_uchar;
    }
    zipAssertValidEntry(zl, zlbytes, p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistNext(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut zlbytes: size_t = *(zl as *mut uint32_t) as size_t;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    p = p.offset(zipRawEntryLength(p) as isize);
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    zipAssertValidEntry(zl, zlbytes, p);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistPrev(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut prevlensize: libc::c_uint = 0;
    let mut prevlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        p = zl
            .offset(
                *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
                    as *mut uint32_t) as isize,
            );
        return if *p.offset(0 as libc::c_int as isize) as libc::c_int
            == 255 as libc::c_int
        {
            0 as *mut libc::c_uchar
        } else {
            p
        };
    } else if p
        == zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            )
    {
        return 0 as *mut libc::c_uchar
    } else {
        if (*p.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
            prevlensize = 1 as libc::c_int as libc::c_uint;
        } else {
            prevlensize = 5 as libc::c_int as libc::c_uint;
        }
        if prevlensize == 1 as libc::c_int as libc::c_uint {
            prevlen = *p.offset(0 as libc::c_int as isize) as libc::c_uint;
        } else {
            prevlen = ((*p.offset(4 as libc::c_int as isize) as libc::c_int)
                << 24 as libc::c_int
                | (*p.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int
                | (*p.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                | *p.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
        }
        if (prevlen > 0 as libc::c_int as libc::c_uint) as libc::c_int as libc::c_long
            != 0
        {} else {
            _serverAssert(
                b"prevlen > 0\0" as *const u8 as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                1228 as libc::c_int,
            );
            unreachable!();
        };
        p = p.offset(-(prevlen as isize));
        let mut zlbytes: size_t = *(zl as *mut uint32_t) as size_t;
        zipAssertValidEntry(zl, zlbytes, p);
        return p;
    };
}
#[no_mangle]
pub unsafe extern "C" fn ziplistGet(
    mut p: *mut libc::c_uchar,
    mut sstr: *mut *mut libc::c_uchar,
    mut slen: *mut libc::c_uint,
    mut sval: *mut libc::c_longlong,
) -> libc::c_uint {
    let mut entry: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    if p.is_null()
        || *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int
    {
        return 0 as libc::c_int as libc::c_uint;
    }
    if !sstr.is_null() {
        *sstr = 0 as *mut libc::c_uchar;
    }
    zipEntry(p, &mut entry);
    if (entry.encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
        if !sstr.is_null() {
            *slen = entry.len;
            *sstr = p.offset(entry.headersize as isize);
        }
    } else if !sval.is_null() {
        *sval = zipLoadInteger(p.offset(entry.headersize as isize), entry.encoding)
            as libc::c_longlong;
    }
    return 1 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistInsert(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: libc::c_uint,
) -> *mut libc::c_uchar {
    return __ziplistInsert(zl, p, s, slen);
}
#[no_mangle]
pub unsafe extern "C" fn ziplistDelete(
    mut zl: *mut libc::c_uchar,
    mut p: *mut *mut libc::c_uchar,
) -> *mut libc::c_uchar {
    let mut offset: size_t = (*p).offset_from(zl) as libc::c_long as size_t;
    zl = __ziplistDelete(zl, *p, 1 as libc::c_int as libc::c_uint);
    *p = zl.offset(offset as isize);
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistDeleteRange(
    mut zl: *mut libc::c_uchar,
    mut index: libc::c_int,
    mut num: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = ziplistIndex(zl, index);
    return if p.is_null() { zl } else { __ziplistDelete(zl, p, num) };
}
#[no_mangle]
pub unsafe extern "C" fn ziplistReplace(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut s: *mut libc::c_uchar,
    mut slen: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut entry: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    zipEntry(p, &mut entry);
    let mut reqlen: libc::c_uint = 0;
    let mut encoding: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut value: libc::c_longlong = 123456789 as libc::c_int as libc::c_longlong;
    if zipTryEncoding(s, slen, &mut value, &mut encoding) != 0 {
        reqlen = zipIntSize(encoding);
    } else {
        reqlen = slen;
    }
    reqlen = reqlen
        .wrapping_add(zipStoreEntryEncoding(0 as *mut libc::c_uchar, encoding, slen));
    if reqlen == (entry.lensize).wrapping_add(entry.len) {
        p = p.offset(entry.prevrawlensize as isize);
        p = p.offset(zipStoreEntryEncoding(p, encoding, slen) as isize);
        if (encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
            memcpy(
                p as *mut libc::c_void,
                s as *const libc::c_void,
                slen as libc::c_ulong,
            );
        } else {
            zipSaveInteger(p, value as int64_t, encoding);
        }
    } else {
        zl = ziplistDelete(zl, &mut p);
        zl = ziplistInsert(zl, p, s, slen);
    }
    return zl;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistCompare(
    mut p: *mut libc::c_uchar,
    mut sstr: *mut libc::c_uchar,
    mut slen: libc::c_uint,
) -> libc::c_uint {
    let mut entry: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    let mut sencoding: libc::c_uchar = 0;
    let mut zval: libc::c_longlong = 0;
    let mut sval: libc::c_longlong = 0;
    if *p.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        return 0 as libc::c_int as libc::c_uint;
    }
    zipEntry(p, &mut entry);
    if (entry.encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
        if entry.len == slen {
            return (memcmp(
                p.offset(entry.headersize as isize) as *const libc::c_void,
                sstr as *const libc::c_void,
                slen as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int as libc::c_uint
        } else {
            return 0 as libc::c_int as libc::c_uint
        }
    } else {
        if zipTryEncoding(sstr, slen, &mut sval, &mut sencoding) != 0 {
            zval = zipLoadInteger(p.offset(entry.headersize as isize), entry.encoding)
                as libc::c_longlong;
            return (zval == sval) as libc::c_int as libc::c_uint;
        }
    }
    return 0 as libc::c_int as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistFind(
    mut zl: *mut libc::c_uchar,
    mut p: *mut libc::c_uchar,
    mut vstr: *mut libc::c_uchar,
    mut vlen: libc::c_uint,
    mut skip: libc::c_uint,
) -> *mut libc::c_uchar {
    let mut skipcnt: libc::c_int = 0 as libc::c_int;
    let mut vencoding: libc::c_uchar = 0 as libc::c_int as libc::c_uchar;
    let mut vll: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut zlbytes: size_t = ziplistBlobLen(zl);
    while *p.offset(0 as libc::c_int as isize) as libc::c_int != 255 as libc::c_int {
        let mut e: zlentry = zlentry {
            prevrawlensize: 0,
            prevrawlen: 0,
            lensize: 0,
            len: 0,
            headersize: 0,
            encoding: 0,
            p: 0 as *mut libc::c_uchar,
        };
        let mut q: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        if (zipEntrySafe(zl, zlbytes, p, &mut e, 1 as libc::c_int) != 0) as libc::c_int
            as libc::c_long != 0
        {} else {
            _serverAssert(
                b"zipEntrySafe(zl, zlbytes, p, &e, 1)\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                1360 as libc::c_int,
            );
            unreachable!();
        };
        q = p.offset(e.prevrawlensize as isize).offset(e.lensize as isize);
        if skipcnt == 0 as libc::c_int {
            if (e.encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
                if e.len == vlen
                    && memcmp(
                        q as *const libc::c_void,
                        vstr as *const libc::c_void,
                        vlen as libc::c_ulong,
                    ) == 0 as libc::c_int
                {
                    return p;
                }
            } else {
                if vencoding as libc::c_int == 0 as libc::c_int {
                    if zipTryEncoding(vstr, vlen, &mut vll, &mut vencoding) == 0 {
                        vencoding = (127 as libc::c_int * 2 as libc::c_int
                            + 1 as libc::c_int) as libc::c_uchar;
                    }
                    if (vencoding != 0) as libc::c_int as libc::c_long != 0 {} else {
                        _serverAssert(
                            b"vencoding\0" as *const u8 as *const libc::c_char,
                            b"ziplist.c\0" as *const u8 as *const libc::c_char,
                            1381 as libc::c_int,
                        );
                        unreachable!();
                    };
                }
                if vencoding as libc::c_int
                    != 127 as libc::c_int * 2 as libc::c_int + 1 as libc::c_int
                {
                    let mut ll: libc::c_longlong = zipLoadInteger(q, e.encoding)
                        as libc::c_longlong;
                    if ll == vll {
                        return p;
                    }
                }
            }
            skipcnt = skip as libc::c_int;
        } else {
            skipcnt -= 1;
        }
        p = q.offset(e.len as isize);
    }
    return 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistLen(mut zl: *mut libc::c_uchar) -> libc::c_uint {
    let mut len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*(zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) as libc::c_int) < 65535 as libc::c_int
    {
        len = *(zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut uint16_t) as libc::c_uint;
    } else {
        let mut p: *mut libc::c_uchar = zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                    as isize,
            );
        let mut zlbytes: size_t = *(zl as *mut uint32_t) as size_t;
        while *p as libc::c_int != 255 as libc::c_int {
            p = p.offset(zipRawEntryLengthSafe(zl, zlbytes, p) as isize);
            len = len.wrapping_add(1);
        }
        if len < 65535 as libc::c_int as libc::c_uint {
            *(zl
                .offset(
                    (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
                ) as *mut uint16_t) = len as uint16_t;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistBlobLen(mut zl: *mut libc::c_uchar) -> size_t {
    return *(zl as *mut uint32_t) as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistRepr(mut zl: *mut libc::c_uchar) {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut entry: zlentry = zlentry {
        prevrawlensize: 0,
        prevrawlen: 0,
        lensize: 0,
        len: 0,
        headersize: 0,
        encoding: 0,
        p: 0 as *mut libc::c_uchar,
    };
    let mut zlbytes: size_t = ziplistBlobLen(zl);
    printf(
        b"{total bytes %u} {num entries %u}\n{tail offset %u}\n\0" as *const u8
            as *const libc::c_char,
        *(zl as *mut uint32_t),
        *(zl
            .offset(
                (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut uint16_t) as libc::c_int,
        *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
            as *mut uint32_t),
    );
    p = zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                as isize,
        );
    while *p as libc::c_int != 255 as libc::c_int {
        if (zipEntrySafe(zl, zlbytes, p, &mut entry, 1 as libc::c_int) != 0)
            as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"zipEntrySafe(zl, zlbytes, p, &entry, 1)\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                1448 as libc::c_int,
            );
            unreachable!();
        };
        printf(
            b"{\n\taddr 0x%08lx,\n\tindex %2d,\n\toffset %5lu,\n\thdr+entry len: %5u,\n\thdr len%2u,\n\tprevrawlen: %5u,\n\tprevrawlensize: %2u,\n\tpayload %5u\n\0"
                as *const u8 as *const libc::c_char,
            p as libc::c_ulong,
            index,
            p.offset_from(zl) as libc::c_long as libc::c_ulong,
            (entry.headersize).wrapping_add(entry.len),
            entry.headersize,
            entry.prevrawlen,
            entry.prevrawlensize,
            entry.len,
        );
        printf(b"\tbytes: \0" as *const u8 as *const libc::c_char);
        let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while i < (entry.headersize).wrapping_add(entry.len) {
            printf(
                b"%02x|\0" as *const u8 as *const libc::c_char,
                *p.offset(i as isize) as libc::c_int,
            );
            i = i.wrapping_add(1);
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
        p = p.offset(entry.headersize as isize);
        if (entry.encoding as libc::c_int & 0xc0 as libc::c_int) < 0xc0 as libc::c_int {
            printf(b"\t[str]\0" as *const u8 as *const libc::c_char);
            if entry.len > 40 as libc::c_int as libc::c_uint {
                if fwrite(
                    p as *const libc::c_void,
                    40 as libc::c_int as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    stdout,
                ) == 0 as libc::c_int as libc::c_ulong
                {
                    perror(b"fwrite\0" as *const u8 as *const libc::c_char);
                }
                printf(b"...\0" as *const u8 as *const libc::c_char);
            } else if entry.len != 0
                && fwrite(
                    p as *const libc::c_void,
                    entry.len as libc::c_ulong,
                    1 as libc::c_int as libc::c_ulong,
                    stdout,
                ) == 0 as libc::c_int as libc::c_ulong
            {
                perror(b"fwrite\0" as *const u8 as *const libc::c_char);
            }
        } else {
            printf(
                b"\t[int]%lld\0" as *const u8 as *const libc::c_char,
                zipLoadInteger(p, entry.encoding) as libc::c_longlong,
            );
        }
        printf(b"\n}\n\0" as *const u8 as *const libc::c_char);
        p = p.offset(entry.len as isize);
        index += 1;
    }
    printf(b"{end}\n\n\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn ziplistValidateIntegrity(
    mut zl: *mut libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
    mut entry_cb: ziplistValidateEntryCB,
    mut cb_userdata: *mut libc::c_void,
) -> libc::c_int {
    if size
        < (core::mem::size_of::<uint32_t>() as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<uint8_t>() as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    let mut bytes: size_t = *(zl as *mut uint32_t) as size_t;
    if bytes != size {
        return 0 as libc::c_int;
    }
    if *zl
        .offset(
            size.wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong)
                as isize,
        ) as libc::c_int != 255 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if *(zl.offset(core::mem::size_of::<uint32_t>() as libc::c_ulong as isize)
        as *mut uint32_t) as libc::c_ulong
        > size.wrapping_sub(core::mem::size_of::<uint8_t>() as libc::c_ulong)
    {
        return 0 as libc::c_int;
    }
    if deep == 0 {
        return 1 as libc::c_int;
    }
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut header_count: libc::c_uint = *(zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut uint16_t) as libc::c_uint;
    let mut p: *mut libc::c_uchar = zl
        .offset(
            (core::mem::size_of::<uint32_t>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(core::mem::size_of::<uint16_t>() as libc::c_ulong)
                as isize,
        );
    let mut prev: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut prev_raw_size: size_t = 0 as libc::c_int as size_t;
    while *p as libc::c_int != 255 as libc::c_int {
        let mut e: zlentry = zlentry {
            prevrawlensize: 0,
            prevrawlen: 0,
            lensize: 0,
            len: 0,
            headersize: 0,
            encoding: 0,
            p: 0 as *mut libc::c_uchar,
        };
        if zipEntrySafe(zl, size, p, &mut e, 1 as libc::c_int) == 0 {
            return 0 as libc::c_int;
        }
        if e.prevrawlen as libc::c_ulong != prev_raw_size {
            return 0 as libc::c_int;
        }
        if entry_cb.is_some()
            && entry_cb.expect("non-null function pointer")(p, header_count, cb_userdata)
                == 0
        {
            return 0 as libc::c_int;
        }
        prev_raw_size = (e.headersize).wrapping_add(e.len) as size_t;
        prev = p;
        p = p.offset((e.headersize).wrapping_add(e.len) as isize);
        count = count.wrapping_add(1);
    }
    if p
        != zl
            .offset(bytes as isize)
            .offset(-(core::mem::size_of::<uint8_t>() as libc::c_ulong as isize))
    {
        return 0 as libc::c_int;
    }
    if !prev.is_null()
        && prev
            != zl
                .offset(
                    *(zl
                        .offset(
                            core::mem::size_of::<uint32_t>() as libc::c_ulong as isize,
                        ) as *mut uint32_t) as isize,
                )
    {
        return 0 as libc::c_int;
    }
    if header_count != 65535 as libc::c_int as libc::c_uint && count != header_count {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistRandomPair(
    mut zl: *mut libc::c_uchar,
    mut total_count: libc::c_ulong,
    mut key: *mut ziplistEntry,
    mut val: *mut ziplistEntry,
) {
    let mut ret: libc::c_int = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (total_count != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"total_count\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            1567 as libc::c_int,
        );
        unreachable!();
    };
    let mut r: libc::c_int = (rand() as libc::c_ulong)
        .wrapping_rem(total_count)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    p = ziplistIndex(zl, r);
    ret = ziplistGet(p, &mut (*key).sval, &mut (*key).slen, &mut (*key).lval)
        as libc::c_int;
    if (ret != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"ret != 0\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            1573 as libc::c_int,
        );
        unreachable!();
    };
    if val.is_null() {
        return;
    }
    p = ziplistNext(zl, p);
    ret = ziplistGet(p, &mut (*val).sval, &mut (*val).slen, &mut (*val).lval)
        as libc::c_int;
    if (ret != 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"ret != 0\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            1579 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn uintCompare(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return (*(a as *mut libc::c_uint)).wrapping_sub(*(b as *mut libc::c_uint))
        as libc::c_int;
}
#[inline]
unsafe extern "C" fn ziplistSaveValue(
    mut val: *mut libc::c_uchar,
    mut len: libc::c_uint,
    mut lval: libc::c_longlong,
    mut dest: *mut ziplistEntry,
) {
    (*dest).sval = val;
    (*dest).slen = len;
    (*dest).lval = lval;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistRandomPairs(
    mut zl: *mut libc::c_uchar,
    mut count: libc::c_uint,
    mut keys: *mut ziplistEntry,
    mut vals: *mut ziplistEntry,
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
    let mut total_size: libc::c_uint = (ziplistLen(zl))
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    if (total_size != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"total_size\0" as *const u8 as *const libc::c_char,
            b"ziplist.c\0" as *const u8 as *const libc::c_char,
            1612 as libc::c_int,
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
    let mut zipindex: libc::c_uint = (*picks.offset(0 as libc::c_int as isize)).index;
    let mut pickindex: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    p = ziplistIndex(zl, zipindex as libc::c_int);
    while ziplistGet(p, &mut key, &mut klen, &mut klval) != 0 && pickindex < count {
        p = ziplistNext(zl, p);
        if (ziplistGet(p, &mut value, &mut vlen, &mut vlval) != 0) as libc::c_int
            as libc::c_long != 0
        {} else {
            _serverAssert(
                b"ziplistGet(p, &value, &vlen, &vlval)\0" as *const u8
                    as *const libc::c_char,
                b"ziplist.c\0" as *const u8 as *const libc::c_char,
                1629 as libc::c_int,
            );
            unreachable!();
        };
        while pickindex < count && zipindex == (*picks.offset(pickindex as isize)).index
        {
            let mut storeorder: libc::c_int = (*picks.offset(pickindex as isize)).order
                as libc::c_int;
            ziplistSaveValue(key, klen, klval, &mut *keys.offset(storeorder as isize));
            if !vals.is_null() {
                ziplistSaveValue(
                    value,
                    vlen,
                    vlval,
                    &mut *vals.offset(storeorder as isize),
                );
            }
            pickindex = pickindex.wrapping_add(1);
        }
        zipindex = zipindex.wrapping_add(2 as libc::c_int as libc::c_uint);
        p = ziplistNext(zl, p);
    }
    zfree(picks as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ziplistRandomPairsUnique(
    mut zl: *mut libc::c_uchar,
    mut count: libc::c_uint,
    mut keys: *mut ziplistEntry,
    mut vals: *mut ziplistEntry,
) -> libc::c_uint {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut key: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut klen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut klval: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut total_size: libc::c_uint = (ziplistLen(zl))
        .wrapping_div(2 as libc::c_int as libc::c_uint);
    let mut index: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if count > total_size {
        count = total_size;
    }
    p = ziplistIndex(zl, 0 as libc::c_int);
    let mut picked: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut remaining: libc::c_uint = count;
    while picked < count && !p.is_null() {
        let mut randomDouble: libc::c_double = rand() as libc::c_double
            / 2147483647 as libc::c_int as libc::c_double;
        let mut threshold: libc::c_double = remaining as libc::c_double
            / total_size.wrapping_sub(index) as libc::c_double;
        if randomDouble <= threshold {
            if (ziplistGet(p, &mut key, &mut klen, &mut klval) != 0) as libc::c_int
                as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"ziplistGet(p, &key, &klen, &klval)\0" as *const u8
                        as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    1669 as libc::c_int,
                );
                unreachable!();
            };
            ziplistSaveValue(key, klen, klval, &mut *keys.offset(picked as isize));
            p = ziplistNext(zl, p);
            if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
                _serverAssert(
                    b"p\0" as *const u8 as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    1672 as libc::c_int,
                );
                unreachable!();
            };
            if !vals.is_null() {
                if (ziplistGet(p, &mut key, &mut klen, &mut klval) != 0) as libc::c_int
                    as libc::c_long != 0
                {} else {
                    _serverAssert(
                        b"ziplistGet(p, &key, &klen, &klval)\0" as *const u8
                            as *const libc::c_char,
                        b"ziplist.c\0" as *const u8 as *const libc::c_char,
                        1674 as libc::c_int,
                    );
                    unreachable!();
                };
                ziplistSaveValue(key, klen, klval, &mut *vals.offset(picked as isize));
            }
            remaining = remaining.wrapping_sub(1);
            picked = picked.wrapping_add(1);
        } else {
            p = ziplistNext(zl, p);
            if !p.is_null() as libc::c_int as libc::c_long != 0 {} else {
                _serverAssert(
                    b"p\0" as *const u8 as *const libc::c_char,
                    b"ziplist.c\0" as *const u8 as *const libc::c_char,
                    1681 as libc::c_int,
                );
                unreachable!();
            };
        }
        p = ziplistNext(zl, p);
        index = index.wrapping_add(1);
    }
    return picked;
}

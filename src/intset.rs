extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn rand() -> libc::c_int;
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
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
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
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intset {
    pub encoding: uint32_t,
    pub length: uint32_t,
    pub contents: [int8_t; 0],
}
unsafe extern "C" fn _intsetValueEncoding(mut v: int64_t) -> uint8_t {
    if v < (-(2147483647 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        || v > 2147483647 as libc::c_int as libc::c_long
    {
        return core::mem::size_of::<int64_t>() as libc::c_ulong as uint8_t
    } else if v < (-(32767 as libc::c_int) - 1 as libc::c_int) as libc::c_long
        || v > 32767 as libc::c_int as libc::c_long
    {
        return core::mem::size_of::<int32_t>() as libc::c_ulong as uint8_t
    } else {
        return core::mem::size_of::<int16_t>() as libc::c_ulong as uint8_t
    };
}
unsafe extern "C" fn _intsetGetEncoded(
    mut is: *mut intset,
    mut pos: libc::c_int,
    mut enc: uint8_t,
) -> int64_t {
    let mut v64: int64_t = 0;
    let mut v32: int32_t = 0;
    let mut v16: int16_t = 0;
    if enc as libc::c_ulong == core::mem::size_of::<int64_t>() as libc::c_ulong {
        memcpy(
            &mut v64 as *mut int64_t as *mut libc::c_void,
            (((*is).contents).as_mut_ptr() as *mut int64_t).offset(pos as isize)
                as *const libc::c_void,
            core::mem::size_of::<int64_t>() as libc::c_ulong,
        );
        return v64;
    } else if enc as libc::c_ulong == core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        memcpy(
            &mut v32 as *mut int32_t as *mut libc::c_void,
            (((*is).contents).as_mut_ptr() as *mut int32_t).offset(pos as isize)
                as *const libc::c_void,
            core::mem::size_of::<int32_t>() as libc::c_ulong,
        );
        return v32 as int64_t;
    } else {
        memcpy(
            &mut v16 as *mut int16_t as *mut libc::c_void,
            (((*is).contents).as_mut_ptr() as *mut int16_t).offset(pos as isize)
                as *const libc::c_void,
            core::mem::size_of::<int16_t>() as libc::c_ulong,
        );
        return v16 as int64_t;
    };
}
unsafe extern "C" fn _intsetGet(mut is: *mut intset, mut pos: libc::c_int) -> int64_t {
    return _intsetGetEncoded(is, pos, (*is).encoding as uint8_t);
}
unsafe extern "C" fn _intsetSet(
    mut is: *mut intset,
    mut pos: libc::c_int,
    mut value: int64_t,
) {
    let mut encoding: uint32_t = (*is).encoding;
    if encoding as libc::c_ulong == core::mem::size_of::<int64_t>() as libc::c_ulong {
        *(((*is).contents).as_mut_ptr() as *mut int64_t).offset(pos as isize) = value;
    } else if encoding as libc::c_ulong
        == core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        *(((*is).contents).as_mut_ptr() as *mut int32_t)
            .offset(pos as isize) = value as int32_t;
    } else {
        *(((*is).contents).as_mut_ptr() as *mut int16_t)
            .offset(pos as isize) = value as int16_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn intsetNew() -> *mut intset {
    let mut is: *mut intset = zmalloc(core::mem::size_of::<intset>() as libc::c_ulong)
        as *mut intset;
    (*is).encoding = core::mem::size_of::<int16_t>() as libc::c_ulong as uint32_t;
    (*is).length = 0 as libc::c_int as uint32_t;
    return is;
}
unsafe extern "C" fn intsetResize(
    mut is: *mut intset,
    mut len: uint32_t,
) -> *mut intset {
    let mut size: uint64_t = (len as uint64_t)
        .wrapping_mul((*is).encoding as libc::c_ulong);
    if (size
        <= (18446744073709551615 as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<intset>() as libc::c_ulong))
        as libc::c_int as libc::c_long != 0
    {} else {
        _serverAssert(
            b"size <= SIZE_MAX - sizeof(intset)\0" as *const u8 as *const libc::c_char,
            b"intset.c\0" as *const u8 as *const libc::c_char,
            108 as libc::c_int,
        );
        unreachable!();
    };
    is = zrealloc(
        is as *mut libc::c_void,
        (core::mem::size_of::<intset>() as libc::c_ulong).wrapping_add(size),
    ) as *mut intset;
    return is;
}
unsafe extern "C" fn intsetSearch(
    mut is: *mut intset,
    mut value: int64_t,
    mut pos: *mut uint32_t,
) -> uint8_t {
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut max: libc::c_int = ((*is).length)
        .wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int;
    let mut mid: libc::c_int = -(1 as libc::c_int);
    let mut cur: int64_t = -(1 as libc::c_int) as int64_t;
    if (*is).length == 0 as libc::c_int as libc::c_uint {
        if !pos.is_null() {
            *pos = 0 as libc::c_int as uint32_t;
        }
        return 0 as libc::c_int as uint8_t;
    } else {
        if value > _intsetGet(is, max) {
            if !pos.is_null() {
                *pos = (*is).length;
            }
            return 0 as libc::c_int as uint8_t;
        } else {
            if value < _intsetGet(is, 0 as libc::c_int) {
                if !pos.is_null() {
                    *pos = 0 as libc::c_int as uint32_t;
                }
                return 0 as libc::c_int as uint8_t;
            }
        }
    }
    while max >= min {
        mid = ((min as libc::c_uint).wrapping_add(max as libc::c_uint)
            >> 1 as libc::c_int) as libc::c_int;
        cur = _intsetGet(is, mid);
        if value > cur {
            min = mid + 1 as libc::c_int;
        } else {
            if !(value < cur) {
                break;
            }
            max = mid - 1 as libc::c_int;
        }
    }
    if value == cur {
        if !pos.is_null() {
            *pos = mid as uint32_t;
        }
        return 1 as libc::c_int as uint8_t;
    } else {
        if !pos.is_null() {
            *pos = min as uint32_t;
        }
        return 0 as libc::c_int as uint8_t;
    };
}
unsafe extern "C" fn intsetUpgradeAndAdd(
    mut is: *mut intset,
    mut value: int64_t,
) -> *mut intset {
    let mut curenc: uint8_t = (*is).encoding as uint8_t;
    let mut newenc: uint8_t = _intsetValueEncoding(value);
    let mut length: libc::c_int = (*is).length as libc::c_int;
    let mut prepend: libc::c_int = if value < 0 as libc::c_int as libc::c_long {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*is).encoding = newenc as uint32_t;
    is = intsetResize(is, ((*is).length).wrapping_add(1 as libc::c_int as libc::c_uint));
    loop {
        let fresh0 = length;
        length = length - 1;
        if !(fresh0 != 0) {
            break;
        }
        _intsetSet(is, length + prepend, _intsetGetEncoded(is, length, curenc));
    }
    if prepend != 0 {
        _intsetSet(is, 0 as libc::c_int, value);
    } else {
        _intsetSet(is, (*is).length as libc::c_int, value);
    }
    (*is).length = ((*is).length).wrapping_add(1 as libc::c_int as libc::c_uint);
    return is;
}
unsafe extern "C" fn intsetMoveTail(
    mut is: *mut intset,
    mut from: uint32_t,
    mut to: uint32_t,
) {
    let mut src: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut dst: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut bytes: uint32_t = ((*is).length).wrapping_sub(from);
    let mut encoding: uint32_t = (*is).encoding;
    if encoding as libc::c_ulong == core::mem::size_of::<int64_t>() as libc::c_ulong {
        src = (((*is).contents).as_mut_ptr() as *mut int64_t).offset(from as isize)
            as *mut libc::c_void;
        dst = (((*is).contents).as_mut_ptr() as *mut int64_t).offset(to as isize)
            as *mut libc::c_void;
        bytes = (bytes as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<int64_t>() as libc::c_ulong) as uint32_t
            as uint32_t;
    } else if encoding as libc::c_ulong
        == core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        src = (((*is).contents).as_mut_ptr() as *mut int32_t).offset(from as isize)
            as *mut libc::c_void;
        dst = (((*is).contents).as_mut_ptr() as *mut int32_t).offset(to as isize)
            as *mut libc::c_void;
        bytes = (bytes as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<int32_t>() as libc::c_ulong) as uint32_t
            as uint32_t;
    } else {
        src = (((*is).contents).as_mut_ptr() as *mut int16_t).offset(from as isize)
            as *mut libc::c_void;
        dst = (((*is).contents).as_mut_ptr() as *mut int16_t).offset(to as isize)
            as *mut libc::c_void;
        bytes = (bytes as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<int16_t>() as libc::c_ulong) as uint32_t
            as uint32_t;
    }
    memmove(dst, src, bytes as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn intsetAdd(
    mut is: *mut intset,
    mut value: int64_t,
    mut success: *mut uint8_t,
) -> *mut intset {
    let mut valenc: uint8_t = _intsetValueEncoding(value);
    let mut pos: uint32_t = 0;
    if !success.is_null() {
        *success = 1 as libc::c_int as uint8_t;
    }
    if valenc as libc::c_uint > (*is).encoding {
        return intsetUpgradeAndAdd(is, value)
    } else {
        if intsetSearch(is, value, &mut pos) != 0 {
            if !success.is_null() {
                *success = 0 as libc::c_int as uint8_t;
            }
            return is;
        }
        is = intsetResize(
            is,
            ((*is).length).wrapping_add(1 as libc::c_int as libc::c_uint),
        );
        if pos < (*is).length {
            intsetMoveTail(is, pos, pos.wrapping_add(1 as libc::c_int as libc::c_uint));
        }
    }
    _intsetSet(is, pos as libc::c_int, value);
    (*is).length = ((*is).length).wrapping_add(1 as libc::c_int as libc::c_uint);
    return is;
}
#[no_mangle]
pub unsafe extern "C" fn intsetRemove(
    mut is: *mut intset,
    mut value: int64_t,
    mut success: *mut libc::c_int,
) -> *mut intset {
    let mut valenc: uint8_t = _intsetValueEncoding(value);
    let mut pos: uint32_t = 0;
    if !success.is_null() {
        *success = 0 as libc::c_int;
    }
    if valenc as libc::c_uint <= (*is).encoding
        && intsetSearch(is, value, &mut pos) as libc::c_int != 0
    {
        let mut len: uint32_t = (*is).length;
        if !success.is_null() {
            *success = 1 as libc::c_int;
        }
        if pos < len.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            intsetMoveTail(is, pos.wrapping_add(1 as libc::c_int as libc::c_uint), pos);
        }
        is = intsetResize(is, len.wrapping_sub(1 as libc::c_int as libc::c_uint));
        (*is).length = len.wrapping_sub(1 as libc::c_int as libc::c_uint);
    }
    return is;
}
#[no_mangle]
pub unsafe extern "C" fn intsetFind(mut is: *mut intset, mut value: int64_t) -> uint8_t {
    let mut valenc: uint8_t = _intsetValueEncoding(value);
    return (valenc as libc::c_uint <= (*is).encoding
        && intsetSearch(is, value, 0 as *mut uint32_t) as libc::c_int != 0)
        as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn intsetRandom(mut is: *mut intset) -> int64_t {
    let mut len: uint32_t = (*is).length;
    if (len != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"len\0" as *const u8 as *const libc::c_char,
            b"intset.c\0" as *const u8 as *const libc::c_char,
            264 as libc::c_int,
        );
        unreachable!();
    };
    return _intsetGet(is, (rand() as libc::c_uint).wrapping_rem(len) as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn intsetGet(
    mut is: *mut intset,
    mut pos: uint32_t,
    mut value: *mut int64_t,
) -> uint8_t {
    if pos < (*is).length {
        *value = _intsetGet(is, pos as libc::c_int);
        return 1 as libc::c_int as uint8_t;
    }
    return 0 as libc::c_int as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn intsetLen(mut is: *const intset) -> uint32_t {
    return (*is).length;
}
#[no_mangle]
pub unsafe extern "C" fn intsetBlobLen(mut is: *mut intset) -> size_t {
    return (core::mem::size_of::<intset>() as libc::c_ulong)
        .wrapping_add(
            ((*is).length as size_t).wrapping_mul((*is).encoding as libc::c_ulong),
        );
}
#[no_mangle]
pub unsafe extern "C" fn intsetValidateIntegrity(
    mut p: *const libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
) -> libc::c_int {
    let mut is: *mut intset = p as *mut intset;
    if size < core::mem::size_of::<intset>() as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut encoding: uint32_t = (*is).encoding;
    let mut record_size: size_t = 0;
    if encoding as libc::c_ulong == core::mem::size_of::<int64_t>() as libc::c_ulong {
        record_size = core::mem::size_of::<int64_t>() as libc::c_ulong;
    } else if encoding as libc::c_ulong
        == core::mem::size_of::<int32_t>() as libc::c_ulong
    {
        record_size = core::mem::size_of::<int32_t>() as libc::c_ulong;
    } else if encoding as libc::c_ulong
        == core::mem::size_of::<int16_t>() as libc::c_ulong
    {
        record_size = core::mem::size_of::<int16_t>() as libc::c_ulong;
    } else {
        return 0 as libc::c_int
    }
    let mut count: uint32_t = (*is).length;
    if (core::mem::size_of::<intset>() as libc::c_ulong)
        .wrapping_add((count as libc::c_ulong).wrapping_mul(record_size)) != size
    {
        return 0 as libc::c_int;
    }
    if count == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if deep == 0 {
        return 1 as libc::c_int;
    }
    let mut prev: int64_t = _intsetGet(is, 0 as libc::c_int);
    let mut i: uint32_t = 1 as libc::c_int as uint32_t;
    while i < count {
        let mut cur: int64_t = _intsetGet(is, i as libc::c_int);
        if cur <= prev {
            return 0 as libc::c_int;
        }
        prev = cur;
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
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
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn zipmapNew() -> *mut libc::c_uchar {
    let mut zm: *mut libc::c_uchar = zmalloc(2 as libc::c_int as size_t)
        as *mut libc::c_uchar;
    *zm.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_uchar;
    *zm.offset(1 as libc::c_int as isize) = 255 as libc::c_int as libc::c_uchar;
    return zm;
}
unsafe extern "C" fn zipmapDecodeLength(mut p: *mut libc::c_uchar) -> libc::c_uint {
    let mut len: libc::c_uint = *p as libc::c_uint;
    if len < 254 as libc::c_int as libc::c_uint {
        return len;
    }
    memcpy(
        &mut len as *mut libc::c_uint as *mut libc::c_void,
        p.offset(1 as libc::c_int as isize) as *const libc::c_void,
        core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
    );
    return len;
}
unsafe extern "C" fn zipmapGetEncodedLengthSize(
    mut p: *mut libc::c_uchar,
) -> libc::c_uint {
    return (if (*p as libc::c_int) < 254 as libc::c_int {
        1 as libc::c_int
    } else {
        5 as libc::c_int
    }) as libc::c_uint;
}
unsafe extern "C" fn zipmapEncodeLength(
    mut p: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_uint {
    if p.is_null() {
        return (if len < 254 as libc::c_int as libc::c_uint {
            1 as libc::c_int as libc::c_ulong
        } else {
            (core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        }) as libc::c_uint
    } else if len < 254 as libc::c_int as libc::c_uint {
        *p.offset(0 as libc::c_int as isize) = len as libc::c_uchar;
        return 1 as libc::c_int as libc::c_uint;
    } else {
        *p.offset(0 as libc::c_int as isize) = 254 as libc::c_int as libc::c_uchar;
        memcpy(
            p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            &mut len as *mut libc::c_uint as *const libc::c_void,
            core::mem::size_of::<libc::c_uint>() as libc::c_ulong,
        );
        return (1 as libc::c_int as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
            as libc::c_uint;
    };
}
unsafe extern "C" fn zipmapLookupRaw(
    mut zm: *mut libc::c_uchar,
    mut key: *mut libc::c_uchar,
    mut klen: libc::c_uint,
    mut totlen: *mut libc::c_uint,
) -> *mut libc::c_uchar {
    let mut p: *mut libc::c_uchar = zm.offset(1 as libc::c_int as isize);
    let mut k: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut l: libc::c_uint = 0;
    let mut llen: libc::c_uint = 0;
    while *p as libc::c_int != 255 as libc::c_int {
        let mut free: libc::c_uchar = 0;
        l = zipmapDecodeLength(p);
        llen = zipmapEncodeLength(0 as *mut libc::c_uchar, l);
        if !key.is_null() && k.is_null() && l == klen
            && memcmp(
                p.offset(llen as isize) as *const libc::c_void,
                key as *const libc::c_void,
                l as libc::c_ulong,
            ) == 0
        {
            if !totlen.is_null() {
                k = p;
            } else {
                return p
            }
        }
        p = p.offset(llen.wrapping_add(l) as isize);
        l = zipmapDecodeLength(p);
        p = p.offset(zipmapEncodeLength(0 as *mut libc::c_uchar, l) as isize);
        free = *p.offset(0 as libc::c_int as isize);
        p = p
            .offset(
                l
                    .wrapping_add(1 as libc::c_int as libc::c_uint)
                    .wrapping_add(free as libc::c_uint) as isize,
            );
    }
    if !totlen.is_null() {
        *totlen = (p.offset_from(zm) as libc::c_long as libc::c_uint)
            .wrapping_add(1 as libc::c_int as libc::c_uint);
    }
    return k;
}
unsafe extern "C" fn zipmapRequiredLength(
    mut klen: libc::c_uint,
    mut vlen: libc::c_uint,
) -> libc::c_ulong {
    let mut l: libc::c_uint = 0;
    l = klen.wrapping_add(vlen).wrapping_add(3 as libc::c_int as libc::c_uint);
    if klen >= 254 as libc::c_int as libc::c_uint {
        l = l.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    if vlen >= 254 as libc::c_int as libc::c_uint {
        l = l.wrapping_add(4 as libc::c_int as libc::c_uint);
    }
    return l as libc::c_ulong;
}
unsafe extern "C" fn zipmapRawKeyLength(mut p: *mut libc::c_uchar) -> libc::c_uint {
    let mut l: libc::c_uint = zipmapDecodeLength(p);
    return (zipmapEncodeLength(0 as *mut libc::c_uchar, l)).wrapping_add(l);
}
unsafe extern "C" fn zipmapRawValueLength(mut p: *mut libc::c_uchar) -> libc::c_uint {
    let mut l: libc::c_uint = zipmapDecodeLength(p);
    let mut used: libc::c_uint = 0;
    used = zipmapEncodeLength(0 as *mut libc::c_uchar, l);
    used = used
        .wrapping_add(
            ((*p.offset(used as isize) as libc::c_int + 1 as libc::c_int)
                as libc::c_uint)
                .wrapping_add(l),
        );
    return used;
}
unsafe extern "C" fn zipmapRawEntryLength(mut p: *mut libc::c_uchar) -> libc::c_uint {
    let mut l: libc::c_uint = zipmapRawKeyLength(p);
    return l.wrapping_add(zipmapRawValueLength(p.offset(l as isize)));
}
#[inline]
unsafe extern "C" fn zipmapResize(
    mut zm: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> *mut libc::c_uchar {
    zm = zrealloc(zm as *mut libc::c_void, len as size_t) as *mut libc::c_uchar;
    *zm
        .offset(
            len.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
        ) = 255 as libc::c_int as libc::c_uchar;
    return zm;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapSet(
    mut zm: *mut libc::c_uchar,
    mut key: *mut libc::c_uchar,
    mut klen: libc::c_uint,
    mut val: *mut libc::c_uchar,
    mut vlen: libc::c_uint,
    mut update: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut zmlen: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut freelen: libc::c_uint = 0;
    let mut reqlen: libc::c_uint = zipmapRequiredLength(klen, vlen) as libc::c_uint;
    let mut empty: libc::c_uint = 0;
    let mut vempty: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    freelen = reqlen;
    if !update.is_null() {
        *update = 0 as libc::c_int;
    }
    p = zipmapLookupRaw(zm, key, klen, &mut zmlen);
    if p.is_null() {
        zm = zipmapResize(zm, zmlen.wrapping_add(reqlen));
        p = zm.offset(zmlen as isize).offset(-(1 as libc::c_int as isize));
        zmlen = zmlen.wrapping_add(reqlen);
        if (*zm.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
            let ref mut fresh0 = *zm.offset(0 as libc::c_int as isize);
            *fresh0 = (*fresh0).wrapping_add(1);
        }
    } else {
        if !update.is_null() {
            *update = 1 as libc::c_int;
        }
        freelen = zipmapRawEntryLength(p);
        if freelen < reqlen {
            offset = p.offset_from(zm) as libc::c_long as libc::c_uint;
            zm = zipmapResize(zm, zmlen.wrapping_sub(freelen).wrapping_add(reqlen));
            p = zm.offset(offset as isize);
            memmove(
                p.offset(reqlen as isize) as *mut libc::c_void,
                p.offset(freelen as isize) as *const libc::c_void,
                zmlen
                    .wrapping_sub(
                        offset
                            .wrapping_add(freelen)
                            .wrapping_add(1 as libc::c_int as libc::c_uint),
                    ) as libc::c_ulong,
            );
            zmlen = zmlen.wrapping_sub(freelen).wrapping_add(reqlen);
            freelen = reqlen;
        }
    }
    empty = freelen.wrapping_sub(reqlen);
    if empty >= 4 as libc::c_int as libc::c_uint {
        offset = p.offset_from(zm) as libc::c_long as libc::c_uint;
        memmove(
            p.offset(reqlen as isize) as *mut libc::c_void,
            p.offset(freelen as isize) as *const libc::c_void,
            zmlen
                .wrapping_sub(
                    offset
                        .wrapping_add(freelen)
                        .wrapping_add(1 as libc::c_int as libc::c_uint),
                ) as libc::c_ulong,
        );
        zmlen = zmlen.wrapping_sub(empty);
        zm = zipmapResize(zm, zmlen);
        p = zm.offset(offset as isize);
        vempty = 0 as libc::c_int as libc::c_uint;
    } else {
        vempty = empty;
    }
    p = p.offset(zipmapEncodeLength(p, klen) as isize);
    memcpy(p as *mut libc::c_void, key as *const libc::c_void, klen as libc::c_ulong);
    p = p.offset(klen as isize);
    p = p.offset(zipmapEncodeLength(p, vlen) as isize);
    let fresh1 = p;
    p = p.offset(1);
    *fresh1 = vempty as libc::c_uchar;
    memcpy(p as *mut libc::c_void, val as *const libc::c_void, vlen as libc::c_ulong);
    return zm;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapDel(
    mut zm: *mut libc::c_uchar,
    mut key: *mut libc::c_uchar,
    mut klen: libc::c_uint,
    mut deleted: *mut libc::c_int,
) -> *mut libc::c_uchar {
    let mut zmlen: libc::c_uint = 0;
    let mut freelen: libc::c_uint = 0;
    let mut p: *mut libc::c_uchar = zipmapLookupRaw(zm, key, klen, &mut zmlen);
    if !p.is_null() {
        freelen = zipmapRawEntryLength(p);
        memmove(
            p as *mut libc::c_void,
            p.offset(freelen as isize) as *const libc::c_void,
            (zmlen as libc::c_long
                - (p.offset_from(zm) as libc::c_long + freelen as libc::c_long
                    + 1 as libc::c_int as libc::c_long)) as libc::c_ulong,
        );
        zm = zipmapResize(zm, zmlen.wrapping_sub(freelen));
        if (*zm.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
            let ref mut fresh2 = *zm.offset(0 as libc::c_int as isize);
            *fresh2 = (*fresh2).wrapping_sub(1);
        }
        if !deleted.is_null() {
            *deleted = 1 as libc::c_int;
        }
    } else if !deleted.is_null() {
        *deleted = 0 as libc::c_int;
    }
    return zm;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapRewind(mut zm: *mut libc::c_uchar) -> *mut libc::c_uchar {
    return zm.offset(1 as libc::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn zipmapNext(
    mut zm: *mut libc::c_uchar,
    mut key: *mut *mut libc::c_uchar,
    mut klen: *mut libc::c_uint,
    mut value: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
) -> *mut libc::c_uchar {
    if *zm.offset(0 as libc::c_int as isize) as libc::c_int == 255 as libc::c_int {
        return 0 as *mut libc::c_uchar;
    }
    if !key.is_null() {
        *key = zm;
        *klen = zipmapDecodeLength(zm);
        *key = (*key)
            .offset(
                (if *klen < 254 as libc::c_int as libc::c_uint {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    (core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                }) as isize,
            );
    }
    zm = zm.offset(zipmapRawKeyLength(zm) as isize);
    if !value.is_null() {
        *value = zm.offset(1 as libc::c_int as isize);
        *vlen = zipmapDecodeLength(zm);
        *value = (*value)
            .offset(
                (if *vlen < 254 as libc::c_int as libc::c_uint {
                    1 as libc::c_int as libc::c_ulong
                } else {
                    (core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                }) as isize,
            );
    }
    zm = zm.offset(zipmapRawValueLength(zm) as isize);
    return zm;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapGet(
    mut zm: *mut libc::c_uchar,
    mut key: *mut libc::c_uchar,
    mut klen: libc::c_uint,
    mut value: *mut *mut libc::c_uchar,
    mut vlen: *mut libc::c_uint,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    p = zipmapLookupRaw(zm, key, klen, 0 as *mut libc::c_uint);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    p = p.offset(zipmapRawKeyLength(p) as isize);
    *vlen = zipmapDecodeLength(p);
    *value = p
        .offset(
            (if *vlen < 254 as libc::c_int as libc::c_uint {
                1 as libc::c_int as libc::c_ulong
            } else {
                (core::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
            }) as isize,
        )
        .offset(1 as libc::c_int as isize);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapExists(
    mut zm: *mut libc::c_uchar,
    mut key: *mut libc::c_uchar,
    mut klen: libc::c_uint,
) -> libc::c_int {
    return (zipmapLookupRaw(zm, key, klen, 0 as *mut libc::c_uint)
        != 0 as *mut libc::c_void as *mut libc::c_uchar) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapLen(mut zm: *mut libc::c_uchar) -> libc::c_uint {
    let mut len: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*zm.offset(0 as libc::c_int as isize) as libc::c_int) < 254 as libc::c_int {
        len = *zm.offset(0 as libc::c_int as isize) as libc::c_uint;
    } else {
        let mut p: *mut libc::c_uchar = zipmapRewind(zm);
        loop {
            p = zipmapNext(
                p,
                0 as *mut *mut libc::c_uchar,
                0 as *mut libc::c_uint,
                0 as *mut *mut libc::c_uchar,
                0 as *mut libc::c_uint,
            );
            if p.is_null() {
                break;
            }
            len = len.wrapping_add(1);
        }
        if len < 254 as libc::c_int as libc::c_uint {
            *zm.offset(0 as libc::c_int as isize) = len as libc::c_uchar;
        }
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapBlobLen(mut zm: *mut libc::c_uchar) -> size_t {
    let mut totlen: libc::c_uint = 0;
    zipmapLookupRaw(
        zm,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as libc::c_uint,
        &mut totlen,
    );
    return totlen as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zipmapValidateIntegrity(
    mut zm: *mut libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
) -> libc::c_int {
    let mut l: libc::c_uint = 0;
    let mut s: libc::c_uint = 0;
    let mut e: libc::c_uint = 0;
    if size < 2 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if *zm.offset(size.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int != 255 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if deep == 0 {
        return 1 as libc::c_int;
    }
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut p: *mut libc::c_uchar = zm.offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 255 as libc::c_int {
        s = zipmapGetEncodedLengthSize(p);
        if p.offset(s as isize) < zm.offset(2 as libc::c_int as isize)
            || p.offset(s as isize)
                > zm.offset(size as isize).offset(-(1 as libc::c_int as isize))
        {
            return 0 as libc::c_int;
        }
        l = zipmapDecodeLength(p);
        p = p.offset(s as isize);
        p = p.offset(l as isize);
        if p < zm.offset(2 as libc::c_int as isize)
            || p > zm.offset(size as isize).offset(-(1 as libc::c_int as isize))
        {
            return 0 as libc::c_int;
        }
        s = zipmapGetEncodedLengthSize(p);
        if p.offset(s as isize) < zm.offset(2 as libc::c_int as isize)
            || p.offset(s as isize)
                > zm.offset(size as isize).offset(-(1 as libc::c_int as isize))
        {
            return 0 as libc::c_int;
        }
        l = zipmapDecodeLength(p);
        p = p.offset(s as isize);
        let fresh3 = p;
        p = p.offset(1);
        e = *fresh3 as libc::c_uint;
        p = p.offset(l.wrapping_add(e) as isize);
        count = count.wrapping_add(1);
        if p < zm.offset(2 as libc::c_int as isize)
            || p > zm.offset(size as isize).offset(-(1 as libc::c_int as isize))
        {
            return 0 as libc::c_int;
        }
    }
    if count == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if *zm.offset(0 as libc::c_int as isize) as libc::c_int != 254 as libc::c_int
        && *zm.offset(0 as libc::c_int as isize) as libc::c_uint != count
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

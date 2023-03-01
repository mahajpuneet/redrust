extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn siptlw(mut c: libc::c_int) -> libc::c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return c + ('a' as i32 - 'A' as i32)
    } else {
        return c
    };
}
#[no_mangle]
pub unsafe extern "C" fn siphash(
    mut in_0: *const uint8_t,
    inlen: size_t,
    mut k: *const uint8_t,
) -> uint64_t {
    let mut v0: uint64_t = 0x736f6d6570736575 as libc::c_ulonglong as uint64_t;
    let mut v1: uint64_t = 0x646f72616e646f6d as libc::c_ulonglong as uint64_t;
    let mut v2: uint64_t = 0x6c7967656e657261 as libc::c_ulonglong as uint64_t;
    let mut v3: uint64_t = 0x7465646279746573 as libc::c_ulonglong as uint64_t;
    let mut k0: uint64_t = *(k as *mut uint64_t);
    let mut k1: uint64_t = *(k.offset(8 as libc::c_int as isize) as *mut uint64_t);
    let mut m: uint64_t = 0;
    let mut end: *const uint8_t = in_0
        .offset(inlen as isize)
        .offset(
            -(inlen.wrapping_rem(core::mem::size_of::<uint64_t>() as libc::c_ulong)
                as isize),
        );
    let left: libc::c_int = (inlen & 7 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut b: uint64_t = inlen << 56 as libc::c_int;
    v3 ^= k1;
    v2 ^= k0;
    v1 ^= k1;
    v0 ^= k0;
    while in_0 != end {
        m = *(in_0 as *mut uint64_t);
        v3 ^= m;
        v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
        v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
        v1 ^= v0;
        v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
        v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
        v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
        v3 ^= v2;
        v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
        v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
        v3 ^= v0;
        v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
        v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
        v1 ^= v2;
        v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
        v0 ^= m;
        in_0 = in_0.offset(8 as libc::c_int as isize);
    }
    let mut current_block_31: u64;
    match left {
        7 => {
            b
                |= (*in_0.offset(6 as libc::c_int as isize) as uint64_t)
                    << 48 as libc::c_int;
            current_block_31 = 1249139192423293139;
        }
        6 => {
            current_block_31 = 1249139192423293139;
        }
        5 => {
            current_block_31 = 17953379472903764582;
        }
        4 => {
            current_block_31 = 13450376713251723712;
        }
        3 => {
            current_block_31 = 17412530589607255837;
        }
        2 => {
            current_block_31 = 3653295323910715944;
        }
        1 => {
            current_block_31 = 15640639866236238398;
        }
        0 | _ => {
            current_block_31 = 4488286894823169796;
        }
    }
    match current_block_31 {
        1249139192423293139 => {
            b
                |= (*in_0.offset(5 as libc::c_int as isize) as uint64_t)
                    << 40 as libc::c_int;
            current_block_31 = 17953379472903764582;
        }
        _ => {}
    }
    match current_block_31 {
        17953379472903764582 => {
            b
                |= (*in_0.offset(4 as libc::c_int as isize) as uint64_t)
                    << 32 as libc::c_int;
            current_block_31 = 13450376713251723712;
        }
        _ => {}
    }
    match current_block_31 {
        13450376713251723712 => {
            b
                |= (*in_0.offset(3 as libc::c_int as isize) as uint64_t)
                    << 24 as libc::c_int;
            current_block_31 = 17412530589607255837;
        }
        _ => {}
    }
    match current_block_31 {
        17412530589607255837 => {
            b
                |= (*in_0.offset(2 as libc::c_int as isize) as uint64_t)
                    << 16 as libc::c_int;
            current_block_31 = 3653295323910715944;
        }
        _ => {}
    }
    match current_block_31 {
        3653295323910715944 => {
            b
                |= (*in_0.offset(1 as libc::c_int as isize) as uint64_t)
                    << 8 as libc::c_int;
            current_block_31 = 15640639866236238398;
        }
        _ => {}
    }
    match current_block_31 {
        15640639866236238398 => {
            b |= *in_0.offset(0 as libc::c_int as isize) as uint64_t;
        }
        _ => {}
    }
    v3 ^= b;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 ^= b;
    v2 ^= 0xff as libc::c_int as libc::c_ulong;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    b = v0 ^ v1 ^ v2 ^ v3;
    return b;
}
#[no_mangle]
pub unsafe extern "C" fn siphash_nocase(
    mut in_0: *const uint8_t,
    inlen: size_t,
    mut k: *const uint8_t,
) -> uint64_t {
    let mut v0: uint64_t = 0x736f6d6570736575 as libc::c_ulonglong as uint64_t;
    let mut v1: uint64_t = 0x646f72616e646f6d as libc::c_ulonglong as uint64_t;
    let mut v2: uint64_t = 0x6c7967656e657261 as libc::c_ulonglong as uint64_t;
    let mut v3: uint64_t = 0x7465646279746573 as libc::c_ulonglong as uint64_t;
    let mut k0: uint64_t = *(k as *mut uint64_t);
    let mut k1: uint64_t = *(k.offset(8 as libc::c_int as isize) as *mut uint64_t);
    let mut m: uint64_t = 0;
    let mut end: *const uint8_t = in_0
        .offset(inlen as isize)
        .offset(
            -(inlen.wrapping_rem(core::mem::size_of::<uint64_t>() as libc::c_ulong)
                as isize),
        );
    let left: libc::c_int = (inlen & 7 as libc::c_int as libc::c_ulong) as libc::c_int;
    let mut b: uint64_t = inlen << 56 as libc::c_int;
    v3 ^= k1;
    v2 ^= k0;
    v1 ^= k1;
    v0 ^= k0;
    while in_0 != end {
        m = siptlw(*in_0.offset(0 as libc::c_int as isize) as libc::c_int) as uint64_t
            | (siptlw(*in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 8 as libc::c_int
            | (siptlw(*in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 16 as libc::c_int
            | (siptlw(*in_0.offset(3 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 24 as libc::c_int
            | (siptlw(*in_0.offset(4 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 32 as libc::c_int
            | (siptlw(*in_0.offset(5 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 40 as libc::c_int
            | (siptlw(*in_0.offset(6 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 48 as libc::c_int
            | (siptlw(*in_0.offset(7 as libc::c_int as isize) as libc::c_int)
                as uint64_t) << 56 as libc::c_int;
        v3 ^= m;
        v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
        v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
        v1 ^= v0;
        v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
        v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
        v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
        v3 ^= v2;
        v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
        v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
        v3 ^= v0;
        v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
        v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
        v1 ^= v2;
        v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
        v0 ^= m;
        in_0 = in_0.offset(8 as libc::c_int as isize);
    }
    let mut current_block_31: u64;
    match left {
        7 => {
            b
                |= (siptlw(*in_0.offset(6 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 48 as libc::c_int;
            current_block_31 = 2776346960145241100;
        }
        6 => {
            current_block_31 = 2776346960145241100;
        }
        5 => {
            current_block_31 = 649228984063957735;
        }
        4 => {
            current_block_31 = 10473994958786212244;
        }
        3 => {
            current_block_31 = 9711627312585919507;
        }
        2 => {
            current_block_31 = 12437083962755276651;
        }
        1 => {
            current_block_31 = 6055168197975140418;
        }
        0 | _ => {
            current_block_31 = 4488286894823169796;
        }
    }
    match current_block_31 {
        2776346960145241100 => {
            b
                |= (siptlw(*in_0.offset(5 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 40 as libc::c_int;
            current_block_31 = 649228984063957735;
        }
        _ => {}
    }
    match current_block_31 {
        649228984063957735 => {
            b
                |= (siptlw(*in_0.offset(4 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 32 as libc::c_int;
            current_block_31 = 10473994958786212244;
        }
        _ => {}
    }
    match current_block_31 {
        10473994958786212244 => {
            b
                |= (siptlw(*in_0.offset(3 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 24 as libc::c_int;
            current_block_31 = 9711627312585919507;
        }
        _ => {}
    }
    match current_block_31 {
        9711627312585919507 => {
            b
                |= (siptlw(*in_0.offset(2 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 16 as libc::c_int;
            current_block_31 = 12437083962755276651;
        }
        _ => {}
    }
    match current_block_31 {
        12437083962755276651 => {
            b
                |= (siptlw(*in_0.offset(1 as libc::c_int as isize) as libc::c_int)
                    as uint64_t) << 8 as libc::c_int;
            current_block_31 = 6055168197975140418;
        }
        _ => {}
    }
    match current_block_31 {
        6055168197975140418 => {
            b
                |= siptlw(*in_0.offset(0 as libc::c_int as isize) as libc::c_int)
                    as uint64_t;
        }
        _ => {}
    }
    v3 ^= b;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 ^= b;
    v2 ^= 0xff as libc::c_int as libc::c_ulong;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    v0 = (v0 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 13 as libc::c_int | v1 >> 64 as libc::c_int - 13 as libc::c_int;
    v1 ^= v0;
    v0 = v0 << 32 as libc::c_int | v0 >> 64 as libc::c_int - 32 as libc::c_int;
    v2 = (v2 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 16 as libc::c_int | v3 >> 64 as libc::c_int - 16 as libc::c_int;
    v3 ^= v2;
    v0 = (v0 as libc::c_ulong).wrapping_add(v3) as uint64_t as uint64_t;
    v3 = v3 << 21 as libc::c_int | v3 >> 64 as libc::c_int - 21 as libc::c_int;
    v3 ^= v0;
    v2 = (v2 as libc::c_ulong).wrapping_add(v1) as uint64_t as uint64_t;
    v1 = v1 << 17 as libc::c_int | v1 >> 64 as libc::c_int - 17 as libc::c_int;
    v1 ^= v2;
    v2 = v2 << 32 as libc::c_int | v2 >> 64 as libc::c_int - 32 as libc::c_int;
    b = v0 ^ v1 ^ v2 ^ v3;
    return b;
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __uint16_t = libc::c_ushort;
pub type __uint64_t = libc::c_ulong;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type crcfn64 = Option::<
    unsafe extern "C" fn(uint64_t, *const libc::c_void, uint64_t) -> uint64_t,
>;
pub type crcfn16 = Option::<
    unsafe extern "C" fn(uint16_t, *const libc::c_void, uint64_t) -> uint16_t,
>;
#[no_mangle]
pub unsafe extern "C" fn crcspeed64little_init(
    mut crcfn: crcfn64,
    mut table: *mut [uint64_t; 256],
) {
    let mut crc: uint64_t = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 256 as libc::c_int {
        let mut v: libc::c_uchar = n as libc::c_uchar;
        (*table
            .offset(
                0 as libc::c_int as isize,
            ))[n
            as usize] = crcfn
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int as uint64_t,
            &mut v as *mut libc::c_uchar as *const libc::c_void,
            1 as libc::c_int as uint64_t,
        );
        n += 1;
    }
    let mut n_0: libc::c_int = 0 as libc::c_int;
    while n_0 < 256 as libc::c_int {
        crc = (*table.offset(0 as libc::c_int as isize))[n_0 as usize];
        let mut k: libc::c_int = 1 as libc::c_int;
        while k < 8 as libc::c_int {
            crc = (*table
                .offset(
                    0 as libc::c_int as isize,
                ))[(crc & 0xff as libc::c_int as libc::c_ulong) as usize]
                ^ crc >> 8 as libc::c_int;
            (*table.offset(k as isize))[n_0 as usize] = crc;
            k += 1;
        }
        n_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16little_init(
    mut crcfn: crcfn16,
    mut table: *mut [uint16_t; 256],
) {
    let mut crc: uint16_t = 0;
    let mut n: libc::c_int = 0 as libc::c_int;
    while n < 256 as libc::c_int {
        (*table
            .offset(
                0 as libc::c_int as isize,
            ))[n
            as usize] = crcfn
            .expect(
                "non-null function pointer",
            )(
            0 as libc::c_int as uint16_t,
            &mut n as *mut libc::c_int as *const libc::c_void,
            1 as libc::c_int as uint64_t,
        );
        n += 1;
    }
    let mut n_0: libc::c_int = 0 as libc::c_int;
    while n_0 < 256 as libc::c_int {
        crc = (*table.offset(0 as libc::c_int as isize))[n_0 as usize];
        let mut k: libc::c_int = 1 as libc::c_int;
        while k < 8 as libc::c_int {
            crc = ((*table
                .offset(
                    0 as libc::c_int as isize,
                ))[(crc as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
                as usize] as libc::c_int ^ (crc as libc::c_int) << 8 as libc::c_int)
                as uint16_t;
            (*table.offset(k as isize))[n_0 as usize] = crc;
            k += 1;
        }
        n_0 += 1;
    }
}
#[inline]
unsafe extern "C" fn rev8(mut a: uint64_t) -> uint64_t {
    return a.swap_bytes();
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed64big_init(
    mut fn_0: crcfn64,
    mut big_table: *mut [uint64_t; 256],
) {
    crcspeed64little_init(fn_0, big_table);
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < 8 as libc::c_int {
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < 256 as libc::c_int {
            (*big_table
                .offset(
                    k as isize,
                ))[n as usize] = rev8((*big_table.offset(k as isize))[n as usize]);
            n += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16big_init(
    mut fn_0: crcfn16,
    mut big_table: *mut [uint16_t; 256],
) {
    crcspeed16little_init(fn_0, big_table);
    let mut k: libc::c_int = 0 as libc::c_int;
    while k < 8 as libc::c_int {
        let mut n: libc::c_int = 0 as libc::c_int;
        while n < 256 as libc::c_int {
            (*big_table
                .offset(
                    k as isize,
                ))[n
                as usize] = rev8((*big_table.offset(k as isize))[n as usize] as uint64_t)
                as uint16_t;
            n += 1;
        }
        k += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed64little(
    mut little_table: *mut [uint64_t; 256],
    mut crc: uint64_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint64_t {
    let mut next: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    while len != 0
        && next as uintptr_t & 7 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        let fresh0 = next;
        next = next.offset(1);
        crc = (*little_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc ^ *fresh0 as libc::c_ulong) & 0xff as libc::c_int as libc::c_ulong)
            as usize] ^ crc >> 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    while len >= 8 as libc::c_int as libc::c_ulong {
        crc ^= *(next as *mut uint64_t);
        crc = (*little_table
            .offset(
                7 as libc::c_int as isize,
            ))[(crc & 0xff as libc::c_int as libc::c_ulong) as usize]
            ^ (*little_table
                .offset(
                    6 as libc::c_int as isize,
                ))[(crc >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(
                    5 as libc::c_int as isize,
                ))[(crc >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(
                    4 as libc::c_int as isize,
                ))[(crc >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(
                    3 as libc::c_int as isize,
                ))[(crc >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(
                    2 as libc::c_int as isize,
                ))[(crc >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(
                    1 as libc::c_int as isize,
                ))[(crc >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*little_table
                .offset(0 as libc::c_int as isize))[(crc >> 56 as libc::c_int) as usize];
        next = next.offset(8 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len != 0 {
        let fresh1 = next;
        next = next.offset(1);
        crc = (*little_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc ^ *fresh1 as libc::c_ulong) & 0xff as libc::c_int as libc::c_ulong)
            as usize] ^ crc >> 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    return crc;
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16little(
    mut little_table: *mut [uint16_t; 256],
    mut crc: uint16_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint16_t {
    let mut next: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    while len != 0
        && next as uintptr_t & 7 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        let fresh2 = next;
        next = next.offset(1);
        crc = ((*little_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc as libc::c_int >> 8 as libc::c_int ^ *fresh2 as libc::c_int)
            & 0xff as libc::c_int) as usize] as libc::c_int
            ^ (crc as libc::c_int) << 8 as libc::c_int) as uint16_t;
        len = len.wrapping_sub(1);
    }
    while len >= 8 as libc::c_int as libc::c_ulong {
        let mut n: uint64_t = *(next as *mut uint64_t);
        crc = ((*little_table
            .offset(
                7 as libc::c_int as isize,
            ))[(n & 0xff as libc::c_int as libc::c_ulong
            ^ (crc as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
                as libc::c_ulong) as usize] as libc::c_int
            ^ (*little_table
                .offset(
                    6 as libc::c_int as isize,
                ))[(n >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong
                ^ (crc as libc::c_int & 0xff as libc::c_int) as libc::c_ulong) as usize]
                as libc::c_int
            ^ (*little_table
                .offset(
                    5 as libc::c_int as isize,
                ))[(n >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*little_table
                .offset(
                    4 as libc::c_int as isize,
                ))[(n >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*little_table
                .offset(
                    3 as libc::c_int as isize,
                ))[(n >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*little_table
                .offset(
                    2 as libc::c_int as isize,
                ))[(n >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*little_table
                .offset(
                    1 as libc::c_int as isize,
                ))[(n >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*little_table
                .offset(0 as libc::c_int as isize))[(n >> 56 as libc::c_int) as usize]
                as libc::c_int) as uint16_t;
        next = next.offset(8 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len != 0 {
        let fresh3 = next;
        next = next.offset(1);
        crc = ((*little_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc as libc::c_int >> 8 as libc::c_int ^ *fresh3 as libc::c_int)
            & 0xff as libc::c_int) as usize] as libc::c_int
            ^ (crc as libc::c_int) << 8 as libc::c_int) as uint16_t;
        len = len.wrapping_sub(1);
    }
    return crc;
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed64big(
    mut big_table: *mut [uint64_t; 256],
    mut crc: uint64_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint64_t {
    let mut next: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    crc = rev8(crc);
    while len != 0
        && next as uintptr_t & 7 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        let fresh4 = next;
        next = next.offset(1);
        crc = (*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[(crc >> 56 as libc::c_int ^ *fresh4 as libc::c_ulong) as usize]
            ^ crc << 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    while len >= 8 as libc::c_int as libc::c_ulong {
        crc ^= *(next as *mut uint64_t);
        crc = (*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[(crc & 0xff as libc::c_int as libc::c_ulong) as usize]
            ^ (*big_table
                .offset(
                    1 as libc::c_int as isize,
                ))[(crc >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(
                    2 as libc::c_int as isize,
                ))[(crc >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(
                    3 as libc::c_int as isize,
                ))[(crc >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(
                    4 as libc::c_int as isize,
                ))[(crc >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(
                    5 as libc::c_int as isize,
                ))[(crc >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(
                    6 as libc::c_int as isize,
                ))[(crc >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize]
            ^ (*big_table
                .offset(7 as libc::c_int as isize))[(crc >> 56 as libc::c_int) as usize];
        next = next.offset(8 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len != 0 {
        let fresh5 = next;
        next = next.offset(1);
        crc = (*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[(crc >> 56 as libc::c_int ^ *fresh5 as libc::c_ulong) as usize]
            ^ crc << 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    return rev8(crc);
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16big(
    mut big_table: *mut [uint16_t; 256],
    mut crc_in: uint16_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint16_t {
    let mut next: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut crc: uint64_t = crc_in as uint64_t;
    crc = rev8(crc);
    while len != 0
        && next as uintptr_t & 7 as libc::c_int as libc::c_ulong
            != 0 as libc::c_int as libc::c_ulong
    {
        let fresh6 = next;
        next = next.offset(1);
        crc = (*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc >> 56 as libc::c_int - 8 as libc::c_int ^ *fresh6 as libc::c_ulong)
            & 0xff as libc::c_int as libc::c_ulong) as usize] as libc::c_ulong
            ^ crc >> 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    while len >= 8 as libc::c_int as libc::c_ulong {
        let mut n: uint64_t = *(next as *mut uint64_t);
        crc = ((*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[(n & 0xff as libc::c_int as libc::c_ulong
            ^ crc >> 56 as libc::c_int - 8 as libc::c_int
                & 0xff as libc::c_int as libc::c_ulong) as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    1 as libc::c_int as isize,
                ))[(n >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong
                ^ crc & 0xff as libc::c_int as libc::c_ulong) as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    2 as libc::c_int as isize,
                ))[(n >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    3 as libc::c_int as isize,
                ))[(n >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    4 as libc::c_int as isize,
                ))[(n >> 32 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    5 as libc::c_int as isize,
                ))[(n >> 40 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*big_table
                .offset(
                    6 as libc::c_int as isize,
                ))[(n >> 48 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
                as usize] as libc::c_int
            ^ (*big_table
                .offset(7 as libc::c_int as isize))[(n >> 56 as libc::c_int) as usize]
                as libc::c_int) as uint64_t;
        next = next.offset(8 as libc::c_int as isize);
        len = (len as libc::c_ulong).wrapping_sub(8 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
    }
    while len != 0 {
        let fresh7 = next;
        next = next.offset(1);
        crc = (*big_table
            .offset(
                0 as libc::c_int as isize,
            ))[((crc >> 56 as libc::c_int - 8 as libc::c_int ^ *fresh7 as libc::c_ulong)
            & 0xff as libc::c_int as libc::c_ulong) as usize] as libc::c_ulong
            ^ crc >> 8 as libc::c_int;
        len = len.wrapping_sub(1);
    }
    return rev8(crc) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed64native(
    mut table: *mut [uint64_t; 256],
    mut crc: uint64_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint64_t {
    let mut n: uint64_t = 1 as libc::c_int as uint64_t;
    return if *(&mut n as *mut uint64_t as *mut libc::c_char) as libc::c_int != 0 {
        crcspeed64little(table, crc, buf, len)
    } else {
        crcspeed64big(table, crc, buf, len)
    };
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16native(
    mut table: *mut [uint16_t; 256],
    mut crc: uint16_t,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> uint16_t {
    let mut n: uint64_t = 1 as libc::c_int as uint64_t;
    return (if *(&mut n as *mut uint64_t as *mut libc::c_char) as libc::c_int != 0 {
        crcspeed16little(table, crc, buf, len) as libc::c_int
    } else {
        crcspeed16big(table, crc, buf, len) as libc::c_int
    }) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed64native_init(
    mut fn_0: crcfn64,
    mut table: *mut [uint64_t; 256],
) {
    let mut n: uint64_t = 1 as libc::c_int as uint64_t;
    if *(&mut n as *mut uint64_t as *mut libc::c_char) as libc::c_int != 0 {
        crcspeed64little_init(fn_0, table);
    } else {
        crcspeed64big_init(fn_0, table);
    };
}
#[no_mangle]
pub unsafe extern "C" fn crcspeed16native_init(
    mut fn_0: crcfn16,
    mut table: *mut [uint16_t; 256],
) {
    let mut n: uint64_t = 1 as libc::c_int as uint64_t;
    if *(&mut n as *mut uint64_t as *mut libc::c_char) as libc::c_int != 0 {
        crcspeed16little_init(fn_0, table);
    } else {
        crcspeed16big_init(fn_0, table);
    };
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn crcspeed64native_init(fn_0: crcfn64, table: *mut [uint64_t; 256]);
    fn crcspeed64native(
        table: *mut [uint64_t; 256],
        crc: uint64_t,
        buf: *mut libc::c_void,
        len: size_t,
    ) -> uint64_t;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uint_fast8_t = libc::c_uchar;
pub type uint_fast64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type crcfn64 = Option::<
    unsafe extern "C" fn(uint64_t, *const libc::c_void, uint64_t) -> uint64_t,
>;
static mut crc64_table: [[uint64_t; 256]; 8] = [
    [
        0 as libc::c_int as uint64_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ],
    [0; 256],
    [0; 256],
    [0; 256],
    [0; 256],
    [0; 256],
    [0; 256],
    [0; 256],
];
#[inline]
unsafe extern "C" fn crc_reflect(
    mut data: uint_fast64_t,
    mut data_len: size_t,
) -> uint_fast64_t {
    let mut ret: uint_fast64_t = data & 0x1 as libc::c_int as libc::c_ulong;
    let mut i: size_t = 1 as libc::c_int as size_t;
    while i < data_len {
        data >>= 1 as libc::c_int;
        ret = ret << 1 as libc::c_int | data & 0x1 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn _crc64(
    mut crc: uint_fast64_t,
    mut in_data: *const libc::c_void,
    len: uint64_t,
) -> uint64_t {
    let mut data: *const uint8_t = in_data as *const uint8_t;
    let mut bit: libc::c_ulonglong = 0;
    let mut offset: uint64_t = 0 as libc::c_int as uint64_t;
    while offset < len {
        let mut c: uint8_t = *data.offset(offset as isize);
        let mut i: uint_fast8_t = 0x1 as libc::c_int as uint_fast8_t;
        while i as libc::c_int & 0xff as libc::c_int != 0 {
            bit = (crc & 0x8000000000000000 as libc::c_ulong) as libc::c_ulonglong;
            if c as libc::c_int & i as libc::c_int != 0 {
                bit = (bit == 0) as libc::c_int as libc::c_ulonglong;
            }
            crc <<= 1 as libc::c_int;
            if bit != 0 {
                crc ^= 0xad93d23594c935a9 as libc::c_ulong;
            }
            i = ((i as libc::c_int) << 1 as libc::c_int) as uint_fast8_t;
        }
        crc &= 0xffffffffffffffff as libc::c_ulong;
        offset = offset.wrapping_add(1);
    }
    crc = crc & 0xffffffffffffffff as libc::c_ulong;
    return crc_reflect(crc, 64 as libc::c_int as size_t)
        ^ 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn crc64_init() {
    crcspeed64native_init(
        Some(
            _crc64
                as unsafe extern "C" fn(
                    uint_fast64_t,
                    *const libc::c_void,
                    uint64_t,
                ) -> uint64_t,
        ),
        crc64_table.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn crc64(
    mut crc: uint64_t,
    mut s: *const libc::c_uchar,
    mut l: uint64_t,
) -> uint64_t {
    return crcspeed64native(crc64_table.as_mut_ptr(), crc, s as *mut libc::c_void, l);
}

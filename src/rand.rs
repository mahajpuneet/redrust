extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type int32_t = __int32_t;
pub type uint32_t = __uint32_t;
static mut x: [uint32_t; 3] = [
    0x330e as libc::c_int as uint32_t,
    0xabcd as libc::c_int as uint32_t,
    0x1234 as libc::c_int as uint32_t,
];
static mut a: [uint32_t; 3] = [
    0xe66d as libc::c_int as uint32_t,
    0xdeec as libc::c_int as uint32_t,
    0x5 as libc::c_int as uint32_t,
];
static mut c: uint32_t = 0xb as libc::c_int as uint32_t;
#[no_mangle]
pub unsafe extern "C" fn redisLrand48() -> int32_t {
    next();
    return (((x[2 as libc::c_int as usize] as int32_t)
        << 16 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
        .wrapping_add(x[1 as libc::c_int as usize] >> 1 as libc::c_int) as int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn redisSrand48(mut seedval: int32_t) {
    x[0 as libc::c_int as usize] = 0x330e as libc::c_int as uint32_t;
    x[1 as libc::c_int
        as usize] = seedval as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    x[2 as libc::c_int
        as usize] = (seedval >> 16 as libc::c_int) as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    a[0 as libc::c_int as usize] = 0xe66d as libc::c_int as uint32_t;
    a[1 as libc::c_int as usize] = 0xdeec as libc::c_int as uint32_t;
    a[2 as libc::c_int as usize] = 0x5 as libc::c_int as uint32_t;
    c = 0xb as libc::c_int as uint32_t;
}
unsafe extern "C" fn next() {
    let mut p: [uint32_t; 2] = [0; 2];
    let mut q: [uint32_t; 2] = [0; 2];
    let mut r: [uint32_t; 2] = [0; 2];
    let mut carry0: uint32_t = 0;
    let mut carry1: uint32_t = 0;
    let mut l: int32_t = (a[0 as libc::c_int as usize] as libc::c_long
        * x[0 as libc::c_int as usize] as libc::c_long) as int32_t;
    p[0 as libc::c_int
        as usize] = l as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    p[1 as libc::c_int
        as usize] = (l >> 16 as libc::c_int) as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    carry0 = (p[0 as libc::c_int as usize] as int32_t as libc::c_long + c as libc::c_long
        > (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_long) as libc::c_int as uint32_t;
    p[0 as libc::c_int
        as usize] = (p[0 as libc::c_int as usize]).wrapping_add(c)
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    carry1 = (p[1 as libc::c_int as usize] as int32_t as libc::c_long
        + carry0 as libc::c_long
        > (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_long) as libc::c_int as uint32_t;
    p[1 as libc::c_int
        as usize] = (p[1 as libc::c_int as usize]).wrapping_add(carry0)
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    let mut l_0: int32_t = (a[0 as libc::c_int as usize] as libc::c_long
        * x[1 as libc::c_int as usize] as libc::c_long) as int32_t;
    q[0 as libc::c_int
        as usize] = l_0 as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    q[1 as libc::c_int
        as usize] = (l_0 >> 16 as libc::c_int) as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    carry0 = (p[1 as libc::c_int as usize] as int32_t as libc::c_long
        + q[0 as libc::c_int as usize] as libc::c_long
        > (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_long) as libc::c_int as uint32_t;
    p[1 as libc::c_int
        as usize] = (p[1 as libc::c_int as usize])
        .wrapping_add(q[0 as libc::c_int as usize])
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    let mut l_1: int32_t = (a[1 as libc::c_int as usize] as libc::c_long
        * x[0 as libc::c_int as usize] as libc::c_long) as int32_t;
    r[0 as libc::c_int
        as usize] = l_1 as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    r[1 as libc::c_int
        as usize] = (l_1 >> 16 as libc::c_int) as libc::c_uint
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    x[2 as libc::c_int
        as usize] = carry0
        .wrapping_add(carry1)
        .wrapping_add(
            (p[1 as libc::c_int as usize] as int32_t as libc::c_long
                + r[0 as libc::c_int as usize] as libc::c_long
                > (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
                    + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
                    - 1 as libc::c_int) as libc::c_long) as libc::c_int as libc::c_uint,
        )
        .wrapping_add(q[1 as libc::c_int as usize])
        .wrapping_add(r[1 as libc::c_int as usize])
        .wrapping_add(
            (a[0 as libc::c_int as usize]).wrapping_mul(x[2 as libc::c_int as usize]),
        )
        .wrapping_add(
            (a[1 as libc::c_int as usize]).wrapping_mul(x[1 as libc::c_int as usize]),
        )
        .wrapping_add(
            (a[2 as libc::c_int as usize]).wrapping_mul(x[0 as libc::c_int as usize]),
        )
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    x[1 as libc::c_int
        as usize] = (p[1 as libc::c_int as usize])
        .wrapping_add(r[0 as libc::c_int as usize])
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
    x[0 as libc::c_int
        as usize] = p[0 as libc::c_int as usize]
        & (((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            + ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int) as libc::c_uint;
}

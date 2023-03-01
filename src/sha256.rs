extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
static mut k: [WORD; 64] = [
    0x428a2f98 as libc::c_int as WORD,
    0x71374491 as libc::c_int as WORD,
    0xb5c0fbcf as libc::c_uint,
    0xe9b5dba5 as libc::c_uint,
    0x3956c25b as libc::c_int as WORD,
    0x59f111f1 as libc::c_int as WORD,
    0x923f82a4 as libc::c_uint,
    0xab1c5ed5 as libc::c_uint,
    0xd807aa98 as libc::c_uint,
    0x12835b01 as libc::c_int as WORD,
    0x243185be as libc::c_int as WORD,
    0x550c7dc3 as libc::c_int as WORD,
    0x72be5d74 as libc::c_int as WORD,
    0x80deb1fe as libc::c_uint,
    0x9bdc06a7 as libc::c_uint,
    0xc19bf174 as libc::c_uint,
    0xe49b69c1 as libc::c_uint,
    0xefbe4786 as libc::c_uint,
    0xfc19dc6 as libc::c_int as WORD,
    0x240ca1cc as libc::c_int as WORD,
    0x2de92c6f as libc::c_int as WORD,
    0x4a7484aa as libc::c_int as WORD,
    0x5cb0a9dc as libc::c_int as WORD,
    0x76f988da as libc::c_int as WORD,
    0x983e5152 as libc::c_uint,
    0xa831c66d as libc::c_uint,
    0xb00327c8 as libc::c_uint,
    0xbf597fc7 as libc::c_uint,
    0xc6e00bf3 as libc::c_uint,
    0xd5a79147 as libc::c_uint,
    0x6ca6351 as libc::c_int as WORD,
    0x14292967 as libc::c_int as WORD,
    0x27b70a85 as libc::c_int as WORD,
    0x2e1b2138 as libc::c_int as WORD,
    0x4d2c6dfc as libc::c_int as WORD,
    0x53380d13 as libc::c_int as WORD,
    0x650a7354 as libc::c_int as WORD,
    0x766a0abb as libc::c_int as WORD,
    0x81c2c92e as libc::c_uint,
    0x92722c85 as libc::c_uint,
    0xa2bfe8a1 as libc::c_uint,
    0xa81a664b as libc::c_uint,
    0xc24b8b70 as libc::c_uint,
    0xc76c51a3 as libc::c_uint,
    0xd192e819 as libc::c_uint,
    0xd6990624 as libc::c_uint,
    0xf40e3585 as libc::c_uint,
    0x106aa070 as libc::c_int as WORD,
    0x19a4c116 as libc::c_int as WORD,
    0x1e376c08 as libc::c_int as WORD,
    0x2748774c as libc::c_int as WORD,
    0x34b0bcb5 as libc::c_int as WORD,
    0x391c0cb3 as libc::c_int as WORD,
    0x4ed8aa4a as libc::c_int as WORD,
    0x5b9cca4f as libc::c_int as WORD,
    0x682e6ff3 as libc::c_int as WORD,
    0x748f82ee as libc::c_int as WORD,
    0x78a5636f as libc::c_int as WORD,
    0x84c87814 as libc::c_uint,
    0x8cc70208 as libc::c_uint,
    0x90befffa as libc::c_uint,
    0xa4506ceb as libc::c_uint,
    0xbef9a3f7 as libc::c_uint,
    0xc67178f2 as libc::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn sha256_transform(
    mut ctx: *mut SHA256_CTX,
    mut data: *const BYTE,
) {
    let mut a: WORD = 0;
    let mut b: WORD = 0;
    let mut c: WORD = 0;
    let mut d: WORD = 0;
    let mut e: WORD = 0;
    let mut f: WORD = 0;
    let mut g: WORD = 0;
    let mut h: WORD = 0;
    let mut i: WORD = 0;
    let mut j: WORD = 0;
    let mut t1: WORD = 0;
    let mut t2: WORD = 0;
    let mut m: [WORD; 64] = [0; 64];
    i = 0 as libc::c_int as WORD;
    j = 0 as libc::c_int as WORD;
    while i < 16 as libc::c_int as libc::c_uint {
        m[i
            as usize] = (*data
            .offset(j.wrapping_add(0 as libc::c_int as libc::c_uint) as isize) as WORD)
            << 24 as libc::c_int
            | (*data.offset(j.wrapping_add(1 as libc::c_int as libc::c_uint) as isize)
                as WORD) << 16 as libc::c_int
            | (*data.offset(j.wrapping_add(2 as libc::c_int as libc::c_uint) as isize)
                as WORD) << 8 as libc::c_int
            | *data.offset(j.wrapping_add(3 as libc::c_int as libc::c_uint) as isize)
                as WORD;
        i = i.wrapping_add(1);
        j = (j as libc::c_uint).wrapping_add(4 as libc::c_int as libc::c_uint) as WORD
            as WORD;
    }
    while i < 64 as libc::c_int as libc::c_uint {
        m[i
            as usize] = ((m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
            >> 17 as libc::c_int
            | m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                << 32 as libc::c_int - 17 as libc::c_int)
            ^ (m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                >> 19 as libc::c_int
                | m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                    << 32 as libc::c_int - 19 as libc::c_int)
            ^ m[i.wrapping_sub(2 as libc::c_int as libc::c_uint) as usize]
                >> 10 as libc::c_int)
            .wrapping_add(m[i.wrapping_sub(7 as libc::c_int as libc::c_uint) as usize])
            .wrapping_add(
                (m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                    >> 7 as libc::c_int
                    | m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                        << 32 as libc::c_int - 7 as libc::c_int)
                    ^ (m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                        >> 18 as libc::c_int
                        | m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                            << 32 as libc::c_int - 18 as libc::c_int)
                    ^ m[i.wrapping_sub(15 as libc::c_int as libc::c_uint) as usize]
                        >> 3 as libc::c_int,
            )
            .wrapping_add(m[i.wrapping_sub(16 as libc::c_int as libc::c_uint) as usize]);
        i = i.wrapping_add(1);
    }
    a = (*ctx).state[0 as libc::c_int as usize];
    b = (*ctx).state[1 as libc::c_int as usize];
    c = (*ctx).state[2 as libc::c_int as usize];
    d = (*ctx).state[3 as libc::c_int as usize];
    e = (*ctx).state[4 as libc::c_int as usize];
    f = (*ctx).state[5 as libc::c_int as usize];
    g = (*ctx).state[6 as libc::c_int as usize];
    h = (*ctx).state[7 as libc::c_int as usize];
    i = 0 as libc::c_int as WORD;
    while i < 64 as libc::c_int as libc::c_uint {
        t1 = h
            .wrapping_add(
                (e >> 6 as libc::c_int | e << 32 as libc::c_int - 6 as libc::c_int)
                    ^ (e >> 11 as libc::c_int
                        | e << 32 as libc::c_int - 11 as libc::c_int)
                    ^ (e >> 25 as libc::c_int
                        | e << 32 as libc::c_int - 25 as libc::c_int),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(k[i as usize])
            .wrapping_add(m[i as usize]);
        t2 = ((a >> 2 as libc::c_int | a << 32 as libc::c_int - 2 as libc::c_int)
            ^ (a >> 13 as libc::c_int | a << 32 as libc::c_int - 13 as libc::c_int)
            ^ (a >> 22 as libc::c_int | a << 32 as libc::c_int - 22 as libc::c_int))
            .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
        i = i.wrapping_add(1);
    }
    (*ctx)
        .state[0 as libc::c_int
        as usize] = ((*ctx).state[0 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(a) as WORD as WORD;
    (*ctx)
        .state[1 as libc::c_int
        as usize] = ((*ctx).state[1 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(b) as WORD as WORD;
    (*ctx)
        .state[2 as libc::c_int
        as usize] = ((*ctx).state[2 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(c) as WORD as WORD;
    (*ctx)
        .state[3 as libc::c_int
        as usize] = ((*ctx).state[3 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(d) as WORD as WORD;
    (*ctx)
        .state[4 as libc::c_int
        as usize] = ((*ctx).state[4 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(e) as WORD as WORD;
    (*ctx)
        .state[5 as libc::c_int
        as usize] = ((*ctx).state[5 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(f) as WORD as WORD;
    (*ctx)
        .state[6 as libc::c_int
        as usize] = ((*ctx).state[6 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(g) as WORD as WORD;
    (*ctx)
        .state[7 as libc::c_int
        as usize] = ((*ctx).state[7 as libc::c_int as usize] as libc::c_uint)
        .wrapping_add(h) as WORD as WORD;
}
#[no_mangle]
pub unsafe extern "C" fn sha256_init(mut ctx: *mut SHA256_CTX) {
    (*ctx).datalen = 0 as libc::c_int as WORD;
    (*ctx).bitlen = 0 as libc::c_int as libc::c_ulonglong;
    (*ctx).state[0 as libc::c_int as usize] = 0x6a09e667 as libc::c_int as WORD;
    (*ctx).state[1 as libc::c_int as usize] = 0xbb67ae85 as libc::c_uint;
    (*ctx).state[2 as libc::c_int as usize] = 0x3c6ef372 as libc::c_int as WORD;
    (*ctx).state[3 as libc::c_int as usize] = 0xa54ff53a as libc::c_uint;
    (*ctx).state[4 as libc::c_int as usize] = 0x510e527f as libc::c_int as WORD;
    (*ctx).state[5 as libc::c_int as usize] = 0x9b05688c as libc::c_uint;
    (*ctx).state[6 as libc::c_int as usize] = 0x1f83d9ab as libc::c_int as WORD;
    (*ctx).state[7 as libc::c_int as usize] = 0x5be0cd19 as libc::c_int as WORD;
}
#[no_mangle]
pub unsafe extern "C" fn sha256_update(
    mut ctx: *mut SHA256_CTX,
    mut data: *const BYTE,
    mut len: size_t,
) {
    let mut i: WORD = 0;
    i = 0 as libc::c_int as WORD;
    while (i as libc::c_ulong) < len {
        (*ctx).data[(*ctx).datalen as usize] = *data.offset(i as isize);
        (*ctx).datalen = ((*ctx).datalen).wrapping_add(1);
        if (*ctx).datalen == 64 as libc::c_int as libc::c_uint {
            sha256_transform(ctx, ((*ctx).data).as_mut_ptr() as *const BYTE);
            (*ctx)
                .bitlen = ((*ctx).bitlen)
                .wrapping_add(512 as libc::c_int as libc::c_ulonglong);
            (*ctx).datalen = 0 as libc::c_int as WORD;
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sha256_final(mut ctx: *mut SHA256_CTX, mut hash: *mut BYTE) {
    let mut i: WORD = 0;
    i = (*ctx).datalen;
    if (*ctx).datalen < 56 as libc::c_int as libc::c_uint {
        let fresh0 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh0 as usize] = 0x80 as libc::c_int as BYTE;
        while i < 56 as libc::c_int as libc::c_uint {
            let fresh1 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh1 as usize] = 0 as libc::c_int as BYTE;
        }
    } else {
        let fresh2 = i;
        i = i.wrapping_add(1);
        (*ctx).data[fresh2 as usize] = 0x80 as libc::c_int as BYTE;
        while i < 64 as libc::c_int as libc::c_uint {
            let fresh3 = i;
            i = i.wrapping_add(1);
            (*ctx).data[fresh3 as usize] = 0 as libc::c_int as BYTE;
        }
        sha256_transform(ctx, ((*ctx).data).as_mut_ptr() as *const BYTE);
        memset(
            ((*ctx).data).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            56 as libc::c_int as libc::c_ulong,
        );
    }
    (*ctx)
        .bitlen = ((*ctx).bitlen)
        .wrapping_add(
            ((*ctx).datalen).wrapping_mul(8 as libc::c_int as libc::c_uint)
                as libc::c_ulonglong,
        );
    (*ctx).data[63 as libc::c_int as usize] = (*ctx).bitlen as BYTE;
    (*ctx)
        .data[62 as libc::c_int as usize] = ((*ctx).bitlen >> 8 as libc::c_int) as BYTE;
    (*ctx)
        .data[61 as libc::c_int as usize] = ((*ctx).bitlen >> 16 as libc::c_int) as BYTE;
    (*ctx)
        .data[60 as libc::c_int as usize] = ((*ctx).bitlen >> 24 as libc::c_int) as BYTE;
    (*ctx)
        .data[59 as libc::c_int as usize] = ((*ctx).bitlen >> 32 as libc::c_int) as BYTE;
    (*ctx)
        .data[58 as libc::c_int as usize] = ((*ctx).bitlen >> 40 as libc::c_int) as BYTE;
    (*ctx)
        .data[57 as libc::c_int as usize] = ((*ctx).bitlen >> 48 as libc::c_int) as BYTE;
    (*ctx)
        .data[56 as libc::c_int as usize] = ((*ctx).bitlen >> 56 as libc::c_int) as BYTE;
    sha256_transform(ctx, ((*ctx).data).as_mut_ptr() as *const BYTE);
    i = 0 as libc::c_int as WORD;
    while i < 4 as libc::c_int as libc::c_uint {
        *hash
            .offset(
                i as isize,
            ) = ((*ctx).state[0 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(4 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[1 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(8 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[2 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(12 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[3 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(16 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[4 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(20 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[5 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(24 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[6 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        *hash
            .offset(
                i.wrapping_add(28 as libc::c_int as libc::c_uint) as isize,
            ) = ((*ctx).state[7 as libc::c_int as usize]
            >> (24 as libc::c_int as libc::c_uint)
                .wrapping_sub(i.wrapping_mul(8 as libc::c_int as libc::c_uint))
            & 0xff as libc::c_int as libc::c_uint) as BYTE;
        i = i.wrapping_add(1);
    }
}

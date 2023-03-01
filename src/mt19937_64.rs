extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
static mut mt: [libc::c_ulonglong; 312] = [0; 312];
static mut mti: libc::c_int = 312 as libc::c_int + 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn init_genrand64(mut seed: libc::c_ulonglong) {
    mt[0 as libc::c_int as usize] = seed;
    mti = 1 as libc::c_int;
    while mti < 312 as libc::c_int {
        mt[mti
            as usize] = (6364136223846793005 as libc::c_ulonglong)
            .wrapping_mul(
                mt[(mti - 1 as libc::c_int) as usize]
                    ^ mt[(mti - 1 as libc::c_int) as usize] >> 62 as libc::c_int,
            )
            .wrapping_add(mti as libc::c_ulonglong);
        mti += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn init_by_array64(
    mut init_key: *mut libc::c_ulonglong,
    mut key_length: libc::c_ulonglong,
) {
    let mut i: libc::c_ulonglong = 0;
    let mut j: libc::c_ulonglong = 0;
    let mut k: libc::c_ulonglong = 0;
    init_genrand64(19650218 as libc::c_ulonglong);
    i = 1 as libc::c_int as libc::c_ulonglong;
    j = 0 as libc::c_int as libc::c_ulonglong;
    k = if 312 as libc::c_int as libc::c_ulonglong > key_length {
        312 as libc::c_int as libc::c_ulonglong
    } else {
        key_length
    };
    while k != 0 {
        mt[i
            as usize] = (mt[i as usize]
            ^ (mt[i.wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as usize]
                ^ mt[i.wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as usize]
                    >> 62 as libc::c_int)
                .wrapping_mul(3935559000370003845 as libc::c_ulonglong))
            .wrapping_add(*init_key.offset(j as isize))
            .wrapping_add(j);
        i = i.wrapping_add(1);
        j = j.wrapping_add(1);
        if i >= 312 as libc::c_int as libc::c_ulonglong {
            mt[0 as libc::c_int
                as usize] = mt[(312 as libc::c_int - 1 as libc::c_int) as usize];
            i = 1 as libc::c_int as libc::c_ulonglong;
        }
        if j >= key_length {
            j = 0 as libc::c_int as libc::c_ulonglong;
        }
        k = k.wrapping_sub(1);
    }
    k = (312 as libc::c_int - 1 as libc::c_int) as libc::c_ulonglong;
    while k != 0 {
        mt[i
            as usize] = (mt[i as usize]
            ^ (mt[i.wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as usize]
                ^ mt[i.wrapping_sub(1 as libc::c_int as libc::c_ulonglong) as usize]
                    >> 62 as libc::c_int)
                .wrapping_mul(2862933555777941757 as libc::c_ulonglong))
            .wrapping_sub(i);
        i = i.wrapping_add(1);
        if i >= 312 as libc::c_int as libc::c_ulonglong {
            mt[0 as libc::c_int
                as usize] = mt[(312 as libc::c_int - 1 as libc::c_int) as usize];
            i = 1 as libc::c_int as libc::c_ulonglong;
        }
        k = k.wrapping_sub(1);
    }
    mt[0 as libc::c_int as usize] = (1 as libc::c_ulonglong) << 63 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn genrand64_int64() -> libc::c_ulonglong {
    let mut i: libc::c_int = 0;
    let mut x: libc::c_ulonglong = 0;
    static mut mag01: [libc::c_ulonglong; 2] = [
        0 as libc::c_ulonglong,
        0xb5026f5aa96619e9 as libc::c_ulonglong,
    ];
    if mti >= 312 as libc::c_int {
        if mti == 312 as libc::c_int + 1 as libc::c_int {
            init_genrand64(5489 as libc::c_ulonglong);
        }
        i = 0 as libc::c_int;
        while i < 312 as libc::c_int - 156 as libc::c_int {
            x = mt[i as usize] & 0xffffffff80000000 as libc::c_ulonglong
                | mt[(i + 1 as libc::c_int) as usize] & 0x7fffffff as libc::c_ulonglong;
            mt[i
                as usize] = mt[(i + 156 as libc::c_int) as usize] ^ x >> 1 as libc::c_int
                ^ mag01[(x & 1 as libc::c_ulonglong) as libc::c_int as usize];
            i += 1;
        }
        while i < 312 as libc::c_int - 1 as libc::c_int {
            x = mt[i as usize] & 0xffffffff80000000 as libc::c_ulonglong
                | mt[(i + 1 as libc::c_int) as usize] & 0x7fffffff as libc::c_ulonglong;
            mt[i
                as usize] = mt[(i + (156 as libc::c_int - 312 as libc::c_int)) as usize]
                ^ x >> 1 as libc::c_int
                ^ mag01[(x & 1 as libc::c_ulonglong) as libc::c_int as usize];
            i += 1;
        }
        x = mt[(312 as libc::c_int - 1 as libc::c_int) as usize]
            & 0xffffffff80000000 as libc::c_ulonglong
            | mt[0 as libc::c_int as usize] & 0x7fffffff as libc::c_ulonglong;
        mt[(312 as libc::c_int - 1 as libc::c_int)
            as usize] = mt[(156 as libc::c_int - 1 as libc::c_int) as usize]
            ^ x >> 1 as libc::c_int
            ^ mag01[(x & 1 as libc::c_ulonglong) as libc::c_int as usize];
        mti = 0 as libc::c_int;
    }
    let fresh0 = mti;
    mti = mti + 1;
    x = mt[fresh0 as usize];
    x ^= x >> 29 as libc::c_int & 0x5555555555555555 as libc::c_ulonglong;
    x ^= x << 17 as libc::c_int & 0x71d67fffeda60000 as libc::c_ulonglong;
    x ^= x << 37 as libc::c_int & 0xfff7eee000000000 as libc::c_ulonglong;
    x ^= x >> 43 as libc::c_int;
    return x;
}
#[no_mangle]
pub unsafe extern "C" fn genrand64_int63() -> libc::c_longlong {
    return (genrand64_int64() >> 1 as libc::c_int) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn genrand64_real1() -> libc::c_double {
    return (genrand64_int64() >> 11 as libc::c_int) as libc::c_double
        * (1.0f64 / 9007199254740991.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn genrand64_real2() -> libc::c_double {
    return (genrand64_int64() >> 11 as libc::c_int) as libc::c_double
        * (1.0f64 / 9007199254740992.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn genrand64_real3() -> libc::c_double {
    return ((genrand64_int64() >> 12 as libc::c_int) as libc::c_double + 0.5f64)
        * (1.0f64 / 4503599627370496.0f64);
}

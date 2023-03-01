extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type int8_t = __int8_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashBits {
    pub bits: uint64_t,
    pub step: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashRange {
    pub min: libc::c_double,
    pub max: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashArea {
    pub hash: GeoHashBits,
    pub longitude: GeoHashRange,
    pub latitude: GeoHashRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashNeighbors {
    pub north: GeoHashBits,
    pub east: GeoHashBits,
    pub west: GeoHashBits,
    pub south: GeoHashBits,
    pub north_east: GeoHashBits,
    pub south_east: GeoHashBits,
    pub north_west: GeoHashBits,
    pub south_west: GeoHashBits,
}
#[inline]
unsafe extern "C" fn interleave64(mut xlo: uint32_t, mut ylo: uint32_t) -> uint64_t {
    static mut B: [uint64_t; 5] = [
        0x5555555555555555 as libc::c_ulonglong as uint64_t,
        0x3333333333333333 as libc::c_ulonglong as uint64_t,
        0xf0f0f0f0f0f0f0f as libc::c_ulonglong as uint64_t,
        0xff00ff00ff00ff as libc::c_ulonglong as uint64_t,
        0xffff0000ffff as libc::c_ulonglong as uint64_t,
    ];
    static mut S: [libc::c_uint; 5] = [
        1 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
    ];
    let mut x: uint64_t = xlo as uint64_t;
    let mut y: uint64_t = ylo as uint64_t;
    x = (x | x << S[4 as libc::c_int as usize]) & B[4 as libc::c_int as usize];
    y = (y | y << S[4 as libc::c_int as usize]) & B[4 as libc::c_int as usize];
    x = (x | x << S[3 as libc::c_int as usize]) & B[3 as libc::c_int as usize];
    y = (y | y << S[3 as libc::c_int as usize]) & B[3 as libc::c_int as usize];
    x = (x | x << S[2 as libc::c_int as usize]) & B[2 as libc::c_int as usize];
    y = (y | y << S[2 as libc::c_int as usize]) & B[2 as libc::c_int as usize];
    x = (x | x << S[1 as libc::c_int as usize]) & B[1 as libc::c_int as usize];
    y = (y | y << S[1 as libc::c_int as usize]) & B[1 as libc::c_int as usize];
    x = (x | x << S[0 as libc::c_int as usize]) & B[0 as libc::c_int as usize];
    y = (y | y << S[0 as libc::c_int as usize]) & B[0 as libc::c_int as usize];
    return x | y << 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn deinterleave64(mut interleaved: uint64_t) -> uint64_t {
    static mut B: [uint64_t; 6] = [
        0x5555555555555555 as libc::c_ulonglong as uint64_t,
        0x3333333333333333 as libc::c_ulonglong as uint64_t,
        0xf0f0f0f0f0f0f0f as libc::c_ulonglong as uint64_t,
        0xff00ff00ff00ff as libc::c_ulonglong as uint64_t,
        0xffff0000ffff as libc::c_ulonglong as uint64_t,
        0xffffffff as libc::c_ulonglong as uint64_t,
    ];
    static mut S: [libc::c_uint; 6] = [
        0 as libc::c_int as libc::c_uint,
        1 as libc::c_int as libc::c_uint,
        2 as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        8 as libc::c_int as libc::c_uint,
        16 as libc::c_int as libc::c_uint,
    ];
    let mut x: uint64_t = interleaved;
    let mut y: uint64_t = interleaved >> 1 as libc::c_int;
    x = (x | x >> S[0 as libc::c_int as usize]) & B[0 as libc::c_int as usize];
    y = (y | y >> S[0 as libc::c_int as usize]) & B[0 as libc::c_int as usize];
    x = (x | x >> S[1 as libc::c_int as usize]) & B[1 as libc::c_int as usize];
    y = (y | y >> S[1 as libc::c_int as usize]) & B[1 as libc::c_int as usize];
    x = (x | x >> S[2 as libc::c_int as usize]) & B[2 as libc::c_int as usize];
    y = (y | y >> S[2 as libc::c_int as usize]) & B[2 as libc::c_int as usize];
    x = (x | x >> S[3 as libc::c_int as usize]) & B[3 as libc::c_int as usize];
    y = (y | y >> S[3 as libc::c_int as usize]) & B[3 as libc::c_int as usize];
    x = (x | x >> S[4 as libc::c_int as usize]) & B[4 as libc::c_int as usize];
    y = (y | y >> S[4 as libc::c_int as usize]) & B[4 as libc::c_int as usize];
    x = (x | x >> S[5 as libc::c_int as usize]) & B[5 as libc::c_int as usize];
    y = (y | y >> S[5 as libc::c_int as usize]) & B[5 as libc::c_int as usize];
    return x | y << 32 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetCoordRange(
    mut long_range: *mut GeoHashRange,
    mut lat_range: *mut GeoHashRange,
) {
    (*long_range).max = 180 as libc::c_int as libc::c_double;
    (*long_range).min = -(180 as libc::c_int) as libc::c_double;
    (*lat_range).max = 85.05112878f64;
    (*lat_range).min = -85.05112878f64;
}
#[no_mangle]
pub unsafe extern "C" fn geohashEncode(
    mut long_range: *const GeoHashRange,
    mut lat_range: *const GeoHashRange,
    mut longitude: libc::c_double,
    mut latitude: libc::c_double,
    mut step: uint8_t,
    mut hash: *mut GeoHashBits,
) -> libc::c_int {
    if hash.is_null() || step as libc::c_int > 32 as libc::c_int
        || step as libc::c_int == 0 as libc::c_int
        || (lat_range.is_null() || (*lat_range).max == 0. && (*lat_range).min == 0.)
        || (long_range.is_null() || (*long_range).max == 0. && (*long_range).min == 0.)
    {
        return 0 as libc::c_int;
    }
    if longitude > 180 as libc::c_int as libc::c_double
        || longitude < -(180 as libc::c_int) as libc::c_double
        || latitude > 85.05112878f64 || latitude < -85.05112878f64
    {
        return 0 as libc::c_int;
    }
    (*hash).bits = 0 as libc::c_int as uint64_t;
    (*hash).step = step;
    if latitude < (*lat_range).min || latitude > (*lat_range).max
        || longitude < (*long_range).min || longitude > (*long_range).max
    {
        return 0 as libc::c_int;
    }
    let mut lat_offset: libc::c_double = (latitude - (*lat_range).min)
        / ((*lat_range).max - (*lat_range).min);
    let mut long_offset: libc::c_double = (longitude - (*long_range).min)
        / ((*long_range).max - (*long_range).min);
    lat_offset *= ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double;
    long_offset *= ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double;
    (*hash).bits = interleave64(lat_offset as uint32_t, long_offset as uint32_t);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashEncodeType(
    mut longitude: libc::c_double,
    mut latitude: libc::c_double,
    mut step: uint8_t,
    mut hash: *mut GeoHashBits,
) -> libc::c_int {
    let mut r: [GeoHashRange; 2] = [
        {
            let mut init = GeoHashRange {
                min: 0 as libc::c_int as libc::c_double,
                max: 0.,
            };
            init
        },
        GeoHashRange { min: 0., max: 0. },
    ];
    geohashGetCoordRange(
        &mut *r.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *r.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    return geohashEncode(
        &mut *r.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *r.as_mut_ptr().offset(1 as libc::c_int as isize),
        longitude,
        latitude,
        step,
        hash,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geohashEncodeWGS84(
    mut longitude: libc::c_double,
    mut latitude: libc::c_double,
    mut step: uint8_t,
    mut hash: *mut GeoHashBits,
) -> libc::c_int {
    return geohashEncodeType(longitude, latitude, step, hash);
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecode(
    long_range: GeoHashRange,
    lat_range: GeoHashRange,
    hash: GeoHashBits,
    mut area: *mut GeoHashArea,
) -> libc::c_int {
    if hash.bits == 0 && hash.step == 0 || area.is_null()
        || lat_range.max == 0. && lat_range.min == 0.
        || long_range.max == 0. && long_range.min == 0.
    {
        return 0 as libc::c_int;
    }
    (*area).hash = hash;
    let mut step: uint8_t = hash.step;
    let mut hash_sep: uint64_t = deinterleave64(hash.bits);
    let mut lat_scale: libc::c_double = lat_range.max - lat_range.min;
    let mut long_scale: libc::c_double = long_range.max - long_range.min;
    let mut ilato: uint32_t = hash_sep as uint32_t;
    let mut ilono: uint32_t = (hash_sep >> 32 as libc::c_int) as uint32_t;
    (*area)
        .latitude
        .min = lat_range.min
        + ilato as libc::c_double * 1.0f64
            / ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double
            * lat_scale;
    (*area)
        .latitude
        .max = lat_range.min
        + ilato.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_double * 1.0f64
            / ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double
            * lat_scale;
    (*area)
        .longitude
        .min = long_range.min
        + ilono as libc::c_double * 1.0f64
            / ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double
            * long_scale;
    (*area)
        .longitude
        .max = long_range.min
        + ilono.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_double * 1.0f64
            / ((1 as libc::c_ulonglong) << step as libc::c_int) as libc::c_double
            * long_scale;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecodeType(
    hash: GeoHashBits,
    mut area: *mut GeoHashArea,
) -> libc::c_int {
    let mut r: [GeoHashRange; 2] = [
        {
            let mut init = GeoHashRange {
                min: 0 as libc::c_int as libc::c_double,
                max: 0.,
            };
            init
        },
        GeoHashRange { min: 0., max: 0. },
    ];
    geohashGetCoordRange(
        &mut *r.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *r.as_mut_ptr().offset(1 as libc::c_int as isize),
    );
    return geohashDecode(
        r[0 as libc::c_int as usize],
        r[1 as libc::c_int as usize],
        hash,
        area,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecodeWGS84(
    hash: GeoHashBits,
    mut area: *mut GeoHashArea,
) -> libc::c_int {
    return geohashDecodeType(hash, area);
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecodeAreaToLongLat(
    mut area: *const GeoHashArea,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    if xy.is_null() {
        return 0 as libc::c_int;
    }
    *xy
        .offset(
            0 as libc::c_int as isize,
        ) = ((*area).longitude.min + (*area).longitude.max)
        / 2 as libc::c_int as libc::c_double;
    if *xy.offset(0 as libc::c_int as isize) > 180 as libc::c_int as libc::c_double {
        *xy.offset(0 as libc::c_int as isize) = 180 as libc::c_int as libc::c_double;
    }
    if *xy.offset(0 as libc::c_int as isize) < -(180 as libc::c_int) as libc::c_double {
        *xy.offset(0 as libc::c_int as isize) = -(180 as libc::c_int) as libc::c_double;
    }
    *xy
        .offset(
            1 as libc::c_int as isize,
        ) = ((*area).latitude.min + (*area).latitude.max)
        / 2 as libc::c_int as libc::c_double;
    if *xy.offset(1 as libc::c_int as isize) > 85.05112878f64 {
        *xy.offset(1 as libc::c_int as isize) = 85.05112878f64;
    }
    if *xy.offset(1 as libc::c_int as isize) < -85.05112878f64 {
        *xy.offset(1 as libc::c_int as isize) = -85.05112878f64;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecodeToLongLatType(
    hash: GeoHashBits,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    let mut area: GeoHashArea = {
        let mut init = GeoHashArea {
            hash: {
                let mut init = GeoHashBits {
                    bits: 0 as libc::c_int as uint64_t,
                    step: 0,
                };
                init
            },
            longitude: GeoHashRange { min: 0., max: 0. },
            latitude: GeoHashRange { min: 0., max: 0. },
        };
        init
    };
    if xy.is_null() || geohashDecodeType(hash, &mut area) == 0 {
        return 0 as libc::c_int;
    }
    return geohashDecodeAreaToLongLat(&mut area, xy);
}
#[no_mangle]
pub unsafe extern "C" fn geohashDecodeToLongLatWGS84(
    hash: GeoHashBits,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    return geohashDecodeToLongLatType(hash, xy);
}
unsafe extern "C" fn geohash_move_x(mut hash: *mut GeoHashBits, mut d: int8_t) {
    if d as libc::c_int == 0 as libc::c_int {
        return;
    }
    let mut x: uint64_t = ((*hash).bits as libc::c_ulonglong
        & 0xaaaaaaaaaaaaaaaa as libc::c_ulonglong) as uint64_t;
    let mut y: uint64_t = ((*hash).bits as libc::c_ulonglong
        & 0x5555555555555555 as libc::c_ulonglong) as uint64_t;
    let mut zz: uint64_t = (0x5555555555555555 as libc::c_ulonglong
        >> 64 as libc::c_int - (*hash).step as libc::c_int * 2 as libc::c_int)
        as uint64_t;
    if d as libc::c_int > 0 as libc::c_int {
        x = x.wrapping_add(zz.wrapping_add(1 as libc::c_int as libc::c_ulong));
    } else {
        x = x | zz;
        x = x.wrapping_sub(zz.wrapping_add(1 as libc::c_int as libc::c_ulong));
    }
    x = (x as libc::c_ulonglong
        & 0xaaaaaaaaaaaaaaaa as libc::c_ulonglong
            >> 64 as libc::c_int - (*hash).step as libc::c_int * 2 as libc::c_int)
        as uint64_t;
    (*hash).bits = x | y;
}
unsafe extern "C" fn geohash_move_y(mut hash: *mut GeoHashBits, mut d: int8_t) {
    if d as libc::c_int == 0 as libc::c_int {
        return;
    }
    let mut x: uint64_t = ((*hash).bits as libc::c_ulonglong
        & 0xaaaaaaaaaaaaaaaa as libc::c_ulonglong) as uint64_t;
    let mut y: uint64_t = ((*hash).bits as libc::c_ulonglong
        & 0x5555555555555555 as libc::c_ulonglong) as uint64_t;
    let mut zz: uint64_t = (0xaaaaaaaaaaaaaaaa as libc::c_ulonglong
        >> 64 as libc::c_int - (*hash).step as libc::c_int * 2 as libc::c_int)
        as uint64_t;
    if d as libc::c_int > 0 as libc::c_int {
        y = y.wrapping_add(zz.wrapping_add(1 as libc::c_int as libc::c_ulong));
    } else {
        y = y | zz;
        y = y.wrapping_sub(zz.wrapping_add(1 as libc::c_int as libc::c_ulong));
    }
    y = (y as libc::c_ulonglong
        & 0x5555555555555555 as libc::c_ulonglong
            >> 64 as libc::c_int - (*hash).step as libc::c_int * 2 as libc::c_int)
        as uint64_t;
    (*hash).bits = x | y;
}
#[no_mangle]
pub unsafe extern "C" fn geohashNeighbors(
    mut hash: *const GeoHashBits,
    mut neighbors: *mut GeoHashNeighbors,
) {
    (*neighbors).east = *hash;
    (*neighbors).west = *hash;
    (*neighbors).north = *hash;
    (*neighbors).south = *hash;
    (*neighbors).south_east = *hash;
    (*neighbors).south_west = *hash;
    (*neighbors).north_east = *hash;
    (*neighbors).north_west = *hash;
    geohash_move_x(&mut (*neighbors).east, 1 as libc::c_int as int8_t);
    geohash_move_y(&mut (*neighbors).east, 0 as libc::c_int as int8_t);
    geohash_move_x(&mut (*neighbors).west, -(1 as libc::c_int) as int8_t);
    geohash_move_y(&mut (*neighbors).west, 0 as libc::c_int as int8_t);
    geohash_move_x(&mut (*neighbors).south, 0 as libc::c_int as int8_t);
    geohash_move_y(&mut (*neighbors).south, -(1 as libc::c_int) as int8_t);
    geohash_move_x(&mut (*neighbors).north, 0 as libc::c_int as int8_t);
    geohash_move_y(&mut (*neighbors).north, 1 as libc::c_int as int8_t);
    geohash_move_x(&mut (*neighbors).north_west, -(1 as libc::c_int) as int8_t);
    geohash_move_y(&mut (*neighbors).north_west, 1 as libc::c_int as int8_t);
    geohash_move_x(&mut (*neighbors).north_east, 1 as libc::c_int as int8_t);
    geohash_move_y(&mut (*neighbors).north_east, 1 as libc::c_int as int8_t);
    geohash_move_x(&mut (*neighbors).south_east, 1 as libc::c_int as int8_t);
    geohash_move_y(&mut (*neighbors).south_east, -(1 as libc::c_int) as int8_t);
    geohash_move_x(&mut (*neighbors).south_west, -(1 as libc::c_int) as int8_t);
    geohash_move_y(&mut (*neighbors).south_west, -(1 as libc::c_int) as int8_t);
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn geohashGetCoordRange(long_range: *mut GeoHashRange, lat_range: *mut GeoHashRange);
    fn geohashEncode(
        long_range: *const GeoHashRange,
        lat_range: *const GeoHashRange,
        longitude: libc::c_double,
        latitude: libc::c_double,
        step: uint8_t,
        hash: *mut GeoHashBits,
    ) -> libc::c_int;
    fn geohashDecode(
        long_range: GeoHashRange,
        lat_range: GeoHashRange,
        hash: GeoHashBits,
        area: *mut GeoHashArea,
    ) -> libc::c_int;
    fn geohashNeighbors(hash: *const GeoHashBits, neighbors: *mut GeoHashNeighbors);
    fn asin(_: libc::c_double) -> libc::c_double;
    fn cos(_: libc::c_double) -> libc::c_double;
    fn sin(_: libc::c_double) -> libc::c_double;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoShape {
    pub type_0: libc::c_int,
    pub xy: [libc::c_double; 2],
    pub conversion: libc::c_double,
    pub bounds: [libc::c_double; 4],
    pub t: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub radius: libc::c_double,
    pub r: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub height: libc::c_double,
    pub width: libc::c_double,
}
pub type GeoHashFix52Bits = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashRadius {
    pub hash: GeoHashBits,
    pub area: GeoHashArea,
    pub neighbors: GeoHashNeighbors,
}
#[no_mangle]
pub static mut DEG_TO_RAD: libc::c_double = 0.017453292519943295769236907684886f64;
#[no_mangle]
pub static mut EARTH_RADIUS_IN_METERS: libc::c_double = 6372797.560856f64;
#[no_mangle]
pub static mut MERCATOR_MAX: libc::c_double = 20037726.37f64;
#[no_mangle]
pub static mut MERCATOR_MIN: libc::c_double = -20037726.37f64;
#[inline]
unsafe extern "C" fn deg_rad(mut ang: libc::c_double) -> libc::c_double {
    return ang * (3.14159265358979323846f64 / 180.0f64);
}
#[inline]
unsafe extern "C" fn rad_deg(mut ang: libc::c_double) -> libc::c_double {
    return ang / (3.14159265358979323846f64 / 180.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn geohashEstimateStepsByRadius(
    mut range_meters: libc::c_double,
    mut lat: libc::c_double,
) -> uint8_t {
    if range_meters == 0 as libc::c_int as libc::c_double {
        return 26 as libc::c_int as uint8_t;
    }
    let mut step: libc::c_int = 1 as libc::c_int;
    while range_meters < MERCATOR_MAX {
        range_meters *= 2 as libc::c_int as libc::c_double;
        step += 1;
    }
    step -= 2 as libc::c_int;
    if lat > 66 as libc::c_int as libc::c_double
        || lat < -(66 as libc::c_int) as libc::c_double
    {
        step -= 1;
        if lat > 80 as libc::c_int as libc::c_double
            || lat < -(80 as libc::c_int) as libc::c_double
        {
            step -= 1;
        }
    }
    if step < 1 as libc::c_int {
        step = 1 as libc::c_int;
    }
    if step > 26 as libc::c_int {
        step = 26 as libc::c_int;
    }
    return step as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn geohashBoundingBox(
    mut shape: *mut GeoShape,
    mut bounds: *mut libc::c_double,
) -> libc::c_int {
    if bounds.is_null() {
        return 0 as libc::c_int;
    }
    let mut longitude: libc::c_double = (*shape).xy[0 as libc::c_int as usize];
    let mut latitude: libc::c_double = (*shape).xy[1 as libc::c_int as usize];
    let mut height: libc::c_double = (*shape).conversion
        * (if (*shape).type_0 == 1 as libc::c_int {
            (*shape).t.radius
        } else {
            (*shape).t.r.height / 2 as libc::c_int as libc::c_double
        });
    let mut width: libc::c_double = (*shape).conversion
        * (if (*shape).type_0 == 1 as libc::c_int {
            (*shape).t.radius
        } else {
            (*shape).t.r.width / 2 as libc::c_int as libc::c_double
        });
    let lat_delta: libc::c_double = rad_deg(height / EARTH_RADIUS_IN_METERS);
    let long_delta_top: libc::c_double = rad_deg(
        width / EARTH_RADIUS_IN_METERS / cos(deg_rad(latitude + lat_delta)),
    );
    let long_delta_bottom: libc::c_double = rad_deg(
        width / EARTH_RADIUS_IN_METERS / cos(deg_rad(latitude - lat_delta)),
    );
    let mut southern_hemisphere: libc::c_int = if latitude
        < 0 as libc::c_int as libc::c_double
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    *bounds
        .offset(
            0 as libc::c_int as isize,
        ) = if southern_hemisphere != 0 {
        longitude - long_delta_bottom
    } else {
        longitude - long_delta_top
    };
    *bounds
        .offset(
            2 as libc::c_int as isize,
        ) = if southern_hemisphere != 0 {
        longitude + long_delta_bottom
    } else {
        longitude + long_delta_top
    };
    *bounds.offset(1 as libc::c_int as isize) = latitude - lat_delta;
    *bounds.offset(3 as libc::c_int as isize) = latitude + lat_delta;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashCalculateAreasByShapeWGS84(
    mut shape: *mut GeoShape,
) -> GeoHashRadius {
    let mut long_range: GeoHashRange = GeoHashRange { min: 0., max: 0. };
    let mut lat_range: GeoHashRange = GeoHashRange { min: 0., max: 0. };
    let mut radius: GeoHashRadius = GeoHashRadius {
        hash: GeoHashBits { bits: 0, step: 0 },
        area: GeoHashArea {
            hash: GeoHashBits { bits: 0, step: 0 },
            longitude: GeoHashRange { min: 0., max: 0. },
            latitude: GeoHashRange { min: 0., max: 0. },
        },
        neighbors: GeoHashNeighbors {
            north: GeoHashBits { bits: 0, step: 0 },
            east: GeoHashBits { bits: 0, step: 0 },
            west: GeoHashBits { bits: 0, step: 0 },
            south: GeoHashBits { bits: 0, step: 0 },
            north_east: GeoHashBits { bits: 0, step: 0 },
            south_east: GeoHashBits { bits: 0, step: 0 },
            north_west: GeoHashBits { bits: 0, step: 0 },
            south_west: GeoHashBits { bits: 0, step: 0 },
        },
    };
    let mut hash: GeoHashBits = GeoHashBits { bits: 0, step: 0 };
    let mut neighbors: GeoHashNeighbors = GeoHashNeighbors {
        north: GeoHashBits { bits: 0, step: 0 },
        east: GeoHashBits { bits: 0, step: 0 },
        west: GeoHashBits { bits: 0, step: 0 },
        south: GeoHashBits { bits: 0, step: 0 },
        north_east: GeoHashBits { bits: 0, step: 0 },
        south_east: GeoHashBits { bits: 0, step: 0 },
        north_west: GeoHashBits { bits: 0, step: 0 },
        south_west: GeoHashBits { bits: 0, step: 0 },
    };
    let mut area: GeoHashArea = GeoHashArea {
        hash: GeoHashBits { bits: 0, step: 0 },
        longitude: GeoHashRange { min: 0., max: 0. },
        latitude: GeoHashRange { min: 0., max: 0. },
    };
    let mut min_lon: libc::c_double = 0.;
    let mut max_lon: libc::c_double = 0.;
    let mut min_lat: libc::c_double = 0.;
    let mut max_lat: libc::c_double = 0.;
    let mut steps: libc::c_int = 0;
    geohashBoundingBox(shape, ((*shape).bounds).as_mut_ptr());
    min_lon = (*shape).bounds[0 as libc::c_int as usize];
    min_lat = (*shape).bounds[1 as libc::c_int as usize];
    max_lon = (*shape).bounds[2 as libc::c_int as usize];
    max_lat = (*shape).bounds[3 as libc::c_int as usize];
    let mut longitude: libc::c_double = (*shape).xy[0 as libc::c_int as usize];
    let mut latitude: libc::c_double = (*shape).xy[1 as libc::c_int as usize];
    let mut radius_meters: libc::c_double = if (*shape).type_0 == 1 as libc::c_int {
        (*shape).t.radius
    } else {
        sqrt(
            (*shape).t.r.width / 2 as libc::c_int as libc::c_double
                * ((*shape).t.r.width / 2 as libc::c_int as libc::c_double)
                + (*shape).t.r.height / 2 as libc::c_int as libc::c_double
                    * ((*shape).t.r.height / 2 as libc::c_int as libc::c_double),
        )
    };
    radius_meters *= (*shape).conversion;
    steps = geohashEstimateStepsByRadius(radius_meters, latitude) as libc::c_int;
    geohashGetCoordRange(&mut long_range, &mut lat_range);
    geohashEncode(
        &mut long_range,
        &mut lat_range,
        longitude,
        latitude,
        steps as uint8_t,
        &mut hash,
    );
    geohashNeighbors(&mut hash, &mut neighbors);
    geohashDecode(long_range, lat_range, hash, &mut area);
    let mut decrease_step: libc::c_int = 0 as libc::c_int;
    let mut north: GeoHashArea = GeoHashArea {
        hash: GeoHashBits { bits: 0, step: 0 },
        longitude: GeoHashRange { min: 0., max: 0. },
        latitude: GeoHashRange { min: 0., max: 0. },
    };
    let mut south: GeoHashArea = GeoHashArea {
        hash: GeoHashBits { bits: 0, step: 0 },
        longitude: GeoHashRange { min: 0., max: 0. },
        latitude: GeoHashRange { min: 0., max: 0. },
    };
    let mut east: GeoHashArea = GeoHashArea {
        hash: GeoHashBits { bits: 0, step: 0 },
        longitude: GeoHashRange { min: 0., max: 0. },
        latitude: GeoHashRange { min: 0., max: 0. },
    };
    let mut west: GeoHashArea = GeoHashArea {
        hash: GeoHashBits { bits: 0, step: 0 },
        longitude: GeoHashRange { min: 0., max: 0. },
        latitude: GeoHashRange { min: 0., max: 0. },
    };
    geohashDecode(long_range, lat_range, neighbors.north, &mut north);
    geohashDecode(long_range, lat_range, neighbors.south, &mut south);
    geohashDecode(long_range, lat_range, neighbors.east, &mut east);
    geohashDecode(long_range, lat_range, neighbors.west, &mut west);
    if north.latitude.max < max_lat {
        decrease_step = 1 as libc::c_int;
    }
    if south.latitude.min > min_lat {
        decrease_step = 1 as libc::c_int;
    }
    if east.longitude.max < max_lon {
        decrease_step = 1 as libc::c_int;
    }
    if west.longitude.min > min_lon {
        decrease_step = 1 as libc::c_int;
    }
    if steps > 1 as libc::c_int && decrease_step != 0 {
        steps -= 1;
        geohashEncode(
            &mut long_range,
            &mut lat_range,
            longitude,
            latitude,
            steps as uint8_t,
            &mut hash,
        );
        geohashNeighbors(&mut hash, &mut neighbors);
        geohashDecode(long_range, lat_range, hash, &mut area);
    }
    if steps >= 2 as libc::c_int {
        if area.latitude.min < min_lat {
            neighbors.south.step = 0 as libc::c_int as uint8_t;
            neighbors.south.bits = neighbors.south.step as uint64_t;
            neighbors.south_west.step = 0 as libc::c_int as uint8_t;
            neighbors.south_west.bits = neighbors.south_west.step as uint64_t;
            neighbors.south_east.step = 0 as libc::c_int as uint8_t;
            neighbors.south_east.bits = neighbors.south_east.step as uint64_t;
        }
        if area.latitude.max > max_lat {
            neighbors.north.step = 0 as libc::c_int as uint8_t;
            neighbors.north.bits = neighbors.north.step as uint64_t;
            neighbors.north_east.step = 0 as libc::c_int as uint8_t;
            neighbors.north_east.bits = neighbors.north_east.step as uint64_t;
            neighbors.north_west.step = 0 as libc::c_int as uint8_t;
            neighbors.north_west.bits = neighbors.north_west.step as uint64_t;
        }
        if area.longitude.min < min_lon {
            neighbors.west.step = 0 as libc::c_int as uint8_t;
            neighbors.west.bits = neighbors.west.step as uint64_t;
            neighbors.south_west.step = 0 as libc::c_int as uint8_t;
            neighbors.south_west.bits = neighbors.south_west.step as uint64_t;
            neighbors.north_west.step = 0 as libc::c_int as uint8_t;
            neighbors.north_west.bits = neighbors.north_west.step as uint64_t;
        }
        if area.longitude.max > max_lon {
            neighbors.east.step = 0 as libc::c_int as uint8_t;
            neighbors.east.bits = neighbors.east.step as uint64_t;
            neighbors.south_east.step = 0 as libc::c_int as uint8_t;
            neighbors.south_east.bits = neighbors.south_east.step as uint64_t;
            neighbors.north_east.step = 0 as libc::c_int as uint8_t;
            neighbors.north_east.bits = neighbors.north_east.step as uint64_t;
        }
    }
    radius.hash = hash;
    radius.neighbors = neighbors;
    radius.area = area;
    return radius;
}
#[no_mangle]
pub unsafe extern "C" fn geohashAlign52Bits(hash: GeoHashBits) -> GeoHashFix52Bits {
    let mut bits: uint64_t = hash.bits;
    bits <<= 52 as libc::c_int - hash.step as libc::c_int * 2 as libc::c_int;
    return bits;
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetLatDistance(
    mut lat1d: libc::c_double,
    mut lat2d: libc::c_double,
) -> libc::c_double {
    return EARTH_RADIUS_IN_METERS * fabs(deg_rad(lat2d) - deg_rad(lat1d));
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetDistance(
    mut lon1d: libc::c_double,
    mut lat1d: libc::c_double,
    mut lon2d: libc::c_double,
    mut lat2d: libc::c_double,
) -> libc::c_double {
    let mut lat1r: libc::c_double = 0.;
    let mut lon1r: libc::c_double = 0.;
    let mut lat2r: libc::c_double = 0.;
    let mut lon2r: libc::c_double = 0.;
    let mut u: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut a: libc::c_double = 0.;
    lon1r = deg_rad(lon1d);
    lon2r = deg_rad(lon2d);
    v = sin((lon2r - lon1r) / 2 as libc::c_int as libc::c_double);
    if v == 0.0f64 {
        return geohashGetLatDistance(lat1d, lat2d);
    }
    lat1r = deg_rad(lat1d);
    lat2r = deg_rad(lat2d);
    u = sin((lat2r - lat1r) / 2 as libc::c_int as libc::c_double);
    a = u * u + cos(lat1r) * cos(lat2r) * v * v;
    return 2.0f64 * EARTH_RADIUS_IN_METERS * asin(sqrt(a));
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetDistanceIfInRadius(
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
    mut radius: libc::c_double,
    mut distance: *mut libc::c_double,
) -> libc::c_int {
    *distance = geohashGetDistance(x1, y1, x2, y2);
    if *distance > radius {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetDistanceIfInRadiusWGS84(
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
    mut radius: libc::c_double,
    mut distance: *mut libc::c_double,
) -> libc::c_int {
    return geohashGetDistanceIfInRadius(x1, y1, x2, y2, radius, distance);
}
#[no_mangle]
pub unsafe extern "C" fn geohashGetDistanceIfInRectangle(
    mut width_m: libc::c_double,
    mut height_m: libc::c_double,
    mut x1: libc::c_double,
    mut y1: libc::c_double,
    mut x2: libc::c_double,
    mut y2: libc::c_double,
    mut distance: *mut libc::c_double,
) -> libc::c_int {
    let mut lat_distance: libc::c_double = geohashGetLatDistance(y2, y1);
    if lat_distance > height_m / 2 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    let mut lon_distance: libc::c_double = geohashGetDistance(x2, y2, x1, y2);
    if lon_distance > width_m / 2 as libc::c_int as libc::c_double {
        return 0 as libc::c_int;
    }
    *distance = geohashGetDistance(x1, y1, x2, y2);
    return 1 as libc::c_int;
}

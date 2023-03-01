extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type __time_t = libc::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub __tm_gmtoff: libc::c_long,
    pub __tm_zone: *const libc::c_char,
}
unsafe extern "C" fn is_leap_year(mut year: time_t) -> libc::c_int {
    if year % 4 as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int
    } else if year % 100 as libc::c_int as libc::c_long != 0 {
        return 1 as libc::c_int
    } else if year % 400 as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn nolocks_localtime(
    mut tmp: *mut tm,
    mut t: time_t,
    mut tz: time_t,
    mut dst: libc::c_int,
) {
    let secs_min: time_t = 60 as libc::c_int as time_t;
    let secs_hour: time_t = 3600 as libc::c_int as time_t;
    let secs_day: time_t = (3600 as libc::c_int * 24 as libc::c_int) as time_t;
    t -= tz;
    t += (3600 as libc::c_int * dst) as libc::c_long;
    let mut days: time_t = t / secs_day;
    let mut seconds: time_t = t % secs_day;
    (*tmp).tm_isdst = dst;
    (*tmp).tm_hour = (seconds / secs_hour) as libc::c_int;
    (*tmp).tm_min = (seconds % secs_hour / secs_min) as libc::c_int;
    (*tmp).tm_sec = (seconds % secs_hour % secs_min) as libc::c_int;
    (*tmp)
        .tm_wday = ((days + 4 as libc::c_int as libc::c_long)
        % 7 as libc::c_int as libc::c_long) as libc::c_int;
    (*tmp).tm_year = 1970 as libc::c_int;
    loop {
        let mut days_this_year: time_t = (365 as libc::c_int
            + is_leap_year((*tmp).tm_year as time_t)) as time_t;
        if days_this_year > days {
            break;
        }
        days -= days_this_year;
        (*tmp).tm_year += 1;
    }
    (*tmp).tm_yday = days as libc::c_int;
    let mut mdays: [libc::c_int; 12] = [
        31 as libc::c_int,
        28 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
        30 as libc::c_int,
        31 as libc::c_int,
    ];
    mdays[1 as libc::c_int as usize] += is_leap_year((*tmp).tm_year as time_t);
    (*tmp).tm_mon = 0 as libc::c_int;
    while days >= mdays[(*tmp).tm_mon as usize] as libc::c_long {
        days -= mdays[(*tmp).tm_mon as usize] as libc::c_long;
        (*tmp).tm_mon += 1;
    }
    (*tmp).tm_mday = (days + 1 as libc::c_int as libc::c_long) as libc::c_int;
    (*tmp).tm_year -= 1900 as libc::c_int;
}

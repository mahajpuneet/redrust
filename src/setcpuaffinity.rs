extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn sched_setaffinity(
        __pid: __pid_t,
        __cpusetsize: size_t,
        __cpuset: *const cpu_set_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __pid_t = libc::c_int;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type __cpu_mask = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cpu_set_t {
    pub __bits: [__cpu_mask; 16],
}
unsafe extern "C" fn next_token(
    mut q: *const libc::c_char,
    mut sep: libc::c_int,
) -> *const libc::c_char {
    if !q.is_null() {
        q = strchr(q, sep);
    }
    if !q.is_null() {
        q = q.offset(1);
    }
    return q;
}
unsafe extern "C" fn next_num(
    mut str: *const libc::c_char,
    mut end: *mut *mut libc::c_char,
    mut result: *mut libc::c_int,
) -> libc::c_int {
    if str.is_null() || *str as libc::c_int == '\0' as i32
        || *(*__ctype_b_loc()).offset(*str as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0
    {
        return -(1 as libc::c_int);
    }
    *result = strtoul(str, end, 10 as libc::c_int) as libc::c_int;
    if str == *end as *const libc::c_char {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn setcpuaffinity(mut cpulist: *const libc::c_char) {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut q: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cpuset: cpu_set_t = cpu_set_t { __bits: [0; 16] };
    if cpulist.is_null() {
        return;
    }
    libc::memset(
        &mut cpuset as *mut cpu_set_t as *mut libc::c_void,
        '\0' as i32,
        core::mem::size_of::<cpu_set_t>() as libc::c_ulong as libc::size_t,
    );
    q = cpulist;
    loop {
        p = q;
        q = next_token(q, ',' as i32);
        if p.is_null() {
            break;
        }
        let mut a: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut s: libc::c_int = 0;
        let mut c1: *const libc::c_char = 0 as *const libc::c_char;
        let mut c2: *const libc::c_char = 0 as *const libc::c_char;
        if next_num(p, &mut end, &mut a) != 0 as libc::c_int {
            return;
        }
        b = a;
        s = 1 as libc::c_int;
        p = end;
        c1 = next_token(p, '-' as i32);
        c2 = next_token(p, ',' as i32);
        if !c1.is_null() && (c2.is_null() || c1 < c2) {
            if next_num(c1, &mut end, &mut b) != 0 as libc::c_int {
                return;
            }
            c1 = if !end.is_null() && *end as libc::c_int != 0 {
                next_token(end, ':' as i32)
            } else {
                0 as *const libc::c_char
            };
            if !c1.is_null() && (c2.is_null() || c1 < c2) {
                if next_num(c1, &mut end, &mut s) != 0 as libc::c_int {
                    return;
                }
                if s == 0 as libc::c_int {
                    return;
                }
            }
        }
        if a > b {
            return;
        }
        while a <= b {
            let mut __cpu: size_t = a as size_t;
            if __cpu.wrapping_div(8 as libc::c_int as libc::c_ulong)
                < core::mem::size_of::<cpu_set_t>() as libc::c_ulong
            {
                let ref mut fresh0 = *(cpuset.__bits)
                    .as_mut_ptr()
                    .offset(
                        __cpu
                            .wrapping_div(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    );
                *fresh0
                    |= (1 as libc::c_int as __cpu_mask)
                        << __cpu
                            .wrapping_rem(
                                (8 as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        core::mem::size_of::<__cpu_mask>() as libc::c_ulong,
                                    ),
                            );
            } else {};
            a += s;
        }
    }
    if !end.is_null() && *end as libc::c_int != 0 {
        return;
    }
    sched_setaffinity(
        0 as libc::c_int,
        core::mem::size_of::<cpu_set_t>() as libc::c_ulong,
        &mut cpuset,
    );
}

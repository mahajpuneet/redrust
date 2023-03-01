extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type size_t = libc::c_ulong;
pub type uintptr_t = libc::c_ulong;
#[inline]
unsafe extern "C" fn swapfunc(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
    mut n: size_t,
    mut swaptype: libc::c_int,
) {
    if swaptype <= 1 as libc::c_int {
        let mut i: size_t = n
            .wrapping_div(core::mem::size_of::<libc::c_long>() as libc::c_ulong);
        let mut pi: *mut libc::c_long = a as *mut libc::c_void as *mut libc::c_long;
        let mut pj: *mut libc::c_long = b as *mut libc::c_void as *mut libc::c_long;
        loop {
            let mut t: libc::c_long = *pi;
            let fresh0 = pi;
            pi = pi.offset(1);
            *fresh0 = *pj;
            let fresh1 = pj;
            pj = pj.offset(1);
            *fresh1 = t;
            i = i.wrapping_sub(1);
            if !(i > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    } else {
        let mut i_0: size_t = n
            .wrapping_div(core::mem::size_of::<libc::c_char>() as libc::c_ulong);
        let mut pi_0: *mut libc::c_char = a as *mut libc::c_void as *mut libc::c_char;
        let mut pj_0: *mut libc::c_char = b as *mut libc::c_void as *mut libc::c_char;
        loop {
            let mut t_0: libc::c_char = *pi_0;
            let fresh2 = pi_0;
            pi_0 = pi_0.offset(1);
            *fresh2 = *pj_0;
            let fresh3 = pj_0;
            pj_0 = pj_0.offset(1);
            *fresh3 = t_0;
            i_0 = i_0.wrapping_sub(1);
            if !(i_0 > 0 as libc::c_int as libc::c_ulong) {
                break;
            }
        }
    };
}
#[inline]
unsafe extern "C" fn med3(
    mut a: *mut libc::c_char,
    mut b: *mut libc::c_char,
    mut c: *mut libc::c_char,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
) -> *mut libc::c_char {
    return if cmp
        .expect(
            "non-null function pointer",
        )(a as *const libc::c_void, b as *const libc::c_void) < 0 as libc::c_int
    {
        if cmp
            .expect(
                "non-null function pointer",
            )(b as *const libc::c_void, c as *const libc::c_void) < 0 as libc::c_int
        {
            b
        } else if cmp
            .expect(
                "non-null function pointer",
            )(a as *const libc::c_void, c as *const libc::c_void) < 0 as libc::c_int
        {
            c
        } else {
            a
        }
    } else if cmp
        .expect(
            "non-null function pointer",
        )(b as *const libc::c_void, c as *const libc::c_void) > 0 as libc::c_int
    {
        b
    } else if cmp
        .expect(
            "non-null function pointer",
        )(a as *const libc::c_void, c as *const libc::c_void) < 0 as libc::c_int
    {
        a
    } else {
        c
    };
}
unsafe extern "C" fn _pqsort(
    mut a: *mut libc::c_void,
    mut n: size_t,
    mut es: size_t,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut lrange: *mut libc::c_void,
    mut rrange: *mut libc::c_void,
) {
    let mut pa: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pb: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pc: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pd: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pl: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pm: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pn: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: size_t = 0;
    let mut r: size_t = 0;
    let mut swaptype: libc::c_int = 0;
    let mut cmp_result: libc::c_int = 0;
    loop {
        swaptype = if (a as uintptr_t)
            .wrapping_rem(core::mem::size_of::<libc::c_long>() as libc::c_ulong) != 0
            || es.wrapping_rem(core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                != 0
        {
            2 as libc::c_int
        } else if es == core::mem::size_of::<libc::c_long>() as libc::c_ulong {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
        if n < 7 as libc::c_int as libc::c_ulong {
            pm = (a as *mut libc::c_char).offset(es as isize);
            while pm < (a as *mut libc::c_char).offset(n.wrapping_mul(es) as isize) {
                pl = pm;
                while pl > a as *mut libc::c_char
                    && cmp
                        .expect(
                            "non-null function pointer",
                        )(
                        pl.offset(-(es as isize)) as *const libc::c_void,
                        pl as *const libc::c_void,
                    ) > 0 as libc::c_int
                {
                    if swaptype == 0 as libc::c_int {
                        let mut t: libc::c_long = *(pl as *mut libc::c_void
                            as *mut libc::c_long);
                        *(pl as *mut libc::c_void
                            as *mut libc::c_long) = *(pl.offset(-(es as isize))
                            as *mut libc::c_void as *mut libc::c_long);
                        *(pl.offset(-(es as isize)) as *mut libc::c_void
                            as *mut libc::c_long) = t;
                    } else {
                        swapfunc(pl, pl.offset(-(es as isize)), es, swaptype);
                    }
                    pl = pl.offset(-(es as isize));
                }
                pm = pm.offset(es as isize);
            }
            return;
        }
        pm = (a as *mut libc::c_char)
            .offset(
                n.wrapping_div(2 as libc::c_int as libc::c_ulong).wrapping_mul(es)
                    as isize,
            );
        if n > 7 as libc::c_int as libc::c_ulong {
            pl = a as *mut libc::c_char;
            pn = (a as *mut libc::c_char)
                .offset(
                    n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(es)
                        as isize,
                );
            if n > 40 as libc::c_int as libc::c_ulong {
                d = n.wrapping_div(8 as libc::c_int as libc::c_ulong).wrapping_mul(es);
                pl = med3(
                    pl,
                    pl.offset(d as isize),
                    pl
                        .offset(
                            (2 as libc::c_int as libc::c_ulong).wrapping_mul(d) as isize,
                        ),
                    cmp,
                );
                pm = med3(pm.offset(-(d as isize)), pm, pm.offset(d as isize), cmp);
                pn = med3(
                    pn
                        .offset(
                            -((2 as libc::c_int as libc::c_ulong).wrapping_mul(d)
                                as isize),
                        ),
                    pn.offset(-(d as isize)),
                    pn,
                    cmp,
                );
            }
            pm = med3(pl, pm, pn, cmp);
        }
        if swaptype == 0 as libc::c_int {
            let mut t_0: libc::c_long = *(a as *mut libc::c_long);
            *(a as *mut libc::c_long) = *(pm as *mut libc::c_void as *mut libc::c_long);
            *(pm as *mut libc::c_void as *mut libc::c_long) = t_0;
        } else {
            swapfunc(a as *mut libc::c_char, pm, es, swaptype);
        }
        pb = (a as *mut libc::c_char).offset(es as isize);
        pa = pb;
        pd = (a as *mut libc::c_char)
            .offset(
                n.wrapping_sub(1 as libc::c_int as libc::c_ulong).wrapping_mul(es)
                    as isize,
            );
        pc = pd;
        loop {
            while pb <= pc
                && {
                    cmp_result = cmp
                        .expect(
                            "non-null function pointer",
                        )(pb as *const libc::c_void, a);
                    cmp_result <= 0 as libc::c_int
                }
            {
                if cmp_result == 0 as libc::c_int {
                    if swaptype == 0 as libc::c_int {
                        let mut t_1: libc::c_long = *(pa as *mut libc::c_void
                            as *mut libc::c_long);
                        *(pa as *mut libc::c_void
                            as *mut libc::c_long) = *(pb as *mut libc::c_void
                            as *mut libc::c_long);
                        *(pb as *mut libc::c_void as *mut libc::c_long) = t_1;
                    } else {
                        swapfunc(pa, pb, es, swaptype);
                    }
                    pa = pa.offset(es as isize);
                }
                pb = pb.offset(es as isize);
            }
            while pb <= pc
                && {
                    cmp_result = cmp
                        .expect(
                            "non-null function pointer",
                        )(pc as *const libc::c_void, a);
                    cmp_result >= 0 as libc::c_int
                }
            {
                if cmp_result == 0 as libc::c_int {
                    if swaptype == 0 as libc::c_int {
                        let mut t_2: libc::c_long = *(pc as *mut libc::c_void
                            as *mut libc::c_long);
                        *(pc as *mut libc::c_void
                            as *mut libc::c_long) = *(pd as *mut libc::c_void
                            as *mut libc::c_long);
                        *(pd as *mut libc::c_void as *mut libc::c_long) = t_2;
                    } else {
                        swapfunc(pc, pd, es, swaptype);
                    }
                    pd = pd.offset(-(es as isize));
                }
                pc = pc.offset(-(es as isize));
            }
            if pb > pc {
                break;
            }
            if swaptype == 0 as libc::c_int {
                let mut t_3: libc::c_long = *(pb as *mut libc::c_void
                    as *mut libc::c_long);
                *(pb as *mut libc::c_void
                    as *mut libc::c_long) = *(pc as *mut libc::c_void
                    as *mut libc::c_long);
                *(pc as *mut libc::c_void as *mut libc::c_long) = t_3;
            } else {
                swapfunc(pb, pc, es, swaptype);
            }
            pb = pb.offset(es as isize);
            pc = pc.offset(-(es as isize));
        }
        pn = (a as *mut libc::c_char).offset(n.wrapping_mul(es) as isize);
        r = (if (pa.offset_from(a as *mut libc::c_char) as libc::c_long)
            < pb.offset_from(pa) as libc::c_long
        {
            pa.offset_from(a as *mut libc::c_char) as libc::c_long
        } else {
            pb.offset_from(pa) as libc::c_long
        }) as size_t;
        if r > 0 as libc::c_int as libc::c_ulong {
            swapfunc(a as *mut libc::c_char, pb.offset(-(r as isize)), r, swaptype);
        }
        r = if (pd.offset_from(pc) as libc::c_long as size_t)
            < (pn.offset_from(pd) as libc::c_long as libc::c_ulong).wrapping_sub(es)
        {
            pd.offset_from(pc) as libc::c_long as size_t
        } else {
            (pn.offset_from(pd) as libc::c_long as libc::c_ulong).wrapping_sub(es)
        };
        if r > 0 as libc::c_int as libc::c_ulong {
            swapfunc(pb, pn.offset(-(r as isize)), r, swaptype);
        }
        r = pb.offset_from(pa) as libc::c_long as size_t;
        if r > es {
            let mut _l: *mut libc::c_void = a;
            let mut _r: *mut libc::c_void = (a as *mut libc::c_uchar)
                .offset(r as isize)
                .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void;
            if !(lrange < _l && rrange < _l || lrange > _r && rrange > _r) {
                _pqsort(a, r.wrapping_div(es), es, cmp, lrange, rrange);
            }
        }
        r = pd.offset_from(pc) as libc::c_long as size_t;
        if !(r > es) {
            break;
        }
        let mut _l_0: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut _r_0: *mut libc::c_void = 0 as *mut libc::c_void;
        a = pn.offset(-(r as isize)) as *mut libc::c_void;
        n = r.wrapping_div(es);
        _l_0 = a;
        _r_0 = (a as *mut libc::c_uchar)
            .offset(r as isize)
            .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void;
        if lrange < _l_0 && rrange < _l_0 || lrange > _r_0 && rrange > _r_0 {
            break;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn pqsort(
    mut a: *mut libc::c_void,
    mut n: size_t,
    mut es: size_t,
    mut cmp: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
    mut lrange: size_t,
    mut rrange: size_t,
) {
    _pqsort(
        a,
        n,
        es,
        cmp,
        (a as *mut libc::c_uchar).offset(lrange.wrapping_mul(es) as isize)
            as *mut libc::c_void,
        (a as *mut libc::c_uchar)
            .offset(
                rrange.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_mul(es)
                    as isize,
            )
            .offset(-(1 as libc::c_int as isize)) as *mut libc::c_void,
    );
}

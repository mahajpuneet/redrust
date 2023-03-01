extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
pub type size_t = libc::c_ulong;
pub type u8_0 = libc::c_uchar;
pub type LZF_HSLOT = *const u8_0;
pub type LZF_STATE = [LZF_HSLOT; 65536];
#[no_mangle]
pub unsafe extern "C" fn lzf_compress(
    in_data: *const libc::c_void,
    mut in_len: size_t,
    mut out_data: *mut libc::c_void,
    mut out_len: size_t,
) -> size_t {
    let mut htab: LZF_STATE = [0 as *const u8_0; 65536];
    let mut ip: *const u8_0 = in_data as *const u8_0;
    let mut op: *mut u8_0 = out_data as *mut u8_0;
    let mut in_end: *const u8_0 = ip.offset(in_len as isize);
    let mut out_end: *mut u8_0 = op.offset(out_len as isize);
    let mut ref_0: *const u8_0 = 0 as *const u8_0;
    let mut off: size_t = 0;
    let mut hval: libc::c_uint = 0;
    let mut lit: libc::c_int = 0;
    if in_len == 0 || out_len == 0 {
        return 0 as libc::c_int as size_t;
    }
    lit = 0 as libc::c_int;
    op = op.offset(1);
    hval = ((*ip.offset(0 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int
        | *ip.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
    while ip < in_end.offset(-(2 as libc::c_int as isize)) {
        let mut hslot: *mut LZF_HSLOT = 0 as *mut LZF_HSLOT;
        hval = hval << 8 as libc::c_int
            | *ip.offset(2 as libc::c_int as isize) as libc::c_uint;
        hslot = htab
            .as_mut_ptr()
            .offset(
                ((hval >> 3 as libc::c_int * 8 as libc::c_int - 16 as libc::c_int)
                    .wrapping_sub(hval.wrapping_mul(5 as libc::c_int as libc::c_uint))
                    & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                        as libc::c_uint) as isize,
            );
        ref_0 = if !(*hslot).is_null() {
            (*hslot).offset(0 as libc::c_int as isize)
        } else {
            0 as LZF_HSLOT
        };
        *hslot = ip.offset(-(0 as libc::c_int as isize));
        if 1 as libc::c_int != 0
            && {
                off = (ip.offset_from(ref_0) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as size_t;
                off < ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong
            } && ref_0 > in_data as *mut u8_0 as *const u8_0
            && *ref_0.offset(2 as libc::c_int as isize) as libc::c_int
                == *ip.offset(2 as libc::c_int as isize) as libc::c_int
            && (*ref_0.offset(1 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int
                | *ref_0.offset(0 as libc::c_int as isize) as libc::c_int
                == (*ip.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int
                    | *ip.offset(0 as libc::c_int as isize) as libc::c_int
        {
            let mut len: libc::c_uint = 2 as libc::c_int as libc::c_uint;
            let mut maxlen: size_t = (in_end.offset_from(ip) as libc::c_long
                - len as libc::c_long) as size_t;
            maxlen = if maxlen
                > (((1 as libc::c_int) << 8 as libc::c_int)
                    + ((1 as libc::c_int) << 3 as libc::c_int)) as libc::c_ulong
            {
                (((1 as libc::c_int) << 8 as libc::c_int)
                    + ((1 as libc::c_int) << 3 as libc::c_int)) as libc::c_ulong
            } else {
                maxlen
            };
            if ((op.offset(3 as libc::c_int as isize).offset(1 as libc::c_int as isize)
                >= out_end) as libc::c_int != 0 as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                if op
                    .offset(-((lit == 0) as libc::c_int as isize))
                    .offset(3 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) >= out_end
                {
                    return 0 as libc::c_int as size_t;
                }
            }
            *op
                .offset(
                    (-lit - 1 as libc::c_int) as isize,
                ) = (lit - 1 as libc::c_int) as u8_0;
            op = op.offset(-((lit == 0) as libc::c_int as isize));
            let mut current_block_32: u64;
            if ((maxlen > 16 as libc::c_int as libc::c_ulong) as libc::c_int
                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                len = len.wrapping_add(1);
                if *ref_0.offset(len as isize) as libc::c_int
                    != *ip.offset(len as isize) as libc::c_int
                {
                    current_block_32 = 1345366029464561491;
                } else {
                    len = len.wrapping_add(1);
                    if *ref_0.offset(len as isize) as libc::c_int
                        != *ip.offset(len as isize) as libc::c_int
                    {
                        current_block_32 = 1345366029464561491;
                    } else {
                        len = len.wrapping_add(1);
                        if *ref_0.offset(len as isize) as libc::c_int
                            != *ip.offset(len as isize) as libc::c_int
                        {
                            current_block_32 = 1345366029464561491;
                        } else {
                            len = len.wrapping_add(1);
                            if *ref_0.offset(len as isize) as libc::c_int
                                != *ip.offset(len as isize) as libc::c_int
                            {
                                current_block_32 = 1345366029464561491;
                            } else {
                                len = len.wrapping_add(1);
                                if *ref_0.offset(len as isize) as libc::c_int
                                    != *ip.offset(len as isize) as libc::c_int
                                {
                                    current_block_32 = 1345366029464561491;
                                } else {
                                    len = len.wrapping_add(1);
                                    if *ref_0.offset(len as isize) as libc::c_int
                                        != *ip.offset(len as isize) as libc::c_int
                                    {
                                        current_block_32 = 1345366029464561491;
                                    } else {
                                        len = len.wrapping_add(1);
                                        if *ref_0.offset(len as isize) as libc::c_int
                                            != *ip.offset(len as isize) as libc::c_int
                                        {
                                            current_block_32 = 1345366029464561491;
                                        } else {
                                            len = len.wrapping_add(1);
                                            if *ref_0.offset(len as isize) as libc::c_int
                                                != *ip.offset(len as isize) as libc::c_int
                                            {
                                                current_block_32 = 1345366029464561491;
                                            } else {
                                                len = len.wrapping_add(1);
                                                if *ref_0.offset(len as isize) as libc::c_int
                                                    != *ip.offset(len as isize) as libc::c_int
                                                {
                                                    current_block_32 = 1345366029464561491;
                                                } else {
                                                    len = len.wrapping_add(1);
                                                    if *ref_0.offset(len as isize) as libc::c_int
                                                        != *ip.offset(len as isize) as libc::c_int
                                                    {
                                                        current_block_32 = 1345366029464561491;
                                                    } else {
                                                        len = len.wrapping_add(1);
                                                        if *ref_0.offset(len as isize) as libc::c_int
                                                            != *ip.offset(len as isize) as libc::c_int
                                                        {
                                                            current_block_32 = 1345366029464561491;
                                                        } else {
                                                            len = len.wrapping_add(1);
                                                            if *ref_0.offset(len as isize) as libc::c_int
                                                                != *ip.offset(len as isize) as libc::c_int
                                                            {
                                                                current_block_32 = 1345366029464561491;
                                                            } else {
                                                                len = len.wrapping_add(1);
                                                                if *ref_0.offset(len as isize) as libc::c_int
                                                                    != *ip.offset(len as isize) as libc::c_int
                                                                {
                                                                    current_block_32 = 1345366029464561491;
                                                                } else {
                                                                    len = len.wrapping_add(1);
                                                                    if *ref_0.offset(len as isize) as libc::c_int
                                                                        != *ip.offset(len as isize) as libc::c_int
                                                                    {
                                                                        current_block_32 = 1345366029464561491;
                                                                    } else {
                                                                        len = len.wrapping_add(1);
                                                                        if *ref_0.offset(len as isize) as libc::c_int
                                                                            != *ip.offset(len as isize) as libc::c_int
                                                                        {
                                                                            current_block_32 = 1345366029464561491;
                                                                        } else {
                                                                            len = len.wrapping_add(1);
                                                                            if *ref_0.offset(len as isize) as libc::c_int
                                                                                != *ip.offset(len as isize) as libc::c_int
                                                                            {
                                                                                current_block_32 = 1345366029464561491;
                                                                            } else {
                                                                                current_block_32 = 3938820862080741272;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                current_block_32 = 3938820862080741272;
            }
            match current_block_32 {
                3938820862080741272 => {
                    loop {
                        len = len.wrapping_add(1);
                        if !((len as libc::c_ulong) < maxlen
                            && *ref_0.offset(len as isize) as libc::c_int
                                == *ip.offset(len as isize) as libc::c_int)
                        {
                            break;
                        }
                    }
                }
                _ => {}
            }
            len = len.wrapping_sub(2 as libc::c_int as libc::c_uint);
            ip = ip.offset(1);
            if len < 7 as libc::c_int as libc::c_uint {
                let fresh0 = op;
                op = op.offset(1);
                *fresh0 = (off >> 8 as libc::c_int)
                    .wrapping_add((len << 5 as libc::c_int) as libc::c_ulong) as u8_0;
            } else {
                let fresh1 = op;
                op = op.offset(1);
                *fresh1 = (off >> 8 as libc::c_int)
                    .wrapping_add(
                        ((7 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong,
                    ) as u8_0;
                let fresh2 = op;
                op = op.offset(1);
                *fresh2 = len.wrapping_sub(7 as libc::c_int as libc::c_uint) as u8_0;
            }
            let fresh3 = op;
            op = op.offset(1);
            *fresh3 = off as u8_0;
            lit = 0 as libc::c_int;
            op = op.offset(1);
            ip = ip.offset(len.wrapping_add(1 as libc::c_int as libc::c_uint) as isize);
            if ((ip >= in_end.offset(-(2 as libc::c_int as isize))) as libc::c_int
                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                break;
            }
            ip = ip.offset(-1);
            ip = ip.offset(-1);
            hval = ((*ip.offset(0 as libc::c_int as isize) as libc::c_int)
                << 8 as libc::c_int
                | *ip.offset(1 as libc::c_int as isize) as libc::c_int) as libc::c_uint;
            hval = hval << 8 as libc::c_int
                | *ip.offset(2 as libc::c_int as isize) as libc::c_uint;
            htab[((hval >> 3 as libc::c_int * 8 as libc::c_int - 16 as libc::c_int)
                .wrapping_sub(hval.wrapping_mul(5 as libc::c_int as libc::c_uint))
                & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as usize] = ip.offset(-(0 as libc::c_int as isize));
            ip = ip.offset(1);
            hval = hval << 8 as libc::c_int
                | *ip.offset(2 as libc::c_int as isize) as libc::c_uint;
            htab[((hval >> 3 as libc::c_int * 8 as libc::c_int - 16 as libc::c_int)
                .wrapping_sub(hval.wrapping_mul(5 as libc::c_int as libc::c_uint))
                & (((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_uint) as usize] = ip.offset(-(0 as libc::c_int as isize));
            ip = ip.offset(1);
        } else {
            if ((op >= out_end) as libc::c_int != 0 as libc::c_int) as libc::c_int
                as libc::c_long != 0
            {
                return 0 as libc::c_int as size_t;
            }
            lit += 1;
            let fresh4 = ip;
            ip = ip.offset(1);
            let fresh5 = op;
            op = op.offset(1);
            *fresh5 = *fresh4;
            if ((lit == (1 as libc::c_int) << 5 as libc::c_int) as libc::c_int
                != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
            {
                *op
                    .offset(
                        (-lit - 1 as libc::c_int) as isize,
                    ) = (lit - 1 as libc::c_int) as u8_0;
                lit = 0 as libc::c_int;
                op = op.offset(1);
            }
        }
    }
    if op.offset(3 as libc::c_int as isize) > out_end {
        return 0 as libc::c_int as size_t;
    }
    while ip < in_end {
        lit += 1;
        let fresh6 = ip;
        ip = ip.offset(1);
        let fresh7 = op;
        op = op.offset(1);
        *fresh7 = *fresh6;
        if ((lit == (1 as libc::c_int) << 5 as libc::c_int) as libc::c_int
            != 0 as libc::c_int) as libc::c_int as libc::c_long != 0
        {
            *op
                .offset(
                    (-lit - 1 as libc::c_int) as isize,
                ) = (lit - 1 as libc::c_int) as u8_0;
            lit = 0 as libc::c_int;
            op = op.offset(1);
        }
    }
    *op.offset((-lit - 1 as libc::c_int) as isize) = (lit - 1 as libc::c_int) as u8_0;
    op = op.offset(-((lit == 0) as libc::c_int as isize));
    return op.offset_from(out_data as *mut u8_0) as libc::c_long as size_t;
}

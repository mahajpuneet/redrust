extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type u8_0 = libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn lzf_decompress(
    in_data: *const libc::c_void,
    mut in_len: size_t,
    mut out_data: *mut libc::c_void,
    mut out_len: size_t,
) -> size_t {
    let mut ip: *const u8_0 = in_data as *const u8_0;
    let mut op: *mut u8_0 = out_data as *mut u8_0;
    let in_end: *const u8_0 = ip.offset(in_len as isize);
    let out_end: *mut u8_0 = op.offset(out_len as isize);
    while ip < in_end {
        let mut ctrl: libc::c_uint = 0;
        let fresh0 = ip;
        ip = ip.offset(1);
        ctrl = *fresh0 as libc::c_uint;
        if ctrl < ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_uint {
            ctrl = ctrl.wrapping_add(1);
            if op.offset(ctrl as isize) > out_end {
                *__errno_location() = 7 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            if ip.offset(ctrl as isize) > in_end {
                *__errno_location() = 22 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            let mut current_block_41: u64;
            match ctrl {
                32 => {
                    let fresh1 = ip;
                    ip = ip.offset(1);
                    let fresh2 = op;
                    op = op.offset(1);
                    *fresh2 = *fresh1;
                    current_block_41 = 9893107113231336900;
                }
                31 => {
                    current_block_41 = 9893107113231336900;
                }
                30 => {
                    current_block_41 = 10913566927878453365;
                }
                29 => {
                    current_block_41 = 10018706083970647872;
                }
                28 => {
                    current_block_41 = 12666025696559833130;
                }
                27 => {
                    current_block_41 = 3648220726014638535;
                }
                26 => {
                    current_block_41 = 12054568458949171421;
                }
                25 => {
                    current_block_41 = 3385372364107690969;
                }
                24 => {
                    current_block_41 = 2593431772108939085;
                }
                23 => {
                    current_block_41 = 17933916765467129433;
                }
                22 => {
                    current_block_41 = 3854013183152116978;
                }
                21 => {
                    current_block_41 = 3695487147870903148;
                }
                20 => {
                    current_block_41 = 14942224046047924118;
                }
                19 => {
                    current_block_41 = 14734027529408650000;
                }
                18 => {
                    current_block_41 = 827679805636257719;
                }
                17 => {
                    current_block_41 = 4141388246766083991;
                }
                16 => {
                    current_block_41 = 798959627835326726;
                }
                15 => {
                    current_block_41 = 15898800284258182807;
                }
                14 => {
                    current_block_41 = 10963457308450014548;
                }
                13 => {
                    current_block_41 = 7808269772932317163;
                }
                12 => {
                    current_block_41 = 8102615537097851273;
                }
                11 => {
                    current_block_41 = 5186551979340825479;
                }
                10 => {
                    current_block_41 = 14048511607678491294;
                }
                9 => {
                    current_block_41 = 9274865807845469680;
                }
                8 => {
                    current_block_41 = 5757468306152155825;
                }
                7 => {
                    current_block_41 = 14757060196771683570;
                }
                6 => {
                    current_block_41 = 14192883302262645059;
                }
                5 => {
                    current_block_41 = 5759164078325609283;
                }
                4 => {
                    current_block_41 = 14812745045700952910;
                }
                3 => {
                    current_block_41 = 16217998861127851865;
                }
                2 => {
                    current_block_41 = 16260555586171583951;
                }
                1 => {
                    current_block_41 = 10464855294165663398;
                }
                _ => {
                    current_block_41 = 2480299350034459858;
                }
            }
            match current_block_41 {
                9893107113231336900 => {
                    let fresh3 = ip;
                    ip = ip.offset(1);
                    let fresh4 = op;
                    op = op.offset(1);
                    *fresh4 = *fresh3;
                    current_block_41 = 10913566927878453365;
                }
                _ => {}
            }
            match current_block_41 {
                10913566927878453365 => {
                    let fresh5 = ip;
                    ip = ip.offset(1);
                    let fresh6 = op;
                    op = op.offset(1);
                    *fresh6 = *fresh5;
                    current_block_41 = 10018706083970647872;
                }
                _ => {}
            }
            match current_block_41 {
                10018706083970647872 => {
                    let fresh7 = ip;
                    ip = ip.offset(1);
                    let fresh8 = op;
                    op = op.offset(1);
                    *fresh8 = *fresh7;
                    current_block_41 = 12666025696559833130;
                }
                _ => {}
            }
            match current_block_41 {
                12666025696559833130 => {
                    let fresh9 = ip;
                    ip = ip.offset(1);
                    let fresh10 = op;
                    op = op.offset(1);
                    *fresh10 = *fresh9;
                    current_block_41 = 3648220726014638535;
                }
                _ => {}
            }
            match current_block_41 {
                3648220726014638535 => {
                    let fresh11 = ip;
                    ip = ip.offset(1);
                    let fresh12 = op;
                    op = op.offset(1);
                    *fresh12 = *fresh11;
                    current_block_41 = 12054568458949171421;
                }
                _ => {}
            }
            match current_block_41 {
                12054568458949171421 => {
                    let fresh13 = ip;
                    ip = ip.offset(1);
                    let fresh14 = op;
                    op = op.offset(1);
                    *fresh14 = *fresh13;
                    current_block_41 = 3385372364107690969;
                }
                _ => {}
            }
            match current_block_41 {
                3385372364107690969 => {
                    let fresh15 = ip;
                    ip = ip.offset(1);
                    let fresh16 = op;
                    op = op.offset(1);
                    *fresh16 = *fresh15;
                    current_block_41 = 2593431772108939085;
                }
                _ => {}
            }
            match current_block_41 {
                2593431772108939085 => {
                    let fresh17 = ip;
                    ip = ip.offset(1);
                    let fresh18 = op;
                    op = op.offset(1);
                    *fresh18 = *fresh17;
                    current_block_41 = 17933916765467129433;
                }
                _ => {}
            }
            match current_block_41 {
                17933916765467129433 => {
                    let fresh19 = ip;
                    ip = ip.offset(1);
                    let fresh20 = op;
                    op = op.offset(1);
                    *fresh20 = *fresh19;
                    current_block_41 = 3854013183152116978;
                }
                _ => {}
            }
            match current_block_41 {
                3854013183152116978 => {
                    let fresh21 = ip;
                    ip = ip.offset(1);
                    let fresh22 = op;
                    op = op.offset(1);
                    *fresh22 = *fresh21;
                    current_block_41 = 3695487147870903148;
                }
                _ => {}
            }
            match current_block_41 {
                3695487147870903148 => {
                    let fresh23 = ip;
                    ip = ip.offset(1);
                    let fresh24 = op;
                    op = op.offset(1);
                    *fresh24 = *fresh23;
                    current_block_41 = 14942224046047924118;
                }
                _ => {}
            }
            match current_block_41 {
                14942224046047924118 => {
                    let fresh25 = ip;
                    ip = ip.offset(1);
                    let fresh26 = op;
                    op = op.offset(1);
                    *fresh26 = *fresh25;
                    current_block_41 = 14734027529408650000;
                }
                _ => {}
            }
            match current_block_41 {
                14734027529408650000 => {
                    let fresh27 = ip;
                    ip = ip.offset(1);
                    let fresh28 = op;
                    op = op.offset(1);
                    *fresh28 = *fresh27;
                    current_block_41 = 827679805636257719;
                }
                _ => {}
            }
            match current_block_41 {
                827679805636257719 => {
                    let fresh29 = ip;
                    ip = ip.offset(1);
                    let fresh30 = op;
                    op = op.offset(1);
                    *fresh30 = *fresh29;
                    current_block_41 = 4141388246766083991;
                }
                _ => {}
            }
            match current_block_41 {
                4141388246766083991 => {
                    let fresh31 = ip;
                    ip = ip.offset(1);
                    let fresh32 = op;
                    op = op.offset(1);
                    *fresh32 = *fresh31;
                    current_block_41 = 798959627835326726;
                }
                _ => {}
            }
            match current_block_41 {
                798959627835326726 => {
                    let fresh33 = ip;
                    ip = ip.offset(1);
                    let fresh34 = op;
                    op = op.offset(1);
                    *fresh34 = *fresh33;
                    current_block_41 = 15898800284258182807;
                }
                _ => {}
            }
            match current_block_41 {
                15898800284258182807 => {
                    let fresh35 = ip;
                    ip = ip.offset(1);
                    let fresh36 = op;
                    op = op.offset(1);
                    *fresh36 = *fresh35;
                    current_block_41 = 10963457308450014548;
                }
                _ => {}
            }
            match current_block_41 {
                10963457308450014548 => {
                    let fresh37 = ip;
                    ip = ip.offset(1);
                    let fresh38 = op;
                    op = op.offset(1);
                    *fresh38 = *fresh37;
                    current_block_41 = 7808269772932317163;
                }
                _ => {}
            }
            match current_block_41 {
                7808269772932317163 => {
                    let fresh39 = ip;
                    ip = ip.offset(1);
                    let fresh40 = op;
                    op = op.offset(1);
                    *fresh40 = *fresh39;
                    current_block_41 = 8102615537097851273;
                }
                _ => {}
            }
            match current_block_41 {
                8102615537097851273 => {
                    let fresh41 = ip;
                    ip = ip.offset(1);
                    let fresh42 = op;
                    op = op.offset(1);
                    *fresh42 = *fresh41;
                    current_block_41 = 5186551979340825479;
                }
                _ => {}
            }
            match current_block_41 {
                5186551979340825479 => {
                    let fresh43 = ip;
                    ip = ip.offset(1);
                    let fresh44 = op;
                    op = op.offset(1);
                    *fresh44 = *fresh43;
                    current_block_41 = 14048511607678491294;
                }
                _ => {}
            }
            match current_block_41 {
                14048511607678491294 => {
                    let fresh45 = ip;
                    ip = ip.offset(1);
                    let fresh46 = op;
                    op = op.offset(1);
                    *fresh46 = *fresh45;
                    current_block_41 = 9274865807845469680;
                }
                _ => {}
            }
            match current_block_41 {
                9274865807845469680 => {
                    let fresh47 = ip;
                    ip = ip.offset(1);
                    let fresh48 = op;
                    op = op.offset(1);
                    *fresh48 = *fresh47;
                    current_block_41 = 5757468306152155825;
                }
                _ => {}
            }
            match current_block_41 {
                5757468306152155825 => {
                    let fresh49 = ip;
                    ip = ip.offset(1);
                    let fresh50 = op;
                    op = op.offset(1);
                    *fresh50 = *fresh49;
                    current_block_41 = 14757060196771683570;
                }
                _ => {}
            }
            match current_block_41 {
                14757060196771683570 => {
                    let fresh51 = ip;
                    ip = ip.offset(1);
                    let fresh52 = op;
                    op = op.offset(1);
                    *fresh52 = *fresh51;
                    current_block_41 = 14192883302262645059;
                }
                _ => {}
            }
            match current_block_41 {
                14192883302262645059 => {
                    let fresh53 = ip;
                    ip = ip.offset(1);
                    let fresh54 = op;
                    op = op.offset(1);
                    *fresh54 = *fresh53;
                    current_block_41 = 5759164078325609283;
                }
                _ => {}
            }
            match current_block_41 {
                5759164078325609283 => {
                    let fresh55 = ip;
                    ip = ip.offset(1);
                    let fresh56 = op;
                    op = op.offset(1);
                    *fresh56 = *fresh55;
                    current_block_41 = 14812745045700952910;
                }
                _ => {}
            }
            match current_block_41 {
                14812745045700952910 => {
                    let fresh57 = ip;
                    ip = ip.offset(1);
                    let fresh58 = op;
                    op = op.offset(1);
                    *fresh58 = *fresh57;
                    current_block_41 = 16217998861127851865;
                }
                _ => {}
            }
            match current_block_41 {
                16217998861127851865 => {
                    let fresh59 = ip;
                    ip = ip.offset(1);
                    let fresh60 = op;
                    op = op.offset(1);
                    *fresh60 = *fresh59;
                    current_block_41 = 16260555586171583951;
                }
                _ => {}
            }
            match current_block_41 {
                16260555586171583951 => {
                    let fresh61 = ip;
                    ip = ip.offset(1);
                    let fresh62 = op;
                    op = op.offset(1);
                    *fresh62 = *fresh61;
                    current_block_41 = 10464855294165663398;
                }
                _ => {}
            }
            match current_block_41 {
                10464855294165663398 => {
                    let fresh63 = ip;
                    ip = ip.offset(1);
                    let fresh64 = op;
                    op = op.offset(1);
                    *fresh64 = *fresh63;
                }
                _ => {}
            }
        } else {
            let mut len: libc::c_uint = ctrl >> 5 as libc::c_int;
            let mut ref_0: *mut u8_0 = op
                .offset(
                    -(((ctrl & 0x1f as libc::c_int as libc::c_uint) << 8 as libc::c_int)
                        as isize),
                )
                .offset(-(1 as libc::c_int as isize));
            if ip >= in_end {
                *__errno_location() = 22 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            if len == 7 as libc::c_int as libc::c_uint {
                let fresh65 = ip;
                ip = ip.offset(1);
                len = len.wrapping_add(*fresh65 as libc::c_uint);
                if ip >= in_end {
                    *__errno_location() = 22 as libc::c_int;
                    return 0 as libc::c_int as size_t;
                }
            }
            let fresh66 = ip;
            ip = ip.offset(1);
            ref_0 = ref_0.offset(-(*fresh66 as libc::c_int as isize));
            if op.offset(len as isize).offset(2 as libc::c_int as isize) > out_end {
                *__errno_location() = 7 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            if ref_0 < out_data as *mut u8_0 {
                *__errno_location() = 22 as libc::c_int;
                return 0 as libc::c_int as size_t;
            }
            let mut current_block_82: u64;
            match len {
                9 => {
                    let fresh69 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh70 = op;
                    op = op.offset(1);
                    *fresh70 = *fresh69;
                    current_block_82 = 14242518865140032052;
                }
                8 => {
                    current_block_82 = 14242518865140032052;
                }
                7 => {
                    current_block_82 = 12388749052780255754;
                }
                6 => {
                    current_block_82 = 3909662535640455022;
                }
                5 => {
                    current_block_82 = 8149943935827593142;
                }
                4 => {
                    current_block_82 = 6175455253125813810;
                }
                3 => {
                    current_block_82 = 16844938162898414895;
                }
                2 => {
                    current_block_82 = 12225724121032071869;
                }
                1 => {
                    current_block_82 = 874927311066059277;
                }
                0 => {
                    current_block_82 = 1907598856535803917;
                }
                _ => {
                    len = len.wrapping_add(2 as libc::c_int as libc::c_uint);
                    if op >= ref_0.offset(len as isize) {
                        memcpy(
                            op as *mut libc::c_void,
                            ref_0 as *const libc::c_void,
                            len as libc::c_ulong,
                        );
                        op = op.offset(len as isize);
                    } else {
                        loop {
                            let fresh67 = ref_0;
                            ref_0 = ref_0.offset(1);
                            let fresh68 = op;
                            op = op.offset(1);
                            *fresh68 = *fresh67;
                            len = len.wrapping_sub(1);
                            if !(len != 0) {
                                break;
                            }
                        }
                    }
                    current_block_82 = 5854763015135596753;
                }
            }
            match current_block_82 {
                14242518865140032052 => {
                    let fresh71 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh72 = op;
                    op = op.offset(1);
                    *fresh72 = *fresh71;
                    current_block_82 = 12388749052780255754;
                }
                _ => {}
            }
            match current_block_82 {
                12388749052780255754 => {
                    let fresh73 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh74 = op;
                    op = op.offset(1);
                    *fresh74 = *fresh73;
                    current_block_82 = 3909662535640455022;
                }
                _ => {}
            }
            match current_block_82 {
                3909662535640455022 => {
                    let fresh75 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh76 = op;
                    op = op.offset(1);
                    *fresh76 = *fresh75;
                    current_block_82 = 8149943935827593142;
                }
                _ => {}
            }
            match current_block_82 {
                8149943935827593142 => {
                    let fresh77 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh78 = op;
                    op = op.offset(1);
                    *fresh78 = *fresh77;
                    current_block_82 = 6175455253125813810;
                }
                _ => {}
            }
            match current_block_82 {
                6175455253125813810 => {
                    let fresh79 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh80 = op;
                    op = op.offset(1);
                    *fresh80 = *fresh79;
                    current_block_82 = 16844938162898414895;
                }
                _ => {}
            }
            match current_block_82 {
                16844938162898414895 => {
                    let fresh81 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh82 = op;
                    op = op.offset(1);
                    *fresh82 = *fresh81;
                    current_block_82 = 12225724121032071869;
                }
                _ => {}
            }
            match current_block_82 {
                12225724121032071869 => {
                    let fresh83 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh84 = op;
                    op = op.offset(1);
                    *fresh84 = *fresh83;
                    current_block_82 = 874927311066059277;
                }
                _ => {}
            }
            match current_block_82 {
                874927311066059277 => {
                    let fresh85 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh86 = op;
                    op = op.offset(1);
                    *fresh86 = *fresh85;
                    current_block_82 = 1907598856535803917;
                }
                _ => {}
            }
            match current_block_82 {
                1907598856535803917 => {
                    let fresh87 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh88 = op;
                    op = op.offset(1);
                    *fresh88 = *fresh87;
                    let fresh89 = ref_0;
                    ref_0 = ref_0.offset(1);
                    let fresh90 = op;
                    op = op.offset(1);
                    *fresh90 = *fresh89;
                }
                _ => {}
            }
        }
    }
    return op.offset_from(out_data as *mut u8_0) as libc::c_long as size_t;
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct winsize {
    pub ws_row: libc::c_ushort,
    pub ws_col: libc::c_ushort,
    pub ws_xpixel: libc::c_ushort,
    pub ws_ypixel: libc::c_ushort,
}
static mut ws: winsize = winsize {
    ws_row: 0,
    ws_col: 0,
    ws_xpixel: 0,
    ws_ypixel: 0,
};
#[no_mangle]
pub static mut progress_printed: size_t = 0;
#[no_mangle]
pub static mut progress_full: size_t = 0;
#[no_mangle]
pub unsafe extern "C" fn memtest_progress_start(
    mut title: *mut libc::c_char,
    mut pass: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    printf(b"\x1B[H\x1B[2J\0" as *const u8 as *const libc::c_char);
    j = 0 as libc::c_int;
    while j < ws.ws_col as libc::c_int * (ws.ws_row as libc::c_int - 2 as libc::c_int) {
        printf(b".\0" as *const u8 as *const libc::c_char);
        j += 1;
    }
    printf(
        b"Please keep the test running several minutes per GB of memory.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Also check http://www.memtest86.com/ and http://pyropus.ca/software/memtester/\0"
            as *const u8 as *const libc::c_char,
    );
    printf(b"\x1B[H\x1B[2K\0" as *const u8 as *const libc::c_char);
    printf(b"%s [%d]\n\0" as *const u8 as *const libc::c_char, title, pass);
    progress_printed = 0 as libc::c_int as size_t;
    progress_full = (ws.ws_col as size_t)
        .wrapping_mul((ws.ws_row as libc::c_int - 3 as libc::c_int) as libc::c_ulong);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn memtest_progress_end() {
    printf(b"\x1B[H\x1B[2J\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn memtest_progress_step(
    mut curr: size_t,
    mut size: size_t,
    mut c: libc::c_char,
) {
    let mut chars: size_t = (curr as libc::c_ulonglong)
        .wrapping_mul(progress_full as libc::c_ulonglong)
        .wrapping_div(size as libc::c_ulonglong) as size_t;
    let mut j: size_t = 0;
    j = 0 as libc::c_int as size_t;
    while j < chars.wrapping_sub(progress_printed) {
        printf(b"%c\0" as *const u8 as *const libc::c_char, c as libc::c_int);
        j = j.wrapping_add(1);
    }
    progress_printed = chars;
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn memtest_addressing(
    mut l: *mut libc::c_ulong,
    mut bytes: size_t,
    mut interactive: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_ulong = bytes
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong);
    let mut j: libc::c_ulong = 0;
    let mut p: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    p = l;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < words {
        *p = p as libc::c_ulong;
        p = p.offset(1);
        if j & 0xffff as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong && interactive != 0
        {
            memtest_progress_step(
                j,
                words.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                'A' as i32 as libc::c_char,
            );
        }
        j = j.wrapping_add(1);
    }
    p = l;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < words {
        if *p != p as libc::c_ulong {
            if interactive != 0 {
                printf(
                    b"\n*** MEMORY ADDRESSING ERROR: %p contains %lu\n\0" as *const u8
                        as *const libc::c_char,
                    p as *mut libc::c_void,
                    *p,
                );
                exit(1 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        if j & 0xffff as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong && interactive != 0
        {
            memtest_progress_step(
                j.wrapping_add(words),
                words.wrapping_mul(2 as libc::c_int as libc::c_ulong),
                'A' as i32 as libc::c_char,
            );
        }
        j = j.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn memtest_fill_random(
    mut l: *mut libc::c_ulong,
    mut bytes: size_t,
    mut interactive: libc::c_int,
) {
    let mut step: libc::c_ulong = (4096 as libc::c_int as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong);
    let mut words: libc::c_ulong = bytes
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut iwords: libc::c_ulong = words.wrapping_div(step);
    let mut off: libc::c_ulong = 0;
    let mut w: libc::c_ulong = 0;
    let mut l1: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut l2: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut rseed: uint64_t = 0xd13133de9afdb566 as libc::c_ulong;
    let mut rout: uint64_t = 0 as libc::c_int as uint64_t;
    if bytes & 4095 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(bytes & 4095) == 0\0" as *const u8 as *const libc::c_char,
            b"memtest.c\0" as *const u8 as *const libc::c_char,
            156 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 55],
                &[libc::c_char; 55],
            >(b"void memtest_fill_random(unsigned long *, size_t, int)\0"))
                .as_ptr(),
        );
    };
    off = 0 as libc::c_int as libc::c_ulong;
    while off < step {
        l1 = l.offset(off as isize);
        l2 = l1.offset(words as isize);
        w = 0 as libc::c_int as libc::c_ulong;
        while w < iwords {
            rseed ^= rseed >> 12 as libc::c_int;
            rseed ^= rseed << 25 as libc::c_int;
            rseed ^= rseed >> 27 as libc::c_int;
            rout = rseed.wrapping_mul(2685821657736338717 as libc::c_ulong);
            *l2 = rout;
            *l1 = *l2;
            l1 = l1.offset(step as isize);
            l2 = l2.offset(step as isize);
            if w & 0xffff as libc::c_int as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong && interactive != 0
            {
                memtest_progress_step(
                    w.wrapping_add(iwords.wrapping_mul(off)),
                    words,
                    'R' as i32 as libc::c_char,
                );
            }
            w = w.wrapping_add(1);
        }
        off = off.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn memtest_fill_value(
    mut l: *mut libc::c_ulong,
    mut bytes: size_t,
    mut v1: libc::c_ulong,
    mut v2: libc::c_ulong,
    mut sym: libc::c_char,
    mut interactive: libc::c_int,
) {
    let mut step: libc::c_ulong = (4096 as libc::c_int as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong);
    let mut words: libc::c_ulong = bytes
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut iwords: libc::c_ulong = words.wrapping_div(step);
    let mut off: libc::c_ulong = 0;
    let mut w: libc::c_ulong = 0;
    let mut l1: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut l2: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut v: libc::c_ulong = 0;
    if bytes & 4095 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(bytes & 4095) == 0\0" as *const u8 as *const libc::c_char,
            b"memtest.c\0" as *const u8 as *const libc::c_char,
            181 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 90],
                &[libc::c_char; 90],
            >(
                b"void memtest_fill_value(unsigned long *, size_t, unsigned long, unsigned long, char, int)\0",
            ))
                .as_ptr(),
        );
    };
    off = 0 as libc::c_int as libc::c_ulong;
    while off < step {
        l1 = l.offset(off as isize);
        l2 = l1.offset(words as isize);
        v = if off & 1 as libc::c_int as libc::c_ulong != 0 { v2 } else { v1 };
        w = 0 as libc::c_int as libc::c_ulong;
        while w < iwords {
            *l2 = v | v << 16 as libc::c_int | v << 32 as libc::c_int
                | v << 48 as libc::c_int;
            *l1 = *l2;
            l1 = l1.offset(step as isize);
            l2 = l2.offset(step as isize);
            if w & 0xffff as libc::c_int as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong && interactive != 0
            {
                memtest_progress_step(
                    w.wrapping_add(iwords.wrapping_mul(off)),
                    words,
                    sym,
                );
            }
            w = w.wrapping_add(1);
        }
        off = off.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn memtest_compare(
    mut l: *mut libc::c_ulong,
    mut bytes: size_t,
    mut interactive: libc::c_int,
) -> libc::c_int {
    let mut words: libc::c_ulong = bytes
        .wrapping_div(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let mut w: libc::c_ulong = 0;
    let mut l1: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut l2: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    if bytes & 4095 as libc::c_int as libc::c_ulong == 0 as libc::c_int as libc::c_ulong
    {} else {
        __assert_fail(
            b"(bytes & 4095) == 0\0" as *const u8 as *const libc::c_char,
            b"memtest.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"int memtest_compare(unsigned long *, size_t, int)\0"))
                .as_ptr(),
        );
    };
    l1 = l;
    l2 = l1.offset(words as isize);
    w = 0 as libc::c_int as libc::c_ulong;
    while w < words {
        if *l1 != *l2 {
            if interactive != 0 {
                printf(
                    b"\n*** MEMORY ERROR DETECTED: %p != %p (%lu vs %lu)\n\0"
                        as *const u8 as *const libc::c_char,
                    l1 as *mut libc::c_void,
                    l2 as *mut libc::c_void,
                    *l1,
                    *l2,
                );
                exit(1 as libc::c_int);
            }
            return 1 as libc::c_int;
        }
        l1 = l1.offset(1);
        l2 = l2.offset(1);
        if w & 0xffff as libc::c_int as libc::c_ulong
            == 0 as libc::c_int as libc::c_ulong && interactive != 0
        {
            memtest_progress_step(w, words, '=' as i32 as libc::c_char);
        }
        w = w.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn memtest_compare_times(
    mut m: *mut libc::c_ulong,
    mut bytes: size_t,
    mut pass: libc::c_int,
    mut times: libc::c_int,
    mut interactive: libc::c_int,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut errors: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < times {
        if interactive != 0 {
            memtest_progress_start(
                b"Compare\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pass,
            );
        }
        errors += memtest_compare(m, bytes, interactive);
        if interactive != 0 {
            memtest_progress_end();
        }
        j += 1;
    }
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn memtest_test(
    mut m: *mut libc::c_ulong,
    mut bytes: size_t,
    mut passes: libc::c_int,
    mut interactive: libc::c_int,
) -> libc::c_int {
    let mut pass: libc::c_int = 0 as libc::c_int;
    let mut errors: libc::c_int = 0 as libc::c_int;
    while pass != passes {
        pass += 1;
        if interactive != 0 {
            memtest_progress_start(
                b"Addressing test\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pass,
            );
        }
        errors += memtest_addressing(m, bytes, interactive);
        if interactive != 0 {
            memtest_progress_end();
        }
        if interactive != 0 {
            memtest_progress_start(
                b"Random fill\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pass,
            );
        }
        memtest_fill_random(m, bytes, interactive);
        if interactive != 0 {
            memtest_progress_end();
        }
        errors += memtest_compare_times(m, bytes, pass, 4 as libc::c_int, interactive);
        if interactive != 0 {
            memtest_progress_start(
                b"Solid fill\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                pass,
            );
        }
        memtest_fill_value(
            m,
            bytes,
            0 as libc::c_int as libc::c_ulong,
            -(1 as libc::c_int) as libc::c_ulong,
            'S' as i32 as libc::c_char,
            interactive,
        );
        if interactive != 0 {
            memtest_progress_end();
        }
        errors += memtest_compare_times(m, bytes, pass, 4 as libc::c_int, interactive);
        if interactive != 0 {
            memtest_progress_start(
                b"Checkerboard fill\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                pass,
            );
        }
        memtest_fill_value(
            m,
            bytes,
            0xaaaaaaaaaaaaaaaa as libc::c_ulong,
            0x5555555555555555 as libc::c_ulong,
            'C' as i32 as libc::c_char,
            interactive,
        );
        if interactive != 0 {
            memtest_progress_end();
        }
        errors += memtest_compare_times(m, bytes, pass, 4 as libc::c_int, interactive);
    }
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn memtest_preserving_test(
    mut m: *mut libc::c_ulong,
    mut bytes: size_t,
    mut passes: libc::c_int,
) -> libc::c_int {
    let mut backup: [libc::c_ulong; 131072] = [0; 131072];
    let mut p: *mut libc::c_ulong = m;
    let mut end: *mut libc::c_ulong = (m as *mut libc::c_uchar)
        .offset(
            bytes.wrapping_sub((1024 as libc::c_int * 8 as libc::c_int) as libc::c_ulong)
                as isize,
        ) as *mut libc::c_ulong;
    let mut left: size_t = bytes;
    let mut errors: libc::c_int = 0 as libc::c_int;
    if bytes & 4095 as libc::c_int as libc::c_ulong != 0 {
        return 0 as libc::c_int;
    }
    if bytes < (4096 as libc::c_int * 2 as libc::c_int) as libc::c_ulong {
        return 0 as libc::c_int;
    }
    while left != 0 {
        if left == 4096 as libc::c_int as libc::c_ulong {
            left = (left as libc::c_ulong)
                .wrapping_add(4096 as libc::c_int as libc::c_ulong) as size_t as size_t;
            p = p
                .offset(
                    -((4096 as libc::c_int as libc::c_ulong)
                        .wrapping_div(
                            core::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                        ) as isize),
                );
        }
        let mut pass: libc::c_int = 0 as libc::c_int;
        let mut len: size_t = if left
            > core::mem::size_of::<[libc::c_ulong; 131072]>() as libc::c_ulong
        {
            core::mem::size_of::<[libc::c_ulong; 131072]>() as libc::c_ulong
        } else {
            left
        };
        if len
            .wrapping_div(4096 as libc::c_int as libc::c_ulong)
            .wrapping_rem(2 as libc::c_int as libc::c_ulong) != 0
        {
            len = (len as libc::c_ulong)
                .wrapping_sub(4096 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        memcpy(backup.as_mut_ptr() as *mut libc::c_void, p as *const libc::c_void, len);
        while pass != passes {
            pass += 1;
            errors += memtest_addressing(p, len, 0 as libc::c_int);
            memtest_fill_random(p, len, 0 as libc::c_int);
            if bytes >= (1024 as libc::c_int * 8 as libc::c_int) as libc::c_ulong {
                memtest_compare_times(
                    m,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                memtest_compare_times(
                    end,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            errors
                += memtest_compare_times(
                    p,
                    len,
                    pass,
                    4 as libc::c_int,
                    0 as libc::c_int,
                );
            memtest_fill_value(
                p,
                len,
                0 as libc::c_int as libc::c_ulong,
                -(1 as libc::c_int) as libc::c_ulong,
                'S' as i32 as libc::c_char,
                0 as libc::c_int,
            );
            if bytes >= (1024 as libc::c_int * 8 as libc::c_int) as libc::c_ulong {
                memtest_compare_times(
                    m,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                memtest_compare_times(
                    end,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            errors
                += memtest_compare_times(
                    p,
                    len,
                    pass,
                    4 as libc::c_int,
                    0 as libc::c_int,
                );
            memtest_fill_value(
                p,
                len,
                0xaaaaaaaaaaaaaaaa as libc::c_ulong,
                0x5555555555555555 as libc::c_ulong,
                'C' as i32 as libc::c_char,
                0 as libc::c_int,
            );
            if bytes >= (1024 as libc::c_int * 8 as libc::c_int) as libc::c_ulong {
                memtest_compare_times(
                    m,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
                memtest_compare_times(
                    end,
                    (1024 as libc::c_int * 8 as libc::c_int) as size_t,
                    pass,
                    1 as libc::c_int,
                    0 as libc::c_int,
                );
            }
            errors
                += memtest_compare_times(
                    p,
                    len,
                    pass,
                    4 as libc::c_int,
                    0 as libc::c_int,
                );
        }
        memcpy(p as *mut libc::c_void, backup.as_mut_ptr() as *const libc::c_void, len);
        left = (left as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        p = p
            .offset(
                len
                    .wrapping_div(
                        core::mem::size_of::<libc::c_ulong>() as libc::c_ulong,
                    ) as isize,
            );
    }
    return errors;
}
#[no_mangle]
pub unsafe extern "C" fn memtest_alloc_and_test(
    mut megabytes: size_t,
    mut passes: libc::c_int,
) {
    let mut bytes: size_t = megabytes
        .wrapping_mul(1024 as libc::c_int as libc::c_ulong)
        .wrapping_mul(1024 as libc::c_int as libc::c_ulong);
    let mut m: *mut libc::c_ulong = malloc(bytes) as *mut libc::c_ulong;
    if m.is_null() {
        fprintf(
            stderr,
            b"Unable to allocate %zu megabytes: %s\0" as *const u8
                as *const libc::c_char,
            megabytes,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    memtest_test(m, bytes, passes, 1 as libc::c_int);
    free(m as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn memtest(mut megabytes: size_t, mut passes: libc::c_int) {
    if ioctl(
        1 as libc::c_int,
        0x5413 as libc::c_int as libc::c_ulong,
        &mut ws as *mut winsize,
    ) == -(1 as libc::c_int)
    {
        ws.ws_col = 80 as libc::c_int as libc::c_ushort;
        ws.ws_row = 20 as libc::c_int as libc::c_ushort;
    }
    memtest_alloc_and_test(megabytes, passes);
    printf(b"\nYour memory passed this test.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"Please if you are still in doubt use the following two tools:\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"1) memtest86: http://www.memtest86.com/\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"2) memtester: http://pyropus.ca/software/memtester/\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(0 as libc::c_int);
}

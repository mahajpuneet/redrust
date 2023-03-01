use std::cell::UnsafeCell;
extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn log(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sample {
    pub value: libc::c_double,
    pub label: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequence {
    pub length: libc::c_int,
    pub labels: libc::c_int,
    pub samples: *mut sample,
    pub min: libc::c_double,
    pub max: libc::c_double,
}
static mut charset: [libc::c_char; 4] = {
    let b = [b'_', b'-', b'`', b'\0'];
    let cell = UnsafeCell::new(b);
    unsafe { *core::mem::transmute::<&UnsafeCell<[u8; 4]>, &[libc::c_char; 4]>(&cell) }
};
static mut charset_fill: [libc::c_char; 4] = {
    let b = [b'_', b'o', b'#', b'\0'];
    let cell = UnsafeCell::new(b);
    unsafe { *core::mem::transmute::<&UnsafeCell<[u8; 4]>, & [libc::c_char; 4]>(&cell) }
};
/*static mut charset: [libc::c_char; 4] = unsafe {
    *core::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"_-`\0")
};
static mut charset_fill: [libc::c_char; 4] = unsafe {
    *core::mem::transmute::<&[u8; 4], &mut [libc::c_char; 4]>(b"_o#\0")
};*/
static mut charset_len: libc::c_int = 0;
static mut label_margin_top: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn createSparklineSequence() -> *mut sequence {
    let mut seq: *mut sequence = zmalloc(
        core::mem::size_of::<sequence>() as libc::c_ulong,
    ) as *mut sequence;
    (*seq).length = 0 as libc::c_int;
    (*seq).samples = 0 as *mut sample;
    return seq;
}
#[no_mangle]
pub unsafe extern "C" fn sparklineSequenceAddSample(
    mut seq: *mut sequence,
    mut value: libc::c_double,
    mut label: *mut libc::c_char,
) {
    label = if label.is_null()
        || *label.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
    {
        0 as *mut libc::c_char
    } else {
        zstrdup(label)
    };
    if (*seq).length == 0 as libc::c_int {
        (*seq).max = value;
        (*seq).min = (*seq).max;
    } else if value < (*seq).min {
        (*seq).min = value;
    } else if value > (*seq).max {
        (*seq).max = value;
    }
    (*seq)
        .samples = zrealloc(
        (*seq).samples as *mut libc::c_void,
        (core::mem::size_of::<sample>() as libc::c_ulong)
            .wrapping_mul(((*seq).length + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut sample;
    (*((*seq).samples).offset((*seq).length as isize)).value = value;
    let ref mut fresh0 = (*((*seq).samples).offset((*seq).length as isize)).label;
    *fresh0 = label;
    (*seq).length += 1;
    if !label.is_null() {
        (*seq).labels += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeSparklineSequence(mut seq: *mut sequence) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*seq).length {
        zfree((*((*seq).samples).offset(j as isize)).label as *mut libc::c_void);
        j += 1;
    }
    zfree((*seq).samples as *mut libc::c_void);
    zfree(seq as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sparklineRenderRange(
    mut output: sds,
    mut seq: *mut sequence,
    mut rows: libc::c_int,
    mut offset: libc::c_int,
    mut len: libc::c_int,
    mut flags: libc::c_int,
) -> sds {
    let mut j: libc::c_int = 0;
    let mut relmax: libc::c_double = (*seq).max - (*seq).min;
    let mut steps: libc::c_int = charset_len * rows;
    let mut row: libc::c_int = 0 as libc::c_int;
    let mut chars: *mut libc::c_char = zmalloc(len as size_t) as *mut libc::c_char;
    let mut loop_0: libc::c_int = 1 as libc::c_int;
    let mut opt_fill: libc::c_int = flags & 1 as libc::c_int;
    let mut opt_log: libc::c_int = flags & 2 as libc::c_int;
    if opt_log != 0 {
        relmax = log(relmax + 1 as libc::c_int as libc::c_double);
    } else if relmax == 0 as libc::c_int as libc::c_double {
        relmax = 1 as libc::c_int as libc::c_double;
    }
    while loop_0 != 0 {
        loop_0 = 0 as libc::c_int;
        memset(chars as *mut libc::c_void, ' ' as i32, len as libc::c_ulong);
        j = 0 as libc::c_int;
        while j < len {
            let mut s: *mut sample = &mut *((*seq).samples).offset((j + offset) as isize)
                as *mut sample;
            let mut relval: libc::c_double = (*s).value - (*seq).min;
            let mut step: libc::c_int = 0;
            if opt_log != 0 {
                relval = log(relval + 1 as libc::c_int as libc::c_double);
            }
            step = ((relval * steps as libc::c_double) as libc::c_int as libc::c_double
                / relmax) as libc::c_int;
            if step < 0 as libc::c_int {
                step = 0 as libc::c_int;
            }
            if step >= steps {
                step = steps - 1 as libc::c_int;
            }
            if row < rows {
                let mut charidx: libc::c_int = step
                    - (rows - row - 1 as libc::c_int) * charset_len;
                loop_0 = 1 as libc::c_int;
                if charidx >= 0 as libc::c_int && charidx < charset_len {
                    *chars
                        .offset(
                            j as isize,
                        ) = (if opt_fill != 0 {
                        charset_fill[charidx as usize] as libc::c_int
                    } else {
                        charset[charidx as usize] as libc::c_int
                    }) as libc::c_char;
                } else if opt_fill != 0 && charidx >= charset_len {
                    *chars.offset(j as isize) = '|' as i32 as libc::c_char;
                }
            } else if (*seq).labels != 0 && row - rows < label_margin_top {
                loop_0 = 1 as libc::c_int;
                break;
            } else if !((*s).label).is_null() {
                let mut label_len: libc::c_int = strlen((*s).label) as libc::c_int;
                let mut label_char: libc::c_int = row - rows - label_margin_top;
                if label_len > label_char {
                    loop_0 = 1 as libc::c_int;
                    *chars
                        .offset(j as isize) = *((*s).label).offset(label_char as isize);
                }
            }
            j += 1;
        }
        if loop_0 != 0 {
            row += 1;
            output = sdscatlen(output, chars as *const libc::c_void, len as size_t);
            output = sdscatlen(
                output,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    zfree(chars as *mut libc::c_void);
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn sparklineRender(
    mut output: sds,
    mut seq: *mut sequence,
    mut columns: libc::c_int,
    mut rows: libc::c_int,
    mut flags: libc::c_int,
) -> sds {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*seq).length {
        let mut sublen: libc::c_int = if (*seq).length - j < columns {
            (*seq).length - j
        } else {
            columns
        };
        if j != 0 as libc::c_int {
            output = sdscatlen(
                output,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        output = sparklineRenderRange(output, seq, rows, j, sublen, flags);
        j += columns;
    }
    return output;
}
unsafe extern "C" fn run_static_initializers() {
    charset_len = (core::mem::size_of::<[libc::c_char; 4]>() as libc::c_ulong)
        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

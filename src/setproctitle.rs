extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn setenv(
        __name: *const libc::c_char,
        __value: *const libc::c_char,
        __replace: libc::c_int,
    ) -> libc::c_int;
    fn clearenv() -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(__s: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    static mut program_invocation_name: *mut libc::c_char;
    static mut program_invocation_short_name: *mut libc::c_char;
    static mut environ: *mut *mut libc::c_char;
}
pub type __builtin_va_list = __va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list {
    pub __stack: *mut libc::c_void,
    pub __gr_top: *mut libc::c_void,
    pub __vr_top: *mut libc::c_void,
    pub __gr_offs: libc::c_int,
    pub __vr_offs: libc::c_int,
}
pub type size_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub arg0: *const libc::c_char,
    pub base: *mut libc::c_char,
    pub end: *mut libc::c_char,
    pub nul: *mut libc::c_char,
    pub reset: bool,
    pub error: libc::c_int,
}
static mut SPT: C2RustUnnamed = C2RustUnnamed {
    arg0: 0 as *const libc::c_char,
    base: 0 as *const libc::c_char as *mut libc::c_char,
    end: 0 as *const libc::c_char as *mut libc::c_char,
    nul: 0 as *const libc::c_char as *mut libc::c_char,
    reset: false,
    error: 0,
};
#[inline]
unsafe extern "C" fn spt_min(mut a: size_t, mut b: size_t) -> size_t {
    return if a < b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn spt_clearenv() -> libc::c_int {
    return clearenv();
}
unsafe extern "C" fn spt_copyenv(
    mut envc: libc::c_int,
    mut oldenv: *mut *mut libc::c_char,
) -> libc::c_int {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut libc::c_char;
    }
    let mut envcopy: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut eq: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut envsize: libc::c_int = 0;
    if environ != oldenv {
        return 0 as libc::c_int;
    }
    envsize = ((envc + 1 as libc::c_int) as libc::c_ulong)
        .wrapping_mul(core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
        as libc::c_int;
    envcopy = malloc(envsize as libc::c_ulong) as *mut *mut libc::c_char;
    if envcopy.is_null() {
        return 12 as libc::c_int;
    }
    memcpy(
        envcopy as *mut libc::c_void,
        oldenv as *const libc::c_void,
        envsize as libc::c_ulong,
    );
    error = spt_clearenv();
    if error != 0 {
        environ = oldenv;
        free(envcopy as *mut libc::c_void);
        return error;
    }
    i = 0 as libc::c_int;
    while !(*envcopy.offset(i as isize)).is_null() {
        eq = strchr(*envcopy.offset(i as isize), '=' as i32);
        if !eq.is_null() {
            *eq = '\0' as i32 as libc::c_char;
            error = if 0 as libc::c_int
                != setenv(
                    *envcopy.offset(i as isize),
                    eq.offset(1 as libc::c_int as isize),
                    1 as libc::c_int,
                )
            {
                *__errno_location()
            } else {
                0 as libc::c_int
            };
            *eq = '=' as i32 as libc::c_char;
            if error != 0 {
                environ = envcopy;
                return error;
            }
        }
        i += 1;
    }
    free(envcopy as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn spt_copyargs(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < argc || i >= argc && !(*argv.offset(i as isize)).is_null() {
        if !(*argv.offset(i as isize)).is_null() {
            tmp = strdup(*argv.offset(i as isize));
            if tmp.is_null() {
                return *__errno_location();
            }
            let ref mut fresh0 = *argv.offset(i as isize);
            *fresh0 = tmp;
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn spt_init(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) {
    let mut current_block: u64;
    let mut envp: *mut *mut libc::c_char = environ;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nul: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    let mut envc: libc::c_int = 0;
    base = *argv.offset(0 as libc::c_int as isize);
    if base.is_null() {
        return;
    }
    nul = &mut *base
        .offset(
            (strlen as unsafe extern "C" fn(*const libc::c_char) -> libc::c_ulong)(base)
                as isize,
        ) as *mut libc::c_char;
    end = nul.offset(1 as libc::c_int as isize);
    i = 0 as libc::c_int;
    while i < argc || i >= argc && !(*argv.offset(i as isize)).is_null() {
        if !((*argv.offset(i as isize)).is_null() || *argv.offset(i as isize) < end) {
            if end >= *argv.offset(i as isize)
                && end
                    <= (*argv.offset(i as isize))
                        .offset(strlen(*argv.offset(i as isize)) as isize)
            {
                end = (*argv.offset(i as isize))
                    .offset(strlen(*argv.offset(i as isize)) as isize)
                    .offset(1 as libc::c_int as isize);
            }
        }
        i += 1;
    }
    i = 0 as libc::c_int;
    while !(*envp.offset(i as isize)).is_null() {
        if !(*envp.offset(i as isize) < end) {
            if end >= *envp.offset(i as isize)
                && end
                    <= (*envp.offset(i as isize))
                        .offset(strlen(*envp.offset(i as isize)) as isize)
            {
                end = (*envp.offset(i as isize))
                    .offset(strlen(*envp.offset(i as isize)) as isize)
                    .offset(1 as libc::c_int as isize);
            }
        }
        i += 1;
    }
    envc = i;
    SPT.arg0 = strdup(*argv.offset(0 as libc::c_int as isize));
    if (SPT.arg0).is_null() {
        current_block = 7326912412152907586;
    } else {
        tmp = strdup(program_invocation_name);
        if tmp.is_null() {
            current_block = 7326912412152907586;
        } else {
            program_invocation_name = tmp;
            tmp = strdup(program_invocation_short_name);
            if tmp.is_null() {
                current_block = 7326912412152907586;
            } else {
                program_invocation_short_name = tmp;
                error = spt_copyenv(envc, envp);
                if error != 0 {
                    current_block = 4174966463740749920;
                } else {
                    error = spt_copyargs(argc, argv);
                    if error != 0 {
                        current_block = 4174966463740749920;
                    } else {
                        SPT.nul = nul;
                        SPT.base = base;
                        SPT.end = end;
                        return;
                    }
                }
            }
        }
    }
    match current_block {
        7326912412152907586 => {
            error = *__errno_location();
        }
        _ => {}
    }
    SPT.error = error;
}
#[no_mangle]
pub unsafe extern "C" fn setproctitle(mut fmt: *const libc::c_char, mut args: ...) {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut ap: core::ffi::VaListImpl;
    let mut nul: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut error: libc::c_int = 0;
    if (SPT.base).is_null() {
        return;
    }
    if !fmt.is_null() {
        ap = args.clone();
        len = vsnprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            fmt,
            ap.as_va_list(),
        );
    } else {
        len = snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%s\0" as *const u8 as *const libc::c_char,
            SPT.arg0,
        );
    }
    if len <= 0 as libc::c_int {
        error = *__errno_location();
        SPT.error = error;
        return;
    } else {
        if !SPT.reset {
            memset(
                SPT.base as *mut libc::c_void,
                0 as libc::c_int,
                (SPT.end).offset_from(SPT.base) as libc::c_long as libc::c_ulong,
            );
            SPT.reset = 1 as libc::c_int != 0;
        } else {
            memset(
                SPT.base as *mut libc::c_void,
                0 as libc::c_int,
                spt_min(
                    core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                    (SPT.end).offset_from(SPT.base) as libc::c_long as size_t,
                ),
            );
        }
        len = spt_min(
            len as size_t,
            (spt_min(
                core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                (SPT.end).offset_from(SPT.base) as libc::c_long as size_t,
            ))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
        memcpy(
            SPT.base as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            len as libc::c_ulong,
        );
        nul = &mut *(SPT.base).offset(len as isize) as *mut libc::c_char;
        if nul < SPT.nul {
            *SPT.nul = '.' as i32 as libc::c_char;
        } else if nul == SPT.nul
            && (&mut *nul.offset(1 as libc::c_int as isize) as *mut libc::c_char)
                < SPT.end
        {
            *SPT.nul = ' ' as i32 as libc::c_char;
            nul = nul.offset(1);
            *nul = '\0' as i32 as libc::c_char;
        }
        return;
    };
}

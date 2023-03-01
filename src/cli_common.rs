extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type redisAsyncContext;
    pub type sockaddr;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn hi_sdsclear(s: hisds);
    fn hi_sdsnewlen(init: *const libc::c_void, initlen: size_t) -> hisds;
    fn hi_sdsnew(init: *const libc::c_char) -> hisds;
    fn hi_sdsempty() -> hisds;
    fn hi_sdsfree(s: hisds);
    fn hi_sdscatlen(s: hisds, t: *const libc::c_void, len: size_t) -> hisds;
    fn hi_sdscatprintf(s: hisds, fmt: *const libc::c_char, _: ...) -> hisds;
    fn hi_sdsrange(s: hisds, start: ssize_t, end: ssize_t) -> libc::c_int;
    fn hi_sdsfreesplitres(tokens: *mut hisds, count: libc::c_int);
    fn hi_sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut hisds;
    fn hi_sds_malloc(size: size_t) -> *mut libc::c_void;
    fn hi_sds_free(ptr: *mut libc::c_void);
    fn redisBufferWrite(c: *mut redisContext, done: *mut libc::c_int) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReplyObjectFunctions {
    pub createString: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createArray: Option::<
        unsafe extern "C" fn(*const redisReadTask, size_t) -> *mut libc::c_void,
    >,
    pub createInteger: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_longlong) -> *mut libc::c_void,
    >,
    pub createDouble: Option::<
        unsafe extern "C" fn(
            *const redisReadTask,
            libc::c_double,
            *mut libc::c_char,
            size_t,
        ) -> *mut libc::c_void,
    >,
    pub createNil: Option::<
        unsafe extern "C" fn(*const redisReadTask) -> *mut libc::c_void,
    >,
    pub createBool: Option::<
        unsafe extern "C" fn(*const redisReadTask, libc::c_int) -> *mut libc::c_void,
    >,
    pub freeObject: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReader {
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub buf: *mut libc::c_char,
    pub pos: size_t,
    pub len: size_t,
    pub maxbuf: size_t,
    pub maxelements: libc::c_longlong,
    pub task: *mut *mut redisReadTask,
    pub tasks: libc::c_int,
    pub ridx: libc::c_int,
    pub reply: *mut libc::c_void,
    pub fn_0: *mut redisReplyObjectFunctions,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type hisds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct hisdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContext {
    pub funcs: *const redisContextFuncs,
    pub err: libc::c_int,
    pub errstr: [libc::c_char; 128],
    pub fd: redisFD,
    pub flags: libc::c_int,
    pub obuf: *mut libc::c_char,
    pub reader: *mut redisReader,
    pub connection_type: redisConnectionType,
    pub connect_timeout: *mut timeval,
    pub command_timeout: *mut timeval,
    pub tcp: C2RustUnnamed_0,
    pub unix_sock: C2RustUnnamed,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub privdata: *mut libc::c_void,
    pub free_privdata: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub privctx: *mut libc::c_void,
    pub push_cb: Option::<redisPushFn>,
}
pub type redisPushFn = unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type redisConnectionType = libc::c_uint;
pub const REDIS_CONN_USERFD: redisConnectionType = 2;
pub const REDIS_CONN_UNIX: redisConnectionType = 1;
pub const REDIS_CONN_TCP: redisConnectionType = 0;
pub type redisFD = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisContextFuncs {
    pub free_privctx: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub async_read: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub async_write: Option::<unsafe extern "C" fn(*mut redisAsyncContext) -> ()>,
    pub read: Option::<
        unsafe extern "C" fn(*mut redisContext, *mut libc::c_char, size_t) -> ssize_t,
    >,
    pub write: Option::<unsafe extern "C" fn(*mut redisContext) -> ssize_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliSSLconfig {
    pub sni: *mut libc::c_char,
    pub cacert: *mut libc::c_char,
    pub cacertdir: *mut libc::c_char,
    pub skip_cert_verify: libc::c_int,
    pub cert: *mut libc::c_char,
    pub key: *mut libc::c_char,
    pub ciphers: *mut libc::c_char,
    pub ciphersuites: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cliConnInfo {
    pub hostip: *mut libc::c_char,
    pub hostport: libc::c_int,
    pub input_dbnum: libc::c_int,
    pub auth: *mut libc::c_char,
    pub user: *mut libc::c_char,
}
pub const _ISdigit: C2RustUnnamed_1 = 2048;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_1 = 8;
pub const _ISpunct: C2RustUnnamed_1 = 4;
pub const _IScntrl: C2RustUnnamed_1 = 2;
pub const _ISblank: C2RustUnnamed_1 = 1;
pub const _ISgraph: C2RustUnnamed_1 = 32768;
pub const _ISprint: C2RustUnnamed_1 = 16384;
pub const _ISspace: C2RustUnnamed_1 = 8192;
pub const _ISxdigit: C2RustUnnamed_1 = 4096;
pub const _ISalpha: C2RustUnnamed_1 = 1024;
pub const _ISlower: C2RustUnnamed_1 = 512;
pub const _ISupper: C2RustUnnamed_1 = 256;
#[inline]
unsafe extern "C" fn hi_sdslen(s: hisds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<hisdshdr8>() as libc::c_ulong as isize))
                as *mut hisdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr16>() as libc::c_ulong as isize),
                ) as *mut hisdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr32>() as libc::c_ulong as isize),
                ) as *mut hisdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(
                    -(core::mem::size_of::<hisdshdr64>() as libc::c_ulong as isize),
                ) as *mut hisdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn cliSecureConnection(
    mut c: *mut redisContext,
    mut config: cliSSLconfig,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cliWriteConn(
    mut c: *mut redisContext,
    mut buf: *const libc::c_char,
    mut buf_len: size_t,
) -> ssize_t {
    let mut done: libc::c_int = 0 as libc::c_int;
    (*c).obuf = hi_sdscatlen((*c).obuf, buf as *const libc::c_void, buf_len);
    if redisBufferWrite(c, &mut done) == -(1 as libc::c_int) {
        if (*c).flags & 0x1 as libc::c_int == 0 {
            *__errno_location() = 11 as libc::c_int;
        }
        if hi_sdslen((*c).obuf) > buf_len {
            hi_sdsrange(
                (*c).obuf,
                0 as libc::c_int as ssize_t,
                buf_len.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_neg()
                    as ssize_t,
            );
        } else {
            hi_sdsclear((*c).obuf);
        }
        return -(1 as libc::c_int) as ssize_t;
    }
    if done != 0 {
        hi_sdsclear((*c).obuf);
        return buf_len as ssize_t;
    }
    if hi_sdslen((*c).obuf) > buf_len {
        hi_sdsrange(
            (*c).obuf,
            0 as libc::c_int as ssize_t,
            buf_len.wrapping_add(1 as libc::c_int as libc::c_ulong).wrapping_neg()
                as ssize_t,
        );
        return 0 as libc::c_int as ssize_t;
    }
    let mut left: size_t = hi_sdslen((*c).obuf);
    hi_sdsclear((*c).obuf);
    return buf_len.wrapping_sub(left) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn cliSecureInit() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readArgFromStdin() -> hisds {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut arg: hisds = hi_sdsempty();
    loop {
        let mut nread: libc::c_int = read(
            fileno(stdin),
            buf.as_mut_ptr() as *mut libc::c_void,
            1024 as libc::c_int as size_t,
        ) as libc::c_int;
        if nread == 0 as libc::c_int {
            break;
        }
        if nread == -(1 as libc::c_int) {
            perror(b"Reading from standard input\0" as *const u8 as *const libc::c_char);
            exit(1 as libc::c_int);
        }
        arg = hi_sdscatlen(
            arg,
            buf.as_mut_ptr() as *const libc::c_void,
            nread as size_t,
        );
    }
    return arg;
}
#[no_mangle]
pub unsafe extern "C" fn getSdsArrayFromArgv(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut quoted: libc::c_int,
) -> *mut hisds {
    let mut res: *mut hisds = hi_sds_malloc(
        (core::mem::size_of::<hisds>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong),
    ) as *mut hisds;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < argc {
        if quoted != 0 {
            let mut unquoted: hisds = unquoteCString(*argv.offset(j as isize));
            if unquoted.is_null() {
                loop {
                    j -= 1;
                    if !(j >= 0 as libc::c_int) {
                        break;
                    }
                    hi_sdsfree(*res.offset(j as isize));
                }
                hi_sds_free(res as *mut libc::c_void);
                return 0 as *mut hisds;
            }
            let ref mut fresh0 = *res.offset(j as isize);
            *fresh0 = unquoted;
        } else {
            let ref mut fresh1 = *res.offset(j as isize);
            *fresh1 = hi_sdsnew(*argv.offset(j as isize));
        }
        j += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn unquoteCString(mut str: *mut libc::c_char) -> hisds {
    let mut count: libc::c_int = 0;
    let mut unquoted: *mut hisds = hi_sdssplitargs(str, &mut count);
    let mut res: hisds = 0 as hisds;
    if !unquoted.is_null() && count == 1 as libc::c_int {
        res = *unquoted.offset(0 as libc::c_int as isize);
        let ref mut fresh2 = *unquoted.offset(0 as libc::c_int as isize);
        *fresh2 = 0 as hisds;
    }
    if !unquoted.is_null() {
        hi_sdsfreesplitres(unquoted, count);
    }
    return res;
}
unsafe extern "C" fn percentDecode(
    mut pe: *const libc::c_char,
    mut len: size_t,
) -> hisds {
    let mut end: *const libc::c_char = pe.offset(len as isize);
    let mut ret: hisds = hi_sdsempty();
    let mut curr: *const libc::c_char = pe;
    while curr < end {
        if *curr as libc::c_int == '%' as i32 {
            if (end.offset_from(curr) as libc::c_long) < 2 as libc::c_int as libc::c_long
            {
                fprintf(
                    stderr,
                    b"Incomplete URI encoding\n\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            let mut h: libc::c_char = ({
                let mut __res: libc::c_int = 0;
                if core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        curr = curr.offset(1);
                        let mut __c: libc::c_int = *curr as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        curr = curr.offset(1);
                        __res = tolower(*curr as libc::c_int);
                    }
                } else {
                    curr = curr.offset(1);
                    __res = *(*__ctype_tolower_loc())
                        .offset(*curr as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            let mut l: libc::c_char = ({
                let mut __res: libc::c_int = 0;
                if core::mem::size_of::<libc::c_char>() as libc::c_ulong
                    > 1 as libc::c_int as libc::c_ulong
                {
                    if 0 != 0 {
                        curr = curr.offset(1);
                        let mut __c: libc::c_int = *curr as libc::c_int;
                        __res = if __c < -(128 as libc::c_int)
                            || __c > 255 as libc::c_int
                        {
                            __c
                        } else {
                            *(*__ctype_tolower_loc()).offset(__c as isize)
                        };
                    } else {
                        curr = curr.offset(1);
                        __res = tolower(*curr as libc::c_int);
                    }
                } else {
                    curr = curr.offset(1);
                    __res = *(*__ctype_tolower_loc())
                        .offset(*curr as libc::c_int as isize);
                }
                __res
            }) as libc::c_char;
            if !(*(*__ctype_b_loc()).offset(h as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                || h as libc::c_int >= 'a' as i32 && h as libc::c_int <= 'f' as i32)
                || !(*(*__ctype_b_loc()).offset(l as libc::c_int as isize) as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                    || l as libc::c_int >= 'a' as i32 && l as libc::c_int <= 'f' as i32)
            {
                fprintf(
                    stderr,
                    b"Illegal character in URI encoding\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            let mut c: libc::c_char = (((if *(*__ctype_b_loc())
                .offset(h as libc::c_int as isize) as libc::c_int
                & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
            {
                h as libc::c_int - '0' as i32
            } else {
                h as libc::c_int - 'a' as i32 + 10 as libc::c_int
            }) << 4 as libc::c_int)
                + (if *(*__ctype_b_loc()).offset(l as libc::c_int as isize)
                    as libc::c_int
                    & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int != 0
                {
                    l as libc::c_int - '0' as i32
                } else {
                    l as libc::c_int - 'a' as i32 + 10 as libc::c_int
                })) as libc::c_char;
            ret = hi_sdscatlen(
                ret,
                &mut c as *mut libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            curr = curr.offset(1);
        } else {
            let fresh3 = curr;
            curr = curr.offset(1);
            ret = hi_sdscatlen(
                ret,
                fresh3 as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn parseRedisUri(
    mut uri: *const libc::c_char,
    mut tool_name: *const libc::c_char,
    mut connInfo: *mut cliConnInfo,
    mut tls_flag: *mut libc::c_int,
) {
    let mut scheme: *const libc::c_char = b"redis://\0" as *const u8
        as *const libc::c_char;
    let mut tlsscheme: *const libc::c_char = b"rediss://\0" as *const u8
        as *const libc::c_char;
    let mut curr: *const libc::c_char = uri;
    let mut end: *const libc::c_char = uri.offset(strlen(uri) as isize);
    let mut userinfo: *const libc::c_char = 0 as *const libc::c_char;
    let mut username: *const libc::c_char = 0 as *const libc::c_char;
    let mut port: *const libc::c_char = 0 as *const libc::c_char;
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    if strncasecmp(tlsscheme, curr, strlen(tlsscheme)) == 0 {
        fprintf(
            stderr,
            b"rediss:// is only supported when %s is compiled with OpenSSL\n\0"
                as *const u8 as *const libc::c_char,
            tool_name,
        );
        exit(1 as libc::c_int);
    } else {
        if strncasecmp(scheme, curr, strlen(scheme)) == 0 {
            curr = curr.offset(strlen(scheme) as isize);
        } else {
            fprintf(
                stderr,
                b"Invalid URI scheme\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
    }
    if curr == end {
        return;
    }
    userinfo = strchr(curr, '@' as i32);
    if !userinfo.is_null() {
        username = strchr(curr, ':' as i32);
        if !username.is_null() && username < userinfo {
            (*connInfo)
                .user = percentDecode(
                curr,
                username.offset_from(curr) as libc::c_long as size_t,
            );
            curr = username.offset(1 as libc::c_int as isize);
        }
        (*connInfo)
            .auth = percentDecode(
            curr,
            userinfo.offset_from(curr) as libc::c_long as size_t,
        );
        curr = userinfo.offset(1 as libc::c_int as isize);
    }
    if curr == end {
        return;
    }
    path = strchr(curr, '/' as i32);
    if *curr as libc::c_int != '/' as i32 {
        host = if !path.is_null() {
            path.offset(-(1 as libc::c_int as isize))
        } else {
            end
        };
        port = strchr(curr, ':' as i32);
        if !port.is_null() {
            (*connInfo).hostport = atoi(port.offset(1 as libc::c_int as isize));
            host = port.offset(-(1 as libc::c_int as isize));
        }
        hi_sdsfree((*connInfo).hostip);
        (*connInfo)
            .hostip = hi_sdsnewlen(
            curr as *const libc::c_void,
            (host.offset_from(curr) as libc::c_long + 1 as libc::c_int as libc::c_long)
                as size_t,
        );
    }
    curr = if !path.is_null() { path.offset(1 as libc::c_int as isize) } else { end };
    if curr == end {
        return;
    }
    (*connInfo).input_dbnum = atoi(curr);
}
#[no_mangle]
pub unsafe extern "C" fn freeCliConnInfo(mut connInfo: cliConnInfo) {
    if !(connInfo.hostip).is_null() {
        hi_sdsfree(connInfo.hostip);
    }
    if !(connInfo.auth).is_null() {
        hi_sdsfree(connInfo.auth);
    }
    if !(connInfo.user).is_null() {
        hi_sdsfree(connInfo.user);
    }
}
#[no_mangle]
pub unsafe extern "C" fn escapeJsonString(
    mut s: hisds,
    mut p: *const libc::c_char,
    mut len: size_t,
) -> hisds {
    s = hi_sdscatlen(
        s,
        b"\"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    loop {
        let fresh4 = len;
        len = len.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        match *p as libc::c_int {
            92 | 34 => {
                s = hi_sdscatprintf(
                    s,
                    b"\\%c\0" as *const u8 as *const libc::c_char,
                    *p as libc::c_int,
                );
            }
            10 => {
                s = hi_sdscatlen(
                    s,
                    b"\\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            12 => {
                s = hi_sdscatlen(
                    s,
                    b"\\f\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            13 => {
                s = hi_sdscatlen(
                    s,
                    b"\\r\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            9 => {
                s = hi_sdscatlen(
                    s,
                    b"\\t\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            8 => {
                s = hi_sdscatlen(
                    s,
                    b"\\b\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                );
            }
            _ => {
                s = hi_sdscatprintf(
                    s,
                    if *(p as *mut libc::c_uchar) as libc::c_int <= 0x1f as libc::c_int {
                        b"\\u%04x\0" as *const u8 as *const libc::c_char
                    } else {
                        b"%c\0" as *const u8 as *const libc::c_char
                    },
                    *p as libc::c_int,
                );
            }
        }
        p = p.offset(1);
    }
    return hi_sdscatlen(
        s,
        b"\"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
}

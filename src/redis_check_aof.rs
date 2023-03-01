extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn sdsfree(s: sds);
    fn __errno_location() -> *mut libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strncasecmp(
        __s1: *const libc::c_char,
        __s2: *const libc::c_char,
        __n: size_t,
    ) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn makePath(path: *mut libc::c_char, filename: *mut libc::c_char) -> sds;
    fn aofManifestFree(am: *mut aofManifest);
    fn redis_check_rdb_main(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        fp: *mut FILE,
    ) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn dirname(__path: *mut libc::c_char) -> *mut libc::c_char;
    fn aofLoadManifestFromFile(am_filepath: sds) -> *mut aofManifest;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type time_t = __time_t;
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
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listNode {
    pub prev: *mut listNode,
    pub next: *mut listNode,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listIter {
    pub next: *mut listNode,
    pub direction: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct list {
    pub head: *mut listNode,
    pub tail: *mut listNode,
    pub dup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub free: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub match_0: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    >,
    pub len: libc::c_ulong,
}
pub type aof_file_type = libc::c_uint;
pub const AOF_FILE_TYPE_INCR: aof_file_type = 105;
pub const AOF_FILE_TYPE_HIST: aof_file_type = 104;
pub const AOF_FILE_TYPE_BASE: aof_file_type = 98;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofInfo {
    pub file_name: sds,
    pub file_seq: libc::c_longlong,
    pub file_type: aof_file_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofManifest {
    pub base_aof_info: *mut aofInfo,
    pub incr_aof_list: *mut list,
    pub history_aof_list: *mut list,
    pub curr_base_file_seq: libc::c_longlong,
    pub curr_incr_file_seq: libc::c_longlong,
    pub dirty: libc::c_int,
}
pub const AOF_RDB_PREAMBLE: input_file_type = 1;
pub const AOF_RESP: input_file_type = 0;
pub const AOF_MULTI_PART: input_file_type = 2;
pub type input_file_type = libc::c_uint;
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
static mut error: [libc::c_char; 1044] = [0; 1044];
static mut epos: off_t = 0;
static mut line: libc::c_longlong = 1 as libc::c_int as libc::c_longlong;
static mut to_timestamp: time_t = 0 as libc::c_int as time_t;
#[no_mangle]
pub unsafe extern "C" fn consumeNewline(mut buf: *mut libc::c_char) -> libc::c_int {
    if strncmp(
        buf,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        let mut __buf: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            __buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"Expected \\r\\n, got: %02x%02x\0" as *const u8 as *const libc::c_char,
            *buf.offset(0 as libc::c_int as isize) as libc::c_int,
            *buf.offset(1 as libc::c_int as isize) as libc::c_int,
        );
        snprintf(
            error.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
            b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
            epos as libc::c_longlong,
            __buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    line += 1 as libc::c_int as libc::c_longlong;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readLong(
    mut fp: *mut FILE,
    mut prefix: libc::c_char,
    mut target: *mut libc::c_long,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    epos = ftello(fp);
    if (fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        return 0 as libc::c_int;
    }
    if buf[0 as libc::c_int as usize] as libc::c_int != prefix as libc::c_int {
        let mut __buf: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            __buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"Expected prefix '%c', got: '%c'\0" as *const u8 as *const libc::c_char,
            prefix as libc::c_int,
            buf[0 as libc::c_int as usize] as libc::c_int,
        );
        snprintf(
            error.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
            b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
            epos as libc::c_longlong,
            __buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    *target = strtol(
        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        &mut eptr,
        10 as libc::c_int,
    );
    return consumeNewline(eptr);
}
#[no_mangle]
pub unsafe extern "C" fn readBytes(
    mut fp: *mut FILE,
    mut target: *mut libc::c_char,
    mut length: libc::c_long,
) -> libc::c_int {
    let mut real: libc::c_long = 0;
    epos = ftello(fp);
    real = fread(
        target as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        length as libc::c_ulong,
        fp,
    ) as libc::c_long;
    if real != length {
        let mut __buf: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            __buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"Expected to read %ld bytes, got %ld bytes\0" as *const u8
                as *const libc::c_char,
            length,
            real,
        );
        snprintf(
            error.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
            b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
            epos as libc::c_longlong,
            __buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readString(
    mut fp: *mut FILE,
    mut target: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut len: libc::c_long = 0;
    *target = 0 as *mut libc::c_char;
    if readLong(fp, '$' as i32 as libc::c_char, &mut len) == 0 {
        return 0 as libc::c_int;
    }
    if len < 0 as libc::c_int as libc::c_long
        || len > 9223372036854775807 as libc::c_long - 2 as libc::c_int as libc::c_long
    {
        let mut __buf: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            __buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"Expected to read string of %ld bytes, which is not in the suitable range\0"
                as *const u8 as *const libc::c_char,
            len,
        );
        snprintf(
            error.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
            b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
            epos as libc::c_longlong,
            __buf.as_mut_ptr(),
        );
        return 0 as libc::c_int;
    }
    len += 2 as libc::c_int as libc::c_long;
    *target = zmalloc(len as size_t) as *mut libc::c_char;
    if readBytes(fp, *target, len) == 0 {
        zfree(*target as *mut libc::c_void);
        *target = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    if consumeNewline(
        (*target).offset(len as isize).offset(-(2 as libc::c_int as isize)),
    ) == 0
    {
        zfree(*target as *mut libc::c_void);
        *target = 0 as *mut libc::c_char;
        return 0 as libc::c_int;
    }
    *(*target)
        .offset(
            (len - 2 as libc::c_int as libc::c_long) as isize,
        ) = '\0' as i32 as libc::c_char;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readArgc(
    mut fp: *mut FILE,
    mut target: *mut libc::c_long,
) -> libc::c_int {
    return readLong(fp, '*' as i32 as libc::c_char, target);
}
#[no_mangle]
pub unsafe extern "C" fn processRESP(
    mut fp: *mut FILE,
    mut filename: *mut libc::c_char,
    mut out_multi: *mut libc::c_int,
) -> libc::c_int {
    let mut argc: libc::c_long = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    if readArgc(fp, &mut argc) == 0 {
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while (i as libc::c_long) < argc {
        if readString(fp, &mut str) == 0 {
            return 0 as libc::c_int;
        }
        if i == 0 as libc::c_int {
            if strcasecmp(str, b"multi\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                let fresh0 = *out_multi;
                *out_multi = *out_multi + 1;
                if fresh0 != 0 {
                    let mut __buf: [libc::c_char; 1024] = [0; 1024];
                    snprintf(
                        __buf.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"Unexpected MULTI in AOF %s\0" as *const u8
                            as *const libc::c_char,
                        filename,
                    );
                    snprintf(
                        error.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
                        b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
                        epos as libc::c_longlong,
                        __buf.as_mut_ptr(),
                    );
                    zfree(str as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
            } else if strcasecmp(str, b"exec\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
            {
                *out_multi -= 1;
                if *out_multi != 0 {
                    let mut __buf_0: [libc::c_char; 1024] = [0; 1024];
                    snprintf(
                        __buf_0.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                        b"Unexpected EXEC in AOF %s\0" as *const u8
                            as *const libc::c_char,
                        filename,
                    );
                    snprintf(
                        error.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
                        b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
                        epos as libc::c_longlong,
                        __buf_0.as_mut_ptr(),
                    );
                    zfree(str as *mut libc::c_void);
                    return 0 as libc::c_int;
                }
            }
        }
        zfree(str as *mut libc::c_void);
        i += 1;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn processAnnotations(
    mut fp: *mut FILE,
    mut filename: *mut libc::c_char,
    mut last_file: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    epos = ftello(fp);
    if (fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        printf(
            b"Failed to read annotations from AOF %s, aborting...\n\0" as *const u8
                as *const libc::c_char,
            filename,
        );
        exit(1 as libc::c_int);
    }
    if to_timestamp != 0
        && strncmp(
            buf.as_mut_ptr(),
            b"#TS:\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
    {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        let mut ts: time_t = strtol(
            buf.as_mut_ptr().offset(4 as libc::c_int as isize),
            &mut endptr,
            10 as libc::c_int,
        );
        if *__errno_location() != 0 as libc::c_int
            || *endptr as libc::c_int != '\r' as i32
        {
            printf(
                b"Invalid timestamp annotation\n\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if ts <= to_timestamp {
            return 1 as libc::c_int;
        }
        if epos == 0 as libc::c_int as libc::c_long {
            printf(
                b"AOF %s has nothing before timestamp %ld, aborting...\n\0" as *const u8
                    as *const libc::c_char,
                filename,
                to_timestamp,
            );
            exit(1 as libc::c_int);
        }
        if last_file == 0 {
            printf(
                b"Failed to truncate AOF %s to timestamp %ld to offset %ld because it is not the last file.\n\0"
                    as *const u8 as *const libc::c_char,
                filename,
                to_timestamp,
                epos,
            );
            printf(
                b"If you insist, please delete all files after this file according to the manifest file and delete the corresponding records in manifest file manually. Then re-run redis-check-aof.\n\0"
                    as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        if ftruncate(fileno(fp), epos) == -(1 as libc::c_int) {
            printf(
                b"Failed to truncate AOF %s to timestamp %ld\n\0" as *const u8
                    as *const libc::c_char,
                filename,
                to_timestamp,
            );
            exit(1 as libc::c_int);
        } else {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkSingleAof(
    mut aof_filename: *mut libc::c_char,
    mut aof_filepath: *mut libc::c_char,
    mut last_file: libc::c_int,
    mut fix: libc::c_int,
    mut preamble: libc::c_int,
) -> libc::c_int {
    let mut pos: off_t = 0 as libc::c_int as off_t;
    let mut diff: off_t = 0;
    let mut multi: libc::c_int = 0 as libc::c_int;
    let mut buf: [libc::c_char; 2] = [0; 2];
    let mut fp: *mut FILE = fopen(
        aof_filepath,
        b"r+\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        printf(
            b"Cannot open file %s: %s, aborting...\n\0" as *const u8
                as *const libc::c_char,
            aof_filepath,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        printf(
            b"Cannot stat file: %s, aborting...\n\0" as *const u8 as *const libc::c_char,
            aof_filename,
        );
        exit(1 as libc::c_int);
    }
    let mut size: off_t = sb.st_size;
    if size == 0 as libc::c_int as libc::c_long {
        return 1 as libc::c_int;
    }
    if preamble != 0 {
        let mut argv: [*mut libc::c_char; 2] = [0 as *mut libc::c_char, aof_filename];
        if redis_check_rdb_main(2 as libc::c_int, argv.as_mut_ptr(), fp)
            == -(1 as libc::c_int)
        {
            printf(
                b"RDB preamble of AOF file is not sane, aborting.\n\0" as *const u8
                    as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        } else {
            printf(
                b"RDB preamble is OK, proceeding with AOF tail...\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
    }
    loop {
        if multi == 0 {
            pos = ftello(fp);
        }
        if (fgets(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong as libc::c_int,
            fp,
        ))
            .is_null()
        {
            if feof(fp) != 0 {
                break;
            }
            printf(
                b"Failed to read from AOF %s, aborting...\n\0" as *const u8
                    as *const libc::c_char,
                aof_filename,
            );
            exit(1 as libc::c_int);
        } else {
            if fseek(fp, -(1 as libc::c_int) as libc::c_long, 1 as libc::c_int)
                == -(1 as libc::c_int)
            {
                printf(
                    b"Failed to fseek in AOF %s: %s\0" as *const u8
                        as *const libc::c_char,
                    aof_filename,
                    strerror(*__errno_location()),
                );
                exit(1 as libc::c_int);
            }
            if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                if processAnnotations(fp, aof_filepath, last_file) == 0 {
                    fclose(fp);
                    return 3 as libc::c_int;
                }
            } else if buf[0 as libc::c_int as usize] as libc::c_int == '*' as i32 {
                if processRESP(fp, aof_filepath, &mut multi) == 0 {
                    break;
                }
            } else {
                printf(
                    b"AOF %s format error\n\0" as *const u8 as *const libc::c_char,
                    aof_filename,
                );
                break;
            }
        }
    }
    if feof(fp) != 0 && multi != 0
        && strlen(error.as_mut_ptr()) == 0 as libc::c_int as libc::c_ulong
    {
        let mut __buf: [libc::c_char; 1024] = [0; 1024];
        snprintf(
            __buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"Reached EOF before reading EXEC for MULTI\0" as *const u8
                as *const libc::c_char,
        );
        snprintf(
            error.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1044]>() as libc::c_ulong,
            b"0x%16llx: %s\0" as *const u8 as *const libc::c_char,
            epos as libc::c_longlong,
            __buf.as_mut_ptr(),
        );
    }
    if strlen(error.as_mut_ptr()) > 0 as libc::c_int as libc::c_ulong {
        printf(b"%s\n\0" as *const u8 as *const libc::c_char, error.as_mut_ptr());
    }
    diff = size - pos;
    if diff == 0 as libc::c_int as libc::c_long && to_timestamp != 0 {
        printf(
            b"Truncate nothing in AOF %s to timestamp %ld\n\0" as *const u8
                as *const libc::c_char,
            aof_filename,
            to_timestamp,
        );
        fclose(fp);
        return 0 as libc::c_int;
    }
    printf(
        b"AOF analyzed: filename=%s, size=%lld, ok_up_to=%lld, ok_up_to_line=%lld, diff=%lld\n\0"
            as *const u8 as *const libc::c_char,
        aof_filename,
        size as libc::c_longlong,
        pos as libc::c_longlong,
        line,
        diff as libc::c_longlong,
    );
    if diff > 0 as libc::c_int as libc::c_long {
        if fix != 0 {
            if last_file == 0 {
                printf(
                    b"Failed to truncate AOF %s because it is not the last file\n\0"
                        as *const u8 as *const libc::c_char,
                    aof_filename,
                );
                exit(1 as libc::c_int);
            }
            let mut buf_0: [libc::c_char; 2] = [0; 2];
            printf(
                b"This will shrink the AOF %s from %lld bytes, with %lld bytes, to %lld bytes\n\0"
                    as *const u8 as *const libc::c_char,
                aof_filename,
                size as libc::c_longlong,
                diff as libc::c_longlong,
                pos as libc::c_longlong,
            );
            printf(b"Continue? [y/N]: \0" as *const u8 as *const libc::c_char);
            if (fgets(
                buf_0.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong
                    as libc::c_int,
                stdin,
            ))
                .is_null()
                || strncasecmp(
                    buf_0.as_mut_ptr(),
                    b"y\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int as size_t,
                ) != 0 as libc::c_int
            {
                printf(b"Aborting...\n\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
            if ftruncate(fileno(fp), pos) == -(1 as libc::c_int) {
                printf(
                    b"Failed to truncate AOF %s\n\0" as *const u8 as *const libc::c_char,
                    aof_filename,
                );
                exit(1 as libc::c_int);
            } else {
                fclose(fp);
                return 2 as libc::c_int;
            }
        } else {
            printf(
                b"AOF %s is not valid. Use the --fix option to try fixing it.\n\0"
                    as *const u8 as *const libc::c_char,
                aof_filename,
            );
            exit(1 as libc::c_int);
        }
    }
    fclose(fp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fileIsRDB(mut filepath: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut FILE = fopen(filepath, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        printf(
            b"Cannot open file %s: %s\n\0" as *const u8 as *const libc::c_char,
            filepath,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        printf(
            b"Cannot stat file: %s\n\0" as *const u8 as *const libc::c_char,
            filepath,
        );
        exit(1 as libc::c_int);
    }
    let mut size: off_t = sb.st_size;
    if size == 0 as libc::c_int as libc::c_long {
        fclose(fp);
        return 0 as libc::c_int;
    }
    if size >= 8 as libc::c_int as libc::c_long {
        let mut sig: [libc::c_char; 5] = [0; 5];
        let mut rdb_file: libc::c_int = (fread(
            sig.as_mut_ptr() as *mut libc::c_void,
            core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fp,
        ) == 1 as libc::c_int as libc::c_ulong
            && memcmp(
                sig.as_mut_ptr() as *const libc::c_void,
                b"REDIS\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong,
            ) == 0 as libc::c_int) as libc::c_int;
        if rdb_file != 0 {
            fclose(fp);
            return 1 as libc::c_int;
        }
    }
    fclose(fp);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fileIsManifest(mut filepath: *mut libc::c_char) -> libc::c_int {
    let mut is_manifest: libc::c_int = 0 as libc::c_int;
    let mut fp: *mut FILE = fopen(filepath, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        printf(
            b"Cannot open file %s: %s\n\0" as *const u8 as *const libc::c_char,
            filepath,
            strerror(*__errno_location()),
        );
        exit(1 as libc::c_int);
    }
    let mut sb: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        printf(
            b"Cannot stat file: %s\n\0" as *const u8 as *const libc::c_char,
            filepath,
        );
        exit(1 as libc::c_int);
    }
    let mut size: off_t = sb.st_size;
    if size == 0 as libc::c_int as libc::c_long {
        fclose(fp);
        return 0 as libc::c_int;
    }
    let mut buf: [libc::c_char; 1025] = [0; 1025];
    loop {
        if (fgets(buf.as_mut_ptr(), 1024 as libc::c_int + 1 as libc::c_int, fp))
            .is_null()
        {
            if feof(fp) != 0 {
                break;
            }
            printf(
                b"Cannot read file: %s\n\0" as *const u8 as *const libc::c_char,
                filepath,
            );
            exit(1 as libc::c_int);
        } else {
            if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                continue;
            }
            if memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                b"file\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                strlen(b"file\0" as *const u8 as *const libc::c_char),
            ) == 0
            {
                is_manifest = 1 as libc::c_int;
            }
        }
    }
    fclose(fp);
    return is_manifest;
}
#[no_mangle]
pub unsafe extern "C" fn getInputFileType(
    mut filepath: *mut libc::c_char,
) -> input_file_type {
    if fileIsManifest(filepath) != 0 {
        return AOF_MULTI_PART
    } else if fileIsRDB(filepath) != 0 {
        return AOF_RDB_PREAMBLE
    } else {
        return AOF_RESP
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkMultiPartAof(
    mut dirpath: *mut libc::c_char,
    mut manifest_filepath: *mut libc::c_char,
    mut fix: libc::c_int,
) {
    let mut total_num: libc::c_int = 0 as libc::c_int;
    let mut aof_num: libc::c_int = 0 as libc::c_int;
    let mut last_file: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    printf(b"Start checking Multi Part AOF\n\0" as *const u8 as *const libc::c_char);
    let mut am: *mut aofManifest = aofLoadManifestFromFile(manifest_filepath);
    if !((*am).base_aof_info).is_null() {
        total_num += 1;
    }
    if !((*am).incr_aof_list).is_null() {
        total_num = (total_num as libc::c_ulong).wrapping_add((*(*am).incr_aof_list).len)
            as libc::c_int as libc::c_int;
    }
    if !((*am).base_aof_info).is_null() {
        let mut aof_filename: sds = (*(*am).base_aof_info).file_name;
        let mut aof_filepath: sds = makePath(dirpath, aof_filename);
        aof_num += 1;
        last_file = (aof_num == total_num) as libc::c_int;
        let mut aof_preable: libc::c_int = fileIsRDB(aof_filepath);
        printf(
            b"Start to check BASE AOF (%s format).\n\0" as *const u8
                as *const libc::c_char,
            if aof_preable != 0 {
                b"RDB\0" as *const u8 as *const libc::c_char
            } else {
                b"RESP\0" as *const u8 as *const libc::c_char
            },
        );
        ret = checkSingleAof(aof_filename, aof_filepath, last_file, fix, aof_preable);
        if ret == 0 as libc::c_int {
            printf(
                b"BASE AOF %s is valid\n\0" as *const u8 as *const libc::c_char,
                aof_filename,
            );
        } else if ret == 1 as libc::c_int {
            printf(
                b"BASE AOF %s is empty\n\0" as *const u8 as *const libc::c_char,
                aof_filename,
            );
        } else if ret == 3 as libc::c_int {
            printf(
                b"Successfully truncated AOF %s to timestamp %ld\n\0" as *const u8
                    as *const libc::c_char,
                aof_filename,
                to_timestamp,
            );
        } else if ret == 2 as libc::c_int {
            printf(
                b"Successfully truncated AOF %s\n\0" as *const u8 as *const libc::c_char,
                aof_filename,
            );
        }
        sdsfree(aof_filepath);
    }
    if (*(*am).incr_aof_list).len != 0 {
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        printf(b"Start to check INCR files.\n\0" as *const u8 as *const libc::c_char);
        listRewind((*am).incr_aof_list, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
            let mut aof_filename_0: sds = (*ai).file_name;
            let mut aof_filepath_0: sds = makePath(dirpath, aof_filename_0);
            aof_num += 1;
            last_file = (aof_num == total_num) as libc::c_int;
            ret = checkSingleAof(
                aof_filename_0,
                aof_filepath_0,
                last_file,
                fix,
                0 as libc::c_int,
            );
            if ret == 0 as libc::c_int {
                printf(
                    b"INCR AOF %s is valid\n\0" as *const u8 as *const libc::c_char,
                    aof_filename_0,
                );
            } else if ret == 1 as libc::c_int {
                printf(
                    b"INCR AOF %s is empty\n\0" as *const u8 as *const libc::c_char,
                    aof_filename_0,
                );
            } else if ret == 3 as libc::c_int {
                printf(
                    b"Successfully truncated AOF %s to timestamp %ld\n\0" as *const u8
                        as *const libc::c_char,
                    aof_filename_0,
                    to_timestamp,
                );
            } else if ret == 2 as libc::c_int {
                printf(
                    b"Successfully truncated AOF %s\n\0" as *const u8
                        as *const libc::c_char,
                    aof_filename_0,
                );
            }
            sdsfree(aof_filepath_0);
        }
    }
    aofManifestFree(am);
    printf(
        b"All AOF files and manifest are valid\n\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn checkOldStyleAof(
    mut filepath: *mut libc::c_char,
    mut fix: libc::c_int,
    mut preamble: libc::c_int,
) {
    printf(b"Start checking Old-Style AOF\n\0" as *const u8 as *const libc::c_char);
    let mut ret: libc::c_int = checkSingleAof(
        filepath,
        filepath,
        1 as libc::c_int,
        fix,
        preamble,
    );
    if ret == 0 as libc::c_int {
        printf(b"AOF %s is valid\n\0" as *const u8 as *const libc::c_char, filepath);
    } else if ret == 1 as libc::c_int {
        printf(b"AOF %s is empty\n\0" as *const u8 as *const libc::c_char, filepath);
    } else if ret == 3 as libc::c_int {
        printf(
            b"Successfully truncated AOF %s to timestamp %ld\n\0" as *const u8
                as *const libc::c_char,
            filepath,
            to_timestamp,
        );
    } else if ret == 2 as libc::c_int {
        printf(
            b"Successfully truncated AOF %s\n\0" as *const u8 as *const libc::c_char,
            filepath,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn redis_check_aof_main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut type_0: input_file_type = AOF_RESP;
    let mut current_block: u64;
    let mut filepath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp_filepath: [libc::c_char; 4097] = [0; 4097];
    let mut dirpath: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fix: libc::c_int = 0 as libc::c_int;
    if !(argc < 2 as libc::c_int) {
        if argc == 2 as libc::c_int {
            filepath = *argv.offset(1 as libc::c_int as isize);
            current_block = 13797916685926291137;
        } else if argc == 3 as libc::c_int {
            if strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"--fix\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                filepath = *argv.offset(2 as libc::c_int as isize);
                fix = 1 as libc::c_int;
                current_block = 13797916685926291137;
            } else {
                current_block = 12445938659794055887;
            }
        } else if argc == 4 as libc::c_int {
            if strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"--truncate-to-timestamp\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
                *__errno_location() = 0 as libc::c_int;
                to_timestamp = strtol(
                    *argv.offset(2 as libc::c_int as isize),
                    &mut endptr,
                    10 as libc::c_int,
                );
                if *__errno_location() != 0 as libc::c_int
                    || *endptr as libc::c_int != '\0' as i32
                {
                    printf(
                        b"Invalid timestamp, aborting...\n\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                filepath = *argv.offset(3 as libc::c_int as isize);
                current_block = 13797916685926291137;
            } else {
                current_block = 12445938659794055887;
            }
        } else {
            current_block = 12445938659794055887;
        }
        match current_block {
            12445938659794055887 => {}
            _ => {
                memcpy(
                    temp_filepath.as_mut_ptr() as *mut libc::c_void,
                    filepath as *const libc::c_void,
                    (strlen(filepath)).wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                dirpath = dirname(temp_filepath.as_mut_ptr());
                type_0 = getInputFileType(filepath);
                match type_0 as libc::c_uint {
                    2 => {
                        checkMultiPartAof(dirpath, filepath, fix);
                    }
                    0 => {
                        checkOldStyleAof(filepath, fix, 0 as libc::c_int);
                    }
                    1 => {
                        checkOldStyleAof(filepath, fix, 1 as libc::c_int);
                    }
                    _ => {}
                }
                exit(0 as libc::c_int);
            }
        }
    }
    printf(
        b"Usage: %s [--fix|--truncate-to-timestamp $timestamp] <file.manifest|file.aof>\n\0"
            as *const u8 as *const libc::c_char,
        *argv.offset(0 as libc::c_int as isize),
    );
    exit(1 as libc::c_int);
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdsclear(s: sds);
    fn sdsMakeRoomFor(s: sds, addlen: size_t) -> sds;
    fn sdsIncrLen(s: sds, incr: ssize_t);
    fn __errno_location() -> *mut libc::c_int;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn crc64(crc: uint64_t, s: *const libc::c_uchar, l: uint64_t) -> uint64_t;
    fn sync_file_range(
        __fd: libc::c_int,
        __offset: __off64_t,
        __count: __off64_t,
        __flags: libc::c_uint,
    ) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr8 {
    pub len: uint8_t,
    pub alloc: uint8_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr16 {
    pub len: uint16_t,
    pub alloc: uint16_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr32 {
    pub len: uint32_t,
    pub alloc: uint32_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct sdshdr64 {
    pub len: uint64_t,
    pub alloc: uint64_t,
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeEventLoop {
    pub maxfd: libc::c_int,
    pub setsize: libc::c_int,
    pub timeEventNextId: libc::c_longlong,
    pub events: *mut aeFileEvent,
    pub fired: *mut aeFiredEvent,
    pub timeEventHead: *mut aeTimeEvent,
    pub stop: libc::c_int,
    pub apidata: *mut libc::c_void,
    pub beforesleep: Option::<aeBeforeSleepProc>,
    pub aftersleep: Option::<aeBeforeSleepProc>,
    pub flags: libc::c_int,
}
pub type aeBeforeSleepProc = unsafe extern "C" fn(*mut aeEventLoop) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeTimeEvent {
    pub id: libc::c_longlong,
    pub when: monotime,
    pub timeProc: Option::<aeTimeProc>,
    pub finalizerProc: Option::<aeEventFinalizerProc>,
    pub clientData: *mut libc::c_void,
    pub prev: *mut aeTimeEvent,
    pub next: *mut aeTimeEvent,
    pub refcount: libc::c_int,
}
pub type aeEventFinalizerProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    *mut libc::c_void,
) -> ();
pub type aeTimeProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_longlong,
    *mut libc::c_void,
) -> libc::c_int;
pub type monotime = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFiredEvent {
    pub fd: libc::c_int,
    pub mask: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aeFileEvent {
    pub mask: libc::c_int,
    pub rfileProc: Option::<aeFileProc>,
    pub wfileProc: Option::<aeFileProc>,
    pub clientData: *mut libc::c_void,
}
pub type aeFileProc = unsafe extern "C" fn(
    *mut aeEventLoop,
    libc::c_int,
    *mut libc::c_void,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct connection {
    pub type_0: *mut ConnectionType,
    pub state: ConnectionState,
    pub flags: libc::c_short,
    pub refs: libc::c_short,
    pub last_errno: libc::c_int,
    pub private_data: *mut libc::c_void,
    pub conn_handler: ConnectionCallbackFunc,
    pub write_handler: ConnectionCallbackFunc,
    pub read_handler: ConnectionCallbackFunc,
    pub fd: libc::c_int,
}
pub type ConnectionCallbackFunc = Option::<unsafe extern "C" fn(*mut connection) -> ()>;
pub type ConnectionState = libc::c_uint;
pub const CONN_STATE_ERROR: ConnectionState = 5;
pub const CONN_STATE_CLOSED: ConnectionState = 4;
pub const CONN_STATE_CONNECTED: ConnectionState = 3;
pub const CONN_STATE_ACCEPTING: ConnectionState = 2;
pub const CONN_STATE_CONNECTING: ConnectionState = 1;
pub const CONN_STATE_NONE: ConnectionState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectionType {
    pub ae_handler: Option::<
        unsafe extern "C" fn(
            *mut aeEventLoop,
            libc::c_int,
            *mut libc::c_void,
            libc::c_int,
        ) -> (),
    >,
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            *const libc::c_char,
            ConnectionCallbackFunc,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut connection, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub writev: Option::<
        unsafe extern "C" fn(*mut connection, *const iovec, libc::c_int) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void, size_t) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut connection) -> ()>,
    pub accept: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub set_write_handler: Option::<
        unsafe extern "C" fn(
            *mut connection,
            ConnectionCallbackFunc,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub set_read_handler: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub get_last_error: Option::<
        unsafe extern "C" fn(*mut connection) -> *const libc::c_char,
    >,
    pub blocking_connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            libc::c_longlong,
        ) -> libc::c_int,
    >,
    pub sync_write: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_read: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_readline: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub get_type: Option::<unsafe extern "C" fn(*mut connection) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rio {
    pub read: Option::<
        unsafe extern "C" fn(*mut _rio, *mut libc::c_void, size_t) -> size_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> size_t,
    >,
    pub tell: Option::<unsafe extern "C" fn(*mut _rio) -> off_t>,
    pub flush: Option::<unsafe extern "C" fn(*mut _rio) -> libc::c_int>,
    pub update_cksum: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> (),
    >,
    pub cksum: uint64_t,
    pub flags: uint64_t,
    pub processed_bytes: size_t,
    pub max_processing_chunk: size_t,
    pub io: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub buffer: C2RustUnnamed_3,
    pub file: C2RustUnnamed_2,
    pub conn: C2RustUnnamed_1,
    pub fd: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub pos: off_t,
    pub buf: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub conn: *mut connection,
    pub pos: off_t,
    pub buf: sds,
    pub read_limit: size_t,
    pub read_so_far: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fp: *mut FILE,
    pub buffered: off_t,
    pub autosync: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ptr: sds,
    pub pos: off_t,
}
pub type rio = _rio;
#[inline]
unsafe extern "C" fn sdslen(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .len as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .len as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .len as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .len;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return 0 as libc::c_int as size_t,
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr8;
            return ((*sh).alloc as libc::c_int - (*sh).len as libc::c_int) as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr16;
            return ((*sh_0).alloc as libc::c_int - (*sh_0).len as libc::c_int) as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr32;
            return ((*sh_1).alloc).wrapping_sub((*sh_1).len) as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr64;
            return ((*sh_2).alloc).wrapping_sub((*sh_2).len);
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn connRead(
    mut conn: *mut connection,
    mut buf: *mut libc::c_void,
    mut buf_len: size_t,
) -> libc::c_int {
    let mut ret: libc::c_int = ((*(*conn).type_0).read)
        .expect("non-null function pointer")(conn, buf, buf_len);
    return ret;
}
#[inline]
unsafe extern "C" fn connLastErrorRetryable(mut conn: *mut connection) -> libc::c_int {
    return ((*conn).last_errno == 4 as libc::c_int) as libc::c_int;
}
#[inline]
unsafe extern "C" fn rioWrite(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    if (*r).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        return 0 as libc::c_int as size_t;
    }
    while len != 0 {
        let mut bytes_to_write: size_t = if (*r).max_processing_chunk != 0
            && (*r).max_processing_chunk < len
        {
            (*r).max_processing_chunk
        } else {
            len
        };
        if ((*r).update_cksum).is_some() {
            ((*r).update_cksum)
                .expect("non-null function pointer")(r, buf, bytes_to_write);
        }
        if ((*r).write).expect("non-null function pointer")(r, buf, bytes_to_write)
            == 0 as libc::c_int as libc::c_ulong
        {
            (*r).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
            return 0 as libc::c_int as size_t;
        }
        buf = (buf as *mut libc::c_char).offset(bytes_to_write as isize)
            as *const libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(bytes_to_write) as size_t as size_t;
        (*r)
            .processed_bytes = ((*r).processed_bytes as libc::c_ulong)
            .wrapping_add(bytes_to_write) as size_t as size_t;
    }
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn rioBufferWrite(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    (*r)
        .io
        .buffer
        .ptr = sdscatlen(
        (*r).io.buffer.ptr,
        buf as *mut libc::c_char as *const libc::c_void,
        len,
    );
    (*r)
        .io
        .buffer
        .pos = ((*r).io.buffer.pos as libc::c_ulong).wrapping_add(len) as off_t as off_t;
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn rioBufferRead(
    mut r: *mut rio,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    if (sdslen((*r).io.buffer.ptr)).wrapping_sub((*r).io.buffer.pos as libc::c_ulong)
        < len
    {
        return 0 as libc::c_int as size_t;
    }
    memcpy(
        buf,
        ((*r).io.buffer.ptr).offset((*r).io.buffer.pos as isize) as *const libc::c_void,
        len,
    );
    (*r)
        .io
        .buffer
        .pos = ((*r).io.buffer.pos as libc::c_ulong).wrapping_add(len) as off_t as off_t;
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn rioBufferTell(mut r: *mut rio) -> off_t {
    return (*r).io.buffer.pos;
}
unsafe extern "C" fn rioBufferFlush(mut r: *mut rio) -> libc::c_int {
    return 1 as libc::c_int;
}
static mut rioBufferIO: rio = unsafe {
    {
        let mut init = _rio {
            read: Some(
                rioBufferRead
                    as unsafe extern "C" fn(
                        *mut rio,
                        *mut libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            write: Some(
                rioBufferWrite
                    as unsafe extern "C" fn(
                        *mut rio,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            tell: Some(rioBufferTell as unsafe extern "C" fn(*mut rio) -> off_t),
            flush: Some(rioBufferFlush as unsafe extern "C" fn(*mut rio) -> libc::c_int),
            update_cksum: None,
            cksum: 0 as libc::c_int as uint64_t,
            flags: 0 as libc::c_int as uint64_t,
            processed_bytes: 0 as libc::c_int as size_t,
            max_processing_chunk: 0 as libc::c_int as size_t,
            io: C2RustUnnamed {
                buffer: {
                    let mut init = C2RustUnnamed_3 {
                        ptr: 0 as *const libc::c_char as sds,
                        pos: 0 as libc::c_int as off_t,
                    };
                    init
                },
            },
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn rioInitWithBuffer(mut r: *mut rio, mut s: sds) {
    *r = rioBufferIO;
    (*r).io.buffer.ptr = s;
    (*r).io.buffer.pos = 0 as libc::c_int as off_t;
}
unsafe extern "C" fn rioFileWrite(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    if (*r).io.file.autosync == 0 {
        return fwrite(buf, len, 1 as libc::c_int as libc::c_ulong, (*r).io.file.fp);
    }
    let mut nwritten: size_t = 0 as libc::c_int as size_t;
    while len != nwritten {
        if (*r).io.file.autosync > (*r).io.file.buffered {} else {
            _serverAssert(
                b"r->io.file.autosync > r->io.file.buffered\0" as *const u8
                    as *const libc::c_char,
                b"rio.c\0" as *const u8 as *const libc::c_char,
                118 as libc::c_int,
            );
            unreachable!();
        };
        let mut nalign: size_t = ((*r).io.file.autosync - (*r).io.file.buffered)
            as size_t;
        let mut towrite: size_t = if nalign > len.wrapping_sub(nwritten) {
            len.wrapping_sub(nwritten)
        } else {
            nalign
        };
        if fwrite(
            (buf as *mut libc::c_char).offset(nwritten as isize) as *const libc::c_void,
            towrite,
            1 as libc::c_int as libc::c_ulong,
            (*r).io.file.fp,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as libc::c_int as size_t;
        }
        nwritten = (nwritten as libc::c_ulong).wrapping_add(towrite) as size_t as size_t;
        (*r)
            .io
            .file
            .buffered = ((*r).io.file.buffered as libc::c_ulong).wrapping_add(towrite)
            as off_t as off_t;
        if (*r).io.file.buffered >= (*r).io.file.autosync {
            fflush((*r).io.file.fp);
            let mut processed: size_t = ((*r).processed_bytes).wrapping_add(nwritten);
            if processed.wrapping_rem((*r).io.file.autosync as libc::c_ulong)
                == 0 as libc::c_int as libc::c_ulong
            {} else {
                _serverAssert(
                    b"processed % r->io.file.autosync == 0\0" as *const u8
                        as *const libc::c_char,
                    b"rio.c\0" as *const u8 as *const libc::c_char,
                    130 as libc::c_int,
                );
                unreachable!();
            };
            if (*r).io.file.buffered == (*r).io.file.autosync {} else {
                _serverAssert(
                    b"r->io.file.buffered == r->io.file.autosync\0" as *const u8
                        as *const libc::c_char,
                    b"rio.c\0" as *const u8 as *const libc::c_char,
                    131 as libc::c_int,
                );
                unreachable!();
            };
            if sync_file_range(
                fileno((*r).io.file.fp),
                processed.wrapping_sub((*r).io.file.autosync as libc::c_ulong)
                    as __off64_t,
                (*r).io.file.autosync,
                2 as libc::c_int as libc::c_uint,
            ) == -(1 as libc::c_int)
            {
                return 0 as libc::c_int as size_t;
            }
            if processed
                >= ((*r).io.file.autosync as size_t)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            {
                if sync_file_range(
                    fileno((*r).io.file.fp),
                    processed
                        .wrapping_sub(
                            ((*r).io.file.autosync * 2 as libc::c_int as libc::c_long)
                                as libc::c_ulong,
                        ) as __off64_t,
                    (*r).io.file.autosync,
                    (1 as libc::c_int | 2 as libc::c_int | 4 as libc::c_int)
                        as libc::c_uint,
                ) == -(1 as libc::c_int)
                {
                    return 0 as libc::c_int as size_t;
                }
            }
            (*r).io.file.buffered = 0 as libc::c_int as off_t;
        }
    }
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn rioFileRead(
    mut r: *mut rio,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    return fread(buf, len, 1 as libc::c_int as libc::c_ulong, (*r).io.file.fp);
}
unsafe extern "C" fn rioFileTell(mut r: *mut rio) -> off_t {
    return ftello((*r).io.file.fp);
}
unsafe extern "C" fn rioFileFlush(mut r: *mut rio) -> libc::c_int {
    return if fflush((*r).io.file.fp) == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
static mut rioFileIO: rio = unsafe {
    {
        let mut init = _rio {
            read: Some(
                rioFileRead
                    as unsafe extern "C" fn(
                        *mut rio,
                        *mut libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            write: Some(
                rioFileWrite
                    as unsafe extern "C" fn(
                        *mut rio,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            tell: Some(rioFileTell as unsafe extern "C" fn(*mut rio) -> off_t),
            flush: Some(rioFileFlush as unsafe extern "C" fn(*mut rio) -> libc::c_int),
            update_cksum: None,
            cksum: 0 as libc::c_int as uint64_t,
            flags: 0 as libc::c_int as uint64_t,
            processed_bytes: 0 as libc::c_int as size_t,
            max_processing_chunk: 0 as libc::c_int as size_t,
            io: C2RustUnnamed {
                buffer: {
                    let mut init = C2RustUnnamed_3 {
                        ptr: 0 as *const libc::c_char as sds,
                        pos: 0 as libc::c_int as off_t,
                    };
                    init
                },
            },
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn rioInitWithFile(mut r: *mut rio, mut fp: *mut FILE) {
    *r = rioFileIO;
    (*r).io.file.fp = fp;
    (*r).io.file.buffered = 0 as libc::c_int as off_t;
    (*r).io.file.autosync = 0 as libc::c_int as off_t;
}
unsafe extern "C" fn rioConnWrite(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn rioConnRead(
    mut r: *mut rio,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut avail: size_t = (sdslen((*r).io.conn.buf))
        .wrapping_sub((*r).io.conn.pos as libc::c_ulong);
    if (sdslen((*r).io.conn.buf)).wrapping_add(sdsavail((*r).io.conn.buf)) < len {
        (*r)
            .io
            .conn
            .buf = sdsMakeRoomFor(
            (*r).io.conn.buf,
            len.wrapping_sub(sdslen((*r).io.conn.buf)),
        );
    }
    if len > avail && sdsavail((*r).io.conn.buf) < len.wrapping_sub(avail) {
        sdsrange((*r).io.conn.buf, (*r).io.conn.pos, -(1 as libc::c_int) as ssize_t);
        (*r).io.conn.pos = 0 as libc::c_int as off_t;
    }
    if (*r).io.conn.read_limit != 0 as libc::c_int as libc::c_ulong
        && (*r).io.conn.read_limit < ((*r).io.conn.read_so_far).wrapping_add(len)
    {
        *__errno_location() = 75 as libc::c_int;
        return 0 as libc::c_int as size_t;
    }
    while len
        > (sdslen((*r).io.conn.buf)).wrapping_sub((*r).io.conn.pos as libc::c_ulong)
    {
        let mut buffered: size_t = (sdslen((*r).io.conn.buf))
            .wrapping_sub((*r).io.conn.pos as libc::c_ulong);
        let mut needs: size_t = len.wrapping_sub(buffered);
        let mut toread: size_t = if needs
            < (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
        {
            (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
        } else {
            needs
        };
        if toread > sdsavail((*r).io.conn.buf) {
            toread = sdsavail((*r).io.conn.buf);
        }
        if (*r).io.conn.read_limit != 0 as libc::c_int as libc::c_ulong
            && ((*r).io.conn.read_so_far).wrapping_add(buffered).wrapping_add(toread)
                > (*r).io.conn.read_limit
        {
            toread = ((*r).io.conn.read_limit)
                .wrapping_sub((*r).io.conn.read_so_far)
                .wrapping_sub(buffered);
        }
        let mut retval: libc::c_int = connRead(
            (*r).io.conn.conn,
            ((*r).io.conn.buf).offset(sdslen((*r).io.conn.buf) as isize)
                as *mut libc::c_void,
            toread,
        );
        if retval == 0 as libc::c_int {
            return 0 as libc::c_int as size_t
        } else if retval < 0 as libc::c_int {
            if connLastErrorRetryable((*r).io.conn.conn) != 0 {
                continue;
            }
            if *__errno_location() == 11 as libc::c_int {
                *__errno_location() = 110 as libc::c_int;
            }
            return 0 as libc::c_int as size_t;
        } else {
            sdsIncrLen((*r).io.conn.buf, retval as ssize_t);
        }
    }
    memcpy(
        buf,
        ((*r).io.conn.buf).offset((*r).io.conn.pos as isize) as *const libc::c_void,
        len,
    );
    (*r)
        .io
        .conn
        .read_so_far = ((*r).io.conn.read_so_far as libc::c_ulong).wrapping_add(len)
        as size_t as size_t;
    (*r)
        .io
        .conn
        .pos = ((*r).io.conn.pos as libc::c_ulong).wrapping_add(len) as off_t as off_t;
    return len;
}
unsafe extern "C" fn rioConnTell(mut r: *mut rio) -> off_t {
    return (*r).io.conn.read_so_far as off_t;
}
unsafe extern "C" fn rioConnFlush(mut r: *mut rio) -> libc::c_int {
    return rioConnWrite(r, 0 as *const libc::c_void, 0 as libc::c_int as size_t)
        as libc::c_int;
}
static mut rioConnIO: rio = unsafe {
    {
        let mut init = _rio {
            read: Some(
                rioConnRead
                    as unsafe extern "C" fn(
                        *mut rio,
                        *mut libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            write: Some(
                rioConnWrite
                    as unsafe extern "C" fn(
                        *mut rio,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            tell: Some(rioConnTell as unsafe extern "C" fn(*mut rio) -> off_t),
            flush: Some(rioConnFlush as unsafe extern "C" fn(*mut rio) -> libc::c_int),
            update_cksum: None,
            cksum: 0 as libc::c_int as uint64_t,
            flags: 0 as libc::c_int as uint64_t,
            processed_bytes: 0 as libc::c_int as size_t,
            max_processing_chunk: 0 as libc::c_int as size_t,
            io: C2RustUnnamed {
                buffer: {
                    let mut init = C2RustUnnamed_3 {
                        ptr: 0 as *const libc::c_char as sds,
                        pos: 0 as libc::c_int as off_t,
                    };
                    init
                },
            },
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn rioInitWithConn(
    mut r: *mut rio,
    mut conn: *mut connection,
    mut read_limit: size_t,
) {
    *r = rioConnIO;
    (*r).io.conn.conn = conn;
    (*r).io.conn.pos = 0 as libc::c_int as off_t;
    (*r).io.conn.read_limit = read_limit;
    (*r).io.conn.read_so_far = 0 as libc::c_int as size_t;
    (*r)
        .io
        .conn
        .buf = sdsnewlen(
        0 as *const libc::c_void,
        (1024 as libc::c_int * 16 as libc::c_int) as size_t,
    );
    sdsclear((*r).io.conn.buf);
}
#[no_mangle]
pub unsafe extern "C" fn rioFreeConn(mut r: *mut rio, mut remaining: *mut sds) {
    if !remaining.is_null() && ((*r).io.conn.pos as size_t) < sdslen((*r).io.conn.buf) {
        if (*r).io.conn.pos > 0 as libc::c_int as libc::c_long {
            sdsrange((*r).io.conn.buf, (*r).io.conn.pos, -(1 as libc::c_int) as ssize_t);
        }
        *remaining = (*r).io.conn.buf;
    } else {
        sdsfree((*r).io.conn.buf);
        if !remaining.is_null() {
            *remaining = 0 as sds;
        }
    }
    (*r).io.conn.buf = 0 as sds;
}
unsafe extern "C" fn rioFdWrite(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) -> size_t {
    let mut retval: ssize_t = 0;
    let mut p: *mut libc::c_uchar = buf as *mut libc::c_uchar;
    let mut doflush: libc::c_int = (buf.is_null()
        && len == 0 as libc::c_int as libc::c_ulong) as libc::c_int;
    if len > (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong {
        if sdslen((*r).io.fd.buf) != 0 {
            if rioFdWrite(r, 0 as *const libc::c_void, 0 as libc::c_int as size_t)
                == 0 as libc::c_int as libc::c_ulong
            {
                return 0 as libc::c_int as size_t;
            }
        }
    } else {
        if len != 0 {
            (*r).io.fd.buf = sdscatlen((*r).io.fd.buf, buf, len);
            if sdslen((*r).io.fd.buf)
                > (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
            {
                doflush = 1 as libc::c_int;
            }
            if doflush == 0 {
                return 1 as libc::c_int as size_t;
            }
        }
        p = (*r).io.fd.buf as *mut libc::c_uchar;
        len = sdslen((*r).io.fd.buf);
    }
    let mut nwritten: size_t = 0 as libc::c_int as size_t;
    while nwritten != len {
        retval = write(
            (*r).io.fd.fd,
            p.offset(nwritten as isize) as *const libc::c_void,
            len.wrapping_sub(nwritten),
        );
        if retval <= 0 as libc::c_int as libc::c_long {
            if retval == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 4 as libc::c_int
            {
                continue;
            }
            if retval == -(1 as libc::c_int) as libc::c_long
                && *__errno_location() == 11 as libc::c_int
            {
                *__errno_location() = 110 as libc::c_int;
            }
            return 0 as libc::c_int as size_t;
        } else {
            nwritten = (nwritten as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
                as size_t as size_t;
        }
    }
    (*r)
        .io
        .fd
        .pos = ((*r).io.fd.pos as libc::c_ulong).wrapping_add(len) as off_t as off_t;
    sdsclear((*r).io.fd.buf);
    return 1 as libc::c_int as size_t;
}
unsafe extern "C" fn rioFdRead(
    mut r: *mut rio,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    return 0 as libc::c_int as size_t;
}
unsafe extern "C" fn rioFdTell(mut r: *mut rio) -> off_t {
    return (*r).io.fd.pos;
}
unsafe extern "C" fn rioFdFlush(mut r: *mut rio) -> libc::c_int {
    return rioFdWrite(r, 0 as *const libc::c_void, 0 as libc::c_int as size_t)
        as libc::c_int;
}
static mut rioFdIO: rio = unsafe {
    {
        let mut init = _rio {
            read: Some(
                rioFdRead
                    as unsafe extern "C" fn(
                        *mut rio,
                        *mut libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            write: Some(
                rioFdWrite
                    as unsafe extern "C" fn(
                        *mut rio,
                        *const libc::c_void,
                        size_t,
                    ) -> size_t,
            ),
            tell: Some(rioFdTell as unsafe extern "C" fn(*mut rio) -> off_t),
            flush: Some(rioFdFlush as unsafe extern "C" fn(*mut rio) -> libc::c_int),
            update_cksum: None,
            cksum: 0 as libc::c_int as uint64_t,
            flags: 0 as libc::c_int as uint64_t,
            processed_bytes: 0 as libc::c_int as size_t,
            max_processing_chunk: 0 as libc::c_int as size_t,
            io: C2RustUnnamed {
                buffer: {
                    let mut init = C2RustUnnamed_3 {
                        ptr: 0 as *const libc::c_char as sds,
                        pos: 0 as libc::c_int as off_t,
                    };
                    init
                },
            },
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn rioInitWithFd(mut r: *mut rio, mut fd: libc::c_int) {
    *r = rioFdIO;
    (*r).io.fd.fd = fd;
    (*r).io.fd.pos = 0 as libc::c_int as off_t;
    (*r).io.fd.buf = sdsempty();
}
#[no_mangle]
pub unsafe extern "C" fn rioFreeFd(mut r: *mut rio) {
    sdsfree((*r).io.fd.buf);
}
#[no_mangle]
pub unsafe extern "C" fn rioGenericUpdateChecksum(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) {
    (*r).cksum = crc64((*r).cksum, buf as *const libc::c_uchar, len);
}
#[no_mangle]
pub unsafe extern "C" fn rioSetAutoSync(mut r: *mut rio, mut bytes: off_t) {
    if (*r).write != rioFileIO.write {
        return;
    }
    (*r).io.file.autosync = bytes;
}
#[no_mangle]
pub unsafe extern "C" fn rioCheckType(mut r: *mut rio) -> uint8_t {
    if (*r).read
        == Some(
            rioFileRead
                as unsafe extern "C" fn(*mut rio, *mut libc::c_void, size_t) -> size_t,
        )
    {
        return ((1 as libc::c_int) << 0 as libc::c_int) as uint8_t
    } else if (*r).read
        == Some(
            rioBufferRead
                as unsafe extern "C" fn(*mut rio, *mut libc::c_void, size_t) -> size_t,
        )
    {
        return ((1 as libc::c_int) << 1 as libc::c_int) as uint8_t
    } else if (*r).read
        == Some(
            rioConnRead
                as unsafe extern "C" fn(*mut rio, *mut libc::c_void, size_t) -> size_t,
        )
    {
        return ((1 as libc::c_int) << 2 as libc::c_int) as uint8_t
    } else {
        return ((1 as libc::c_int) << 3 as libc::c_int) as uint8_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkCount(
    mut r: *mut rio,
    mut prefix: libc::c_char,
    mut count: libc::c_long,
) -> size_t {
    let mut cbuf: [libc::c_char; 128] = [0; 128];
    let mut clen: libc::c_int = 0;
    cbuf[0 as libc::c_int as usize] = prefix;
    clen = 1 as libc::c_int
        + ll2string(
            cbuf.as_mut_ptr().offset(1 as libc::c_int as isize),
            (core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            count as libc::c_longlong,
        );
    let fresh0 = clen;
    clen = clen + 1;
    cbuf[fresh0 as usize] = '\r' as i32 as libc::c_char;
    let fresh1 = clen;
    clen = clen + 1;
    cbuf[fresh1 as usize] = '\n' as i32 as libc::c_char;
    if rioWrite(r, cbuf.as_mut_ptr() as *const libc::c_void, clen as size_t)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    return clen as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkString(
    mut r: *mut rio,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut nwritten: size_t = 0;
    nwritten = rioWriteBulkCount(r, '$' as i32 as libc::c_char, len as libc::c_long);
    if nwritten == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    if len > 0 as libc::c_int as libc::c_ulong
        && rioWrite(r, buf as *const libc::c_void, len)
            == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    if rioWrite(
        r,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as size_t;
    }
    return nwritten.wrapping_add(len).wrapping_add(2 as libc::c_int as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkLongLong(
    mut r: *mut rio,
    mut l: libc::c_longlong,
) -> size_t {
    let mut lbuf: [libc::c_char; 32] = [0; 32];
    let mut llen: libc::c_uint = 0;
    llen = ll2string(
        lbuf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        l,
    ) as libc::c_uint;
    return rioWriteBulkString(r, lbuf.as_mut_ptr(), llen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkDouble(
    mut r: *mut rio,
    mut d: libc::c_double,
) -> size_t {
    let mut dbuf: [libc::c_char; 128] = [0; 128];
    let mut dlen: libc::c_uint = 0;
    dlen = snprintf(
        dbuf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        b"%.17g\0" as *const u8 as *const libc::c_char,
        d,
    ) as libc::c_uint;
    return rioWriteBulkString(r, dbuf.as_mut_ptr(), dlen as size_t);
}

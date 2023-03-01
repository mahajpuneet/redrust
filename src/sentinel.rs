extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    pub type rewriteConfigState;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn __errno_location() -> *mut libc::c_int;
    fn ctime_r(__timer: *const time_t, __buf: *mut libc::c_char) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn rand() -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(__status: libc::c_int) -> !;
    fn fork() -> __pid_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn aeCreateFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
        proc_0: Option::<aeFileProc>,
        clientData: *mut libc::c_void,
    ) -> libc::c_int;
    fn aeDeleteFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
    );
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictAddRaw(
        d: *mut dict,
        key: *mut libc::c_void,
        existing: *mut *mut dictEntry,
    ) -> *mut dictEntry;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn anetResolve(
        err: *mut libc::c_char,
        host: *mut libc::c_char,
        ipbuf: *mut libc::c_char,
        ipbuf_len: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn anetCloexec(fd: libc::c_int) -> libc::c_int;
    fn anetFdToString(
        fd: libc::c_int,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
        fd_to_str_type: libc::c_int,
    ) -> libc::c_int;
    fn stringmatch(
        p: *const libc::c_char,
        s: *const libc::c_char,
        nocase: libc::c_int,
    ) -> libc::c_int;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn yesnotoi(s: *mut libc::c_char) -> libc::c_int;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut stringSetDictType: dictType;
    fn mstime() -> libc::c_longlong;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn addReplyNull(c: *mut client);
    fn addReplyNullArray(c: *mut client);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplySds(c: *mut client, s: sds);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorSds(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyErrorArity(c: *mut client);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getLongLongFromObject(o: *mut robj, target: *mut libc::c_longlong) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn pubsubPublishMessage(
        channel: *mut robj,
        message: *mut robj,
        sharded: libc::c_int,
    ) -> libc::c_int;
    fn dictSdsKeyCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsCaseHash(key: *const libc::c_void) -> uint64_t;
    fn rewriteConfig(path: *mut libc::c_char, force_write: libc::c_int) -> libc::c_int;
    fn rewriteConfigMarkAsProcessed(
        state: *mut rewriteConfigState,
        option: *const libc::c_char,
    );
    fn rewriteConfigRewriteLine(
        state: *mut rewriteConfigState,
        option: *const libc::c_char,
        line: sds,
        force: libc::c_int,
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn genInfoSectionDict(
        argv: *mut *mut robj,
        argc: libc::c_int,
        defaults: *mut *mut libc::c_char,
        out_all: *mut libc::c_int,
        out_everything: *mut libc::c_int,
    ) -> *mut dict;
    fn releaseInfoSectionDict(sec: *mut dict);
    fn genRedisInfoString(
        section_dict: *mut dict,
        all_sections: libc::c_int,
        everything: libc::c_int,
    ) -> sds;
    fn tlsCleanup();
    fn redisAsyncConnectBind(
        ip: *const libc::c_char,
        port: libc::c_int,
        source_addr: *const libc::c_char,
    ) -> *mut redisAsyncContext;
    fn redisAsyncSetConnectCallback(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisConnectCallback>,
    ) -> libc::c_int;
    fn redisAsyncSetDisconnectCallback(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisDisconnectCallback>,
    ) -> libc::c_int;
    fn redisAsyncFree(ac: *mut redisAsyncContext);
    fn redisAsyncHandleRead(ac: *mut redisAsyncContext);
    fn redisAsyncHandleWrite(ac: *mut redisAsyncContext);
    fn redisAsyncCommand(
        ac: *mut redisAsyncContext,
        fn_0: Option::<redisCallbackFn>,
        privdata: *mut libc::c_void,
        format: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn addInfoSectionsToDict(section_dict: *mut dict, sections: *mut *mut libc::c_char);
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
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type __intmax_t = libc::c_long;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;
pub type va_list = __builtin_va_list;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type intmax_t = __intmax_t;
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_histogram {
    pub lowest_discernible_value: int64_t,
    pub highest_trackable_value: int64_t,
    pub unit_magnitude: int32_t,
    pub significant_figures: int32_t,
    pub sub_bucket_half_count_magnitude: int32_t,
    pub sub_bucket_half_count: int32_t,
    pub sub_bucket_mask: int64_t,
    pub sub_bucket_count: int32_t,
    pub bucket_count: int32_t,
    pub min_value: int64_t,
    pub max_value: int64_t,
    pub normalizing_index_offset: int32_t,
    pub conversion_ratio: libc::c_double,
    pub counts_len: int32_t,
    pub total_count: int64_t,
    pub counts: *mut int64_t,
}
pub type mstime_t = libc::c_longlong;
pub type ustime_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub v: C2RustUnnamed,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub val: *mut libc::c_void,
    pub u64_0: uint64_t,
    pub s64: int64_t,
    pub d: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dict {
    pub type_0: *mut dictType,
    pub ht_table: [*mut *mut dictEntry; 2],
    pub ht_used: [libc::c_ulong; 2],
    pub rehashidx: libc::c_long,
    pub pauserehash: int16_t,
    pub ht_size_exp: [libc::c_schar; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictType {
    pub hashFunction: Option::<unsafe extern "C" fn(*const libc::c_void) -> uint64_t>,
    pub keyDup: Option::<
        unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub valDup: Option::<
        unsafe extern "C" fn(*mut dict, *const libc::c_void) -> *mut libc::c_void,
    >,
    pub keyCompare: Option::<
        unsafe extern "C" fn(
            *mut dict,
            *const libc::c_void,
            *const libc::c_void,
        ) -> libc::c_int,
    >,
    pub keyDestructor: Option::<
        unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
    >,
    pub valDestructor: Option::<
        unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
    >,
    pub expandAllowed: Option::<
        unsafe extern "C" fn(size_t, libc::c_double) -> libc::c_int,
    >,
    pub dictEntryMetadataBytes: Option::<unsafe extern "C" fn(*mut dict) -> size_t>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictIterator {
    pub d: *mut dict,
    pub index: libc::c_long,
    pub table: libc::c_int,
    pub safe: libc::c_int,
    pub entry: *mut dictEntry,
    pub nextEntry: *mut dictEntry,
    pub fingerprint: libc::c_ulonglong,
}
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct raxNode {
    #[bitfield(name = "iskey", ty = "uint32_t", bits = "0..=0")]
    #[bitfield(name = "isnull", ty = "uint32_t", bits = "1..=1")]
    #[bitfield(name = "iscompr", ty = "uint32_t", bits = "2..=2")]
    #[bitfield(name = "size", ty = "uint32_t", bits = "3..=31")]
    pub iskey_isnull_iscompr_size: [u8; 4],
    pub data: [libc::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rax {
    pub head: *mut raxNode,
    pub numele: uint64_t,
    pub numnodes: uint64_t,
}
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pause_event {
    pub type_0: pause_type,
    pub end: mstime_t,
}
pub type robj = redisObject;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisDb {
    pub dict: *mut dict,
    pub expires: *mut dict,
    pub blocking_keys: *mut dict,
    pub ready_keys: *mut dict,
    pub watched_keys: *mut dict,
    pub id: libc::c_int,
    pub avg_ttl: libc::c_longlong,
    pub expires_cursor: libc::c_ulong,
    pub defrag_later: *mut list,
    pub slots_to_keys: *mut clusterSlotToKeyMapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiCmd {
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub argc: libc::c_int,
    pub cmd: *mut redisCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommand {
    pub declared_name: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub complexity: *const libc::c_char,
    pub since: *const libc::c_char,
    pub doc_flags: libc::c_int,
    pub replaced_by: *const libc::c_char,
    pub deprecated_since: *const libc::c_char,
    pub group: redisCommandGroup,
    pub history: *mut commandHistory,
    pub tips: *mut *const libc::c_char,
    pub proc_0: Option::<redisCommandProc>,
    pub arity: libc::c_int,
    pub flags: uint64_t,
    pub acl_categories: uint64_t,
    pub key_specs_static: [keySpec; 4],
    pub getkeys_proc: Option::<redisGetKeysProc>,
    pub subcommands: *mut redisCommand,
    pub args: *mut redisCommandArg,
    pub microseconds: libc::c_longlong,
    pub calls: libc::c_longlong,
    pub rejected_calls: libc::c_longlong,
    pub failed_calls: libc::c_longlong,
    pub id: libc::c_int,
    pub fullname: sds,
    pub latency_histogram: *mut hdr_histogram,
    pub key_specs: *mut keySpec,
    pub legacy_range_key_spec: keySpec,
    pub num_args: libc::c_int,
    pub num_history: libc::c_int,
    pub num_tips: libc::c_int,
    pub key_specs_num: libc::c_int,
    pub key_specs_max: libc::c_int,
    pub subcommands_dict: *mut dict,
    pub parent: *mut redisCommand,
    pub module_cmd: *mut RedisModuleCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keySpec {
    pub notes: *const libc::c_char,
    pub flags: uint64_t,
    pub begin_search_type: kspec_bs_type,
    pub bs: C2RustUnnamed_3,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub range: C2RustUnnamed_2,
    pub keynum: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub lastkey: libc::c_int,
    pub keystep: libc::c_int,
    pub limit: libc::c_int,
}
pub type kspec_fk_type = libc::c_uint;
pub const KSPEC_FK_KEYNUM: kspec_fk_type = 3;
pub const KSPEC_FK_RANGE: kspec_fk_type = 2;
pub const KSPEC_FK_UNKNOWN: kspec_fk_type = 1;
pub const KSPEC_FK_INVALID: kspec_fk_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub index: C2RustUnnamed_5,
    pub keyword: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub pos: libc::c_int,
}
pub type kspec_bs_type = libc::c_uint;
pub const KSPEC_BS_KEYWORD: kspec_bs_type = 3;
pub const KSPEC_BS_INDEX: kspec_bs_type = 2;
pub const KSPEC_BS_UNKNOWN: kspec_bs_type = 1;
pub const KSPEC_BS_INVALID: kspec_bs_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommandArg {
    pub name: *const libc::c_char,
    pub type_0: redisCommandArgType,
    pub key_spec_index: libc::c_int,
    pub token: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub since: *const libc::c_char,
    pub flags: libc::c_int,
    pub deprecated_since: *const libc::c_char,
    pub subargs: *mut redisCommandArg,
    pub num_args: libc::c_int,
}
pub type redisCommandArgType = libc::c_uint;
pub const ARG_TYPE_BLOCK: redisCommandArgType = 8;
pub const ARG_TYPE_ONEOF: redisCommandArgType = 7;
pub const ARG_TYPE_PURE_TOKEN: redisCommandArgType = 6;
pub const ARG_TYPE_UNIX_TIME: redisCommandArgType = 5;
pub const ARG_TYPE_PATTERN: redisCommandArgType = 4;
pub const ARG_TYPE_KEY: redisCommandArgType = 3;
pub const ARG_TYPE_DOUBLE: redisCommandArgType = 2;
pub const ARG_TYPE_INTEGER: redisCommandArgType = 1;
pub const ARG_TYPE_STRING: redisCommandArgType = 0;
pub type redisGetKeysProc = unsafe extern "C" fn(
    *mut redisCommand,
    *mut *mut robj,
    libc::c_int,
    *mut getKeysResult,
) -> libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getKeysResult {
    pub keysbuf: [keyReference; 256],
    pub keys: *mut keyReference,
    pub numkeys: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyReference {
    pub pos: libc::c_int,
    pub flags: libc::c_int,
}
pub type redisCommandProc = unsafe extern "C" fn(*mut client) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub id: uint64_t,
    pub flags: uint64_t,
    pub conn: *mut connection,
    pub resp: libc::c_int,
    pub db: *mut redisDb,
    pub name: *mut robj,
    pub querybuf: sds,
    pub qb_pos: size_t,
    pub querybuf_peak: size_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub original_argc: libc::c_int,
    pub original_argv: *mut *mut robj,
    pub argv_len_sum: size_t,
    pub cmd: *mut redisCommand,
    pub lastcmd: *mut redisCommand,
    pub realcmd: *mut redisCommand,
    pub user: *mut user,
    pub reqtype: libc::c_int,
    pub multibulklen: libc::c_int,
    pub bulklen: libc::c_long,
    pub reply: *mut list,
    pub reply_bytes: libc::c_ulonglong,
    pub deferred_reply_errors: *mut list,
    pub sentlen: size_t,
    pub ctime: time_t,
    pub duration: libc::c_long,
    pub slot: libc::c_int,
    pub cur_script: *mut dictEntry,
    pub lastinteraction: time_t,
    pub obuf_soft_limit_reached_time: time_t,
    pub authenticated: libc::c_int,
    pub replstate: libc::c_int,
    pub repl_start_cmd_stream_on_ack: libc::c_int,
    pub repldbfd: libc::c_int,
    pub repldboff: off_t,
    pub repldbsize: off_t,
    pub replpreamble: sds,
    pub read_reploff: libc::c_longlong,
    pub reploff: libc::c_longlong,
    pub repl_applied: libc::c_longlong,
    pub repl_ack_off: libc::c_longlong,
    pub repl_ack_time: libc::c_longlong,
    pub repl_last_partial_write: libc::c_longlong,
    pub psync_initial_offset: libc::c_longlong,
    pub replid: [libc::c_char; 41],
    pub slave_listening_port: libc::c_int,
    pub slave_addr: *mut libc::c_char,
    pub slave_capa: libc::c_int,
    pub slave_req: libc::c_int,
    pub mstate: multiState,
    pub btype: libc::c_int,
    pub bpop: blockingState,
    pub woff: libc::c_longlong,
    pub watched_keys: *mut list,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut list,
    pub pubsubshard_channels: *mut dict,
    pub peerid: sds,
    pub sockname: sds,
    pub client_list_node: *mut listNode,
    pub postponed_list_node: *mut listNode,
    pub pending_read_list_node: *mut listNode,
    pub auth_callback: RedisModuleUserChangedFunc,
    pub auth_callback_privdata: *mut libc::c_void,
    pub auth_module: *mut libc::c_void,
    pub client_tracking_redirection: uint64_t,
    pub client_tracking_prefixes: *mut rax,
    pub last_memory_usage: size_t,
    pub last_memory_type: libc::c_int,
    pub mem_usage_bucket_node: *mut listNode,
    pub mem_usage_bucket: *mut clientMemUsageBucket,
    pub ref_repl_buf_node: *mut listNode,
    pub ref_block_pos: size_t,
    pub buf_peak: size_t,
    pub buf_peak_last_reset_time: mstime_t,
    pub bufpos: libc::c_int,
    pub buf_usable_size: size_t,
    pub buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientMemUsageBucket {
    pub clients: *mut list,
    pub mem_usage_sum: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockingState {
    pub count: libc::c_long,
    pub timeout: mstime_t,
    pub keys: *mut dict,
    pub target: *mut robj,
    pub blockpos: blockPos,
    pub xread_count: size_t,
    pub xread_group: *mut robj,
    pub xread_consumer: *mut robj,
    pub xread_group_noack: libc::c_int,
    pub numreplicas: libc::c_int,
    pub reploffset: libc::c_longlong,
    pub module_blocked_handle: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockPos {
    pub wherefrom: libc::c_int,
    pub whereto: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiState {
    pub commands: *mut multiCmd,
    pub count: libc::c_int,
    pub cmd_flags: libc::c_int,
    pub cmd_inv_flags: libc::c_int,
    pub argv_len_sums: size_t,
    pub alloc_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user {
    pub name: sds,
    pub flags: uint32_t,
    pub passwords: *mut list,
    pub selectors: *mut list,
    pub acl_string: *mut robj,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandHistory {
    pub since: *const libc::c_char,
    pub changes: *const libc::c_char,
}
pub type redisCommandGroup = libc::c_uint;
pub const COMMAND_GROUP_MODULE: redisCommandGroup = 17;
pub const COMMAND_GROUP_BITMAP: redisCommandGroup = 16;
pub const COMMAND_GROUP_STREAM: redisCommandGroup = 15;
pub const COMMAND_GROUP_GEO: redisCommandGroup = 14;
pub const COMMAND_GROUP_SENTINEL: redisCommandGroup = 13;
pub const COMMAND_GROUP_CLUSTER: redisCommandGroup = 12;
pub const COMMAND_GROUP_HYPERLOGLOG: redisCommandGroup = 11;
pub const COMMAND_GROUP_SCRIPTING: redisCommandGroup = 10;
pub const COMMAND_GROUP_SERVER: redisCommandGroup = 9;
pub const COMMAND_GROUP_CONNECTION: redisCommandGroup = 8;
pub const COMMAND_GROUP_TRANSACTIONS: redisCommandGroup = 7;
pub const COMMAND_GROUP_PUBSUB: redisCommandGroup = 6;
pub const COMMAND_GROUP_HASH: redisCommandGroup = 5;
pub const COMMAND_GROUP_SORTED_SET: redisCommandGroup = 4;
pub const COMMAND_GROUP_SET: redisCommandGroup = 3;
pub const COMMAND_GROUP_LIST: redisCommandGroup = 2;
pub const COMMAND_GROUP_STRING: redisCommandGroup = 1;
pub const COMMAND_GROUP_GENERIC: redisCommandGroup = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replBacklog {
    pub ref_repl_buf_node: *mut listNode,
    pub unindexed_count: size_t,
    pub blocks_index: *mut rax,
    pub histlen: libc::c_longlong,
    pub offset: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveparam {
    pub seconds: time_t,
    pub changes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelLoadQueueEntry {
    pub argc: libc::c_int,
    pub argv: *mut sds,
    pub linenum: libc::c_int,
    pub line: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelConfig {
    pub pre_monitor_cfg: *mut list,
    pub monitor_cfg: *mut list,
    pub post_monitor_cfg: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharedObjectsStruct {
    pub crlf: *mut robj,
    pub ok: *mut robj,
    pub err: *mut robj,
    pub emptybulk: *mut robj,
    pub czero: *mut robj,
    pub cone: *mut robj,
    pub pong: *mut robj,
    pub space: *mut robj,
    pub queued: *mut robj,
    pub null: [*mut robj; 4],
    pub nullarray: [*mut robj; 4],
    pub emptymap: [*mut robj; 4],
    pub emptyset: [*mut robj; 4],
    pub emptyarray: *mut robj,
    pub wrongtypeerr: *mut robj,
    pub nokeyerr: *mut robj,
    pub syntaxerr: *mut robj,
    pub sameobjecterr: *mut robj,
    pub outofrangeerr: *mut robj,
    pub noscripterr: *mut robj,
    pub loadingerr: *mut robj,
    pub slowevalerr: *mut robj,
    pub slowscripterr: *mut robj,
    pub slowmoduleerr: *mut robj,
    pub bgsaveerr: *mut robj,
    pub masterdownerr: *mut robj,
    pub roslaveerr: *mut robj,
    pub execaborterr: *mut robj,
    pub noautherr: *mut robj,
    pub noreplicaserr: *mut robj,
    pub busykeyerr: *mut robj,
    pub oomerr: *mut robj,
    pub plus: *mut robj,
    pub messagebulk: *mut robj,
    pub pmessagebulk: *mut robj,
    pub subscribebulk: *mut robj,
    pub unsubscribebulk: *mut robj,
    pub psubscribebulk: *mut robj,
    pub punsubscribebulk: *mut robj,
    pub del: *mut robj,
    pub unlink: *mut robj,
    pub rpop: *mut robj,
    pub lpop: *mut robj,
    pub lpush: *mut robj,
    pub rpoplpush: *mut robj,
    pub lmove: *mut robj,
    pub blmove: *mut robj,
    pub zpopmin: *mut robj,
    pub zpopmax: *mut robj,
    pub emptyscan: *mut robj,
    pub multi: *mut robj,
    pub exec: *mut robj,
    pub left: *mut robj,
    pub right: *mut robj,
    pub hset: *mut robj,
    pub srem: *mut robj,
    pub xgroup: *mut robj,
    pub xclaim: *mut robj,
    pub script: *mut robj,
    pub replconf: *mut robj,
    pub eval: *mut robj,
    pub persist: *mut robj,
    pub set: *mut robj,
    pub pexpireat: *mut robj,
    pub pexpire: *mut robj,
    pub time: *mut robj,
    pub pxat: *mut robj,
    pub absttl: *mut robj,
    pub retrycount: *mut robj,
    pub force: *mut robj,
    pub justid: *mut robj,
    pub entriesread: *mut robj,
    pub lastid: *mut robj,
    pub ping: *mut robj,
    pub setid: *mut robj,
    pub keepttl: *mut robj,
    pub load: *mut robj,
    pub createconsumer: *mut robj,
    pub getack: *mut robj,
    pub special_asterick: *mut robj,
    pub special_equals: *mut robj,
    pub default_username: *mut robj,
    pub redacted: *mut robj,
    pub ssubscribebulk: *mut robj,
    pub sunsubscribebulk: *mut robj,
    pub smessagebulk: *mut robj,
    pub select: [*mut robj; 10],
    pub integers: [*mut robj; 10000],
    pub mbulkhdr: [*mut robj; 32],
    pub bulkhdr: [*mut robj; 32],
    pub maphdr: [*mut robj; 32],
    pub sethdr: [*mut robj; 32],
    pub minstring: sds,
    pub maxstring: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientBufferLimitsConfig {
    pub hard_limit_bytes: libc::c_ulonglong,
    pub soft_limit_bytes: libc::c_ulonglong,
    pub soft_limit_seconds: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOp {
    pub argv: *mut *mut robj,
    pub argc: libc::c_int,
    pub dbid: libc::c_int,
    pub target: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOpArray {
    pub ops: *mut redisOp,
    pub numops: libc::c_int,
    pub capacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_stats {
    pub zmalloc_used: size_t,
    pub process_rss: size_t,
    pub allocator_allocated: size_t,
    pub allocator_active: size_t,
    pub allocator_resident: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socketFds {
    pub fd: [libc::c_int; 16],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisTLSContextConfig {
    pub cert_file: *mut libc::c_char,
    pub key_file: *mut libc::c_char,
    pub key_file_pass: *mut libc::c_char,
    pub client_cert_file: *mut libc::c_char,
    pub client_key_file: *mut libc::c_char,
    pub client_key_file_pass: *mut libc::c_char,
    pub dh_params_file: *mut libc::c_char,
    pub ca_cert_file: *mut libc::c_char,
    pub ca_cert_dir: *mut libc::c_char,
    pub protocols: *mut libc::c_char,
    pub ciphers: *mut libc::c_char,
    pub ciphersuites: *mut libc::c_char,
    pub prefer_server_ciphers: libc::c_int,
    pub session_caching: libc::c_int,
    pub session_cache_size: libc::c_int,
    pub session_cache_timeout: libc::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisServer {
    pub pid: pid_t,
    pub main_thread_id: pthread_t,
    pub configfile: *mut libc::c_char,
    pub executable: *mut libc::c_char,
    pub exec_argv: *mut *mut libc::c_char,
    pub dynamic_hz: libc::c_int,
    pub config_hz: libc::c_int,
    pub umask: mode_t,
    pub hz: libc::c_int,
    pub in_fork_child: libc::c_int,
    pub db: *mut redisDb,
    pub commands: *mut dict,
    pub orig_commands: *mut dict,
    pub el: *mut aeEventLoop,
    pub errors: *mut rax,
    pub lruclock: atomic_uint,
    pub shutdown_asap: sig_atomic_t,
    pub shutdown_mstime: mstime_t,
    pub last_sig_received: libc::c_int,
    pub shutdown_flags: libc::c_int,
    pub activerehashing: libc::c_int,
    pub active_defrag_running: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub arch_bits: libc::c_int,
    pub cronloops: libc::c_int,
    pub runid: [libc::c_char; 41],
    pub sentinel_mode: libc::c_int,
    pub initial_memory_usage: size_t,
    pub always_show_logo: libc::c_int,
    pub in_exec: libc::c_int,
    pub busy_module_yield_flags: libc::c_int,
    pub busy_module_yield_reply: *const libc::c_char,
    pub core_propagates: libc::c_int,
    pub propagate_no_multi: libc::c_int,
    pub module_ctx_nesting: libc::c_int,
    pub ignore_warnings: *mut libc::c_char,
    pub client_pause_in_transaction: libc::c_int,
    pub thp_enabled: libc::c_int,
    pub page_size: size_t,
    pub moduleapi: *mut dict,
    pub sharedapi: *mut dict,
    pub module_configs_queue: *mut dict,
    pub loadmodule_queue: *mut list,
    pub module_pipe: [libc::c_int; 2],
    pub child_pid: pid_t,
    pub child_type: libc::c_int,
    pub port: libc::c_int,
    pub tls_port: libc::c_int,
    pub tcp_backlog: libc::c_int,
    pub bindaddr: [*mut libc::c_char; 16],
    pub bindaddr_count: libc::c_int,
    pub bind_source_addr: *mut libc::c_char,
    pub unixsocket: *mut libc::c_char,
    pub unixsocketperm: libc::c_uint,
    pub ipfd: socketFds,
    pub tlsfd: socketFds,
    pub sofd: libc::c_int,
    pub socket_mark_id: uint32_t,
    pub cfd: socketFds,
    pub clients: *mut list,
    pub clients_to_close: *mut list,
    pub clients_pending_write: *mut list,
    pub clients_pending_read: *mut list,
    pub slaves: *mut list,
    pub monitors: *mut list,
    pub current_client: *mut client,
    pub client_mem_usage_buckets: *mut clientMemUsageBucket,
    pub clients_timeout_table: *mut rax,
    pub fixed_time_expire: libc::c_long,
    pub in_nested_call: libc::c_int,
    pub clients_index: *mut rax,
    pub client_pause_type: pause_type,
    pub postponed_clients: *mut list,
    pub client_pause_end_time: mstime_t,
    pub client_pause_per_purpose: [*mut pause_event; 3],
    pub neterr: [libc::c_char; 256],
    pub migrate_cached_sockets: *mut dict,
    pub next_client_id: uint_least64_t,
    pub protected_mode: libc::c_int,
    pub io_threads_num: libc::c_int,
    pub io_threads_do_reads: libc::c_int,
    pub io_threads_active: libc::c_int,
    pub events_processed_while_blocked: libc::c_longlong,
    pub enable_protected_configs: libc::c_int,
    pub enable_debug_cmd: libc::c_int,
    pub enable_module_cmd: libc::c_int,
    pub loading: sig_atomic_t,
    pub async_loading: sig_atomic_t,
    pub loading_total_bytes: off_t,
    pub loading_rdb_used_mem: off_t,
    pub loading_loaded_bytes: off_t,
    pub loading_start_time: time_t,
    pub loading_process_events_interval_bytes: off_t,
    pub stat_starttime: time_t,
    pub stat_numcommands: libc::c_longlong,
    pub stat_numconnections: libc::c_longlong,
    pub stat_expiredkeys: libc::c_longlong,
    pub stat_expired_stale_perc: libc::c_double,
    pub stat_expired_time_cap_reached_count: libc::c_longlong,
    pub stat_expire_cycle_time_used: libc::c_longlong,
    pub stat_evictedkeys: libc::c_longlong,
    pub stat_evictedclients: libc::c_longlong,
    pub stat_total_eviction_exceeded_time: libc::c_longlong,
    pub stat_last_eviction_exceeded_time: monotime,
    pub stat_keyspace_hits: libc::c_longlong,
    pub stat_keyspace_misses: libc::c_longlong,
    pub stat_active_defrag_hits: libc::c_longlong,
    pub stat_active_defrag_misses: libc::c_longlong,
    pub stat_active_defrag_key_hits: libc::c_longlong,
    pub stat_active_defrag_key_misses: libc::c_longlong,
    pub stat_active_defrag_scanned: libc::c_longlong,
    pub stat_total_active_defrag_time: libc::c_longlong,
    pub stat_last_active_defrag_time: monotime,
    pub stat_peak_memory: size_t,
    pub stat_aof_rewrites: libc::c_longlong,
    pub stat_aofrw_consecutive_failures: libc::c_longlong,
    pub stat_rdb_saves: libc::c_longlong,
    pub stat_fork_time: libc::c_longlong,
    pub stat_fork_rate: libc::c_double,
    pub stat_total_forks: libc::c_longlong,
    pub stat_rejected_conn: libc::c_longlong,
    pub stat_sync_full: libc::c_longlong,
    pub stat_sync_partial_ok: libc::c_longlong,
    pub stat_sync_partial_err: libc::c_longlong,
    pub slowlog: *mut list,
    pub slowlog_entry_id: libc::c_longlong,
    pub slowlog_log_slower_than: libc::c_longlong,
    pub slowlog_max_len: libc::c_ulong,
    pub cron_malloc_stats: malloc_stats,
    pub stat_net_input_bytes: atomic_llong,
    pub stat_net_output_bytes: atomic_llong,
    pub stat_net_repl_input_bytes: atomic_llong,
    pub stat_net_repl_output_bytes: atomic_llong,
    pub stat_current_cow_peak: size_t,
    pub stat_current_cow_bytes: size_t,
    pub stat_current_cow_updated: monotime,
    pub stat_current_save_keys_processed: size_t,
    pub stat_current_save_keys_total: size_t,
    pub stat_rdb_cow_bytes: size_t,
    pub stat_aof_cow_bytes: size_t,
    pub stat_module_cow_bytes: size_t,
    pub stat_module_progress: libc::c_double,
    pub stat_clients_type_memory: [size_t; 4],
    pub stat_cluster_links_memory: size_t,
    pub stat_unexpected_error_replies: libc::c_longlong,
    pub stat_total_error_replies: libc::c_longlong,
    pub stat_dump_payload_sanitizations: libc::c_longlong,
    pub stat_io_reads_processed: libc::c_longlong,
    pub stat_io_writes_processed: libc::c_longlong,
    pub stat_total_reads_processed: atomic_llong,
    pub stat_total_writes_processed: atomic_llong,
    pub inst_metric: [C2RustUnnamed_6; 5],
    pub stat_reply_buffer_shrinks: libc::c_longlong,
    pub stat_reply_buffer_expands: libc::c_longlong,
    pub verbosity: libc::c_int,
    pub maxidletime: libc::c_int,
    pub tcpkeepalive: libc::c_int,
    pub active_expire_enabled: libc::c_int,
    pub active_expire_effort: libc::c_int,
    pub active_defrag_enabled: libc::c_int,
    pub sanitize_dump_payload: libc::c_int,
    pub skip_checksum_validation: libc::c_int,
    pub jemalloc_bg_thread: libc::c_int,
    pub active_defrag_ignore_bytes: size_t,
    pub active_defrag_threshold_lower: libc::c_int,
    pub active_defrag_threshold_upper: libc::c_int,
    pub active_defrag_cycle_min: libc::c_int,
    pub active_defrag_cycle_max: libc::c_int,
    pub active_defrag_max_scan_fields: libc::c_ulong,
    pub client_max_querybuf_len: size_t,
    pub dbnum: libc::c_int,
    pub supervised: libc::c_int,
    pub supervised_mode: libc::c_int,
    pub daemonize: libc::c_int,
    pub set_proc_title: libc::c_int,
    pub proc_title_template: *mut libc::c_char,
    pub client_obuf_limits: [clientBufferLimitsConfig; 3],
    pub pause_cron: libc::c_int,
    pub latency_tracking_enabled: libc::c_int,
    pub latency_tracking_info_percentiles: *mut libc::c_double,
    pub latency_tracking_info_percentiles_len: libc::c_int,
    pub aof_enabled: libc::c_int,
    pub aof_state: libc::c_int,
    pub aof_fsync: libc::c_int,
    pub aof_filename: *mut libc::c_char,
    pub aof_dirname: *mut libc::c_char,
    pub aof_no_fsync_on_rewrite: libc::c_int,
    pub aof_rewrite_perc: libc::c_int,
    pub aof_rewrite_min_size: off_t,
    pub aof_rewrite_base_size: off_t,
    pub aof_current_size: off_t,
    pub aof_last_incr_size: off_t,
    pub aof_fsync_offset: off_t,
    pub aof_flush_sleep: libc::c_int,
    pub aof_rewrite_scheduled: libc::c_int,
    pub aof_buf: sds,
    pub aof_fd: libc::c_int,
    pub aof_selected_db: libc::c_int,
    pub aof_flush_postponed_start: time_t,
    pub aof_last_fsync: time_t,
    pub aof_rewrite_time_last: time_t,
    pub aof_rewrite_time_start: time_t,
    pub aof_cur_timestamp: time_t,
    pub aof_timestamp_enabled: libc::c_int,
    pub aof_lastbgrewrite_status: libc::c_int,
    pub aof_delayed_fsync: libc::c_ulong,
    pub aof_rewrite_incremental_fsync: libc::c_int,
    pub rdb_save_incremental_fsync: libc::c_int,
    pub aof_last_write_status: libc::c_int,
    pub aof_last_write_errno: libc::c_int,
    pub aof_load_truncated: libc::c_int,
    pub aof_use_rdb_preamble: libc::c_int,
    pub aof_bio_fsync_status: atomic_int,
    pub aof_bio_fsync_errno: atomic_int,
    pub aof_manifest: *mut aofManifest,
    pub aof_disable_auto_gc: libc::c_int,
    pub dirty: libc::c_longlong,
    pub dirty_before_bgsave: libc::c_longlong,
    pub rdb_last_load_keys_expired: libc::c_longlong,
    pub rdb_last_load_keys_loaded: libc::c_longlong,
    pub saveparams: *mut saveparam,
    pub saveparamslen: libc::c_int,
    pub rdb_filename: *mut libc::c_char,
    pub rdb_compression: libc::c_int,
    pub rdb_checksum: libc::c_int,
    pub rdb_del_sync_files: libc::c_int,
    pub lastsave: time_t,
    pub lastbgsave_try: time_t,
    pub rdb_save_time_last: time_t,
    pub rdb_save_time_start: time_t,
    pub rdb_bgsave_scheduled: libc::c_int,
    pub rdb_child_type: libc::c_int,
    pub lastbgsave_status: libc::c_int,
    pub stop_writes_on_bgsave_err: libc::c_int,
    pub rdb_pipe_read: libc::c_int,
    pub rdb_child_exit_pipe: libc::c_int,
    pub rdb_pipe_conns: *mut *mut connection,
    pub rdb_pipe_numconns: libc::c_int,
    pub rdb_pipe_numconns_writing: libc::c_int,
    pub rdb_pipe_buff: *mut libc::c_char,
    pub rdb_pipe_bufflen: libc::c_int,
    pub rdb_key_save_delay: libc::c_int,
    pub key_load_delay: libc::c_int,
    pub child_info_pipe: [libc::c_int; 2],
    pub child_info_nread: libc::c_int,
    pub also_propagate: redisOpArray,
    pub replication_allowed: libc::c_int,
    pub logfile: *mut libc::c_char,
    pub syslog_enabled: libc::c_int,
    pub syslog_ident: *mut libc::c_char,
    pub syslog_facility: libc::c_int,
    pub crashlog_enabled: libc::c_int,
    pub memcheck_enabled: libc::c_int,
    pub use_exit_on_panic: libc::c_int,
    pub shutdown_timeout: libc::c_int,
    pub shutdown_on_sigint: libc::c_int,
    pub shutdown_on_sigterm: libc::c_int,
    pub replid: [libc::c_char; 41],
    pub replid2: [libc::c_char; 41],
    pub master_repl_offset: libc::c_longlong,
    pub second_replid_offset: libc::c_longlong,
    pub slaveseldb: libc::c_int,
    pub repl_ping_slave_period: libc::c_int,
    pub repl_backlog: *mut replBacklog,
    pub repl_backlog_size: libc::c_longlong,
    pub repl_backlog_time_limit: time_t,
    pub repl_no_slaves_since: time_t,
    pub repl_min_slaves_to_write: libc::c_int,
    pub repl_min_slaves_max_lag: libc::c_int,
    pub repl_good_slaves_count: libc::c_int,
    pub repl_diskless_sync: libc::c_int,
    pub repl_diskless_load: libc::c_int,
    pub repl_diskless_sync_delay: libc::c_int,
    pub repl_diskless_sync_max_replicas: libc::c_int,
    pub repl_buffer_mem: size_t,
    pub repl_buffer_blocks: *mut list,
    pub masteruser: *mut libc::c_char,
    pub masterauth: sds,
    pub masterhost: *mut libc::c_char,
    pub masterport: libc::c_int,
    pub repl_timeout: libc::c_int,
    pub master: *mut client,
    pub cached_master: *mut client,
    pub repl_syncio_timeout: libc::c_int,
    pub repl_state: libc::c_int,
    pub repl_transfer_size: off_t,
    pub repl_transfer_read: off_t,
    pub repl_transfer_last_fsync_off: off_t,
    pub repl_transfer_s: *mut connection,
    pub repl_transfer_fd: libc::c_int,
    pub repl_transfer_tmpfile: *mut libc::c_char,
    pub repl_transfer_lastio: time_t,
    pub repl_serve_stale_data: libc::c_int,
    pub repl_slave_ro: libc::c_int,
    pub repl_slave_ignore_maxmemory: libc::c_int,
    pub repl_down_since: time_t,
    pub repl_disable_tcp_nodelay: libc::c_int,
    pub slave_priority: libc::c_int,
    pub replica_announced: libc::c_int,
    pub slave_announce_port: libc::c_int,
    pub slave_announce_ip: *mut libc::c_char,
    pub propagation_error_behavior: libc::c_int,
    pub repl_ignore_disk_write_error: libc::c_int,
    pub master_replid: [libc::c_char; 41],
    pub master_initial_offset: libc::c_longlong,
    pub repl_slave_lazy_flush: libc::c_int,
    pub clients_waiting_acks: *mut list,
    pub get_ack_from_slaves: libc::c_int,
    pub maxclients: libc::c_uint,
    pub maxmemory: libc::c_ulonglong,
    pub maxmemory_clients: ssize_t,
    pub maxmemory_policy: libc::c_int,
    pub maxmemory_samples: libc::c_int,
    pub maxmemory_eviction_tenacity: libc::c_int,
    pub lfu_log_factor: libc::c_int,
    pub lfu_decay_time: libc::c_int,
    pub proto_max_bulk_len: libc::c_longlong,
    pub oom_score_adj_values: [libc::c_int; 3],
    pub oom_score_adj: libc::c_int,
    pub disable_thp: libc::c_int,
    pub blocked_clients: libc::c_uint,
    pub blocked_clients_by_type: [libc::c_uint; 8],
    pub unblocked_clients: *mut list,
    pub ready_keys: *mut list,
    pub tracking_clients: libc::c_uint,
    pub tracking_table_max_keys: size_t,
    pub tracking_pending_keys: *mut list,
    pub sort_desc: libc::c_int,
    pub sort_alpha: libc::c_int,
    pub sort_bypattern: libc::c_int,
    pub sort_store: libc::c_int,
    pub hash_max_listpack_entries: size_t,
    pub hash_max_listpack_value: size_t,
    pub set_max_intset_entries: size_t,
    pub zset_max_listpack_entries: size_t,
    pub zset_max_listpack_value: size_t,
    pub hll_sparse_max_bytes: size_t,
    pub stream_node_max_bytes: size_t,
    pub stream_node_max_entries: libc::c_longlong,
    pub list_max_listpack_size: libc::c_int,
    pub list_compress_depth: libc::c_int,
    pub unixtime: atomic_int,
    pub timezone: time_t,
    pub daylight_active: libc::c_int,
    pub mstime: mstime_t,
    pub ustime: ustime_t,
    pub blocking_op_nesting: size_t,
    pub blocked_last_cron: libc::c_longlong,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut dict,
    pub notify_keyspace_events: libc::c_int,
    pub pubsubshard_channels: *mut dict,
    pub cluster_enabled: libc::c_int,
    pub cluster_port: libc::c_int,
    pub cluster_node_timeout: mstime_t,
    pub cluster_configfile: *mut libc::c_char,
    pub cluster: *mut clusterState,
    pub cluster_migration_barrier: libc::c_int,
    pub cluster_allow_replica_migration: libc::c_int,
    pub cluster_slave_validity_factor: libc::c_int,
    pub cluster_require_full_coverage: libc::c_int,
    pub cluster_slave_no_failover: libc::c_int,
    pub cluster_announce_ip: *mut libc::c_char,
    pub cluster_announce_hostname: *mut libc::c_char,
    pub cluster_preferred_endpoint_type: libc::c_int,
    pub cluster_announce_port: libc::c_int,
    pub cluster_announce_tls_port: libc::c_int,
    pub cluster_announce_bus_port: libc::c_int,
    pub cluster_module_flags: libc::c_int,
    pub cluster_allow_reads_when_down: libc::c_int,
    pub cluster_config_file_lock_fd: libc::c_int,
    pub cluster_link_sendbuf_limit_bytes: libc::c_ulonglong,
    pub cluster_drop_packet_filter: libc::c_int,
    pub script_caller: *mut client,
    pub busy_reply_threshold: mstime_t,
    pub pre_command_oom_state: libc::c_int,
    pub script_disable_deny_script: libc::c_int,
    pub lazyfree_lazy_eviction: libc::c_int,
    pub lazyfree_lazy_expire: libc::c_int,
    pub lazyfree_lazy_server_del: libc::c_int,
    pub lazyfree_lazy_user_del: libc::c_int,
    pub lazyfree_lazy_user_flush: libc::c_int,
    pub latency_monitor_threshold: libc::c_longlong,
    pub latency_events: *mut dict,
    pub acl_filename: *mut libc::c_char,
    pub acllog_max_len: libc::c_ulong,
    pub requirepass: sds,
    pub acl_pubsub_default: libc::c_int,
    pub watchdog_period: libc::c_int,
    pub system_memory_size: size_t,
    pub tls_cluster: libc::c_int,
    pub tls_replication: libc::c_int,
    pub tls_auth_clients: libc::c_int,
    pub tls_ctx_config: redisTLSContextConfig,
    pub server_cpulist: *mut libc::c_char,
    pub bio_cpulist: *mut libc::c_char,
    pub aof_rewrite_cpulist: *mut libc::c_char,
    pub bgsave_cpulist: *mut libc::c_char,
    pub sentinel_config: *mut sentinelConfig,
    pub failover_end_time: mstime_t,
    pub force_failover: libc::c_int,
    pub target_replica_host: *mut libc::c_char,
    pub target_replica_port: libc::c_int,
    pub failover_state: libc::c_int,
    pub cluster_allow_pubsubshard_when_down: libc::c_int,
    pub reply_buffer_peak_reset_time: libc::c_long,
    pub reply_buffer_resizing_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelState {
    pub myid: [libc::c_char; 41],
    pub current_epoch: uint64_t,
    pub masters: *mut dict,
    pub tilt: libc::c_int,
    pub running_scripts: libc::c_int,
    pub tilt_start_time: mstime_t,
    pub previous_time: mstime_t,
    pub scripts_queue: *mut list,
    pub announce_ip: *mut libc::c_char,
    pub announce_port: libc::c_int,
    pub simfailure_flags: libc::c_ulong,
    pub deny_scripts_reconfig: libc::c_int,
    pub sentinel_auth_pass: *mut libc::c_char,
    pub sentinel_auth_user: *mut libc::c_char,
    pub resolve_hostnames: libc::c_int,
    pub announce_hostnames: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelRedisInstance {
    pub flags: libc::c_int,
    pub name: *mut libc::c_char,
    pub runid: *mut libc::c_char,
    pub config_epoch: uint64_t,
    pub addr: *mut sentinelAddr,
    pub link: *mut instanceLink,
    pub last_pub_time: mstime_t,
    pub last_hello_time: mstime_t,
    pub last_master_down_reply_time: mstime_t,
    pub s_down_since_time: mstime_t,
    pub o_down_since_time: mstime_t,
    pub down_after_period: mstime_t,
    pub master_reboot_down_after_period: mstime_t,
    pub master_reboot_since_time: mstime_t,
    pub info_refresh: mstime_t,
    pub renamed_commands: *mut dict,
    pub role_reported: libc::c_int,
    pub role_reported_time: mstime_t,
    pub slave_conf_change_time: mstime_t,
    pub sentinels: *mut dict,
    pub slaves: *mut dict,
    pub quorum: libc::c_uint,
    pub parallel_syncs: libc::c_int,
    pub auth_pass: *mut libc::c_char,
    pub auth_user: *mut libc::c_char,
    pub master_link_down_time: mstime_t,
    pub slave_priority: libc::c_int,
    pub replica_announced: libc::c_int,
    pub slave_reconf_sent_time: mstime_t,
    pub master: *mut sentinelRedisInstance,
    pub slave_master_host: *mut libc::c_char,
    pub slave_master_port: libc::c_int,
    pub slave_master_link_status: libc::c_int,
    pub slave_repl_offset: libc::c_ulonglong,
    pub leader: *mut libc::c_char,
    pub leader_epoch: uint64_t,
    pub failover_epoch: uint64_t,
    pub failover_state: libc::c_int,
    pub failover_state_change_time: mstime_t,
    pub failover_start_time: mstime_t,
    pub failover_timeout: mstime_t,
    pub failover_delay_logged: mstime_t,
    pub promoted_slave: *mut sentinelRedisInstance,
    pub notification_script: *mut libc::c_char,
    pub client_reconfig_script: *mut libc::c_char,
    pub info: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instanceLink {
    pub refcount: libc::c_int,
    pub disconnected: libc::c_int,
    pub pending_commands: libc::c_int,
    pub cc: *mut redisAsyncContext,
    pub pc: *mut redisAsyncContext,
    pub cc_conn_time: mstime_t,
    pub pc_conn_time: mstime_t,
    pub pc_last_activity: mstime_t,
    pub last_avail_time: mstime_t,
    pub act_ping_time: mstime_t,
    pub last_ping_time: mstime_t,
    pub last_pong_time: mstime_t,
    pub last_reconn_time: mstime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisAsyncContext {
    pub c: redisContext,
    pub err: libc::c_int,
    pub errstr: *mut libc::c_char,
    pub data: *mut libc::c_void,
    pub dataCleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub ev: C2RustUnnamed_8,
    pub onDisconnect: Option::<redisDisconnectCallback>,
    pub onConnect: Option::<redisConnectCallback>,
    pub replies: redisCallbackList,
    pub saddr: *mut sockaddr,
    pub addrlen: size_t,
    pub sub: C2RustUnnamed_7,
    pub push_cb: Option::<redisAsyncPushFn>,
}
pub type redisAsyncPushFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub replies: redisCallbackList,
    pub channels: *mut dict,
    pub patterns: *mut dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallbackList {
    pub head: *mut redisCallback,
    pub tail: *mut redisCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCallback {
    pub next: *mut redisCallback,
    pub fn_0: Option::<redisCallbackFn>,
    pub pending_subs: libc::c_int,
    pub privdata: *mut libc::c_void,
}
pub type redisCallbackFn = unsafe extern "C" fn(
    *mut redisAsyncContext,
    *mut libc::c_void,
    *mut libc::c_void,
) -> ();
pub type redisConnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
pub type redisDisconnectCallback = unsafe extern "C" fn(
    *const redisAsyncContext,
    libc::c_int,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub data: *mut libc::c_void,
    pub addRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delRead: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub addWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub delWrite: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub cleanup: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub scheduleTimer: Option::<unsafe extern "C" fn(*mut libc::c_void, timeval) -> ()>,
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
    pub tcp: C2RustUnnamed_10,
    pub unix_sock: C2RustUnnamed_9,
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
pub struct C2RustUnnamed_9 {
    pub path: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub host: *mut libc::c_char,
    pub source_addr: *mut libc::c_char,
    pub port: libc::c_int,
}
pub type redisConnectionType = libc::c_uint;
pub const REDIS_CONN_USERFD: redisConnectionType = 2;
pub const REDIS_CONN_UNIX: redisConnectionType = 1;
pub const REDIS_CONN_TCP: redisConnectionType = 0;
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
pub struct redisReadTask {
    pub type_0: libc::c_int,
    pub elements: libc::c_longlong,
    pub idx: libc::c_int,
    pub obj: *mut libc::c_void,
    pub parent: *mut redisReadTask,
    pub privdata: *mut libc::c_void,
}
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
pub struct sentinelAddr {
    pub hostname: *mut libc::c_char,
    pub ip: *mut libc::c_char,
    pub port: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelScriptJob {
    pub flags: libc::c_int,
    pub retry_num: libc::c_int,
    pub argv: *mut *mut libc::c_char,
    pub start_time: mstime_t,
    pub pid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisReply {
    pub type_0: libc::c_int,
    pub integer: libc::c_longlong,
    pub dval: libc::c_double,
    pub len: size_t,
    pub str_0: *mut libc::c_char,
    pub vtype: [libc::c_char; 4],
    pub elements: size_t,
    pub element: *mut *mut redisReply,
}
pub const _ISdigit: C2RustUnnamed_11 = 2048;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisAeEvents {
    pub context: *mut redisAsyncContext,
    pub loop_0: *mut aeEventLoop,
    pub fd: libc::c_int,
    pub reading: libc::c_int,
    pub writing: libc::c_int,
}
pub type C2RustUnnamed_11 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_11 = 8;
pub const _ISpunct: C2RustUnnamed_11 = 4;
pub const _IScntrl: C2RustUnnamed_11 = 2;
pub const _ISblank: C2RustUnnamed_11 = 1;
pub const _ISgraph: C2RustUnnamed_11 = 32768;
pub const _ISprint: C2RustUnnamed_11 = 16384;
pub const _ISspace: C2RustUnnamed_11 = 8192;
pub const _ISxdigit: C2RustUnnamed_11 = 4096;
pub const _ISalpha: C2RustUnnamed_11 = 1024;
pub const _ISlower: C2RustUnnamed_11 = 512;
pub const _ISupper: C2RustUnnamed_11 = 256;
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut sentinel_info_period: mstime_t = 10000 as libc::c_int as mstime_t;
static mut sentinel_ping_period: mstime_t = 1000 as libc::c_int as mstime_t;
static mut sentinel_ask_period: mstime_t = 1000 as libc::c_int as mstime_t;
static mut sentinel_publish_period: mstime_t = 2000 as libc::c_int as mstime_t;
static mut sentinel_default_down_after: mstime_t = 30000 as libc::c_int as mstime_t;
static mut sentinel_tilt_trigger: mstime_t = 2000 as libc::c_int as mstime_t;
static mut sentinel_tilt_period: mstime_t = (1000 as libc::c_int * 30 as libc::c_int)
    as mstime_t;
static mut sentinel_slave_reconf_timeout: mstime_t = 10000 as libc::c_int as mstime_t;
static mut sentinel_min_link_reconnect_period: mstime_t = 15000 as libc::c_int
    as mstime_t;
static mut sentinel_election_timeout: mstime_t = 10000 as libc::c_int as mstime_t;
static mut sentinel_script_max_runtime: mstime_t = 60000 as libc::c_int as mstime_t;
static mut sentinel_script_retry_delay: mstime_t = 30000 as libc::c_int as mstime_t;
static mut sentinel_default_failover_timeout: mstime_t = (60 as libc::c_int
    * 3 as libc::c_int * 1000 as libc::c_int) as mstime_t;
#[no_mangle]
pub static mut sentinel: sentinelState = sentinelState {
    myid: [0; 41],
    current_epoch: 0,
    masters: 0 as *const dict as *mut dict,
    tilt: 0,
    running_scripts: 0,
    tilt_start_time: 0,
    previous_time: 0,
    scripts_queue: 0 as *const list as *mut list,
    announce_ip: 0 as *const libc::c_char as *mut libc::c_char,
    announce_port: 0,
    simfailure_flags: 0,
    deny_scripts_reconfig: 0,
    sentinel_auth_pass: 0 as *const libc::c_char as *mut libc::c_char,
    sentinel_auth_user: 0 as *const libc::c_char as *mut libc::c_char,
    resolve_hostnames: 0,
    announce_hostnames: 0,
};
unsafe extern "C" fn redisAeReadEvent(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    redisAsyncHandleRead((*e).context);
}
unsafe extern "C" fn redisAeWriteEvent(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    redisAsyncHandleWrite((*e).context);
}
unsafe extern "C" fn redisAeAddRead(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    let mut loop_0: *mut aeEventLoop = (*e).loop_0;
    if (*e).reading == 0 {
        (*e).reading = 1 as libc::c_int;
        aeCreateFileEvent(
            loop_0,
            (*e).fd,
            1 as libc::c_int,
            Some(
                redisAeReadEvent
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            e as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn redisAeDelRead(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    let mut loop_0: *mut aeEventLoop = (*e).loop_0;
    if (*e).reading != 0 {
        (*e).reading = 0 as libc::c_int;
        aeDeleteFileEvent(loop_0, (*e).fd, 1 as libc::c_int);
    }
}
unsafe extern "C" fn redisAeAddWrite(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    let mut loop_0: *mut aeEventLoop = (*e).loop_0;
    if (*e).writing == 0 {
        (*e).writing = 1 as libc::c_int;
        aeCreateFileEvent(
            loop_0,
            (*e).fd,
            2 as libc::c_int,
            Some(
                redisAeWriteEvent
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            e as *mut libc::c_void,
        );
    }
}
unsafe extern "C" fn redisAeDelWrite(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    let mut loop_0: *mut aeEventLoop = (*e).loop_0;
    if (*e).writing != 0 {
        (*e).writing = 0 as libc::c_int;
        aeDeleteFileEvent(loop_0, (*e).fd, 2 as libc::c_int);
    }
}
unsafe extern "C" fn redisAeCleanup(mut privdata: *mut libc::c_void) {
    let mut e: *mut redisAeEvents = privdata as *mut redisAeEvents;
    redisAeDelRead(privdata);
    redisAeDelWrite(privdata);
    zfree(e as *mut libc::c_void);
}
unsafe extern "C" fn redisAeAttach(
    mut loop_0: *mut aeEventLoop,
    mut ac: *mut redisAsyncContext,
) -> libc::c_int {
    let mut c: *mut redisContext = &mut (*ac).c;
    let mut e: *mut redisAeEvents = 0 as *mut redisAeEvents;
    if !((*ac).ev.data).is_null() {
        return -(1 as libc::c_int);
    }
    e = zmalloc(core::mem::size_of::<redisAeEvents>() as libc::c_ulong)
        as *mut redisAeEvents;
    (*e).context = ac;
    (*e).loop_0 = loop_0;
    (*e).fd = (*c).fd;
    (*e).writing = 0 as libc::c_int;
    (*e).reading = (*e).writing;
    (*ac)
        .ev
        .addRead = Some(redisAeAddRead as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*ac)
        .ev
        .delRead = Some(redisAeDelRead as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*ac)
        .ev
        .addWrite = Some(
        redisAeAddWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .delWrite = Some(
        redisAeDelWrite as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*ac)
        .ev
        .cleanup = Some(redisAeCleanup as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*ac).ev.data = e as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictInstancesValDestructor(
    mut d: *mut dict,
    mut obj: *mut libc::c_void,
) {
    releaseSentinelRedisInstance(obj as *mut sentinelRedisInstance);
}
#[no_mangle]
pub static mut instancesDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: Some(
                dictInstancesValDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut leaderVotesDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut renamedCommandsDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCaseCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut preMonitorCfgName: [*const libc::c_char; 9] = [
    b"announce-ip\0" as *const u8 as *const libc::c_char,
    b"announce-port\0" as *const u8 as *const libc::c_char,
    b"deny-scripts-reconfig\0" as *const u8 as *const libc::c_char,
    b"sentinel-user\0" as *const u8 as *const libc::c_char,
    b"sentinel-pass\0" as *const u8 as *const libc::c_char,
    b"current-epoch\0" as *const u8 as *const libc::c_char,
    b"myid\0" as *const u8 as *const libc::c_char,
    b"resolve-hostnames\0" as *const u8 as *const libc::c_char,
    b"announce-hostnames\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn initSentinelConfig() {
    server.port = 26379 as libc::c_int;
    server.protected_mode = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initSentinel() {
    sentinel.current_epoch = 0 as libc::c_int as uint64_t;
    sentinel.masters = dictCreate(&mut instancesDictType);
    sentinel.tilt = 0 as libc::c_int;
    sentinel.tilt_start_time = 0 as libc::c_int as mstime_t;
    sentinel.previous_time = mstime();
    sentinel.running_scripts = 0 as libc::c_int;
    sentinel.scripts_queue = listCreate();
    sentinel.announce_ip = 0 as *mut libc::c_char;
    sentinel.announce_port = 0 as libc::c_int;
    sentinel.simfailure_flags = 0 as libc::c_int as libc::c_ulong;
    sentinel.deny_scripts_reconfig = 1 as libc::c_int;
    sentinel.sentinel_auth_pass = 0 as *mut libc::c_char;
    sentinel.sentinel_auth_user = 0 as *mut libc::c_char;
    sentinel.resolve_hostnames = 0 as libc::c_int;
    sentinel.announce_hostnames = 0 as libc::c_int;
    memset(
        (sentinel.myid).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    );
    server.sentinel_config = 0 as *mut sentinelConfig;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCheckConfigFile() {
    if (server.configfile).is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Sentinel needs config file on disk to save state. Exiting...\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    } else {
        if access(server.configfile, 2 as libc::c_int) == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Sentinel config file %s is not writable: %s. Exiting...\0"
                        as *const u8 as *const libc::c_char,
                    server.configfile,
                    strerror(*__errno_location()),
                );
            }
            exit(1 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelIsRunning() {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 40 as libc::c_int {
        if sentinel.myid[j as usize] as libc::c_int != 0 as libc::c_int {
            break;
        }
        j += 1;
    }
    if j == 40 as libc::c_int {
        getRandomHexChars((sentinel.myid).as_mut_ptr(), 40 as libc::c_int as size_t);
        sentinelFlushConfig();
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Sentinel ID is %s\0" as *const u8 as *const libc::c_char,
            (sentinel.myid).as_mut_ptr(),
        );
    }
    sentinelGenerateInitialMonitorEvents();
}
#[no_mangle]
pub unsafe extern "C" fn createSentinelAddr(
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut is_accept_unresolved: libc::c_int,
) -> *mut sentinelAddr {
    let mut ip: [libc::c_char; 46] = [0; 46];
    let mut sa: *mut sentinelAddr = 0 as *mut sentinelAddr;
    if port < 0 as libc::c_int || port > 65535 as libc::c_int {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut sentinelAddr;
    }
    if anetResolve(
        0 as *mut libc::c_char,
        hostname,
        ip.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
        (if sentinel.resolve_hostnames != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 0 as libc::c_int
        }),
    ) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed to resolve hostname '%s'\0" as *const u8 as *const libc::c_char,
                hostname,
            );
        }
        if sentinel.resolve_hostnames != 0 && is_accept_unresolved != 0 {
            ip[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        } else {
            *__errno_location() = 2 as libc::c_int;
            return 0 as *mut sentinelAddr;
        }
    }
    sa = zmalloc(core::mem::size_of::<sentinelAddr>() as libc::c_ulong)
        as *mut sentinelAddr;
    (*sa).hostname = sdsnew(hostname);
    (*sa).ip = sdsnew(ip.as_mut_ptr());
    (*sa).port = port;
    return sa;
}
#[no_mangle]
pub unsafe extern "C" fn dupSentinelAddr(
    mut src: *mut sentinelAddr,
) -> *mut sentinelAddr {
    let mut sa: *mut sentinelAddr = 0 as *mut sentinelAddr;
    sa = zmalloc(core::mem::size_of::<sentinelAddr>() as libc::c_ulong)
        as *mut sentinelAddr;
    (*sa).hostname = sdsnew((*src).hostname);
    (*sa).ip = sdsnew((*src).ip);
    (*sa).port = (*src).port;
    return sa;
}
#[no_mangle]
pub unsafe extern "C" fn releaseSentinelAddr(mut sa: *mut sentinelAddr) {
    sdsfree((*sa).hostname);
    sdsfree((*sa).ip);
    zfree(sa as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelAddrOrHostnameEqual(
    mut a: *mut sentinelAddr,
    mut b: *mut sentinelAddr,
) -> libc::c_int {
    return ((*a).port == (*b).port
        && (strcmp((*a).ip, (*b).ip) == 0
            || strcasecmp((*a).hostname, (*b).hostname) == 0)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelAddrEqualsHostname(
    mut a: *mut sentinelAddr,
    mut hostname: *mut libc::c_char,
) -> libc::c_int {
    let mut ip: [libc::c_char; 46] = [0; 46];
    if anetResolve(
        0 as *mut libc::c_char,
        hostname,
        ip.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
        (if sentinel.resolve_hostnames != 0 {
            0 as libc::c_int
        } else {
            (1 as libc::c_int) << 0 as libc::c_int
        }),
    ) == -(1 as libc::c_int)
    {
        return (strcasecmp(
            if sentinel.resolve_hostnames != 0 { (*a).hostname } else { (*a).ip },
            hostname,
        ) == 0) as libc::c_int;
    }
    return (strcasecmp((*a).ip, ip.as_mut_ptr()) == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn announceSentinelAddr(
    mut a: *const sentinelAddr,
) -> *const libc::c_char {
    return if sentinel.announce_hostnames != 0 { (*a).hostname } else { (*a).ip };
}
#[no_mangle]
pub unsafe extern "C" fn announceSentinelAddrAndPort(mut a: *const sentinelAddr) -> sds {
    let mut addr: *const libc::c_char = announceSentinelAddr(a);
    if !(strchr(addr, ':' as i32)).is_null() {
        return sdscatprintf(
            sdsempty(),
            b"[%s]:%d\0" as *const u8 as *const libc::c_char,
            addr,
            (*a).port,
        )
    } else {
        return sdscatprintf(
            sdsempty(),
            b"%s:%d\0" as *const u8 as *const libc::c_char,
            addr,
            (*a).port,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelEvent(
    mut level: libc::c_int,
    mut type_0: *mut libc::c_char,
    mut ri: *mut sentinelRedisInstance,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut channel: *mut robj = 0 as *mut robj;
    let mut payload: *mut robj = 0 as *mut robj;
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
        && *fmt.offset(1 as libc::c_int as isize) as libc::c_int == '@' as i32
    {
        let mut master: *mut sentinelRedisInstance = if (*ri).flags
            & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            0 as *mut sentinelRedisInstance
        } else {
            (*ri).master
        };
        if !master.is_null() {
            snprintf(
                msg.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%s %s %s %d @ %s %s %d\0" as *const u8 as *const libc::c_char,
                sentinelRedisInstanceTypeStr(ri),
                (*ri).name,
                announceSentinelAddr((*ri).addr),
                (*(*ri).addr).port,
                (*master).name,
                announceSentinelAddr((*master).addr),
                (*(*master).addr).port,
            );
        } else {
            snprintf(
                msg.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                b"%s %s %s %d\0" as *const u8 as *const libc::c_char,
                sentinelRedisInstanceTypeStr(ri),
                (*ri).name,
                announceSentinelAddr((*ri).addr),
                (*(*ri).addr).port,
            );
        }
        fmt = fmt.offset(2 as libc::c_int as isize);
    } else {
        msg[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        ap = args.clone();
        vsnprintf(
            msg.as_mut_ptr().offset(strlen(msg.as_mut_ptr()) as isize),
            (core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
                .wrapping_sub(strlen(msg.as_mut_ptr())),
            fmt,
            ap.as_va_list(),
        );
    }
    if level >= server.verbosity {
        if !((level & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                level,
                b"%s %s\0" as *const u8 as *const libc::c_char,
                type_0,
                msg.as_mut_ptr(),
            );
        }
    }
    if level != 0 as libc::c_int {
        channel = createStringObject(type_0, strlen(type_0));
        payload = createStringObject(msg.as_mut_ptr(), strlen(msg.as_mut_ptr()));
        pubsubPublishMessage(channel, payload, 0 as libc::c_int);
        decrRefCount(channel);
        decrRefCount(payload);
    }
    if level == 3 as libc::c_int && !ri.is_null() {
        let mut master_0: *mut sentinelRedisInstance = if (*ri).flags
            & (1 as libc::c_int) << 0 as libc::c_int != 0
        {
            ri
        } else {
            (*ri).master
        };
        if !master_0.is_null() && !((*master_0).notification_script).is_null() {
            sentinelScheduleScriptExecution(
                (*master_0).notification_script,
                type_0,
                msg.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGenerateInitialMonitorEvents() {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        sentinelEvent(
            3 as libc::c_int,
            b"+monitor\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ri,
            b"%@ quorum %d\0" as *const u8 as *const libc::c_char,
            (*ri).quorum,
        );
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelReleaseScriptJob(mut sj: *mut sentinelScriptJob) {
    let mut j: libc::c_int = 0 as libc::c_int;
    while !(*((*sj).argv).offset(j as isize)).is_null() {
        let fresh0 = j;
        j = j + 1;
        sdsfree(*((*sj).argv).offset(fresh0 as isize));
    }
    zfree((*sj).argv as *mut libc::c_void);
    zfree(sj as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelScheduleScriptExecution(
    mut path: *mut libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    let mut argv: [*mut libc::c_char; 17] = [0 as *mut libc::c_char; 17];
    let mut argc: libc::c_int = 1 as libc::c_int;
    let mut sj: *mut sentinelScriptJob = 0 as *mut sentinelScriptJob;
    ap = args.clone();
    while argc < 16 as libc::c_int {
        argv[argc as usize] = ap.arg::<*mut libc::c_char>();
        if (argv[argc as usize]).is_null() {
            break;
        }
        argv[argc as usize] = sdsnew(argv[argc as usize]);
        argc += 1;
    }
    argv[0 as libc::c_int as usize] = sdsnew(path);
    sj = zmalloc(core::mem::size_of::<sentinelScriptJob>() as libc::c_ulong)
        as *mut sentinelScriptJob;
    (*sj).flags = 0 as libc::c_int;
    (*sj).retry_num = 0 as libc::c_int;
    (*sj)
        .argv = zmalloc(
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    (*sj).start_time = 0 as libc::c_int as mstime_t;
    (*sj).pid = 0 as libc::c_int;
    memcpy(
        (*sj).argv as *mut libc::c_void,
        argv.as_mut_ptr() as *const libc::c_void,
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
    );
    listAddNodeTail(sentinel.scripts_queue, sj as *mut libc::c_void);
    if (*sentinel.scripts_queue).len > 256 as libc::c_int as libc::c_ulong {
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        listRewind(sentinel.scripts_queue, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            sj = (*ln).value as *mut sentinelScriptJob;
            if (*sj).flags & 1 as libc::c_int != 0 {
                continue;
            }
            listDelNode(sentinel.scripts_queue, ln);
            sentinelReleaseScriptJob(sj);
            break;
        }
        if (*sentinel.scripts_queue).len <= 256 as libc::c_int as libc::c_ulong {} else {
            _serverAssert(
                b"listLength(sentinel.scripts_queue) <= SENTINEL_SCRIPT_MAX_QUEUE\0"
                    as *const u8 as *const libc::c_char,
                b"sentinel.c\0" as *const u8 as *const libc::c_char,
                795 as libc::c_int,
            );
            unreachable!();
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGetScriptListNodeByPid(
    mut pid: pid_t,
) -> *mut listNode {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(sentinel.scripts_queue, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut sj: *mut sentinelScriptJob = (*ln).value as *mut sentinelScriptJob;
        if (*sj).flags & 1 as libc::c_int != 0 && (*sj).pid == pid {
            return ln;
        }
    }
    return 0 as *mut listNode;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRunPendingScripts() {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut now: mstime_t = mstime();
    listRewind(sentinel.scripts_queue, &mut li);
    while sentinel.running_scripts < 16 as libc::c_int
        && {
            ln = listNext(&mut li);
            !ln.is_null()
        }
    {
        let mut sj: *mut sentinelScriptJob = (*ln).value as *mut sentinelScriptJob;
        let mut pid: pid_t = 0;
        if (*sj).flags & 1 as libc::c_int != 0 {
            continue;
        }
        if (*sj).start_time != 0 && (*sj).start_time > now {
            continue;
        }
        (*sj).flags |= 1 as libc::c_int;
        (*sj).start_time = mstime();
        (*sj).retry_num += 1;
        pid = fork();
        if pid == -(1 as libc::c_int) {
            sentinelEvent(
                3 as libc::c_int,
                b"-script-error\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut sentinelRedisInstance,
                b"%s %d %d\0" as *const u8 as *const libc::c_char,
                *((*sj).argv).offset(0 as libc::c_int as isize),
                99 as libc::c_int,
                0 as libc::c_int,
            );
            (*sj).flags &= !(1 as libc::c_int);
            (*sj).pid = 0 as libc::c_int;
        } else if pid == 0 as libc::c_int {
            tlsCleanup();
            execve(
                *((*sj).argv).offset(0 as libc::c_int as isize),
                (*sj).argv as *const *mut libc::c_char,
                environ as *const *mut libc::c_char,
            );
            _exit(2 as libc::c_int);
        } else {
            sentinel.running_scripts += 1;
            (*sj).pid = pid;
            sentinelEvent(
                0 as libc::c_int,
                b"+script-child\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut sentinelRedisInstance,
                b"%ld\0" as *const u8 as *const libc::c_char,
                pid as libc::c_long,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelScriptRetryDelay(
    mut retry_num: libc::c_int,
) -> mstime_t {
    let mut delay: mstime_t = sentinel_script_retry_delay;
    loop {
        let fresh1 = retry_num;
        retry_num = retry_num - 1;
        if !(fresh1 > 1 as libc::c_int) {
            break;
        }
        delay *= 2 as libc::c_int as libc::c_longlong;
    }
    return delay;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCollectTerminatedScripts() {
    let mut statloc: libc::c_int = 0;
    let mut pid: pid_t = 0;
    loop {
        pid = waitpid(-(1 as libc::c_int), &mut statloc, 1 as libc::c_int);
        if !(pid > 0 as libc::c_int) {
            break;
        }
        let mut exitcode: libc::c_int = (statloc & 0xff00 as libc::c_int)
            >> 8 as libc::c_int;
        let mut bysignal: libc::c_int = 0 as libc::c_int;
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut sj: *mut sentinelScriptJob = 0 as *mut sentinelScriptJob;
        if ((statloc & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            bysignal = statloc & 0x7f as libc::c_int;
        }
        sentinelEvent(
            0 as libc::c_int,
            b"-script-child\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut sentinelRedisInstance,
            b"%ld %d %d\0" as *const u8 as *const libc::c_char,
            pid as libc::c_long,
            exitcode,
            bysignal,
        );
        ln = sentinelGetScriptListNodeByPid(pid);
        if ln.is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"waitpid() returned a pid (%ld) we can't find in our scripts execution queue!\0"
                        as *const u8 as *const libc::c_char,
                    pid as libc::c_long,
                );
            }
        } else {
            sj = (*ln).value as *mut sentinelScriptJob;
            if (bysignal != 0 || exitcode == 1 as libc::c_int)
                && (*sj).retry_num != 10 as libc::c_int
            {
                (*sj).flags &= !(1 as libc::c_int);
                (*sj).pid = 0 as libc::c_int;
                (*sj).start_time = mstime() + sentinelScriptRetryDelay((*sj).retry_num);
            } else {
                if bysignal != 0 || exitcode != 0 as libc::c_int {
                    sentinelEvent(
                        3 as libc::c_int,
                        b"-script-error\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut sentinelRedisInstance,
                        b"%s %d %d\0" as *const u8 as *const libc::c_char,
                        *((*sj).argv).offset(0 as libc::c_int as isize),
                        bysignal,
                        exitcode,
                    );
                }
                listDelNode(sentinel.scripts_queue, ln);
                sentinelReleaseScriptJob(sj);
            }
            sentinel.running_scripts -= 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelKillTimedoutScripts() {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut now: mstime_t = mstime();
    listRewind(sentinel.scripts_queue, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut sj: *mut sentinelScriptJob = (*ln).value as *mut sentinelScriptJob;
        if (*sj).flags & 1 as libc::c_int != 0
            && now - (*sj).start_time > sentinel_script_max_runtime
        {
            sentinelEvent(
                3 as libc::c_int,
                b"-script-timeout\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut sentinelRedisInstance,
                b"%s %ld\0" as *const u8 as *const libc::c_char,
                *((*sj).argv).offset(0 as libc::c_int as isize),
                (*sj).pid as libc::c_long,
            );
            kill((*sj).pid, 9 as libc::c_int);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelPendingScriptsCommand(mut c: *mut client) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    addReplyArrayLen(c, (*sentinel.scripts_queue).len as libc::c_long);
    listRewind(sentinel.scripts_queue, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut sj: *mut sentinelScriptJob = (*ln).value as *mut sentinelScriptJob;
        let mut j: libc::c_int = 0 as libc::c_int;
        addReplyMapLen(c, 5 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, b"argv\0" as *const u8 as *const libc::c_char);
        while !(*((*sj).argv).offset(j as isize)).is_null() {
            j += 1;
        }
        addReplyArrayLen(c, j as libc::c_long);
        j = 0 as libc::c_int;
        while !(*((*sj).argv).offset(j as isize)).is_null() {
            let fresh2 = j;
            j = j + 1;
            addReplyBulkCString(c, *((*sj).argv).offset(fresh2 as isize));
        }
        addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if (*sj).flags & 1 as libc::c_int != 0 {
                b"running\0" as *const u8 as *const libc::c_char
            } else {
                b"scheduled\0" as *const u8 as *const libc::c_char
            },
        );
        addReplyBulkCString(c, b"pid\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*sj).pid as libc::c_longlong);
        if (*sj).flags & 1 as libc::c_int != 0 {
            addReplyBulkCString(c, b"run-time\0" as *const u8 as *const libc::c_char);
            addReplyBulkLongLong(c, mstime() - (*sj).start_time);
        } else {
            let mut delay: mstime_t = if (*sj).start_time != 0 {
                (*sj).start_time - mstime()
            } else {
                0 as libc::c_int as libc::c_longlong
            };
            if delay < 0 as libc::c_int as libc::c_longlong {
                delay = 0 as libc::c_int as mstime_t;
            }
            addReplyBulkCString(c, b"run-delay\0" as *const u8 as *const libc::c_char);
            addReplyBulkLongLong(c, delay);
        }
        addReplyBulkCString(c, b"retry-num\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*sj).retry_num as libc::c_longlong);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCallClientReconfScript(
    mut master: *mut sentinelRedisInstance,
    mut role: libc::c_int,
    mut state: *mut libc::c_char,
    mut from: *mut sentinelAddr,
    mut to: *mut sentinelAddr,
) {
    let mut fromport: [libc::c_char; 32] = [0; 32];
    let mut toport: [libc::c_char; 32] = [0; 32];
    if ((*master).client_reconfig_script).is_null() {
        return;
    }
    ll2string(
        fromport.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*from).port as libc::c_longlong,
    );
    ll2string(
        toport.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        (*to).port as libc::c_longlong,
    );
    sentinelScheduleScriptExecution(
        (*master).client_reconfig_script,
        (*master).name,
        if role == (1 as libc::c_int) << 17 as libc::c_int {
            b"leader\0" as *const u8 as *const libc::c_char
        } else {
            b"observer\0" as *const u8 as *const libc::c_char
        },
        state,
        announceSentinelAddr(from),
        fromport.as_mut_ptr(),
        announceSentinelAddr(to),
        toport.as_mut_ptr(),
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn createInstanceLink() -> *mut instanceLink {
    let mut link: *mut instanceLink = zmalloc(
        core::mem::size_of::<instanceLink>() as libc::c_ulong,
    ) as *mut instanceLink;
    (*link).refcount = 1 as libc::c_int;
    (*link).disconnected = 1 as libc::c_int;
    (*link).pending_commands = 0 as libc::c_int;
    (*link).cc = 0 as *mut redisAsyncContext;
    (*link).pc = 0 as *mut redisAsyncContext;
    (*link).cc_conn_time = 0 as libc::c_int as mstime_t;
    (*link).pc_conn_time = 0 as libc::c_int as mstime_t;
    (*link).last_reconn_time = 0 as libc::c_int as mstime_t;
    (*link).pc_last_activity = 0 as libc::c_int as mstime_t;
    (*link).act_ping_time = mstime();
    (*link).last_ping_time = 0 as libc::c_int as mstime_t;
    (*link).last_avail_time = mstime();
    (*link).last_pong_time = mstime();
    return link;
}
#[no_mangle]
pub unsafe extern "C" fn instanceLinkCloseConnection(
    mut link: *mut instanceLink,
    mut c: *mut redisAsyncContext,
) {
    if c.is_null() {
        return;
    }
    if (*link).cc == c {
        (*link).cc = 0 as *mut redisAsyncContext;
        (*link).pending_commands = 0 as libc::c_int;
    }
    if (*link).pc == c {
        (*link).pc = 0 as *mut redisAsyncContext;
    }
    (*c).data = 0 as *mut libc::c_void;
    (*link).disconnected = 1 as libc::c_int;
    redisAsyncFree(c);
}
#[no_mangle]
pub unsafe extern "C" fn releaseInstanceLink(
    mut link: *mut instanceLink,
    mut ri: *mut sentinelRedisInstance,
) -> *mut instanceLink {
    if (*link).refcount > 0 as libc::c_int {} else {
        _serverAssert(
            b"link->refcount > 0\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1064 as libc::c_int,
        );
        unreachable!();
    };
    (*link).refcount -= 1;
    if (*link).refcount != 0 as libc::c_int {
        if !ri.is_null() && !((*(*ri).link).cc).is_null() {
            let mut cb: *mut redisCallback = 0 as *mut redisCallback;
            let mut callbacks: *mut redisCallbackList = &mut (*(*link).cc).replies;
            cb = (*callbacks).head;
            while !cb.is_null() {
                if (*cb).privdata == ri as *mut libc::c_void {
                    (*cb)
                        .fn_0 = Some(
                        sentinelDiscardReplyCallback
                            as unsafe extern "C" fn(
                                *mut redisAsyncContext,
                                *mut libc::c_void,
                                *mut libc::c_void,
                            ) -> (),
                    );
                    (*cb).privdata = 0 as *mut libc::c_void;
                }
                cb = (*cb).next;
            }
        }
        return link;
    }
    instanceLinkCloseConnection(link, (*link).cc);
    instanceLinkCloseConnection(link, (*link).pc);
    zfree(link as *mut libc::c_void);
    return 0 as *mut instanceLink;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelTryConnectionSharing(
    mut ri: *mut sentinelRedisInstance,
) -> libc::c_int {
    if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_SENTINEL\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1107 as libc::c_int,
        );
        unreachable!();
    };
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    if ((*ri).runid).is_null() {
        return -(1 as libc::c_int);
    }
    if (*(*ri).link).refcount > 1 as libc::c_int {
        return -(1 as libc::c_int);
    }
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut master: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        let mut match_0: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        if master == (*ri).master {
            continue;
        }
        match_0 = getSentinelRedisInstanceByAddrAndRunID(
            (*master).sentinels,
            0 as *mut libc::c_char,
            0 as libc::c_int,
            (*ri).runid,
        );
        if match_0.is_null() {
            continue;
        }
        if match_0 == ri {
            continue;
        }
        releaseInstanceLink((*ri).link, 0 as *mut sentinelRedisInstance);
        (*ri).link = (*match_0).link;
        (*(*match_0).link).refcount += 1;
        dictReleaseIterator(di);
        return 0 as libc::c_int;
    }
    dictReleaseIterator(di);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dropInstanceConnections(mut ri: *mut sentinelRedisInstance) {
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_MASTER\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1139 as libc::c_int,
        );
        unreachable!();
    };
    instanceLinkCloseConnection((*ri).link, (*(*ri).link).cc);
    instanceLinkCloseConnection((*ri).link, (*(*ri).link).pc);
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut repl_ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    di = dictGetIterator((*ri).slaves);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        repl_ri = (*de).v.val as *mut sentinelRedisInstance;
        instanceLinkCloseConnection((*repl_ri).link, (*(*repl_ri).link).cc);
        instanceLinkCloseConnection((*repl_ri).link, (*(*repl_ri).link).pc);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelDropConnections() -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut dropped: libc::c_int = 0 as libc::c_int;
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut sdi: *mut dictIterator = 0 as *mut dictIterator;
        let mut sde: *mut dictEntry = 0 as *mut dictEntry;
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        sdi = dictGetIterator((*ri).sentinels);
        loop {
            sde = dictNext(sdi);
            if sde.is_null() {
                break;
            }
            let mut si: *mut sentinelRedisInstance = (*sde).v.val
                as *mut sentinelRedisInstance;
            if (*(*si).link).disconnected == 0 {
                instanceLinkCloseConnection((*si).link, (*(*si).link).pc);
                instanceLinkCloseConnection((*si).link, (*(*si).link).cc);
                dropped += 1;
            }
        }
        dictReleaseIterator(sdi);
    }
    dictReleaseIterator(di);
    return dropped;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelUpdateSentinelAddressInAllMasters(
    mut ri: *mut sentinelRedisInstance,
) -> libc::c_int {
    if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_SENTINEL\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1194 as libc::c_int,
        );
        unreachable!();
    };
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut reconfigured: libc::c_int = 0 as libc::c_int;
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut master: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        let mut match_0: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        match_0 = getSentinelRedisInstanceByAddrAndRunID(
            (*master).sentinels,
            0 as *mut libc::c_char,
            0 as libc::c_int,
            (*ri).runid,
        );
        if match_0.is_null() {
            continue;
        }
        if !((*(*match_0).link).cc).is_null() {
            instanceLinkCloseConnection((*match_0).link, (*(*match_0).link).cc);
        }
        if !((*(*match_0).link).pc).is_null() {
            instanceLinkCloseConnection((*match_0).link, (*(*match_0).link).pc);
        }
        if match_0 == ri {
            continue;
        }
        releaseSentinelAddr((*match_0).addr);
        (*match_0).addr = dupSentinelAddr((*ri).addr);
        reconfigured += 1;
    }
    dictReleaseIterator(di);
    if reconfigured != 0 {
        sentinelEvent(
            2 as libc::c_int,
            b"+sentinel-address-update\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ri,
            b"%@ %d additional matching instances\0" as *const u8 as *const libc::c_char,
            reconfigured,
        );
    }
    return reconfigured;
}
#[no_mangle]
pub unsafe extern "C" fn instanceLinkConnectionError(mut c: *const redisAsyncContext) {
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    let mut pubsub: libc::c_int = 0;
    if link.is_null() {
        return;
    }
    pubsub = ((*link).pc == c as *mut redisAsyncContext) as libc::c_int;
    if pubsub != 0 {
        (*link).pc = 0 as *mut redisAsyncContext;
    } else {
        (*link).cc = 0 as *mut redisAsyncContext;
    }
    (*link).disconnected = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelLinkEstablishedCallback(
    mut c: *const redisAsyncContext,
    mut status: libc::c_int,
) {
    if status != 0 as libc::c_int {
        instanceLinkConnectionError(c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelDisconnectCallback(
    mut c: *const redisAsyncContext,
    mut status: libc::c_int,
) {
    instanceLinkConnectionError(c);
}
#[no_mangle]
pub unsafe extern "C" fn createSentinelRedisInstance(
    mut name: *mut libc::c_char,
    mut flags: libc::c_int,
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
    mut quorum: libc::c_int,
    mut master: *mut sentinelRedisInstance,
) -> *mut sentinelRedisInstance {
    let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
    let mut table: *mut dict = 0 as *mut dict;
    let mut sdsname: sds = 0 as *mut libc::c_char;
    if flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int) != 0
    {} else {
        _serverAssert(
            b"flags & (SRI_MASTER|SRI_SLAVE|SRI_SENTINEL)\0" as *const u8
                as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1290 as libc::c_int,
        );
        unreachable!();
    };
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 || !master.is_null() {} else {
        _serverAssert(
            b"(flags & SRI_MASTER) || master != NULL\0" as *const u8
                as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1291 as libc::c_int,
        );
        unreachable!();
    };
    addr = createSentinelAddr(hostname, port, 1 as libc::c_int);
    if addr.is_null() {
        return 0 as *mut sentinelRedisInstance;
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        sdsname = announceSentinelAddrAndPort(addr);
    } else {
        sdsname = sdsnew(name);
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        table = sentinel.masters;
    } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        table = (*master).slaves;
    } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        table = (*master).sentinels;
    }
    if !(dictFind(table, sdsname as *const libc::c_void)).is_null() {
        releaseSentinelAddr(addr);
        sdsfree(sdsname);
        *__errno_location() = 16 as libc::c_int;
        return 0 as *mut sentinelRedisInstance;
    }
    ri = zmalloc(core::mem::size_of::<sentinelRedisInstance>() as libc::c_ulong)
        as *mut sentinelRedisInstance;
    (*ri).flags = flags;
    (*ri).name = sdsname;
    (*ri).runid = 0 as *mut libc::c_char;
    (*ri).config_epoch = 0 as libc::c_int as uint64_t;
    (*ri).addr = addr;
    (*ri).link = createInstanceLink();
    (*ri).last_pub_time = mstime();
    (*ri).last_hello_time = mstime();
    (*ri).last_master_down_reply_time = mstime();
    (*ri).s_down_since_time = 0 as libc::c_int as mstime_t;
    (*ri).o_down_since_time = 0 as libc::c_int as mstime_t;
    (*ri)
        .down_after_period = if !master.is_null() {
        (*master).down_after_period
    } else {
        sentinel_default_down_after
    };
    (*ri).master_reboot_down_after_period = 0 as libc::c_int as mstime_t;
    (*ri).master_link_down_time = 0 as libc::c_int as mstime_t;
    (*ri).auth_pass = 0 as *mut libc::c_char;
    (*ri).auth_user = 0 as *mut libc::c_char;
    (*ri).slave_priority = 100 as libc::c_int;
    (*ri).replica_announced = 1 as libc::c_int;
    (*ri).slave_reconf_sent_time = 0 as libc::c_int as mstime_t;
    (*ri).slave_master_host = 0 as *mut libc::c_char;
    (*ri).slave_master_port = 0 as libc::c_int;
    (*ri).slave_master_link_status = 1 as libc::c_int;
    (*ri).slave_repl_offset = 0 as libc::c_int as libc::c_ulonglong;
    (*ri).sentinels = dictCreate(&mut instancesDictType);
    (*ri).quorum = quorum as libc::c_uint;
    (*ri).parallel_syncs = 1 as libc::c_int;
    (*ri).master = master;
    (*ri).slaves = dictCreate(&mut instancesDictType);
    (*ri).info_refresh = 0 as libc::c_int as mstime_t;
    (*ri).renamed_commands = dictCreate(&mut renamedCommandsDictType);
    (*ri).leader = 0 as *mut libc::c_char;
    (*ri).leader_epoch = 0 as libc::c_int as uint64_t;
    (*ri).failover_epoch = 0 as libc::c_int as uint64_t;
    (*ri).failover_state = 0 as libc::c_int;
    (*ri).failover_state_change_time = 0 as libc::c_int as mstime_t;
    (*ri).failover_start_time = 0 as libc::c_int as mstime_t;
    (*ri).failover_timeout = sentinel_default_failover_timeout;
    (*ri).failover_delay_logged = 0 as libc::c_int as mstime_t;
    (*ri).promoted_slave = 0 as *mut sentinelRedisInstance;
    (*ri).notification_script = 0 as *mut libc::c_char;
    (*ri).client_reconfig_script = 0 as *mut libc::c_char;
    (*ri).info = 0 as sds;
    (*ri)
        .role_reported = (*ri).flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int);
    (*ri).role_reported_time = mstime();
    (*ri).slave_conf_change_time = mstime();
    dictAdd(table, (*ri).name as *mut libc::c_void, ri as *mut libc::c_void);
    return ri;
}
#[no_mangle]
pub unsafe extern "C" fn releaseSentinelRedisInstance(
    mut ri: *mut sentinelRedisInstance,
) {
    dictRelease((*ri).sentinels);
    dictRelease((*ri).slaves);
    releaseInstanceLink((*ri).link, ri);
    sdsfree((*ri).name);
    sdsfree((*ri).runid);
    sdsfree((*ri).notification_script);
    sdsfree((*ri).client_reconfig_script);
    sdsfree((*ri).slave_master_host);
    sdsfree((*ri).leader);
    sdsfree((*ri).auth_pass);
    sdsfree((*ri).auth_user);
    sdsfree((*ri).info);
    releaseSentinelAddr((*ri).addr);
    dictRelease((*ri).renamed_commands);
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && (*ri).flags & (1 as libc::c_int) << 7 as libc::c_int != 0
        && !((*ri).master).is_null()
    {
        (*(*ri).master).promoted_slave = 0 as *mut sentinelRedisInstance;
    }
    zfree(ri as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRedisInstanceLookupSlave(
    mut ri: *mut sentinelRedisInstance,
    mut slave_addr: *mut libc::c_char,
    mut port: libc::c_int,
) -> *mut sentinelRedisInstance {
    let mut key: sds = 0 as *mut libc::c_char;
    let mut slave: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_MASTER\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1416 as libc::c_int,
        );
        unreachable!();
    };
    addr = createSentinelAddr(slave_addr, port, 0 as libc::c_int);
    if addr.is_null() {
        return 0 as *mut sentinelRedisInstance;
    }
    key = announceSentinelAddrAndPort(addr);
    releaseSentinelAddr(addr);
    slave = dictFetchValue((*ri).slaves, key as *const libc::c_void)
        as *mut sentinelRedisInstance;
    sdsfree(key);
    return slave;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRedisInstanceTypeStr(
    mut ri: *mut sentinelRedisInstance,
) -> *const libc::c_char {
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        return b"master\0" as *const u8 as *const libc::c_char
    } else if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return b"slave\0" as *const u8 as *const libc::c_char
    } else if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return b"sentinel\0" as *const u8 as *const libc::c_char
    } else {
        return b"unknown\0" as *const u8 as *const libc::c_char
    };
}
#[no_mangle]
pub unsafe extern "C" fn removeMatchingSentinelFromMaster(
    mut master: *mut sentinelRedisInstance,
    mut runid: *mut libc::c_char,
) -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut removed: libc::c_int = 0 as libc::c_int;
    if runid.is_null() {
        return 0 as libc::c_int;
    }
    di = dictGetSafeIterator((*master).sentinels);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if !((*ri).runid).is_null() && strcmp((*ri).runid, runid) == 0 as libc::c_int {
            dictDelete((*master).sentinels, (*ri).name as *const libc::c_void);
            removed += 1;
        }
    }
    dictReleaseIterator(di);
    return removed;
}
#[no_mangle]
pub unsafe extern "C" fn getSentinelRedisInstanceByAddrAndRunID(
    mut instances: *mut dict,
    mut addr: *mut libc::c_char,
    mut port: libc::c_int,
    mut runid: *mut libc::c_char,
) -> *mut sentinelRedisInstance {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut instance: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut ri_addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
    if !addr.is_null() || !runid.is_null() {} else {
        _serverAssert(
            b"addr || runid\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1483 as libc::c_int,
        );
        unreachable!();
    };
    if !addr.is_null() {
        ri_addr = createSentinelAddr(addr, port, 1 as libc::c_int);
        if ri_addr.is_null() {
            return 0 as *mut sentinelRedisInstance;
        }
    }
    di = dictGetIterator(instances);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if !runid.is_null() && ((*ri).runid).is_null() {
            continue;
        }
        if !((runid.is_null() || strcmp((*ri).runid, runid) == 0 as libc::c_int)
            && (addr.is_null() || sentinelAddrOrHostnameEqual((*ri).addr, ri_addr) != 0))
        {
            continue;
        }
        instance = ri;
        break;
    }
    dictReleaseIterator(di);
    if !ri_addr.is_null() {
        releaseSentinelAddr(ri_addr);
    }
    return instance;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGetMasterByName(
    mut name: *mut libc::c_char,
) -> *mut sentinelRedisInstance {
    let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut sdsname: sds = sdsnew(name);
    ri = dictFetchValue(sentinel.masters, sdsname as *const libc::c_void)
        as *mut sentinelRedisInstance;
    sdsfree(sdsname);
    return ri;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelResetMaster(
    mut ri: *mut sentinelRedisInstance,
    mut flags: libc::c_int,
) {
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_MASTER\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            1533 as libc::c_int,
        );
        unreachable!();
    };
    dictRelease((*ri).slaves);
    (*ri).slaves = dictCreate(&mut instancesDictType);
    if flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        dictRelease((*ri).sentinels);
        (*ri).sentinels = dictCreate(&mut instancesDictType);
    }
    instanceLinkCloseConnection((*ri).link, (*(*ri).link).cc);
    instanceLinkCloseConnection((*ri).link, (*(*ri).link).pc);
    (*ri).flags &= (1 as libc::c_int) << 0 as libc::c_int;
    if !((*ri).leader).is_null() {
        sdsfree((*ri).leader);
        (*ri).leader = 0 as *mut libc::c_char;
    }
    (*ri).failover_state = 0 as libc::c_int;
    (*ri).failover_state_change_time = 0 as libc::c_int as mstime_t;
    (*ri).failover_start_time = 0 as libc::c_int as mstime_t;
    (*ri).promoted_slave = 0 as *mut sentinelRedisInstance;
    sdsfree((*ri).runid);
    sdsfree((*ri).slave_master_host);
    (*ri).runid = 0 as *mut libc::c_char;
    (*ri).slave_master_host = 0 as *mut libc::c_char;
    (*(*ri).link).act_ping_time = mstime();
    (*(*ri).link).last_ping_time = 0 as libc::c_int as mstime_t;
    (*(*ri).link).last_avail_time = mstime();
    (*(*ri).link).last_pong_time = mstime();
    (*ri).role_reported_time = mstime();
    (*ri).role_reported = (1 as libc::c_int) << 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        sentinelEvent(
            3 as libc::c_int,
            b"+reset-master\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ri,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelResetMastersByPattern(
    mut pattern: *mut libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut reset: libc::c_int = 0 as libc::c_int;
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if !((*ri).name).is_null() {
            if stringmatch(pattern, (*ri).name, 0 as libc::c_int) != 0 {
                sentinelResetMaster(ri, flags);
                reset += 1;
            }
        }
    }
    dictReleaseIterator(di);
    return reset;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelResetMasterAndChangeAddress(
    mut master: *mut sentinelRedisInstance,
    mut hostname: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    let mut oldaddr: *mut sentinelAddr = 0 as *mut sentinelAddr;
    let mut newaddr: *mut sentinelAddr = 0 as *mut sentinelAddr;
    let mut slaves: *mut *mut sentinelAddr = 0 as *mut *mut sentinelAddr;
    let mut numslaves: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    newaddr = createSentinelAddr(hostname, port, 0 as libc::c_int);
    if newaddr.is_null() {
        return -(1 as libc::c_int);
    }
    slaves = zmalloc(
        (core::mem::size_of::<*mut sentinelAddr>() as libc::c_ulong)
            .wrapping_mul(
                ((*(*master).slaves).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*(*master).slaves).ht_used[1 as libc::c_int as usize])
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ),
    ) as *mut *mut sentinelAddr;
    di = dictGetIterator((*master).slaves);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut slave: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if sentinelAddrOrHostnameEqual((*slave).addr, newaddr) != 0 {
            continue;
        }
        let fresh3 = numslaves;
        numslaves = numslaves + 1;
        let ref mut fresh4 = *slaves.offset(fresh3 as isize);
        *fresh4 = dupSentinelAddr((*slave).addr);
    }
    dictReleaseIterator(di);
    if sentinelAddrOrHostnameEqual(newaddr, (*master).addr) == 0 {
        let fresh5 = numslaves;
        numslaves = numslaves + 1;
        let ref mut fresh6 = *slaves.offset(fresh5 as isize);
        *fresh6 = dupSentinelAddr((*master).addr);
    }
    sentinelResetMaster(master, (1 as libc::c_int) << 0 as libc::c_int);
    oldaddr = (*master).addr;
    (*master).addr = newaddr;
    (*master).o_down_since_time = 0 as libc::c_int as mstime_t;
    (*master).s_down_since_time = 0 as libc::c_int as mstime_t;
    j = 0 as libc::c_int;
    while j < numslaves {
        let mut slave_0: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        slave_0 = createSentinelRedisInstance(
            0 as *mut libc::c_char,
            (1 as libc::c_int) << 1 as libc::c_int,
            (**slaves.offset(j as isize)).hostname,
            (**slaves.offset(j as isize)).port,
            (*master).quorum as libc::c_int,
            master,
        );
        releaseSentinelAddr(*slaves.offset(j as isize));
        if !slave_0.is_null() {
            sentinelEvent(
                2 as libc::c_int,
                b"+slave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                slave_0,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
        }
        j += 1;
    }
    zfree(slaves as *mut libc::c_void);
    releaseSentinelAddr(oldaddr);
    sentinelFlushConfig();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRedisInstanceNoDownFor(
    mut ri: *mut sentinelRedisInstance,
    mut ms: mstime_t,
) -> libc::c_int {
    let mut most_recent: mstime_t = 0;
    most_recent = (*ri).s_down_since_time;
    if (*ri).o_down_since_time > most_recent {
        most_recent = (*ri).o_down_since_time;
    }
    return (most_recent == 0 as libc::c_int as libc::c_longlong
        || mstime() - most_recent > ms) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGetCurrentMasterAddress(
    mut master: *mut sentinelRedisInstance,
) -> *mut sentinelAddr {
    if (*master).flags & (1 as libc::c_int) << 6 as libc::c_int != 0
        && !((*master).promoted_slave).is_null()
        && (*master).failover_state >= 5 as libc::c_int
    {
        return (*(*master).promoted_slave).addr
    } else {
        return (*master).addr
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelPropagateDownAfterPeriod(
    mut master: *mut sentinelRedisInstance,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut j: libc::c_int = 0;
    let mut d: [*mut dict; 3] = [(*master).slaves, (*master).sentinels, 0 as *mut dict];
    j = 0 as libc::c_int;
    while !(d[j as usize]).is_null() {
        di = dictGetIterator(d[j as usize]);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut ri: *mut sentinelRedisInstance = (*de).v.val
                as *mut sentinelRedisInstance;
            (*ri).down_after_period = (*master).down_after_period;
        }
        dictReleaseIterator(di);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelInstanceMapCommand(
    mut ri: *mut sentinelRedisInstance,
    mut command: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut sc: sds = sdsnew(command);
    if !((*ri).master).is_null() {
        ri = (*ri).master;
    }
    let mut retval: *mut libc::c_char = dictFetchValue(
        (*ri).renamed_commands,
        sc as *const libc::c_void,
    ) as *mut libc::c_char;
    sdsfree(sc);
    return if !retval.is_null() { retval } else { command };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCheckCreateInstanceErrors(
    mut role: libc::c_int,
) -> *const libc::c_char {
    match *__errno_location() {
        16 => {
            match role {
                1 => {
                    return b"Duplicate master name.\0" as *const u8
                        as *const libc::c_char;
                }
                2 => {
                    return b"Duplicate hostname and port for replica.\0" as *const u8
                        as *const libc::c_char;
                }
                4 => {
                    return b"Duplicate runid for sentinel.\0" as *const u8
                        as *const libc::c_char;
                }
                _ => {
                    if 0 as libc::c_int != 0 {} else {
                        _serverAssert(
                            b"0\0" as *const u8 as *const libc::c_char,
                            b"sentinel.c\0" as *const u8 as *const libc::c_char,
                            1728 as libc::c_int,
                        );
                        unreachable!();
                    };
                }
            }
        }
        2 => {
            return b"Can't resolve instance hostname.\0" as *const u8
                as *const libc::c_char;
        }
        22 => return b"Invalid port number.\0" as *const u8 as *const libc::c_char,
        _ => {
            return b"Unknown Error for creating instances.\0" as *const u8
                as *const libc::c_char;
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn initializeSentinelConfig() {
    server
        .sentinel_config = zmalloc(
        core::mem::size_of::<sentinelConfig>() as libc::c_ulong,
    ) as *mut sentinelConfig;
    (*server.sentinel_config).monitor_cfg = listCreate();
    (*server.sentinel_config).pre_monitor_cfg = listCreate();
    (*server.sentinel_config).post_monitor_cfg = listCreate();
    (*(*server.sentinel_config).monitor_cfg)
        .free = Some(
        freeSentinelLoadQueueEntry as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*(*server.sentinel_config).pre_monitor_cfg)
        .free = Some(
        freeSentinelLoadQueueEntry as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*(*server.sentinel_config).post_monitor_cfg)
        .free = Some(
        freeSentinelLoadQueueEntry as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
}
#[no_mangle]
pub unsafe extern "C" fn freeSentinelConfig() {
    listRelease((*server.sentinel_config).pre_monitor_cfg);
    listRelease((*server.sentinel_config).monitor_cfg);
    listRelease((*server.sentinel_config).post_monitor_cfg);
    zfree(server.sentinel_config as *mut libc::c_void);
    server.sentinel_config = 0 as *mut sentinelConfig;
}
#[no_mangle]
pub unsafe extern "C" fn searchPreMonitorCfgName(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (core::mem::size_of::<[*const libc::c_char; 9]>() as libc::c_ulong)
            .wrapping_div(core::mem::size_of::<*const libc::c_char>() as libc::c_ulong)
    {
        if strcasecmp(preMonitorCfgName[i as usize], name) == 0 {
            return 1 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeSentinelLoadQueueEntry(mut item: *mut libc::c_void) {
    let mut entry: *mut sentinelLoadQueueEntry = item as *mut sentinelLoadQueueEntry;
    sdsfreesplitres((*entry).argv, (*entry).argc);
    sdsfree((*entry).line);
    zfree(entry as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn queueSentinelConfig(
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut linenum: libc::c_int,
    mut line: sds,
) {
    let mut i: libc::c_int = 0;
    let mut entry: *mut sentinelLoadQueueEntry = 0 as *mut sentinelLoadQueueEntry;
    if (server.sentinel_config).is_null() {
        initializeSentinelConfig();
    }
    entry = zmalloc(core::mem::size_of::<sentinelLoadQueueEntry>() as libc::c_ulong)
        as *mut sentinelLoadQueueEntry;
    (*entry)
        .argv = zmalloc(
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong),
    ) as *mut sds;
    (*entry).argc = argc;
    (*entry).linenum = linenum;
    (*entry).line = sdsdup(line);
    i = 0 as libc::c_int;
    while i < argc {
        let ref mut fresh7 = *((*entry).argv).offset(i as isize);
        *fresh7 = sdsdup(*argv.offset(i as isize));
        i += 1;
    }
    if strcasecmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"monitor\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        listAddNodeTail(
            (*server.sentinel_config).monitor_cfg,
            entry as *mut libc::c_void,
        );
    } else if searchPreMonitorCfgName(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
    ) != 0
    {
        listAddNodeTail(
            (*server.sentinel_config).pre_monitor_cfg,
            entry as *mut libc::c_void,
        );
    } else {
        listAddNodeTail(
            (*server.sentinel_config).post_monitor_cfg,
            entry as *mut libc::c_void,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn loadSentinelConfigFromQueue() {
    let mut current_block: u64;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut linenum: libc::c_int = 0 as libc::c_int;
    let mut line: sds = 0 as sds;
    let mut j: libc::c_uint = 0;
    if (server.sentinel_config).is_null() {
        return;
    }
    let mut sentinel_configs: [*mut list; 3] = [
        (*server.sentinel_config).pre_monitor_cfg,
        (*server.sentinel_config).monitor_cfg,
        (*server.sentinel_config).post_monitor_cfg,
    ];
    j = 0 as libc::c_int as libc::c_uint;
    's_23: loop {
        if !((j as libc::c_ulong)
            < (core::mem::size_of::<[*mut list; 3]>() as libc::c_ulong)
                .wrapping_div(core::mem::size_of::<*mut list>() as libc::c_ulong))
        {
            current_block = 1841672684692190573;
            break;
        }
        listRewind(sentinel_configs[j as usize], &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut entry: *mut sentinelLoadQueueEntry = (*ln).value
                as *mut sentinelLoadQueueEntry;
            err = sentinelHandleConfiguration((*entry).argv, (*entry).argc);
            if err.is_null() {
                continue;
            }
            linenum = (*entry).linenum;
            line = (*entry).line;
            current_block = 1516778841877903976;
            break 's_23;
        }
        j = j.wrapping_add(1);
    }
    match current_block {
        1516778841877903976 => {
            fprintf(
                stderr,
                b"\n*** FATAL CONFIG FILE ERROR (Redis %s) ***\n\0" as *const u8
                    as *const libc::c_char,
                b"7.0.8\0" as *const u8 as *const libc::c_char,
            );
            fprintf(
                stderr,
                b"Reading the configuration file, at line %d\n\0" as *const u8
                    as *const libc::c_char,
                linenum,
            );
            fprintf(stderr, b">>> '%s'\n\0" as *const u8 as *const libc::c_char, line);
            fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, err);
            exit(1 as libc::c_int);
        }
        _ => {
            freeSentinelConfig();
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelHandleConfiguration(
    mut argv: *mut *mut libc::c_char,
    mut argc: libc::c_int,
) -> *const libc::c_char {
    let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"monitor\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 5 as libc::c_int
    {
        let mut quorum: libc::c_int = atoi(*argv.offset(4 as libc::c_int as isize));
        if quorum <= 0 as libc::c_int {
            return b"Quorum must be 1 or greater.\0" as *const u8 as *const libc::c_char;
        }
        if (createSentinelRedisInstance(
            *argv.offset(1 as libc::c_int as isize),
            (1 as libc::c_int) << 0 as libc::c_int,
            *argv.offset(2 as libc::c_int as isize),
            atoi(*argv.offset(3 as libc::c_int as isize)),
            quorum,
            0 as *mut sentinelRedisInstance,
        ))
            .is_null()
        {
            return sentinelCheckCreateInstanceErrors(
                (1 as libc::c_int) << 0 as libc::c_int,
            );
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"down-after-milliseconds\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri)
            .down_after_period = atoi(*argv.offset(2 as libc::c_int as isize))
            as mstime_t;
        if (*ri).down_after_period <= 0 as libc::c_int as libc::c_longlong {
            return b"negative or zero time parameter.\0" as *const u8
                as *const libc::c_char;
        }
        sentinelPropagateDownAfterPeriod(ri);
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"failover-timeout\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri)
            .failover_timeout = atoi(*argv.offset(2 as libc::c_int as isize))
            as mstime_t;
        if (*ri).failover_timeout <= 0 as libc::c_int as libc::c_longlong {
            return b"negative or zero time parameter.\0" as *const u8
                as *const libc::c_char;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"parallel-syncs\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri).parallel_syncs = atoi(*argv.offset(2 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"notification-script\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        if access(*argv.offset(2 as libc::c_int as isize), 1 as libc::c_int)
            == -(1 as libc::c_int)
        {
            return b"Notification script seems non existing or non executable.\0"
                as *const u8 as *const libc::c_char;
        }
        (*ri).notification_script = sdsnew(*argv.offset(2 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"client-reconfig-script\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        if access(*argv.offset(2 as libc::c_int as isize), 1 as libc::c_int)
            == -(1 as libc::c_int)
        {
            return b"Client reconfiguration script seems non existing or non executable.\0"
                as *const u8 as *const libc::c_char;
        }
        (*ri).client_reconfig_script = sdsnew(*argv.offset(2 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"auth-pass\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri).auth_pass = sdsnew(*argv.offset(2 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"auth-user\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri).auth_user = sdsnew(*argv.offset(2 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"current-epoch\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        let mut current_epoch: libc::c_ulonglong = strtoull(
            *argv.offset(1 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        );
        if current_epoch > sentinel.current_epoch as libc::c_ulonglong {
            sentinel.current_epoch = current_epoch as uint64_t;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"myid\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        if strlen(*argv.offset(1 as libc::c_int as isize))
            != 40 as libc::c_int as libc::c_ulong
        {
            return b"Malformed Sentinel id in myid option.\0" as *const u8
                as *const libc::c_char;
        }
        memcpy(
            (sentinel.myid).as_mut_ptr() as *mut libc::c_void,
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"config-epoch\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri)
            .config_epoch = strtoull(
            *argv.offset(2 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as uint64_t;
        if (*ri).config_epoch > sentinel.current_epoch {
            sentinel.current_epoch = (*ri).config_epoch;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"leader-epoch\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri)
            .leader_epoch = strtoull(
            *argv.offset(2 as libc::c_int as isize),
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as uint64_t;
    } else if (strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"known-slave\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            *argv.offset(0 as libc::c_int as isize),
            b"known-replica\0" as *const u8 as *const libc::c_char,
        ) == 0) && argc == 4 as libc::c_int
    {
        let mut slave: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        slave = createSentinelRedisInstance(
            0 as *mut libc::c_char,
            (1 as libc::c_int) << 1 as libc::c_int,
            *argv.offset(2 as libc::c_int as isize),
            atoi(*argv.offset(3 as libc::c_int as isize)),
            (*ri).quorum as libc::c_int,
            ri,
        );
        if slave.is_null() {
            return sentinelCheckCreateInstanceErrors(
                (1 as libc::c_int) << 1 as libc::c_int,
            );
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"known-sentinel\0" as *const u8 as *const libc::c_char,
    ) == 0 && (argc == 4 as libc::c_int || argc == 5 as libc::c_int)
    {
        let mut si: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        if argc == 5 as libc::c_int {
            ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
            if ri.is_null() {
                return b"No such master with specified name.\0" as *const u8
                    as *const libc::c_char;
            }
            si = createSentinelRedisInstance(
                *argv.offset(4 as libc::c_int as isize),
                (1 as libc::c_int) << 2 as libc::c_int,
                *argv.offset(2 as libc::c_int as isize),
                atoi(*argv.offset(3 as libc::c_int as isize)),
                (*ri).quorum as libc::c_int,
                ri,
            );
            if si.is_null() {
                return sentinelCheckCreateInstanceErrors(
                    (1 as libc::c_int) << 2 as libc::c_int,
                );
            }
            (*si).runid = sdsnew(*argv.offset(4 as libc::c_int as isize));
            sentinelTryConnectionSharing(si);
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"rename-command\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 4 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        let mut oldcmd: sds = sdsnew(*argv.offset(2 as libc::c_int as isize));
        let mut newcmd: sds = sdsnew(*argv.offset(3 as libc::c_int as isize));
        if dictAdd(
            (*ri).renamed_commands,
            oldcmd as *mut libc::c_void,
            newcmd as *mut libc::c_void,
        ) != 0 as libc::c_int
        {
            sdsfree(oldcmd);
            sdsfree(newcmd);
            return b"Same command renamed multiple times with rename-command.\0"
                as *const u8 as *const libc::c_char;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"announce-ip\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        if strlen(*argv.offset(1 as libc::c_int as isize)) != 0 {
            sentinel.announce_ip = sdsnew(*argv.offset(1 as libc::c_int as isize));
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"announce-port\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        sentinel.announce_port = atoi(*argv.offset(1 as libc::c_int as isize));
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"deny-scripts-reconfig\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        sentinel
            .deny_scripts_reconfig = yesnotoi(*argv.offset(1 as libc::c_int as isize));
        if sentinel.deny_scripts_reconfig == -(1 as libc::c_int) {
            return b"Please specify yes or no for the deny-scripts-reconfig options.\0"
                as *const u8 as *const libc::c_char;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"sentinel-user\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        if strlen(*argv.offset(1 as libc::c_int as isize)) != 0 {
            sentinel
                .sentinel_auth_user = sdsnew(*argv.offset(1 as libc::c_int as isize));
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"sentinel-pass\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        if strlen(*argv.offset(1 as libc::c_int as isize)) != 0 {
            sentinel
                .sentinel_auth_pass = sdsnew(*argv.offset(1 as libc::c_int as isize));
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"resolve-hostnames\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        sentinel.resolve_hostnames = yesnotoi(*argv.offset(1 as libc::c_int as isize));
        if sentinel.resolve_hostnames == -(1 as libc::c_int) {
            return b"Please specify yes or no for the resolve-hostnames option.\0"
                as *const u8 as *const libc::c_char;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"announce-hostnames\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 2 as libc::c_int
    {
        sentinel.announce_hostnames = yesnotoi(*argv.offset(1 as libc::c_int as isize));
        if sentinel.announce_hostnames == -(1 as libc::c_int) {
            return b"Please specify yes or no for the announce-hostnames option.\0"
                as *const u8 as *const libc::c_char;
        }
    } else if strcasecmp(
        *argv.offset(0 as libc::c_int as isize),
        b"master-reboot-down-after-period\0" as *const u8 as *const libc::c_char,
    ) == 0 && argc == 3 as libc::c_int
    {
        ri = sentinelGetMasterByName(*argv.offset(1 as libc::c_int as isize));
        if ri.is_null() {
            return b"No such master with specified name.\0" as *const u8
                as *const libc::c_char;
        }
        (*ri)
            .master_reboot_down_after_period = atoi(
            *argv.offset(2 as libc::c_int as isize),
        ) as mstime_t;
        if (*ri).master_reboot_down_after_period < 0 as libc::c_int as libc::c_longlong {
            return b"negative time parameter.\0" as *const u8 as *const libc::c_char;
        }
    } else {
        return b"Unrecognized sentinel configuration statement.\0" as *const u8
            as *const libc::c_char
    }
    return 0 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigSentinelOption(
    mut state: *mut rewriteConfigState,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut di2: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut line: sds = 0 as *mut libc::c_char;
    line = sdscatprintf(
        sdsempty(),
        b"sentinel myid %s\0" as *const u8 as *const libc::c_char,
        (sentinel.myid).as_mut_ptr(),
    );
    rewriteConfigRewriteLine(
        state,
        b"sentinel myid\0" as *const u8 as *const libc::c_char,
        line,
        1 as libc::c_int,
    );
    line = sdscatprintf(
        sdsempty(),
        b"sentinel deny-scripts-reconfig %s\0" as *const u8 as *const libc::c_char,
        if sentinel.deny_scripts_reconfig != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    rewriteConfigRewriteLine(
        state,
        b"sentinel deny-scripts-reconfig\0" as *const u8 as *const libc::c_char,
        line,
        (sentinel.deny_scripts_reconfig != 1 as libc::c_int) as libc::c_int,
    );
    line = sdscatprintf(
        sdsempty(),
        b"sentinel resolve-hostnames %s\0" as *const u8 as *const libc::c_char,
        if sentinel.resolve_hostnames != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    rewriteConfigRewriteLine(
        state,
        b"sentinel resolve-hostnames\0" as *const u8 as *const libc::c_char,
        line,
        (sentinel.resolve_hostnames != 0 as libc::c_int) as libc::c_int,
    );
    line = sdscatprintf(
        sdsempty(),
        b"sentinel announce-hostnames %s\0" as *const u8 as *const libc::c_char,
        if sentinel.announce_hostnames != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    rewriteConfigRewriteLine(
        state,
        b"sentinel announce-hostnames\0" as *const u8 as *const libc::c_char,
        line,
        (sentinel.announce_hostnames != 0 as libc::c_int) as libc::c_int,
    );
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut master: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        let mut master_addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
        master = (*de).v.val as *mut sentinelRedisInstance;
        master_addr = sentinelGetCurrentMasterAddress(master);
        line = sdscatprintf(
            sdsempty(),
            b"sentinel monitor %s %s %d %d\0" as *const u8 as *const libc::c_char,
            (*master).name,
            announceSentinelAddr(master_addr),
            (*master_addr).port,
            (*master).quorum,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel monitor\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
        if (*master).down_after_period != sentinel_default_down_after {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel down-after-milliseconds %s %ld\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                (*master).down_after_period as libc::c_long,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel down-after-milliseconds\0" as *const u8
                    as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if (*master).failover_timeout != sentinel_default_failover_timeout {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel failover-timeout %s %ld\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                (*master).failover_timeout as libc::c_long,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel failover-timeout\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if (*master).parallel_syncs != 1 as libc::c_int {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel parallel-syncs %s %d\0" as *const u8 as *const libc::c_char,
                (*master).name,
                (*master).parallel_syncs,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel parallel-syncs\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if !((*master).notification_script).is_null() {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel notification-script %s %s\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                (*master).notification_script,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel notification-script\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if !((*master).client_reconfig_script).is_null() {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel client-reconfig-script %s %s\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                (*master).client_reconfig_script,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel client-reconfig-script\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if !((*master).auth_pass).is_null() {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel auth-pass %s %s\0" as *const u8 as *const libc::c_char,
                (*master).name,
                (*master).auth_pass,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel auth-pass\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if !((*master).auth_user).is_null() {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel auth-user %s %s\0" as *const u8 as *const libc::c_char,
                (*master).name,
                (*master).auth_user,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel auth-user\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        if (*master).master_reboot_down_after_period
            != 0 as libc::c_int as libc::c_longlong
        {
            line = sdscatprintf(
                sdsempty(),
                b"sentinel master-reboot-down-after-period %s %ld\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                (*master).master_reboot_down_after_period as libc::c_long,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel master-reboot-down-after-period\0" as *const u8
                    as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        line = sdscatprintf(
            sdsempty(),
            b"sentinel config-epoch %s %llu\0" as *const u8 as *const libc::c_char,
            (*master).name,
            (*master).config_epoch as libc::c_ulonglong,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel config-epoch\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
        line = sdscatprintf(
            sdsempty(),
            b"sentinel leader-epoch %s %llu\0" as *const u8 as *const libc::c_char,
            (*master).name,
            (*master).leader_epoch as libc::c_ulonglong,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel leader-epoch\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
        di2 = dictGetIterator((*master).slaves);
        loop {
            de = dictNext(di2);
            if de.is_null() {
                break;
            }
            let mut slave_addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
            ri = (*de).v.val as *mut sentinelRedisInstance;
            slave_addr = (*ri).addr;
            if sentinelAddrOrHostnameEqual(slave_addr, master_addr) != 0 {
                slave_addr = (*master).addr;
            }
            line = sdscatprintf(
                sdsempty(),
                b"sentinel known-replica %s %s %d\0" as *const u8 as *const libc::c_char,
                (*master).name,
                announceSentinelAddr(slave_addr),
                (*slave_addr).port,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel known-replica\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        dictReleaseIterator(di2);
        di2 = dictGetIterator((*master).sentinels);
        loop {
            de = dictNext(di2);
            if de.is_null() {
                break;
            }
            ri = (*de).v.val as *mut sentinelRedisInstance;
            if ((*ri).runid).is_null() {
                continue;
            }
            line = sdscatprintf(
                sdsempty(),
                b"sentinel known-sentinel %s %s %d %s\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                announceSentinelAddr((*ri).addr),
                (*(*ri).addr).port,
                (*ri).runid,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel known-sentinel\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        dictReleaseIterator(di2);
        di2 = dictGetIterator((*master).renamed_commands);
        loop {
            de = dictNext(di2);
            if de.is_null() {
                break;
            }
            let mut oldname: sds = (*de).key as sds;
            let mut newname: sds = (*de).v.val as sds;
            line = sdscatprintf(
                sdsempty(),
                b"sentinel rename-command %s %s %s\0" as *const u8
                    as *const libc::c_char,
                (*master).name,
                oldname,
                newname,
            );
            rewriteConfigRewriteLine(
                state,
                b"sentinel rename-command\0" as *const u8 as *const libc::c_char,
                line,
                1 as libc::c_int,
            );
        }
        dictReleaseIterator(di2);
    }
    line = sdscatprintf(
        sdsempty(),
        b"sentinel current-epoch %llu\0" as *const u8 as *const libc::c_char,
        sentinel.current_epoch as libc::c_ulonglong,
    );
    rewriteConfigRewriteLine(
        state,
        b"sentinel current-epoch\0" as *const u8 as *const libc::c_char,
        line,
        1 as libc::c_int,
    );
    if !(sentinel.announce_ip).is_null() {
        line = sdsnew(b"sentinel announce-ip \0" as *const u8 as *const libc::c_char);
        line = sdscatrepr(line, sentinel.announce_ip, sdslen(sentinel.announce_ip));
        rewriteConfigRewriteLine(
            state,
            b"sentinel announce-ip\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    } else {
        rewriteConfigMarkAsProcessed(
            state,
            b"sentinel announce-ip\0" as *const u8 as *const libc::c_char,
        );
    }
    if sentinel.announce_port != 0 {
        line = sdscatprintf(
            sdsempty(),
            b"sentinel announce-port %d\0" as *const u8 as *const libc::c_char,
            sentinel.announce_port,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel announce-port\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    } else {
        rewriteConfigMarkAsProcessed(
            state,
            b"sentinel announce-port\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(sentinel.sentinel_auth_user).is_null() {
        line = sdscatprintf(
            sdsempty(),
            b"sentinel sentinel-user %s\0" as *const u8 as *const libc::c_char,
            sentinel.sentinel_auth_user,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel sentinel-user\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    } else {
        rewriteConfigMarkAsProcessed(
            state,
            b"sentinel sentinel-user\0" as *const u8 as *const libc::c_char,
        );
    }
    if !(sentinel.sentinel_auth_pass).is_null() {
        line = sdscatprintf(
            sdsempty(),
            b"sentinel sentinel-pass %s\0" as *const u8 as *const libc::c_char,
            sentinel.sentinel_auth_pass,
        );
        rewriteConfigRewriteLine(
            state,
            b"sentinel sentinel-pass\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    } else {
        rewriteConfigMarkAsProcessed(
            state,
            b"sentinel sentinel-pass\0" as *const u8 as *const libc::c_char,
        );
    }
    dictReleaseIterator(di);
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel monitor\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel down-after-milliseconds\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel failover-timeout\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel parallel-syncs\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel notification-script\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel client-reconfig-script\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel auth-pass\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel auth-user\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel config-epoch\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel leader-epoch\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel known-replica\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel known-sentinel\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel rename-command\0" as *const u8 as *const libc::c_char,
    );
    rewriteConfigMarkAsProcessed(
        state,
        b"sentinel master-reboot-down-after-period\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFlushConfig() -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut saved_hz: libc::c_int = server.hz;
    let mut rewrite_status: libc::c_int = 0;
    server.hz = 10 as libc::c_int;
    rewrite_status = rewriteConfig(server.configfile, 0 as libc::c_int);
    server.hz = saved_hz;
    if !(rewrite_status == -(1 as libc::c_int)) {
        fd = open(server.configfile, 0 as libc::c_int);
        if !(fd == -(1 as libc::c_int)) {
            if !(fsync(fd) == -(1 as libc::c_int)) {
                if !(close(fd) == -(1 as libc::c_int)) {
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"Sentinel new configuration saved on disk\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"WARNING: Sentinel was not able to save the new configuration on disk!!!: %s\0"
                as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn sentinelFlushConfigAndReply(mut c: *mut client) {
    if sentinelFlushConfig() == -(1 as libc::c_int) {
        addReplyError(
            c,
            b"Failed to save config file. Check server logs.\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        addReply(c, shared.ok);
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSendAuthIfNeeded(
    mut ri: *mut sentinelRedisInstance,
    mut c: *mut redisAsyncContext,
) {
    let mut auth_pass: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut auth_user: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        auth_pass = (*ri).auth_pass;
        auth_user = (*ri).auth_user;
    } else if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        auth_pass = (*(*ri).master).auth_pass;
        auth_user = (*(*ri).master).auth_user;
    } else if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        if !(sentinel.sentinel_auth_pass).is_null() {
            auth_pass = sentinel.sentinel_auth_pass;
            auth_user = sentinel.sentinel_auth_user;
        } else {
            auth_pass = server.requirepass;
            auth_user = 0 as *mut libc::c_char;
        }
    }
    if !auth_pass.is_null() && auth_user.is_null() {
        if redisAsyncCommand(
            c,
            Some(
                sentinelDiscardReplyCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ri as *mut libc::c_void,
            b"%s %s\0" as *const u8 as *const libc::c_char,
            sentinelInstanceMapCommand(
                ri,
                b"AUTH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            auth_pass,
        ) == 0 as libc::c_int
        {
            (*(*ri).link).pending_commands += 1;
        }
    } else if !auth_pass.is_null() && !auth_user.is_null() {
        if redisAsyncCommand(
            c,
            Some(
                sentinelDiscardReplyCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ri as *mut libc::c_void,
            b"%s %s %s\0" as *const u8 as *const libc::c_char,
            sentinelInstanceMapCommand(
                ri,
                b"AUTH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            auth_user,
            auth_pass,
        ) == 0 as libc::c_int
        {
            (*(*ri).link).pending_commands += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSetClientName(
    mut ri: *mut sentinelRedisInstance,
    mut c: *mut redisAsyncContext,
    mut type_0: *mut libc::c_char,
) {
    let mut name: [libc::c_char; 64] = [0; 64];
    snprintf(
        name.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        b"sentinel-%.8s-%s\0" as *const u8 as *const libc::c_char,
        (sentinel.myid).as_mut_ptr(),
        type_0,
    );
    if redisAsyncCommand(
        c,
        Some(
            sentinelDiscardReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s SETNAME %s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"CLIENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        name.as_mut_ptr(),
    ) == 0 as libc::c_int
    {
        (*(*ri).link).pending_commands += 1;
    }
}
unsafe extern "C" fn instanceLinkNegotiateTLS(
    mut context: *mut redisAsyncContext,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelReconnectInstance(mut ri: *mut sentinelRedisInstance) {
    if (*(*ri).link).disconnected == 0 as libc::c_int {
        return;
    }
    if (*(*ri).addr).port == 0 as libc::c_int {
        return;
    }
    let mut link: *mut instanceLink = (*ri).link;
    let mut now: mstime_t = mstime();
    if now - (*(*ri).link).last_reconn_time < sentinel_ping_period {
        return;
    }
    (*(*ri).link).last_reconn_time = now;
    if ((*link).cc).is_null() {
        if sentinel.resolve_hostnames != 0 {
            let mut tryResolveAddr: *mut sentinelAddr = createSentinelAddr(
                (*(*ri).addr).hostname,
                (*(*ri).addr).port,
                0 as libc::c_int,
            );
            if !tryResolveAddr.is_null() {
                releaseSentinelAddr((*ri).addr);
                (*ri).addr = tryResolveAddr;
            }
        }
        (*link)
            .cc = redisAsyncConnectBind(
            (*(*ri).addr).ip,
            (*(*ri).addr).port,
            server.bind_source_addr,
        );
        if !((*link).cc).is_null() && (*(*link).cc).err == 0 {
            anetCloexec((*(*link).cc).c.fd);
        }
        if ((*link).cc).is_null() {
            sentinelEvent(
                0 as libc::c_int,
                b"-cmd-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #Failed to establish connection\0" as *const u8
                    as *const libc::c_char,
            );
        } else if (*(*link).cc).err == 0 && server.tls_replication != 0
            && instanceLinkNegotiateTLS((*link).cc) == -(1 as libc::c_int)
        {
            sentinelEvent(
                0 as libc::c_int,
                b"-cmd-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #Failed to initialize TLS\0" as *const u8 as *const libc::c_char,
            );
            instanceLinkCloseConnection(link, (*link).cc);
        } else if (*(*link).cc).err != 0 {
            sentinelEvent(
                0 as libc::c_int,
                b"-cmd-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #%s\0" as *const u8 as *const libc::c_char,
                (*(*link).cc).errstr,
            );
            instanceLinkCloseConnection(link, (*link).cc);
        } else {
            (*link).pending_commands = 0 as libc::c_int;
            (*link).cc_conn_time = mstime();
            (*(*link).cc).data = link as *mut libc::c_void;
            redisAeAttach(server.el, (*link).cc);
            redisAsyncSetConnectCallback(
                (*link).cc,
                Some(
                    sentinelLinkEstablishedCallback
                        as unsafe extern "C" fn(
                            *const redisAsyncContext,
                            libc::c_int,
                        ) -> (),
                ),
            );
            redisAsyncSetDisconnectCallback(
                (*link).cc,
                Some(
                    sentinelDisconnectCallback
                        as unsafe extern "C" fn(
                            *const redisAsyncContext,
                            libc::c_int,
                        ) -> (),
                ),
            );
            sentinelSendAuthIfNeeded(ri, (*link).cc);
            sentinelSetClientName(
                ri,
                (*link).cc,
                b"cmd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            sentinelSendPing(ri);
        }
    }
    if (*ri).flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0 && ((*link).pc).is_null()
    {
        (*link)
            .pc = redisAsyncConnectBind(
            (*(*ri).addr).ip,
            (*(*ri).addr).port,
            server.bind_source_addr,
        );
        if !((*link).pc).is_null() && (*(*link).pc).err == 0 {
            anetCloexec((*(*link).pc).c.fd);
        }
        if ((*link).pc).is_null() {
            sentinelEvent(
                0 as libc::c_int,
                b"-pubsub-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #Failed to establish connection\0" as *const u8
                    as *const libc::c_char,
            );
        } else if (*(*link).pc).err == 0 && server.tls_replication != 0
            && instanceLinkNegotiateTLS((*link).pc) == -(1 as libc::c_int)
        {
            sentinelEvent(
                0 as libc::c_int,
                b"-pubsub-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #Failed to initialize TLS\0" as *const u8 as *const libc::c_char,
            );
        } else if (*(*link).pc).err != 0 {
            sentinelEvent(
                0 as libc::c_int,
                b"-pubsub-link-reconnection\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@ #%s\0" as *const u8 as *const libc::c_char,
                (*(*link).pc).errstr,
            );
            instanceLinkCloseConnection(link, (*link).pc);
        } else {
            let mut retval: libc::c_int = 0;
            (*link).pc_conn_time = mstime();
            (*(*link).pc).data = link as *mut libc::c_void;
            redisAeAttach(server.el, (*link).pc);
            redisAsyncSetConnectCallback(
                (*link).pc,
                Some(
                    sentinelLinkEstablishedCallback
                        as unsafe extern "C" fn(
                            *const redisAsyncContext,
                            libc::c_int,
                        ) -> (),
                ),
            );
            redisAsyncSetDisconnectCallback(
                (*link).pc,
                Some(
                    sentinelDisconnectCallback
                        as unsafe extern "C" fn(
                            *const redisAsyncContext,
                            libc::c_int,
                        ) -> (),
                ),
            );
            sentinelSendAuthIfNeeded(ri, (*link).pc);
            sentinelSetClientName(
                ri,
                (*link).pc,
                b"pubsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            retval = redisAsyncCommand(
                (*link).pc,
                Some(
                    sentinelReceiveHelloMessages
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ri as *mut libc::c_void,
                b"%s %s\0" as *const u8 as *const libc::c_char,
                sentinelInstanceMapCommand(
                    ri,
                    b"SUBSCRIBE\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                ),
                b"__sentinel__:hello\0" as *const u8 as *const libc::c_char,
            );
            if retval != 0 as libc::c_int {
                instanceLinkCloseConnection(link, (*link).pc);
                return;
            }
        }
    }
    if !((*link).cc).is_null()
        && ((*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0
            || !((*link).pc).is_null())
    {
        (*link).disconnected = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelMasterLooksSane(
    mut master: *mut sentinelRedisInstance,
) -> libc::c_int {
    return ((*master).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
        && (*master).role_reported == (1 as libc::c_int) << 0 as libc::c_int
        && (*master).flags
            & ((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) == 0 as libc::c_int
        && mstime() - (*master).info_refresh
            < sentinel_info_period * 2 as libc::c_int as libc::c_longlong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRefreshInstanceInfo(
    mut ri: *mut sentinelRedisInstance,
    mut info: *const libc::c_char,
) {
    let mut lines: *mut sds = 0 as *mut sds;
    let mut numlines: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut role: libc::c_int = 0 as libc::c_int;
    sdsfree((*ri).info);
    (*ri).info = sdsnew(info);
    (*ri).master_link_down_time = 0 as libc::c_int as mstime_t;
    lines = sdssplitlen(
        info,
        strlen(info) as ssize_t,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        &mut numlines,
    );
    let mut current_block_73: u64;
    j = 0 as libc::c_int;
    while j < numlines {
        let mut slave: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
        let mut l: sds = *lines.offset(j as isize);
        if sdslen(l) >= 47 as libc::c_int as libc::c_ulong
            && memcmp(
                l as *const libc::c_void,
                b"run_id:\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                7 as libc::c_int as libc::c_ulong,
            ) == 0
        {
            if ((*ri).runid).is_null() {
                (*ri)
                    .runid = sdsnewlen(
                    l.offset(7 as libc::c_int as isize) as *const libc::c_void,
                    40 as libc::c_int as size_t,
                );
            } else if strncmp(
                (*ri).runid,
                l.offset(7 as libc::c_int as isize) as *const libc::c_char,
                40 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                sentinelEvent(
                    2 as libc::c_int,
                    b"+reboot\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ri,
                    b"%@\0" as *const u8 as *const libc::c_char,
                );
                if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                    && (*ri).master_reboot_down_after_period
                        != 0 as libc::c_int as libc::c_longlong
                {
                    (*ri).flags |= (1 as libc::c_int) << 13 as libc::c_int;
                    (*ri).master_reboot_since_time = mstime();
                }
                sdsfree((*ri).runid);
                (*ri)
                    .runid = sdsnewlen(
                    l.offset(7 as libc::c_int as isize) as *const libc::c_void,
                    40 as libc::c_int as size_t,
                );
            }
        }
        if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
            && sdslen(l) >= 7 as libc::c_int as libc::c_ulong
            && memcmp(
                l as *const libc::c_void,
                b"slave\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                5 as libc::c_int as libc::c_ulong,
            ) == 0
            && *(*__ctype_b_loc())
                .offset(*l.offset(5 as libc::c_int as isize) as libc::c_int as isize)
                as libc::c_int & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            let mut ip: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut port: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
            if (strstr(
                l as *const libc::c_char,
                b"ip=\0" as *const u8 as *const libc::c_char,
            ))
                .is_null()
            {
                ip = strchr(l as *const libc::c_char, ':' as i32);
                if ip.is_null() {
                    current_block_73 = 17179679302217393232;
                } else {
                    ip = ip.offset(1);
                    port = strchr(ip, ',' as i32);
                    if port.is_null() {
                        current_block_73 = 17179679302217393232;
                    } else {
                        *port = '\0' as i32 as libc::c_char;
                        port = port.offset(1);
                        end = strchr(port, ',' as i32);
                        if end.is_null() {
                            current_block_73 = 17179679302217393232;
                        } else {
                            *end = '\0' as i32 as libc::c_char;
                            current_block_73 = 8180496224585318153;
                        }
                    }
                }
            } else {
                ip = strstr(
                    l as *const libc::c_char,
                    b"ip=\0" as *const u8 as *const libc::c_char,
                );
                if ip.is_null() {
                    current_block_73 = 17179679302217393232;
                } else {
                    ip = ip.offset(3 as libc::c_int as isize);
                    port = strstr(
                        l as *const libc::c_char,
                        b"port=\0" as *const u8 as *const libc::c_char,
                    );
                    if port.is_null() {
                        current_block_73 = 17179679302217393232;
                    } else {
                        port = port.offset(5 as libc::c_int as isize);
                        end = strchr(ip, ',' as i32);
                        if !end.is_null() {
                            *end = '\0' as i32 as libc::c_char;
                        }
                        end = strchr(port, ',' as i32);
                        if !end.is_null() {
                            *end = '\0' as i32 as libc::c_char;
                        }
                        current_block_73 = 8180496224585318153;
                    }
                }
            }
            match current_block_73 {
                17179679302217393232 => {}
                _ => {
                    if (sentinelRedisInstanceLookupSlave(ri, ip, atoi(port))).is_null() {
                        slave = createSentinelRedisInstance(
                            0 as *mut libc::c_char,
                            (1 as libc::c_int) << 1 as libc::c_int,
                            ip,
                            atoi(port),
                            (*ri).quorum as libc::c_int,
                            ri,
                        );
                        if !slave.is_null() {
                            sentinelEvent(
                                2 as libc::c_int,
                                b"+slave\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                slave,
                                b"%@\0" as *const u8 as *const libc::c_char,
                            );
                            sentinelFlushConfig();
                        }
                    }
                    current_block_73 = 15512526488502093901;
                }
            }
        } else {
            current_block_73 = 15512526488502093901;
        }
        match current_block_73 {
            15512526488502093901 => {
                if sdslen(l) >= 32 as libc::c_int as libc::c_ulong
                    && memcmp(
                        l as *const libc::c_void,
                        b"master_link_down_since_seconds\0" as *const u8
                            as *const libc::c_char as *const libc::c_void,
                        30 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    (*ri)
                        .master_link_down_time = strtoll(
                        l.offset(31 as libc::c_int as isize) as *const libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) * 1000 as libc::c_int as libc::c_longlong;
                }
                if sdslen(l) >= 11 as libc::c_int as libc::c_ulong
                    && memcmp(
                        l as *const libc::c_void,
                        b"role:master\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        11 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    role = (1 as libc::c_int) << 0 as libc::c_int;
                } else if sdslen(l) >= 10 as libc::c_int as libc::c_ulong
                    && memcmp(
                        l as *const libc::c_void,
                        b"role:slave\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        10 as libc::c_int as libc::c_ulong,
                    ) == 0
                {
                    role = (1 as libc::c_int) << 1 as libc::c_int;
                }
                if role == (1 as libc::c_int) << 1 as libc::c_int {
                    if sdslen(l) >= 12 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"master_host:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            12 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        if ((*ri).slave_master_host).is_null()
                            || strcasecmp(
                                l.offset(12 as libc::c_int as isize) as *const libc::c_char,
                                (*ri).slave_master_host,
                            ) != 0
                        {
                            sdsfree((*ri).slave_master_host);
                            (*ri)
                                .slave_master_host = sdsnew(
                                l.offset(12 as libc::c_int as isize) as *const libc::c_char,
                            );
                            (*ri).slave_conf_change_time = mstime();
                        }
                    }
                    if sdslen(l) >= 12 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"master_port:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            12 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        let mut slave_master_port: libc::c_int = atoi(
                            l.offset(12 as libc::c_int as isize) as *const libc::c_char,
                        );
                        if (*ri).slave_master_port != slave_master_port {
                            (*ri).slave_master_port = slave_master_port;
                            (*ri).slave_conf_change_time = mstime();
                        }
                    }
                    if sdslen(l) >= 19 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"master_link_status:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            19 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        (*ri)
                            .slave_master_link_status = if strcasecmp(
                            l.offset(19 as libc::c_int as isize) as *const libc::c_char,
                            b"up\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int
                        {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        };
                    }
                    if sdslen(l) >= 15 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"slave_priority:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            15 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        (*ri)
                            .slave_priority = atoi(
                            l.offset(15 as libc::c_int as isize) as *const libc::c_char,
                        );
                    }
                    if sdslen(l) >= 18 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"slave_repl_offset:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            18 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        (*ri)
                            .slave_repl_offset = strtoull(
                            l.offset(18 as libc::c_int as isize) as *const libc::c_char,
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        );
                    }
                    if sdslen(l) >= 18 as libc::c_int as libc::c_ulong
                        && memcmp(
                            l as *const libc::c_void,
                            b"replica_announced:\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            18 as libc::c_int as libc::c_ulong,
                        ) == 0
                    {
                        (*ri)
                            .replica_announced = atoi(
                            l.offset(18 as libc::c_int as isize) as *const libc::c_char,
                        );
                    }
                }
            }
            _ => {}
        }
        j += 1;
    }
    (*ri).info_refresh = mstime();
    sdsfreesplitres(lines, numlines);
    if role != (*ri).role_reported {
        (*ri).role_reported_time = mstime();
        (*ri).role_reported = role;
        if role == (1 as libc::c_int) << 1 as libc::c_int {
            (*ri).slave_conf_change_time = mstime();
        }
        sentinelEvent(
            1 as libc::c_int,
            (if (*ri).flags
                & ((1 as libc::c_int) << 0 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int) == role
            {
                b"+role-change\0" as *const u8 as *const libc::c_char
            } else {
                b"-role-change\0" as *const u8 as *const libc::c_char
            }) as *mut libc::c_char,
            ri,
            b"%@ new reported role is %s\0" as *const u8 as *const libc::c_char,
            if role == (1 as libc::c_int) << 0 as libc::c_int {
                b"master\0" as *const u8 as *const libc::c_char
            } else {
                b"slave\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if sentinel.tilt != 0 {
        return;
    }
    (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
        && role == (1 as libc::c_int) << 1 as libc::c_int;
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && role == (1 as libc::c_int) << 0 as libc::c_int
    {
        if (*ri).flags & (1 as libc::c_int) << 7 as libc::c_int != 0
            && (*(*ri).master).flags & (1 as libc::c_int) << 6 as libc::c_int != 0
            && (*(*ri).master).failover_state == 4 as libc::c_int
        {
            (*(*ri).master).config_epoch = (*(*ri).master).failover_epoch;
            (*(*ri).master).failover_state = 5 as libc::c_int;
            (*(*ri).master).failover_state_change_time = mstime();
            sentinelFlushConfig();
            sentinelEvent(
                3 as libc::c_int,
                b"+promoted-slave\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            if sentinel.simfailure_flags
                & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
            {
                sentinelSimFailureCrash();
            }
            sentinelEvent(
                3 as libc::c_int,
                b"+failover-state-reconf-slaves\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                (*ri).master,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            sentinelCallClientReconfScript(
                (*ri).master,
                (1 as libc::c_int) << 17 as libc::c_int,
                b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                (*(*ri).master).addr,
                (*ri).addr,
            );
            sentinelForceHelloUpdateForMaster((*ri).master);
        } else {
            let mut wait_time: mstime_t = sentinel_publish_period
                * 4 as libc::c_int as libc::c_longlong;
            if (*ri).flags & (1 as libc::c_int) << 7 as libc::c_int == 0
                && sentinelMasterLooksSane((*ri).master) != 0
                && sentinelRedisInstanceNoDownFor(ri, wait_time) != 0
                && mstime() - (*ri).role_reported_time > wait_time
            {
                let mut retval: libc::c_int = sentinelSendSlaveOf(
                    ri,
                    (*(*ri).master).addr,
                );
                if retval == 0 as libc::c_int {
                    sentinelEvent(
                        2 as libc::c_int,
                        b"+convert-to-slave\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        ri,
                        b"%@\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
    }
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && role == (1 as libc::c_int) << 1 as libc::c_int
        && ((*ri).slave_master_port != (*(*(*ri).master).addr).port
            || sentinelAddrEqualsHostname((*(*ri).master).addr, (*ri).slave_master_host)
                == 0)
    {
        let mut wait_time_0: mstime_t = (*(*ri).master).failover_timeout;
        if sentinelMasterLooksSane((*ri).master) != 0
            && sentinelRedisInstanceNoDownFor(ri, wait_time_0) != 0
            && mstime() - (*ri).slave_conf_change_time > wait_time_0
        {
            let mut retval_0: libc::c_int = sentinelSendSlaveOf(
                ri,
                (*(*ri).master).addr,
            );
            if retval_0 == 0 as libc::c_int {
                sentinelEvent(
                    2 as libc::c_int,
                    b"+fix-slave-config\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ri,
                    b"%@\0" as *const u8 as *const libc::c_char,
                );
            }
        }
    }
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && role == (1 as libc::c_int) << 1 as libc::c_int
        && (*ri).flags
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) != 0
    {
        if (*ri).flags & (1 as libc::c_int) << 8 as libc::c_int != 0
            && !((*ri).slave_master_host).is_null()
            && sentinelAddrEqualsHostname(
                (*(*(*ri).master).promoted_slave).addr,
                (*ri).slave_master_host,
            ) != 0
            && (*ri).slave_master_port == (*(*(*(*ri).master).promoted_slave).addr).port
        {
            (*ri).flags &= !((1 as libc::c_int) << 8 as libc::c_int);
            (*ri).flags |= (1 as libc::c_int) << 9 as libc::c_int;
            sentinelEvent(
                2 as libc::c_int,
                b"+slave-reconf-inprog\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*ri).flags & (1 as libc::c_int) << 9 as libc::c_int != 0
            && (*ri).slave_master_link_status == 0 as libc::c_int
        {
            (*ri).flags &= !((1 as libc::c_int) << 9 as libc::c_int);
            (*ri).flags |= (1 as libc::c_int) << 10 as libc::c_int;
            sentinelEvent(
                2 as libc::c_int,
                b"+slave-reconf-done\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelInfoReplyCallback(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut ri: *mut sentinelRedisInstance = privdata as *mut sentinelRedisInstance;
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    if reply.is_null() || link.is_null() {
        return;
    }
    (*link).pending_commands -= 1;
    r = reply as *mut redisReply;
    if (*r).type_0 == 1 as libc::c_int {
        sentinelRefreshInstanceInfo(ri, (*r).str_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelDiscardReplyCallback(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    if !link.is_null() {
        (*link).pending_commands -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelPingReplyCallback(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut ri: *mut sentinelRedisInstance = privdata as *mut sentinelRedisInstance;
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    if reply.is_null() || link.is_null() {
        return;
    }
    (*link).pending_commands -= 1;
    r = reply as *mut redisReply;
    if (*r).type_0 == 5 as libc::c_int || (*r).type_0 == 6 as libc::c_int {
        if strncmp(
            (*r).str_0,
            b"PONG\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            || strncmp(
                (*r).str_0,
                b"LOADING\0" as *const u8 as *const libc::c_char,
                7 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
            || strncmp(
                (*r).str_0,
                b"MASTERDOWN\0" as *const u8 as *const libc::c_char,
                10 as libc::c_int as libc::c_ulong,
            ) == 0 as libc::c_int
        {
            (*link).last_avail_time = mstime();
            (*link).act_ping_time = 0 as libc::c_int as mstime_t;
            if (*ri).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
                && strncmp(
                    (*r).str_0,
                    b"PONG\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
            {
                (*ri).flags &= !((1 as libc::c_int) << 13 as libc::c_int);
            }
        } else if strncmp(
            (*r).str_0,
            b"BUSY\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
            && (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int != 0
            && (*ri).flags & (1 as libc::c_int) << 12 as libc::c_int == 0
        {
            if redisAsyncCommand(
                (*(*ri).link).cc,
                Some(
                    sentinelDiscardReplyCallback
                        as unsafe extern "C" fn(
                            *mut redisAsyncContext,
                            *mut libc::c_void,
                            *mut libc::c_void,
                        ) -> (),
                ),
                ri as *mut libc::c_void,
                b"%s KILL\0" as *const u8 as *const libc::c_char,
                sentinelInstanceMapCommand(
                    ri,
                    b"SCRIPT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ),
            ) == 0 as libc::c_int
            {
                (*(*ri).link).pending_commands += 1;
            }
            (*ri).flags |= (1 as libc::c_int) << 12 as libc::c_int;
        }
    }
    (*link).last_pong_time = mstime();
}
#[no_mangle]
pub unsafe extern "C" fn sentinelPublishReplyCallback(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut ri: *mut sentinelRedisInstance = privdata as *mut sentinelRedisInstance;
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    if reply.is_null() || link.is_null() {
        return;
    }
    (*link).pending_commands -= 1;
    r = reply as *mut redisReply;
    if (*r).type_0 != 6 as libc::c_int {
        (*ri).last_pub_time = mstime();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelProcessHelloMessage(
    mut hello: *mut libc::c_char,
    mut hello_len: libc::c_int,
) {
    let mut numtokens: libc::c_int = 0;
    let mut port: libc::c_int = 0;
    let mut removed: libc::c_int = 0;
    let mut master_port: libc::c_int = 0;
    let mut current_epoch: uint64_t = 0;
    let mut master_config_epoch: uint64_t = 0;
    let mut token: *mut *mut libc::c_char = sdssplitlen(
        hello,
        hello_len as ssize_t,
        b",\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut numtokens,
    );
    let mut si: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut master: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    if numtokens == 8 as libc::c_int {
        master = sentinelGetMasterByName(*token.offset(4 as libc::c_int as isize));
        if !master.is_null() {
            port = atoi(*token.offset(1 as libc::c_int as isize));
            master_port = atoi(*token.offset(6 as libc::c_int as isize));
            si = getSentinelRedisInstanceByAddrAndRunID(
                (*master).sentinels,
                *token.offset(0 as libc::c_int as isize),
                port,
                *token.offset(2 as libc::c_int as isize),
            );
            current_epoch = strtoull(
                *token.offset(3 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) as uint64_t;
            master_config_epoch = strtoull(
                *token.offset(7 as libc::c_int as isize),
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            ) as uint64_t;
            if si.is_null() {
                removed = removeMatchingSentinelFromMaster(
                    master,
                    *token.offset(2 as libc::c_int as isize),
                );
                if removed != 0 {
                    sentinelEvent(
                        2 as libc::c_int,
                        b"+sentinel-address-switch\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        master,
                        b"%@ ip %s port %d for %s\0" as *const u8 as *const libc::c_char,
                        *token.offset(0 as libc::c_int as isize),
                        port,
                        *token.offset(2 as libc::c_int as isize),
                    );
                } else {
                    let mut other: *mut sentinelRedisInstance = getSentinelRedisInstanceByAddrAndRunID(
                        (*master).sentinels,
                        *token.offset(0 as libc::c_int as isize),
                        port,
                        0 as *mut libc::c_char,
                    );
                    if !other.is_null() {
                        sentinelEvent(
                            2 as libc::c_int,
                            b"+sentinel-invalid-addr\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            other,
                            b"%@\0" as *const u8 as *const libc::c_char,
                        );
                        let mut di: *mut dictIterator = 0 as *mut dictIterator;
                        let mut de: *mut dictEntry = 0 as *mut dictEntry;
                        let mut runid_obsolete: sds = sdsnew((*other).runid);
                        di = dictGetIterator(sentinel.masters);
                        loop {
                            de = dictNext(di);
                            if de.is_null() {
                                break;
                            }
                            let mut master_0: *mut sentinelRedisInstance = (*de).v.val
                                as *mut sentinelRedisInstance;
                            removeMatchingSentinelFromMaster(master_0, runid_obsolete);
                        }
                        dictReleaseIterator(di);
                        sdsfree(runid_obsolete);
                    }
                }
                si = createSentinelRedisInstance(
                    *token.offset(2 as libc::c_int as isize),
                    (1 as libc::c_int) << 2 as libc::c_int,
                    *token.offset(0 as libc::c_int as isize),
                    port,
                    (*master).quorum as libc::c_int,
                    master,
                );
                if !si.is_null() {
                    if removed == 0 {
                        sentinelEvent(
                            2 as libc::c_int,
                            b"+sentinel\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            si,
                            b"%@\0" as *const u8 as *const libc::c_char,
                        );
                    }
                    (*si).runid = sdsnew(*token.offset(2 as libc::c_int as isize));
                    sentinelTryConnectionSharing(si);
                    if removed != 0 {
                        sentinelUpdateSentinelAddressInAllMasters(si);
                    }
                    sentinelFlushConfig();
                }
            }
            if current_epoch > sentinel.current_epoch {
                sentinel.current_epoch = current_epoch;
                sentinelFlushConfig();
                sentinelEvent(
                    3 as libc::c_int,
                    b"+new-epoch\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    master,
                    b"%llu\0" as *const u8 as *const libc::c_char,
                    sentinel.current_epoch as libc::c_ulonglong,
                );
            }
            if !si.is_null() && (*master).config_epoch < master_config_epoch {
                (*master).config_epoch = master_config_epoch;
                if master_port != (*(*master).addr).port
                    || sentinelAddrEqualsHostname(
                        (*master).addr,
                        *token.offset(5 as libc::c_int as isize),
                    ) == 0
                {
                    let mut old_addr: *mut sentinelAddr = 0 as *mut sentinelAddr;
                    sentinelEvent(
                        3 as libc::c_int,
                        b"+config-update-from\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        si,
                        b"%@\0" as *const u8 as *const libc::c_char,
                    );
                    sentinelEvent(
                        3 as libc::c_int,
                        b"+switch-master\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        master,
                        b"%s %s %d %s %d\0" as *const u8 as *const libc::c_char,
                        (*master).name,
                        announceSentinelAddr((*master).addr),
                        (*(*master).addr).port,
                        *token.offset(5 as libc::c_int as isize),
                        master_port,
                    );
                    old_addr = dupSentinelAddr((*master).addr);
                    sentinelResetMasterAndChangeAddress(
                        master,
                        *token.offset(5 as libc::c_int as isize),
                        master_port,
                    );
                    sentinelCallClientReconfScript(
                        master,
                        (1 as libc::c_int) << 18 as libc::c_int,
                        b"start\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        old_addr,
                        (*master).addr,
                    );
                    releaseSentinelAddr(old_addr);
                }
            }
            if !si.is_null() {
                (*si).last_hello_time = mstime();
            }
        }
    }
    sdsfreesplitres(token, numtokens);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelReceiveHelloMessages(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut ri: *mut sentinelRedisInstance = privdata as *mut sentinelRedisInstance;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    if reply.is_null() || ri.is_null() {
        return;
    }
    r = reply as *mut redisReply;
    (*(*ri).link).pc_last_activity = mstime();
    if (*r).type_0 != 2 as libc::c_int
        || (*r).elements != 3 as libc::c_int as libc::c_ulong
        || (**((*r).element).offset(0 as libc::c_int as isize)).type_0
            != 1 as libc::c_int
        || (**((*r).element).offset(1 as libc::c_int as isize)).type_0
            != 1 as libc::c_int
        || (**((*r).element).offset(2 as libc::c_int as isize)).type_0
            != 1 as libc::c_int
        || strcmp(
            (**((*r).element).offset(0 as libc::c_int as isize)).str_0,
            b"message\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        return;
    }
    if !(strstr(
        (**((*r).element).offset(2 as libc::c_int as isize)).str_0,
        (sentinel.myid).as_mut_ptr(),
    ))
        .is_null()
    {
        return;
    }
    sentinelProcessHelloMessage(
        (**((*r).element).offset(2 as libc::c_int as isize)).str_0,
        (**((*r).element).offset(2 as libc::c_int as isize)).len as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSendHello(
    mut ri: *mut sentinelRedisInstance,
) -> libc::c_int {
    let mut ip: [libc::c_char; 46] = [0; 46];
    let mut payload: [libc::c_char; 1070] = [0; 1070];
    let mut retval: libc::c_int = 0;
    let mut announce_ip: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut announce_port: libc::c_int = 0;
    let mut master: *mut sentinelRedisInstance = if (*ri).flags
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        ri
    } else {
        (*ri).master
    };
    let mut master_addr: *mut sentinelAddr = sentinelGetCurrentMasterAddress(master);
    if (*(*ri).link).disconnected != 0 {
        return -(1 as libc::c_int);
    }
    if !(sentinel.announce_ip).is_null() {
        announce_ip = sentinel.announce_ip;
    } else {
        if anetFdToString(
            (*(*(*ri).link).cc).c.fd,
            ip.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
            0 as *mut libc::c_int,
            1 as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        announce_ip = ip.as_mut_ptr();
    }
    if sentinel.announce_port != 0 {
        announce_port = sentinel.announce_port;
    } else if server.tls_replication != 0 && server.tls_port != 0 {
        announce_port = server.tls_port;
    } else {
        announce_port = server.port;
    }
    snprintf(
        payload.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1070]>() as libc::c_ulong,
        b"%s,%d,%s,%llu,%s,%s,%d,%llu\0" as *const u8 as *const libc::c_char,
        announce_ip,
        announce_port,
        (sentinel.myid).as_mut_ptr(),
        sentinel.current_epoch as libc::c_ulonglong,
        (*master).name,
        announceSentinelAddr(master_addr),
        (*master_addr).port,
        (*master).config_epoch as libc::c_ulonglong,
    );
    retval = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelPublishReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s %s %s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"PUBLISH\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        b"__sentinel__:hello\0" as *const u8 as *const libc::c_char,
        payload.as_mut_ptr(),
    );
    if retval != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*(*ri).link).pending_commands += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelForceHelloUpdateDictOfRedisInstances(
    mut instances: *mut dict,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator(instances);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if (*ri).last_pub_time
            >= sentinel_publish_period + 1 as libc::c_int as libc::c_longlong
        {
            (*ri).last_pub_time
                -= sentinel_publish_period + 1 as libc::c_int as libc::c_longlong;
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelForceHelloUpdateForMaster(
    mut master: *mut sentinelRedisInstance,
) -> libc::c_int {
    if (*master).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return -(1 as libc::c_int);
    }
    if (*master).last_pub_time
        >= sentinel_publish_period + 1 as libc::c_int as libc::c_longlong
    {
        (*master).last_pub_time
            -= sentinel_publish_period + 1 as libc::c_int as libc::c_longlong;
    }
    sentinelForceHelloUpdateDictOfRedisInstances((*master).sentinels);
    sentinelForceHelloUpdateDictOfRedisInstances((*master).slaves);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSendPing(
    mut ri: *mut sentinelRedisInstance,
) -> libc::c_int {
    let mut retval: libc::c_int = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelPingReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"PING\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    if retval == 0 as libc::c_int {
        (*(*ri).link).pending_commands += 1;
        (*(*ri).link).last_ping_time = mstime();
        if (*(*ri).link).act_ping_time == 0 as libc::c_int as libc::c_longlong {
            (*(*ri).link).act_ping_time = (*(*ri).link).last_ping_time;
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSendPeriodicCommands(
    mut ri: *mut sentinelRedisInstance,
) {
    let mut now: mstime_t = mstime();
    let mut info_period: mstime_t = 0;
    let mut ping_period: mstime_t = 0;
    let mut retval: libc::c_int = 0;
    if (*(*ri).link).disconnected != 0 {
        return;
    }
    if (*(*ri).link).pending_commands >= 100 as libc::c_int * (*(*ri).link).refcount {
        return;
    }
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && ((*(*ri).master).flags
            & ((1 as libc::c_int) << 4 as libc::c_int
                | (1 as libc::c_int) << 6 as libc::c_int) != 0
            || (*ri).master_link_down_time != 0 as libc::c_int as libc::c_longlong)
    {
        info_period = 1000 as libc::c_int as mstime_t;
    } else {
        info_period = sentinel_info_period;
    }
    ping_period = (*ri).down_after_period;
    if ping_period > sentinel_ping_period {
        ping_period = sentinel_ping_period;
    }
    if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int == 0 as libc::c_int
        && ((*ri).info_refresh == 0 as libc::c_int as libc::c_longlong
            || now - (*ri).info_refresh > info_period)
    {
        retval = redisAsyncCommand(
            (*(*ri).link).cc,
            Some(
                sentinelInfoReplyCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ri as *mut libc::c_void,
            b"%s\0" as *const u8 as *const libc::c_char,
            sentinelInstanceMapCommand(
                ri,
                b"INFO\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
        );
        if retval == 0 as libc::c_int {
            (*(*ri).link).pending_commands += 1;
        }
    }
    if now - (*(*ri).link).last_pong_time > ping_period
        && now - (*(*ri).link).last_ping_time
            > ping_period / 2 as libc::c_int as libc::c_longlong
    {
        sentinelSendPing(ri);
    }
    if now - (*ri).last_pub_time > sentinel_publish_period {
        sentinelSendHello(ri);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelConfigSetCommand(mut c: *mut client) {
    let mut current_block: u64;
    let mut o: *mut robj = *((*c).argv).offset(3 as libc::c_int as isize);
    let mut val: *mut robj = *((*c).argv).offset(4 as libc::c_int as isize);
    let mut numval: libc::c_longlong = 0;
    let mut drop_conns: libc::c_int = 0 as libc::c_int;
    if strcasecmp(
        (*o).ptr as *const libc::c_char,
        b"resolve-hostnames\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        numval = yesnotoi((*val).ptr as *mut libc::c_char) as libc::c_longlong;
        if numval == -(1 as libc::c_int) as libc::c_longlong {
            current_block = 4398398724228373571;
        } else {
            sentinel.resolve_hostnames = numval as libc::c_int;
            current_block = 10043043949733653460;
        }
    } else if strcasecmp(
        (*o).ptr as *const libc::c_char,
        b"announce-hostnames\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        numval = yesnotoi((*val).ptr as *mut libc::c_char) as libc::c_longlong;
        if numval == -(1 as libc::c_int) as libc::c_longlong {
            current_block = 4398398724228373571;
        } else {
            sentinel.announce_hostnames = numval as libc::c_int;
            current_block = 10043043949733653460;
        }
    } else if strcasecmp(
        (*o).ptr as *const libc::c_char,
        b"announce-ip\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if !(sentinel.announce_ip).is_null() {
            sdsfree(sentinel.announce_ip);
        }
        sentinel.announce_ip = sdsnew((*val).ptr as *const libc::c_char);
        current_block = 10043043949733653460;
    } else if strcasecmp(
        (*o).ptr as *const libc::c_char,
        b"announce-port\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if getLongLongFromObject(val, &mut numval) == -(1 as libc::c_int)
            || numval < 0 as libc::c_int as libc::c_longlong
            || numval > 65535 as libc::c_int as libc::c_longlong
        {
            current_block = 4398398724228373571;
        } else {
            sentinel.announce_port = numval as libc::c_int;
            current_block = 10043043949733653460;
        }
    } else {
        if strcasecmp(
            (*o).ptr as *const libc::c_char,
            b"sentinel-user\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            sdsfree(sentinel.sentinel_auth_user);
            sentinel
                .sentinel_auth_user = if sdslen((*val).ptr as sds)
                == 0 as libc::c_int as libc::c_ulong
            {
                0 as sds
            } else {
                sdsdup((*val).ptr as sds)
            };
            drop_conns = 1 as libc::c_int;
        } else if strcasecmp(
            (*o).ptr as *const libc::c_char,
            b"sentinel-pass\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            sdsfree(sentinel.sentinel_auth_pass);
            sentinel
                .sentinel_auth_pass = if sdslen((*val).ptr as sds)
                == 0 as libc::c_int as libc::c_ulong
            {
                0 as sds
            } else {
                sdsdup((*val).ptr as sds)
            };
            drop_conns = 1 as libc::c_int;
        } else {
            addReplyErrorFormat(
                c,
                b"Invalid argument '%s' to SENTINEL CONFIG SET\0" as *const u8
                    as *const libc::c_char,
                (*o).ptr as *mut libc::c_char,
            );
            return;
        }
        current_block = 10043043949733653460;
    }
    match current_block {
        4398398724228373571 => {
            addReplyErrorFormat(
                c,
                b"Invalid value '%s' to SENTINEL CONFIG SET '%s'\0" as *const u8
                    as *const libc::c_char,
                (*val).ptr as *mut libc::c_char,
                (*o).ptr as *mut libc::c_char,
            );
            return;
        }
        _ => {
            sentinelFlushConfigAndReply(c);
            if drop_conns != 0 {
                sentinelDropConnections();
            }
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelConfigGetCommand(mut c: *mut client) {
    let mut o: *mut robj = *((*c).argv).offset(3 as libc::c_int as isize);
    let mut pattern: *const libc::c_char = (*o).ptr as *const libc::c_char;
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut matches: libc::c_int = 0 as libc::c_int;
    if stringmatch(
        pattern,
        b"resolve-hostnames\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(
            c,
            b"resolve-hostnames\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkCString(
            c,
            if sentinel.resolve_hostnames != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
        matches += 1;
    }
    if stringmatch(
        pattern,
        b"announce-hostnames\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(
            c,
            b"announce-hostnames\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkCString(
            c,
            if sentinel.announce_hostnames != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
        matches += 1;
    }
    if stringmatch(
        pattern,
        b"announce-ip\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(c, b"announce-ip\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !(sentinel.announce_ip).is_null() {
                sentinel.announce_ip as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        matches += 1;
    }
    if stringmatch(
        pattern,
        b"announce-port\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(c, b"announce-port\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, sentinel.announce_port as libc::c_longlong);
        matches += 1;
    }
    if stringmatch(
        pattern,
        b"sentinel-user\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(c, b"sentinel-user\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !(sentinel.sentinel_auth_user).is_null() {
                sentinel.sentinel_auth_user as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        matches += 1;
    }
    if stringmatch(
        pattern,
        b"sentinel-pass\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
    ) != 0
    {
        addReplyBulkCString(c, b"sentinel-pass\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !(sentinel.sentinel_auth_pass).is_null() {
                sentinel.sentinel_auth_pass as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        matches += 1;
    }
    setDeferredMapLen(c, replylen, matches as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverStateStr(
    mut state: libc::c_int,
) -> *const libc::c_char {
    match state {
        0 => return b"none\0" as *const u8 as *const libc::c_char,
        1 => return b"wait_start\0" as *const u8 as *const libc::c_char,
        2 => return b"select_slave\0" as *const u8 as *const libc::c_char,
        3 => return b"send_slaveof_noone\0" as *const u8 as *const libc::c_char,
        4 => return b"wait_promotion\0" as *const u8 as *const libc::c_char,
        5 => return b"reconf_slaves\0" as *const u8 as *const libc::c_char,
        6 => return b"update_config\0" as *const u8 as *const libc::c_char,
        _ => return b"unknown\0" as *const u8 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplySentinelRedisInstance(
    mut c: *mut client,
    mut ri: *mut sentinelRedisInstance,
) {
    let mut flags: *mut libc::c_char = sdsempty();
    let mut mbl: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fields: libc::c_int = 0 as libc::c_int;
    mbl = addReplyDeferredLen(c);
    addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, (*ri).name);
    fields += 1;
    addReplyBulkCString(c, b"ip\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, announceSentinelAddr((*ri).addr));
    fields += 1;
    addReplyBulkCString(c, b"port\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, (*(*ri).addr).port as libc::c_longlong);
    fields += 1;
    addReplyBulkCString(c, b"runid\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(
        c,
        if !((*ri).runid).is_null() {
            (*ri).runid as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
    fields += 1;
    addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
    if (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        flags = sdscat(flags, b"s_down,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        flags = sdscat(flags, b"o_down,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        flags = sdscat(flags, b"master,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        flags = sdscat(flags, b"slave,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        flags = sdscat(flags, b"sentinel,\0" as *const u8 as *const libc::c_char);
    }
    if (*(*ri).link).disconnected != 0 {
        flags = sdscat(flags, b"disconnected,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        flags = sdscat(flags, b"master_down,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        flags = sdscat(
            flags,
            b"failover_in_progress,\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*ri).flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        flags = sdscat(flags, b"promoted,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 {
        flags = sdscat(flags, b"reconf_sent,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
        flags = sdscat(flags, b"reconf_inprog,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 10 as libc::c_int != 0 {
        flags = sdscat(flags, b"reconf_done,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 11 as libc::c_int != 0 {
        flags = sdscat(flags, b"force_failover,\0" as *const u8 as *const libc::c_char);
    }
    if (*ri).flags & (1 as libc::c_int) << 12 as libc::c_int != 0 {
        flags = sdscat(
            flags,
            b"script_kill_sent,\0" as *const u8 as *const libc::c_char,
        );
    }
    if sdslen(flags) != 0 as libc::c_int as libc::c_ulong {
        sdsrange(flags, 0 as libc::c_int as ssize_t, -(2 as libc::c_int) as ssize_t);
    }
    addReplyBulkCString(c, flags);
    sdsfree(flags);
    fields += 1;
    addReplyBulkCString(
        c,
        b"link-pending-commands\0" as *const u8 as *const libc::c_char,
    );
    addReplyBulkLongLong(c, (*(*ri).link).pending_commands as libc::c_longlong);
    fields += 1;
    addReplyBulkCString(c, b"link-refcount\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, (*(*ri).link).refcount as libc::c_longlong);
    fields += 1;
    if (*ri).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        addReplyBulkCString(c, b"failover-state\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            sentinelFailoverStateStr((*ri).failover_state) as *mut libc::c_char,
        );
        fields += 1;
    }
    addReplyBulkCString(c, b"last-ping-sent\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(
        c,
        if (*(*ri).link).act_ping_time != 0 {
            mstime() - (*(*ri).link).act_ping_time
        } else {
            0 as libc::c_int as libc::c_longlong
        },
    );
    fields += 1;
    addReplyBulkCString(c, b"last-ok-ping-reply\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, mstime() - (*(*ri).link).last_avail_time);
    fields += 1;
    addReplyBulkCString(c, b"last-ping-reply\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, mstime() - (*(*ri).link).last_pong_time);
    fields += 1;
    if (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        addReplyBulkCString(c, b"s-down-time\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, mstime() - (*ri).s_down_since_time);
        fields += 1;
    }
    if (*ri).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        addReplyBulkCString(c, b"o-down-time\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, mstime() - (*ri).o_down_since_time);
        fields += 1;
    }
    addReplyBulkCString(
        c,
        b"down-after-milliseconds\0" as *const u8 as *const libc::c_char,
    );
    addReplyBulkLongLong(c, (*ri).down_after_period);
    fields += 1;
    if (*ri).flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        addReplyBulkCString(c, b"info-refresh\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(
            c,
            if (*ri).info_refresh != 0 {
                mstime() - (*ri).info_refresh
            } else {
                0 as libc::c_int as libc::c_longlong
            },
        );
        fields += 1;
        addReplyBulkCString(c, b"role-reported\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if (*ri).role_reported == (1 as libc::c_int) << 0 as libc::c_int {
                b"master\0" as *const u8 as *const libc::c_char
            } else {
                b"slave\0" as *const u8 as *const libc::c_char
            },
        );
        fields += 1;
        addReplyBulkCString(
            c,
            b"role-reported-time\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, mstime() - (*ri).role_reported_time);
        fields += 1;
    }
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        addReplyBulkCString(c, b"config-epoch\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*ri).config_epoch as libc::c_longlong);
        fields += 1;
        addReplyBulkCString(c, b"num-slaves\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(
            c,
            ((*(*ri).slaves).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*ri).slaves).ht_used[1 as libc::c_int as usize])
                as libc::c_longlong,
        );
        fields += 1;
        addReplyBulkCString(
            c,
            b"num-other-sentinels\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(
            c,
            ((*(*ri).sentinels).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*ri).sentinels).ht_used[1 as libc::c_int as usize])
                as libc::c_longlong,
        );
        fields += 1;
        addReplyBulkCString(c, b"quorum\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*ri).quorum as libc::c_longlong);
        fields += 1;
        addReplyBulkCString(
            c,
            b"failover-timeout\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, (*ri).failover_timeout);
        fields += 1;
        addReplyBulkCString(c, b"parallel-syncs\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*ri).parallel_syncs as libc::c_longlong);
        fields += 1;
        if !((*ri).notification_script).is_null() {
            addReplyBulkCString(
                c,
                b"notification-script\0" as *const u8 as *const libc::c_char,
            );
            addReplyBulkCString(c, (*ri).notification_script);
            fields += 1;
        }
        if !((*ri).client_reconfig_script).is_null() {
            addReplyBulkCString(
                c,
                b"client-reconfig-script\0" as *const u8 as *const libc::c_char,
            );
            addReplyBulkCString(c, (*ri).client_reconfig_script);
            fields += 1;
        }
    }
    if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        addReplyBulkCString(
            c,
            b"master-link-down-time\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, (*ri).master_link_down_time);
        fields += 1;
        addReplyBulkCString(
            c,
            b"master-link-status\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkCString(
            c,
            if (*ri).slave_master_link_status == 0 as libc::c_int {
                b"ok\0" as *const u8 as *const libc::c_char
            } else {
                b"err\0" as *const u8 as *const libc::c_char
            },
        );
        fields += 1;
        addReplyBulkCString(c, b"master-host\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !((*ri).slave_master_host).is_null() {
                (*ri).slave_master_host as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
        );
        fields += 1;
        addReplyBulkCString(c, b"master-port\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*ri).slave_master_port as libc::c_longlong);
        fields += 1;
        addReplyBulkCString(c, b"slave-priority\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*ri).slave_priority as libc::c_longlong);
        fields += 1;
        addReplyBulkCString(
            c,
            b"slave-repl-offset\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, (*ri).slave_repl_offset as libc::c_longlong);
        fields += 1;
        addReplyBulkCString(
            c,
            b"replica-announced\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, (*ri).replica_announced as libc::c_longlong);
        fields += 1;
    }
    if (*ri).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        addReplyBulkCString(
            c,
            b"last-hello-message\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, mstime() - (*ri).last_hello_time);
        fields += 1;
        addReplyBulkCString(c, b"voted-leader\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !((*ri).leader).is_null() {
                (*ri).leader as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
        );
        fields += 1;
        addReplyBulkCString(
            c,
            b"voted-leader-epoch\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkLongLong(c, (*ri).leader_epoch as libc::c_longlong);
        fields += 1;
    }
    setDeferredMapLen(c, mbl, fields as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSetDebugConfigParameters(mut c: *mut client) {
    let mut current_block: u64;
    let mut j: libc::c_int = 0;
    let mut badarg: libc::c_int = 0 as libc::c_int;
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    j = 2 as libc::c_int;
    loop {
        if !(j < (*c).argc) {
            current_block = 8869332144787829186;
            break;
        }
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        option = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
        let mut ll: libc::c_longlong = 0;
        if strcasecmp(option, b"info-period\0" as *const u8 as *const libc::c_char) == 0
            && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_info_period = ll;
            }
        } else if strcasecmp(
            option,
            b"ping-period\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_0: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_0, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_ping_period = ll;
            }
        } else if strcasecmp(option, b"ask-period\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_1: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_1, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_ask_period = ll;
            }
        } else if strcasecmp(
            option,
            b"publish-period\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_2: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_2, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_publish_period = ll;
            }
        } else if strcasecmp(
            option,
            b"default-down-after\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_3: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_3, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_default_down_after = ll;
            }
        } else if strcasecmp(
            option,
            b"tilt-trigger\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_4: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_4, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_tilt_trigger = ll;
            }
        } else if strcasecmp(
            option,
            b"tilt-period\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_5: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_5, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_tilt_period = ll;
            }
        } else if strcasecmp(
            option,
            b"slave-reconf-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_6: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_6, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_slave_reconf_timeout = ll;
            }
        } else if strcasecmp(
            option,
            b"min-link-reconnect-period\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_7: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_7, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_min_link_reconnect_period = ll;
            }
        } else if strcasecmp(
            option,
            b"default-failover-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_8: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_8, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_default_failover_timeout = ll;
            }
        } else if strcasecmp(
            option,
            b"election-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_9: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_9, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_election_timeout = ll;
            }
        } else if strcasecmp(
            option,
            b"script-max-runtime\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_10: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_10, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_script_max_runtime = ll;
            }
        } else if strcasecmp(
            option,
            b"script-retry-delay\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_11: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_11, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 9460579968316997561;
                break;
            } else {
                sentinel_script_retry_delay = ll;
            }
        } else {
            addReplyErrorFormat(
                c,
                b"Unknown option or number of arguments for SENTINEL DEBUG '%s'\0"
                    as *const u8 as *const libc::c_char,
                option,
            );
            return;
        }
        j += 1;
    }
    match current_block {
        9460579968316997561 => {
            addReplyErrorFormat(
                c,
                b"Invalid argument '%s' for SENTINEL DEBUG '%s'\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(badarg as isize)).ptr as *mut libc::c_char,
                option,
            );
            return;
        }
        _ => {
            addReply(c, shared.ok);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplySentinelDebugInfo(mut c: *mut client) {
    let mut mbl: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut fields: libc::c_int = 0 as libc::c_int;
    mbl = addReplyDeferredLen(c);
    addReplyBulkCString(c, b"INFO-PERIOD\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_info_period);
    fields += 1;
    addReplyBulkCString(c, b"PING-PERIOD\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_ping_period);
    fields += 1;
    addReplyBulkCString(c, b"ASK-PERIOD\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_ask_period);
    fields += 1;
    addReplyBulkCString(c, b"PUBLISH-PERIOD\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_publish_period);
    fields += 1;
    addReplyBulkCString(c, b"DEFAULT-DOWN-AFTER\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_default_down_after);
    fields += 1;
    addReplyBulkCString(
        c,
        b"DEFAULT-FAILOVER-TIMEOUT\0" as *const u8 as *const libc::c_char,
    );
    addReplyBulkLongLong(c, sentinel_default_failover_timeout);
    fields += 1;
    addReplyBulkCString(c, b"TILT-TRIGGER\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_tilt_trigger);
    fields += 1;
    addReplyBulkCString(c, b"TILT-PERIOD\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_tilt_period);
    fields += 1;
    addReplyBulkCString(
        c,
        b"SLAVE-RECONF-TIMEOUT\0" as *const u8 as *const libc::c_char,
    );
    addReplyBulkLongLong(c, sentinel_slave_reconf_timeout);
    fields += 1;
    addReplyBulkCString(
        c,
        b"MIN-LINK-RECONNECT-PERIOD\0" as *const u8 as *const libc::c_char,
    );
    addReplyBulkLongLong(c, sentinel_min_link_reconnect_period);
    fields += 1;
    addReplyBulkCString(c, b"ELECTION-TIMEOUT\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_election_timeout);
    fields += 1;
    addReplyBulkCString(c, b"SCRIPT-MAX-RUNTIME\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_script_max_runtime);
    fields += 1;
    addReplyBulkCString(c, b"SCRIPT-RETRY-DELAY\0" as *const u8 as *const libc::c_char);
    addReplyBulkLongLong(c, sentinel_script_retry_delay);
    fields += 1;
    setDeferredMapLen(c, mbl, fields as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyDictOfRedisInstances(
    mut c: *mut client,
    mut instances: *mut dict,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut slaves: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    di = dictGetIterator(instances);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if (*ri).flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            && (*ri).replica_announced == 0
        {
            continue;
        }
        addReplySentinelRedisInstance(c, ri);
        slaves += 1;
    }
    dictReleaseIterator(di);
    setDeferredArrayLen(c, replylen, slaves);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGetMasterByNameOrReplyError(
    mut c: *mut client,
    mut name: *mut robj,
) -> *mut sentinelRedisInstance {
    let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    ri = dictFetchValue(sentinel.masters, (*name).ptr) as *mut sentinelRedisInstance;
    if ri.is_null() {
        addReplyError(
            c,
            b"No such master with that name\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut sentinelRedisInstance;
    }
    return ri;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelIsQuorumReachable(
    mut master: *mut sentinelRedisInstance,
    mut usableptr: *mut libc::c_int,
) -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut usable: libc::c_int = 1 as libc::c_int;
    let mut result: libc::c_int = 0 as libc::c_int;
    let mut voters: libc::c_int = ((*(*master).sentinels)
        .ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*master).sentinels).ht_used[1 as libc::c_int as usize])
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    di = dictGetIterator((*master).sentinels);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if (*ri).flags
            & ((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            continue;
        }
        usable += 1;
    }
    dictReleaseIterator(di);
    if usable < (*master).quorum as libc::c_int {
        result |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if usable < voters / 2 as libc::c_int + 1 as libc::c_int {
        result |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if !usableptr.is_null() {
        *usableptr = usable;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCommand(mut c: *mut client) {
    let mut current_block: u64;
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 47] = [
            b"CKQUORUM <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Check if the current Sentinel configuration is able to reach the quorum\0"
                as *const u8 as *const libc::c_char,
            b"    needed to failover a master and the majority needed to authorize the\0"
                as *const u8 as *const libc::c_char,
            b"    failover.\0" as *const u8 as *const libc::c_char,
            b"CONFIG SET <param> <value>\0" as *const u8 as *const libc::c_char,
            b"    Set a global Sentinel configuration parameter.\0" as *const u8
                as *const libc::c_char,
            b"CONFIG GET <param>\0" as *const u8 as *const libc::c_char,
            b"    Get global Sentinel configuration parameter.\0" as *const u8
                as *const libc::c_char,
            b"DEBUG [<param> <value> ...]\0" as *const u8 as *const libc::c_char,
            b"    Show a list of configurable time parameters and their values (milliseconds).\0"
                as *const u8 as *const libc::c_char,
            b"    Or update current configurable parameters values (one or more).\0"
                as *const u8 as *const libc::c_char,
            b"GET-MASTER-ADDR-BY-NAME <master-name>\0" as *const u8
                as *const libc::c_char,
            b"    Return the ip and port number of the master with that name.\0"
                as *const u8 as *const libc::c_char,
            b"FAILOVER <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Manually failover a master node without asking for agreement from other\0"
                as *const u8 as *const libc::c_char,
            b"    Sentinels\0" as *const u8 as *const libc::c_char,
            b"FLUSHCONFIG\0" as *const u8 as *const libc::c_char,
            b"    Force Sentinel to rewrite its configuration on disk, including the current\0"
                as *const u8 as *const libc::c_char,
            b"    Sentinel state.\0" as *const u8 as *const libc::c_char,
            b"INFO-CACHE <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Return last cached INFO output from masters and all its replicas.\0"
                as *const u8 as *const libc::c_char,
            b"IS-MASTER-DOWN-BY-ADDR <ip> <port> <current-epoch> <runid>\0" as *const u8
                as *const libc::c_char,
            b"    Check if the master specified by ip:port is down from current Sentinel's\0"
                as *const u8 as *const libc::c_char,
            b"    point of view.\0" as *const u8 as *const libc::c_char,
            b"MASTER <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Show the state and info of the specified master.\0" as *const u8
                as *const libc::c_char,
            b"MASTERS\0" as *const u8 as *const libc::c_char,
            b"    Show a list of monitored masters and their state.\0" as *const u8
                as *const libc::c_char,
            b"MONITOR <name> <ip> <port> <quorum>\0" as *const u8 as *const libc::c_char,
            b"    Start monitoring a new master with the specified name, ip, port and quorum.\0"
                as *const u8 as *const libc::c_char,
            b"MYID\0" as *const u8 as *const libc::c_char,
            b"    Return the ID of the Sentinel instance.\0" as *const u8
                as *const libc::c_char,
            b"PENDING-SCRIPTS\0" as *const u8 as *const libc::c_char,
            b"    Get pending scripts information.\0" as *const u8
                as *const libc::c_char,
            b"REMOVE <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Remove master from Sentinel's monitor list.\0" as *const u8
                as *const libc::c_char,
            b"REPLICAS <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Show a list of replicas for this master and their state.\0"
                as *const u8 as *const libc::c_char,
            b"RESET <pattern>\0" as *const u8 as *const libc::c_char,
            b"    Reset masters for specific master name matching this pattern.\0"
                as *const u8 as *const libc::c_char,
            b"SENTINELS <master-name>\0" as *const u8 as *const libc::c_char,
            b"    Show a list of Sentinel instances for this master and their state.\0"
                as *const u8 as *const libc::c_char,
            b"SET <master-name> <option> <value> [<option> <value> ...]\0" as *const u8
                as *const libc::c_char,
            b"    Set configuration parameters for certain masters.\0" as *const u8
                as *const libc::c_char,
            b"SIMULATE-FAILURE [CRASH-AFTER-ELECTION] [CRASH-AFTER-PROMOTION] [HELP]\0"
                as *const u8 as *const libc::c_char,
            b"    Simulate a Sentinel crash.\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else {
        if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"masters\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc != 2 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                addReplyDictOfRedisInstances(c, sentinel.masters);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"master\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri.is_null() {
                    return;
                }
                addReplySentinelRedisInstance(c, ri);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"slaves\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"replicas\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            let mut ri_0: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_0 = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri_0.is_null() {
                    return;
                }
                addReplyDictOfRedisInstances(c, (*ri_0).slaves);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"sentinels\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_1: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_1 = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri_1.is_null() {
                    return;
                }
                addReplyDictOfRedisInstances(c, (*ri_1).sentinels);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"myid\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc == 2 as libc::c_int
        {
            addReplyBulkCBuffer(
                c,
                (sentinel.myid).as_mut_ptr() as *const libc::c_void,
                40 as libc::c_int as size_t,
            );
            current_block = 9846950269610550213;
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"is-master-down-by-addr\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_2: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            let mut req_epoch: libc::c_longlong = 0;
            let mut leader_epoch: uint64_t = 0 as libc::c_int as uint64_t;
            let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut port: libc::c_long = 0;
            let mut isdown: libc::c_int = 0 as libc::c_int;
            if (*c).argc != 6 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                if getLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(3 as libc::c_int as isize),
                    &mut port,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                    || getLongLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset(4 as libc::c_int as isize),
                        &mut req_epoch,
                        0 as *const libc::c_char,
                    ) != 0 as libc::c_int
                {
                    return;
                }
                ri_2 = getSentinelRedisInstanceByAddrAndRunID(
                    sentinel.masters,
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                    port as libc::c_int,
                    0 as *mut libc::c_char,
                );
                if sentinel.tilt == 0 && !ri_2.is_null()
                    && (*ri_2).flags & (1 as libc::c_int) << 3 as libc::c_int != 0
                    && (*ri_2).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                {
                    isdown = 1 as libc::c_int;
                }
                if !ri_2.is_null()
                    && (*ri_2).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
                    && strcasecmp(
                        (**((*c).argv).offset(5 as libc::c_int as isize)).ptr
                            as *const libc::c_char,
                        b"*\0" as *const u8 as *const libc::c_char,
                    ) != 0
                {
                    leader = sentinelVoteLeader(
                        ri_2,
                        req_epoch as uint64_t,
                        (**((*c).argv).offset(5 as libc::c_int as isize)).ptr
                            as *mut libc::c_char,
                        &mut leader_epoch,
                    );
                }
                addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
                addReply(c, if isdown != 0 { shared.cone } else { shared.czero });
                addReplyBulkCString(
                    c,
                    if !leader.is_null() {
                        leader as *const libc::c_char
                    } else {
                        b"*\0" as *const u8 as *const libc::c_char
                    },
                );
                addReplyLongLong(c, leader_epoch as libc::c_longlong);
                if !leader.is_null() {
                    sdsfree(leader);
                }
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"reset\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                addReplyLongLong(
                    c,
                    sentinelResetMastersByPattern(
                        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                            as *mut libc::c_char,
                        (1 as libc::c_int) << 16 as libc::c_int,
                    ) as libc::c_longlong,
                );
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"get-master-addr-by-name\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_3: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_3 = sentinelGetMasterByName(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                if ri_3.is_null() {
                    addReplyNullArray(c);
                } else {
                    let mut addr: *mut sentinelAddr = sentinelGetCurrentMasterAddress(
                        ri_3,
                    );
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                    addReplyBulkCString(c, announceSentinelAddr(addr));
                    addReplyBulkLongLong(c, (*addr).port as libc::c_longlong);
                }
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"failover\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_4: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_4 = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri_4.is_null() {
                    return;
                }
                if (*ri_4).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                    addReplyError(
                        c,
                        b"-INPROG Failover already in progress\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                if (sentinelSelectSlave(ri_4)).is_null() {
                    addReplyError(
                        c,
                        b"-NOGOODSLAVE No suitable replica to promote\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Executing user requested FAILOVER of '%s'\0" as *const u8
                            as *const libc::c_char,
                        (*ri_4).name,
                    );
                }
                sentinelStartFailover(ri_4);
                (*ri_4).flags |= (1 as libc::c_int) << 11 as libc::c_int;
                addReply(c, shared.ok);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"pending-scripts\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc != 2 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                sentinelPendingScriptsCommand(c);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"monitor\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_5: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            let mut quorum: libc::c_long = 0;
            let mut port_0: libc::c_long = 0;
            let mut ip: [libc::c_char; 46] = [0; 46];
            if (*c).argc != 6 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                if getLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(5 as libc::c_int as isize),
                    &mut quorum,
                    b"Invalid quorum\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    return;
                }
                if getLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(4 as libc::c_int as isize),
                    &mut port_0,
                    b"Invalid port\0" as *const u8 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    return;
                }
                if quorum <= 0 as libc::c_int as libc::c_long {
                    addReplyError(
                        c,
                        b"Quorum must be 1 or greater.\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                if anetResolve(
                    0 as *mut libc::c_char,
                    (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                    ip.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                    (if sentinel.resolve_hostnames != 0 {
                        0 as libc::c_int
                    } else {
                        (1 as libc::c_int) << 0 as libc::c_int
                    }),
                ) == -(1 as libc::c_int)
                {
                    addReplyError(
                        c,
                        b"Invalid IP address or hostname specified\0" as *const u8
                            as *const libc::c_char,
                    );
                    return;
                }
                ri_5 = createSentinelRedisInstance(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                    (1 as libc::c_int) << 0 as libc::c_int,
                    (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                    port_0 as libc::c_int,
                    quorum as libc::c_int,
                    0 as *mut sentinelRedisInstance,
                );
                if ri_5.is_null() {
                    addReplyError(
                        c,
                        sentinelCheckCreateInstanceErrors(
                            (1 as libc::c_int) << 0 as libc::c_int,
                        ),
                    );
                } else {
                    sentinelFlushConfigAndReply(c);
                    sentinelEvent(
                        3 as libc::c_int,
                        b"+monitor\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        ri_5,
                        b"%@ quorum %d\0" as *const u8 as *const libc::c_char,
                        (*ri_5).quorum,
                    );
                }
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"flushconfig\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc != 2 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                sentinelFlushConfigAndReply(c);
                return;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"remove\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_6: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_6 = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri_6.is_null() {
                    return;
                }
                sentinelEvent(
                    3 as libc::c_int,
                    b"-monitor\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    ri_6,
                    b"%@\0" as *const u8 as *const libc::c_char,
                );
                dictDelete(
                    sentinel.masters,
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
                );
                sentinelFlushConfigAndReply(c);
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"ckquorum\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut ri_7: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
            let mut usable: libc::c_int = 0;
            if (*c).argc != 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                ri_7 = sentinelGetMasterByNameOrReplyError(
                    c,
                    *((*c).argv).offset(2 as libc::c_int as isize),
                );
                if ri_7.is_null() {
                    return;
                }
                let mut result: libc::c_int = sentinelIsQuorumReachable(
                    ri_7,
                    &mut usable,
                );
                if result == 0 as libc::c_int {
                    addReplySds(
                        c,
                        sdscatfmt(
                            sdsempty(),
                            b"+OK %i usable Sentinels. Quorum and failover authorization can be reached\r\n\0"
                                as *const u8 as *const libc::c_char,
                            usable,
                        ),
                    );
                } else {
                    let mut e: sds = sdscatfmt(
                        sdsempty(),
                        b"-NOQUORUM %i usable Sentinels. \0" as *const u8
                            as *const libc::c_char,
                        usable,
                    );
                    if result & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                        e = sdscat(
                            e,
                            b"Not enough available Sentinels to reach the specified quorum for this master\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    if result & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                        if result & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                            e = sdscat(e, b". \0" as *const u8 as *const libc::c_char);
                        }
                        e = sdscat(
                            e,
                            b"Not enough available Sentinels to reach the majority and authorize a failover\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    addReplyErrorSds(c, e);
                }
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"set\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            sentinelSetCommand(c);
            current_block = 9846950269610550213;
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"config\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc < 3 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                if strcasecmp(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    b"set\0" as *const u8 as *const libc::c_char,
                ) == 0 && (*c).argc == 5 as libc::c_int
                {
                    sentinelConfigSetCommand(c);
                } else if strcasecmp(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    b"get\0" as *const u8 as *const libc::c_char,
                ) == 0 && (*c).argc == 4 as libc::c_int
                {
                    sentinelConfigGetCommand(c);
                } else {
                    addReplyError(
                        c,
                        b"Only SENTINEL CONFIG GET <option> / SET <option> <value> are supported.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                current_block = 9846950269610550213;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"info-cache\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).argc < 2 as libc::c_int {
                current_block = 10347061906550722908;
            } else {
                let mut now: mstime_t = mstime();
                let mut copy_keeper: dictType = instancesDictType;
                copy_keeper.valDestructor = None;
                let mut masters_local: *mut dict = sentinel.masters;
                if (*c).argc > 2 as libc::c_int {
                    masters_local = dictCreate(&mut copy_keeper);
                    let mut i: libc::c_int = 2 as libc::c_int;
                    while i < (*c).argc {
                        let mut ri_8: *mut sentinelRedisInstance = 0
                            as *mut sentinelRedisInstance;
                        ri_8 = sentinelGetMasterByName(
                            (**((*c).argv).offset(i as isize)).ptr as *mut libc::c_char,
                        );
                        if !ri_8.is_null() {
                            dictAdd(
                                masters_local,
                                (*ri_8).name as *mut libc::c_void,
                                ri_8 as *mut libc::c_void,
                            );
                        }
                        i += 1;
                    }
                }
                addReplyArrayLen(
                    c,
                    ((*masters_local).ht_used[0 as libc::c_int as usize])
                        .wrapping_add(
                            (*masters_local).ht_used[1 as libc::c_int as usize],
                        )
                        .wrapping_mul(2 as libc::c_int as libc::c_ulong) as libc::c_long,
                );
                let mut di: *mut dictIterator = 0 as *mut dictIterator;
                let mut de: *mut dictEntry = 0 as *mut dictEntry;
                di = dictGetIterator(masters_local);
                loop {
                    de = dictNext(di);
                    if de.is_null() {
                        break;
                    }
                    let mut ri_9: *mut sentinelRedisInstance = (*de).v.val
                        as *mut sentinelRedisInstance;
                    addReplyBulkCBuffer(
                        c,
                        (*ri_9).name as *const libc::c_void,
                        strlen((*ri_9).name),
                    );
                    addReplyArrayLen(
                        c,
                        ((*(*ri_9).slaves).ht_used[0 as libc::c_int as usize])
                            .wrapping_add(
                                (*(*ri_9).slaves).ht_used[1 as libc::c_int as usize],
                            )
                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_long,
                    );
                    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                    addReplyLongLong(
                        c,
                        if (*ri_9).info_refresh != 0 {
                            now - (*ri_9).info_refresh
                        } else {
                            0 as libc::c_int as libc::c_longlong
                        },
                    );
                    if !((*ri_9).info).is_null() {
                        addReplyBulkCBuffer(
                            c,
                            (*ri_9).info as *const libc::c_void,
                            sdslen((*ri_9).info),
                        );
                    } else {
                        addReplyNull(c);
                    }
                    let mut sdi: *mut dictIterator = 0 as *mut dictIterator;
                    let mut sde: *mut dictEntry = 0 as *mut dictEntry;
                    sdi = dictGetIterator((*ri_9).slaves);
                    loop {
                        sde = dictNext(sdi);
                        if sde.is_null() {
                            break;
                        }
                        let mut sri: *mut sentinelRedisInstance = (*sde).v.val
                            as *mut sentinelRedisInstance;
                        addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                        addReplyLongLong(
                            c,
                            if (*ri_9).info_refresh != 0 {
                                now - (*sri).info_refresh
                            } else {
                                0 as libc::c_int as libc::c_longlong
                            },
                        );
                        if !((*sri).info).is_null() {
                            addReplyBulkCBuffer(
                                c,
                                (*sri).info as *const libc::c_void,
                                sdslen((*sri).info),
                            );
                        } else {
                            addReplyNull(c);
                        }
                    }
                    dictReleaseIterator(sdi);
                }
                dictReleaseIterator(di);
                if masters_local != sentinel.masters {
                    dictRelease(masters_local);
                }
                current_block = 9846950269610550213;
            }
        } else {
            if strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"simulate-failure\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                let mut j: libc::c_int = 0;
                sentinel.simfailure_flags = 0 as libc::c_int as libc::c_ulong;
                j = 2 as libc::c_int;
                while j < (*c).argc {
                    if strcasecmp(
                        (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                        b"crash-after-election\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        sentinel.simfailure_flags
                            |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failure simulation: this Sentinel will crash after being successfully elected as failover leader\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    } else if strcasecmp(
                        (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                        b"crash-after-promotion\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        sentinel.simfailure_flags
                            |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failure simulation: this Sentinel will crash after promoting the selected replica to master\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    } else if strcasecmp(
                        (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                        b"help\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                        addReplyBulkCString(
                            c,
                            b"crash-after-election\0" as *const u8 as *const libc::c_char,
                        );
                        addReplyBulkCString(
                            c,
                            b"crash-after-promotion\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    } else {
                        addReplyError(
                            c,
                            b"Unknown failure simulation specified\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    }
                    j += 1;
                }
                addReply(c, shared.ok);
            } else if strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"debug\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if (*c).argc == 2 as libc::c_int {
                    addReplySentinelDebugInfo(c);
                } else {
                    sentinelSetDebugConfigParameters(c);
                }
            } else {
                addReplySubcommandSyntaxError(c);
            }
            current_block = 9846950269610550213;
        }
        match current_block {
            9846950269610550213 => {}
            _ => {
                addReplyErrorArity(c);
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelInfoCommand(mut c: *mut client) {
    let mut sentinel_sections: [*mut libc::c_char; 6] = [
        b"server\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"clients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cpu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"sentinel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    let mut sec_all: libc::c_int = 0 as libc::c_int;
    let mut sec_everything: libc::c_int = 0 as libc::c_int;
    static mut cached_all_info_sections: *mut dict = 0 as *const dict as *mut dict;
    let mut sections_dict: *mut dict = genInfoSectionDict(
        ((*c).argv).offset(1 as libc::c_int as isize),
        (*c).argc - 1 as libc::c_int,
        sentinel_sections.as_mut_ptr(),
        &mut sec_all,
        &mut sec_everything,
    );
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetSafeIterator(sections_dict);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut i: libc::c_int = 0;
        let mut sec: sds = (*de).key as sds;
        i = 0 as libc::c_int;
        while !(sentinel_sections[i as usize]).is_null() {
            if strcasecmp(sentinel_sections[i as usize], sec as *const libc::c_char) == 0
            {
                break;
            }
            i += 1;
        }
        if (sentinel_sections[i as usize]).is_null() {
            dictDelete(sections_dict, sec as *const libc::c_void);
        }
    }
    dictReleaseIterator(di);
    if sec_all != 0 || sec_everything != 0 {
        releaseInfoSectionDict(sections_dict);
        if cached_all_info_sections.is_null() {
            cached_all_info_sections = dictCreate(&mut stringSetDictType);
            addInfoSectionsToDict(
                cached_all_info_sections,
                sentinel_sections.as_mut_ptr(),
            );
        }
        sections_dict = cached_all_info_sections;
    }
    let mut info: sds = genRedisInfoString(
        sections_dict,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    if sec_all != 0
        || !(dictFind(
            sections_dict,
            b"sentinel\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let mut di_0: *mut dictIterator = 0 as *mut dictIterator;
        let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
        let mut master_id: libc::c_int = 0 as libc::c_int;
        if sdslen(info) != 0 as libc::c_int as libc::c_ulong {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Sentinel\r\nsentinel_masters:%lu\r\nsentinel_tilt:%d\r\nsentinel_tilt_since_seconds:%jd\r\nsentinel_running_scripts:%d\r\nsentinel_scripts_queue_length:%ld\r\nsentinel_simulate_failure_flags:%lu\r\n\0"
                as *const u8 as *const libc::c_char,
            ((*sentinel.masters).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*sentinel.masters).ht_used[1 as libc::c_int as usize]),
            sentinel.tilt,
            if sentinel.tilt != 0 {
                ((mstime() - sentinel.tilt_start_time)
                    / 1000 as libc::c_int as libc::c_longlong) as intmax_t
            } else {
                -(1 as libc::c_int) as libc::c_long
            },
            sentinel.running_scripts,
            (*sentinel.scripts_queue).len,
            sentinel.simfailure_flags,
        );
        di_0 = dictGetIterator(sentinel.masters);
        loop {
            de_0 = dictNext(di_0);
            if de_0.is_null() {
                break;
            }
            let mut ri: *mut sentinelRedisInstance = (*de_0).v.val
                as *mut sentinelRedisInstance;
            let mut status: *mut libc::c_char = b"ok\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
            if (*ri).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                status = b"odown\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                status = b"sdown\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            let fresh8 = master_id;
            master_id = master_id + 1;
            info = sdscatprintf(
                info,
                b"master%d:name=%s,status=%s,address=%s:%d,slaves=%lu,sentinels=%lu\r\n\0"
                    as *const u8 as *const libc::c_char,
                fresh8,
                (*ri).name,
                status,
                announceSentinelAddr((*ri).addr),
                (*(*ri).addr).port,
                ((*(*ri).slaves).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*(*ri).slaves).ht_used[1 as libc::c_int as usize]),
                ((*(*ri).sentinels).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*(*ri).sentinels).ht_used[1 as libc::c_int as usize])
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        dictReleaseIterator(di_0);
    }
    if sections_dict != cached_all_info_sections {
        releaseInfoSectionDict(sections_dict);
    }
    addReplyBulkSds(c, info);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelRoleCommand(mut c: *mut client) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
    addReplyBulkCBuffer(
        c,
        b"sentinel\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        8 as libc::c_int as size_t,
    );
    addReplyArrayLen(
        c,
        ((*sentinel.masters).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*sentinel.masters).ht_used[1 as libc::c_int as usize])
            as libc::c_long,
    );
    di = dictGetIterator(sentinel.masters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        addReplyBulkCString(c, (*ri).name);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSetCommand(mut c: *mut client) {
    let mut current_block: u64;
    let mut ri: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut j: libc::c_int = 0;
    let mut changes: libc::c_int = 0 as libc::c_int;
    let mut badarg: libc::c_int = 0 as libc::c_int;
    let mut option: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut redacted: libc::c_int = 0;
    ri = sentinelGetMasterByNameOrReplyError(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
    );
    if ri.is_null() {
        return;
    }
    j = 3 as libc::c_int;
    loop {
        if !(j < (*c).argc) {
            current_block = 17441561948628420366;
            break;
        }
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        option = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
        let mut ll: libc::c_longlong = 0;
        let mut old_j: libc::c_int = j;
        redacted = 0 as libc::c_int;
        if strcasecmp(
            option,
            b"down-after-milliseconds\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 4920003593487925665;
                break;
            } else {
                (*ri).down_after_period = ll;
                sentinelPropagateDownAfterPeriod(ri);
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"failover-timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_0: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_0, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 4920003593487925665;
                break;
            } else {
                (*ri).failover_timeout = ll;
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"parallel-syncs\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_1: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_1, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 4920003593487925665;
                break;
            } else {
                (*ri).parallel_syncs = ll as libc::c_int;
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"notification-script\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut value: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            if sentinel.deny_scripts_reconfig != 0 {
                addReplyError(
                    c,
                    b"Reconfiguration of scripts path is denied for security reasons. Check the deny-scripts-reconfig configuration directive in your Sentinel configuration\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 11951393851839785357;
                break;
            } else if strlen(value) != 0
                && access(value, 1 as libc::c_int) == -(1 as libc::c_int)
            {
                addReplyError(
                    c,
                    b"Notification script seems non existing or non executable\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 11951393851839785357;
                break;
            } else {
                sdsfree((*ri).notification_script);
                (*ri)
                    .notification_script = if strlen(value) != 0 {
                    sdsnew(value)
                } else {
                    0 as sds
                };
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"client-reconfig-script\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut value_0: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            if sentinel.deny_scripts_reconfig != 0 {
                addReplyError(
                    c,
                    b"Reconfiguration of scripts path is denied for security reasons. Check the deny-scripts-reconfig configuration directive in your Sentinel configuration\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 11951393851839785357;
                break;
            } else if strlen(value_0) != 0
                && access(value_0, 1 as libc::c_int) == -(1 as libc::c_int)
            {
                addReplyError(
                    c,
                    b"Client reconfiguration script seems non existing or non executable\0"
                        as *const u8 as *const libc::c_char,
                );
                current_block = 11951393851839785357;
                break;
            } else {
                sdsfree((*ri).client_reconfig_script);
                (*ri)
                    .client_reconfig_script = if strlen(value_0) != 0 {
                    sdsnew(value_0)
                } else {
                    0 as sds
                };
                changes += 1;
            }
        } else if strcasecmp(option, b"auth-pass\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut value_1: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            sdsfree((*ri).auth_pass);
            (*ri)
                .auth_pass = if strlen(value_1) != 0 {
                sdsnew(value_1)
            } else {
                0 as sds
            };
            dropInstanceConnections(ri);
            changes += 1;
            redacted = 1 as libc::c_int;
        } else if strcasecmp(option, b"auth-user\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut value_2: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            sdsfree((*ri).auth_user);
            (*ri)
                .auth_user = if strlen(value_2) != 0 {
                sdsnew(value_2)
            } else {
                0 as sds
            };
            dropInstanceConnections(ri);
            changes += 1;
        } else if strcasecmp(option, b"quorum\0" as *const u8 as *const libc::c_char)
            == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_2: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_2, &mut ll) == -(1 as libc::c_int)
                || ll <= 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 4920003593487925665;
                break;
            } else {
                (*ri).quorum = ll as libc::c_uint;
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"rename-command\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 1 as libc::c_int
        {
            j += 1;
            let mut oldname: sds = (**((*c).argv).offset(j as isize)).ptr as sds;
            j += 1;
            let mut newname: sds = (**((*c).argv).offset(j as isize)).ptr as sds;
            if sdslen(oldname) == 0 as libc::c_int as libc::c_ulong
                || sdslen(newname) == 0 as libc::c_int as libc::c_ulong
            {
                badarg = if sdslen(newname) != 0 { j - 1 as libc::c_int } else { j };
                current_block = 4920003593487925665;
                break;
            } else {
                dictDelete((*ri).renamed_commands, oldname as *const libc::c_void);
                if dictSdsKeyCaseCompare(
                    (*ri).renamed_commands,
                    oldname as *const libc::c_void,
                    newname as *const libc::c_void,
                ) == 0
                {
                    oldname = sdsdup(oldname);
                    newname = sdsdup(newname);
                    dictAdd(
                        (*ri).renamed_commands,
                        oldname as *mut libc::c_void,
                        newname as *mut libc::c_void,
                    );
                }
                changes += 1;
            }
        } else if strcasecmp(
            option,
            b"master-reboot-down-after-period\0" as *const u8 as *const libc::c_char,
        ) == 0 && moreargs > 0 as libc::c_int
        {
            j += 1;
            let mut o_3: *mut robj = *((*c).argv).offset(j as isize);
            if getLongLongFromObject(o_3, &mut ll) == -(1 as libc::c_int)
                || ll < 0 as libc::c_int as libc::c_longlong
            {
                badarg = j;
                current_block = 4920003593487925665;
                break;
            } else {
                (*ri).master_reboot_down_after_period = ll;
                changes += 1;
            }
        } else {
            addReplyErrorFormat(
                c,
                b"Unknown option or number of arguments for SENTINEL SET '%s'\0"
                    as *const u8 as *const libc::c_char,
                option,
            );
            current_block = 11951393851839785357;
            break;
        }
        let mut numargs: libc::c_int = j - old_j + 1 as libc::c_int;
        match numargs {
            2 => {
                sentinelEvent(
                    3 as libc::c_int,
                    b"+set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ri,
                    b"%@ %s %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(old_j as isize)).ptr as *mut libc::c_char,
                    if redacted != 0 {
                        b"******\0" as *const u8 as *const libc::c_char
                    } else {
                        (**((*c).argv).offset((old_j + 1 as libc::c_int) as isize)).ptr
                            as *mut libc::c_char as *const libc::c_char
                    },
                );
            }
            3 => {
                sentinelEvent(
                    3 as libc::c_int,
                    b"+set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ri,
                    b"%@ %s %s %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(old_j as isize)).ptr as *mut libc::c_char,
                    (**((*c).argv).offset((old_j + 1 as libc::c_int) as isize)).ptr
                        as *mut libc::c_char,
                    (**((*c).argv).offset((old_j + 2 as libc::c_int) as isize)).ptr
                        as *mut libc::c_char,
                );
            }
            _ => {
                sentinelEvent(
                    3 as libc::c_int,
                    b"+set\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    ri,
                    b"%@ %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(old_j as isize)).ptr as *mut libc::c_char,
                );
            }
        }
        j += 1;
    }
    match current_block {
        17441561948628420366 => {
            if changes != 0 {
                sentinelFlushConfigAndReply(c);
            }
            return;
        }
        4920003593487925665 => {
            addReplyErrorFormat(
                c,
                b"Invalid argument '%s' for SENTINEL SET '%s'\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(badarg as isize)).ptr as *mut libc::c_char,
                option,
            );
        }
        _ => {}
    }
    if changes != 0 {
        sentinelFlushConfig();
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelPublishCommand(mut c: *mut client) {
    if strcmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"__sentinel__:hello\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        addReplyError(
            c,
            b"Only HELLO messages are accepted by Sentinel instances.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    sentinelProcessHelloMessage(
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
        sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
            as libc::c_int,
    );
    addReplyLongLong(c, 1 as libc::c_int as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCheckSubjectivelyDown(
    mut ri: *mut sentinelRedisInstance,
) {
    let mut elapsed: mstime_t = 0 as libc::c_int as mstime_t;
    if (*(*ri).link).act_ping_time != 0 {
        elapsed = mstime() - (*(*ri).link).act_ping_time;
    } else if (*(*ri).link).disconnected != 0 {
        elapsed = mstime() - (*(*ri).link).last_avail_time;
    }
    if !((*(*ri).link).cc).is_null()
        && mstime() - (*(*ri).link).cc_conn_time > sentinel_min_link_reconnect_period
        && (*(*ri).link).act_ping_time != 0 as libc::c_int as libc::c_longlong
        && mstime() - (*(*ri).link).act_ping_time
            > (*ri).down_after_period / 2 as libc::c_int as libc::c_longlong
        && mstime() - (*(*ri).link).last_pong_time
            > (*ri).down_after_period / 2 as libc::c_int as libc::c_longlong
    {
        instanceLinkCloseConnection((*ri).link, (*(*ri).link).cc);
    }
    if !((*(*ri).link).pc).is_null()
        && mstime() - (*(*ri).link).pc_conn_time > sentinel_min_link_reconnect_period
        && mstime() - (*(*ri).link).pc_last_activity
            > sentinel_publish_period * 3 as libc::c_int as libc::c_longlong
    {
        instanceLinkCloseConnection((*ri).link, (*(*ri).link).pc);
    }
    if elapsed > (*ri).down_after_period
        || (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
            && (*ri).role_reported == (1 as libc::c_int) << 1 as libc::c_int
            && mstime() - (*ri).role_reported_time
                > (*ri).down_after_period
                    + sentinel_info_period * 2 as libc::c_int as libc::c_longlong
        || (*ri).flags & (1 as libc::c_int) << 13 as libc::c_int != 0
            && mstime() - (*ri).master_reboot_since_time
                > (*ri).master_reboot_down_after_period
    {
        if (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int == 0 as libc::c_int {
            sentinelEvent(
                3 as libc::c_int,
                b"+sdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            (*ri).s_down_since_time = mstime();
            (*ri).flags |= (1 as libc::c_int) << 3 as libc::c_int;
        }
    } else if (*ri).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        sentinelEvent(
            3 as libc::c_int,
            b"-sdown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ri,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        (*ri).flags
            &= !((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 12 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCheckObjectivelyDown(
    mut master: *mut sentinelRedisInstance,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut quorum: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut odown: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if (*master).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        quorum = 1 as libc::c_int as libc::c_uint;
        di = dictGetIterator((*master).sentinels);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut ri: *mut sentinelRedisInstance = (*de).v.val
                as *mut sentinelRedisInstance;
            if (*ri).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                quorum = quorum.wrapping_add(1);
            }
        }
        dictReleaseIterator(di);
        if quorum >= (*master).quorum {
            odown = 1 as libc::c_int as libc::c_uint;
        }
    }
    if odown != 0 {
        if (*master).flags & (1 as libc::c_int) << 4 as libc::c_int == 0 as libc::c_int {
            sentinelEvent(
                3 as libc::c_int,
                b"+odown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                master,
                b"%@ #quorum %d/%d\0" as *const u8 as *const libc::c_char,
                quorum,
                (*master).quorum,
            );
            (*master).flags |= (1 as libc::c_int) << 4 as libc::c_int;
            (*master).o_down_since_time = mstime();
        }
    } else if (*master).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        sentinelEvent(
            3 as libc::c_int,
            b"-odown\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            master,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        (*master).flags &= !((1 as libc::c_int) << 4 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelReceiveIsMasterDownReply(
    mut c: *mut redisAsyncContext,
    mut reply: *mut libc::c_void,
    mut privdata: *mut libc::c_void,
) {
    let mut ri: *mut sentinelRedisInstance = privdata as *mut sentinelRedisInstance;
    let mut link: *mut instanceLink = (*c).data as *mut instanceLink;
    let mut r: *mut redisReply = 0 as *mut redisReply;
    if reply.is_null() || link.is_null() {
        return;
    }
    (*link).pending_commands -= 1;
    r = reply as *mut redisReply;
    if (*r).type_0 == 2 as libc::c_int
        && (*r).elements == 3 as libc::c_int as libc::c_ulong
        && (**((*r).element).offset(0 as libc::c_int as isize)).type_0
            == 3 as libc::c_int
        && (**((*r).element).offset(1 as libc::c_int as isize)).type_0
            == 1 as libc::c_int
        && (**((*r).element).offset(2 as libc::c_int as isize)).type_0
            == 3 as libc::c_int
    {
        (*ri).last_master_down_reply_time = mstime();
        if (**((*r).element).offset(0 as libc::c_int as isize)).integer
            == 1 as libc::c_int as libc::c_longlong
        {
            (*ri).flags |= (1 as libc::c_int) << 5 as libc::c_int;
        } else {
            (*ri).flags &= !((1 as libc::c_int) << 5 as libc::c_int);
        }
        if strcmp(
            (**((*r).element).offset(1 as libc::c_int as isize)).str_0,
            b"*\0" as *const u8 as *const libc::c_char,
        ) != 0
        {
            sdsfree((*ri).leader);
            if (*ri).leader_epoch as libc::c_longlong
                != (**((*r).element).offset(2 as libc::c_int as isize)).integer
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"%s voted for %s %llu\0" as *const u8 as *const libc::c_char,
                        (*ri).name,
                        (**((*r).element).offset(1 as libc::c_int as isize)).str_0,
                        (**((*r).element).offset(2 as libc::c_int as isize)).integer
                            as libc::c_ulonglong,
                    );
                }
            }
            (*ri)
                .leader = sdsnew(
                (**((*r).element).offset(1 as libc::c_int as isize)).str_0,
            );
            (*ri)
                .leader_epoch = (**((*r).element).offset(2 as libc::c_int as isize))
                .integer as uint64_t;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelAskMasterStateToOtherSentinels(
    mut master: *mut sentinelRedisInstance,
    mut flags: libc::c_int,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetIterator((*master).sentinels);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        let mut elapsed: mstime_t = mstime() - (*ri).last_master_down_reply_time;
        let mut port: [libc::c_char; 32] = [0; 32];
        let mut retval: libc::c_int = 0;
        if elapsed > sentinel_ask_period * 5 as libc::c_int as libc::c_longlong {
            (*ri).flags &= !((1 as libc::c_int) << 5 as libc::c_int);
            sdsfree((*ri).leader);
            (*ri).leader = 0 as *mut libc::c_char;
        }
        if (*master).flags & (1 as libc::c_int) << 3 as libc::c_int == 0 as libc::c_int {
            continue;
        }
        if (*(*ri).link).disconnected != 0 {
            continue;
        }
        if flags & (1 as libc::c_int) << 0 as libc::c_int == 0
            && mstime() - (*ri).last_master_down_reply_time < sentinel_ask_period
        {
            continue;
        }
        ll2string(
            port.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            (*(*master).addr).port as libc::c_longlong,
        );
        retval = redisAsyncCommand(
            (*(*ri).link).cc,
            Some(
                sentinelReceiveIsMasterDownReply
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ri as *mut libc::c_void,
            b"%s is-master-down-by-addr %s %s %llu %s\0" as *const u8
                as *const libc::c_char,
            sentinelInstanceMapCommand(
                ri,
                b"SENTINEL\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            announceSentinelAddr((*master).addr),
            port.as_mut_ptr(),
            sentinel.current_epoch,
            if (*master).failover_state > 0 as libc::c_int {
                (sentinel.myid).as_mut_ptr() as *const libc::c_char
            } else {
                b"*\0" as *const u8 as *const libc::c_char
            },
        );
        if retval == 0 as libc::c_int {
            (*(*ri).link).pending_commands += 1;
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSimFailureCrash() {
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Sentinel CRASH because of SENTINEL simulate-failure\0" as *const u8
                as *const libc::c_char,
        );
    }
    exit(99 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelVoteLeader(
    mut master: *mut sentinelRedisInstance,
    mut req_epoch: uint64_t,
    mut req_runid: *mut libc::c_char,
    mut leader_epoch: *mut uint64_t,
) -> *mut libc::c_char {
    if req_epoch > sentinel.current_epoch {
        sentinel.current_epoch = req_epoch;
        sentinelFlushConfig();
        sentinelEvent(
            3 as libc::c_int,
            b"+new-epoch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            master,
            b"%llu\0" as *const u8 as *const libc::c_char,
            sentinel.current_epoch as libc::c_ulonglong,
        );
    }
    if (*master).leader_epoch < req_epoch && sentinel.current_epoch <= req_epoch {
        sdsfree((*master).leader);
        (*master).leader = sdsnew(req_runid);
        (*master).leader_epoch = sentinel.current_epoch;
        sentinelFlushConfig();
        sentinelEvent(
            3 as libc::c_int,
            b"+vote-for-leader\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            master,
            b"%s %llu\0" as *const u8 as *const libc::c_char,
            (*master).leader,
            (*master).leader_epoch as libc::c_ulonglong,
        );
        if strcasecmp((*master).leader, (sentinel.myid).as_mut_ptr()) != 0 {
            (*master)
                .failover_start_time = mstime()
                + (rand() % 1000 as libc::c_int) as libc::c_longlong;
        }
    }
    *leader_epoch = (*master).leader_epoch;
    return if !((*master).leader).is_null() {
        sdsnew((*master).leader)
    } else {
        0 as sds
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelLeaderIncr(
    mut counters: *mut dict,
    mut runid: *mut libc::c_char,
) -> libc::c_int {
    let mut existing: *mut dictEntry = 0 as *mut dictEntry;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut oldval: uint64_t = 0;
    de = dictAddRaw(counters, runid as *mut libc::c_void, &mut existing);
    if !existing.is_null() {
        oldval = (*existing).v.u64_0;
        (*existing).v.u64_0 = oldval.wrapping_add(1 as libc::c_int as libc::c_ulong);
        return oldval.wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int;
    } else {
        if !de.is_null() {} else {
            _serverAssert(
                b"de != NULL\0" as *const u8 as *const libc::c_char,
                b"sentinel.c\0" as *const u8 as *const libc::c_char,
                4679 as libc::c_int,
            );
            unreachable!();
        };
        (*de).v.u64_0 = 1 as libc::c_int as uint64_t;
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelGetLeader(
    mut master: *mut sentinelRedisInstance,
    mut epoch: uint64_t,
) -> *mut libc::c_char {
    let mut counters: *mut dict = 0 as *mut dict;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut voters: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut voters_quorum: libc::c_uint = 0;
    let mut myvote: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut winner: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut leader_epoch: uint64_t = 0;
    let mut max_votes: uint64_t = 0 as libc::c_int as uint64_t;
    if (*master).flags
        & ((1 as libc::c_int) << 4 as libc::c_int
            | (1 as libc::c_int) << 6 as libc::c_int) != 0
    {} else {
        _serverAssert(
            b"master->flags & (SRI_O_DOWN|SRI_FAILOVER_IN_PROGRESS)\0" as *const u8
                as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            4701 as libc::c_int,
        );
        unreachable!();
    };
    counters = dictCreate(&mut leaderVotesDictType);
    voters = ((*(*master).sentinels).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*master).sentinels).ht_used[1 as libc::c_int as usize])
        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_uint;
    di = dictGetIterator((*master).sentinels);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if !((*ri).leader).is_null() && (*ri).leader_epoch == sentinel.current_epoch {
            sentinelLeaderIncr(counters, (*ri).leader);
        }
    }
    dictReleaseIterator(di);
    di = dictGetIterator(counters);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut votes: uint64_t = (*de).v.u64_0;
        if votes > max_votes {
            max_votes = votes;
            winner = (*de).key as *mut libc::c_char;
        }
    }
    dictReleaseIterator(di);
    if !winner.is_null() {
        myvote = sentinelVoteLeader(master, epoch, winner, &mut leader_epoch);
    } else {
        myvote = sentinelVoteLeader(
            master,
            epoch,
            (sentinel.myid).as_mut_ptr(),
            &mut leader_epoch,
        );
    }
    if !myvote.is_null() && leader_epoch == epoch {
        let mut votes_0: uint64_t = sentinelLeaderIncr(counters, myvote) as uint64_t;
        if votes_0 > max_votes {
            max_votes = votes_0;
            winner = myvote;
        }
    }
    voters_quorum = voters
        .wrapping_div(2 as libc::c_int as libc::c_uint)
        .wrapping_add(1 as libc::c_int as libc::c_uint);
    if !winner.is_null()
        && (max_votes < voters_quorum as libc::c_ulong
            || max_votes < (*master).quorum as libc::c_ulong)
    {
        winner = 0 as *mut libc::c_char;
    }
    winner = if !winner.is_null() { sdsnew(winner) } else { 0 as sds };
    sdsfree(myvote);
    dictRelease(counters);
    return winner;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSendSlaveOf(
    mut ri: *mut sentinelRedisInstance,
    mut addr: *const sentinelAddr,
) -> libc::c_int {
    let mut portstr: [libc::c_char; 32] = [0; 32];
    let mut host: *const libc::c_char = 0 as *const libc::c_char;
    let mut retval: libc::c_int = 0;
    if addr.is_null() {
        host = b"NO\0" as *const u8 as *const libc::c_char;
        memcpy(
            portstr.as_mut_ptr() as *mut libc::c_void,
            b"ONE\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
    } else {
        host = announceSentinelAddr(addr);
        ll2string(
            portstr.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            (*addr).port as libc::c_longlong,
        );
    }
    retval = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelDiscardReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"MULTI\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    if retval == -(1 as libc::c_int) {
        return retval;
    }
    (*(*ri).link).pending_commands += 1;
    retval = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelDiscardReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s %s %s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"SLAVEOF\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        host,
        portstr.as_mut_ptr(),
    );
    if retval == -(1 as libc::c_int) {
        return retval;
    }
    (*(*ri).link).pending_commands += 1;
    retval = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelDiscardReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s REWRITE\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"CONFIG\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    if retval == -(1 as libc::c_int) {
        return retval;
    }
    (*(*ri).link).pending_commands += 1;
    let mut type_0: libc::c_int = 0 as libc::c_int;
    while type_0 < 2 as libc::c_int {
        retval = redisAsyncCommand(
            (*(*ri).link).cc,
            Some(
                sentinelDiscardReplyCallback
                    as unsafe extern "C" fn(
                        *mut redisAsyncContext,
                        *mut libc::c_void,
                        *mut libc::c_void,
                    ) -> (),
            ),
            ri as *mut libc::c_void,
            b"%s KILL TYPE %s\0" as *const u8 as *const libc::c_char,
            sentinelInstanceMapCommand(
                ri,
                b"CLIENT\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ),
            if type_0 == 0 as libc::c_int {
                b"normal\0" as *const u8 as *const libc::c_char
            } else {
                b"pubsub\0" as *const u8 as *const libc::c_char
            },
        );
        if retval == -(1 as libc::c_int) {
            return retval;
        }
        (*(*ri).link).pending_commands += 1;
        type_0 += 1;
    }
    retval = redisAsyncCommand(
        (*(*ri).link).cc,
        Some(
            sentinelDiscardReplyCallback
                as unsafe extern "C" fn(
                    *mut redisAsyncContext,
                    *mut libc::c_void,
                    *mut libc::c_void,
                ) -> (),
        ),
        ri as *mut libc::c_void,
        b"%s\0" as *const u8 as *const libc::c_char,
        sentinelInstanceMapCommand(
            ri,
            b"EXEC\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
    );
    if retval == -(1 as libc::c_int) {
        return retval;
    }
    (*(*ri).link).pending_commands += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelStartFailover(mut master: *mut sentinelRedisInstance) {
    if (*master).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {} else {
        _serverAssert(
            b"master->flags & SRI_MASTER\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            4835 as libc::c_int,
        );
        unreachable!();
    };
    (*master).failover_state = 1 as libc::c_int;
    (*master).flags |= (1 as libc::c_int) << 6 as libc::c_int;
    sentinel.current_epoch = (sentinel.current_epoch).wrapping_add(1);
    (*master).failover_epoch = sentinel.current_epoch;
    sentinelEvent(
        3 as libc::c_int,
        b"+new-epoch\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        master,
        b"%llu\0" as *const u8 as *const libc::c_char,
        sentinel.current_epoch as libc::c_ulonglong,
    );
    sentinelEvent(
        3 as libc::c_int,
        b"+try-failover\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        master,
        b"%@\0" as *const u8 as *const libc::c_char,
    );
    (*master)
        .failover_start_time = mstime()
        + (rand() % 1000 as libc::c_int) as libc::c_longlong;
    (*master).failover_state_change_time = mstime();
}
#[no_mangle]
pub unsafe extern "C" fn sentinelStartFailoverIfNeeded(
    mut master: *mut sentinelRedisInstance,
) -> libc::c_int {
    if (*master).flags & (1 as libc::c_int) << 4 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    if (*master).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if mstime() - (*master).failover_start_time
        < (*master).failover_timeout * 2 as libc::c_int as libc::c_longlong
    {
        if (*master).failover_delay_logged != (*master).failover_start_time {
            let mut clock: time_t = (((*master).failover_start_time
                + (*master).failover_timeout * 2 as libc::c_int as libc::c_longlong)
                / 1000 as libc::c_int as libc::c_longlong) as time_t;
            let mut ctimebuf: [libc::c_char; 26] = [0; 26];
            ctime_r(&mut clock, ctimebuf.as_mut_ptr());
            ctimebuf[24 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            (*master).failover_delay_logged = (*master).failover_start_time;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Next failover delay: I will not start a failover before %s\0"
                        as *const u8 as *const libc::c_char,
                    ctimebuf.as_mut_ptr(),
                );
            }
        }
        return 0 as libc::c_int;
    }
    sentinelStartFailover(master);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn compareSlavesForPromotion(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut sa: *mut *mut sentinelRedisInstance = a as *mut *mut sentinelRedisInstance;
    let mut sb: *mut *mut sentinelRedisInstance = b as *mut *mut sentinelRedisInstance;
    let mut sa_runid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sb_runid: *mut libc::c_char = 0 as *mut libc::c_char;
    if (**sa).slave_priority != (**sb).slave_priority {
        return (**sa).slave_priority - (**sb).slave_priority;
    }
    if (**sa).slave_repl_offset > (**sb).slave_repl_offset {
        return -(1 as libc::c_int)
    } else {
        if (**sa).slave_repl_offset < (**sb).slave_repl_offset {
            return 1 as libc::c_int;
        }
    }
    sa_runid = (**sa).runid;
    sb_runid = (**sb).runid;
    if sa_runid.is_null() && sb_runid.is_null() {
        return 0 as libc::c_int
    } else {
        if sa_runid.is_null() {
            return 1 as libc::c_int
        } else {
            if sb_runid.is_null() {
                return -(1 as libc::c_int);
            }
        }
    }
    return strcasecmp(sa_runid, sb_runid);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelSelectSlave(
    mut master: *mut sentinelRedisInstance,
) -> *mut sentinelRedisInstance {
    let mut instance: *mut *mut sentinelRedisInstance = zmalloc(
        (core::mem::size_of::<*mut sentinelRedisInstance>() as libc::c_ulong)
            .wrapping_mul(
                ((*(*master).slaves).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*(*master).slaves).ht_used[1 as libc::c_int as usize]),
            ),
    ) as *mut *mut sentinelRedisInstance;
    let mut selected: *mut sentinelRedisInstance = 0 as *mut sentinelRedisInstance;
    let mut instances: libc::c_int = 0 as libc::c_int;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut max_master_down_time: mstime_t = 0 as libc::c_int as mstime_t;
    if (*master).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        max_master_down_time += mstime() - (*master).s_down_since_time;
    }
    max_master_down_time
        += (*master).down_after_period * 10 as libc::c_int as libc::c_longlong;
    di = dictGetIterator((*master).slaves);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut slave: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        let mut info_validity_time: mstime_t = 0;
        if (*slave).flags
            & ((1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) != 0
        {
            continue;
        }
        if (*(*slave).link).disconnected != 0 {
            continue;
        }
        if mstime() - (*(*slave).link).last_avail_time
            > sentinel_ping_period * 5 as libc::c_int as libc::c_longlong
        {
            continue;
        }
        if (*slave).slave_priority == 0 as libc::c_int {
            continue;
        }
        if (*master).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            info_validity_time = sentinel_ping_period
                * 5 as libc::c_int as libc::c_longlong;
        } else {
            info_validity_time = sentinel_info_period
                * 3 as libc::c_int as libc::c_longlong;
        }
        if mstime() - (*slave).info_refresh > info_validity_time {
            continue;
        }
        if (*slave).master_link_down_time > max_master_down_time {
            continue;
        }
        let fresh9 = instances;
        instances = instances + 1;
        let ref mut fresh10 = *instance.offset(fresh9 as isize);
        *fresh10 = slave;
    }
    dictReleaseIterator(di);
    if instances != 0 {
        qsort(
            instance as *mut libc::c_void,
            instances as size_t,
            core::mem::size_of::<*mut sentinelRedisInstance>() as libc::c_ulong,
            Some(
                compareSlavesForPromotion
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
        );
        selected = *instance.offset(0 as libc::c_int as isize);
    }
    zfree(instance as *mut libc::c_void);
    return selected;
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverWaitStart(mut ri: *mut sentinelRedisInstance) {
    let mut leader: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut isleader: libc::c_int = 0;
    leader = sentinelGetLeader(ri, (*ri).failover_epoch);
    isleader = (!leader.is_null()
        && strcasecmp(leader, (sentinel.myid).as_mut_ptr()) == 0 as libc::c_int)
        as libc::c_int;
    sdsfree(leader);
    if isleader == 0 && (*ri).flags & (1 as libc::c_int) << 11 as libc::c_int == 0 {
        let mut election_timeout: mstime_t = sentinel_election_timeout;
        if election_timeout > (*ri).failover_timeout {
            election_timeout = (*ri).failover_timeout;
        }
        if mstime() - (*ri).failover_start_time > election_timeout {
            sentinelEvent(
                3 as libc::c_int,
                b"-failover-abort-not-elected\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            sentinelAbortFailover(ri);
        }
        return;
    }
    sentinelEvent(
        3 as libc::c_int,
        b"+elected-leader\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ri,
        b"%@\0" as *const u8 as *const libc::c_char,
    );
    if sentinel.simfailure_flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
    {
        sentinelSimFailureCrash();
    }
    (*ri).failover_state = 2 as libc::c_int;
    (*ri).failover_state_change_time = mstime();
    sentinelEvent(
        3 as libc::c_int,
        b"+failover-state-select-slave\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        ri,
        b"%@\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverSelectSlave(
    mut ri: *mut sentinelRedisInstance,
) {
    let mut slave: *mut sentinelRedisInstance = sentinelSelectSlave(ri);
    if slave.is_null() {
        sentinelEvent(
            3 as libc::c_int,
            b"-failover-abort-no-good-slave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ri,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        sentinelAbortFailover(ri);
    } else {
        sentinelEvent(
            3 as libc::c_int,
            b"+selected-slave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            slave,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        (*slave).flags |= (1 as libc::c_int) << 7 as libc::c_int;
        (*ri).promoted_slave = slave;
        (*ri).failover_state = 3 as libc::c_int;
        (*ri).failover_state_change_time = mstime();
        sentinelEvent(
            2 as libc::c_int,
            b"+failover-state-send-slaveof-noone\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            slave,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverSendSlaveOfNoOne(
    mut ri: *mut sentinelRedisInstance,
) {
    let mut retval: libc::c_int = 0;
    if (*(*(*ri).promoted_slave).link).disconnected != 0 {
        if mstime() - (*ri).failover_state_change_time > (*ri).failover_timeout {
            sentinelEvent(
                3 as libc::c_int,
                b"-failover-abort-slave-timeout\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                ri,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            sentinelAbortFailover(ri);
        }
        return;
    }
    retval = sentinelSendSlaveOf((*ri).promoted_slave, 0 as *const sentinelAddr);
    if retval != 0 as libc::c_int {
        return;
    }
    sentinelEvent(
        2 as libc::c_int,
        b"+failover-state-wait-promotion\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        (*ri).promoted_slave,
        b"%@\0" as *const u8 as *const libc::c_char,
    );
    (*ri).failover_state = 4 as libc::c_int;
    (*ri).failover_state_change_time = mstime();
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverWaitPromotion(
    mut ri: *mut sentinelRedisInstance,
) {
    if mstime() - (*ri).failover_state_change_time > (*ri).failover_timeout {
        sentinelEvent(
            3 as libc::c_int,
            b"-failover-abort-slave-timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            ri,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        sentinelAbortFailover(ri);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverDetectEnd(
    mut master: *mut sentinelRedisInstance,
) {
    let mut not_reconfigured: libc::c_int = 0 as libc::c_int;
    let mut timeout: libc::c_int = 0 as libc::c_int;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut elapsed: mstime_t = mstime() - (*master).failover_state_change_time;
    if ((*master).promoted_slave).is_null()
        || (*(*master).promoted_slave).flags & (1 as libc::c_int) << 3 as libc::c_int
            != 0
    {
        return;
    }
    di = dictGetIterator((*master).slaves);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut slave: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if (*slave).flags
            & ((1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) != 0
        {
            continue;
        }
        if (*slave).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            continue;
        }
        not_reconfigured += 1;
    }
    dictReleaseIterator(di);
    if elapsed > (*master).failover_timeout {
        not_reconfigured = 0 as libc::c_int;
        timeout = 1 as libc::c_int;
        sentinelEvent(
            3 as libc::c_int,
            b"+failover-end-for-timeout\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            master,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
    }
    if not_reconfigured == 0 as libc::c_int {
        sentinelEvent(
            3 as libc::c_int,
            b"+failover-end\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            master,
            b"%@\0" as *const u8 as *const libc::c_char,
        );
        (*master).failover_state = 6 as libc::c_int;
        (*master).failover_state_change_time = mstime();
    }
    if timeout != 0 {
        let mut di_0: *mut dictIterator = 0 as *mut dictIterator;
        let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
        di_0 = dictGetIterator((*master).slaves);
        loop {
            de_0 = dictNext(di_0);
            if de_0.is_null() {
                break;
            }
            let mut slave_0: *mut sentinelRedisInstance = (*de_0).v.val
                as *mut sentinelRedisInstance;
            let mut retval: libc::c_int = 0;
            if (*slave_0).flags
                & ((1 as libc::c_int) << 7 as libc::c_int
                    | (1 as libc::c_int) << 10 as libc::c_int
                    | (1 as libc::c_int) << 8 as libc::c_int) != 0
            {
                continue;
            }
            if (*(*slave_0).link).disconnected != 0 {
                continue;
            }
            retval = sentinelSendSlaveOf(slave_0, (*(*master).promoted_slave).addr);
            if retval == 0 as libc::c_int {
                sentinelEvent(
                    2 as libc::c_int,
                    b"+slave-reconf-sent-be\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    slave_0,
                    b"%@\0" as *const u8 as *const libc::c_char,
                );
                (*slave_0).flags |= (1 as libc::c_int) << 8 as libc::c_int;
            }
        }
        dictReleaseIterator(di_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverReconfNextSlave(
    mut master: *mut sentinelRedisInstance,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut in_progress: libc::c_int = 0 as libc::c_int;
    di = dictGetIterator((*master).slaves);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut slave: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        if (*slave).flags
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) != 0
        {
            in_progress += 1;
        }
    }
    dictReleaseIterator(di);
    di = dictGetIterator((*master).slaves);
    while in_progress < (*master).parallel_syncs
        && {
            de = dictNext(di);
            !de.is_null()
        }
    {
        let mut slave_0: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        let mut retval: libc::c_int = 0;
        if (*slave_0).flags
            & ((1 as libc::c_int) << 7 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) != 0
        {
            continue;
        }
        if (*slave_0).flags & (1 as libc::c_int) << 8 as libc::c_int != 0
            && mstime() - (*slave_0).slave_reconf_sent_time
                > sentinel_slave_reconf_timeout
        {
            sentinelEvent(
                2 as libc::c_int,
                b"-slave-reconf-sent-timeout\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                slave_0,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            (*slave_0).flags &= !((1 as libc::c_int) << 8 as libc::c_int);
            (*slave_0).flags |= (1 as libc::c_int) << 10 as libc::c_int;
        }
        if (*slave_0).flags
            & ((1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 9 as libc::c_int) != 0
        {
            continue;
        }
        if (*(*slave_0).link).disconnected != 0 {
            continue;
        }
        retval = sentinelSendSlaveOf(slave_0, (*(*master).promoted_slave).addr);
        if retval == 0 as libc::c_int {
            (*slave_0).flags |= (1 as libc::c_int) << 8 as libc::c_int;
            (*slave_0).slave_reconf_sent_time = mstime();
            sentinelEvent(
                2 as libc::c_int,
                b"+slave-reconf-sent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                slave_0,
                b"%@\0" as *const u8 as *const libc::c_char,
            );
            in_progress += 1;
        }
    }
    dictReleaseIterator(di);
    sentinelFailoverDetectEnd(master);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverSwitchToPromotedSlave(
    mut master: *mut sentinelRedisInstance,
) {
    let mut ref_0: *mut sentinelRedisInstance = if !((*master).promoted_slave).is_null()
    {
        (*master).promoted_slave
    } else {
        master
    };
    sentinelEvent(
        3 as libc::c_int,
        b"+switch-master\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        master,
        b"%s %s %d %s %d\0" as *const u8 as *const libc::c_char,
        (*master).name,
        announceSentinelAddr((*master).addr),
        (*(*master).addr).port,
        announceSentinelAddr((*ref_0).addr),
        (*(*ref_0).addr).port,
    );
    sentinelResetMasterAndChangeAddress(
        master,
        (*(*ref_0).addr).hostname,
        (*(*ref_0).addr).port,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sentinelFailoverStateMachine(
    mut ri: *mut sentinelRedisInstance,
) {
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_MASTER\0" as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            5218 as libc::c_int,
        );
        unreachable!();
    };
    if (*ri).flags & (1 as libc::c_int) << 6 as libc::c_int == 0 {
        return;
    }
    match (*ri).failover_state {
        1 => {
            sentinelFailoverWaitStart(ri);
        }
        2 => {
            sentinelFailoverSelectSlave(ri);
        }
        3 => {
            sentinelFailoverSendSlaveOfNoOne(ri);
        }
        4 => {
            sentinelFailoverWaitPromotion(ri);
        }
        5 => {
            sentinelFailoverReconfNextSlave(ri);
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn sentinelAbortFailover(mut ri: *mut sentinelRedisInstance) {
    if (*ri).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {} else {
        _serverAssert(
            b"ri->flags & SRI_FAILOVER_IN_PROGRESS\0" as *const u8
                as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            5247 as libc::c_int,
        );
        unreachable!();
    };
    if (*ri).failover_state <= 4 as libc::c_int {} else {
        _serverAssert(
            b"ri->failover_state <= SENTINEL_FAILOVER_STATE_WAIT_PROMOTION\0"
                as *const u8 as *const libc::c_char,
            b"sentinel.c\0" as *const u8 as *const libc::c_char,
            5248 as libc::c_int,
        );
        unreachable!();
    };
    (*ri).flags
        &= !((1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 11 as libc::c_int);
    (*ri).failover_state = 0 as libc::c_int;
    (*ri).failover_state_change_time = mstime();
    if !((*ri).promoted_slave).is_null() {
        (*(*ri).promoted_slave).flags &= !((1 as libc::c_int) << 7 as libc::c_int);
        (*ri).promoted_slave = 0 as *mut sentinelRedisInstance;
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelHandleRedisInstance(
    mut ri: *mut sentinelRedisInstance,
) {
    sentinelReconnectInstance(ri);
    sentinelSendPeriodicCommands(ri);
    if sentinel.tilt != 0 {
        if mstime() - sentinel.tilt_start_time < sentinel_tilt_period {
            return;
        }
        sentinel.tilt = 0 as libc::c_int;
        sentinelEvent(
            3 as libc::c_int,
            b"-tilt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut sentinelRedisInstance,
            b"#tilt mode exited\0" as *const u8 as *const libc::c_char,
        );
    }
    sentinelCheckSubjectivelyDown(ri);
    (*ri).flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0;
    if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        sentinelCheckObjectivelyDown(ri);
        if sentinelStartFailoverIfNeeded(ri) != 0 {
            sentinelAskMasterStateToOtherSentinels(
                ri,
                (1 as libc::c_int) << 0 as libc::c_int,
            );
        }
        sentinelFailoverStateMachine(ri);
        sentinelAskMasterStateToOtherSentinels(ri, 0 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn sentinelHandleDictOfRedisInstances(mut instances: *mut dict) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut switch_to_promoted: *mut sentinelRedisInstance = 0
        as *mut sentinelRedisInstance;
    di = dictGetIterator(instances);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut ri: *mut sentinelRedisInstance = (*de).v.val
            as *mut sentinelRedisInstance;
        sentinelHandleRedisInstance(ri);
        if (*ri).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            sentinelHandleDictOfRedisInstances((*ri).slaves);
            sentinelHandleDictOfRedisInstances((*ri).sentinels);
            if (*ri).failover_state == 6 as libc::c_int {
                switch_to_promoted = ri;
            }
        }
    }
    if !switch_to_promoted.is_null() {
        sentinelFailoverSwitchToPromotedSlave(switch_to_promoted);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn sentinelCheckTiltCondition() {
    let mut now: mstime_t = mstime();
    let mut delta: mstime_t = now - sentinel.previous_time;
    if delta < 0 as libc::c_int as libc::c_longlong || delta > sentinel_tilt_trigger {
        sentinel.tilt = 1 as libc::c_int;
        sentinel.tilt_start_time = mstime();
        sentinelEvent(
            3 as libc::c_int,
            b"+tilt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as *mut sentinelRedisInstance,
            b"#tilt mode entered\0" as *const u8 as *const libc::c_char,
        );
    }
    sentinel.previous_time = mstime();
}
#[no_mangle]
pub unsafe extern "C" fn sentinelTimer() {
    sentinelCheckTiltCondition();
    sentinelHandleDictOfRedisInstances(sentinel.masters);
    sentinelRunPendingScripts();
    sentinelCollectTerminatedScripts();
    sentinelKillTimedoutScripts();
    server.hz = 10 as libc::c_int + rand() % 10 as libc::c_int;
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use std::cell::UnsafeCell;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    pub type functionsLibCtx;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn sync_file_range(
        __fd: libc::c_int,
        __offset: __off64_t,
        __count: __off64_t,
        __flags: libc::c_uint,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdsclear(s: sds);
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn __errno_location() -> *mut libc::c_int;
    fn connCreateSocket() -> *mut connection;
    fn connCreateTLS() -> *mut connection;
    fn connSetPrivateData(conn: *mut connection, data: *mut libc::c_void);
    fn connGetPrivateData(conn: *mut connection) -> *mut libc::c_void;
    fn connGetState(conn: *mut connection) -> libc::c_int;
    fn connHasWriteHandler(conn: *mut connection) -> libc::c_int;
    fn connBlock(conn: *mut connection) -> libc::c_int;
    fn connNonBlock(conn: *mut connection) -> libc::c_int;
    fn connDisableTcpNoDelay(conn: *mut connection) -> libc::c_int;
    fn connRecvTimeout(conn: *mut connection, ms: libc::c_longlong) -> libc::c_int;
    fn connPeerToString(
        conn: *mut connection,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn connGetInfo(
        conn: *mut connection,
        buf: *mut libc::c_char,
        buf_len: size_t,
    ) -> *const libc::c_char;
    fn rioInitWithConn(r: *mut rio, conn: *mut connection, read_limit: size_t);
    fn rioFreeConn(r: *mut rio, out_remainingBufferedData: *mut sds);
    fn time(__timer: *mut time_t) -> time_t;
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
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
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
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
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
    fn listCreate() -> *mut list;
    fn listEmpty(list: *mut list);
    fn listAddNodeHead(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zmalloc_usable(size: size_t, usable: *mut size_t) -> *mut libc::c_void;
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn anetFormatAddr(
        fmt: *mut libc::c_char,
        fmt_len: size_t,
        ip: *mut libc::c_char,
        port: libc::c_int,
    ) -> libc::c_int;
    fn fsyncFileDir(filename: *const libc::c_char) -> libc::c_int;
    fn raxNew() -> *mut rax;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxRemove(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxFree(rax: *mut rax);
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxPrev(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxEOF(it: *mut raxIterator) -> libc::c_int;
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn intrev64(v: uint64_t) -> uint64_t;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn moduleAllDatatypesHandleErrors() -> libc::c_int;
    fn moduleAllModulesHandleReplAsyncLoad() -> libc::c_int;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn mstime() -> libc::c_longlong;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn redisCommunicateSystemd(sd_notify_msg: *const libc::c_char) -> libc::c_int;
    fn createClient(conn: *mut connection) -> *mut client;
    fn freeClient(c: *mut client);
    fn freeClientAsync(c: *mut client);
    fn resetClient(c: *mut client);
    fn sendReplyToClient(conn: *mut connection);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn readQueryFromClient(conn: *mut connection);
    fn prepareClientToWrite(c: *mut client) -> libc::c_int;
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplySds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn copyReplicaOutputBuffer(dst: *mut client, src: *mut client);
    fn getClientPeerId(client: *mut client) -> *mut libc::c_char;
    fn catClientInfoString(s: sds, client: *mut client) -> sds;
    fn closeClientOnOutputBufferLimitReached(
        c: *mut client,
        async_0: libc::c_int,
    ) -> libc::c_int;
    fn disconnectSlaves();
    fn pauseClients(purpose: pause_purpose, end: mstime_t, type_0: pause_type);
    fn unpauseClients(purpose: pause_purpose);
    fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int;
    fn clientHasPendingReplies(c: *mut client) -> libc::c_int;
    fn unlinkClient(c: *mut client);
    fn linkClient(c: *mut client);
    fn putClientInPendingWriteQueue(c: *mut client);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn rdbLoadRioWithLoadingCtx(
        rdb: *mut rio,
        rdbflags: libc::c_int,
        rsi: *mut rdbSaveInfo,
        rdb_loading_ctx: *mut rdbLoadingCtx,
    ) -> libc::c_int;
    fn rdbLoad(
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
        rdbflags: libc::c_int,
    ) -> libc::c_int;
    fn rdbPopulateSaveInfo(rsi: *mut rdbSaveInfo) -> *mut rdbSaveInfo;
    fn rdbSaveToSlavesSockets(req: libc::c_int, rsi: *mut rdbSaveInfo) -> libc::c_int;
    fn rdbSaveBackground(
        req: libc::c_int,
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn allPersistenceDisabled() -> libc::c_int;
    fn hasActiveChildProcess() -> libc::c_int;
    fn startAppendOnly() -> libc::c_int;
    fn stopLoading(success: libc::c_int);
    fn getLongLongFromObject(o: *mut robj, target: *mut libc::c_longlong) -> libc::c_int;
    fn startLoading(size: size_t, rdbflags: libc::c_int, async_0: libc::c_int);
    fn killRDBChild();
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn stopAppendOnly();
    fn getRangeLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        min: libc::c_long,
        max: libc::c_long,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn setOOMScoreAdj(process_class: libc::c_int) -> libc::c_int;
    fn stringObjectLen(o: *mut robj) -> size_t;
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn decrRefCount(o: *mut robj);
    fn discardTransaction(c: *mut client);
    fn checkChildrenDone();
    fn freeReplicationBacklogRefMemAsync(blocks: *mut list, index: *mut rax);
    fn emptyData(
        dbnum: libc::c_int,
        flags: libc::c_int,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    ) -> libc::c_longlong;
    fn unblockClient(c: *mut client);
    fn disconnectAllBlockedClients();
    fn getTimeoutFromObjectOrReply(
        c: *mut client,
        object: *mut robj,
        timeout: *mut mstime_t,
        unit: libc::c_int,
    ) -> libc::c_int;
    fn blockClient(c: *mut client, btype: libc::c_int);
    fn initTempDb() -> *mut redisDb;
    fn dbTotalServerKeyCount() -> libc::c_longlong;
    fn sentinelRoleCommand(c: *mut client);
    fn discardTempDb(
        tempDb: *mut redisDb,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    );
    fn selectDb(c: *mut client, id: libc::c_int) -> libc::c_int;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn swapMainDbWithTempDb(tempDb: *mut redisDb);
    fn bioCreateCloseJob(fd: libc::c_int, need_fsync: libc::c_int);
    fn functionsLibCtxGetCurrent() -> *mut functionsLibCtx;
    fn functionsLibCtxCreate() -> *mut functionsLibCtx;
    fn functionsLibCtxSwapWithCurrent(lib_ctx: *mut functionsLibCtx);
    fn functionsLibCtxClear(lib_ctx: *mut functionsLibCtx);
    fn functionsLibCtxFree(lib_ctx: *mut functionsLibCtx);
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __lxstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
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
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
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
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
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
    pub v: C2RustUnnamed_4,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raxStack {
    pub stack: *mut *mut libc::c_void,
    pub items: size_t,
    pub maxitems: size_t,
    pub static_items: [*mut libc::c_void; 32],
    pub oom: libc::c_int,
}
pub type raxNodeCallback = Option::<
    unsafe extern "C" fn(*mut *mut raxNode) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct raxIterator {
    pub flags: libc::c_int,
    pub rt: *mut rax,
    pub key: *mut libc::c_uchar,
    pub data: *mut libc::c_void,
    pub key_len: size_t,
    pub key_max: size_t,
    pub key_static_string: [libc::c_uchar; 128],
    pub node: *mut raxNode,
    pub stack: raxStack,
    pub node_cb: raxNodeCallback,
}
pub type C2RustUnnamed_5 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_5 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_5 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_5 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_5 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_5 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_5 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_5 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_5 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_5 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_5 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_5 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_5 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_5 = 0;
pub type C2RustUnnamed_6 = libc::c_uint;
pub const FAILOVER_IN_PROGRESS: C2RustUnnamed_6 = 2;
pub const FAILOVER_WAIT_FOR_SYNC: C2RustUnnamed_6 = 1;
pub const NO_FAILOVER: C2RustUnnamed_6 = 0;
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
pub type pause_purpose = libc::c_uint;
pub const NUM_PAUSE_PURPOSES: pause_purpose = 3;
pub const PAUSE_DURING_FAILOVER: pause_purpose = 2;
pub const PAUSE_DURING_SHUTDOWN: pause_purpose = 1;
pub const PAUSE_BY_CLIENT_COMMAND: pause_purpose = 0;
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
pub struct replBufBlock {
    pub refcount: libc::c_int,
    pub id: libc::c_longlong,
    pub repl_offset: libc::c_longlong,
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterSlotToKeyMapping {
    pub by_slot: [slotToKeys; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slotToKeys {
    pub count: uint64_t,
    pub head: *mut dictEntry,
}
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
pub struct rdbLoadingCtx {
    pub dbarray: *mut redisDb,
    pub functions_lib_ctx: *mut functionsLibCtx,
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
    pub bs: C2RustUnnamed_10,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub range: C2RustUnnamed_9,
    pub keynum: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
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
pub union C2RustUnnamed_10 {
    pub index: C2RustUnnamed_12,
    pub keyword: C2RustUnnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
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
pub struct rdbSaveInfo {
    pub repl_stream_db: libc::c_int,
    pub repl_id_is_set: libc::c_int,
    pub repl_id: [libc::c_char; 41],
    pub repl_offset: libc::c_longlong,
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
    pub inst_metric: [C2RustUnnamed_13; 5],
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
pub struct clusterState {
    pub myself: *mut clusterNode,
    pub currentEpoch: uint64_t,
    pub state: libc::c_int,
    pub size: libc::c_int,
    pub nodes: *mut dict,
    pub nodes_black_list: *mut dict,
    pub migrating_slots_to: [*mut clusterNode; 16384],
    pub importing_slots_from: [*mut clusterNode; 16384],
    pub slots: [*mut clusterNode; 16384],
    pub slots_to_channels: *mut rax,
    pub failover_auth_time: mstime_t,
    pub failover_auth_count: libc::c_int,
    pub failover_auth_sent: libc::c_int,
    pub failover_auth_rank: libc::c_int,
    pub failover_auth_epoch: uint64_t,
    pub cant_failover_reason: libc::c_int,
    pub mf_end: mstime_t,
    pub mf_slave: *mut clusterNode,
    pub mf_master_offset: libc::c_longlong,
    pub mf_can_start: libc::c_int,
    pub lastVoteEpoch: uint64_t,
    pub todo_before_sleep: libc::c_int,
    pub stats_bus_messages_sent: [libc::c_longlong; 11],
    pub stats_bus_messages_received: [libc::c_longlong; 11],
    pub stats_pfail_nodes: libc::c_longlong,
    pub stat_cluster_links_buffer_limit_exceeded: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterNode {
    pub ctime: mstime_t,
    pub name: [libc::c_char; 40],
    pub flags: libc::c_int,
    pub configEpoch: uint64_t,
    pub slots: [libc::c_uchar; 2048],
    pub slot_info_pairs: *mut uint16_t,
    pub slot_info_pairs_count: libc::c_int,
    pub numslots: libc::c_int,
    pub numslaves: libc::c_int,
    pub slaves: *mut *mut clusterNode,
    pub slaveof: *mut clusterNode,
    pub last_in_ping_gossip: libc::c_ulonglong,
    pub ping_sent: mstime_t,
    pub pong_received: mstime_t,
    pub data_received: mstime_t,
    pub fail_time: mstime_t,
    pub voted_time: mstime_t,
    pub repl_offset_time: mstime_t,
    pub orphaned_time: mstime_t,
    pub repl_offset: libc::c_longlong,
    pub ip: [libc::c_char; 46],
    pub hostname: sds,
    pub port: libc::c_int,
    pub pport: libc::c_int,
    pub cport: libc::c_int,
    pub link: *mut clusterLink,
    pub inbound_link: *mut clusterLink,
    pub fail_reports: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterLink {
    pub ctime: mstime_t,
    pub conn: *mut connection,
    pub sndbuf: sds,
    pub rcvbuf: *mut libc::c_char,
    pub rcvbuf_len: size_t,
    pub rcvbuf_alloc: size_t,
    pub node: *mut clusterNode,
    pub inbound: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
pub const _ISprint: C2RustUnnamed_14 = 16384;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_14 = 8;
pub const _ISpunct: C2RustUnnamed_14 = 4;
pub const _IScntrl: C2RustUnnamed_14 = 2;
pub const _ISblank: C2RustUnnamed_14 = 1;
pub const _ISgraph: C2RustUnnamed_14 = 32768;
pub const _ISspace: C2RustUnnamed_14 = 8192;
pub const _ISxdigit: C2RustUnnamed_14 = 4096;
pub const _ISdigit: C2RustUnnamed_14 = 2048;
pub const _ISalpha: C2RustUnnamed_14 = 1024;
pub const _ISlower: C2RustUnnamed_14 = 512;
pub const _ISupper: C2RustUnnamed_14 = 256;
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
unsafe extern "C" fn connConnect(
    mut conn: *mut connection,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut src_addr: *const libc::c_char,
    mut connect_handler: ConnectionCallbackFunc,
) -> libc::c_int {
    return ((*(*conn).type_0).connect)
        .expect(
            "non-null function pointer",
        )(conn, addr, port, src_addr, connect_handler);
}
#[inline]
unsafe extern "C" fn connWrite(
    mut conn: *mut connection,
    mut data: *const libc::c_void,
    mut data_len: size_t,
) -> libc::c_int {
    return ((*(*conn).type_0).write)
        .expect("non-null function pointer")(conn, data, data_len);
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
unsafe extern "C" fn connSetWriteHandler(
    mut conn: *mut connection,
    mut func: ConnectionCallbackFunc,
) -> libc::c_int {
    return ((*(*conn).type_0).set_write_handler)
        .expect("non-null function pointer")(conn, func, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn connSetReadHandler(
    mut conn: *mut connection,
    mut func: ConnectionCallbackFunc,
) -> libc::c_int {
    return ((*(*conn).type_0).set_read_handler)
        .expect("non-null function pointer")(conn, func);
}
#[inline]
unsafe extern "C" fn connClose(mut conn: *mut connection) {
    ((*(*conn).type_0).close).expect("non-null function pointer")(conn);
}
#[inline]
unsafe extern "C" fn connGetLastError(mut conn: *mut connection) -> *const libc::c_char {
    return ((*(*conn).type_0).get_last_error).expect("non-null function pointer")(conn);
}
#[inline]
unsafe extern "C" fn connSyncWrite(
    mut conn: *mut connection,
    mut ptr: *mut libc::c_char,
    mut size: ssize_t,
    mut timeout: libc::c_longlong,
) -> ssize_t {
    return ((*(*conn).type_0).sync_write)
        .expect("non-null function pointer")(conn, ptr, size, timeout);
}
#[inline]
unsafe extern "C" fn connSyncReadLine(
    mut conn: *mut connection,
    mut ptr: *mut libc::c_char,
    mut size: ssize_t,
    mut timeout: libc::c_longlong,
) -> ssize_t {
    return ((*(*conn).type_0).sync_readline)
        .expect("non-null function pointer")(conn, ptr, size, timeout);
}
#[inline]
unsafe extern "C" fn rioRead(
    mut r: *mut rio,
    mut buf: *mut libc::c_void,
    mut len: size_t,
) -> size_t {
    if (*r).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
        return 0 as libc::c_int as size_t;
    }
    while len != 0 {
        let mut bytes_to_read: size_t = if (*r).max_processing_chunk != 0
            && (*r).max_processing_chunk < len
        {
            (*r).max_processing_chunk
        } else {
            len
        };
        if ((*r).read).expect("non-null function pointer")(r, buf, bytes_to_read)
            == 0 as libc::c_int as libc::c_ulong
        {
            (*r).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
            return 0 as libc::c_int as size_t;
        }
        if ((*r).update_cksum).is_some() {
            ((*r).update_cksum)
                .expect("non-null function pointer")(r, buf, bytes_to_read);
        }
        buf = (buf as *mut libc::c_char).offset(bytes_to_read as isize)
            as *mut libc::c_void;
        len = (len as libc::c_ulong).wrapping_sub(bytes_to_read) as size_t as size_t;
        (*r)
            .processed_bytes = ((*r).processed_bytes as libc::c_ulong)
            .wrapping_add(bytes_to_read) as size_t as size_t;
    }
    return 1 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn lstat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __lxstat(0 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut RDBGeneratedByReplication: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn replicationGetSlaveName(
    mut c: *mut client,
) -> *mut libc::c_char {
    static mut buf: [libc::c_char; 288] = [0; 288];
    let mut ip: [libc::c_char; 46] = [0; 46];
    ip[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    if !((*c).slave_addr).is_null()
        || connPeerToString(
            (*c).conn,
            ip.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
            0 as *mut libc::c_int,
        ) != -(1 as libc::c_int)
    {
        let mut addr: *mut libc::c_char = if !((*c).slave_addr).is_null() {
            (*c).slave_addr
        } else {
            ip.as_mut_ptr()
        };
        if (*c).slave_listening_port != 0 {
            anetFormatAddr(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong,
                addr,
                (*c).slave_listening_port,
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong,
                b"%s:<unknown-replica-port>\0" as *const u8 as *const libc::c_char,
                addr,
            );
        }
    } else {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 288]>() as libc::c_ulong,
            b"client id #%llu\0" as *const u8 as *const libc::c_char,
            (*c).id as libc::c_ulonglong,
        );
    }
    return buf.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn bg_unlink(mut filename: *const libc::c_char) -> libc::c_int {
    let mut fd: libc::c_int = open(filename, 0 as libc::c_int | 0o4000 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        return unlink(filename)
    } else {
        let mut retval: libc::c_int = unlink(filename);
        if retval == -(1 as libc::c_int) {
            let mut old_errno: libc::c_int = *__errno_location();
            close(fd);
            *__errno_location() = old_errno;
            return -(1 as libc::c_int);
        }
        bioCreateCloseJob(fd, 0 as libc::c_int);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn createReplicationBacklog() {
    if (server.repl_backlog).is_null() {} else {
        _serverAssert(
            b"server.repl_backlog == NULL\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
        );
        unreachable!();
    };
    server
        .repl_backlog = zmalloc(core::mem::size_of::<replBacklog>() as libc::c_ulong)
        as *mut replBacklog;
    (*server.repl_backlog).ref_repl_buf_node = 0 as *mut listNode;
    (*server.repl_backlog).unindexed_count = 0 as libc::c_int as size_t;
    (*server.repl_backlog).blocks_index = raxNew();
    (*server.repl_backlog).histlen = 0 as libc::c_int as libc::c_longlong;
    (*server.repl_backlog)
        .offset = server.master_repl_offset + 1 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn resizeReplicationBacklog() {
    if server.repl_backlog_size
        < (1024 as libc::c_int * 16 as libc::c_int) as libc::c_longlong
    {
        server
            .repl_backlog_size = (1024 as libc::c_int * 16 as libc::c_int)
            as libc::c_longlong;
    }
    if !(server.repl_backlog).is_null() {
        incrementalTrimReplicationBacklog(64 as libc::c_int as size_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeReplicationBacklog() {
    if (*server.slaves).len == 0 as libc::c_int as libc::c_ulong {} else {
        _serverAssert(
            b"listLength(server.slaves) == 0\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
        );
        unreachable!();
    };
    if (server.repl_backlog).is_null() {
        return;
    }
    if !((*server.repl_backlog).ref_repl_buf_node).is_null() {
        let mut o: *mut replBufBlock = (*(*server.repl_backlog).ref_repl_buf_node).value
            as *mut replBufBlock;
        if (*o).refcount == 1 as libc::c_int {} else {
            _serverAssert(
                b"o->refcount == 1\0" as *const u8 as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                146 as libc::c_int,
            );
            unreachable!();
        };
        (*o).refcount -= 1;
    }
    freeReplicationBacklogRefMemAsync(
        server.repl_buffer_blocks,
        (*server.repl_backlog).blocks_index,
    );
    resetReplicationBuffer();
    zfree(server.repl_backlog as *mut libc::c_void);
    server.repl_backlog = 0 as *mut replBacklog;
}
#[no_mangle]
pub unsafe extern "C" fn createReplicationBacklogIndex(mut ln: *mut listNode) {
    (*server.repl_backlog)
        .unindexed_count = ((*server.repl_backlog).unindexed_count).wrapping_add(1);
    if (*server.repl_backlog).unindexed_count >= 64 as libc::c_int as libc::c_ulong {
        let mut o: *mut replBufBlock = (*ln).value as *mut replBufBlock;
        let mut encoded_offset: uint64_t = intrev64((*o).repl_offset as uint64_t);
        raxInsert(
            (*server.repl_backlog).blocks_index,
            &mut encoded_offset as *mut uint64_t as *mut libc::c_uchar,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
            ln as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
        (*server.repl_backlog).unindexed_count = 0 as libc::c_int as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rebaseReplicationBuffer(
    mut base_repl_offset: libc::c_longlong,
) {
    raxFree((*server.repl_backlog).blocks_index);
    (*server.repl_backlog).blocks_index = raxNew();
    (*server.repl_backlog).unindexed_count = 0 as libc::c_int as size_t;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.repl_buffer_blocks, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut o: *mut replBufBlock = (*ln).value as *mut replBufBlock;
        (*o).repl_offset += base_repl_offset;
        createReplicationBacklogIndex(ln);
    };
}
#[no_mangle]
pub unsafe extern "C" fn resetReplicationBuffer() {
    server.repl_buffer_mem = 0 as libc::c_int as size_t;
    server.repl_buffer_blocks = listCreate();
    (*server.repl_buffer_blocks)
        .free = core::mem::transmute::<
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    >(Some(zfree as unsafe extern "C" fn(*mut libc::c_void) -> ()));
}
#[no_mangle]
pub unsafe extern "C" fn canFeedReplicaReplBuffer(
    mut replica: *mut client,
) -> libc::c_int {
    if (*replica).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 42 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    if (*replica).replstate == 6 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn prepareReplicasToWrite() -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut prepared: libc::c_int = 0 as libc::c_int;
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        if canFeedReplicaReplBuffer(slave) == 0 {
            continue;
        }
        if prepareClientToWrite(slave) == -(1 as libc::c_int) {
            continue;
        }
        prepared += 1;
    }
    return prepared;
}
#[no_mangle]
pub unsafe extern "C" fn feedReplicationBufferWithObject(mut o: *mut robj) {
    let mut llstr: [libc::c_char; 21] = [0; 21];
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut len: size_t = 0;
    if (*o).encoding() as libc::c_int == 1 as libc::c_int {
        len = ll2string(
            llstr.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
            (*o).ptr as libc::c_long as libc::c_longlong,
        ) as size_t;
        p = llstr.as_mut_ptr() as *mut libc::c_void;
    } else {
        len = sdslen((*o).ptr as sds);
        p = (*o).ptr;
    }
    feedReplicationBuffer(p as *mut libc::c_char, len);
}
#[no_mangle]
pub unsafe extern "C" fn incrementalTrimReplicationBacklog(mut max_blocks: size_t) {
    if !(server.repl_backlog).is_null() {} else {
        _serverAssert(
            b"server.repl_backlog != NULL\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            251 as libc::c_int,
        );
        unreachable!();
    };
    let mut trimmed_blocks: size_t = 0 as libc::c_int as size_t;
    while (*server.repl_backlog).histlen > server.repl_backlog_size
        && trimmed_blocks < max_blocks
    {
        if (*server.repl_buffer_blocks).len <= 1 as libc::c_int as libc::c_ulong {
            break;
        }
        let mut first: *mut listNode = (*server.repl_buffer_blocks).head;
        if first == (*server.repl_backlog).ref_repl_buf_node {} else {
            _serverAssert(
                b"first == server.repl_backlog->ref_repl_buf_node\0" as *const u8
                    as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                267 as libc::c_int,
            );
            unreachable!();
        };
        let mut fo: *mut replBufBlock = (*first).value as *mut replBufBlock;
        if (*fo).refcount != 1 as libc::c_int {
            break;
        }
        if (*server.repl_backlog).histlen - (*fo).size as libc::c_longlong
            <= server.repl_backlog_size
        {
            break;
        }
        (*fo).refcount -= 1;
        trimmed_blocks = trimmed_blocks.wrapping_add(1);
        (*server.repl_backlog)
            .histlen = ((*server.repl_backlog).histlen as libc::c_ulonglong)
            .wrapping_sub((*fo).size as libc::c_ulonglong) as libc::c_longlong
            as libc::c_longlong;
        let mut next: *mut listNode = (*first).next;
        (*server.repl_backlog).ref_repl_buf_node = next;
        if !((*server.repl_backlog).ref_repl_buf_node).is_null() {} else {
            _serverAssert(
                b"server.repl_backlog->ref_repl_buf_node != NULL\0" as *const u8
                    as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                284 as libc::c_int,
            );
            unreachable!();
        };
        let ref mut fresh0 = (*((*next).value as *mut replBufBlock)).refcount;
        *fresh0 += 1;
        let mut encoded_offset: uint64_t = intrev64((*fo).repl_offset as uint64_t);
        raxRemove(
            (*server.repl_backlog).blocks_index,
            &mut encoded_offset as *mut uint64_t as *mut libc::c_uchar,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
            0 as *mut *mut libc::c_void,
        );
        if (*fo).refcount == 0 as libc::c_int && (*fo).used == (*fo).size {} else {
            _serverAssert(
                b"fo->refcount == 0 && fo->used == fo->size\0" as *const u8
                    as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                294 as libc::c_int,
            );
            unreachable!();
        };
        server
            .repl_buffer_mem = (server.repl_buffer_mem as libc::c_ulong)
            .wrapping_sub(
                ((*fo).size)
                    .wrapping_add(core::mem::size_of::<listNode>() as libc::c_ulong)
                    .wrapping_add(
                        core::mem::size_of::<replBufBlock>() as libc::c_ulong,
                    ),
            ) as size_t as size_t;
        listDelNode(server.repl_buffer_blocks, first);
    }
    (*server.repl_backlog)
        .offset = server.master_repl_offset - (*server.repl_backlog).histlen
        + 1 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn freeReplicaReferencedReplBuffer(mut replica: *mut client) {
    if !((*replica).ref_repl_buf_node).is_null() {
        let mut o: *mut replBufBlock = (*(*replica).ref_repl_buf_node).value
            as *mut replBufBlock;
        if (*o).refcount > 0 as libc::c_int {} else {
            _serverAssert(
                b"o->refcount > 0\0" as *const u8 as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                310 as libc::c_int,
            );
            unreachable!();
        };
        (*o).refcount -= 1;
        incrementalTrimReplicationBacklog(64 as libc::c_int as size_t);
    }
    (*replica).ref_repl_buf_node = 0 as *mut listNode;
    (*replica).ref_block_pos = 0 as libc::c_int as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn feedReplicationBuffer(
    mut s: *mut libc::c_char,
    mut len: size_t,
) {
    static mut repl_block_id: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if (server.repl_backlog).is_null() {
        return;
    }
    server
        .master_repl_offset = (server.master_repl_offset as libc::c_ulonglong)
        .wrapping_add(len as libc::c_ulonglong) as libc::c_longlong as libc::c_longlong;
    (*server.repl_backlog)
        .histlen = ((*server.repl_backlog).histlen as libc::c_ulonglong)
        .wrapping_add(len as libc::c_ulonglong) as libc::c_longlong as libc::c_longlong;
    let mut start_pos: size_t = 0 as libc::c_int as size_t;
    let mut start_node: *mut listNode = 0 as *mut listNode;
    let mut add_new_block: libc::c_int = 0 as libc::c_int;
    let mut ln: *mut listNode = (*server.repl_buffer_blocks).tail;
    let mut tail: *mut replBufBlock = (if !ln.is_null() {
        (*ln).value
    } else {
        0 as *mut libc::c_void
    }) as *mut replBufBlock;
    if !tail.is_null() && (*tail).size > (*tail).used {
        start_node = (*server.repl_buffer_blocks).tail;
        start_pos = (*tail).used;
        let mut avail: size_t = ((*tail).size).wrapping_sub((*tail).used);
        let mut copy: size_t = if avail >= len { len } else { avail };
        memcpy(
            ((*tail).buf).as_mut_ptr().offset((*tail).used as isize)
                as *mut libc::c_void,
            s as *const libc::c_void,
            copy,
        );
        (*tail)
            .used = ((*tail).used as libc::c_ulong).wrapping_add(copy) as size_t
            as size_t;
        s = s.offset(copy as isize);
        len = (len as libc::c_ulong).wrapping_sub(copy) as size_t as size_t;
    }
    if len != 0 {
        let mut usable_size: size_t = 0;
        let mut size: size_t = if len
            < (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        {
            (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        } else {
            len
        };
        tail = zmalloc_usable(
            size.wrapping_add(core::mem::size_of::<replBufBlock>() as libc::c_ulong),
            &mut usable_size,
        ) as *mut replBufBlock;
        (*tail)
            .size = usable_size
            .wrapping_sub(core::mem::size_of::<replBufBlock>() as libc::c_ulong);
        (*tail).used = len;
        (*tail).refcount = 0 as libc::c_int;
        (*tail)
            .repl_offset = (server.master_repl_offset as libc::c_ulonglong)
            .wrapping_sub((*tail).used as libc::c_ulonglong)
            .wrapping_add(1 as libc::c_int as libc::c_ulonglong) as libc::c_longlong;
        let fresh1 = repl_block_id;
        repl_block_id = repl_block_id + 1;
        (*tail).id = fresh1;
        memcpy(
            ((*tail).buf).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            len,
        );
        listAddNodeTail(server.repl_buffer_blocks, tail as *mut libc::c_void);
        server
            .repl_buffer_mem = (server.repl_buffer_mem as libc::c_ulong)
            .wrapping_add(
                usable_size
                    .wrapping_add(core::mem::size_of::<listNode>() as libc::c_ulong),
            ) as size_t as size_t;
        add_new_block = 1 as libc::c_int;
        if start_node.is_null() {
            start_node = (*server.repl_buffer_blocks).tail;
            start_pos = 0 as libc::c_int as size_t;
        }
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        if canFeedReplicaReplBuffer(slave) == 0 {
            continue;
        }
        if ((*slave).ref_repl_buf_node).is_null() {
            (*slave).ref_repl_buf_node = start_node;
            (*slave).ref_block_pos = start_pos;
            let ref mut fresh2 = (*((*start_node).value as *mut replBufBlock)).refcount;
            *fresh2 += 1;
        }
        if add_new_block != 0 {
            closeClientOnOutputBufferLimitReached(slave, 1 as libc::c_int);
        }
    }
    if ((*server.repl_backlog).ref_repl_buf_node).is_null() {
        (*server.repl_backlog).ref_repl_buf_node = start_node;
        let ref mut fresh3 = (*((*start_node).value as *mut replBufBlock)).refcount;
        *fresh3 += 1;
        if add_new_block == 1 as libc::c_int
            && start_pos == 0 as libc::c_int as libc::c_ulong
        {} else {
            _serverAssert(
                b"add_new_block == 1 && start_pos == 0\0" as *const u8
                    as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                399 as libc::c_int,
            );
            unreachable!();
        };
    }
    if add_new_block != 0 {
        createReplicationBacklogIndex((*server.repl_buffer_blocks).tail);
    }
    incrementalTrimReplicationBacklog(64 as libc::c_int as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn replicationFeedSlaves(
    mut slaves: *mut list,
    mut dictid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut llstr: [libc::c_char; 21] = [0; 21];
    if dictid == -(1 as libc::c_int)
        || dictid >= 0 as libc::c_int && dictid < server.dbnum
    {} else {
        _serverAssert(
            b"dictid == -1 || (dictid >= 0 && dictid < server.dbnum)\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
        );
        unreachable!();
    };
    if !(server.masterhost).is_null() {
        return;
    }
    if (server.repl_backlog).is_null()
        && (*slaves).len == 0 as libc::c_int as libc::c_ulong
    {
        return;
    }
    if !((*slaves).len != 0 as libc::c_int as libc::c_ulong
        && (server.repl_backlog).is_null())
    {} else {
        _serverAssert(
            b"!(listLength(slaves) != 0 && server.repl_backlog == NULL)\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            438 as libc::c_int,
        );
        unreachable!();
    };
    prepareReplicasToWrite();
    if server.slaveseldb != dictid {
        let mut selectcmd: *mut robj = 0 as *mut robj;
        if dictid >= 0 as libc::c_int && dictid < 10 as libc::c_int {
            selectcmd = shared.select[dictid as usize];
        } else {
            let mut dictid_len: libc::c_int = 0;
            dictid_len = ll2string(
                llstr.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
                dictid as libc::c_longlong,
            );
            selectcmd = createObject(
                0 as libc::c_int,
                sdscatprintf(
                    sdsempty(),
                    b"*2\r\n$6\r\nSELECT\r\n$%d\r\n%s\r\n\0" as *const u8
                        as *const libc::c_char,
                    dictid_len,
                    llstr.as_mut_ptr(),
                ) as *mut libc::c_void,
            );
        }
        feedReplicationBufferWithObject(selectcmd);
        if dictid < 0 as libc::c_int || dictid >= 10 as libc::c_int {
            decrRefCount(selectcmd);
        }
        server.slaveseldb = dictid;
    }
    let mut aux: [libc::c_char; 24] = [0; 24];
    aux[0 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    len = ll2string(
        aux.as_mut_ptr().offset(1 as libc::c_int as isize),
        (core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        argc as libc::c_longlong,
    );
    aux[(len + 1 as libc::c_int) as usize] = '\r' as i32 as libc::c_char;
    aux[(len + 2 as libc::c_int) as usize] = '\n' as i32 as libc::c_char;
    feedReplicationBuffer(aux.as_mut_ptr(), (len + 3 as libc::c_int) as size_t);
    j = 0 as libc::c_int;
    while j < argc {
        let mut objlen: libc::c_long = stringObjectLen(*argv.offset(j as isize))
            as libc::c_long;
        aux[0 as libc::c_int as usize] = '$' as i32 as libc::c_char;
        len = ll2string(
            aux.as_mut_ptr().offset(1 as libc::c_int as isize),
            (core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            objlen as libc::c_longlong,
        );
        aux[(len + 1 as libc::c_int) as usize] = '\r' as i32 as libc::c_char;
        aux[(len + 2 as libc::c_int) as usize] = '\n' as i32 as libc::c_char;
        feedReplicationBuffer(aux.as_mut_ptr(), (len + 3 as libc::c_int) as size_t);
        feedReplicationBufferWithObject(*argv.offset(j as isize));
        feedReplicationBuffer(
            aux.as_mut_ptr().offset(len as isize).offset(1 as libc::c_int as isize),
            2 as libc::c_int as size_t,
        );
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn showLatestBacklog() {
    if (server.repl_backlog).is_null() {
        return;
    }
    if (*server.repl_buffer_blocks).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut dumplen: size_t = 256 as libc::c_int as size_t;
    if (*server.repl_backlog).histlen < dumplen as libc::c_longlong {
        dumplen = (*server.repl_backlog).histlen as size_t;
    }
    let mut dump: sds = sdsempty();
    let mut node: *mut listNode = (*server.repl_buffer_blocks).tail;
    while dumplen != 0 {
        if node.is_null() {
            break;
        }
        let mut o: *mut replBufBlock = (*node).value as *mut replBufBlock;
        let mut thislen: size_t = if (*o).used >= dumplen { dumplen } else { (*o).used };
        let mut head: sds = sdscatrepr(
            sdsempty(),
            ((*o).buf)
                .as_mut_ptr()
                .offset((*o).used as isize)
                .offset(-(thislen as isize)),
            thislen,
        );
        let mut tmp: sds = sdscatsds(head, dump);
        sdsfree(dump);
        dump = tmp;
        dumplen = (dumplen as libc::c_ulong).wrapping_sub(thislen) as size_t as size_t;
        node = (*node).prev;
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Latest backlog is: '%s'\0" as *const u8 as *const libc::c_char,
            dump,
        );
    }
    sdsfree(dump);
}
#[no_mangle]
pub unsafe extern "C" fn replicationFeedStreamFromMasterStream(
    mut buf: *mut libc::c_char,
    mut buflen: size_t,
) {
    if (*server.slaves).len != 0 {
        if !(server.repl_backlog).is_null() {} else {
            _serverAssert(
                b"server.repl_backlog != NULL\0" as *const u8 as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                542 as libc::c_int,
            );
            unreachable!();
        };
    }
    if !(server.repl_backlog).is_null() {
        prepareReplicasToWrite();
        feedReplicationBuffer(buf, buflen);
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicationFeedMonitors(
    mut c: *mut client,
    mut monitors: *mut list,
    mut dictid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) {
    if monitors.is_null() || (*monitors).len == 0 as libc::c_int as libc::c_ulong
        || server.loading != 0
    {
        return;
    }
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut j: libc::c_int = 0;
    let mut cmdrepr: sds = sdsnew(b"+\0" as *const u8 as *const libc::c_char);
    let mut cmdobj: *mut robj = 0 as *mut robj;
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    cmdrepr = sdscatprintf(
        cmdrepr,
        b"%ld.%06ld \0" as *const u8 as *const libc::c_char,
        tv.tv_sec,
        tv.tv_usec,
    );
    if (*c).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0 {
        cmdrepr = sdscatprintf(
            cmdrepr,
            b"[%d lua] \0" as *const u8 as *const libc::c_char,
            dictid,
        );
    } else if (*c).flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong
        != 0
    {
        cmdrepr = sdscatprintf(
            cmdrepr,
            b"[%d unix:%s] \0" as *const u8 as *const libc::c_char,
            dictid,
            server.unixsocket,
        );
    } else {
        cmdrepr = sdscatprintf(
            cmdrepr,
            b"[%d %s] \0" as *const u8 as *const libc::c_char,
            dictid,
            getClientPeerId(c),
        );
    }
    j = 0 as libc::c_int;
    while j < argc {
        if (**argv.offset(j as isize)).encoding() as libc::c_int == 1 as libc::c_int {
            cmdrepr = sdscatprintf(
                cmdrepr,
                b"\"%ld\"\0" as *const u8 as *const libc::c_char,
                (**argv.offset(j as isize)).ptr as libc::c_long,
            );
        } else {
            cmdrepr = sdscatrepr(
                cmdrepr,
                (**argv.offset(j as isize)).ptr as *mut libc::c_char,
                sdslen((**argv.offset(j as isize)).ptr as sds),
            );
        }
        if j != argc - 1 as libc::c_int {
            cmdrepr = sdscatlen(
                cmdrepr,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    cmdrepr = sdscatlen(
        cmdrepr,
        b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    cmdobj = createObject(0 as libc::c_int, cmdrepr as *mut libc::c_void);
    listRewind(monitors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut monitor: *mut client = (*ln).value as *mut client;
        addReply(monitor, cmdobj);
    }
    decrRefCount(cmdobj);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyReplicationBacklog(
    mut c: *mut client,
    mut offset: libc::c_longlong,
) -> libc::c_longlong {
    let mut skip: libc::c_longlong = 0;
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"[PSYNC] Replica request offset: %lld\0" as *const u8
                as *const libc::c_char,
            offset,
        );
    }
    if (*server.repl_backlog).histlen == 0 as libc::c_int as libc::c_longlong {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"[PSYNC] Backlog history len is zero\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int as libc::c_longlong;
    }
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"[PSYNC] Backlog size: %lld\0" as *const u8 as *const libc::c_char,
            server.repl_backlog_size,
        );
    }
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"[PSYNC] First byte: %lld\0" as *const u8 as *const libc::c_char,
            (*server.repl_backlog).offset,
        );
    }
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"[PSYNC] History len: %lld\0" as *const u8 as *const libc::c_char,
            (*server.repl_backlog).histlen,
        );
    }
    skip = offset - (*server.repl_backlog).offset;
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"[PSYNC] Skipping: %lld\0" as *const u8 as *const libc::c_char,
            skip,
        );
    }
    let mut node: *mut listNode = 0 as *mut listNode;
    if raxSize((*server.repl_backlog).blocks_index) > 0 as libc::c_int as libc::c_ulong {
        let mut encoded_offset: uint64_t = intrev64(offset as uint64_t);
        let mut ri: raxIterator = raxIterator {
            flags: 0,
            rt: 0 as *mut rax,
            key: 0 as *mut libc::c_uchar,
            data: 0 as *mut libc::c_void,
            key_len: 0,
            key_max: 0,
            key_static_string: [0; 128],
            node: 0 as *mut raxNode,
            stack: raxStack {
                stack: 0 as *mut *mut libc::c_void,
                items: 0,
                maxitems: 0,
                static_items: [0 as *mut libc::c_void; 32],
                oom: 0,
            },
            node_cb: None,
        };
        raxStart(&mut ri, (*server.repl_backlog).blocks_index);
        raxSeek(
            &mut ri,
            b">\0" as *const u8 as *const libc::c_char,
            &mut encoded_offset as *mut uint64_t as *mut libc::c_uchar,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        );
        if raxEOF(&mut ri) != 0 {
            raxSeek(
                &mut ri,
                b"$\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            raxPrev(&mut ri);
            node = ri.data as *mut listNode;
        } else {
            raxPrev(&mut ri);
            if raxPrev(&mut ri) != 0 {
                node = ri.data as *mut listNode;
            } else {
                node = (*server.repl_backlog).ref_repl_buf_node;
            }
        }
        raxStop(&mut ri);
    } else {
        node = (*server.repl_backlog).ref_repl_buf_node;
    }
    while !node.is_null() {
        let mut o: *mut replBufBlock = (*node).value as *mut replBufBlock;
        if (*o).repl_offset + (*o).used as libc::c_longlong >= offset {
            break;
        }
        node = (*node).next;
    }
    if !node.is_null() {} else {
        _serverAssert(
            b"node != NULL\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            648 as libc::c_int,
        );
        unreachable!();
    };
    prepareClientToWrite(c);
    let mut o_0: *mut replBufBlock = (*node).value as *mut replBufBlock;
    (*o_0).refcount += 1;
    (*c).ref_repl_buf_node = node;
    (*c).ref_block_pos = (offset - (*o_0).repl_offset) as size_t;
    return (*server.repl_backlog).histlen - skip;
}
#[no_mangle]
pub unsafe extern "C" fn getPsyncInitialOffset() -> libc::c_longlong {
    return server.master_repl_offset;
}
#[no_mangle]
pub unsafe extern "C" fn replicationSetupSlaveForFullResync(
    mut slave: *mut client,
    mut offset: libc::c_longlong,
) -> libc::c_int {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut buflen: libc::c_int = 0;
    (*slave).psync_initial_offset = offset;
    (*slave).replstate = 7 as libc::c_int;
    server.slaveseldb = -(1 as libc::c_int);
    if (*slave).flags & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong == 0 {
        buflen = snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"+FULLRESYNC %s %lld\r\n\0" as *const u8 as *const libc::c_char,
            (server.replid).as_mut_ptr(),
            offset,
        );
        if connWrite(
            (*slave).conn,
            buf.as_mut_ptr() as *const libc::c_void,
            buflen as size_t,
        ) != buflen
        {
            freeClientAsync(slave);
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn masterTryPartialResynchronization(
    mut c: *mut client,
    mut psync_offset: libc::c_longlong,
) -> libc::c_int {
    let mut psync_len: libc::c_longlong = 0;
    let mut master_replid: *mut libc::c_char = (**((*c).argv)
        .offset(1 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut buflen: libc::c_int = 0;
    if strcasecmp(master_replid, (server.replid).as_mut_ptr()) != 0
        && (strcasecmp(master_replid, (server.replid2).as_mut_ptr()) != 0
            || psync_offset > server.second_replid_offset)
    {
        if *master_replid.offset(0 as libc::c_int as isize) as libc::c_int != '?' as i32
        {
            if strcasecmp(master_replid, (server.replid).as_mut_ptr()) != 0
                && strcasecmp(master_replid, (server.replid2).as_mut_ptr()) != 0
            {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Partial resynchronization not accepted: Replication ID mismatch (Replica asked for '%s', my replication IDs are '%s' and '%s')\0"
                            as *const u8 as *const libc::c_char,
                        master_replid,
                        (server.replid).as_mut_ptr(),
                        (server.replid2).as_mut_ptr(),
                    );
                }
            } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Partial resynchronization not accepted: Requested offset for second ID was %lld, but I can reply up to %lld\0"
                        as *const u8 as *const libc::c_char,
                    psync_offset,
                    server.second_replid_offset,
                );
            }
        } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Full resync requested by replica %s\0" as *const u8
                    as *const libc::c_char,
                replicationGetSlaveName(c),
            );
        }
    } else if (server.repl_backlog).is_null()
        || psync_offset < (*server.repl_backlog).offset
        || psync_offset > (*server.repl_backlog).offset + (*server.repl_backlog).histlen
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Unable to partial resync with replica %s for lack of backlog (Replica request was: %lld).\0"
                    as *const u8 as *const libc::c_char,
                replicationGetSlaveName(c),
                psync_offset,
            );
        }
        if psync_offset > server.master_repl_offset {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Warning: replica %s tried to PSYNC with an offset that is greater than the master replication offset.\0"
                        as *const u8 as *const libc::c_char,
                    replicationGetSlaveName(c),
                );
            }
        }
    } else {
        (*c).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
        (*c).replstate = 9 as libc::c_int;
        (*c).repl_ack_time = server.unixtime as libc::c_longlong;
        (*c).repl_start_cmd_stream_on_ack = 0 as libc::c_int;
        listAddNodeTail(server.slaves, c as *mut libc::c_void);
        if (*c).slave_capa & (1 as libc::c_int) << 1 as libc::c_int != 0 {
            buflen = snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"+CONTINUE %s\r\n\0" as *const u8 as *const libc::c_char,
                (server.replid).as_mut_ptr(),
            );
        } else {
            buflen = snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"+CONTINUE\r\n\0" as *const u8 as *const libc::c_char,
            );
        }
        if connWrite(
            (*c).conn,
            buf.as_mut_ptr() as *const libc::c_void,
            buflen as size_t,
        ) != buflen
        {
            freeClientAsync(c);
            return 0 as libc::c_int;
        }
        psync_len = addReplyReplicationBacklog(c, psync_offset);
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Partial resynchronization request from %s accepted. Sending %lld bytes of backlog starting from offset %lld.\0"
                    as *const u8 as *const libc::c_char,
                replicationGetSlaveName(c),
                psync_len,
                psync_offset,
            );
        }
        refreshGoodSlavesCount();
        moduleFireServerEvent(
            6 as libc::c_int as uint64_t,
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn startBgsaveForReplication(
    mut mincapa: libc::c_int,
    mut req: libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut socket_target: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    socket_target = ((server.repl_diskless_sync != 0
        || req
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0)
        && mincapa & (1 as libc::c_int) << 0 as libc::c_int != 0) as libc::c_int;
    if socket_target != 0
        || req
            & ((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) == 0
    {} else {
        _serverAssert(
            b"socket_target || !(req & SLAVE_REQ_RDB_MASK)\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            841 as libc::c_int,
        );
        unreachable!();
    };
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Starting BGSAVE for SYNC with target: %s\0" as *const u8
                as *const libc::c_char,
            if socket_target != 0 {
                b"replicas sockets\0" as *const u8 as *const libc::c_char
            } else {
                b"disk\0" as *const u8 as *const libc::c_char
            },
        );
    }
    let mut rsi: rdbSaveInfo = rdbSaveInfo {
        repl_stream_db: 0,
        repl_id_is_set: 0,
        repl_id: [0; 41],
        repl_offset: 0,
    };
    let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
    rsiptr = rdbPopulateSaveInfo(&mut rsi);
    if !rsiptr.is_null() {
        if socket_target != 0 {
            retval = rdbSaveToSlavesSockets(req, rsiptr);
        } else {
            retval = rdbSaveBackground(req, server.rdb_filename, rsiptr);
        }
    } else {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"BGSAVE for replication: replication information not available, can't generate the RDB file right now. Try later.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        retval = -(1 as libc::c_int);
    }
    if retval == 0 as libc::c_int && socket_target == 0 && server.rdb_del_sync_files != 0
    {
        RDBGeneratedByReplication = 1 as libc::c_int;
    }
    if retval == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"BGSAVE for replication failed\0" as *const u8 as *const libc::c_char,
            );
        }
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut slave: *mut client = (*ln).value as *mut client;
            if (*slave).replstate == 6 as libc::c_int {
                (*slave).replstate = REPL_STATE_NONE as libc::c_int;
                (*slave).flags
                    &= !((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
                listDelNode(server.slaves, ln);
                addReplyError(
                    slave,
                    b"BGSAVE failed, replication can't continue\0" as *const u8
                        as *const libc::c_char,
                );
                (*slave).flags
                    |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
            }
        }
        return retval;
    }
    if socket_target == 0 {
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut slave_0: *mut client = (*ln).value as *mut client;
            if !((*slave_0).replstate == 6 as libc::c_int) {
                continue;
            }
            if (*slave_0).slave_req != req {
                continue;
            }
            replicationSetupSlaveForFullResync(slave_0, getPsyncInitialOffset());
        }
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn syncCommand(mut c: *mut client) {
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    if (*c).argc > 3 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"psync\0" as *const u8 as *const libc::c_char,
        ) == 0
        && strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"failover\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failover request received for replid %s.\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *mut libc::c_uchar,
            );
        }
        if (server.masterhost).is_null() {
            addReplyError(
                c,
                b"PSYNC FAILOVER can't be sent to a master.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            (server.replid).as_mut_ptr(),
        ) == 0
        {
            replicationUnsetMaster();
            let mut client: sds = catClientInfoString(sdsempty(), c);
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"MASTER MODE enabled (failover request from '%s')\0" as *const u8
                        as *const libc::c_char,
                    client,
                );
            }
            sdsfree(client);
        } else {
            addReplyError(
                c,
                b"PSYNC FAILOVER replid must match my replid.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    if server.failover_state != NO_FAILOVER as libc::c_int {
        addReplyError(
            c,
            b"-NOMASTERLINK Can't SYNC while failing over\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !(server.masterhost).is_null()
        && server.repl_state != REPL_STATE_CONNECTED as libc::c_int
    {
        addReplyError(
            c,
            b"-NOMASTERLINK Can't SYNC while not connected with my master\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if clientHasPendingReplies(c) != 0 {
        addReplyError(
            c,
            b"SYNC and PSYNC are invalid with pending output\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*c).slave_req
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) != 0
        && (*c).slave_capa & (1 as libc::c_int) << 0 as libc::c_int == 0
    {
        addReplyError(
            c,
            b"Filtered replica requires EOF capability\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Replica %s asks for synchronization\0" as *const u8 as *const libc::c_char,
            replicationGetSlaveName(c),
        );
    }
    if strcasecmp(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"psync\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut psync_offset: libc::c_longlong = 0;
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut psync_offset,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Replica %s asks for synchronization but with a wrong offset\0"
                        as *const u8 as *const libc::c_char,
                    replicationGetSlaveName(c),
                );
            }
            return;
        }
        if masterTryPartialResynchronization(c, psync_offset) == 0 as libc::c_int {
            server.stat_sync_partial_ok += 1;
            return;
        } else {
            let mut master_replid: *mut libc::c_char = (**((*c).argv)
                .offset(1 as libc::c_int as isize))
                .ptr as *mut libc::c_char;
            if *master_replid.offset(0 as libc::c_int as isize) as libc::c_int
                != '?' as i32
            {
                server.stat_sync_partial_err += 1;
            }
        }
    } else {
        (*c).flags |= ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong;
    }
    server.stat_sync_full += 1;
    (*c).replstate = 6 as libc::c_int;
    if server.repl_disable_tcp_nodelay != 0 {
        connDisableTcpNoDelay((*c).conn);
    }
    (*c).repldbfd = -(1 as libc::c_int);
    (*c).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    listAddNodeTail(server.slaves, c as *mut libc::c_void);
    if (*server.slaves).len == 1 as libc::c_int as libc::c_ulong
        && (server.repl_backlog).is_null()
    {
        changeReplicationId();
        clearReplicationId2();
        createReplicationBacklog();
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Replication backlog created, my new replication IDs are '%s' and '%s'\0"
                    as *const u8 as *const libc::c_char,
                (server.replid).as_mut_ptr(),
                (server.replid2).as_mut_ptr(),
            );
        }
    }
    if server.child_type == 1 as libc::c_int && server.rdb_child_type == 1 as libc::c_int
    {
        let mut slave: *mut client = 0 as *mut client;
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            slave = (*ln).value as *mut client;
            if (*slave).replstate == 7 as libc::c_int
                && ((*slave).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 42 as libc::c_int == 0
                    || (*c).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 42 as libc::c_int != 0)
            {
                break;
            }
        }
        if !ln.is_null() && (*c).slave_capa & (*slave).slave_capa == (*slave).slave_capa
            && (*c).slave_req == (*slave).slave_req
        {
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 42 as libc::c_int == 0
            {
                copyReplicaOutputBuffer(c, slave);
            }
            replicationSetupSlaveForFullResync(c, (*slave).psync_initial_offset);
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Waiting for end of BGSAVE for SYNC\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Can't attach the replica to the current BGSAVE. Waiting for next BGSAVE for SYNC\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if server.child_type == 1 as libc::c_int
        && server.rdb_child_type == 2 as libc::c_int
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Current BGSAVE has socket target. Waiting for next BGSAVE for SYNC\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if server.repl_diskless_sync != 0
        && (*c).slave_capa & (1 as libc::c_int) << 0 as libc::c_int != 0
        && server.repl_diskless_sync_delay != 0
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Delay next BGSAVE for diskless SYNC\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if hasActiveChildProcess() == 0 {
        startBgsaveForReplication((*c).slave_capa, (*c).slave_req);
    } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"No BGSAVE in progress, but another BG operation is active. BGSAVE for replication delayed\0"
                as *const u8 as *const libc::c_char,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn replconfCommand(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    if (*c).argc % 2 as libc::c_int == 0 as libc::c_int {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    j = 1 as libc::c_int;
    while j < (*c).argc {
        if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"listening-port\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut port: libc::c_long = 0;
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut port,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            (*c).slave_listening_port = port as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"ip-address\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut addr: sds = (**((*c).argv).offset((j + 1 as libc::c_int) as isize))
                .ptr as sds;
            if sdslen(addr) < 256 as libc::c_int as libc::c_ulong {
                if !((*c).slave_addr).is_null() {
                    sdsfree((*c).slave_addr);
                }
                (*c).slave_addr = sdsdup(addr);
            } else {
                addReplyErrorFormat(
                    c,
                    b"REPLCONF ip-address provided by replica instance is too long: %zd bytes\0"
                        as *const u8 as *const libc::c_char,
                    sdslen(addr),
                );
                return;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"capa\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if strcasecmp(
                (**((*c).argv).offset((j + 1 as libc::c_int) as isize)).ptr
                    as *const libc::c_char,
                b"eof\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*c).slave_capa |= (1 as libc::c_int) << 0 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset((j + 1 as libc::c_int) as isize)).ptr
                    as *const libc::c_char,
                b"psync2\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                (*c).slave_capa |= (1 as libc::c_int) << 1 as libc::c_int;
            }
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"ack\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut offset: libc::c_longlong = 0;
            if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                == 0
            {
                return;
            }
            if getLongLongFromObject(
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut offset,
            ) != 0 as libc::c_int
            {
                return;
            }
            if offset > (*c).repl_ack_off {
                (*c).repl_ack_off = offset;
            }
            (*c).repl_ack_time = server.unixtime as libc::c_longlong;
            if server.child_type == 1 as libc::c_int
                && (*c).replstate == 7 as libc::c_int
            {
                checkChildrenDone();
            }
            if (*c).repl_start_cmd_stream_on_ack != 0
                && (*c).replstate == 9 as libc::c_int
            {
                replicaStartCommandStream(c);
            }
            return;
        } else {
            if strcasecmp(
                (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                b"getack\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                if !(server.masterhost).is_null() && !(server.master).is_null() {
                    replicationSendAck();
                }
                return;
            } else {
                if strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"rdb-only\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    let mut rdb_only: libc::c_long = 0 as libc::c_int as libc::c_long;
                    if getRangeLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                        0 as libc::c_int as libc::c_long,
                        1 as libc::c_int as libc::c_long,
                        &mut rdb_only,
                        0 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        return;
                    }
                    if rdb_only == 1 as libc::c_int as libc::c_long {
                        (*c)
                            .flags = ((*c).flags as libc::c_ulonglong
                            | (1 as libc::c_ulonglong) << 42 as libc::c_int) as uint64_t;
                    } else {
                        (*c)
                            .flags = ((*c).flags as libc::c_ulonglong
                            & !((1 as libc::c_ulonglong) << 42 as libc::c_int))
                            as uint64_t;
                    }
                } else if strcasecmp(
                    (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
                    b"rdb-filter-only\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    let mut filter_count: libc::c_int = 0;
                    let mut i: libc::c_int = 0;
                    let mut filters: *mut sds = 0 as *mut sds;
                    filters = sdssplitargs(
                        (**((*c).argv).offset((j + 1 as libc::c_int) as isize)).ptr
                            as *const libc::c_char,
                        &mut filter_count,
                    );
                    if filters.is_null() {
                        addReplyErrorFormat(
                            c,
                            b"Missing rdb-filter-only values\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    }
                    (*c).slave_req |= (1 as libc::c_int) << 0 as libc::c_int;
                    (*c).slave_req |= (1 as libc::c_int) << 1 as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < filter_count {
                        if strcasecmp(
                            *filters.offset(i as isize) as *const libc::c_char,
                            b"functions\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            (*c).slave_req &= !((1 as libc::c_int) << 1 as libc::c_int);
                        } else {
                            addReplyErrorFormat(
                                c,
                                b"Unsupported rdb-filter-only option: %s\0" as *const u8
                                    as *const libc::c_char,
                                *filters.offset(i as isize),
                            );
                            sdsfreesplitres(filters, filter_count);
                            return;
                        }
                        i += 1;
                    }
                    sdsfreesplitres(filters, filter_count);
                } else {
                    addReplyErrorFormat(
                        c,
                        b"Unrecognized REPLCONF option: %s\0" as *const u8
                            as *const libc::c_char,
                        (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char,
                    );
                    return;
                }
            }
        }
        j += 2 as libc::c_int;
    }
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn replicaPutOnline(mut slave: *mut client) -> libc::c_int {
    if (*slave).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 42 as libc::c_int != 0
    {
        (*slave).replstate = 10 as libc::c_int;
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"RDB transfer completed, rdb only replica (%s) should be disconnected asap\0"
                    as *const u8 as *const libc::c_char,
                replicationGetSlaveName(slave),
            );
        }
        return 0 as libc::c_int;
    }
    (*slave).replstate = 9 as libc::c_int;
    (*slave).repl_ack_time = server.unixtime as libc::c_longlong;
    refreshGoodSlavesCount();
    moduleFireServerEvent(
        6 as libc::c_int as uint64_t,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Synchronization with replica %s succeeded\0" as *const u8
                as *const libc::c_char,
            replicationGetSlaveName(slave),
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicaStartCommandStream(mut slave: *mut client) {
    if (*slave).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 42 as libc::c_int == 0
    {} else {
        _serverAssert(
            b"!(slave->flags & CLIENT_REPL_RDBONLY)\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            1292 as libc::c_int,
        );
        unreachable!();
    };
    (*slave).repl_start_cmd_stream_on_ack = 0 as libc::c_int;
    putClientInPendingWriteQueue(slave);
}
#[no_mangle]
pub unsafe extern "C" fn removeRDBUsedToSyncReplicas() {
    if server.rdb_del_sync_files == 0 {
        RDBGeneratedByReplication = 0 as libc::c_int;
        return;
    }
    if allPersistenceDisabled() != 0 && RDBGeneratedByReplication != 0 {
        let mut slave: *mut client = 0 as *mut client;
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut delrdb: libc::c_int = 1 as libc::c_int;
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            slave = (*ln).value as *mut client;
            if !((*slave).replstate == 6 as libc::c_int
                || (*slave).replstate == 7 as libc::c_int
                || (*slave).replstate == 8 as libc::c_int)
            {
                continue;
            }
            delrdb = 0 as libc::c_int;
            break;
        }
        if delrdb != 0 {
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
            if lstat(server.rdb_filename, &mut sb) != -(1 as libc::c_int) {
                RDBGeneratedByReplication = 0 as libc::c_int;
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Removing the RDB file used to feed replicas in a persistence-less instance\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                bg_unlink(server.rdb_filename);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn sendBulkToSlave(mut conn: *mut connection) {
    let mut slave: *mut client = connGetPrivateData(conn) as *mut client;
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut nwritten: ssize_t = 0;
    let mut buflen: ssize_t = 0;
    if !((*slave).replpreamble).is_null() {
        nwritten = connWrite(
            conn,
            (*slave).replpreamble as *const libc::c_void,
            sdslen((*slave).replpreamble),
        ) as ssize_t;
        if nwritten == -(1 as libc::c_int) as libc::c_long {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Write error sending RDB preamble to replica: %s\0" as *const u8
                        as *const libc::c_char,
                    connGetLastError(conn),
                );
            }
            freeClient(slave);
            return;
        }
        let fresh4 = &mut server.stat_net_repl_output_bytes;
        let fresh5 = nwritten as libc::c_longlong;
        core::intrinsics::atomic_xadd_relaxed(fresh4, fresh5) + fresh5;
        sdsrange((*slave).replpreamble, nwritten, -(1 as libc::c_int) as ssize_t);
        if sdslen((*slave).replpreamble) == 0 as libc::c_int as libc::c_ulong {
            sdsfree((*slave).replpreamble);
            (*slave).replpreamble = 0 as sds;
        } else {
            return
        }
    }
    lseek((*slave).repldbfd, (*slave).repldboff, 0 as libc::c_int);
    buflen = read(
        (*slave).repldbfd,
        buf.as_mut_ptr() as *mut libc::c_void,
        (1024 as libc::c_int * 16 as libc::c_int) as size_t,
    );
    if buflen <= 0 as libc::c_int as libc::c_long {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Read error sending DB to replica: %s\0" as *const u8
                    as *const libc::c_char,
                if buflen == 0 as libc::c_int as libc::c_long {
                    b"premature EOF\0" as *const u8 as *const libc::c_char
                } else {
                    strerror(*__errno_location()) as *const libc::c_char
                },
            );
        }
        freeClient(slave);
        return;
    }
    nwritten = connWrite(conn, buf.as_mut_ptr() as *const libc::c_void, buflen as size_t)
        as ssize_t;
    if nwritten == -(1 as libc::c_int) as libc::c_long {
        if connGetState(conn) != CONN_STATE_CONNECTED as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Write error sending DB to replica: %s\0" as *const u8
                        as *const libc::c_char,
                    connGetLastError(conn),
                );
            }
            freeClient(slave);
        }
        return;
    }
    (*slave).repldboff += nwritten;
    let fresh6 = &mut server.stat_net_repl_output_bytes;
    let fresh7 = nwritten as libc::c_longlong;
    core::intrinsics::atomic_xadd_relaxed(fresh6, fresh7) + fresh7;
    if (*slave).repldboff == (*slave).repldbsize {
        close((*slave).repldbfd);
        (*slave).repldbfd = -(1 as libc::c_int);
        connSetWriteHandler((*slave).conn, None);
        if replicaPutOnline(slave) == 0 {
            freeClient(slave);
            return;
        }
        replicaStartCommandStream(slave);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdbPipeWriteHandlerConnRemoved(mut conn: *mut connection) {
    if connHasWriteHandler(conn) == 0 {
        return;
    }
    connSetWriteHandler(conn, None);
    let mut slave: *mut client = connGetPrivateData(conn) as *mut client;
    (*slave).repl_last_partial_write = 0 as libc::c_int as libc::c_longlong;
    server.rdb_pipe_numconns_writing -= 1;
    if server.rdb_pipe_numconns_writing == 0 as libc::c_int {
        if aeCreateFileEvent(
            server.el,
            server.rdb_pipe_read,
            1 as libc::c_int,
            Some(
                rdbPipeReadHandler
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int)
        {
            _serverPanic(
                b"replication.c\0" as *const u8 as *const libc::c_char,
                1416 as libc::c_int,
                b"Unrecoverable error creating server.rdb_pipe_read file event.\0"
                    as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdbPipeWriteHandler(mut conn: *mut connection) {
    if server.rdb_pipe_bufflen > 0 as libc::c_int {} else {
        _serverAssert(
            b"server.rdb_pipe_bufflen>0\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            1424 as libc::c_int,
        );
        unreachable!();
    };
    let mut slave: *mut client = connGetPrivateData(conn) as *mut client;
    let mut nwritten: ssize_t = 0;
    nwritten = connWrite(
        conn,
        (server.rdb_pipe_buff).offset((*slave).repldboff as isize)
            as *const libc::c_void,
        (server.rdb_pipe_bufflen as libc::c_long - (*slave).repldboff) as size_t,
    ) as ssize_t;
    if nwritten == -(1 as libc::c_int) as libc::c_long {
        if connGetState(conn) == CONN_STATE_CONNECTED as libc::c_int {
            return;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Write error sending DB to replica: %s\0" as *const u8
                    as *const libc::c_char,
                connGetLastError(conn),
            );
        }
        freeClient(slave);
        return;
    } else {
        (*slave).repldboff += nwritten;
        let fresh8 = &mut server.stat_net_repl_output_bytes;
        let fresh9 = nwritten as libc::c_longlong;
        core::intrinsics::atomic_xadd_relaxed(fresh8, fresh9) + fresh9;
        if (*slave).repldboff < server.rdb_pipe_bufflen as libc::c_long {
            (*slave).repl_last_partial_write = server.unixtime as libc::c_longlong;
            return;
        }
    }
    rdbPipeWriteHandlerConnRemoved(conn);
}
#[no_mangle]
pub unsafe extern "C" fn rdbPipeReadHandler(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut clientData: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if (server.rdb_pipe_buff).is_null() {
        server
            .rdb_pipe_buff = zmalloc((1024 as libc::c_int * 16 as libc::c_int) as size_t)
            as *mut libc::c_char;
    }
    if server.rdb_pipe_numconns_writing == 0 as libc::c_int {} else {
        _serverAssert(
            b"server.rdb_pipe_numconns_writing==0\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            1455 as libc::c_int,
        );
        unreachable!();
    };
    loop {
        server
            .rdb_pipe_bufflen = read(
            fd,
            server.rdb_pipe_buff as *mut libc::c_void,
            (1024 as libc::c_int * 16 as libc::c_int) as size_t,
        ) as libc::c_int;
        if server.rdb_pipe_bufflen < 0 as libc::c_int {
            if *__errno_location() == 11 as libc::c_int
                || *__errno_location() == 11 as libc::c_int
            {
                return;
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Diskless rdb transfer, read error sending DB to replicas: %s\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            i = 0 as libc::c_int;
            while i < server.rdb_pipe_numconns {
                let mut conn: *mut connection = *(server.rdb_pipe_conns)
                    .offset(i as isize);
                if !conn.is_null() {
                    let mut slave: *mut client = connGetPrivateData(conn) as *mut client;
                    freeClient(slave);
                    let ref mut fresh10 = *(server.rdb_pipe_conns).offset(i as isize);
                    *fresh10 = 0 as *mut connection;
                }
                i += 1;
            }
            killRDBChild();
            return;
        }
        if server.rdb_pipe_bufflen == 0 as libc::c_int {
            let mut stillUp: libc::c_int = 0 as libc::c_int;
            aeDeleteFileEvent(server.el, server.rdb_pipe_read, 1 as libc::c_int);
            i = 0 as libc::c_int;
            while i < server.rdb_pipe_numconns {
                let mut conn_0: *mut connection = *(server.rdb_pipe_conns)
                    .offset(i as isize);
                if !conn_0.is_null() {
                    stillUp += 1;
                }
                i += 1;
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Diskless rdb transfer, done reading from pipe, %d replicas still up.\0"
                        as *const u8 as *const libc::c_char,
                    stillUp,
                );
            }
            close(server.rdb_child_exit_pipe);
            server.rdb_child_exit_pipe = -(1 as libc::c_int);
            return;
        }
        let mut stillAlive: libc::c_int = 0 as libc::c_int;
        let mut current_block_44: u64;
        i = 0 as libc::c_int;
        while i < server.rdb_pipe_numconns {
            let mut nwritten: ssize_t = 0;
            let mut conn_1: *mut connection = *(server.rdb_pipe_conns)
                .offset(i as isize);
            if !conn_1.is_null() {
                let mut slave_0: *mut client = connGetPrivateData(conn_1) as *mut client;
                nwritten = connWrite(
                    conn_1,
                    server.rdb_pipe_buff as *const libc::c_void,
                    server.rdb_pipe_bufflen as size_t,
                ) as ssize_t;
                if nwritten == -(1 as libc::c_int) as libc::c_long {
                    if connGetState(conn_1) != CONN_STATE_CONNECTED as libc::c_int {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Diskless rdb transfer, write error sending DB to replica: %s\0"
                                    as *const u8 as *const libc::c_char,
                                connGetLastError(conn_1),
                            );
                        }
                        freeClient(slave_0);
                        let ref mut fresh11 = *(server.rdb_pipe_conns)
                            .offset(i as isize);
                        *fresh11 = 0 as *mut connection;
                        current_block_44 = 3275366147856559585;
                    } else {
                        (*slave_0).repldboff = 0 as libc::c_int as off_t;
                        current_block_44 = 11626999923138678822;
                    }
                } else {
                    (*slave_0).repldboff = nwritten;
                    let fresh12 = &mut server.stat_net_repl_output_bytes;
                    let fresh13 = nwritten as libc::c_longlong;
                    core::intrinsics::atomic_xadd_relaxed(fresh12, fresh13) + fresh13;
                    current_block_44 = 11626999923138678822;
                }
                match current_block_44 {
                    3275366147856559585 => {}
                    _ => {
                        if nwritten != server.rdb_pipe_bufflen as libc::c_long {
                            (*slave_0)
                                .repl_last_partial_write = server.unixtime
                                as libc::c_longlong;
                            server.rdb_pipe_numconns_writing += 1;
                            connSetWriteHandler(
                                conn_1,
                                Some(
                                    rdbPipeWriteHandler
                                        as unsafe extern "C" fn(*mut connection) -> (),
                                ),
                            );
                        }
                        stillAlive += 1;
                    }
                }
            }
            i += 1;
        }
        if stillAlive == 0 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Diskless rdb transfer, last replica dropped, killing fork child.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            killRDBChild();
        }
        if !(server.rdb_pipe_numconns_writing != 0 || stillAlive == 0 as libc::c_int) {
            continue;
        }
        aeDeleteFileEvent(server.el, server.rdb_pipe_read, 1 as libc::c_int);
        break;
    };
}
#[no_mangle]
pub unsafe extern "C" fn updateSlavesWaitingBgsave(
    mut bgsaveerr: libc::c_int,
    mut type_0: libc::c_int,
) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        if !((*slave).replstate == 7 as libc::c_int) {
            continue;
        }
        let mut buf: stat = stat {
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
        if bgsaveerr != 0 as libc::c_int {
            freeClientAsync(slave);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"SYNC failed. BGSAVE child returned an error\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if type_0 == 2 as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Streamed RDB transfer with replica %s succeeded (socket). Waiting for REPLCONF ACK from slave to enable streaming\0"
                        as *const u8 as *const libc::c_char,
                    replicationGetSlaveName(slave),
                );
            }
            if replicaPutOnline(slave) == 0 {
                freeClientAsync(slave);
            } else {
                (*slave).repl_start_cmd_stream_on_ack = 1 as libc::c_int;
            }
        } else {
            (*slave).repldbfd = open(server.rdb_filename, 0 as libc::c_int);
            if (*slave).repldbfd == -(1 as libc::c_int)
                || fstat((*slave).repldbfd, &mut buf) == -(1 as libc::c_int)
            {
                freeClientAsync(slave);
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"SYNC failed. Can't open/stat DB after BGSAVE: %s\0"
                            as *const u8 as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            } else {
                (*slave).repldboff = 0 as libc::c_int as off_t;
                (*slave).repldbsize = buf.st_size;
                (*slave).replstate = 8 as libc::c_int;
                (*slave)
                    .replpreamble = sdscatprintf(
                    sdsempty(),
                    b"$%lld\r\n\0" as *const u8 as *const libc::c_char,
                    (*slave).repldbsize as libc::c_ulonglong,
                );
                connSetWriteHandler((*slave).conn, None);
                if !(connSetWriteHandler(
                    (*slave).conn,
                    Some(sendBulkToSlave as unsafe extern "C" fn(*mut connection) -> ()),
                ) == -(1 as libc::c_int))
                {
                    continue;
                }
                freeClientAsync(slave);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn changeReplicationId() {
    getRandomHexChars((server.replid).as_mut_ptr(), 40 as libc::c_int as size_t);
    server.replid[40 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn clearReplicationId2() {
    memset(
        (server.replid2).as_mut_ptr() as *mut libc::c_void,
        '0' as i32,
        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    );
    server.replid2[40 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    server.second_replid_offset = -(1 as libc::c_int) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn shiftReplicationId() {
    memcpy(
        (server.replid2).as_mut_ptr() as *mut libc::c_void,
        (server.replid).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    );
    server
        .second_replid_offset = server.master_repl_offset
        + 1 as libc::c_int as libc::c_longlong;
    changeReplicationId();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Setting secondary replication ID to %s, valid up to offset: %lld. New replication ID is %s\0"
                as *const u8 as *const libc::c_char,
            (server.replid2).as_mut_ptr(),
            server.second_replid_offset,
            (server.replid).as_mut_ptr(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn slaveIsInHandshakeState() -> libc::c_int {
    return (server.repl_state >= REPL_STATE_RECEIVE_PING_REPLY as libc::c_int
        && server.repl_state <= REPL_STATE_RECEIVE_PSYNC_REPLY as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicationSendNewlineToMaster() {
    static mut newline_sent: time_t = 0;
    if time(0 as *mut time_t) != newline_sent {
        newline_sent = time(0 as *mut time_t);
        if !(server.repl_transfer_s).is_null() {
            connWrite(
                server.repl_transfer_s,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicationEmptyDbCallback(mut d: *mut dict) {
    if server.repl_state == REPL_STATE_TRANSFER as libc::c_int {
        replicationSendNewlineToMaster();
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicationCreateMasterClient(
    mut conn: *mut connection,
    mut dbid: libc::c_int,
) {
    server.master = createClient(conn);
    if !conn.is_null() {
        connSetReadHandler(
            (*server.master).conn,
            Some(readQueryFromClient as unsafe extern "C" fn(*mut connection) -> ()),
        );
    }
    (*server.master).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
    (*server.master).authenticated = 1 as libc::c_int;
    (*server.master).reploff = server.master_initial_offset;
    (*server.master).read_reploff = (*server.master).reploff;
    (*server.master).user = 0 as *mut user;
    memcpy(
        ((*server.master).replid).as_mut_ptr() as *mut libc::c_void,
        (server.master_replid).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    );
    if (*server.master).reploff == -(1 as libc::c_int) as libc::c_longlong {
        (*server.master).flags
            |= ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong;
    }
    if dbid != -(1 as libc::c_int) {
        selectDb(server.master, dbid);
    }
}
#[no_mangle]
pub unsafe extern "C" fn restartAOFAfterSYNC() {
    let mut tries: libc::c_uint = 0;
    let mut max_tries: libc::c_uint = 10 as libc::c_int as libc::c_uint;
    tries = 0 as libc::c_int as libc::c_uint;
    while tries < max_tries {
        if startAppendOnly() == 0 as libc::c_int {
            break;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed enabling the AOF after successful master synchronization! Trying it again in one second.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        sleep(1 as libc::c_int as libc::c_uint);
        tries = tries.wrapping_add(1);
    }
    if tries == max_tries {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"FATAL: this replica instance finished the synchronization with its master, but the AOF can't be turned on. Exiting now.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
}
unsafe extern "C" fn useDisklessLoad() -> libc::c_int {
    let mut enabled: libc::c_int = (server.repl_diskless_load == 2 as libc::c_int
        || server.repl_diskless_load == 1 as libc::c_int
            && dbTotalServerKeyCount() == 0 as libc::c_int as libc::c_longlong)
        as libc::c_int;
    if enabled != 0 {
        if moduleAllDatatypesHandleErrors() == 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Skipping diskless-load because there are modules that don't handle read errors.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            enabled = 0 as libc::c_int;
        } else if server.repl_diskless_load == 2 as libc::c_int
            && moduleAllModulesHandleReplAsyncLoad() == 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Skipping diskless-load because there are modules that are not aware of async replication.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            enabled = 0 as libc::c_int;
        }
    }
    return enabled;
}
#[no_mangle]
pub unsafe extern "C" fn disklessLoadInitTempDb() -> *mut redisDb {
    return initTempDb();
}
#[no_mangle]
pub unsafe extern "C" fn disklessLoadDiscardTempDb(mut tempDb: *mut redisDb) {
    discardTempDb(
        tempDb,
        Some(replicationEmptyDbCallback as unsafe extern "C" fn(*mut dict) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn replicationAttachToNewMaster() {
    if (server.master).is_null() {} else {
        _serverAssert(
            b"server.master == NULL\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            1799 as libc::c_int,
        );
        unreachable!();
    };
    replicationDiscardCachedMaster();
    disconnectSlaves();
    freeReplicationBacklog();
}
#[no_mangle]
pub unsafe extern "C" fn readSyncBulkPayload(mut conn: *mut connection) {
    let mut rsi: rdbSaveInfo = rdbSaveInfo {
        repl_stream_db: 0,
        repl_id_is_set: 0,
        repl_id: [0; 41],
        repl_offset: 0,
    };
    let mut current_block: u64;
    let mut buf: [libc::c_char; 16384] = [0; 16384];
    let mut nread: ssize_t = 0;
    let mut readlen: ssize_t = 0;
    let mut nwritten: ssize_t = 0;
    let mut use_diskless_load: libc::c_int = useDisklessLoad();
    let mut diskless_load_tempDb: *mut redisDb = 0 as *mut redisDb;
    let mut temp_functions_lib_ctx: *mut functionsLibCtx = 0 as *mut functionsLibCtx;
    let mut empty_db_flags: libc::c_int = if server.repl_slave_lazy_flush != 0 {
        (1 as libc::c_int) << 0 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut left: off_t = 0;
    static mut eofmark: [libc::c_char; 40] = [0; 40];
    static mut lastbytes: [libc::c_char; 40] = [0; 40];
    static mut usemark: libc::c_int = 0 as libc::c_int;
    if server.repl_transfer_size == -(1 as libc::c_int) as libc::c_long {
        nread = connSyncReadLine(
            conn,
            buf.as_mut_ptr(),
            1024 as libc::c_int as ssize_t,
            (server.repl_syncio_timeout * 1000 as libc::c_int) as libc::c_longlong,
        );
        if nread == -(1 as libc::c_int) as libc::c_long {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"I/O error reading bulk count from MASTER: %s\0" as *const u8
                        as *const libc::c_char,
                    connGetLastError(conn),
                );
            }
        } else {
            let fresh14 = &mut server.stat_net_repl_input_bytes;
            let fresh15 = (nread + 1 as libc::c_int as libc::c_long) as libc::c_longlong;
            core::intrinsics::atomic_xadd_relaxed(fresh14, fresh15) + fresh15;
            if buf[0 as libc::c_int as usize] as libc::c_int == '-' as i32 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"MASTER aborted replication with an error: %s\0" as *const u8
                            as *const libc::c_char,
                        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                }
            } else if buf[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                server.repl_transfer_lastio = server.unixtime as time_t;
                return;
            } else if buf[0 as libc::c_int as usize] as libc::c_int != '$' as i32 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Bad protocol from MASTER, the first byte is not '$' (we received '%s'), are you sure the host and port are right?\0"
                            as *const u8 as *const libc::c_char,
                        buf.as_mut_ptr(),
                    );
                }
            } else {
                if strncmp(
                    buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                    b"EOF:\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                    && strlen(buf.as_mut_ptr().offset(5 as libc::c_int as isize))
                        >= 40 as libc::c_int as libc::c_ulong
                {
                    usemark = 1 as libc::c_int;
                    memcpy(
                        eofmark.as_mut_ptr() as *mut libc::c_void,
                        buf.as_mut_ptr().offset(5 as libc::c_int as isize)
                            as *const libc::c_void,
                        40 as libc::c_int as libc::c_ulong,
                    );
                    memset(
                        lastbytes.as_mut_ptr() as *mut libc::c_void,
                        0 as libc::c_int,
                        40 as libc::c_int as libc::c_ulong,
                    );
                    server.repl_transfer_size = 0 as libc::c_int as off_t;
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"MASTER <-> REPLICA sync: receiving streamed RDB from master with EOF %s\0"
                                as *const u8 as *const libc::c_char,
                            if use_diskless_load != 0 {
                                b"to parser\0" as *const u8 as *const libc::c_char
                            } else {
                                b"to disk\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                } else {
                    usemark = 0 as libc::c_int;
                    server
                        .repl_transfer_size = strtol(
                        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    );
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"MASTER <-> REPLICA sync: receiving %lld bytes from master %s\0"
                                as *const u8 as *const libc::c_char,
                            server.repl_transfer_size as libc::c_longlong,
                            if use_diskless_load != 0 {
                                b"to parser\0" as *const u8 as *const libc::c_char
                            } else {
                                b"to disk\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                }
                return;
            }
        }
    } else {
        if use_diskless_load == 0 {
            if usemark != 0 {
                readlen = core::mem::size_of::<[libc::c_char; 16384]>()
                    as libc::c_ulong as ssize_t;
            } else {
                left = server.repl_transfer_size - server.repl_transfer_read;
                readlen = if left
                    < core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
                        as libc::c_int as libc::c_long
                {
                    left
                } else {
                    core::mem::size_of::<[libc::c_char; 16384]>() as libc::c_ulong
                        as libc::c_int as libc::c_long
                };
            }
            nread = connRead(
                conn,
                buf.as_mut_ptr() as *mut libc::c_void,
                readlen as size_t,
            ) as ssize_t;
            if nread <= 0 as libc::c_int as libc::c_long {
                if connGetState(conn) == CONN_STATE_CONNECTED as libc::c_int {
                    return;
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"I/O error trying to sync with MASTER: %s\0" as *const u8
                            as *const libc::c_char,
                        if nread == -(1 as libc::c_int) as libc::c_long {
                            connGetLastError(conn)
                        } else {
                            b"connection lost\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                cancelReplicationHandshake(1 as libc::c_int);
                return;
            }
            let fresh16 = &mut server.stat_net_repl_input_bytes;
            let fresh17 = nread as libc::c_longlong;
            core::intrinsics::atomic_xadd_relaxed(fresh16, fresh17) + fresh17;
            let mut eof_reached: libc::c_int = 0 as libc::c_int;
            if usemark != 0 {
                if nread >= 40 as libc::c_int as libc::c_long {
                    memcpy(
                        lastbytes.as_mut_ptr() as *mut libc::c_void,
                        buf
                            .as_mut_ptr()
                            .offset(nread as isize)
                            .offset(-(40 as libc::c_int as isize))
                            as *const libc::c_void,
                        40 as libc::c_int as libc::c_ulong,
                    );
                } else {
                    let mut rem: libc::c_int = (40 as libc::c_int as libc::c_long
                        - nread) as libc::c_int;
                    memmove(
                        lastbytes.as_mut_ptr() as *mut libc::c_void,
                        lastbytes.as_mut_ptr().offset(nread as isize)
                            as *const libc::c_void,
                        rem as libc::c_ulong,
                    );
                    memcpy(
                        lastbytes.as_mut_ptr().offset(rem as isize) as *mut libc::c_void,
                        buf.as_mut_ptr() as *const libc::c_void,
                        nread as libc::c_ulong,
                    );
                }
                if memcmp(
                    lastbytes.as_mut_ptr() as *const libc::c_void,
                    eofmark.as_mut_ptr() as *const libc::c_void,
                    40 as libc::c_int as libc::c_ulong,
                ) == 0 as libc::c_int
                {
                    eof_reached = 1 as libc::c_int;
                }
            }
            server.repl_transfer_lastio = server.unixtime as time_t;
            nwritten = write(
                server.repl_transfer_fd,
                buf.as_mut_ptr() as *const libc::c_void,
                nread as size_t,
            );
            if nwritten != nread {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Write error or short write writing to the DB dump file needed for MASTER <-> REPLICA synchronization: %s\0"
                            as *const u8 as *const libc::c_char,
                        if nwritten == -(1 as libc::c_int) as libc::c_long {
                            strerror(*__errno_location()) as *const libc::c_char
                        } else {
                            b"short write\0" as *const u8 as *const libc::c_char
                        },
                    );
                }
                current_block = 705019609901258711;
            } else {
                server.repl_transfer_read += nread;
                if usemark != 0 && eof_reached != 0 {
                    if ftruncate(
                        server.repl_transfer_fd,
                        server.repl_transfer_read - 40 as libc::c_int as libc::c_long,
                    ) == -(1 as libc::c_int)
                    {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Error truncating the RDB file received from the master for SYNC: %s\0"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                        current_block = 705019609901258711;
                    } else {
                        current_block = 2723324002591448311;
                    }
                } else {
                    current_block = 2723324002591448311;
                }
                match current_block {
                    705019609901258711 => {}
                    _ => {
                        if server.repl_transfer_read
                            >= server.repl_transfer_last_fsync_off
                                + (1024 as libc::c_int * 1024 as libc::c_int
                                    * 8 as libc::c_int) as libc::c_long
                        {
                            let mut sync_size: off_t = server.repl_transfer_read
                                - server.repl_transfer_last_fsync_off;
                            sync_file_range(
                                server.repl_transfer_fd,
                                server.repl_transfer_last_fsync_off,
                                sync_size,
                                (1 as libc::c_int | 2 as libc::c_int) as libc::c_uint,
                            );
                            server.repl_transfer_last_fsync_off += sync_size;
                        }
                        if usemark == 0 {
                            if server.repl_transfer_read == server.repl_transfer_size {
                                eof_reached = 1 as libc::c_int;
                            }
                        }
                        if eof_reached == 0 {
                            return;
                        }
                        current_block = 6002151390280567665;
                    }
                }
            }
        } else {
            current_block = 6002151390280567665;
        }
        match current_block {
            705019609901258711 => {}
            _ => {
                if server.aof_state != 0 as libc::c_int {
                    stopAppendOnly();
                }
                if server.child_type == 1 as libc::c_int {
                    if use_diskless_load == 0 {
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"Replica is about to load the RDB file received from the master, but there is a pending RDB child running. Killing process %ld and removing its temp file to avoid any race\0"
                                    as *const u8 as *const libc::c_char,
                                server.child_pid as libc::c_long,
                            );
                        }
                    }
                    killRDBChild();
                }
                if use_diskless_load != 0
                    && server.repl_diskless_load == 2 as libc::c_int
                {
                    diskless_load_tempDb = disklessLoadInitTempDb();
                    temp_functions_lib_ctx = functionsLibCtxCreate();
                    moduleFireServerEvent(
                        14 as libc::c_int as uint64_t,
                        0 as libc::c_int,
                        0 as *mut libc::c_void,
                    );
                } else {
                    replicationAttachToNewMaster();
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"MASTER <-> REPLICA sync: Flushing old data\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    emptyData(
                        -(1 as libc::c_int),
                        empty_db_flags,
                        Some(
                            replicationEmptyDbCallback
                                as unsafe extern "C" fn(*mut dict) -> (),
                        ),
                    );
                }
                connSetReadHandler(conn, None);
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"MASTER <-> REPLICA sync: Loading DB in memory\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                rsi = {
                    let mut init: rdbSaveInfo = {
                        let repl_id_bytes: &[u8; 41] = b"0000000000000000000000000000000000000000\0";
                        let repl_id_chars: &mut [libc::c_char; 41] = unsafe {
                            &mut *(&repl_id_bytes as *const _ as *mut [libc::c_char; 41])
                        };
                        let repl_id_cell = UnsafeCell::new(repl_id_chars);
                        rdbSaveInfo {
                            repl_stream_db: -(1 as libc::c_int),
                            repl_id_is_set: 0 as libc::c_int,
                            repl_id: unsafe { **repl_id_cell.get() },
                            repl_offset: -(1 as libc::c_int) as libc::c_longlong,
                        }
                    };

                    init
                };
                if use_diskless_load != 0 {
                    let mut rdb: rio = rio {
                        read: None,
                        write: None,
                        tell: None,
                        flush: None,
                        update_cksum: None,
                        cksum: 0,
                        flags: 0,
                        processed_bytes: 0,
                        max_processing_chunk: 0,
                        io: C2RustUnnamed {
                            buffer: C2RustUnnamed_3 {
                                ptr: 0 as *mut libc::c_char,
                                pos: 0,
                            },
                        },
                    };
                    let mut dbarray: *mut redisDb = 0 as *mut redisDb;
                    let mut functions_lib_ctx: *mut functionsLibCtx = 0
                        as *mut functionsLibCtx;
                    let mut asyncLoading: libc::c_int = 0 as libc::c_int;
                    if server.repl_diskless_load == 2 as libc::c_int {
                        if memcmp(
                            (server.replid).as_mut_ptr() as *const libc::c_void,
                            (server.master_replid).as_mut_ptr() as *const libc::c_void,
                            40 as libc::c_int as libc::c_ulong,
                        ) == 0 as libc::c_int
                        {
                            asyncLoading = 1 as libc::c_int;
                        }
                        dbarray = diskless_load_tempDb;
                        functions_lib_ctx = temp_functions_lib_ctx;
                    } else {
                        dbarray = server.db;
                        functions_lib_ctx = functionsLibCtxGetCurrent();
                        functionsLibCtxClear(functions_lib_ctx);
                    }
                    rioInitWithConn(&mut rdb, conn, server.repl_transfer_size as size_t);
                    connBlock(conn);
                    connRecvTimeout(
                        conn,
                        (server.repl_timeout * 1000 as libc::c_int) as libc::c_longlong,
                    );
                    startLoading(
                        server.repl_transfer_size as size_t,
                        (1 as libc::c_int) << 1 as libc::c_int,
                        asyncLoading,
                    );
                    let mut loadingFailed: libc::c_int = 0 as libc::c_int;
                    let mut loadingCtx: rdbLoadingCtx = {
                        let mut init = rdbLoadingCtx {
                            dbarray: dbarray,
                            functions_lib_ctx: functions_lib_ctx,
                        };
                        init
                    };
                    if rdbLoadRioWithLoadingCtx(
                        &mut rdb,
                        (1 as libc::c_int) << 1 as libc::c_int,
                        &mut rsi,
                        &mut loadingCtx,
                    ) != 0 as libc::c_int
                    {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed trying to load the MASTER synchronization DB from socket: %s\0"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                        loadingFailed = 1 as libc::c_int;
                    } else if usemark != 0 {
                        if rioRead(
                            &mut rdb,
                            buf.as_mut_ptr() as *mut libc::c_void,
                            40 as libc::c_int as size_t,
                        ) == 0
                            || memcmp(
                                buf.as_mut_ptr() as *const libc::c_void,
                                eofmark.as_mut_ptr() as *const libc::c_void,
                                40 as libc::c_int as libc::c_ulong,
                            ) != 0 as libc::c_int
                        {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"Replication stream EOF marker is broken\0" as *const u8
                                        as *const libc::c_char,
                                );
                            }
                            loadingFailed = 1 as libc::c_int;
                        }
                    }
                    if loadingFailed != 0 {
                        stopLoading(0 as libc::c_int);
                        cancelReplicationHandshake(1 as libc::c_int);
                        rioFreeConn(&mut rdb, 0 as *mut sds);
                        if server.repl_diskless_load == 2 as libc::c_int {
                            moduleFireServerEvent(
                                14 as libc::c_int as uint64_t,
                                1 as libc::c_int,
                                0 as *mut libc::c_void,
                            );
                            disklessLoadDiscardTempDb(diskless_load_tempDb);
                            functionsLibCtxFree(temp_functions_lib_ctx);
                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    2 as libc::c_int,
                                    b"MASTER <-> REPLICA sync: Discarding temporary DB in background\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        } else {
                            emptyData(
                                -(1 as libc::c_int),
                                empty_db_flags,
                                Some(
                                    replicationEmptyDbCallback
                                        as unsafe extern "C" fn(*mut dict) -> (),
                                ),
                            );
                        }
                        return;
                    }
                    if server.repl_diskless_load == 2 as libc::c_int {
                        replicationAttachToNewMaster();
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"MASTER <-> REPLICA sync: Swapping active DB with loaded DB\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        swapMainDbWithTempDb(diskless_load_tempDb);
                        functionsLibCtxSwapWithCurrent(temp_functions_lib_ctx);
                        moduleFireServerEvent(
                            14 as libc::c_int as uint64_t,
                            2 as libc::c_int,
                            0 as *mut libc::c_void,
                        );
                        disklessLoadDiscardTempDb(diskless_load_tempDb);
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"MASTER <-> REPLICA sync: Discarding old DB in background\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    server.dirty += 1;
                    stopLoading(1 as libc::c_int);
                    rioFreeConn(&mut rdb, 0 as *mut sds);
                    connNonBlock(conn);
                    connRecvTimeout(conn, 0 as libc::c_int as libc::c_longlong);
                } else {
                    if fsync(server.repl_transfer_fd) == -(1 as libc::c_int) {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed trying to sync the temp DB to disk in MASTER <-> REPLICA synchronization: %s\0"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                        cancelReplicationHandshake(1 as libc::c_int);
                        return;
                    }
                    let mut old_rdb_fd: libc::c_int = open(
                        server.rdb_filename,
                        0 as libc::c_int | 0o4000 as libc::c_int,
                    );
                    if rename(server.repl_transfer_tmpfile, server.rdb_filename)
                        == -(1 as libc::c_int)
                    {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed trying to rename the temp DB into %s in MASTER <-> REPLICA synchronization: %s\0"
                                    as *const u8 as *const libc::c_char,
                                server.rdb_filename,
                                strerror(*__errno_location()),
                            );
                        }
                        cancelReplicationHandshake(1 as libc::c_int);
                        if old_rdb_fd != -(1 as libc::c_int) {
                            close(old_rdb_fd);
                        }
                        return;
                    }
                    if old_rdb_fd != -(1 as libc::c_int) {
                        bioCreateCloseJob(old_rdb_fd, 0 as libc::c_int);
                    }
                    if fsyncFileDir(server.rdb_filename) == -(1 as libc::c_int) {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed trying to sync DB directory %s in MASTER <-> REPLICA synchronization: %s\0"
                                    as *const u8 as *const libc::c_char,
                                server.rdb_filename,
                                strerror(*__errno_location()),
                            );
                        }
                        cancelReplicationHandshake(1 as libc::c_int);
                        return;
                    }
                    if rdbLoad(
                        server.rdb_filename,
                        &mut rsi,
                        (1 as libc::c_int) << 1 as libc::c_int,
                    ) != 0 as libc::c_int
                    {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed trying to load the MASTER synchronization DB from disk: %s\0"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                        cancelReplicationHandshake(1 as libc::c_int);
                        if server.rdb_del_sync_files != 0
                            && allPersistenceDisabled() != 0
                        {
                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    2 as libc::c_int,
                                    b"Removing the RDB file obtained from the master. This replica has persistence disabled\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            bg_unlink(server.rdb_filename);
                        }
                        return;
                    }
                    if server.rdb_del_sync_files != 0 && allPersistenceDisabled() != 0 {
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"Removing the RDB file obtained from the master. This replica has persistence disabled\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        bg_unlink(server.rdb_filename);
                    }
                    zfree(server.repl_transfer_tmpfile as *mut libc::c_void);
                    close(server.repl_transfer_fd);
                    server.repl_transfer_fd = -(1 as libc::c_int);
                    server.repl_transfer_tmpfile = 0 as *mut libc::c_char;
                }
                replicationCreateMasterClient(
                    server.repl_transfer_s,
                    rsi.repl_stream_db,
                );
                server.repl_state = REPL_STATE_CONNECTED as libc::c_int;
                server.repl_down_since = 0 as libc::c_int as time_t;
                moduleFireServerEvent(
                    7 as libc::c_int as uint64_t,
                    0 as libc::c_int,
                    0 as *mut libc::c_void,
                );
                memcpy(
                    (server.replid).as_mut_ptr() as *mut libc::c_void,
                    ((*server.master).replid).as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                );
                server.master_repl_offset = (*server.master).reploff;
                clearReplicationId2();
                if (server.repl_backlog).is_null() {
                    createReplicationBacklog();
                }
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"MASTER <-> REPLICA sync: Finished with success\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if server.supervised_mode == 2 as libc::c_int {
                    redisCommunicateSystemd(
                        b"STATUS=MASTER <-> REPLICA sync: Finished with success. Ready to accept connections in read-write mode.\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                if usemark != 0 {
                    replicationSendAck();
                }
                if server.aof_enabled != 0 {
                    restartAOFAfterSYNC();
                }
                return;
            }
        }
    }
    cancelReplicationHandshake(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn receiveSynchronousResponse(
    mut conn: *mut connection,
) -> *mut libc::c_char {
    let mut buf: [libc::c_char; 256] = [0; 256];
    if connSyncReadLine(
        conn,
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as ssize_t,
        (server.repl_syncio_timeout * 1000 as libc::c_int) as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed to read response from the server: %s\0" as *const u8
                    as *const libc::c_char,
                connGetLastError(conn),
            );
        }
        return 0 as *mut libc::c_char;
    }
    server.repl_transfer_lastio = server.unixtime as time_t;
    return sdsnew(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn sendCommandRaw(
    mut conn: *mut connection,
    mut cmd: sds,
) -> *mut libc::c_char {
    if connSyncWrite(
        conn,
        cmd,
        sdslen(cmd) as ssize_t,
        (server.repl_syncio_timeout * 1000 as libc::c_int) as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return sdscatprintf(
            sdsempty(),
            b"-Writing to master: %s\0" as *const u8 as *const libc::c_char,
            connGetLastError(conn),
        );
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sendCommand(
    mut conn: *mut connection,
    mut args: ...
) -> *mut libc::c_char {
    let mut ap: core::ffi::VaListImpl;
    let mut cmd: sds = sdsempty();
    let mut cmdargs: sds = sdsempty();
    let mut argslen: size_t = 0 as libc::c_int as size_t;
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    ap = args.clone();
    loop {
        arg = ap.arg::<*mut libc::c_char>();
        if arg.is_null() {
            break;
        }
        cmdargs = sdscatprintf(
            cmdargs,
            b"$%zu\r\n%s\r\n\0" as *const u8 as *const libc::c_char,
            strlen(arg),
            arg,
        );
        argslen = argslen.wrapping_add(1);
    }
    cmd = sdscatprintf(cmd, b"*%zu\r\n\0" as *const u8 as *const libc::c_char, argslen);
    cmd = sdscatsds(cmd, cmdargs);
    sdsfree(cmdargs);
    let mut err: *mut libc::c_char = sendCommandRaw(conn, cmd);
    sdsfree(cmd);
    if !err.is_null() {
        return err;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn sendCommandArgv(
    mut conn: *mut connection,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut argv_lens: *mut size_t,
) -> *mut libc::c_char {
    let mut cmd: sds = sdsempty();
    let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    cmd = sdscatfmt(cmd, b"*%i\r\n\0" as *const u8 as *const libc::c_char, argc);
    i = 0 as libc::c_int;
    while i < argc {
        let mut len: libc::c_int = 0;
        arg = *argv.offset(i as isize);
        len = (if !argv_lens.is_null() {
            *argv_lens.offset(i as isize)
        } else {
            strlen(arg)
        }) as libc::c_int;
        cmd = sdscatfmt(cmd, b"$%i\r\n\0" as *const u8 as *const libc::c_char, len);
        cmd = sdscatlen(cmd, arg as *const libc::c_void, len as size_t);
        cmd = sdscatlen(
            cmd,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        i += 1;
    }
    let mut err: *mut libc::c_char = sendCommandRaw(conn, cmd);
    sdsfree(cmd);
    if !err.is_null() {
        return err;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn slaveTryPartialResynchronization(
    mut conn: *mut connection,
    mut read_reply: libc::c_int,
) -> libc::c_int {
    let mut psync_replid: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut psync_offset: [libc::c_char; 32] = [0; 32];
    let mut reply: sds = 0 as *mut libc::c_char;
    if read_reply == 0 {
        server.master_initial_offset = -(1 as libc::c_int) as libc::c_longlong;
        if !(server.cached_master).is_null() {
            psync_replid = ((*server.cached_master).replid).as_mut_ptr();
            snprintf(
                psync_offset.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
                b"%lld\0" as *const u8 as *const libc::c_char,
                (*server.cached_master).reploff + 1 as libc::c_int as libc::c_longlong,
            );
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Trying a partial resynchronization (request %s:%s).\0" as *const u8
                        as *const libc::c_char,
                    psync_replid,
                    psync_offset.as_mut_ptr(),
                );
            }
        } else {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Partial resynchronization not possible (no cached master)\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            psync_replid = b"?\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            memcpy(
                psync_offset.as_mut_ptr() as *mut libc::c_void,
                b"-1\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                3 as libc::c_int as libc::c_ulong,
            );
        }
        if server.failover_state == FAILOVER_IN_PROGRESS as libc::c_int {
            reply = sendCommand(
                conn,
                b"PSYNC\0" as *const u8 as *const libc::c_char,
                psync_replid,
                psync_offset.as_mut_ptr(),
                b"FAILOVER\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
        } else {
            reply = sendCommand(
                conn,
                b"PSYNC\0" as *const u8 as *const libc::c_char,
                psync_replid,
                psync_offset.as_mut_ptr(),
                0 as *mut libc::c_void,
            );
        }
        if !reply.is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Unable to send PSYNC to master: %s\0" as *const u8
                        as *const libc::c_char,
                    reply,
                );
            }
            sdsfree(reply);
            connSetReadHandler(conn, None);
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    reply = receiveSynchronousResponse(conn);
    if reply.is_null() {
        connSetReadHandler(conn, None);
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Master did not reply to PSYNC, will try later\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 5 as libc::c_int;
    }
    if sdslen(reply) == 0 as libc::c_int as libc::c_ulong {
        sdsfree(reply);
        return 1 as libc::c_int;
    }
    connSetReadHandler(conn, None);
    if strncmp(
        reply as *const libc::c_char,
        b"+FULLRESYNC\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        let mut replid: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut offset: *mut libc::c_char = 0 as *mut libc::c_char;
        replid = strchr(reply as *const libc::c_char, ' ' as i32);
        if !replid.is_null() {
            replid = replid.offset(1);
            offset = strchr(replid, ' ' as i32);
            if !offset.is_null() {
                offset = offset.offset(1);
            }
        }
        if replid.is_null() || offset.is_null()
            || offset.offset_from(replid) as libc::c_long
                - 1 as libc::c_int as libc::c_long != 40 as libc::c_int as libc::c_long
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Master replied with wrong +FULLRESYNC syntax.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            memset(
                (server.master_replid).as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                (40 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
            );
        } else {
            memcpy(
                (server.master_replid).as_mut_ptr() as *mut libc::c_void,
                replid as *const libc::c_void,
                (offset.offset_from(replid) as libc::c_long
                    - 1 as libc::c_int as libc::c_long) as libc::c_ulong,
            );
            server
                .master_replid[40 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            server
                .master_initial_offset = strtoll(
                offset,
                0 as *mut *mut libc::c_char,
                10 as libc::c_int,
            );
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Full resync from master: %s:%lld\0" as *const u8
                        as *const libc::c_char,
                    (server.master_replid).as_mut_ptr(),
                    server.master_initial_offset,
                );
            }
        }
        sdsfree(reply);
        return 3 as libc::c_int;
    }
    if strncmp(
        reply as *const libc::c_char,
        b"+CONTINUE\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Successful partial resynchronization with master.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let mut start: *mut libc::c_char = reply.offset(10 as libc::c_int as isize);
        let mut end: *mut libc::c_char = reply.offset(9 as libc::c_int as isize);
        while *end.offset(0 as libc::c_int as isize) as libc::c_int != '\r' as i32
            && *end.offset(0 as libc::c_int as isize) as libc::c_int != '\n' as i32
            && *end.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
        {
            end = end.offset(1);
        }
        if end.offset_from(start) as libc::c_long == 40 as libc::c_int as libc::c_long {
            let mut new: [libc::c_char; 41] = [0; 41];
            memcpy(
                new.as_mut_ptr() as *mut libc::c_void,
                start as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
            );
            new[40 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
            if strcmp(new.as_mut_ptr(), ((*server.cached_master).replid).as_mut_ptr())
                != 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Master replication ID changed to %s\0" as *const u8
                            as *const libc::c_char,
                        new.as_mut_ptr(),
                    );
                }
                memcpy(
                    (server.replid2).as_mut_ptr() as *mut libc::c_void,
                    ((*server.cached_master).replid).as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                );
                server
                    .second_replid_offset = server.master_repl_offset
                    + 1 as libc::c_int as libc::c_longlong;
                memcpy(
                    (server.replid).as_mut_ptr() as *mut libc::c_void,
                    new.as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                );
                memcpy(
                    ((*server.cached_master).replid).as_mut_ptr() as *mut libc::c_void,
                    new.as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                );
                disconnectSlaves();
            }
        }
        sdsfree(reply);
        replicationResurrectCachedMaster(conn);
        if (server.repl_backlog).is_null() {
            createReplicationBacklog();
        }
        return 2 as libc::c_int;
    }
    if strncmp(
        reply as *const libc::c_char,
        b"-NOMASTERLINK\0" as *const u8 as *const libc::c_char,
        13 as libc::c_int as libc::c_ulong,
    ) == 0
        || strncmp(
            reply as *const libc::c_char,
            b"-LOADING\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Master is currently unable to PSYNC but should be in the future: %s\0"
                    as *const u8 as *const libc::c_char,
                reply,
            );
        }
        sdsfree(reply);
        return 5 as libc::c_int;
    }
    if strncmp(
        reply as *const libc::c_char,
        b"-ERR\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) != 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unexpected reply to PSYNC from master: %s\0" as *const u8
                    as *const libc::c_char,
                reply,
            );
        }
    } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Master does not support PSYNC or is in error state (reply: %s)\0"
                as *const u8 as *const libc::c_char,
            reply,
        );
    }
    sdsfree(reply);
    return 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn syncWithMaster(mut conn: *mut connection) {
    let mut current_block: u64;
    let mut tmpfile: [libc::c_char; 256] = [0; 256];
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dfd: libc::c_int = -(1 as libc::c_int);
    let mut maxtries: libc::c_int = 5 as libc::c_int;
    let mut psync_result: libc::c_int = 0;
    if server.repl_state == REPL_STATE_NONE as libc::c_int {
        connClose(conn);
        return;
    }
    if connGetState(conn) != CONN_STATE_CONNECTED as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error condition on socket for SYNC: %s\0" as *const u8
                    as *const libc::c_char,
                connGetLastError(conn),
            );
        }
    } else {
        if server.repl_state == REPL_STATE_CONNECTING as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Non blocking connect for SYNC fired the event.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            connSetReadHandler(
                conn,
                Some(syncWithMaster as unsafe extern "C" fn(*mut connection) -> ()),
            );
            connSetWriteHandler(conn, None);
            server.repl_state = REPL_STATE_RECEIVE_PING_REPLY as libc::c_int;
            err = sendCommand(
                conn,
                b"PING\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_void,
            );
            if !err.is_null() {
                current_block = 4996778623374542017;
            } else {
                return
            }
        } else {
            if server.repl_state == REPL_STATE_RECEIVE_PING_REPLY as libc::c_int {
                err = receiveSynchronousResponse(conn);
                if err.is_null() {
                    current_block = 11760413616134933622;
                } else if *err.offset(0 as libc::c_int as isize) as libc::c_int
                    != '+' as i32
                    && strncmp(
                        err,
                        b"-NOAUTH\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    && strncmp(
                        err,
                        b"-NOPERM\0" as *const u8 as *const libc::c_char,
                        7 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                    && strncmp(
                        err,
                        b"-ERR operation not permitted\0" as *const u8
                            as *const libc::c_char,
                        28 as libc::c_int as libc::c_ulong,
                    ) != 0 as libc::c_int
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error reply to PING from master: '%s'\0" as *const u8
                                as *const libc::c_char,
                            err,
                        );
                    }
                    sdsfree(err);
                    current_block = 2982467534821209940;
                } else {
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"Master replied to PING, replication can continue...\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    sdsfree(err);
                    err = 0 as *mut libc::c_char;
                    server.repl_state = REPL_STATE_SEND_HANDSHAKE as libc::c_int;
                    current_block = 17281240262373992796;
                }
            } else {
                current_block = 17281240262373992796;
            }
            match current_block {
                2982467534821209940 => {}
                _ => {
                    match current_block {
                        17281240262373992796 => {
                            if server.repl_state
                                == REPL_STATE_SEND_HANDSHAKE as libc::c_int
                            {
                                if !(server.masterauth).is_null() {
                                    let mut args: [*mut libc::c_char; 3] = [
                                        b"AUTH\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        0 as *mut libc::c_char,
                                        0 as *mut libc::c_char,
                                    ];
                                    let mut lens: [size_t; 3] = [
                                        4 as libc::c_int as size_t,
                                        0 as libc::c_int as size_t,
                                        0 as libc::c_int as size_t,
                                    ];
                                    let mut argc: libc::c_int = 1 as libc::c_int;
                                    if !(server.masteruser).is_null() {
                                        args[argc as usize] = server.masteruser;
                                        lens[argc as usize] = strlen(server.masteruser);
                                        argc += 1;
                                    }
                                    args[argc as usize] = server.masterauth;
                                    lens[argc as usize] = sdslen(server.masterauth);
                                    argc += 1;
                                    err = sendCommandArgv(
                                        conn,
                                        argc,
                                        args.as_mut_ptr(),
                                        lens.as_mut_ptr(),
                                    );
                                    if !err.is_null() {
                                        current_block = 4996778623374542017;
                                    } else {
                                        current_block = 12199444798915819164;
                                    }
                                } else {
                                    current_block = 12199444798915819164;
                                }
                                match current_block {
                                    4996778623374542017 => {}
                                    _ => {
                                        let mut port: libc::c_int = 0;
                                        if server.slave_announce_port != 0 {
                                            port = server.slave_announce_port;
                                        } else if server.tls_replication != 0
                                            && server.tls_port != 0
                                        {
                                            port = server.tls_port;
                                        } else {
                                            port = server.port;
                                        }
                                        let mut portstr: sds = sdsfromlonglong(
                                            port as libc::c_longlong,
                                        );
                                        err = sendCommand(
                                            conn,
                                            b"REPLCONF\0" as *const u8 as *const libc::c_char,
                                            b"listening-port\0" as *const u8 as *const libc::c_char,
                                            portstr,
                                            0 as *mut libc::c_void,
                                        );
                                        sdsfree(portstr);
                                        if !err.is_null() {
                                            current_block = 4996778623374542017;
                                        } else {
                                            if !(server.slave_announce_ip).is_null() {
                                                err = sendCommand(
                                                    conn,
                                                    b"REPLCONF\0" as *const u8 as *const libc::c_char,
                                                    b"ip-address\0" as *const u8 as *const libc::c_char,
                                                    server.slave_announce_ip,
                                                    0 as *mut libc::c_void,
                                                );
                                                if !err.is_null() {
                                                    current_block = 4996778623374542017;
                                                } else {
                                                    current_block = 2516253395664191498;
                                                }
                                            } else {
                                                current_block = 2516253395664191498;
                                            }
                                            match current_block {
                                                4996778623374542017 => {}
                                                _ => {
                                                    err = sendCommand(
                                                        conn,
                                                        b"REPLCONF\0" as *const u8 as *const libc::c_char,
                                                        b"capa\0" as *const u8 as *const libc::c_char,
                                                        b"eof\0" as *const u8 as *const libc::c_char,
                                                        b"capa\0" as *const u8 as *const libc::c_char,
                                                        b"psync2\0" as *const u8 as *const libc::c_char,
                                                        0 as *mut libc::c_void,
                                                    );
                                                    if !err.is_null() {
                                                        current_block = 4996778623374542017;
                                                    } else {
                                                        server
                                                            .repl_state = REPL_STATE_RECEIVE_AUTH_REPLY as libc::c_int;
                                                        return;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                if server.repl_state
                                    == REPL_STATE_RECEIVE_AUTH_REPLY as libc::c_int
                                    && (server.masterauth).is_null()
                                {
                                    server
                                        .repl_state = REPL_STATE_RECEIVE_PORT_REPLY as libc::c_int;
                                }
                                if server.repl_state
                                    == REPL_STATE_RECEIVE_AUTH_REPLY as libc::c_int
                                {
                                    err = receiveSynchronousResponse(conn);
                                    if err.is_null() {
                                        current_block = 11760413616134933622;
                                    } else {
                                        if *err.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '-' as i32
                                        {
                                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                                < server.verbosity)
                                            {
                                                _serverLog(
                                                    3 as libc::c_int,
                                                    b"Unable to AUTH to MASTER: %s\0" as *const u8
                                                        as *const libc::c_char,
                                                    err,
                                                );
                                            }
                                            sdsfree(err);
                                        } else {
                                            sdsfree(err);
                                            err = 0 as *mut libc::c_char;
                                            server
                                                .repl_state = REPL_STATE_RECEIVE_PORT_REPLY as libc::c_int;
                                            return;
                                        }
                                        current_block = 2982467534821209940;
                                    }
                                } else if server.repl_state
                                    == REPL_STATE_RECEIVE_PORT_REPLY as libc::c_int
                                {
                                    err = receiveSynchronousResponse(conn);
                                    if err.is_null() {
                                        current_block = 11760413616134933622;
                                    } else {
                                        if *err.offset(0 as libc::c_int as isize) as libc::c_int
                                            == '-' as i32
                                        {
                                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                                < server.verbosity)
                                            {
                                                _serverLog(
                                                    2 as libc::c_int,
                                                    b"(Non critical) Master does not understand REPLCONF listening-port: %s\0"
                                                        as *const u8 as *const libc::c_char,
                                                    err,
                                                );
                                            }
                                        }
                                        sdsfree(err);
                                        server
                                            .repl_state = REPL_STATE_RECEIVE_IP_REPLY as libc::c_int;
                                        return;
                                    }
                                } else {
                                    if server.repl_state
                                        == REPL_STATE_RECEIVE_IP_REPLY as libc::c_int
                                        && (server.slave_announce_ip).is_null()
                                    {
                                        server
                                            .repl_state = REPL_STATE_RECEIVE_CAPA_REPLY as libc::c_int;
                                    }
                                    if server.repl_state
                                        == REPL_STATE_RECEIVE_IP_REPLY as libc::c_int
                                    {
                                        err = receiveSynchronousResponse(conn);
                                        if err.is_null() {
                                            current_block = 11760413616134933622;
                                        } else {
                                            if *err.offset(0 as libc::c_int as isize) as libc::c_int
                                                == '-' as i32
                                            {
                                                if !((2 as libc::c_int & 0xff as libc::c_int)
                                                    < server.verbosity)
                                                {
                                                    _serverLog(
                                                        2 as libc::c_int,
                                                        b"(Non critical) Master does not understand REPLCONF ip-address: %s\0"
                                                            as *const u8 as *const libc::c_char,
                                                        err,
                                                    );
                                                }
                                            }
                                            sdsfree(err);
                                            server
                                                .repl_state = REPL_STATE_RECEIVE_CAPA_REPLY as libc::c_int;
                                            return;
                                        }
                                    } else {
                                        if server.repl_state
                                            == REPL_STATE_RECEIVE_CAPA_REPLY as libc::c_int
                                        {
                                            err = receiveSynchronousResponse(conn);
                                            if err.is_null() {
                                                current_block = 11760413616134933622;
                                            } else {
                                                if *err.offset(0 as libc::c_int as isize) as libc::c_int
                                                    == '-' as i32
                                                {
                                                    if !((2 as libc::c_int & 0xff as libc::c_int)
                                                        < server.verbosity)
                                                    {
                                                        _serverLog(
                                                            2 as libc::c_int,
                                                            b"(Non critical) Master does not understand REPLCONF capa: %s\0"
                                                                as *const u8 as *const libc::c_char,
                                                            err,
                                                        );
                                                    }
                                                }
                                                sdsfree(err);
                                                err = 0 as *mut libc::c_char;
                                                server.repl_state = REPL_STATE_SEND_PSYNC as libc::c_int;
                                                current_block = 13349765058737954042;
                                            }
                                        } else {
                                            current_block = 13349765058737954042;
                                        }
                                        match current_block {
                                            11760413616134933622 => {}
                                            _ => {
                                                if server.repl_state == REPL_STATE_SEND_PSYNC as libc::c_int
                                                {
                                                    if slaveTryPartialResynchronization(conn, 0 as libc::c_int)
                                                        == 0 as libc::c_int
                                                    {
                                                        err = sdsnew(
                                                            b"Write error sending the PSYNC command.\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                        abortFailover(
                                                            b"Write error to failover target\0" as *const u8
                                                                as *const libc::c_char,
                                                        );
                                                    } else {
                                                        server
                                                            .repl_state = REPL_STATE_RECEIVE_PSYNC_REPLY as libc::c_int;
                                                        return;
                                                    }
                                                    current_block = 4996778623374542017;
                                                } else {
                                                    if server.repl_state
                                                        != REPL_STATE_RECEIVE_PSYNC_REPLY as libc::c_int
                                                    {
                                                        if !((3 as libc::c_int & 0xff as libc::c_int)
                                                            < server.verbosity)
                                                        {
                                                            _serverLog(
                                                                3 as libc::c_int,
                                                                b"syncWithMaster(): state machine error, state should be RECEIVE_PSYNC but is %d\0"
                                                                    as *const u8 as *const libc::c_char,
                                                                server.repl_state,
                                                            );
                                                        }
                                                    } else {
                                                        psync_result = slaveTryPartialResynchronization(
                                                            conn,
                                                            1 as libc::c_int,
                                                        );
                                                        if psync_result == 1 as libc::c_int {
                                                            return;
                                                        }
                                                        if server.failover_state
                                                            == FAILOVER_IN_PROGRESS as libc::c_int
                                                        {
                                                            if psync_result == 2 as libc::c_int
                                                                || psync_result == 3 as libc::c_int
                                                            {
                                                                clearFailoverState();
                                                            } else {
                                                                abortFailover(
                                                                    b"Failover target rejected psync request\0" as *const u8
                                                                        as *const libc::c_char,
                                                                );
                                                                return;
                                                            }
                                                        }
                                                        if !(psync_result == 5 as libc::c_int) {
                                                            if psync_result == 2 as libc::c_int {
                                                                if !((2 as libc::c_int & 0xff as libc::c_int)
                                                                    < server.verbosity)
                                                                {
                                                                    _serverLog(
                                                                        2 as libc::c_int,
                                                                        b"MASTER <-> REPLICA sync: Master accepted a Partial Resynchronization.\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                if server.supervised_mode == 2 as libc::c_int {
                                                                    redisCommunicateSystemd(
                                                                        b"STATUS=MASTER <-> REPLICA sync: Partial Resynchronization accepted. Ready to accept connections in read-write mode.\n\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                return;
                                                            }
                                                            if psync_result == 4 as libc::c_int {
                                                                if !((2 as libc::c_int & 0xff as libc::c_int)
                                                                    < server.verbosity)
                                                                {
                                                                    _serverLog(
                                                                        2 as libc::c_int,
                                                                        b"Retrying with SYNC...\0" as *const u8
                                                                            as *const libc::c_char,
                                                                    );
                                                                }
                                                                if connSyncWrite(
                                                                    conn,
                                                                    b"SYNC\r\n\0" as *const u8 as *const libc::c_char
                                                                        as *mut libc::c_char,
                                                                    6 as libc::c_int as ssize_t,
                                                                    (server.repl_syncio_timeout * 1000 as libc::c_int)
                                                                        as libc::c_longlong,
                                                                ) == -(1 as libc::c_int) as libc::c_long
                                                                {
                                                                    if !((3 as libc::c_int & 0xff as libc::c_int)
                                                                        < server.verbosity)
                                                                    {
                                                                        _serverLog(
                                                                            3 as libc::c_int,
                                                                            b"I/O error writing to MASTER: %s\0" as *const u8
                                                                                as *const libc::c_char,
                                                                            connGetLastError(conn),
                                                                        );
                                                                    }
                                                                    current_block = 2982467534821209940;
                                                                } else {
                                                                    current_block = 7370318721998929769;
                                                                }
                                                            } else {
                                                                current_block = 7370318721998929769;
                                                            }
                                                            match current_block {
                                                                2982467534821209940 => {}
                                                                _ => {
                                                                    if useDisklessLoad() == 0 {
                                                                        loop {
                                                                            let fresh18 = maxtries;
                                                                            maxtries = maxtries - 1;
                                                                            if !(fresh18 != 0) {
                                                                                break;
                                                                            }
                                                                            snprintf(
                                                                                tmpfile.as_mut_ptr(),
                                                                                256 as libc::c_int as libc::c_ulong,
                                                                                b"temp-%d.%ld.rdb\0" as *const u8 as *const libc::c_char,
                                                                                server.unixtime,
                                                                                getpid() as libc::c_long,
                                                                            );
                                                                            dfd = open(
                                                                                tmpfile.as_mut_ptr(),
                                                                                0o100 as libc::c_int | 0o1 as libc::c_int
                                                                                    | 0o200 as libc::c_int,
                                                                                0o644 as libc::c_int,
                                                                            );
                                                                            if dfd != -(1 as libc::c_int) {
                                                                                break;
                                                                            }
                                                                            sleep(1 as libc::c_int as libc::c_uint);
                                                                        }
                                                                        if dfd == -(1 as libc::c_int) {
                                                                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                                                                < server.verbosity)
                                                                            {
                                                                                _serverLog(
                                                                                    3 as libc::c_int,
                                                                                    b"Opening the temp file needed for MASTER <-> REPLICA synchronization: %s\0"
                                                                                        as *const u8 as *const libc::c_char,
                                                                                    strerror(*__errno_location()),
                                                                                );
                                                                            }
                                                                            current_block = 2982467534821209940;
                                                                        } else {
                                                                            server
                                                                                .repl_transfer_tmpfile = zstrdup(tmpfile.as_mut_ptr());
                                                                            server.repl_transfer_fd = dfd;
                                                                            current_block = 12099607619007264150;
                                                                        }
                                                                    } else {
                                                                        current_block = 12099607619007264150;
                                                                    }
                                                                    match current_block {
                                                                        2982467534821209940 => {}
                                                                        _ => {
                                                                            if connSetReadHandler(
                                                                                conn,
                                                                                Some(
                                                                                    readSyncBulkPayload
                                                                                        as unsafe extern "C" fn(*mut connection) -> (),
                                                                                ),
                                                                            ) == -(1 as libc::c_int)
                                                                            {
                                                                                let mut conninfo: [libc::c_char; 32] = [0; 32];
                                                                                if !((3 as libc::c_int & 0xff as libc::c_int)
                                                                                    < server.verbosity)
                                                                                {
                                                                                    _serverLog(
                                                                                        3 as libc::c_int,
                                                                                        b"Can't create readable event for SYNC: %s (%s)\0"
                                                                                            as *const u8 as *const libc::c_char,
                                                                                        strerror(*__errno_location()),
                                                                                        connGetInfo(
                                                                                            conn,
                                                                                            conninfo.as_mut_ptr(),
                                                                                            core::mem::size_of::<[libc::c_char; 32]>()
                                                                                                as libc::c_ulong,
                                                                                        ),
                                                                                    );
                                                                                }
                                                                            } else {
                                                                                server.repl_state = REPL_STATE_TRANSFER as libc::c_int;
                                                                                server.repl_transfer_size = -(1 as libc::c_int) as off_t;
                                                                                server.repl_transfer_read = 0 as libc::c_int as off_t;
                                                                                server
                                                                                    .repl_transfer_last_fsync_off = 0 as libc::c_int as off_t;
                                                                                server.repl_transfer_lastio = server.unixtime as time_t;
                                                                                return;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                    current_block = 2982467534821209940;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                    match current_block {
                        4996778623374542017 => {}
                        2982467534821209940 => {}
                        _ => {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"Master did not respond to command during SYNC handshake\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                            current_block = 2982467534821209940;
                        }
                    }
                }
            }
        }
        match current_block {
            2982467534821209940 => {}
            _ => {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Sending command to master in replication handshake: %s\0"
                            as *const u8 as *const libc::c_char,
                        err,
                    );
                }
                sdsfree(err);
            }
        }
    }
    if dfd != -(1 as libc::c_int) {
        close(dfd);
    }
    connClose(conn);
    server.repl_transfer_s = 0 as *mut connection;
    if server.repl_transfer_fd != -(1 as libc::c_int) {
        close(server.repl_transfer_fd);
    }
    if !(server.repl_transfer_tmpfile).is_null() {
        zfree(server.repl_transfer_tmpfile as *mut libc::c_void);
    }
    server.repl_transfer_tmpfile = 0 as *mut libc::c_char;
    server.repl_transfer_fd = -(1 as libc::c_int);
    server.repl_state = REPL_STATE_CONNECT as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn connectWithMaster() -> libc::c_int {
    server
        .repl_transfer_s = if server.tls_replication != 0 {
        connCreateTLS()
    } else {
        connCreateSocket()
    };
    if connConnect(
        server.repl_transfer_s,
        server.masterhost,
        server.masterport,
        server.bind_source_addr,
        Some(syncWithMaster as unsafe extern "C" fn(*mut connection) -> ()),
    ) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unable to connect to MASTER: %s\0" as *const u8 as *const libc::c_char,
                connGetLastError(server.repl_transfer_s),
            );
        }
        connClose(server.repl_transfer_s);
        server.repl_transfer_s = 0 as *mut connection;
        return -(1 as libc::c_int);
    }
    server.repl_transfer_lastio = server.unixtime as time_t;
    server.repl_state = REPL_STATE_CONNECTING as libc::c_int;
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"MASTER <-> REPLICA sync started\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn undoConnectWithMaster() {
    connClose(server.repl_transfer_s);
    server.repl_transfer_s = 0 as *mut connection;
}
#[no_mangle]
pub unsafe extern "C" fn replicationAbortSyncTransfer() {
    if server.repl_state == REPL_STATE_TRANSFER as libc::c_int {} else {
        _serverAssert(
            b"server.repl_state == REPL_STATE_TRANSFER\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            2903 as libc::c_int,
        );
        unreachable!();
    };
    undoConnectWithMaster();
    if server.repl_transfer_fd != -(1 as libc::c_int) {
        close(server.repl_transfer_fd);
        bg_unlink(server.repl_transfer_tmpfile);
        zfree(server.repl_transfer_tmpfile as *mut libc::c_void);
        server.repl_transfer_tmpfile = 0 as *mut libc::c_char;
        server.repl_transfer_fd = -(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cancelReplicationHandshake(
    mut reconnect: libc::c_int,
) -> libc::c_int {
    if server.repl_state == REPL_STATE_TRANSFER as libc::c_int {
        replicationAbortSyncTransfer();
        server.repl_state = REPL_STATE_CONNECT as libc::c_int;
    } else if server.repl_state == REPL_STATE_CONNECTING as libc::c_int
        || slaveIsInHandshakeState() != 0
    {
        undoConnectWithMaster();
        server.repl_state = REPL_STATE_CONNECT as libc::c_int;
    } else {
        return 0 as libc::c_int
    }
    if reconnect == 0 {
        return 1 as libc::c_int;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Reconnecting to MASTER %s:%d after failure\0" as *const u8
                as *const libc::c_char,
            server.masterhost,
            server.masterport,
        );
    }
    connectWithMaster();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicationSetMaster(
    mut ip: *mut libc::c_char,
    mut port: libc::c_int,
) {
    let mut was_master: libc::c_int = (server.masterhost
        == 0 as *mut libc::c_void as *mut libc::c_char) as libc::c_int;
    sdsfree(server.masterhost);
    server.masterhost = 0 as *mut libc::c_char;
    if !(server.master).is_null() {
        freeClient(server.master);
    }
    disconnectAllBlockedClients();
    server.masterhost = sdsnew(ip);
    server.masterport = port;
    setOOMScoreAdj(-(1 as libc::c_int));
    cancelReplicationHandshake(0 as libc::c_int);
    if was_master != 0 {
        replicationDiscardCachedMaster();
        replicationCacheMasterUsingMyself();
    }
    moduleFireServerEvent(
        0 as libc::c_int as uint64_t,
        1 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
        moduleFireServerEvent(
            7 as libc::c_int as uint64_t,
            1 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    server.repl_state = REPL_STATE_CONNECT as libc::c_int;
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Connecting to MASTER %s:%d\0" as *const u8 as *const libc::c_char,
            server.masterhost,
            server.masterport,
        );
    }
    connectWithMaster();
}
#[no_mangle]
pub unsafe extern "C" fn replicationUnsetMaster() {
    if (server.masterhost).is_null() {
        return;
    }
    if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
        moduleFireServerEvent(
            7 as libc::c_int as uint64_t,
            1 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    sdsfree(server.masterhost);
    server.masterhost = 0 as *mut libc::c_char;
    if !(server.master).is_null() {
        freeClient(server.master);
    }
    replicationDiscardCachedMaster();
    cancelReplicationHandshake(0 as libc::c_int);
    shiftReplicationId();
    disconnectSlaves();
    server.repl_state = REPL_STATE_NONE as libc::c_int;
    server.slaveseldb = -(1 as libc::c_int);
    setOOMScoreAdj(-(1 as libc::c_int));
    server.repl_no_slaves_since = server.unixtime as time_t;
    server.repl_down_since = 0 as libc::c_int as time_t;
    moduleFireServerEvent(
        0 as libc::c_int as uint64_t,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
    if server.aof_enabled != 0 && server.aof_state == 0 as libc::c_int {
        restartAOFAfterSYNC();
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicationHandleMasterDisconnection() {
    if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
        moduleFireServerEvent(
            7 as libc::c_int as uint64_t,
            1 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    server.master = 0 as *mut client;
    server.repl_state = REPL_STATE_CONNECT as libc::c_int;
    server.repl_down_since = server.unixtime as time_t;
    if !(server.masterhost).is_null() {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Reconnecting to MASTER %s:%d\0" as *const u8 as *const libc::c_char,
                server.masterhost,
                server.masterport,
            );
        }
        connectWithMaster();
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicaofCommand(mut c: *mut client) {
    if server.cluster_enabled != 0 {
        addReplyError(
            c,
            b"REPLICAOF not allowed in cluster mode.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if server.failover_state != NO_FAILOVER as libc::c_int {
        addReplyError(
            c,
            b"REPLICAOF not allowed while failing over.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
    ) == 0
        && strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"one\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        if !(server.masterhost).is_null() {
            replicationUnsetMaster();
            let mut client: sds = catClientInfoString(sdsempty(), c);
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"MASTER MODE enabled (user request from '%s')\0" as *const u8
                        as *const libc::c_char,
                    client,
                );
            }
            sdsfree(client);
        }
    } else {
        let mut port: libc::c_long = 0;
        if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
            addReplyError(
                c,
                b"Command is not valid when client is a replica.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if getRangeLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            0 as libc::c_int as libc::c_long,
            65535 as libc::c_int as libc::c_long,
            &mut port,
            b"Invalid master port\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if !(server.masterhost).is_null()
            && strcasecmp(
                server.masterhost,
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
            ) == 0 && server.masterport as libc::c_long == port
        {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"REPLICAOF would result into synchronization with the master we are already connected with. No operation performed.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            addReplySds(
                c,
                sdsnew(
                    b"+OK Already connected to specified master\r\n\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            return;
        }
        replicationSetMaster(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char,
            port as libc::c_int,
        );
        let mut client_0: sds = catClientInfoString(sdsempty(), c);
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"REPLICAOF %s:%d enabled (user request from '%s')\0" as *const u8
                    as *const libc::c_char,
                server.masterhost,
                server.masterport,
                client_0,
            );
        }
        sdsfree(client_0);
    }
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn roleCommand(mut c: *mut client) {
    if server.sentinel_mode != 0 {
        sentinelRoleCommand(c);
        return;
    }
    if (server.masterhost).is_null() {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut mbcount: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut slaves: libc::c_int = 0 as libc::c_int;
        addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
        addReplyBulkCBuffer(
            c,
            b"master\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            6 as libc::c_int as size_t,
        );
        addReplyLongLong(c, server.master_repl_offset);
        mbcount = addReplyDeferredLen(c);
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut slave: *mut client = (*ln).value as *mut client;
            let mut ip: [libc::c_char; 46] = [0; 46];
            let mut slaveaddr: *mut libc::c_char = (*slave).slave_addr;
            if slaveaddr.is_null() {
                if connPeerToString(
                    (*slave).conn,
                    ip.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                    0 as *mut libc::c_int,
                ) == -(1 as libc::c_int)
                {
                    continue;
                }
                slaveaddr = ip.as_mut_ptr();
            }
            if (*slave).replstate != 9 as libc::c_int {
                continue;
            }
            addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
            addReplyBulkCString(c, slaveaddr);
            addReplyBulkLongLong(c, (*slave).slave_listening_port as libc::c_longlong);
            addReplyBulkLongLong(c, (*slave).repl_ack_off);
            slaves += 1;
        }
        setDeferredArrayLen(c, mbcount, slaves as libc::c_long);
    } else {
        let mut slavestate: *mut libc::c_char = 0 as *mut libc::c_char;
        addReplyArrayLen(c, 5 as libc::c_int as libc::c_long);
        addReplyBulkCBuffer(
            c,
            b"slave\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as size_t,
        );
        addReplyBulkCString(c, server.masterhost);
        addReplyLongLong(c, server.masterport as libc::c_longlong);
        if slaveIsInHandshakeState() != 0 {
            slavestate = b"handshake\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            match server.repl_state {
                0 => {
                    slavestate = b"none\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                1 => {
                    slavestate = b"connect\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                2 => {
                    slavestate = b"connecting\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                11 => {
                    slavestate = b"sync\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                12 => {
                    slavestate = b"connected\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
                _ => {
                    slavestate = b"unknown\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char;
                }
            }
        }
        addReplyBulkCString(c, slavestate);
        addReplyLongLong(
            c,
            if !(server.master).is_null() {
                (*server.master).reploff
            } else {
                -(1 as libc::c_int) as libc::c_longlong
            },
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn replicationSendAck() {
    let mut c: *mut client = server.master;
    if !c.is_null() {
        (*c).flags |= ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong;
        addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, b"REPLCONF\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, b"ACK\0" as *const u8 as *const libc::c_char);
        addReplyBulkLongLong(c, (*c).reploff);
        (*c).flags &= !((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn replicationCacheMaster(mut c: *mut client) {
    if !(server.master).is_null() && (server.cached_master).is_null() {} else {
        _serverAssert(
            b"server.master != NULL && server.cached_master == NULL\0" as *const u8
                as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            3239 as libc::c_int,
        );
        unreachable!();
    };
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Caching the disconnected master state.\0" as *const u8
                as *const libc::c_char,
        );
    }
    unlinkClient(c);
    sdsclear((*server.master).querybuf);
    (*server.master).qb_pos = 0 as libc::c_int as size_t;
    (*server.master).repl_applied = 0 as libc::c_int as libc::c_longlong;
    (*server.master).read_reploff = (*server.master).reploff;
    if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
        discardTransaction(c);
    }
    listEmpty((*c).reply);
    (*c).sentlen = 0 as libc::c_int as size_t;
    (*c).reply_bytes = 0 as libc::c_int as libc::c_ulonglong;
    (*c).bufpos = 0 as libc::c_int;
    resetClient(c);
    server.cached_master = server.master;
    if !((*c).peerid).is_null() {
        sdsfree((*c).peerid);
        (*c).peerid = 0 as sds;
    }
    if !((*c).sockname).is_null() {
        sdsfree((*c).sockname);
        (*c).sockname = 0 as sds;
    }
    replicationHandleMasterDisconnection();
}
#[no_mangle]
pub unsafe extern "C" fn replicationCacheMasterUsingMyself() {
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Before turning into a replica, using my own master parameters to synthesize a cached master: I may be able to synchronize with the new master with just a partial transfer.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    server.master_initial_offset = server.master_repl_offset;
    replicationCreateMasterClient(0 as *mut connection, -(1 as libc::c_int));
    memcpy(
        ((*server.master).replid).as_mut_ptr() as *mut libc::c_void,
        (server.replid).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
    );
    unlinkClient(server.master);
    server.cached_master = server.master;
    server.master = 0 as *mut client;
}
#[no_mangle]
pub unsafe extern "C" fn replicationDiscardCachedMaster() {
    if (server.cached_master).is_null() {
        return;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Discarding previously cached master state.\0" as *const u8
                as *const libc::c_char,
        );
    }
    (*server.cached_master).flags
        &= !((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
    freeClient(server.cached_master);
    server.cached_master = 0 as *mut client;
}
#[no_mangle]
pub unsafe extern "C" fn replicationResurrectCachedMaster(mut conn: *mut connection) {
    server.master = server.cached_master;
    server.cached_master = 0 as *mut client;
    (*server.master).conn = conn;
    connSetPrivateData((*server.master).conn, server.master as *mut libc::c_void);
    (*server.master).flags
        &= !((1 as libc::c_int) << 6 as libc::c_int
            | (1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong;
    (*server.master).authenticated = 1 as libc::c_int;
    (*server.master).lastinteraction = server.unixtime as time_t;
    server.repl_state = REPL_STATE_CONNECTED as libc::c_int;
    server.repl_down_since = 0 as libc::c_int as time_t;
    moduleFireServerEvent(
        7 as libc::c_int as uint64_t,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
    linkClient(server.master);
    if connSetReadHandler(
        (*server.master).conn,
        Some(readQueryFromClient as unsafe extern "C" fn(*mut connection) -> ()),
    ) != 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error resurrecting the cached master, impossible to add the readable handler: %s\0"
                    as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        freeClientAsync(server.master);
    }
    if clientHasPendingReplies(server.master) != 0 {
        if connSetWriteHandler(
            (*server.master).conn,
            Some(sendReplyToClient as unsafe extern "C" fn(*mut connection) -> ()),
        ) != 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Error resurrecting the cached master, impossible to add the writable handler: %s\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            freeClientAsync(server.master);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn refreshGoodSlavesCount() {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut good: libc::c_int = 0 as libc::c_int;
    if server.repl_min_slaves_to_write == 0 || server.repl_min_slaves_max_lag == 0 {
        return;
    }
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        let mut lag: time_t = (server.unixtime as libc::c_longlong
            - (*slave).repl_ack_time) as time_t;
        if (*slave).replstate == 9 as libc::c_int
            && lag <= server.repl_min_slaves_max_lag as libc::c_long
        {
            good += 1;
        }
    }
    server.repl_good_slaves_count = good;
}
#[no_mangle]
pub unsafe extern "C" fn checkGoodReplicasStatus() -> libc::c_int {
    return (!(server.masterhost).is_null() || server.repl_min_slaves_max_lag == 0
        || server.repl_min_slaves_to_write == 0
        || server.repl_good_slaves_count >= server.repl_min_slaves_to_write)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicationRequestAckFromSlaves() {
    server.get_ack_from_slaves = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicationCountAcksByOffset(
    mut offset: libc::c_longlong,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut count: libc::c_int = 0 as libc::c_int;
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        if (*slave).replstate != 9 as libc::c_int {
            continue;
        }
        if (*slave).repl_ack_off >= offset {
            count += 1;
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn waitCommand(mut c: *mut client) {
    let mut timeout: mstime_t = 0;
    let mut numreplicas: libc::c_long = 0;
    let mut ackreplicas: libc::c_long = 0;
    let mut offset: libc::c_longlong = (*c).woff;
    if !(server.masterhost).is_null() {
        addReplyError(
            c,
            b"WAIT cannot be used with replica instances. Please also note that since Redis 4.0 if a replica is configured to be writable (which is not the default) writes to replicas are just local and are not propagated.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        &mut numreplicas,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return;
    }
    if getTimeoutFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut timeout,
        1 as libc::c_int,
    ) != 0 as libc::c_int
    {
        return;
    }
    ackreplicas = replicationCountAcksByOffset((*c).woff) as libc::c_long;
    if ackreplicas >= numreplicas
        || (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
    {
        addReplyLongLong(c, ackreplicas as libc::c_longlong);
        return;
    }
    (*c).bpop.timeout = timeout;
    (*c).bpop.reploffset = offset;
    (*c).bpop.numreplicas = numreplicas as libc::c_int;
    listAddNodeHead(server.clients_waiting_acks, c as *mut libc::c_void);
    blockClient(c, 2 as libc::c_int);
    replicationRequestAckFromSlaves();
}
#[no_mangle]
pub unsafe extern "C" fn unblockClientWaitingReplicas(mut c: *mut client) {
    let mut ln: *mut listNode = listSearchKey(
        server.clients_waiting_acks,
        c as *mut libc::c_void,
    );
    if !ln.is_null() {} else {
        _serverAssert(
            b"ln != NULL\0" as *const u8 as *const libc::c_char,
            b"replication.c\0" as *const u8 as *const libc::c_char,
            3492 as libc::c_int,
        );
        unreachable!();
    };
    listDelNode(server.clients_waiting_acks, ln);
}
#[no_mangle]
pub unsafe extern "C" fn processClientsWaitingReplicas() {
    let mut last_offset: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut last_numreplicas: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients_waiting_acks, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        if last_offset != 0 && last_offset >= (*c).bpop.reploffset
            && last_numreplicas >= (*c).bpop.numreplicas
        {
            unblockClient(c);
            addReplyLongLong(c, last_numreplicas as libc::c_longlong);
        } else {
            let mut numreplicas: libc::c_int = replicationCountAcksByOffset(
                (*c).bpop.reploffset,
            );
            if numreplicas >= (*c).bpop.numreplicas {
                last_offset = (*c).bpop.reploffset;
                last_numreplicas = numreplicas;
                unblockClient(c);
                addReplyLongLong(c, numreplicas as libc::c_longlong);
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn replicationGetSlaveOffset() -> libc::c_longlong {
    let mut offset: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if !(server.masterhost).is_null() {
        if !(server.master).is_null() {
            offset = (*server.master).reploff;
        } else if !(server.cached_master).is_null() {
            offset = (*server.cached_master).reploff;
        }
    }
    if offset < 0 as libc::c_int as libc::c_longlong {
        offset = 0 as libc::c_int as libc::c_longlong;
    }
    return offset;
}
#[no_mangle]
pub unsafe extern "C" fn replicationCron() {
    static mut replication_cron_loops: libc::c_longlong = 0 as libc::c_int
        as libc::c_longlong;
    updateFailoverStatus();
    if !(server.masterhost).is_null()
        && (server.repl_state == REPL_STATE_CONNECTING as libc::c_int
            || slaveIsInHandshakeState() != 0)
        && time(0 as *mut time_t) - server.repl_transfer_lastio
            > server.repl_timeout as libc::c_long
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Timeout connecting to the MASTER...\0" as *const u8
                    as *const libc::c_char,
            );
        }
        cancelReplicationHandshake(1 as libc::c_int);
    }
    if !(server.masterhost).is_null()
        && server.repl_state == REPL_STATE_TRANSFER as libc::c_int
        && time(0 as *mut time_t) - server.repl_transfer_lastio
            > server.repl_timeout as libc::c_long
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Timeout receiving bulk data from MASTER... If the problem persists try to set the 'repl-timeout' parameter in redis.conf to a larger value.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        cancelReplicationHandshake(1 as libc::c_int);
    }
    if !(server.masterhost).is_null()
        && server.repl_state == REPL_STATE_CONNECTED as libc::c_int
        && time(0 as *mut time_t) - (*server.master).lastinteraction
            > server.repl_timeout as libc::c_long
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"MASTER timeout: no data nor PING received...\0" as *const u8
                    as *const libc::c_char,
            );
        }
        freeClient(server.master);
    }
    if server.repl_state == REPL_STATE_CONNECT as libc::c_int {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Connecting to MASTER %s:%d\0" as *const u8 as *const libc::c_char,
                server.masterhost,
                server.masterport,
            );
        }
        connectWithMaster();
    }
    if !(server.masterhost).is_null() && !(server.master).is_null()
        && (*server.master).flags
            & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong == 0
    {
        replicationSendAck();
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut ping_argv: [*mut robj; 1] = [0 as *mut robj; 1];
    if replication_cron_loops % server.repl_ping_slave_period as libc::c_longlong
        == 0 as libc::c_int as libc::c_longlong && (*server.slaves).len != 0
    {
        let mut manual_failover_in_progress: libc::c_int = ((server.cluster_enabled != 0
            && (*server.cluster).mf_end != 0 || server.failover_end_time != 0)
            && checkClientPauseTimeoutAndReturnIfPaused() != 0) as libc::c_int;
        if manual_failover_in_progress == 0 {
            ping_argv[0 as libc::c_int as usize] = shared.ping;
            replicationFeedSlaves(
                server.slaves,
                server.slaveseldb,
                ping_argv.as_mut_ptr(),
                1 as libc::c_int,
            );
        }
    }
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut slave: *mut client = (*ln).value as *mut client;
        let mut is_presync: libc::c_int = ((*slave).replstate == 6 as libc::c_int
            || (*slave).replstate == 7 as libc::c_int
                && server.rdb_child_type != 2 as libc::c_int) as libc::c_int;
        if is_presync != 0 {
            connWrite(
                (*slave).conn,
                b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
    }
    if (*server.slaves).len != 0 {
        let mut li_0: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln_0: *mut listNode = 0 as *mut listNode;
        listRewind(server.slaves, &mut li_0);
        loop {
            ln_0 = listNext(&mut li_0);
            if ln_0.is_null() {
                break;
            }
            let mut slave_0: *mut client = (*ln_0).value as *mut client;
            if (*slave_0).replstate == 9 as libc::c_int {
                if (*slave_0).flags
                    & ((1 as libc::c_int) << 16 as libc::c_int) as libc::c_ulong != 0
                {
                    continue;
                }
                if server.unixtime as libc::c_longlong - (*slave_0).repl_ack_time
                    > server.repl_timeout as libc::c_longlong
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Disconnecting timedout replica (streaming sync): %s\0"
                                as *const u8 as *const libc::c_char,
                            replicationGetSlaveName(slave_0),
                        );
                    }
                    freeClient(slave_0);
                    continue;
                }
            }
            if !((*slave_0).replstate == 7 as libc::c_int
                && server.rdb_child_type == 2 as libc::c_int)
            {
                continue;
            }
            if !((*slave_0).repl_last_partial_write
                != 0 as libc::c_int as libc::c_longlong
                && server.unixtime as libc::c_longlong
                    - (*slave_0).repl_last_partial_write
                    > server.repl_timeout as libc::c_longlong)
            {
                continue;
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Disconnecting timedout replica (full sync): %s\0" as *const u8
                        as *const libc::c_char,
                    replicationGetSlaveName(slave_0),
                );
            }
            freeClient(slave_0);
        }
    }
    if (*server.slaves).len == 0 as libc::c_int as libc::c_ulong
        && server.repl_backlog_time_limit != 0 && !(server.repl_backlog).is_null()
        && (server.masterhost).is_null()
    {
        let mut idle: time_t = server.unixtime as libc::c_long
            - server.repl_no_slaves_since;
        if idle > server.repl_backlog_time_limit {
            changeReplicationId();
            clearReplicationId2();
            freeReplicationBacklog();
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Replication backlog freed after %d seconds without connected replicas.\0"
                        as *const u8 as *const libc::c_char,
                    server.repl_backlog_time_limit as libc::c_int,
                );
            }
        }
    }
    replicationStartPendingFork();
    removeRDBUsedToSyncReplicas();
    if (*server.repl_buffer_blocks).len > 0 as libc::c_int as libc::c_ulong {
        let mut o: *mut replBufBlock = (*(*server.repl_buffer_blocks).head).value
            as *mut replBufBlock;
        if (*o).refcount > 0 as libc::c_int
            && (*o).refcount <= (*server.slaves).len as libc::c_int + 1 as libc::c_int
        {} else {
            _serverAssert(
                b"o->refcount > 0 && o->refcount <= (int)listLength(server.slaves)+1\0"
                    as *const u8 as *const libc::c_char,
                b"replication.c\0" as *const u8 as *const libc::c_char,
                3743 as libc::c_int,
            );
            unreachable!();
        };
    }
    refreshGoodSlavesCount();
    replication_cron_loops += 1;
}
#[no_mangle]
pub unsafe extern "C" fn shouldStartChildReplication(
    mut mincapa_out: *mut libc::c_int,
    mut req_out: *mut libc::c_int,
) -> libc::c_int {
    if hasActiveChildProcess() == 0 {
        let mut idle: time_t = 0;
        let mut max_idle: time_t = 0 as libc::c_int as time_t;
        let mut slaves_waiting: libc::c_int = 0 as libc::c_int;
        let mut mincapa: libc::c_int = 0;
        let mut req: libc::c_int = 0;
        let mut first: libc::c_int = 1 as libc::c_int;
        let mut ln: *mut listNode = 0 as *mut listNode;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut slave: *mut client = (*ln).value as *mut client;
            if !((*slave).replstate == 6 as libc::c_int) {
                continue;
            }
            if first != 0 {
                req = (*slave).slave_req;
            } else if req != (*slave).slave_req {
                continue;
            }
            idle = server.unixtime as libc::c_long - (*slave).lastinteraction;
            if idle > max_idle {
                max_idle = idle;
            }
            slaves_waiting += 1;
            mincapa = if first != 0 {
                (*slave).slave_capa
            } else {
                mincapa & (*slave).slave_capa
            };
            first = 0 as libc::c_int;
        }
        if slaves_waiting != 0
            && (server.repl_diskless_sync == 0
                || server.repl_diskless_sync_max_replicas > 0 as libc::c_int
                    && slaves_waiting >= server.repl_diskless_sync_max_replicas
                || max_idle >= server.repl_diskless_sync_delay as libc::c_long)
        {
            if !mincapa_out.is_null() {
                *mincapa_out = mincapa;
            }
            if !req_out.is_null() {
                *req_out = req;
            }
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn replicationStartPendingFork() {
    let mut mincapa: libc::c_int = -(1 as libc::c_int);
    let mut req: libc::c_int = -(1 as libc::c_int);
    if shouldStartChildReplication(&mut mincapa, &mut req) != 0 {
        startBgsaveForReplication(mincapa, req);
    }
}
unsafe extern "C" fn findReplica(
    mut host: *mut libc::c_char,
    mut port: libc::c_int,
) -> *mut client {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut replica: *mut client = 0 as *mut client;
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        replica = (*ln).value as *mut client;
        let mut ip: [libc::c_char; 46] = [0; 46];
        let mut replicaip: *mut libc::c_char = (*replica).slave_addr;
        if replicaip.is_null() {
            if connPeerToString(
                (*replica).conn,
                ip.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                0 as *mut libc::c_int,
            ) == -(1 as libc::c_int)
            {
                continue;
            }
            replicaip = ip.as_mut_ptr();
        }
        if strcasecmp(host, replicaip) == 0 && port == (*replica).slave_listening_port {
            return replica;
        }
    }
    return 0 as *mut client;
}
#[no_mangle]
pub unsafe extern "C" fn getFailoverStateString() -> *const libc::c_char {
    match server.failover_state {
        0 => return b"no-failover\0" as *const u8 as *const libc::c_char,
        2 => return b"failover-in-progress\0" as *const u8 as *const libc::c_char,
        1 => return b"waiting-for-sync\0" as *const u8 as *const libc::c_char,
        _ => return b"unknown\0" as *const u8 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn clearFailoverState() {
    server.failover_end_time = 0 as libc::c_int as mstime_t;
    server.force_failover = 0 as libc::c_int;
    zfree(server.target_replica_host as *mut libc::c_void);
    server.target_replica_host = 0 as *mut libc::c_char;
    server.target_replica_port = 0 as libc::c_int;
    server.failover_state = NO_FAILOVER as libc::c_int;
    unpauseClients(PAUSE_DURING_FAILOVER);
}
#[no_mangle]
pub unsafe extern "C" fn abortFailover(mut err: *const libc::c_char) {
    if server.failover_state == NO_FAILOVER as libc::c_int {
        return;
    }
    if !(server.target_replica_host).is_null() {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"FAILOVER to %s:%d aborted: %s\0" as *const u8 as *const libc::c_char,
                server.target_replica_host,
                server.target_replica_port,
                err,
            );
        }
    } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"FAILOVER to any replica aborted: %s\0" as *const u8 as *const libc::c_char,
            err,
        );
    }
    if server.failover_state == FAILOVER_IN_PROGRESS as libc::c_int {
        replicationUnsetMaster();
    }
    clearFailoverState();
}
#[no_mangle]
pub unsafe extern "C" fn failoverCommand(mut c: *mut client) {
    if server.cluster_enabled != 0 {
        addReplyError(
            c,
            b"FAILOVER not allowed in cluster mode. Use CLUSTER FAILOVER command instead.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"abort\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        if server.failover_state == NO_FAILOVER as libc::c_int {
            addReplyError(
                c,
                b"No failover in progress.\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        abortFailover(
            b"Failover manually aborted\0" as *const u8 as *const libc::c_char,
        );
        addReply(c, shared.ok);
        return;
    }
    let mut timeout_in_ms: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut force_flag: libc::c_int = 0 as libc::c_int;
    let mut port: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut host: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < (*c).argc {
        if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"timeout\0" as *const u8 as *const libc::c_char,
        ) == 0 && (j + 1 as libc::c_int) < (*c).argc
            && timeout_in_ms == 0 as libc::c_int as libc::c_long
        {
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut timeout_in_ms,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if timeout_in_ms <= 0 as libc::c_int as libc::c_long {
                addReplyError(
                    c,
                    b"FAILOVER timeout must be greater than 0\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            j += 1;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"to\0" as *const u8 as *const libc::c_char,
        ) == 0 && (j + 2 as libc::c_int) < (*c).argc && host.is_null()
        {
            if getLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 2 as libc::c_int) as isize),
                &mut port,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            host = (**((*c).argv).offset((j + 1 as libc::c_int) as isize)).ptr
                as *mut libc::c_char;
            j += 2 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"force\0" as *const u8 as *const libc::c_char,
        ) == 0 && force_flag == 0
        {
            force_flag = 1 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if server.failover_state != NO_FAILOVER as libc::c_int {
        addReplyError(
            c,
            b"FAILOVER already in progress.\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !(server.masterhost).is_null() {
        addReplyError(
            c,
            b"FAILOVER is not valid when server is a replica.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*server.slaves).len == 0 as libc::c_int as libc::c_ulong {
        addReplyError(
            c,
            b"FAILOVER requires connected replicas.\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if force_flag != 0 && (timeout_in_ms == 0 || host.is_null()) {
        addReplyError(
            c,
            b"FAILOVER with force option requires both a timeout and target HOST and IP.\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if !host.is_null() {
        let mut replica: *mut client = findReplica(host, port as libc::c_int);
        if replica.is_null() {
            addReplyError(
                c,
                b"FAILOVER target HOST and PORT is not a replica.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if (*replica).replstate != 9 as libc::c_int {
            addReplyError(
                c,
                b"FAILOVER target replica is not online.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        server.target_replica_host = zstrdup(host);
        server.target_replica_port = port as libc::c_int;
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"FAILOVER requested to %s:%ld.\0" as *const u8 as *const libc::c_char,
                host,
                port,
            );
        }
    } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"FAILOVER requested to any replica.\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut now: mstime_t = mstime();
    if timeout_in_ms != 0 {
        server.failover_end_time = now + timeout_in_ms as libc::c_longlong;
    }
    server.force_failover = force_flag;
    server.failover_state = FAILOVER_WAIT_FOR_SYNC as libc::c_int;
    pauseClients(
        PAUSE_DURING_FAILOVER,
        9223372036854775807 as libc::c_longlong,
        CLIENT_PAUSE_WRITE,
    );
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn updateFailoverStatus() {
    if server.failover_state != FAILOVER_WAIT_FOR_SYNC as libc::c_int {
        return;
    }
    let mut now: mstime_t = server.mstime;
    if server.failover_end_time != 0 && server.failover_end_time <= now {
        if server.force_failover != 0 {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"FAILOVER to %s:%d time out exceeded, failing over.\0" as *const u8
                        as *const libc::c_char,
                    server.target_replica_host,
                    server.target_replica_port,
                );
            }
            server.failover_state = FAILOVER_IN_PROGRESS as libc::c_int;
            replicationSetMaster(server.target_replica_host, server.target_replica_port);
            return;
        } else {
            abortFailover(
                b"Replica never caught up before timeout\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    let mut replica: *mut client = 0 as *mut client;
    if !(server.target_replica_host).is_null() {
        replica = findReplica(server.target_replica_host, server.target_replica_port);
    } else {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(server.slaves, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            replica = (*ln).value as *mut client;
            if !((*replica).repl_ack_off == server.master_repl_offset) {
                continue;
            }
            let mut ip: [libc::c_char; 46] = [0; 46];
            let mut replicaaddr: *mut libc::c_char = (*replica).slave_addr;
            if replicaaddr.is_null() {
                if connPeerToString(
                    (*replica).conn,
                    ip.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                    0 as *mut libc::c_int,
                ) == -(1 as libc::c_int)
                {
                    continue;
                }
                replicaaddr = ip.as_mut_ptr();
            }
            server.target_replica_host = zstrdup(replicaaddr);
            server.target_replica_port = (*replica).slave_listening_port;
            break;
        }
    }
    if !replica.is_null() && (*replica).repl_ack_off == server.master_repl_offset {
        server.failover_state = FAILOVER_IN_PROGRESS as libc::c_int;
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Failover target %s:%d is synced, failing over.\0" as *const u8
                    as *const libc::c_char,
                server.target_replica_host,
                server.target_replica_port,
            );
        }
        replicationSetMaster(server.target_replica_host, server.target_replica_port);
    }
}

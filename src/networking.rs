extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use std::convert::TryInto;
extern "C" {
    pub type RedisModuleCommand;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut SDS_NOINIT: *const libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscatvprintf(s: sds, fmt: *const libc::c_char, ap: core::ffi::VaList) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdsclear(s: sds);
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdstoupper(s: sds);
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsmapchars(
        s: sds,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> sds;
    fn sdsMakeRoomFor(s: sds, addlen: size_t) -> sds;
    fn sdsMakeRoomForNonGreedy(s: sds, addlen: size_t) -> sds;
    fn sdsIncrLen(s: sds, incr: ssize_t);
    fn sdsAllocPtr(s: sds) -> *mut libc::c_void;
    fn __errno_location() -> *mut libc::c_int;
    fn connCreateAcceptedSocket(fd: libc::c_int) -> *mut connection;
    fn connCreateAcceptedTLS(
        fd: libc::c_int,
        require_auth: libc::c_int,
    ) -> *mut connection;
    fn connSetPrivateData(conn: *mut connection, data: *mut libc::c_void);
    fn connGetPrivateData(conn: *mut connection) -> *mut libc::c_void;
    fn connGetState(conn: *mut connection) -> libc::c_int;
    fn connHasWriteHandler(conn: *mut connection) -> libc::c_int;
    fn connHasReadHandler(conn: *mut connection) -> libc::c_int;
    fn connEnableTcpNoDelay(conn: *mut connection) -> libc::c_int;
    fn connKeepAlive(conn: *mut connection, interval: libc::c_int) -> libc::c_int;
    fn connPeerToString(
        conn: *mut connection,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn connFormatFdAddr(
        conn: *mut connection,
        buf: *mut libc::c_char,
        buf_len: size_t,
        fd_to_str_type: libc::c_int,
    ) -> libc::c_int;
    fn connGetInfo(
        conn: *mut connection,
        buf: *mut libc::c_char,
        buf_len: size_t,
    ) -> *const libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option::<
            unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
        >,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const libc::c_char,
    ) -> libc::c_int;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn llabs(_: libc::c_longlong) -> libc::c_longlong;
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
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn aeProcessEvents(eventLoop: *mut aeEventLoop, flags: libc::c_int) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictRelease(d: *mut dict);
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listEmpty(list: *mut list);
    fn listAddNodeHead(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn listJoin(l: *mut list, o: *mut list);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zmalloc_usable(size: size_t, usable: *mut size_t) -> *mut libc::c_void;
    fn zmalloc_used_memory() -> size_t;
    fn anetTcpAccept(
        err: *mut libc::c_char,
        serversock: libc::c_int,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn anetUnixAccept(err: *mut libc::c_char, serversock: libc::c_int) -> libc::c_int;
    fn digits10(v: uint64_t) -> uint32_t;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn ld2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: f64,
        mode: ld2string_mode,
    ) -> libc::c_int;
    static mut raxNotFound: *mut libc::c_void;
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
    fn raxFind(rax: *mut rax, s: *mut libc::c_uchar, len: size_t) -> *mut libc::c_void;
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn intrev64(v: uint64_t) -> uint64_t;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut objectKeyPointerValueDictType: dictType;
    static mut objectKeyHeapPointerValueDictType: dictType;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn moduleBlockedClientMayTimeout(c: *mut client) -> libc::c_int;
    fn moduleNotifyUserChanged(c: *mut client);
    fn redisSetCpuAffinity(cpulist: *const libc::c_char);
    fn initClientMultiState(c: *mut client);
    fn equalStringObjects(a: *mut robj, b: *mut robj) -> libc::c_int;
    fn decrRefCountVoid(o: *mut libc::c_void);
    static mut DefaultUser: *mut user;
    fn freeClientMultiState(c: *mut client);
    fn decrRefCount(o: *mut robj);
    fn replicationHandleMasterDisconnection();
    fn refreshGoodSlavesCount();
    fn killRDBChild();
    fn disableTracking(c: *mut client);
    fn rdbPipeWriteHandlerConnRemoved(conn: *mut connection);
    fn freeReplicaReferencedReplBuffer(replica: *mut client);
    fn unwatchAllKeys(c: *mut client);
    fn replicationGetSlaveName(c: *mut client) -> *mut libc::c_char;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn replicationCacheMaster(c: *mut client);
    fn updateClientMemUsageAndBucket(c: *mut client) -> libc::c_int;
    fn replicationFeedStreamFromMasterStream(buf: *mut libc::c_char, buflen: size_t);
    fn showLatestBacklog();
    fn multiStateMemOverhead(c: *mut client) -> size_t;
    fn enableTracking(
        c: *mut client,
        redirect_to: uint64_t,
        options: uint64_t,
        prefix: *mut *mut robj,
        numprefix: size_t,
    );
    fn checkPrefixCollisionsOrReply(
        c: *mut client,
        prefix: *mut *mut robj,
        numprefix: size_t,
    ) -> libc::c_int;
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn stringObjectLen(o: *mut robj) -> size_t;
    fn incrRefCount(o: *mut robj);
    fn ACLGetUserByName(name: *const libc::c_char, namelen: size_t) -> *mut user;
    fn getRangeLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        min: libc::c_long,
        max: libc::c_long,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn removeClientFromMemUsageBucket(c: *mut client, allow_eviction: libc::c_int);
    fn processCommand(c: *mut client) -> libc::c_int;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn discardTransaction(c: *mut client);
    fn incrementalTrimReplicationBacklog(blocks: size_t);
    fn createStringObjectFromLongDouble(
        value: f64,
        humanfriendly: libc::c_int,
    ) -> *mut robj;
    fn addReplyLoadedModules(c: *mut client);
    fn lookupCommandOrOriginal(
        argv: *mut *mut robj,
        argc: libc::c_int,
    ) -> *mut redisCommand;
    fn whileBlockedCron();
    fn ACLAuthenticateUser(
        c: *mut client,
        username: *mut robj,
        password: *mut robj,
    ) -> libc::c_int;
    fn updateCachedTime(update_daylight_info: libc::c_int);
    fn isInsideYieldingLongCommand() -> libc::c_int;
    fn askingCommand(c: *mut client);
    fn replyToBlockedClientTimedOut(c: *mut client);
    fn updateStatsOnUnblock(
        c: *mut client,
        blocked_us: libc::c_long,
        reply_us: libc::c_long,
        had_errors: libc::c_int,
    );
    fn getTimeoutFromObjectOrReply(
        c: *mut client,
        object: *mut robj,
        timeout: *mut mstime_t,
        unit: libc::c_int,
    ) -> libc::c_int;
    fn pubsubMemOverhead(c: *mut client) -> size_t;
    fn incrementErrorCount(fullerr: *const libc::c_char, namelen: size_t);
    fn unblockClient(c: *mut client);
    fn pubsubUnsubscribeAllChannels(c: *mut client, notify: libc::c_int) -> libc::c_int;
    fn pubsubUnsubscribeShardAllChannels(
        c: *mut client,
        notify: libc::c_int,
    ) -> libc::c_int;
    fn pubsubUnsubscribeAllPatterns(c: *mut client, notify: libc::c_int) -> libc::c_int;
    fn selectDb(c: *mut client, id: libc::c_int) -> libc::c_int;
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn makeThreadKillable();
    fn getClusterConnectionsCount() -> libc::c_ulong;
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
pub type __mode_t = libc::c_uint;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
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
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_int,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 8],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 64],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
pub type va_list = __builtin_va_list;
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
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed = 0;
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
    pub v: C2RustUnnamed_0,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type ld2string_mode = libc::c_uint;
pub const LD_STR_HEX: ld2string_mode = 2;
pub const LD_STR_HUMAN: ld2string_mode = 1;
pub const LD_STR_AUTO: ld2string_mode = 0;
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_1 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_1 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_1 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_1 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_1 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_1 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_1 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_1 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_1 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_1 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_1 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_1 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_1 = 0;
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
pub struct clientReplyBlock {
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
}
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
    pub bs: C2RustUnnamed_5,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub range: C2RustUnnamed_4,
    pub keynum: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
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
pub union C2RustUnnamed_5 {
    pub index: C2RustUnnamed_7,
    pub keyword: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub type C2RustUnnamed_8 = libc::c_uint;
pub const PROPAGATION_ERR_BEHAVIOR_PANIC_ON_REPLICAS: C2RustUnnamed_8 = 2;
pub const PROPAGATION_ERR_BEHAVIOR_PANIC: C2RustUnnamed_8 = 1;
pub const PROPAGATION_ERR_BEHAVIOR_IGNORE: C2RustUnnamed_8 = 0;
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
    pub inst_metric: [C2RustUnnamed_9; 5],
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
pub struct C2RustUnnamed_9 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
pub const _ISprint: C2RustUnnamed_10 = 16384;
#[derive(Copy, Clone)]
#[repr(C, align(64))]
pub struct threads_pending(pub threads_pending_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct threads_pending_Inner {
    pub value: libc::c_ulong,
}
pub const threads_pending_cnt: threads_pending = threads_pending(threads_pending_Inner {
    value: 0,
});
#[allow(dead_code, non_upper_case_globals)]
const threads_pending_PADDING: usize = core::mem::size_of::<threads_pending>()
    - core::mem::size_of::<threads_pending_Inner>();
pub type C2RustUnnamed_10 = libc::c_uint;
pub const _ISalnum: C2RustUnnamed_10 = 8;
pub const _ISpunct: C2RustUnnamed_10 = 4;
pub const _IScntrl: C2RustUnnamed_10 = 2;
pub const _ISblank: C2RustUnnamed_10 = 1;
pub const _ISgraph: C2RustUnnamed_10 = 32768;
pub const _ISspace: C2RustUnnamed_10 = 8192;
pub const _ISxdigit: C2RustUnnamed_10 = 4096;
pub const _ISdigit: C2RustUnnamed_10 = 2048;
pub const _ISalpha: C2RustUnnamed_10 = 1024;
pub const _ISlower: C2RustUnnamed_10 = 512;
pub const _ISupper: C2RustUnnamed_10 = 256;
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
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .alloc;
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn connAccept(
    mut conn: *mut connection,
    mut accept_handler: ConnectionCallbackFunc,
) -> libc::c_int {
    return ((*(*conn).type_0).accept)
        .expect("non-null function pointer")(conn, accept_handler);
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
unsafe extern "C" fn connWritev(
    mut conn: *mut connection,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> libc::c_int {
    return ((*(*conn).type_0).writev)
        .expect("non-null function pointer")(conn, iov, iovcnt);
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
unsafe extern "C" fn connSetWriteHandlerWithBarrier(
    mut conn: *mut connection,
    mut func: ConnectionCallbackFunc,
    mut barrier: libc::c_int,
) -> libc::c_int {
    return ((*(*conn).type_0).set_write_handler)
        .expect("non-null function pointer")(conn, func, barrier);
}
#[inline]
unsafe extern "C" fn connClose(mut conn: *mut connection) {
    ((*(*conn).type_0).close).expect("non-null function pointer")(conn);
}
#[inline]
unsafe extern "C" fn connGetLastError(mut conn: *mut connection) -> *const libc::c_char {
    return ((*(*conn).type_0).get_last_error).expect("non-null function pointer")(conn);
}
#[no_mangle]
pub static mut ProcessingEventsWhileBlocked: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn sdsZmallocSize(mut s: sds) -> size_t {
    let mut sh: *mut libc::c_void = sdsAllocPtr(s);
    return je_malloc_usable_size(sh);
}
#[no_mangle]
pub unsafe extern "C" fn getStringObjectSdsUsedMemory(mut o: *mut robj) -> size_t {
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
        );
        unreachable!();
    };
    match (*o).encoding() as libc::c_int {
        0 => return sdsZmallocSize((*o).ptr as sds),
        8 => {
            return (je_malloc_usable_size(o as *mut libc::c_void))
                .wrapping_sub(core::mem::size_of::<robj>() as libc::c_ulong);
        }
        _ => return 0 as libc::c_int as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn getStringObjectLen(mut o: *mut robj) -> size_t {
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"o->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
        );
        unreachable!();
    };
    match (*o).encoding() as libc::c_int {
        0 => return sdslen((*o).ptr as sds),
        8 => return sdslen((*o).ptr as sds),
        _ => return 0 as libc::c_int as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn dupClientReplyValue(
    mut o: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut old: *mut clientReplyBlock = o as *mut clientReplyBlock;
    let mut buf: *mut clientReplyBlock = zmalloc(
        (core::mem::size_of::<clientReplyBlock>() as libc::c_ulong)
            .wrapping_add((*old).size),
    ) as *mut clientReplyBlock;
    memcpy(
        buf as *mut libc::c_void,
        o,
        (core::mem::size_of::<clientReplyBlock>() as libc::c_ulong)
            .wrapping_add((*old).size),
    );
    return buf as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn freeClientReplyValue(mut o: *mut libc::c_void) {
    zfree(o);
}
#[no_mangle]
pub unsafe extern "C" fn listMatchObjects(
    mut a: *mut libc::c_void,
    mut b: *mut libc::c_void,
) -> libc::c_int {
    return equalStringObjects(a as *mut robj, b as *mut robj);
}
#[no_mangle]
pub unsafe extern "C" fn linkClient(mut c: *mut client) {
    listAddNodeTail(server.clients, c as *mut libc::c_void);
    (*c).client_list_node = (*server.clients).tail;
    let mut id: uint64_t = intrev64((*c).id);
    raxInsert(
        server.clients_index,
        &mut id as *mut uint64_t as *mut libc::c_uchar,
        core::mem::size_of::<uint64_t>() as libc::c_ulong,
        c as *mut libc::c_void,
        0 as *mut *mut libc::c_void,
    );
}
unsafe extern "C" fn clientSetDefaultAuth(mut c: *mut client) {
    (*c).user = DefaultUser;
    (*c)
        .authenticated = ((*(*c).user).flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
        && (*(*c).user).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint
            == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn authRequired(mut c: *mut client) -> libc::c_int {
    let mut auth_required: libc::c_int = (((*DefaultUser).flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint == 0
        || (*DefaultUser).flags
            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0)
        && (*c).authenticated == 0) as libc::c_int;
    return auth_required;
}
#[no_mangle]
pub unsafe extern "C" fn createClient(mut conn: *mut connection) -> *mut client {
    let mut c: *mut client = zmalloc(core::mem::size_of::<client>() as libc::c_ulong)
        as *mut client;
    if !conn.is_null() {
        connEnableTcpNoDelay(conn);
        if server.tcpkeepalive != 0 {
            connKeepAlive(conn, server.tcpkeepalive);
        }
        connSetReadHandler(
            conn,
            Some(readQueryFromClient as unsafe extern "C" fn(*mut connection) -> ()),
        );
        connSetPrivateData(conn, c as *mut libc::c_void);
    }
    (*c)
        .buf = zmalloc((16 as libc::c_int * 1024 as libc::c_int) as size_t)
        as *mut libc::c_char;
    selectDb(c, 0 as libc::c_int);
    let mut client_id: uint64_t = 0;
    (*c).id = client_id;
    (*c).resp = 2 as libc::c_int;
    (*c).conn = conn;
    (*c).name = 0 as *mut robj;
    (*c).bufpos = 0 as libc::c_int;
    (*c).buf_usable_size = je_malloc_usable_size((*c).buf as *mut libc::c_void);
    (*c).buf_peak = (*c).buf_usable_size;
    (*c).buf_peak_last_reset_time = server.unixtime as mstime_t;
    (*c).ref_repl_buf_node = 0 as *mut listNode;
    (*c).ref_block_pos = 0 as libc::c_int as size_t;
    (*c).qb_pos = 0 as libc::c_int as size_t;
    (*c).querybuf = sdsempty();
    (*c).querybuf_peak = 0 as libc::c_int as size_t;
    (*c).reqtype = 0 as libc::c_int;
    (*c).argc = 0 as libc::c_int;
    (*c).argv = 0 as *mut *mut robj;
    (*c).argv_len = 0 as libc::c_int;
    (*c).argv_len_sum = 0 as libc::c_int as size_t;
    (*c).original_argc = 0 as libc::c_int;
    (*c).original_argv = 0 as *mut *mut robj;
    (*c).realcmd = 0 as *mut redisCommand;
    (*c).lastcmd = (*c).realcmd;
    (*c).cmd = (*c).lastcmd;
    (*c).cur_script = 0 as *mut dictEntry;
    (*c).multibulklen = 0 as libc::c_int;
    (*c).bulklen = -(1 as libc::c_int) as libc::c_long;
    (*c).sentlen = 0 as libc::c_int as size_t;
    (*c).flags = 0 as libc::c_int as uint64_t;
    (*c).slot = -(1 as libc::c_int);
    (*c).lastinteraction = server.unixtime as time_t;
    (*c).ctime = (*c).lastinteraction;
    clientSetDefaultAuth(c);
    (*c).replstate = REPL_STATE_NONE as libc::c_int;
    (*c).repl_start_cmd_stream_on_ack = 0 as libc::c_int;
    (*c).reploff = 0 as libc::c_int as libc::c_longlong;
    (*c).read_reploff = 0 as libc::c_int as libc::c_longlong;
    (*c).repl_applied = 0 as libc::c_int as libc::c_longlong;
    (*c).repl_ack_off = 0 as libc::c_int as libc::c_longlong;
    (*c).repl_ack_time = 0 as libc::c_int as libc::c_longlong;
    (*c).repl_last_partial_write = 0 as libc::c_int as libc::c_longlong;
    (*c).slave_listening_port = 0 as libc::c_int;
    (*c).slave_addr = 0 as *mut libc::c_char;
    (*c).slave_capa = 0 as libc::c_int;
    (*c).slave_req = 0 as libc::c_int;
    (*c).reply = listCreate();
    (*c).deferred_reply_errors = 0 as *mut list;
    (*c).reply_bytes = 0 as libc::c_int as libc::c_ulonglong;
    (*c).obuf_soft_limit_reached_time = 0 as libc::c_int as time_t;
    (*(*c).reply)
        .free = Some(
        freeClientReplyValue as unsafe extern "C" fn(*mut libc::c_void) -> (),
    );
    (*(*c).reply)
        .dup = Some(
        dupClientReplyValue
            as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    );
    (*c).btype = 0 as libc::c_int;
    (*c).bpop.timeout = 0 as libc::c_int as mstime_t;
    (*c).bpop.keys = dictCreate(&mut objectKeyHeapPointerValueDictType);
    (*c).bpop.target = 0 as *mut robj;
    (*c).bpop.xread_group = 0 as *mut robj;
    (*c).bpop.xread_consumer = 0 as *mut robj;
    (*c).bpop.xread_group_noack = 0 as libc::c_int;
    (*c).bpop.numreplicas = 0 as libc::c_int;
    (*c).bpop.reploffset = 0 as libc::c_int as libc::c_longlong;
    (*c).woff = 0 as libc::c_int as libc::c_longlong;
    (*c).watched_keys = listCreate();
    (*c).pubsub_channels = dictCreate(&mut objectKeyPointerValueDictType);
    (*c).pubsub_patterns = listCreate();
    (*c).pubsubshard_channels = dictCreate(&mut objectKeyPointerValueDictType);
    (*c).peerid = 0 as sds;
    (*c).sockname = 0 as sds;
    (*c).client_list_node = 0 as *mut listNode;
    (*c).postponed_list_node = 0 as *mut listNode;
    (*c).pending_read_list_node = 0 as *mut listNode;
    (*c).client_tracking_redirection = 0 as libc::c_int as uint64_t;
    (*c).client_tracking_prefixes = 0 as *mut rax;
    (*c).last_memory_usage = 0 as libc::c_int as size_t;
    (*c).last_memory_type = 0 as libc::c_int;
    (*c).auth_callback = None;
    (*c).auth_callback_privdata = 0 as *mut libc::c_void;
    (*c).auth_module = 0 as *mut libc::c_void;
    (*(*c).pubsub_patterns)
        .free = Some(decrRefCountVoid as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*c).pubsub_patterns)
        .match_0 = Some(
        listMatchObjects
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    (*c).mem_usage_bucket = 0 as *mut clientMemUsageBucket;
    (*c).mem_usage_bucket_node = 0 as *mut listNode;
    if !conn.is_null() {
        linkClient(c);
    }
    initClientMultiState(c);
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn installClientWriteHandler(mut c: *mut client) {
    let mut ae_barrier: libc::c_int = 0 as libc::c_int;
    if server.aof_state == 1 as libc::c_int && server.aof_fsync == 1 as libc::c_int {
        ae_barrier = 1 as libc::c_int;
    }
    if connSetWriteHandlerWithBarrier(
        (*c).conn,
        Some(sendReplyToClient as unsafe extern "C" fn(*mut connection) -> ()),
        ae_barrier,
    ) == -(1 as libc::c_int)
    {
        freeClientAsync(c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn putClientInPendingWriteQueue(mut c: *mut client) {
    if (*c).flags & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong == 0
        && ((*c).replstate == REPL_STATE_NONE as libc::c_int
            || (*c).replstate == 9 as libc::c_int
                && (*c).repl_start_cmd_stream_on_ack == 0)
    {
        (*c).flags |= ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong;
        listAddNodeHead(server.clients_pending_write, c as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepareClientToWrite(mut c: *mut client) -> libc::c_int {
    if (*c).flags
        & ((1 as libc::c_int) << 8 as libc::c_int
            | (1 as libc::c_int) << 27 as libc::c_int) as libc::c_ulong != 0
    {
        return 0 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
        return -(1 as libc::c_int);
    }
    if (*c).flags
        & ((1 as libc::c_int) << 22 as libc::c_int
            | (1 as libc::c_int) << 24 as libc::c_int) as libc::c_ulong != 0
    {
        return -(1 as libc::c_int);
    }
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
        && (*c).flags & ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong == 0
    {
        return -(1 as libc::c_int);
    }
    if ((*c).conn).is_null() {
        return -(1 as libc::c_int);
    }
    if clientHasPendingReplies(c) == 0 && io_threads_op == 0 as libc::c_int {
        putClientInPendingWriteQueue(c);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _addReplyToBuffer(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) -> size_t {
    let mut available: size_t = ((*c).buf_usable_size)
        .wrapping_sub((*c).bufpos as libc::c_ulong);
    if (*(*c).reply).len > 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as size_t;
    }
    let mut reply_len: size_t = if len > available { available } else { len };
    memcpy(
        ((*c).buf).offset((*c).bufpos as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        reply_len,
    );
    (*c)
        .bufpos = ((*c).bufpos as libc::c_ulong).wrapping_add(reply_len) as libc::c_int
        as libc::c_int;
    if (*c).buf_peak < (*c).bufpos as size_t {
        (*c).buf_peak = (*c).bufpos as size_t;
    }
    return reply_len;
}
#[no_mangle]
pub unsafe extern "C" fn _addReplyProtoToList(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    let mut ln: *mut listNode = (*(*c).reply).tail;
    let mut tail: *mut clientReplyBlock = (if !ln.is_null() {
        (*ln).value
    } else {
        0 as *mut libc::c_void
    }) as *mut clientReplyBlock;
    if !tail.is_null() {
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
            size
                .wrapping_add(
                    core::mem::size_of::<clientReplyBlock>() as libc::c_ulong,
                ),
            &mut usable_size,
        ) as *mut clientReplyBlock;
        (*tail)
            .size = usable_size
            .wrapping_sub(core::mem::size_of::<clientReplyBlock>() as libc::c_ulong);
        (*tail).used = len;
        memcpy(
            ((*tail).buf).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            len,
        );
        listAddNodeTail((*c).reply, tail as *mut libc::c_void);
        (*c)
            .reply_bytes = ((*c).reply_bytes)
            .wrapping_add((*tail).size as libc::c_ulonglong);
        closeClientOnOutputBufferLimitReached(c, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _addReplyToBufferOrList(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    if (*c).flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    if getClientType(c) == 1 as libc::c_int {
        let mut cmdname: sds = if !((*c).lastcmd).is_null() {
            (*(*c).lastcmd).fullname
        } else {
            0 as sds
        };
        logInvalidUseAndFreeClientAsync(
            c,
            b"Replica generated a reply to command '%s'\0" as *const u8
                as *const libc::c_char,
            if !cmdname.is_null() {
                cmdname as *const libc::c_char
            } else {
                b"<unknown>\0" as *const u8 as *const libc::c_char
            },
        );
        return;
    }
    let mut reply_len: size_t = _addReplyToBuffer(c, s, len);
    if len > reply_len {
        _addReplyProtoToList(
            c,
            s.offset(reply_len as isize),
            len.wrapping_sub(reply_len),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReply(mut c: *mut client, mut obj: *mut robj) {
    if prepareClientToWrite(c) != 0 as libc::c_int {
        return;
    }
    if (*obj).encoding() as libc::c_int == 0 as libc::c_int
        || (*obj).encoding() as libc::c_int == 8 as libc::c_int
    {
        _addReplyToBufferOrList(
            c,
            (*obj).ptr as *const libc::c_char,
            sdslen((*obj).ptr as sds),
        );
    } else if (*obj).encoding() as libc::c_int == 1 as libc::c_int {
        let mut buf: [libc::c_char; 32] = [0; 32];
        let mut len: size_t = ll2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            (*obj).ptr as libc::c_long as libc::c_longlong,
        ) as size_t;
        _addReplyToBufferOrList(c, buf.as_mut_ptr(), len);
    } else {
        _serverPanic(
            b"networking.c\0" as *const u8 as *const libc::c_char,
            419 as libc::c_int,
            b"Wrong obj->encoding in addReply()\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplySds(mut c: *mut client, mut s: sds) {
    if prepareClientToWrite(c) != 0 as libc::c_int {
        sdsfree(s);
        return;
    }
    _addReplyToBufferOrList(c, s as *const libc::c_char, sdslen(s));
    sdsfree(s);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyProto(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    if prepareClientToWrite(c) != 0 as libc::c_int {
        return;
    }
    _addReplyToBufferOrList(c, s, len);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorLength(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    if len == 0 || *s.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        addReplyProto(
            c,
            b"-ERR \0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
    }
    addReplyProto(c, s, len);
    addReplyProto(
        c,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn afterErrorReply(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
    mut flags: libc::c_int,
) {
    if (*c).flags & ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_ulong != 0 {
        if ((*c).deferred_reply_errors).is_null() {
            (*c).deferred_reply_errors = listCreate();
            (*(*c).deferred_reply_errors)
                .free = core::mem::transmute::<
                Option::<unsafe extern "C" fn(sds) -> ()>,
                Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            >(Some(sdsfree as unsafe extern "C" fn(sds) -> ()));
        }
        listAddNodeTail(
            (*c).deferred_reply_errors,
            sdsnewlen(s as *const libc::c_void, len) as *mut libc::c_void,
        );
        return;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 0 as libc::c_int == 0 {
        server.stat_total_error_replies += 1;
        if *s.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
            incrementErrorCount(
                b"ERR\0" as *const u8 as *const libc::c_char,
                3 as libc::c_int as size_t,
            );
        } else {
            let mut spaceloc: *mut libc::c_char = memchr(
                s as *const libc::c_void,
                ' ' as i32,
                if len < 32 as libc::c_int as libc::c_ulong {
                    len
                } else {
                    32 as libc::c_int as libc::c_ulong
                },
            ) as *mut libc::c_char;
            if !spaceloc.is_null() {
                let errEndPos: size_t = spaceloc.offset_from(s) as libc::c_long
                    as size_t;
                incrementErrorCount(
                    s.offset(1 as libc::c_int as isize),
                    errEndPos.wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            } else {
                incrementErrorCount(
                    b"ERR\0" as *const u8 as *const libc::c_char,
                    3 as libc::c_int as size_t,
                );
            }
        }
    } else {
        (*(*c).realcmd).failed_calls += 1;
    }
    let mut ctype: libc::c_int = getClientType(c);
    if ctype == 3 as libc::c_int || ctype == 1 as libc::c_int
        || (*c).id == 18446744073709551615 as libc::c_ulong
    {
        let mut to: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut from: *mut libc::c_char = 0 as *mut libc::c_char;
        if (*c).id == 18446744073709551615 as libc::c_ulong {
            to = b"AOF-loading-client\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
            from = b"server\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if ctype == 3 as libc::c_int {
            to = b"master\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            from = b"replica\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            to = b"replica\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            from = b"master\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if len > 4096 as libc::c_int as libc::c_ulong {
            len = 4096 as libc::c_int as size_t;
        }
        let mut cmdname: sds = if !((*c).lastcmd).is_null() {
            (*(*c).lastcmd).fullname
        } else {
            0 as sds
        };
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"== CRITICAL == This %s is sending an error to its %s: '%.*s' after processing the command '%s'\0"
                    as *const u8 as *const libc::c_char,
                from,
                to,
                len as libc::c_int,
                s,
                if !cmdname.is_null() {
                    cmdname as *const libc::c_char
                } else {
                    b"<unknown>\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if ctype == 3 as libc::c_int && !(server.repl_backlog).is_null()
            && (*server.repl_backlog).histlen > 0 as libc::c_int as libc::c_longlong
        {
            showLatestBacklog();
        }
        server.stat_unexpected_error_replies += 1;
        let mut panic_in_replicas: libc::c_int = (ctype == 3 as libc::c_int
            && server.repl_slave_ro != 0
            && (server.propagation_error_behavior
                == PROPAGATION_ERR_BEHAVIOR_PANIC as libc::c_int
                || server.propagation_error_behavior
                    == PROPAGATION_ERR_BEHAVIOR_PANIC_ON_REPLICAS as libc::c_int))
            as libc::c_int;
        let mut panic_in_aof: libc::c_int = ((*c).id
            == 18446744073709551615 as libc::c_ulong
            && server.propagation_error_behavior
                == PROPAGATION_ERR_BEHAVIOR_PANIC as libc::c_int) as libc::c_int;
        if panic_in_replicas != 0 || panic_in_aof != 0 {
            _serverPanic(
                b"networking.c\0" as *const u8 as *const libc::c_char,
                556 as libc::c_int,
                b"This %s panicked sending an error to its %s after processing the command '%s'\0"
                    as *const u8 as *const libc::c_char,
                from,
                to,
                (if !cmdname.is_null() {
                    cmdname as *const libc::c_char
                } else {
                    b"<unknown>\0" as *const u8 as *const libc::c_char
                }),
            );
            unreachable!();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorObject(mut c: *mut client, mut err: *mut robj) {
    addReply(c, err);
    afterErrorReply(
        c,
        (*err).ptr as *const libc::c_char,
        (sdslen((*err).ptr as sds)).wrapping_sub(2 as libc::c_int as libc::c_ulong),
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyOrErrorObject(
    mut c: *mut client,
    mut reply: *mut robj,
) {
    if (*reply).encoding() as libc::c_int == 0 as libc::c_int
        || (*reply).encoding() as libc::c_int == 8 as libc::c_int
    {} else {
        _serverAssert(
            b"sdsEncodedObject(reply)\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            574 as libc::c_int,
        );
        unreachable!();
    };
    let mut rep: sds = (*reply).ptr as sds;
    if sdslen(rep) > 1 as libc::c_int as libc::c_ulong
        && *rep.offset(0 as libc::c_int as isize) as libc::c_int == '-' as i32
    {
        addReplyErrorObject(c, reply);
    } else {
        addReply(c, reply);
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyError(
    mut c: *mut client,
    mut err: *const libc::c_char,
) {
    addReplyErrorLength(c, err, strlen(err));
    afterErrorReply(c, err, strlen(err), 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorSdsEx(
    mut c: *mut client,
    mut err: sds,
    mut flags: libc::c_int,
) {
    addReplyErrorLength(c, err as *const libc::c_char, sdslen(err));
    afterErrorReply(c, err as *const libc::c_char, sdslen(err), flags);
    sdsfree(err);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorSds(mut c: *mut client, mut err: sds) {
    addReplyErrorSdsEx(c, err, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorSdsSafe(mut c: *mut client, mut err: sds) {
    err = sdsmapchars(
        err,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        b"  \0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    addReplyErrorSdsEx(c, err, 0 as libc::c_int);
}
unsafe extern "C" fn addReplyErrorFormatInternal(
    mut c: *mut client,
    mut flags: libc::c_int,
    mut fmt: *const libc::c_char,
    mut ap: core::ffi::VaList,
) {
    let mut cpy: core::ffi::VaListImpl;
    cpy = ap.clone();
    let mut s: sds = sdscatvprintf(sdsempty(), fmt, cpy.as_va_list());
    s = sdstrim(s, b"\r\n\0" as *const u8 as *const libc::c_char);
    s = sdsmapchars(
        s,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        b"  \0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    addReplyErrorLength(c, s as *const libc::c_char, sdslen(s));
    afterErrorReply(c, s as *const libc::c_char, sdslen(s), flags);
    sdsfree(s);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorFormatEx(
    mut c: *mut client,
    mut flags: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    addReplyErrorFormatInternal(c, flags, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorFormat(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    addReplyErrorFormatInternal(c, 0 as libc::c_int, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorArity(mut c: *mut client) {
    addReplyErrorFormat(
        c,
        b"wrong number of arguments for '%s' command\0" as *const u8
            as *const libc::c_char,
        (*(*c).cmd).fullname,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyErrorExpireTime(mut c: *mut client) {
    addReplyErrorFormat(
        c,
        b"invalid expire time in '%s' command\0" as *const u8 as *const libc::c_char,
        (*(*c).cmd).fullname,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyStatusLength(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
) {
    addReplyProto(
        c,
        b"+\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    addReplyProto(c, s, len);
    addReplyProto(
        c,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addReplyStatus(
    mut c: *mut client,
    mut status: *const libc::c_char,
) {
    addReplyStatusLength(c, status, strlen(status));
}
#[no_mangle]
pub unsafe extern "C" fn addReplyStatusFormat(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    let mut s: sds = sdscatvprintf(sdsempty(), fmt, ap.as_va_list());
    addReplyStatusLength(c, s as *const libc::c_char, sdslen(s));
    sdsfree(s);
}
#[no_mangle]
pub unsafe extern "C" fn trimReplyUnusedTailSpace(mut c: *mut client) {
    let mut ln: *mut listNode = (*(*c).reply).tail;
    let mut tail: *mut clientReplyBlock = (if !ln.is_null() {
        (*ln).value
    } else {
        0 as *mut libc::c_void
    }) as *mut clientReplyBlock;
    if tail.is_null() {
        return;
    }
    if ((*tail).size).wrapping_sub((*tail).used)
        > ((*tail).size).wrapping_div(4 as libc::c_int as libc::c_ulong)
        && (*tail).used < (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
    {
        let mut old_size: size_t = (*tail).size;
        tail = zrealloc(
            tail as *mut libc::c_void,
            ((*tail).used)
                .wrapping_add(
                    core::mem::size_of::<clientReplyBlock>() as libc::c_ulong,
                ),
        ) as *mut clientReplyBlock;
        (*tail)
            .size = (je_malloc_usable_size(tail as *mut libc::c_void))
            .wrapping_sub(core::mem::size_of::<clientReplyBlock>() as libc::c_ulong);
        (*c)
            .reply_bytes = ((*c).reply_bytes)
            .wrapping_add((*tail).size as libc::c_ulonglong)
            .wrapping_sub(old_size as libc::c_ulonglong);
        (*ln).value = tail as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyDeferredLen(mut c: *mut client) -> *mut libc::c_void {
    if prepareClientToWrite(c) != 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    if getClientType(c) == 1 as libc::c_int {
        let mut cmdname: sds = if !((*c).lastcmd).is_null() {
            (*(*c).lastcmd).fullname
        } else {
            0 as sds
        };
        logInvalidUseAndFreeClientAsync(
            c,
            b"Replica generated a reply to command '%s'\0" as *const u8
                as *const libc::c_char,
            if !cmdname.is_null() {
                cmdname as *const libc::c_char
            } else {
                b"<unknown>\0" as *const u8 as *const libc::c_char
            },
        );
        return 0 as *mut libc::c_void;
    }
    trimReplyUnusedTailSpace(c);
    listAddNodeTail((*c).reply, 0 as *mut libc::c_void);
    return (*(*c).reply).tail as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredReply(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut s: *const libc::c_char,
    mut length: size_t,
) {
    let mut ln: *mut listNode = node as *mut listNode;
    let mut next: *mut clientReplyBlock = 0 as *mut clientReplyBlock;
    let mut prev: *mut clientReplyBlock = 0 as *mut clientReplyBlock;
    if node.is_null() {
        return;
    }
    if ((*ln).value).is_null() {} else {
        _serverAssert(
            b"!listNodeValue(ln)\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            732 as libc::c_int,
        );
        unreachable!();
    };
    if !((*ln).prev).is_null()
        && {
            prev = (*(*ln).prev).value as *mut clientReplyBlock;
            !prev.is_null()
        }
        && ((*prev).size).wrapping_sub((*prev).used) > 0 as libc::c_int as libc::c_ulong
    {
        let mut len_to_copy: size_t = ((*prev).size).wrapping_sub((*prev).used);
        if len_to_copy > length {
            len_to_copy = length;
        }
        memcpy(
            ((*prev).buf).as_mut_ptr().offset((*prev).used as isize)
                as *mut libc::c_void,
            s as *const libc::c_void,
            len_to_copy,
        );
        (*prev)
            .used = ((*prev).used as libc::c_ulong).wrapping_add(len_to_copy) as size_t
            as size_t;
        length = (length as libc::c_ulong).wrapping_sub(len_to_copy) as size_t as size_t;
        if length == 0 as libc::c_int as libc::c_ulong {
            listDelNode((*c).reply, ln);
            return;
        }
        s = s.offset(len_to_copy as isize);
    }
    if !((*ln).next).is_null()
        && {
            next = (*(*ln).next).value as *mut clientReplyBlock;
            !next.is_null()
        } && ((*next).size).wrapping_sub((*next).used) >= length
        && (*next).used
            < (16 as libc::c_int * 1024 as libc::c_int * 4 as libc::c_int)
                as libc::c_ulong
    {
        memmove(
            ((*next).buf).as_mut_ptr().offset(length as isize) as *mut libc::c_void,
            ((*next).buf).as_mut_ptr() as *const libc::c_void,
            (*next).used,
        );
        memcpy(
            ((*next).buf).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            length,
        );
        (*next)
            .used = ((*next).used as libc::c_ulong).wrapping_add(length) as size_t
            as size_t;
        listDelNode((*c).reply, ln);
    } else {
        let mut buf: *mut clientReplyBlock = zmalloc(
            length
                .wrapping_add(
                    core::mem::size_of::<clientReplyBlock>() as libc::c_ulong,
                ),
        ) as *mut clientReplyBlock;
        (*buf)
            .size = (je_malloc_usable_size(buf as *mut libc::c_void))
            .wrapping_sub(core::mem::size_of::<clientReplyBlock>() as libc::c_ulong);
        (*buf).used = length;
        memcpy(
            ((*buf).buf).as_mut_ptr() as *mut libc::c_void,
            s as *const libc::c_void,
            length,
        );
        (*ln).value = buf as *mut libc::c_void;
        (*c)
            .reply_bytes = ((*c).reply_bytes)
            .wrapping_add((*buf).size as libc::c_ulonglong);
        closeClientOnOutputBufferLimitReached(c, 1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredAggregateLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
    mut prefix: libc::c_char,
) {
    if length >= 0 as libc::c_int as libc::c_long {} else {
        _serverAssert(
            b"length >= 0\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            785 as libc::c_int,
        );
        unreachable!();
    };
    if node.is_null() {
        return;
    }
    let hdr_len: size_t = (if length < 10 as libc::c_int as libc::c_long {
        4 as libc::c_int
    } else {
        5 as libc::c_int
    }) as size_t;
    let opt_hdr: libc::c_int = (length < 32 as libc::c_int as libc::c_long)
        as libc::c_int;
    if prefix as libc::c_int == '*' as i32 && opt_hdr != 0 {
        setDeferredReply(
            c,
            node,
            (*shared.mbulkhdr[length as usize]).ptr as *const libc::c_char,
            hdr_len,
        );
        return;
    }
    if prefix as libc::c_int == '%' as i32 && opt_hdr != 0 {
        setDeferredReply(
            c,
            node,
            (*shared.maphdr[length as usize]).ptr as *const libc::c_char,
            hdr_len,
        );
        return;
    }
    if prefix as libc::c_int == '~' as i32 && opt_hdr != 0 {
        setDeferredReply(
            c,
            node,
            (*shared.sethdr[length as usize]).ptr as *const libc::c_char,
            hdr_len,
        );
        return;
    }
    let mut lenstr: [libc::c_char; 128] = [0; 128];
    let mut lenstr_len: size_t = sprintf(
        lenstr.as_mut_ptr(),
        b"%c%ld\r\n\0" as *const u8 as *const libc::c_char,
        prefix as libc::c_int,
        length,
    ) as size_t;
    setDeferredReply(c, node, lenstr.as_mut_ptr(), lenstr_len);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredArrayLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
) {
    setDeferredAggregateLen(c, node, length, '*' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredMapLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
) {
    let mut prefix: libc::c_int = if (*c).resp == 2 as libc::c_int {
        '*' as i32
    } else {
        '%' as i32
    };
    if (*c).resp == 2 as libc::c_int {
        length *= 2 as libc::c_int as libc::c_long;
    }
    setDeferredAggregateLen(c, node, length, prefix as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredSetLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
) {
    let mut prefix: libc::c_int = if (*c).resp == 2 as libc::c_int {
        '*' as i32
    } else {
        '~' as i32
    };
    setDeferredAggregateLen(c, node, length, prefix as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredAttributeLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
) {
    if (*c).resp >= 3 as libc::c_int {} else {
        _serverAssert(
            b"c->resp >= 3\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            830 as libc::c_int,
        );
        unreachable!();
    };
    setDeferredAggregateLen(c, node, length, '|' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredPushLen(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut length: libc::c_long,
) {
    if (*c).resp >= 3 as libc::c_int {} else {
        _serverAssert(
            b"c->resp >= 3\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            835 as libc::c_int,
        );
        unreachable!();
    };
    setDeferredAggregateLen(c, node, length, '>' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyDouble(mut c: *mut client, mut d: libc::c_double) {
    if if d.is_infinite() { if d.is_sign_positive() { 1 } else { -1 } } else { 0 } != 0 {
        if (*c).resp == 2 as libc::c_int {
            addReplyBulkCString(
                c,
                if d > 0 as libc::c_int as libc::c_double {
                    b"inf\0" as *const u8 as *const libc::c_char
                } else {
                    b"-inf\0" as *const u8 as *const libc::c_char
                },
            );
        } else {
            addReplyProto(
                c,
                if d > 0 as libc::c_int as libc::c_double {
                    b",inf\r\n\0" as *const u8 as *const libc::c_char
                } else {
                    b",-inf\r\n\0" as *const u8 as *const libc::c_char
                },
                (if d > 0 as libc::c_int as libc::c_double {
                    6 as libc::c_int
                } else {
                    7 as libc::c_int
                }) as size_t,
            );
        }
    } else {
        let mut dbuf: [libc::c_char; 5152] = [0; 5152];
        let mut dlen: libc::c_int = 0 as libc::c_int;
        if (*c).resp == 2 as libc::c_int {
            let mut dlen_0: libc::c_int = snprintf(
                dbuf.as_mut_ptr().offset(7 as libc::c_int as isize),
                (core::mem::size_of::<[libc::c_char; 5152]>() as libc::c_ulong)
                    .wrapping_sub(7 as libc::c_int as libc::c_ulong),
                b"%.17g\0" as *const u8 as *const libc::c_char,
                d,
            );
            let mut digits: libc::c_int = digits10(dlen_0 as uint64_t) as libc::c_int;
            let mut start: libc::c_int = 4 as libc::c_int - digits;
            dbuf[start as usize] = '$' as i32 as libc::c_char;
            let mut i: libc::c_int = digits;
            let mut val: libc::c_int = dlen_0;
            while val != 0 && i > 0 as libc::c_int {
                dbuf[(start + i)
                    as usize] = (*core::mem::transmute::<
                    &[u8; 11],
                    &[libc::c_char; 11],
                >(b"0123456789\0"))[(val % 10 as libc::c_int) as usize];
                i -= 1;
                val /= 10 as libc::c_int;
            }
            dbuf[5 as libc::c_int as usize] = '\r' as i32 as libc::c_char;
            dbuf[6 as libc::c_int as usize] = '\n' as i32 as libc::c_char;
            dbuf[(dlen_0 + 7 as libc::c_int) as usize] = '\r' as i32 as libc::c_char;
            dbuf[(dlen_0 + 8 as libc::c_int) as usize] = '\n' as i32 as libc::c_char;
            addReplyProto(
                c,
                dbuf.as_mut_ptr().offset(start as isize),
                (dlen_0 + 9 as libc::c_int - start) as size_t,
            );
        } else {
            dlen = snprintf(
                dbuf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 5152]>() as libc::c_ulong,
                b",%.17g\r\n\0" as *const u8 as *const libc::c_char,
                d,
            );
            addReplyProto(c, dbuf.as_mut_ptr(), dlen as size_t);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBigNum(
    mut c: *mut client,
    mut num: *const libc::c_char,
    mut len: size_t,
) {
    if (*c).resp == 2 as libc::c_int {
        addReplyBulkCBuffer(c, num as *const libc::c_void, len);
    } else {
        addReplyProto(
            c,
            b"(\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        addReplyProto(c, num, len);
        addReply(c, shared.crlf);
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyHumanLongDouble(mut c: *mut client, mut d: f64) {
    if (*c).resp == 2 as libc::c_int {
        let mut o: *mut robj = createStringObjectFromLongDouble(d, 1 as libc::c_int);
        addReplyBulk(c, o);
        decrRefCount(o);
    } else {
        let mut buf: [libc::c_char; 5120] = [0; 5120];
        let mut len: libc::c_int = ld2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 5120]>() as libc::c_ulong,
            d,
            LD_STR_HUMAN,
        );
        addReplyProto(
            c,
            b",\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int as size_t,
        );
        addReplyProto(c, buf.as_mut_ptr(), len as size_t);
        addReplyProto(
            c,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyLongLongWithPrefix(
    mut c: *mut client,
    mut ll: libc::c_longlong,
    mut prefix: libc::c_char,
) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    let opt_hdr: libc::c_int = (ll < 32 as libc::c_int as libc::c_longlong
        && ll >= 0 as libc::c_int as libc::c_longlong) as libc::c_int;
    let hdr_len: size_t = (if ll < 10 as libc::c_int as libc::c_longlong {
        4 as libc::c_int
    } else {
        5 as libc::c_int
    }) as size_t;
    if prefix as libc::c_int == '*' as i32 && opt_hdr != 0 {
        addReplyProto(
            c,
            (*shared.mbulkhdr[ll as usize]).ptr as *const libc::c_char,
            hdr_len,
        );
        return;
    } else {
        if prefix as libc::c_int == '$' as i32 && opt_hdr != 0 {
            addReplyProto(
                c,
                (*shared.bulkhdr[ll as usize]).ptr as *const libc::c_char,
                hdr_len,
            );
            return;
        } else {
            if prefix as libc::c_int == '%' as i32 && opt_hdr != 0 {
                addReplyProto(
                    c,
                    (*shared.maphdr[ll as usize]).ptr as *const libc::c_char,
                    hdr_len,
                );
                return;
            } else {
                if prefix as libc::c_int == '~' as i32 && opt_hdr != 0 {
                    addReplyProto(
                        c,
                        (*shared.sethdr[ll as usize]).ptr as *const libc::c_char,
                        hdr_len,
                    );
                    return;
                }
            }
        }
    }
    buf[0 as libc::c_int as usize] = prefix;
    len = ll2string(
        buf.as_mut_ptr().offset(1 as libc::c_int as isize),
        (core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ll,
    );
    buf[(len + 1 as libc::c_int) as usize] = '\r' as i32 as libc::c_char;
    buf[(len + 2 as libc::c_int) as usize] = '\n' as i32 as libc::c_char;
    addReplyProto(c, buf.as_mut_ptr(), (len + 3 as libc::c_int) as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyLongLong(mut c: *mut client, mut ll: libc::c_longlong) {
    if ll == 0 as libc::c_int as libc::c_longlong {
        addReply(c, shared.czero);
    } else if ll == 1 as libc::c_int as libc::c_longlong {
        addReply(c, shared.cone);
    } else {
        addReplyLongLongWithPrefix(c, ll, ':' as i32 as libc::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyAggregateLen(
    mut c: *mut client,
    mut length: libc::c_long,
    mut prefix: libc::c_int,
) {
    if length >= 0 as libc::c_int as libc::c_long {} else {
        _serverAssert(
            b"length >= 0\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            950 as libc::c_int,
        );
        unreachable!();
    };
    addReplyLongLongWithPrefix(c, length as libc::c_longlong, prefix as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyArrayLen(mut c: *mut client, mut length: libc::c_long) {
    addReplyAggregateLen(c, length, '*' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyMapLen(mut c: *mut client, mut length: libc::c_long) {
    let mut prefix: libc::c_int = if (*c).resp == 2 as libc::c_int {
        '*' as i32
    } else {
        '%' as i32
    };
    if (*c).resp == 2 as libc::c_int {
        length *= 2 as libc::c_int as libc::c_long;
    }
    addReplyAggregateLen(c, length, prefix);
}
#[no_mangle]
pub unsafe extern "C" fn addReplySetLen(mut c: *mut client, mut length: libc::c_long) {
    let mut prefix: libc::c_int = if (*c).resp == 2 as libc::c_int {
        '*' as i32
    } else {
        '~' as i32
    };
    addReplyAggregateLen(c, length, prefix);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyAttributeLen(
    mut c: *mut client,
    mut length: libc::c_long,
) {
    if (*c).resp >= 3 as libc::c_int {} else {
        _serverAssert(
            b"c->resp >= 3\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            970 as libc::c_int,
        );
        unreachable!();
    };
    addReplyAggregateLen(c, length, '|' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyPushLen(mut c: *mut client, mut length: libc::c_long) {
    if (*c).resp >= 3 as libc::c_int {} else {
        _serverAssert(
            b"c->resp >= 3\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            975 as libc::c_int,
        );
        unreachable!();
    };
    addReplyAggregateLen(c, length, '>' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyNull(mut c: *mut client) {
    if (*c).resp == 2 as libc::c_int {
        addReplyProto(
            c,
            b"$-1\r\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
    } else {
        addReplyProto(
            c,
            b"_\r\n\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBool(mut c: *mut client, mut b: libc::c_int) {
    if (*c).resp == 2 as libc::c_int {
        addReply(c, if b != 0 { shared.cone } else { shared.czero });
    } else {
        addReplyProto(
            c,
            if b != 0 {
                b"#t\r\n\0" as *const u8 as *const libc::c_char
            } else {
                b"#f\r\n\0" as *const u8 as *const libc::c_char
            },
            4 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyNullArray(mut c: *mut client) {
    if (*c).resp == 2 as libc::c_int {
        addReplyProto(
            c,
            b"*-1\r\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int as size_t,
        );
    } else {
        addReplyProto(
            c,
            b"_\r\n\0" as *const u8 as *const libc::c_char,
            3 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulkLen(mut c: *mut client, mut obj: *mut robj) {
    let mut len: size_t = stringObjectLen(obj);
    addReplyLongLongWithPrefix(c, len as libc::c_longlong, '$' as i32 as libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulk(mut c: *mut client, mut obj: *mut robj) {
    addReplyBulkLen(c, obj);
    addReply(c, obj);
    addReply(c, shared.crlf);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulkCBuffer(
    mut c: *mut client,
    mut p: *const libc::c_void,
    mut len: size_t,
) {
    addReplyLongLongWithPrefix(c, len as libc::c_longlong, '$' as i32 as libc::c_char);
    addReplyProto(c, p as *const libc::c_char, len);
    addReply(c, shared.crlf);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulkSds(mut c: *mut client, mut s: sds) {
    addReplyLongLongWithPrefix(
        c,
        sdslen(s) as libc::c_longlong,
        '$' as i32 as libc::c_char,
    );
    addReplySds(c, s);
    addReply(c, shared.crlf);
}
#[no_mangle]
pub unsafe extern "C" fn setDeferredReplyBulkSds(
    mut c: *mut client,
    mut node: *mut libc::c_void,
    mut s: sds,
) {
    let mut reply: sds = sdscatprintf(
        sdsempty(),
        b"$%d\r\n%s\r\n\0" as *const u8 as *const libc::c_char,
        sdslen(s) as libc::c_uint,
        s,
    );
    setDeferredReply(c, node, reply as *const libc::c_char, sdslen(reply));
    sdsfree(reply);
    sdsfree(s);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulkCString(
    mut c: *mut client,
    mut s: *const libc::c_char,
) {
    if s.is_null() {
        addReplyNull(c);
    } else {
        addReplyBulkCBuffer(c, s as *const libc::c_void, strlen(s));
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyBulkLongLong(
    mut c: *mut client,
    mut ll: libc::c_longlong,
) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut len: libc::c_int = 0;
    len = ll2string(buf.as_mut_ptr(), 64 as libc::c_int as size_t, ll);
    addReplyBulkCBuffer(c, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyVerbatim(
    mut c: *mut client,
    mut s: *const libc::c_char,
    mut len: size_t,
    mut ext: *const libc::c_char,
) {
    if (*c).resp == 2 as libc::c_int {
        addReplyBulkCBuffer(c, s as *const libc::c_void, len);
    } else {
        let mut buf: [libc::c_char; 32] = [0; 32];
        let mut preflen: size_t = snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            b"=%zu\r\nxxx:\0" as *const u8 as *const libc::c_char,
            len.wrapping_add(4 as libc::c_int as libc::c_ulong),
        ) as size_t;
        let mut p: *mut libc::c_char = buf
            .as_mut_ptr()
            .offset(preflen as isize)
            .offset(-(4 as libc::c_int as isize));
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 3 as libc::c_int {
            if *ext as libc::c_int == '\0' as i32 {
                *p.offset(i as isize) = ' ' as i32 as libc::c_char;
            } else {
                let fresh0 = ext;
                ext = ext.offset(1);
                *p.offset(i as isize) = *fresh0;
            }
            i += 1;
        }
        addReplyProto(c, buf.as_mut_ptr(), preflen);
        addReplyProto(c, s, len);
        addReplyProto(
            c,
            b"\r\n\0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyHelp(
    mut c: *mut client,
    mut help: *mut *const libc::c_char,
) {
    let mut cmd: sds = sdsnew(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
    );
    let mut blenp: *mut libc::c_void = addReplyDeferredLen(c);
    let mut blen: libc::c_int = 0 as libc::c_int;
    sdstoupper(cmd);
    addReplyStatusFormat(
        c,
        b"%s <subcommand> [<arg> [value] [opt] ...]. Subcommands are:\0" as *const u8
            as *const libc::c_char,
        cmd,
    );
    sdsfree(cmd);
    while !(*help.offset(blen as isize)).is_null() {
        let fresh1 = blen;
        blen = blen + 1;
        addReplyStatus(c, *help.offset(fresh1 as isize));
    }
    addReplyStatus(c, b"HELP\0" as *const u8 as *const libc::c_char);
    addReplyStatus(c, b"    Prints this help.\0" as *const u8 as *const libc::c_char);
    blen += 1 as libc::c_int;
    blen += 2 as libc::c_int;
    setDeferredArrayLen(c, blenp, blen as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn addReplySubcommandSyntaxError(mut c: *mut client) {
    let mut cmd: sds = sdsnew(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
    );
    sdstoupper(cmd);
    addReplyErrorFormat(
        c,
        b"unknown subcommand or wrong number of arguments for '%.128s'. Try %s HELP.\0"
            as *const u8 as *const libc::c_char,
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char,
        cmd,
    );
    sdsfree(cmd);
}
#[no_mangle]
pub unsafe extern "C" fn AddReplyFromClient(mut dst: *mut client, mut src: *mut client) {
    if (*src).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
        let mut client: sds = catClientInfoString(sdsempty(), dst);
        freeClientAsync(dst);
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Client %s scheduled to be closed ASAP for overcoming of output buffer limits.\0"
                    as *const u8 as *const libc::c_char,
                client,
            );
        }
        sdsfree(client);
        return;
    }
    addReplyProto(dst, (*src).buf, (*src).bufpos as size_t);
    if prepareClientToWrite(dst) != 0 as libc::c_int {
        return;
    }
    if (*dst).flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    if (*(*src).reply).len != 0 {
        listJoin((*dst).reply, (*src).reply);
    }
    (*dst).reply_bytes = ((*dst).reply_bytes).wrapping_add((*src).reply_bytes);
    (*src).reply_bytes = 0 as libc::c_int as libc::c_ulonglong;
    (*src).bufpos = 0 as libc::c_int;
    if !((*src).deferred_reply_errors).is_null() {
        deferredAfterErrorReply(dst, (*src).deferred_reply_errors);
        listRelease((*src).deferred_reply_errors);
        (*src).deferred_reply_errors = 0 as *mut list;
    }
    closeClientOnOutputBufferLimitReached(dst, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn deferredAfterErrorReply(
    mut c: *mut client,
    mut errors: *mut list,
) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(errors, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut err: sds = (*ln).value as sds;
        afterErrorReply(c, err as *const libc::c_char, sdslen(err), 0 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn copyReplicaOutputBuffer(
    mut dst: *mut client,
    mut src: *mut client,
) {
    if (*src).bufpos == 0 as libc::c_int
        && (*(*src).reply).len == 0 as libc::c_int as libc::c_ulong
    {} else {
        _serverAssert(
            b"src->bufpos == 0 && listLength(src->reply) == 0\0" as *const u8
                as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            1186 as libc::c_int,
        );
        unreachable!();
    };
    if ((*src).ref_repl_buf_node).is_null() {
        return;
    }
    (*dst).ref_repl_buf_node = (*src).ref_repl_buf_node;
    (*dst).ref_block_pos = (*src).ref_block_pos;
    let ref mut fresh2 = (*((*(*dst).ref_repl_buf_node).value as *mut replBufBlock))
        .refcount;
    *fresh2 += 1;
}
#[no_mangle]
pub unsafe extern "C" fn clientHasPendingReplies(mut c: *mut client) -> libc::c_int {
    if getClientType(c) == 1 as libc::c_int {
        if (*c).bufpos == 0 as libc::c_int
            && (*(*c).reply).len == 0 as libc::c_int as libc::c_ulong
        {} else {
            _serverAssert(
                b"c->bufpos == 0 && listLength(c->reply) == 0\0" as *const u8
                    as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1200 as libc::c_int,
            );
            unreachable!();
        };
        if ((*c).ref_repl_buf_node).is_null() {
            return 0 as libc::c_int;
        }
        let mut ln: *mut listNode = (*server.repl_buffer_blocks).tail;
        let mut tail: *mut replBufBlock = (*ln).value as *mut replBufBlock;
        if ln == (*c).ref_repl_buf_node && (*c).ref_block_pos == (*tail).used {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    } else {
        return ((*c).bufpos != 0 || (*(*c).reply).len != 0) as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn islocalClient(mut c: *mut client) -> libc::c_int {
    if (*c).flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0 {
        return 1 as libc::c_int;
    }
    let mut cip: [libc::c_char; 47] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    connPeerToString(
        (*c).conn,
        cip.as_mut_ptr(),
        (core::mem::size_of::<[libc::c_char; 47]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        0 as *mut libc::c_int,
    );
    return (strcmp(cip.as_mut_ptr(), b"127.0.0.1\0" as *const u8 as *const libc::c_char)
        == 0
        || strcmp(cip.as_mut_ptr(), b"::1\0" as *const u8 as *const libc::c_char) == 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clientAcceptHandler(mut conn: *mut connection) {
    let mut c: *mut client = connGetPrivateData(conn) as *mut client;
    if connGetState(conn) != CONN_STATE_CONNECTED as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error accepting a client connection: %s\0" as *const u8
                    as *const libc::c_char,
                connGetLastError(conn),
            );
        }
        freeClientAsync(c);
        return;
    }
    if server.protected_mode != 0
        && (*DefaultUser).flags
            & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        if islocalClient(c) == 0 {
            let mut err: *mut libc::c_char = b"-DENIED Redis is running in protected mode because protected mode is enabled and no password is set for the default user. In this mode connections are only accepted from the loopback interface. If you want to connect from external computers to Redis you may adopt one of the following solutions: 1) Just disable protected mode sending the command 'CONFIG SET protected-mode no' from the loopback interface by connecting to Redis from the same host the server is running, however MAKE SURE Redis is not publicly accessible from internet if you do so. Use CONFIG REWRITE to make this change permanent. 2) Alternatively you can just disable the protected mode by editing the Redis configuration file, and setting the protected mode option to 'no', and then restarting the server. 3) If you started the server manually just for testing, restart it with the '--protected-mode no' option. 4) Setup a an authentication password for the default user. NOTE: You only need to do one of the above things in order for the server to start accepting connections from the outside.\r\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
            connWrite((*c).conn, err as *const libc::c_void, strlen(err))
                == -(1 as libc::c_int);
            server.stat_rejected_conn += 1;
            freeClientAsync(c);
            return;
        }
    }
    server.stat_numconnections += 1;
    moduleFireServerEvent(
        4 as libc::c_int as uint64_t,
        0 as libc::c_int,
        c as *mut libc::c_void,
    );
}
unsafe extern "C" fn acceptCommonHandler(
    mut conn: *mut connection,
    mut flags: libc::c_int,
    mut ip: *mut libc::c_char,
) {
    let mut c: *mut client = 0 as *mut client;
    let mut conninfo: [libc::c_char; 100] = [0; 100];
    if connGetState(conn) != CONN_STATE_ACCEPTING as libc::c_int {
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Accepted client connection in error state: %s (conn: %s)\0"
                    as *const u8 as *const libc::c_char,
                connGetLastError(conn),
                connGetInfo(
                    conn,
                    conninfo.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                ),
            );
        }
        connClose(conn);
        return;
    }
    if ((*server.clients).len).wrapping_add(getClusterConnectionsCount())
        >= server.maxclients as libc::c_ulong
    {
        let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
        if server.cluster_enabled != 0 {
            err = b"-ERR max number of clients + cluster connections reached\r\n\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else {
            err = b"-ERR max number of clients reached\r\n\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        connWrite(conn, err as *const libc::c_void, strlen(err)) == -(1 as libc::c_int);
        server.stat_rejected_conn += 1;
        connClose(conn);
        return;
    }
    c = createClient(conn);
    if c.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error registering fd event for the new client: %s (conn: %s)\0"
                    as *const u8 as *const libc::c_char,
                connGetLastError(conn),
                connGetInfo(
                    conn,
                    conninfo.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                ),
            );
        }
        connClose(conn);
        return;
    }
    (*c).flags |= flags as libc::c_ulong;
    if connAccept(
        conn,
        Some(clientAcceptHandler as unsafe extern "C" fn(*mut connection) -> ()),
    ) == -(1 as libc::c_int)
    {
        let mut conninfo_0: [libc::c_char; 100] = [0; 100];
        if connGetState(conn) == CONN_STATE_ERROR as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Error accepting a client connection: %s (conn: %s)\0" as *const u8
                        as *const libc::c_char,
                    connGetLastError(conn),
                    connGetInfo(
                        conn,
                        conninfo_0.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                    ),
                );
            }
        }
        freeClient(connGetPrivateData(conn) as *mut client);
        return;
    }
}
#[no_mangle]
pub unsafe extern "C" fn acceptTcpHandler(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut cport: libc::c_int = 0;
    let mut cfd: libc::c_int = 0;
    let mut max: libc::c_int = 1000 as libc::c_int;
    let mut cip: [libc::c_char; 46] = [0; 46];
    loop {
        let fresh3 = max;
        max = max - 1;
        if !(fresh3 != 0) {
            break;
        }
        cfd = anetTcpAccept(
            (server.neterr).as_mut_ptr(),
            fd,
            cip.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
            &mut cport,
        );
        if cfd == -(1 as libc::c_int) {
            if *__errno_location() != 11 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Accepting client connection: %s\0" as *const u8
                            as *const libc::c_char,
                        (server.neterr).as_mut_ptr(),
                    );
                }
            }
            return;
        }
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Accepted %s:%d\0" as *const u8 as *const libc::c_char,
                cip.as_mut_ptr(),
                cport,
            );
        }
        acceptCommonHandler(
            connCreateAcceptedSocket(cfd),
            0 as libc::c_int,
            cip.as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn acceptTLSHandler(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut cport: libc::c_int = 0;
    let mut cfd: libc::c_int = 0;
    let mut max: libc::c_int = 1000 as libc::c_int;
    let mut cip: [libc::c_char; 46] = [0; 46];
    loop {
        let fresh4 = max;
        max = max - 1;
        if !(fresh4 != 0) {
            break;
        }
        cfd = anetTcpAccept(
            (server.neterr).as_mut_ptr(),
            fd,
            cip.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
            &mut cport,
        );
        if cfd == -(1 as libc::c_int) {
            if *__errno_location() != 11 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Accepting client connection: %s\0" as *const u8
                            as *const libc::c_char,
                        (server.neterr).as_mut_ptr(),
                    );
                }
            }
            return;
        }
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Accepted %s:%d\0" as *const u8 as *const libc::c_char,
                cip.as_mut_ptr(),
                cport,
            );
        }
        acceptCommonHandler(
            connCreateAcceptedTLS(cfd, server.tls_auth_clients),
            0 as libc::c_int,
            cip.as_mut_ptr(),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn acceptUnixHandler(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut cfd: libc::c_int = 0;
    let mut max: libc::c_int = 1000 as libc::c_int;
    loop {
        let fresh5 = max;
        max = max - 1;
        if !(fresh5 != 0) {
            break;
        }
        cfd = anetUnixAccept((server.neterr).as_mut_ptr(), fd);
        if cfd == -(1 as libc::c_int) {
            if *__errno_location() != 11 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Accepting client connection: %s\0" as *const u8
                            as *const libc::c_char,
                        (server.neterr).as_mut_ptr(),
                    );
                }
            }
            return;
        }
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Accepted connection to %s\0" as *const u8 as *const libc::c_char,
                server.unixsocket,
            );
        }
        acceptCommonHandler(
            connCreateAcceptedSocket(cfd),
            (1 as libc::c_int) << 11 as libc::c_int,
            0 as *mut libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeClientOriginalArgv(mut c: *mut client) {
    if ((*c).original_argv).is_null() {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*c).original_argc {
        decrRefCount(*((*c).original_argv).offset(j as isize));
        j += 1;
    }
    zfree((*c).original_argv as *mut libc::c_void);
    (*c).original_argv = 0 as *mut *mut robj;
    (*c).original_argc = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeClientArgv(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*c).argc {
        decrRefCount(*((*c).argv).offset(j as isize));
        j += 1;
    }
    (*c).argc = 0 as libc::c_int;
    (*c).cmd = 0 as *mut redisCommand;
    (*c).argv_len_sum = 0 as libc::c_int as size_t;
    (*c).argv_len = 0 as libc::c_int;
    zfree((*c).argv as *mut libc::c_void);
    (*c).argv = 0 as *mut *mut robj;
}
#[no_mangle]
pub unsafe extern "C" fn disconnectSlaves() {
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
        freeClient((*ln).value as *mut client);
    };
}
#[no_mangle]
pub unsafe extern "C" fn anyOtherSlaveWaitRdb(
    mut except_me: *mut client,
) -> libc::c_int {
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
        let mut slave: *mut client = (*ln).value as *mut client;
        if slave != except_me && (*slave).replstate == 7 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn unlinkClient(mut c: *mut client) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    if server.current_client == c {
        server.current_client = 0 as *mut client;
    }
    if !((*c).conn).is_null() {
        if !((*c).client_list_node).is_null() {
            let mut id: uint64_t = intrev64((*c).id);
            raxRemove(
                server.clients_index,
                &mut id as *mut uint64_t as *mut libc::c_uchar,
                core::mem::size_of::<uint64_t>() as libc::c_ulong,
                0 as *mut *mut libc::c_void,
            );
            listDelNode(server.clients, (*c).client_list_node);
            (*c).client_list_node = 0 as *mut listNode;
        }
        if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
            && (*c).replstate == 7 as libc::c_int && !(server.rdb_pipe_conns).is_null()
        {
            let mut i: libc::c_int = 0;
            i = 0 as libc::c_int;
            while i < server.rdb_pipe_numconns {
                if *(server.rdb_pipe_conns).offset(i as isize) == (*c).conn {
                    rdbPipeWriteHandlerConnRemoved((*c).conn);
                    let ref mut fresh6 = *(server.rdb_pipe_conns).offset(i as isize);
                    *fresh6 = 0 as *mut connection;
                    break;
                } else {
                    i += 1;
                }
            }
        }
        connClose((*c).conn);
        (*c).conn = 0 as *mut connection;
    }
    if (*c).flags & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong != 0 {
        ln = listSearchKey(server.clients_pending_write, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1511 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(server.clients_pending_write, ln);
        (*c).flags &= !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong;
    }
    if io_threads_op == 0 as libc::c_int {} else {
        _serverAssert(
            b"io_threads_op == IO_THREADS_OP_IDLE\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            1517 as libc::c_int,
        );
        unreachable!();
    };
    if !((*c).pending_read_list_node).is_null() {
        listDelNode(server.clients_pending_read, (*c).pending_read_list_node);
        (*c).pending_read_list_node = 0 as *mut listNode;
    }
    if (*c).flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
        ln = listSearchKey(server.unblocked_clients, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1528 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(server.unblocked_clients, ln);
        (*c).flags &= !((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 31 as libc::c_int
        != 0
    {
        disableTracking(c);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clearClientConnectionState(mut c: *mut client) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    if (*c).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
        ln = listSearchKey(server.monitors, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1546 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(server.monitors, ln);
        (*c).flags
            &= !((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    }
    if (*c).flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
    {} else {
        _serverAssert(
            b"!(c->flags &(CLIENT_SLAVE|CLIENT_MASTER))\0" as *const u8
                as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            1552 as libc::c_int,
        );
        unreachable!();
    };
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 31 as libc::c_int
        != 0
    {
        disableTracking(c);
    }
    selectDb(c, 0 as libc::c_int);
    (*c).resp = 2 as libc::c_int;
    clientSetDefaultAuth(c);
    moduleNotifyUserChanged(c);
    discardTransaction(c);
    pubsubUnsubscribeAllChannels(c, 0 as libc::c_int);
    pubsubUnsubscribeShardAllChannels(c, 0 as libc::c_int);
    pubsubUnsubscribeAllPatterns(c, 0 as libc::c_int);
    if !((*c).name).is_null() {
        decrRefCount((*c).name);
        (*c).name = 0 as *mut robj;
    }
    (*c).flags
        &= !((1 as libc::c_int) << 9 as libc::c_int
            | (1 as libc::c_int) << 17 as libc::c_int
            | (1 as libc::c_int) << 18 as libc::c_int
            | (1 as libc::c_int) << 22 as libc::c_int
            | (1 as libc::c_int) << 23 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn freeClient(mut c: *mut client) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    if (*c).flags & ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong != 0 {
        freeClientAsync(c);
        return;
    }
    if !((*c).conn).is_null() {
        moduleFireServerEvent(
            4 as libc::c_int as uint64_t,
            1 as libc::c_int,
            c as *mut libc::c_void,
        );
    }
    moduleNotifyUserChanged(c);
    if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
        ln = listSearchKey(server.clients_to_close, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1602 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(server.clients_to_close, ln);
    }
    if !(server.master).is_null()
        && (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Connection with master lost.\0" as *const u8 as *const libc::c_char,
            );
        }
        if (*c).flags as libc::c_ulonglong
            & ((1 as libc::c_ulonglong) << 39 as libc::c_int
                | ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulonglong) == 0
        {
            (*c).flags
                &= !((1 as libc::c_int) << 10 as libc::c_int
                    | (1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
            replicationCacheMaster(c);
            return;
        }
    }
    if getClientType(c) == 1 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Connection with replica %s lost.\0" as *const u8
                    as *const libc::c_char,
                replicationGetSlaveName(c),
            );
        }
    }
    sdsfree((*c).querybuf);
    (*c).querybuf = 0 as sds;
    if (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
        unblockClient(c);
    }
    dictRelease((*c).bpop.keys);
    unwatchAllKeys(c);
    listRelease((*c).watched_keys);
    pubsubUnsubscribeAllChannels(c, 0 as libc::c_int);
    pubsubUnsubscribeShardAllChannels(c, 0 as libc::c_int);
    pubsubUnsubscribeAllPatterns(c, 0 as libc::c_int);
    dictRelease((*c).pubsub_channels);
    listRelease((*c).pubsub_patterns);
    dictRelease((*c).pubsubshard_channels);
    listRelease((*c).reply);
    zfree((*c).buf as *mut libc::c_void);
    freeReplicaReferencedReplBuffer(c);
    freeClientArgv(c);
    freeClientOriginalArgv(c);
    if !((*c).deferred_reply_errors).is_null() {
        listRelease((*c).deferred_reply_errors);
    }
    unlinkClient(c);
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
        if server.saveparamslen == 0 as libc::c_int && (*c).replstate == 7 as libc::c_int
            && server.child_type == 1 as libc::c_int
            && server.rdb_child_type == 1 as libc::c_int
            && anyOtherSlaveWaitRdb(c) == 0 as libc::c_int
        {
            killRDBChild();
        }
        if (*c).replstate == 8 as libc::c_int {
            if (*c).repldbfd != -(1 as libc::c_int) {
                close((*c).repldbfd);
            }
            if !((*c).replpreamble).is_null() {
                sdsfree((*c).replpreamble);
            }
        }
        let mut l: *mut list = if (*c).flags
            & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0
        {
            server.monitors
        } else {
            server.slaves
        };
        ln = listSearchKey(l, c as *mut libc::c_void);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1684 as libc::c_int,
            );
            unreachable!();
        };
        listDelNode(l, ln);
        if getClientType(c) == 1 as libc::c_int
            && (*server.slaves).len == 0 as libc::c_int as libc::c_ulong
        {
            server.repl_no_slaves_since = server.unixtime as time_t;
        }
        refreshGoodSlavesCount();
        if (*c).replstate == 9 as libc::c_int {
            moduleFireServerEvent(
                6 as libc::c_int as uint64_t,
                1 as libc::c_int,
                0 as *mut libc::c_void,
            );
        }
    }
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        replicationHandleMasterDisconnection();
    }
    server
        .stat_clients_type_memory[(*c).last_memory_type
        as usize] = (server.stat_clients_type_memory[(*c).last_memory_type as usize]
        as libc::c_ulong)
        .wrapping_sub((*c).last_memory_usage) as size_t as size_t;
    if !((*c).mem_usage_bucket).is_null() {
        (*(*c).mem_usage_bucket)
            .mem_usage_sum = ((*(*c).mem_usage_bucket).mem_usage_sum as libc::c_ulong)
            .wrapping_sub((*c).last_memory_usage) as size_t as size_t;
        listDelNode((*(*c).mem_usage_bucket).clients, (*c).mem_usage_bucket_node);
    }
    if !((*c).name).is_null() {
        decrRefCount((*c).name);
    }
    freeClientMultiState(c);
    sdsfree((*c).peerid);
    sdsfree((*c).sockname);
    sdsfree((*c).slave_addr);
    zfree(c as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn freeClientAsync(mut c: *mut client) {
    if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
        || (*c).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0
    {
        return;
    }
    (*c).flags |= ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong;
    if server.io_threads_num == 1 as libc::c_int {
        listAddNodeTail(server.clients_to_close, c as *mut libc::c_void);
        return;
    }
    static mut async_free_queue_mutex: pthread_mutex_t = pthread_mutex_t {
        __data: {
            let mut init = __pthread_mutex_s {
                __lock: 0 as libc::c_int,
                __count: 0 as libc::c_int as libc::c_uint,
                __owner: 0 as libc::c_int,
                __nusers: 0 as libc::c_int as libc::c_uint,
                __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
                __spins: 0 as libc::c_int,
                __list: {
                    let mut init = __pthread_internal_list {
                        __prev: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                        __next: 0 as *const __pthread_internal_list
                            as *mut __pthread_internal_list,
                    };
                    init
                },
            };
            init
        },
    };
    pthread_mutex_lock(&mut async_free_queue_mutex);
    listAddNodeTail(server.clients_to_close, c as *mut libc::c_void);
    pthread_mutex_unlock(&mut async_free_queue_mutex);
}
#[no_mangle]
pub unsafe extern "C" fn logInvalidUseAndFreeClientAsync(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    let mut info: sds = sdscatvprintf(sdsempty(), fmt, ap.as_va_list());
    let mut client: sds = catClientInfoString(sdsempty(), c);
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"%s, disconnecting it: %s\0" as *const u8 as *const libc::c_char,
            info,
            client,
        );
    }
    sdsfree(info);
    sdsfree(client);
    freeClientAsync(c);
}
#[no_mangle]
pub unsafe extern "C" fn beforeNextClient(mut c: *mut client) -> libc::c_int {
    if io_threads_op != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if !c.is_null()
        && (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
    {
        freeClient(c);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn freeClientsInAsyncFreeQueue() -> libc::c_int {
    let mut freed: libc::c_int = 0 as libc::c_int;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients_to_close, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        if (*c).flags & ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong != 0 {
            continue;
        }
        (*c).flags &= !((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong;
        freeClient(c);
        listDelNode(server.clients_to_close, ln);
        freed += 1;
    }
    return freed;
}
#[no_mangle]
pub unsafe extern "C" fn lookupClientByID(mut id: uint64_t) -> *mut client {
    id = intrev64(id);
    let mut c: *mut client = raxFind(
        server.clients_index,
        &mut id as *mut uint64_t as *mut libc::c_uchar,
        core::mem::size_of::<uint64_t>() as libc::c_ulong,
    ) as *mut client;
    return if c == raxNotFound as *mut client { 0 as *mut client } else { c };
}
unsafe extern "C" fn _writevToClient(
    mut c: *mut client,
    mut nwritten: *mut ssize_t,
) -> libc::c_int {
    let mut iov: [iovec; 1024] = [iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    }; 1024];
    let mut iovcnt: libc::c_int = 0 as libc::c_int;
    let mut iov_bytes_len: size_t = 0 as libc::c_int as size_t;
    if (*c).bufpos > 0 as libc::c_int {
        iov[iovcnt as usize]
            .iov_base = ((*c).buf).offset((*c).sentlen as isize) as *mut libc::c_void;
        iov[iovcnt as usize]
            .iov_len = ((*c).bufpos as libc::c_ulong).wrapping_sub((*c).sentlen);
        let fresh7 = iovcnt;
        iovcnt = iovcnt + 1;
        iov_bytes_len = (iov_bytes_len as libc::c_ulong)
            .wrapping_add(iov[fresh7 as usize].iov_len) as size_t as size_t;
    }
    let mut offset: size_t = if (*c).bufpos > 0 as libc::c_int {
        0 as libc::c_int as libc::c_ulong
    } else {
        (*c).sentlen
    };
    let mut iter: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut next: *mut listNode = 0 as *mut listNode;
    let mut o: *mut clientReplyBlock = 0 as *mut clientReplyBlock;
    listRewind((*c).reply, &mut iter);
    loop {
        next = listNext(&mut iter);
        if !(!next.is_null() && iovcnt < 1024 as libc::c_int
            && iov_bytes_len
                < (1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong)
        {
            break;
        }
        o = (*next).value as *mut clientReplyBlock;
        if (*o).used == 0 as libc::c_int as libc::c_ulong {
            (*c)
                .reply_bytes = ((*c).reply_bytes)
                .wrapping_sub((*o).size as libc::c_ulonglong);
            listDelNode((*c).reply, next);
            offset = 0 as libc::c_int as size_t;
        } else {
            iov[iovcnt as usize]
                .iov_base = ((*o).buf).as_mut_ptr().offset(offset as isize)
                as *mut libc::c_void;
            iov[iovcnt as usize].iov_len = ((*o).used).wrapping_sub(offset);
            let fresh8 = iovcnt;
            iovcnt = iovcnt + 1;
            iov_bytes_len = (iov_bytes_len as libc::c_ulong)
                .wrapping_add(iov[fresh8 as usize].iov_len) as size_t as size_t;
            offset = 0 as libc::c_int as size_t;
        }
    }
    if iovcnt == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    *nwritten = connWritev((*c).conn, iov.as_mut_ptr(), iovcnt) as ssize_t;
    if *nwritten <= 0 as libc::c_int as libc::c_long {
        return -(1 as libc::c_int);
    }
    let mut remaining: ssize_t = *nwritten;
    if (*c).bufpos > 0 as libc::c_int {
        let mut buf_len: libc::c_int = ((*c).bufpos as libc::c_ulong)
            .wrapping_sub((*c).sentlen) as libc::c_int;
        (*c)
            .sentlen = ((*c).sentlen as libc::c_ulong)
            .wrapping_add(remaining as libc::c_ulong) as size_t as size_t;
        if remaining >= buf_len as libc::c_long {
            (*c).bufpos = 0 as libc::c_int;
            (*c).sentlen = 0 as libc::c_int as size_t;
        }
        remaining -= buf_len as libc::c_long;
    }
    listRewind((*c).reply, &mut iter);
    while remaining > 0 as libc::c_int as libc::c_long {
        next = listNext(&mut iter);
        o = (*next).value as *mut clientReplyBlock;
        if remaining < ((*o).used).wrapping_sub((*c).sentlen) as ssize_t {
            (*c)
                .sentlen = ((*c).sentlen as libc::c_ulong)
                .wrapping_add(remaining as libc::c_ulong) as size_t as size_t;
            break;
        } else {
            remaining -= ((*o).used).wrapping_sub((*c).sentlen) as ssize_t;
            (*c)
                .reply_bytes = ((*c).reply_bytes)
                .wrapping_sub((*o).size as libc::c_ulonglong);
            listDelNode((*c).reply, next);
            (*c).sentlen = 0 as libc::c_int as size_t;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _writeToClient(
    mut c: *mut client,
    mut nwritten: *mut ssize_t,
) -> libc::c_int {
    *nwritten = 0 as libc::c_int as ssize_t;
    if getClientType(c) == 1 as libc::c_int {
        if (*c).bufpos == 0 as libc::c_int
            && (*(*c).reply).len == 0 as libc::c_int as libc::c_ulong
        {} else {
            _serverAssert(
                b"c->bufpos == 0 && listLength(c->reply) == 0\0" as *const u8
                    as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1895 as libc::c_int,
            );
            unreachable!();
        };
        let mut o: *mut replBufBlock = (*(*c).ref_repl_buf_node).value
            as *mut replBufBlock;
        if (*o).used >= (*c).ref_block_pos {} else {
            _serverAssert(
                b"o->used >= c->ref_block_pos\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                1898 as libc::c_int,
            );
            unreachable!();
        };
        if (*o).used > (*c).ref_block_pos {
            *nwritten = connWrite(
                (*c).conn,
                ((*o).buf).as_mut_ptr().offset((*c).ref_block_pos as isize)
                    as *const libc::c_void,
                ((*o).used).wrapping_sub((*c).ref_block_pos),
            ) as ssize_t;
            if *nwritten <= 0 as libc::c_int as libc::c_long {
                return -(1 as libc::c_int);
            }
            (*c)
                .ref_block_pos = ((*c).ref_block_pos as libc::c_ulong)
                .wrapping_add(*nwritten as libc::c_ulong) as size_t as size_t;
        }
        let mut next: *mut listNode = (*(*c).ref_repl_buf_node).next;
        if !next.is_null() && (*c).ref_block_pos == (*o).used {
            (*o).refcount -= 1;
            let ref mut fresh9 = (*((*next).value as *mut replBufBlock)).refcount;
            *fresh9 += 1;
            (*c).ref_repl_buf_node = next;
            (*c).ref_block_pos = 0 as libc::c_int as size_t;
            incrementalTrimReplicationBacklog(64 as libc::c_int as size_t);
        }
        return 0 as libc::c_int;
    }
    if (*(*c).reply).len > 0 as libc::c_int as libc::c_ulong {
        let mut ret: libc::c_int = _writevToClient(c, nwritten);
        if ret != 0 as libc::c_int {
            return ret;
        }
        if (*(*c).reply).len == 0 as libc::c_int as libc::c_ulong {
            if (*c).reply_bytes == 0 as libc::c_int as libc::c_ulonglong {} else {
                _serverAssert(
                    b"c->reply_bytes == 0\0" as *const u8 as *const libc::c_char,
                    b"networking.c\0" as *const u8 as *const libc::c_char,
                    1928 as libc::c_int,
                );
                unreachable!();
            };
        }
    } else if (*c).bufpos > 0 as libc::c_int {
        *nwritten = connWrite(
            (*c).conn,
            ((*c).buf).offset((*c).sentlen as isize) as *const libc::c_void,
            ((*c).bufpos as libc::c_ulong).wrapping_sub((*c).sentlen),
        ) as ssize_t;
        if *nwritten <= 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        (*c)
            .sentlen = ((*c).sentlen as libc::c_ulong)
            .wrapping_add(*nwritten as libc::c_ulong) as size_t as size_t;
        if (*c).sentlen as libc::c_int == (*c).bufpos {
            (*c).bufpos = 0 as libc::c_int;
            (*c).sentlen = 0 as libc::c_int as size_t;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn writeToClient(
    mut c: *mut client,
    mut handler_installed: libc::c_int,
) -> libc::c_int {
    let fresh10 = &mut server.stat_total_writes_processed;
    let fresh11 = 1 as libc::c_int as libc::c_longlong;
    core::intrinsics::atomic_xadd_relaxed(fresh10, fresh11) + fresh11;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    let mut totwritten: ssize_t = 0 as libc::c_int as ssize_t;
    while clientHasPendingReplies(c) != 0 {
        let mut ret: libc::c_int = _writeToClient(c, &mut nwritten);
        if ret == -(1 as libc::c_int) {
            break;
        }
        totwritten += nwritten;
        if totwritten > (1024 as libc::c_int * 64 as libc::c_int) as libc::c_long
            && (server.maxmemory == 0 as libc::c_int as libc::c_ulonglong
                || (zmalloc_used_memory() as libc::c_ulonglong) < server.maxmemory)
            && (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
                == 0
        {
            break;
        }
    }
    if getClientType(c) == 1 as libc::c_int {
        let fresh12 = &mut server.stat_net_repl_output_bytes;
        let fresh13 = totwritten as libc::c_longlong;
        core::intrinsics::atomic_xadd_relaxed(fresh12, fresh13) + fresh13;
    } else {
        let fresh14 = &mut server.stat_net_output_bytes;
        let fresh15 = totwritten as libc::c_longlong;
        core::intrinsics::atomic_xadd_relaxed(fresh14, fresh15) + fresh15;
    }
    if nwritten == -(1 as libc::c_int) as libc::c_long {
        if connGetState((*c).conn) != CONN_STATE_CONNECTED as libc::c_int {
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Error writing to client: %s\0" as *const u8 as *const libc::c_char,
                    connGetLastError((*c).conn),
                );
            }
            freeClientAsync(c);
            return -(1 as libc::c_int);
        }
    }
    if totwritten > 0 as libc::c_int as libc::c_long {
        if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0 {
            (*c).lastinteraction = server.unixtime as time_t;
        }
    }
    if clientHasPendingReplies(c) == 0 {
        (*c).sentlen = 0 as libc::c_int as size_t;
        if handler_installed != 0 {
            if io_threads_op == 0 as libc::c_int {} else {
                _serverAssert(
                    b"io_threads_op == IO_THREADS_OP_IDLE\0" as *const u8
                        as *const libc::c_char,
                    b"networking.c\0" as *const u8 as *const libc::c_char,
                    2009 as libc::c_int,
                );
                unreachable!();
            };
            connSetWriteHandler((*c).conn, None);
        }
        if (*c).flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
            freeClientAsync(c);
            return -(1 as libc::c_int);
        }
    }
    if io_threads_op == 0 as libc::c_int {
        updateClientMemUsageAndBucket(c);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sendReplyToClient(mut conn: *mut connection) {
    let mut c: *mut client = connGetPrivateData(conn) as *mut client;
    writeToClient(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn handleClientsWithPendingWrites() -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut processed: libc::c_int = (*server.clients_pending_write).len as libc::c_int;
    listRewind(server.clients_pending_write, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        (*c).flags &= !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong;
        listDelNode(server.clients_pending_write, ln);
        if (*c).flags & ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong != 0 {
            continue;
        }
        if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
            continue;
        }
        if writeToClient(c, 0 as libc::c_int) == -(1 as libc::c_int) {
            continue;
        }
        if clientHasPendingReplies(c) != 0 {
            installClientWriteHandler(c);
        }
    }
    return processed;
}
#[no_mangle]
pub unsafe extern "C" fn resetClient(mut c: *mut client) {
    let mut prevcmd: Option::<redisCommandProc> = if !((*c).cmd).is_null() {
        (*(*c).cmd).proc_0
    } else {
        None
    };
    freeClientArgv(c);
    (*c).cur_script = 0 as *mut dictEntry;
    (*c).reqtype = 0 as libc::c_int;
    (*c).multibulklen = 0 as libc::c_int;
    (*c).bulklen = -(1 as libc::c_int) as libc::c_long;
    (*c).slot = -(1 as libc::c_int);
    if !((*c).deferred_reply_errors).is_null() {
        listRelease((*c).deferred_reply_errors);
    }
    (*c).deferred_reply_errors = 0 as *mut list;
    if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0
        && prevcmd != Some(askingCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        (*c).flags &= !((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong;
    }
    if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0
        && prevcmd != Some(clientCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            & !((1 as libc::c_ulonglong) << 36 as libc::c_int)) as uint64_t;
    }
    (*c).flags &= !((1 as libc::c_int) << 24 as libc::c_int) as libc::c_ulong;
    if (*c).flags & ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_ulong != 0 {
        (*c).flags |= ((1 as libc::c_int) << 24 as libc::c_int) as libc::c_ulong;
        (*c).flags &= !((1 as libc::c_int) << 23 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn protectClient(mut c: *mut client) {
    (*c).flags |= ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong;
    if !((*c).conn).is_null() {
        connSetReadHandler((*c).conn, None);
        connSetWriteHandler((*c).conn, None);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unprotectClient(mut c: *mut client) {
    if (*c).flags & ((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong != 0 {
        (*c).flags &= !((1 as libc::c_int) << 28 as libc::c_int) as libc::c_ulong;
        if !((*c).conn).is_null() {
            connSetReadHandler(
                (*c).conn,
                Some(readQueryFromClient as unsafe extern "C" fn(*mut connection) -> ()),
            );
            if clientHasPendingReplies(c) != 0 {
                putClientInPendingWriteQueue(c);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn processInlineBuffer(mut c: *mut client) -> libc::c_int {
    let mut newline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut argc: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut linefeed_chars: libc::c_int = 1 as libc::c_int;
    let mut argv: *mut sds = 0 as *mut sds;
    let mut aux: sds = 0 as *mut libc::c_char;
    let mut querylen: size_t = 0;
    newline = strchr(
        ((*c).querybuf).offset((*c).qb_pos as isize) as *const libc::c_char,
        '\n' as i32,
    );
    if newline.is_null() {
        if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
            > (1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong
        {
            addReplyError(
                c,
                b"Protocol error: too big inline request\0" as *const u8
                    as *const libc::c_char,
            );
            setProtocolError(
                b"too big inline request\0" as *const u8 as *const libc::c_char,
                c,
            );
        }
        return -(1 as libc::c_int);
    }
    if newline != ((*c).querybuf).offset((*c).qb_pos as isize)
        && *newline.offset(-(1 as libc::c_int as isize)) as libc::c_int == '\r' as i32
    {
        newline = newline.offset(-1);
        linefeed_chars += 1;
    }
    querylen = newline.offset_from(((*c).querybuf).offset((*c).qb_pos as isize))
        as libc::c_long as size_t;
    aux = sdsnewlen(
        ((*c).querybuf).offset((*c).qb_pos as isize) as *const libc::c_void,
        querylen,
    );
    argv = sdssplitargs(aux as *const libc::c_char, &mut argc);
    sdsfree(aux);
    if argv.is_null() {
        addReplyError(
            c,
            b"Protocol error: unbalanced quotes in request\0" as *const u8
                as *const libc::c_char,
        );
        setProtocolError(
            b"unbalanced quotes in inline request\0" as *const u8 as *const libc::c_char,
            c,
        );
        return -(1 as libc::c_int);
    }
    if querylen == 0 as libc::c_int as libc::c_ulong
        && getClientType(c) == 1 as libc::c_int
    {
        (*c).repl_ack_time = server.unixtime as libc::c_longlong;
    }
    if querylen != 0 as libc::c_int as libc::c_ulong
        && (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
    {
        sdsfreesplitres(argv, argc);
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"WARNING: Receiving inline protocol from master, master stream corruption? Closing the master connection and discarding the cached master.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        setProtocolError(
            b"Master using the inline protocol. Desync?\0" as *const u8
                as *const libc::c_char,
            c,
        );
        return -(1 as libc::c_int);
    }
    (*c)
        .qb_pos = ((*c).qb_pos as libc::c_ulong)
        .wrapping_add(querylen.wrapping_add(linefeed_chars as libc::c_ulong)) as size_t
        as size_t;
    if argc != 0 {
        if !((*c).argv).is_null() {
            zfree((*c).argv as *mut libc::c_void);
        }
        (*c).argv_len = argc;
        (*c)
            .argv = zmalloc(
            (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                .wrapping_mul((*c).argv_len as libc::c_ulong),
        ) as *mut *mut robj;
        (*c).argv_len_sum = 0 as libc::c_int as size_t;
    }
    (*c).argc = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < argc {
        let ref mut fresh16 = *((*c).argv).offset((*c).argc as isize);
        *fresh16 = createObject(
            0 as libc::c_int,
            *argv.offset(j as isize) as *mut libc::c_void,
        );
        (*c).argc += 1;
        (*c)
            .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
            .wrapping_add(sdslen(*argv.offset(j as isize))) as size_t as size_t;
        j += 1;
    }
    zfree(argv as *mut libc::c_void);
    return 0 as libc::c_int;
}
unsafe extern "C" fn setProtocolError(
    mut errstr: *const libc::c_char,
    mut c: *mut client,
) {
    if server.verbosity <= 1 as libc::c_int
        || (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
    {
        let mut client: sds = catClientInfoString(sdsempty(), c);
        let mut buf: [libc::c_char; 256] = [0; 256];
        if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
            < 128 as libc::c_int as libc::c_ulong
        {
            snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"Query buffer during protocol error: '%s'\0" as *const u8
                    as *const libc::c_char,
                ((*c).querybuf).offset((*c).qb_pos as isize),
            );
        } else {
            snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
                b"Query buffer during protocol error: '%.*s' (... more %zu bytes ...) '%.*s'\0"
                    as *const u8 as *const libc::c_char,
                128 as libc::c_int / 2 as libc::c_int,
                ((*c).querybuf).offset((*c).qb_pos as isize),
                (sdslen((*c).querybuf))
                    .wrapping_sub((*c).qb_pos)
                    .wrapping_sub(128 as libc::c_int as libc::c_ulong),
                128 as libc::c_int / 2 as libc::c_int,
                ((*c).querybuf)
                    .offset(sdslen((*c).querybuf) as isize)
                    .offset(-((128 as libc::c_int / 2 as libc::c_int) as isize)),
            );
        }
        let mut p: *mut libc::c_char = buf.as_mut_ptr();
        while *p as libc::c_int != '\0' as i32 {
            if *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISprint as libc::c_int as libc::c_ushort as libc::c_int == 0
            {
                *p = '.' as i32 as libc::c_char;
            }
            p = p.offset(1);
        }
        let mut loglevel: libc::c_int = if (*c).flags
            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
        {
            3 as libc::c_int
        } else {
            1 as libc::c_int
        };
        if !((loglevel & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                loglevel,
                b"Protocol error (%s) from client: %s. %s\0" as *const u8
                    as *const libc::c_char,
                errstr,
                client,
                buf.as_mut_ptr(),
            );
        }
        sdsfree(client);
    }
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        | (((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 39 as libc::c_int)) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn processMultibulkBuffer(mut c: *mut client) -> libc::c_int {
    let mut newline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ok: libc::c_int = 0;
    let mut ll: libc::c_longlong = 0;
    if (*c).multibulklen == 0 as libc::c_int {
        if (*c).argc == 0 as libc::c_int {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"c->argc == 0\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                2267 as libc::c_int,
            );
            unreachable!();
        };
        newline = strchr(
            ((*c).querybuf).offset((*c).qb_pos as isize) as *const libc::c_char,
            '\r' as i32,
        );
        if newline.is_null() {
            if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
                > (1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong
            {
                addReplyError(
                    c,
                    b"Protocol error: too big mbulk count string\0" as *const u8
                        as *const libc::c_char,
                );
                setProtocolError(
                    b"too big mbulk count string\0" as *const u8 as *const libc::c_char,
                    c,
                );
            }
            return -(1 as libc::c_int);
        }
        if newline.offset_from(((*c).querybuf).offset((*c).qb_pos as isize))
            as libc::c_long
            > (sdslen((*c).querybuf))
                .wrapping_sub((*c).qb_pos)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong) as ssize_t
        {
            return -(1 as libc::c_int);
        }
        if *((*c).querybuf).offset((*c).qb_pos as isize) as libc::c_int == '*' as i32
        {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"c->querybuf[c->qb_pos] == '*'\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                2285 as libc::c_int,
            );
            unreachable!();
        };
        ok = string2ll(
            ((*c).querybuf)
                .offset(1 as libc::c_int as isize)
                .offset((*c).qb_pos as isize) as *const libc::c_char,
            newline
                .offset_from(
                    ((*c).querybuf)
                        .offset(1 as libc::c_int as isize)
                        .offset((*c).qb_pos as isize),
                ) as libc::c_long as size_t,
            &mut ll,
        );
        if ok == 0 || ll > 2147483647 as libc::c_int as libc::c_longlong {
            addReplyError(
                c,
                b"Protocol error: invalid multibulk length\0" as *const u8
                    as *const libc::c_char,
            );
            setProtocolError(
                b"invalid mbulk count\0" as *const u8 as *const libc::c_char,
                c,
            );
            return -(1 as libc::c_int);
        } else {
            if ll > 10 as libc::c_int as libc::c_longlong && authRequired(c) != 0 {
                addReplyError(
                    c,
                    b"Protocol error: unauthenticated multibulk length\0" as *const u8
                        as *const libc::c_char,
                );
                setProtocolError(
                    b"unauth mbulk count\0" as *const u8 as *const libc::c_char,
                    c,
                );
                return -(1 as libc::c_int);
            }
        }
        (*c)
            .qb_pos = (newline.offset_from((*c).querybuf) as libc::c_long
            + 2 as libc::c_int as libc::c_long) as size_t;
        if ll <= 0 as libc::c_int as libc::c_longlong {
            return 0 as libc::c_int;
        }
        (*c).multibulklen = ll as libc::c_int;
        if !((*c).argv).is_null() {
            zfree((*c).argv as *mut libc::c_void);
        }
        (*c)
            .argv_len = if (*c).multibulklen < 1024 as libc::c_int {
            (*c).multibulklen
        } else {
            1024 as libc::c_int
        };
        (*c)
            .argv = zmalloc(
            (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                .wrapping_mul((*c).argv_len as libc::c_ulong),
        ) as *mut *mut robj;
        (*c).argv_len_sum = 0 as libc::c_int as size_t;
    }
    if (*c).multibulklen > 0 as libc::c_int {} else {
        _serverAssertWithInfo(
            c,
            0 as *const robj,
            b"c->multibulklen > 0\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            2310 as libc::c_int,
        );
        unreachable!();
    };
    while (*c).multibulklen != 0 {
        if (*c).bulklen == -(1 as libc::c_int) as libc::c_long {
            newline = strchr(
                ((*c).querybuf).offset((*c).qb_pos as isize) as *const libc::c_char,
                '\r' as i32,
            );
            if newline.is_null() {
                if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
                    > (1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong
                {
                    addReplyError(
                        c,
                        b"Protocol error: too big bulk count string\0" as *const u8
                            as *const libc::c_char,
                    );
                    setProtocolError(
                        b"too big bulk count string\0" as *const u8
                            as *const libc::c_char,
                        c,
                    );
                    return -(1 as libc::c_int);
                }
                break;
            } else {
                if newline.offset_from(((*c).querybuf).offset((*c).qb_pos as isize))
                    as libc::c_long
                    > (sdslen((*c).querybuf))
                        .wrapping_sub((*c).qb_pos)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as ssize_t
                {
                    break;
                }
                if *((*c).querybuf).offset((*c).qb_pos as isize) as libc::c_int
                    != '$' as i32
                {
                    addReplyErrorFormat(
                        c,
                        b"Protocol error: expected '$', got '%c'\0" as *const u8
                            as *const libc::c_char,
                        *((*c).querybuf).offset((*c).qb_pos as isize) as libc::c_int,
                    );
                    setProtocolError(
                        b"expected $ but got something else\0" as *const u8
                            as *const libc::c_char,
                        c,
                    );
                    return -(1 as libc::c_int);
                }
                ok = string2ll(
                    ((*c).querybuf)
                        .offset((*c).qb_pos as isize)
                        .offset(1 as libc::c_int as isize) as *const libc::c_char,
                    newline
                        .offset_from(
                            ((*c).querybuf)
                                .offset((*c).qb_pos as isize)
                                .offset(1 as libc::c_int as isize),
                        ) as libc::c_long as size_t,
                    &mut ll,
                );
                if ok == 0 || ll < 0 as libc::c_int as libc::c_longlong
                    || (*c).flags
                        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
                        && ll > server.proto_max_bulk_len
                {
                    addReplyError(
                        c,
                        b"Protocol error: invalid bulk length\0" as *const u8
                            as *const libc::c_char,
                    );
                    setProtocolError(
                        b"invalid bulk length\0" as *const u8 as *const libc::c_char,
                        c,
                    );
                    return -(1 as libc::c_int);
                } else {
                    if ll > 16384 as libc::c_int as libc::c_longlong
                        && authRequired(c) != 0
                    {
                        addReplyError(
                            c,
                            b"Protocol error: unauthenticated bulk length\0" as *const u8
                                as *const libc::c_char,
                        );
                        setProtocolError(
                            b"unauth bulk length\0" as *const u8 as *const libc::c_char,
                            c,
                        );
                        return -(1 as libc::c_int);
                    }
                }
                (*c)
                    .qb_pos = (newline.offset_from((*c).querybuf) as libc::c_long
                    + 2 as libc::c_int as libc::c_long) as size_t;
                if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
                    == 0
                    && ll
                        >= (1024 as libc::c_int * 32 as libc::c_int) as libc::c_longlong
                {
                    if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
                        <= (ll as size_t).wrapping_add(2 as libc::c_int as libc::c_ulong)
                    {
                        sdsrange(
                            (*c).querybuf,
                            (*c).qb_pos as ssize_t,
                            -(1 as libc::c_int) as ssize_t,
                        );
                        (*c).qb_pos = 0 as libc::c_int as size_t;
                        (*c)
                            .querybuf = sdsMakeRoomForNonGreedy(
                            (*c).querybuf,
                            ((ll + 2 as libc::c_int as libc::c_longlong)
                                as libc::c_ulonglong)
                                .wrapping_sub(sdslen((*c).querybuf) as libc::c_ulonglong)
                                as size_t,
                        );
                    }
                }
                (*c).bulklen = ll as libc::c_long;
            }
        }
        if (sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)
            < ((*c).bulklen + 2 as libc::c_int as libc::c_long) as size_t
        {
            break;
        }
        if (*c).argc >= (*c).argv_len {
            (*c)
                .argv_len = if (if (*c).argv_len
                < 2147483647 as libc::c_int / 2 as libc::c_int
            {
                (*c).argv_len * 2 as libc::c_int
            } else {
                2147483647 as libc::c_int
            }) < (*c).argc + (*c).multibulklen
            {
                if (*c).argv_len < 2147483647 as libc::c_int / 2 as libc::c_int {
                    (*c).argv_len * 2 as libc::c_int
                } else {
                    2147483647 as libc::c_int
                }
            } else {
                (*c).argc + (*c).multibulklen
            };
            (*c)
                .argv = zrealloc(
                (*c).argv as *mut libc::c_void,
                (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                    .wrapping_mul((*c).argv_len as libc::c_ulong),
            ) as *mut *mut robj;
        }
        if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
            && (*c).qb_pos == 0 as libc::c_int as libc::c_ulong
            && (*c).bulklen >= (1024 as libc::c_int * 32 as libc::c_int) as libc::c_long
            && sdslen((*c).querybuf)
                == ((*c).bulklen + 2 as libc::c_int as libc::c_long) as size_t
        {
            let fresh17 = (*c).argc;
            (*c).argc = (*c).argc + 1;
            let ref mut fresh18 = *((*c).argv).offset(fresh17 as isize);
            *fresh18 = createObject(
                0 as libc::c_int,
                (*c).querybuf as *mut libc::c_void,
            );
            (*c)
                .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
                .wrapping_add((*c).bulklen as libc::c_ulong) as size_t as size_t;
            sdsIncrLen((*c).querybuf, -(2 as libc::c_int) as ssize_t);
            (*c)
                .querybuf = sdsnewlen(
                SDS_NOINIT as *const libc::c_void,
                ((*c).bulklen + 2 as libc::c_int as libc::c_long) as size_t,
            );
            sdsclear((*c).querybuf);
        } else {
            let fresh19 = (*c).argc;
            (*c).argc = (*c).argc + 1;
            let ref mut fresh20 = *((*c).argv).offset(fresh19 as isize);
            *fresh20 = createStringObject(
                ((*c).querybuf).offset((*c).qb_pos as isize) as *const libc::c_char,
                (*c).bulklen as size_t,
            );
            (*c)
                .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
                .wrapping_add((*c).bulklen as libc::c_ulong) as size_t as size_t;
            (*c)
                .qb_pos = ((*c).qb_pos as libc::c_ulong)
                .wrapping_add(
                    ((*c).bulklen + 2 as libc::c_int as libc::c_long) as libc::c_ulong,
                ) as size_t as size_t;
        }
        (*c).bulklen = -(1 as libc::c_int) as libc::c_long;
        (*c).multibulklen -= 1;
    }
    if (*c).multibulklen == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn commandProcessed(mut c: *mut client) {
    if (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    resetClient(c);
    let mut prev_offset: libc::c_longlong = (*c).reploff;
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
        && (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0
    {
        (*c)
            .reploff = ((*c).read_reploff as libc::c_ulonglong)
            .wrapping_sub(sdslen((*c).querybuf) as libc::c_ulonglong)
            .wrapping_add((*c).qb_pos as libc::c_ulonglong) as libc::c_longlong;
    }
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        let mut applied: libc::c_longlong = (*c).reploff - prev_offset;
        if applied != 0 {
            replicationFeedStreamFromMasterStream(
                ((*c).querybuf).offset((*c).repl_applied as isize),
                applied as size_t,
            );
            (*c).repl_applied += applied;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn processCommandAndResetClient(
    mut c: *mut client,
) -> libc::c_int {
    let mut deadclient: libc::c_int = 0 as libc::c_int;
    let mut old_client: *mut client = server.current_client;
    server.current_client = c;
    if processCommand(c) == 0 as libc::c_int {
        commandProcessed(c);
        updateClientMemUsageAndBucket(c);
    }
    if (server.current_client).is_null() {
        deadclient = 1 as libc::c_int;
    }
    server.current_client = old_client;
    return if deadclient != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn processPendingCommandAndInputBuffer(
    mut c: *mut client,
) -> libc::c_int {
    if (*c).flags & ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong != 0 {
        (*c).flags &= !((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong;
        if processCommandAndResetClient(c) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if !((*c).querybuf).is_null()
        && sdslen((*c).querybuf) > 0 as libc::c_int as libc::c_ulong
    {
        return processInputBuffer(c);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn processInputBuffer(mut c: *mut client) -> libc::c_int {
    while (*c).qb_pos < sdslen((*c).querybuf) {
        if (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
            break;
        }
        if (*c).flags & ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong != 0 {
            break;
        }
        if isInsideYieldingLongCommand() != 0
            && (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong
                != 0
        {
            break;
        }
        if (*c).flags
            & ((1 as libc::c_int) << 6 as libc::c_int
                | (1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
        {
            break;
        }
        if (*c).reqtype == 0 {
            if *((*c).querybuf).offset((*c).qb_pos as isize) as libc::c_int == '*' as i32
            {
                (*c).reqtype = 2 as libc::c_int;
            } else {
                (*c).reqtype = 1 as libc::c_int;
            }
        }
        if (*c).reqtype == 1 as libc::c_int {
            if processInlineBuffer(c) != 0 as libc::c_int {
                break;
            }
        } else if (*c).reqtype == 2 as libc::c_int {
            if processMultibulkBuffer(c) != 0 as libc::c_int {
                break;
            }
        } else {
            _serverPanic(
                b"networking.c\0" as *const u8 as *const libc::c_char,
                2556 as libc::c_int,
                b"Unknown request type\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
        if (*c).argc == 0 as libc::c_int {
            resetClient(c);
        } else if io_threads_op != 0 as libc::c_int {
            if io_threads_op == 1 as libc::c_int {} else {
                _serverAssert(
                    b"io_threads_op == IO_THREADS_OP_READ\0" as *const u8
                        as *const libc::c_char,
                    b"networking.c\0" as *const u8 as *const libc::c_char,
                    2567 as libc::c_int,
                );
                unreachable!();
            };
            (*c).flags |= ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong;
            break;
        } else if processCommandAndResetClient(c) == -(1 as libc::c_int) {
            return -(1 as libc::c_int)
        }
    }
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        if (*c).repl_applied != 0 {
            sdsrange(
                (*c).querybuf,
                (*c).repl_applied as ssize_t,
                -(1 as libc::c_int) as ssize_t,
            );
            (*c)
                .qb_pos = ((*c).qb_pos as libc::c_ulonglong)
                .wrapping_sub((*c).repl_applied as libc::c_ulonglong) as size_t
                as size_t;
            (*c).repl_applied = 0 as libc::c_int as libc::c_longlong;
        }
    } else if (*c).qb_pos != 0 {
        sdsrange((*c).querybuf, (*c).qb_pos as ssize_t, -(1 as libc::c_int) as ssize_t);
        (*c).qb_pos = 0 as libc::c_int as size_t;
    }
    if io_threads_op == 0 as libc::c_int {
        updateClientMemUsageAndBucket(c);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn readQueryFromClient(mut conn: *mut connection) {
    let mut c: *mut client = connGetPrivateData(conn) as *mut client;
    let mut nread: libc::c_int = 0;
    let mut big_arg: libc::c_int = 0 as libc::c_int;
    let mut qblen: size_t = 0;
    let mut readlen: size_t = 0;
    if postponeClientRead(c) != 0 {
        return;
    }
    let fresh21 = &mut server.stat_total_reads_processed;
    let fresh22 = 1 as libc::c_int as libc::c_longlong;
    core::intrinsics::atomic_xadd_relaxed(fresh21, fresh22) + fresh22;
    readlen = (1024 as libc::c_int * 16 as libc::c_int) as size_t;
    if (*c).reqtype == 2 as libc::c_int && (*c).multibulklen != 0
        && (*c).bulklen != -(1 as libc::c_int) as libc::c_long
        && (*c).bulklen >= (1024 as libc::c_int * 32 as libc::c_int) as libc::c_long
    {
        let mut remaining: ssize_t = (((*c).bulklen + 2 as libc::c_int as libc::c_long)
            as size_t)
            .wrapping_sub((sdslen((*c).querybuf)).wrapping_sub((*c).qb_pos)) as ssize_t;
        big_arg = 1 as libc::c_int;
        if remaining > 0 as libc::c_int as libc::c_long {
            readlen = remaining as size_t;
        }
        if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
            && readlen < (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong
        {
            readlen = (1024 as libc::c_int * 16 as libc::c_int) as size_t;
        }
    }
    qblen = sdslen((*c).querybuf);
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
        && (big_arg != 0
            || sdsalloc((*c).querybuf)
                < (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong)
    {
        (*c).querybuf = sdsMakeRoomForNonGreedy((*c).querybuf, readlen);
    } else {
        (*c).querybuf = sdsMakeRoomFor((*c).querybuf, readlen);
        readlen = sdsavail((*c).querybuf);
    }
    nread = connRead(
        (*c).conn,
        ((*c).querybuf).offset(qblen as isize) as *mut libc::c_void,
        readlen,
    );
    if nread == -(1 as libc::c_int) {
        if connGetState(conn) == CONN_STATE_CONNECTED as libc::c_int {
            return
        } else {
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Reading from client: %s\0" as *const u8 as *const libc::c_char,
                    connGetLastError((*c).conn),
                );
            }
            freeClientAsync(c);
        }
    } else if nread == 0 as libc::c_int {
        if server.verbosity <= 1 as libc::c_int {
            let mut info: sds = catClientInfoString(sdsempty(), c);
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Client closed connection %s\0" as *const u8 as *const libc::c_char,
                    info,
                );
            }
            sdsfree(info);
        }
        freeClientAsync(c);
    } else {
        sdsIncrLen((*c).querybuf, nread as ssize_t);
        qblen = sdslen((*c).querybuf);
        if (*c).querybuf_peak < qblen {
            (*c).querybuf_peak = qblen;
        }
        (*c).lastinteraction = server.unixtime as time_t;
        if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
            (*c).read_reploff += nread as libc::c_longlong;
            let fresh23 = &mut server.stat_net_repl_input_bytes;
            let fresh24 = nread as libc::c_longlong;
            core::intrinsics::atomic_xadd_relaxed(fresh23, fresh24) + fresh24;
        } else {
            let fresh25 = &mut server.stat_net_input_bytes;
            let fresh26 = nread as libc::c_longlong;
            core::intrinsics::atomic_xadd_relaxed(fresh25, fresh26) + fresh26;
        }
        if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong == 0
            && sdslen((*c).querybuf) > server.client_max_querybuf_len
        {
            let mut ci: sds = catClientInfoString(sdsempty(), c);
            let mut bytes: sds = sdsempty();
            bytes = sdscatrepr(
                bytes,
                (*c).querybuf as *const libc::c_char,
                64 as libc::c_int as size_t,
            );
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Closing client that reached max query buffer length: %s (qbuf initial bytes: %s)\0"
                        as *const u8 as *const libc::c_char,
                    ci,
                    bytes,
                );
            }
            sdsfree(ci);
            sdsfree(bytes);
            freeClientAsync(c);
        } else if processInputBuffer(c) == -(1 as libc::c_int) {
            c = 0 as *mut client;
        }
    }
    beforeNextClient(c);
}
#[no_mangle]
pub unsafe extern "C" fn genClientAddrString(
    mut client: *mut client,
    mut addr: *mut libc::c_char,
    mut addr_len: size_t,
    mut fd_to_str_type: libc::c_int,
) {
    if (*client).flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0
    {
        snprintf(
            addr,
            addr_len,
            b"%s:0\0" as *const u8 as *const libc::c_char,
            server.unixsocket,
        );
    } else {
        connFormatFdAddr((*client).conn, addr, addr_len, fd_to_str_type);
    };
}
#[no_mangle]
pub unsafe extern "C" fn getClientPeerId(mut c: *mut client) -> *mut libc::c_char {
    let mut peerid: [libc::c_char; 78] = [0; 78];
    if ((*c).peerid).is_null() {
        genClientAddrString(
            c,
            peerid.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 78]>() as libc::c_ulong,
            0 as libc::c_int,
        );
        (*c).peerid = sdsnew(peerid.as_mut_ptr());
    }
    return (*c).peerid;
}
#[no_mangle]
pub unsafe extern "C" fn getClientSockname(mut c: *mut client) -> *mut libc::c_char {
    let mut sockname: [libc::c_char; 78] = [0; 78];
    if ((*c).sockname).is_null() {
        genClientAddrString(
            c,
            sockname.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 78]>() as libc::c_ulong,
            1 as libc::c_int,
        );
        (*c).sockname = sdsnew(sockname.as_mut_ptr());
    }
    return (*c).sockname;
}
#[no_mangle]
pub unsafe extern "C" fn catClientInfoString(
    mut s: sds,
    mut client: *mut client,
) -> sds {
    let mut flags: [libc::c_char; 16] = [0; 16];
    let mut events: [libc::c_char; 3] = [0; 3];
    let mut conninfo: [libc::c_char; 32] = [0; 32];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = flags.as_mut_ptr();
    if (*client).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
        if (*client).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
            != 0
        {
            let fresh27 = p;
            p = p.offset(1);
            *fresh27 = 'O' as i32 as libc::c_char;
        } else {
            let fresh28 = p;
            p = p.offset(1);
            *fresh28 = 'S' as i32 as libc::c_char;
        }
    }
    if (*client).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        let fresh29 = p;
        p = p.offset(1);
        *fresh29 = 'M' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
    {
        let fresh30 = p;
        p = p.offset(1);
        *fresh30 = 'P' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
        let fresh31 = p;
        p = p.offset(1);
        *fresh31 = 'x' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
        let fresh32 = p;
        p = p.offset(1);
        *fresh32 = 'b' as i32 as libc::c_char;
    }
    if (*client).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
    {
        let fresh33 = p;
        p = p.offset(1);
        *fresh33 = 't' as i32 as libc::c_char;
    }
    if (*client).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 32 as libc::c_int != 0
    {
        let fresh34 = p;
        p = p.offset(1);
        *fresh34 = 'R' as i32 as libc::c_char;
    }
    if (*client).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
    {
        let fresh35 = p;
        p = p.offset(1);
        *fresh35 = 'B' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong != 0 {
        let fresh36 = p;
        p = p.offset(1);
        *fresh36 = 'd' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong != 0 {
        let fresh37 = p;
        p = p.offset(1);
        *fresh37 = 'c' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 7 as libc::c_int) as libc::c_ulong != 0 {
        let fresh38 = p;
        p = p.offset(1);
        *fresh38 = 'u' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
    {
        let fresh39 = p;
        p = p.offset(1);
        *fresh39 = 'A' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0
    {
        let fresh40 = p;
        p = p.offset(1);
        *fresh40 = 'U' as i32 as libc::c_char;
    }
    if (*client).flags & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong != 0
    {
        let fresh41 = p;
        p = p.offset(1);
        *fresh41 = 'r' as i32 as libc::c_char;
    }
    if (*client).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 43 as libc::c_int != 0
    {
        let fresh42 = p;
        p = p.offset(1);
        *fresh42 = 'e' as i32 as libc::c_char;
    }
    if p == flags.as_mut_ptr() {
        let fresh43 = p;
        p = p.offset(1);
        *fresh43 = 'N' as i32 as libc::c_char;
    }
    let fresh44 = p;
    p = p.offset(1);
    *fresh44 = '\0' as i32 as libc::c_char;
    p = events.as_mut_ptr();
    if !((*client).conn).is_null() {
        if connHasReadHandler((*client).conn) != 0 {
            let fresh45 = p;
            p = p.offset(1);
            *fresh45 = 'r' as i32 as libc::c_char;
        }
        if connHasWriteHandler((*client).conn) != 0 {
            let fresh46 = p;
            p = p.offset(1);
            *fresh46 = 'w' as i32 as libc::c_char;
        }
    }
    *p = '\0' as i32 as libc::c_char;
    let mut obufmem: size_t = 0;
    let mut total_mem: size_t = getClientMemoryUsage(client, &mut obufmem);
    let mut used_blocks_of_repl_buf: size_t = 0 as libc::c_int as size_t;
    if !((*client).ref_repl_buf_node).is_null() {
        let mut last: *mut replBufBlock = (*(*server.repl_buffer_blocks).tail).value
            as *mut replBufBlock;
        let mut cur: *mut replBufBlock = (*(*client).ref_repl_buf_node).value
            as *mut replBufBlock;
        used_blocks_of_repl_buf = ((*last).id - (*cur).id
            + 1 as libc::c_int as libc::c_longlong) as size_t;
    }
    let mut ret: sds = sdscatfmt(
        s,
        b"id=%U addr=%s laddr=%s %s name=%s age=%I idle=%I flags=%s db=%i sub=%i psub=%i ssub=%i multi=%i qbuf=%U qbuf-free=%U argv-mem=%U multi-mem=%U rbs=%U rbp=%U obl=%U oll=%U omem=%U tot-mem=%U events=%s cmd=%s user=%s redir=%I resp=%i\0"
            as *const u8 as *const libc::c_char,
        (*client).id as libc::c_ulonglong,
        getClientPeerId(client),
        getClientSockname(client),
        connGetInfo(
            (*client).conn,
            conninfo.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        ),
        if !((*client).name).is_null() {
            (*(*client).name).ptr as *mut libc::c_char as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
        (server.unixtime as libc::c_long - (*client).ctime) as libc::c_longlong,
        (server.unixtime as libc::c_long - (*client).lastinteraction)
            as libc::c_longlong,
        flags.as_mut_ptr(),
        (*(*client).db).id,
        ((*(*client).pubsub_channels).ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*(*client).pubsub_channels).ht_used[1 as libc::c_int as usize],
            ) as libc::c_int,
        (*(*client).pubsub_patterns).len as libc::c_int,
        ((*(*client).pubsubshard_channels).ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*(*client).pubsubshard_channels).ht_used[1 as libc::c_int as usize],
            ) as libc::c_int,
        if (*client).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
            != 0
        {
            (*client).mstate.count
        } else {
            -(1 as libc::c_int)
        },
        sdslen((*client).querybuf) as libc::c_ulonglong,
        sdsavail((*client).querybuf) as libc::c_ulonglong,
        (*client).argv_len_sum as libc::c_ulonglong,
        (*client).mstate.argv_len_sums as libc::c_ulonglong,
        (*client).buf_usable_size as libc::c_ulonglong,
        (*client).buf_peak as libc::c_ulonglong,
        (*client).bufpos as libc::c_ulonglong,
        ((*(*client).reply).len as libc::c_ulonglong)
            .wrapping_add(used_blocks_of_repl_buf as libc::c_ulonglong),
        obufmem as libc::c_ulonglong,
        total_mem as libc::c_ulonglong,
        events.as_mut_ptr(),
        if !((*client).lastcmd).is_null() {
            (*(*client).lastcmd).fullname as *const libc::c_char
        } else {
            b"NULL\0" as *const u8 as *const libc::c_char
        },
        if !((*client).user).is_null() {
            (*(*client).user).name as *const libc::c_char
        } else {
            b"(superuser)\0" as *const u8 as *const libc::c_char
        },
        if (*client).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
        {
            (*client).client_tracking_redirection as libc::c_longlong
        } else {
            -(1 as libc::c_int) as libc::c_longlong
        },
        (*client).resp,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn getAllClientsInfoString(mut type_0: libc::c_int) -> sds {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut client: *mut client = 0 as *mut client;
    let mut o: sds = sdsnewlen(
        SDS_NOINIT as *const libc::c_void,
        (200 as libc::c_int as libc::c_ulong).wrapping_mul((*server.clients).len),
    );
    sdsclear(o);
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        client = (*ln).value as *mut client;
        if type_0 != -(1 as libc::c_int) && getClientType(client) != type_0 {
            continue;
        }
        o = catClientInfoString(o, client);
        o = sdscatlen(
            o,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn clientSetName(
    mut c: *mut client,
    mut name: *mut robj,
) -> libc::c_int {
    let mut len: libc::c_int = (if !name.is_null() {
        sdslen((*name).ptr as sds)
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_int;
    if len == 0 as libc::c_int {
        if !((*c).name).is_null() {
            decrRefCount((*c).name);
        }
        (*c).name = 0 as *mut robj;
        return 0 as libc::c_int;
    }
    let mut p: *mut libc::c_char = (*name).ptr as *mut libc::c_char;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < len {
        if (*p.offset(j as isize) as libc::c_int) < '!' as i32
            || *p.offset(j as isize) as libc::c_int > '~' as i32
        {
            return -(1 as libc::c_int);
        }
        j += 1;
    }
    if !((*c).name).is_null() {
        decrRefCount((*c).name);
    }
    (*c).name = name;
    incrRefCount(name);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clientSetNameOrReply(
    mut c: *mut client,
    mut name: *mut robj,
) -> libc::c_int {
    let mut result: libc::c_int = clientSetName(c, name);
    if result == -(1 as libc::c_int) {
        addReplyError(
            c,
            b"Client names cannot contain spaces, newlines or special characters.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn resetCommand(mut c: *mut client) {
    let mut flags: uint64_t = (*c).flags;
    if flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
        flags
            &= !((1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    }
    if flags
        & ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 27 as libc::c_int) as libc::c_ulong != 0
    {
        addReplyError(
            c,
            b"can only reset normal client connections\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    clearClientConnectionState(c);
    addReplyStatus(c, b"RESET\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn quitCommand(mut c: *mut client) {
    addReply(c, shared.ok);
    (*c).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn clientCommand(mut c: *mut client) {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 46] = [
            b"CACHING (YES|NO)\0" as *const u8 as *const libc::c_char,
            b"    Enable/disable tracking of the keys for next command in OPTIN/OPTOUT modes.\0"
                as *const u8 as *const libc::c_char,
            b"GETREDIR\0" as *const u8 as *const libc::c_char,
            b"    Return the client ID we are redirecting to when tracking is enabled.\0"
                as *const u8 as *const libc::c_char,
            b"GETNAME\0" as *const u8 as *const libc::c_char,
            b"    Return the name of the current connection.\0" as *const u8
                as *const libc::c_char,
            b"ID\0" as *const u8 as *const libc::c_char,
            b"    Return the ID of the current connection.\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
            b"    Return information about the current client connection.\0" as *const u8
                as *const libc::c_char,
            b"KILL <ip:port>\0" as *const u8 as *const libc::c_char,
            b"    Kill connection made from <ip:port>.\0" as *const u8
                as *const libc::c_char,
            b"KILL <option> <value> [<option> <value> [...]]\0" as *const u8
                as *const libc::c_char,
            b"    Kill connections. Options are:\0" as *const u8 as *const libc::c_char,
            b"    * ADDR (<ip:port>|<unixsocket>:0)\0" as *const u8
                as *const libc::c_char,
            b"      Kill connections made from the specified address\0" as *const u8
                as *const libc::c_char,
            b"    * LADDR (<ip:port>|<unixsocket>:0)\0" as *const u8
                as *const libc::c_char,
            b"      Kill connections made to specified local address\0" as *const u8
                as *const libc::c_char,
            b"    * TYPE (NORMAL|MASTER|REPLICA|PUBSUB)\0" as *const u8
                as *const libc::c_char,
            b"      Kill connections by type.\0" as *const u8 as *const libc::c_char,
            b"    * USER <username>\0" as *const u8 as *const libc::c_char,
            b"      Kill connections authenticated by <username>.\0" as *const u8
                as *const libc::c_char,
            b"    * SKIPME (YES|NO)\0" as *const u8 as *const libc::c_char,
            b"      Skip killing current connection (default: yes).\0" as *const u8
                as *const libc::c_char,
            b"LIST [options ...]\0" as *const u8 as *const libc::c_char,
            b"    Return information about client connections. Options:\0" as *const u8
                as *const libc::c_char,
            b"    * TYPE (NORMAL|MASTER|REPLICA|PUBSUB)\0" as *const u8
                as *const libc::c_char,
            b"      Return clients of specified type.\0" as *const u8
                as *const libc::c_char,
            b"UNPAUSE\0" as *const u8 as *const libc::c_char,
            b"    Stop the current client pause, resuming traffic.\0" as *const u8
                as *const libc::c_char,
            b"PAUSE <timeout> [WRITE|ALL]\0" as *const u8 as *const libc::c_char,
            b"    Suspend all, or just write, clients for <timeout> milliseconds.\0"
                as *const u8 as *const libc::c_char,
            b"REPLY (ON|OFF|SKIP)\0" as *const u8 as *const libc::c_char,
            b"    Control the replies sent to the current connection.\0" as *const u8
                as *const libc::c_char,
            b"SETNAME <name>\0" as *const u8 as *const libc::c_char,
            b"    Assign the name <name> to the current connection.\0" as *const u8
                as *const libc::c_char,
            b"UNBLOCK <clientid> [TIMEOUT|ERROR]\0" as *const u8 as *const libc::c_char,
            b"    Unblock the specified blocked client.\0" as *const u8
                as *const libc::c_char,
            b"TRACKING (ON|OFF) [REDIRECT <id>] [BCAST] [PREFIX <prefix> [...]]\0"
                as *const u8 as *const libc::c_char,
            b"         [OPTIN] [OPTOUT] [NOLOOP]\0" as *const u8 as *const libc::c_char,
            b"    Control server assisted client side caching.\0" as *const u8
                as *const libc::c_char,
            b"TRACKINGINFO\0" as *const u8 as *const libc::c_char,
            b"    Report tracking status for the current connection.\0" as *const u8
                as *const libc::c_char,
            b"NO-EVICT (ON|OFF)\0" as *const u8 as *const libc::c_char,
            b"    Protect current client connection from eviction.\0" as *const u8
                as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"id\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        addReplyLongLong(c, (*c).id as libc::c_longlong);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"info\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut o: sds = catClientInfoString(sdsempty(), c);
        o = sdscatlen(
            o,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        addReplyVerbatim(
            c,
            o as *const libc::c_char,
            sdslen(o),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(o);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"list\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut type_0: libc::c_int = -(1 as libc::c_int);
        let mut o_0: sds = 0 as sds;
        if (*c).argc == 4 as libc::c_int
            && strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"type\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            type_0 = getClientTypeByName(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            if type_0 == -(1 as libc::c_int) {
                addReplyErrorFormat(
                    c,
                    b"Unknown client type '%s'\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
        } else if (*c).argc > 3 as libc::c_int
            && strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"id\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            let mut j: libc::c_int = 0;
            o_0 = sdsempty();
            j = 3 as libc::c_int;
            while j < (*c).argc {
                let mut cid: libc::c_longlong = 0;
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(j as isize),
                    &mut cid,
                    b"Invalid client ID\0" as *const u8 as *const libc::c_char,
                ) != 0
                {
                    sdsfree(o_0);
                    return;
                }
                let mut cl: *mut client = lookupClientByID(cid as uint64_t);
                if !cl.is_null() {
                    o_0 = catClientInfoString(o_0, cl);
                    o_0 = sdscatlen(
                        o_0,
                        b"\n\0" as *const u8 as *const libc::c_char
                            as *const libc::c_void,
                        1 as libc::c_int as size_t,
                    );
                }
                j += 1;
            }
        } else if (*c).argc != 2 as libc::c_int {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        if o_0.is_null() {
            o_0 = getAllClientsInfoString(type_0);
        }
        addReplyVerbatim(
            c,
            o_0 as *const libc::c_char,
            sdslen(o_0),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(o_0);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"reply\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"on\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*c).flags
                &= !((1 as libc::c_int) << 24 as libc::c_int
                    | (1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong;
            addReply(c, shared.ok);
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"off\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*c).flags |= ((1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong;
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"skip\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if (*c).flags & ((1 as libc::c_int) << 22 as libc::c_int) as libc::c_ulong
                == 0
            {
                (*c).flags |= ((1 as libc::c_int) << 23 as libc::c_int) as libc::c_ulong;
            }
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"no-evict\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"on\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 43 as libc::c_int) as uint64_t;
            removeClientFromMemUsageBucket(c, 0 as libc::c_int);
            addReply(c, shared.ok);
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"off\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                & !((1 as libc::c_ulonglong) << 43 as libc::c_int)) as uint64_t;
            updateClientMemUsageAndBucket(c);
            addReply(c, shared.ok);
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"kill\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut addr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut laddr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut user: *mut user = 0 as *mut user;
        let mut type_1: libc::c_int = -(1 as libc::c_int);
        let mut id: uint64_t = 0 as libc::c_int as uint64_t;
        let mut skipme: libc::c_int = 1 as libc::c_int;
        let mut killed: libc::c_int = 0 as libc::c_int;
        let mut close_this_client: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 3 as libc::c_int {
            addr = (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                as *mut libc::c_char;
            skipme = 0 as libc::c_int;
        } else if (*c).argc > 3 as libc::c_int {
            let mut i: libc::c_int = 2 as libc::c_int;
            while i < (*c).argc {
                let mut moreargs: libc::c_int = ((*c).argc > i + 1 as libc::c_int)
                    as libc::c_int;
                if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"id\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    let mut tmp: libc::c_long = 0;
                    if getRangeLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset((i + 1 as libc::c_int) as isize),
                        1 as libc::c_int as libc::c_long,
                        9223372036854775807 as libc::c_long,
                        &mut tmp,
                        b"client-id should be greater than 0\0" as *const u8
                            as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        return;
                    }
                    id = tmp as uint64_t;
                } else if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"type\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    type_1 = getClientTypeByName(
                        (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                            as *mut libc::c_char,
                    );
                    if type_1 == -(1 as libc::c_int) {
                        addReplyErrorFormat(
                            c,
                            b"Unknown client type '%s'\0" as *const u8
                                as *const libc::c_char,
                            (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                                as *mut libc::c_char,
                        );
                        return;
                    }
                } else if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"addr\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    addr = (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                        as *mut libc::c_char;
                } else if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"laddr\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    laddr = (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                        as *mut libc::c_char;
                } else if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"user\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    user = ACLGetUserByName(
                        (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                            as *const libc::c_char,
                        sdslen(
                            (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                                as sds,
                        ),
                    );
                    if user.is_null() {
                        addReplyErrorFormat(
                            c,
                            b"No such user '%s'\0" as *const u8 as *const libc::c_char,
                            (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                                as *mut libc::c_char,
                        );
                        return;
                    }
                } else if strcasecmp(
                    (**((*c).argv).offset(i as isize)).ptr as *const libc::c_char,
                    b"skipme\0" as *const u8 as *const libc::c_char,
                ) == 0 && moreargs != 0
                {
                    if strcasecmp(
                        (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                            as *const libc::c_char,
                        b"yes\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        skipme = 1 as libc::c_int;
                    } else if strcasecmp(
                        (**((*c).argv).offset((i + 1 as libc::c_int) as isize)).ptr
                            as *const libc::c_char,
                        b"no\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        skipme = 0 as libc::c_int;
                    } else {
                        addReplyErrorObject(c, shared.syntaxerr);
                        return;
                    }
                } else {
                    addReplyErrorObject(c, shared.syntaxerr);
                    return;
                }
                i += 2 as libc::c_int;
            }
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        listRewind(server.clients, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut client: *mut client = (*ln).value as *mut client;
            if !addr.is_null()
                && strcmp(getClientPeerId(client), addr) != 0 as libc::c_int
            {
                continue;
            }
            if !laddr.is_null()
                && strcmp(getClientSockname(client), laddr) != 0 as libc::c_int
            {
                continue;
            }
            if type_1 != -(1 as libc::c_int) && getClientType(client) != type_1 {
                continue;
            }
            if id != 0 as libc::c_int as libc::c_ulong && (*client).id != id {
                continue;
            }
            if !user.is_null() && (*client).user != user {
                continue;
            }
            if c == client && skipme != 0 {
                continue;
            }
            if c == client {
                close_this_client = 1 as libc::c_int;
            } else {
                freeClient(client);
            }
            killed += 1;
        }
        if (*c).argc == 3 as libc::c_int {
            if killed == 0 as libc::c_int {
                addReplyError(
                    c,
                    b"No such client\0" as *const u8 as *const libc::c_char,
                );
            } else {
                addReply(c, shared.ok);
            }
        } else {
            addReplyLongLong(c, killed as libc::c_longlong);
        }
        if close_this_client != 0 {
            (*c).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"unblock\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 3 as libc::c_int || (*c).argc == 4 as libc::c_int)
    {
        let mut id_0: libc::c_longlong = 0;
        let mut unblock_error: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 4 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"timeout\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                unblock_error = 0 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"error\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                unblock_error = 1 as libc::c_int;
            } else {
                addReplyError(
                    c,
                    b"CLIENT UNBLOCK reason should be TIMEOUT or ERROR\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut id_0,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        let mut target: *mut client = lookupClientByID(id_0 as uint64_t);
        if !target.is_null()
            && (*target).flags
                & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0
            && moduleBlockedClientMayTimeout(target) != 0
        {
            if unblock_error != 0 {
                addReplyError(
                    target,
                    b"-UNBLOCKED client unblocked via CLIENT UNBLOCK\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                replyToBlockedClientTimedOut(target);
            }
            unblockClient(target);
            updateStatsOnUnblock(
                target,
                0 as libc::c_int as libc::c_long,
                0 as libc::c_int as libc::c_long,
                1 as libc::c_int,
            );
            addReply(c, shared.cone);
        } else {
            addReply(c, shared.czero);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"setname\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        if clientSetNameOrReply(c, *((*c).argv).offset(2 as libc::c_int as isize))
            == 0 as libc::c_int
        {
            addReply(c, shared.ok);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"getname\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if !((*c).name).is_null() {
            addReplyBulk(c, (*c).name);
        } else {
            addReplyNull(c);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"unpause\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        unpauseClients(PAUSE_BY_CLIENT_COMMAND);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"pause\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 3 as libc::c_int || (*c).argc == 4 as libc::c_int)
    {
        let mut end: mstime_t = 0;
        let mut type_2: libc::c_int = CLIENT_PAUSE_ALL as libc::c_int;
        if (*c).argc == 4 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"write\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                type_2 = CLIENT_PAUSE_WRITE as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"all\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                type_2 = CLIENT_PAUSE_ALL as libc::c_int;
            } else {
                addReplyError(
                    c,
                    b"CLIENT PAUSE mode must be WRITE or ALL\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
        if getTimeoutFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut end,
            1 as libc::c_int,
        ) != 0 as libc::c_int
        {
            return;
        }
        pauseClients(PAUSE_BY_CLIENT_COMMAND, end, type_2 as pause_type);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"tracking\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int
    {
        let mut redir: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut options: uint64_t = 0 as libc::c_int as uint64_t;
        let mut prefix: *mut *mut robj = 0 as *mut *mut robj;
        let mut numprefix: size_t = 0 as libc::c_int as size_t;
        let mut j_0: libc::c_int = 3 as libc::c_int;
        while j_0 < (*c).argc {
            let mut moreargs_0: libc::c_int = (*c).argc - 1 as libc::c_int - j_0;
            if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"redirect\0" as *const u8 as *const libc::c_char,
            ) == 0 && moreargs_0 != 0
            {
                j_0 += 1;
                if redir != 0 as libc::c_int as libc::c_longlong {
                    addReplyError(
                        c,
                        b"A client can only redirect to a single other client\0"
                            as *const u8 as *const libc::c_char,
                    );
                    zfree(prefix as *mut libc::c_void);
                    return;
                }
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset(j_0 as isize),
                    &mut redir,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    zfree(prefix as *mut libc::c_void);
                    return;
                }
                if (lookupClientByID(redir as uint64_t)).is_null() {
                    addReplyError(
                        c,
                        b"The client ID you want redirect to does not exist\0"
                            as *const u8 as *const libc::c_char,
                    );
                    zfree(prefix as *mut libc::c_void);
                    return;
                }
            } else if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"bcast\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                options = (options as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 33 as libc::c_int) as uint64_t;
            } else if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"optin\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                options = (options as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 34 as libc::c_int) as uint64_t;
            } else if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"optout\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                options = (options as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 35 as libc::c_int) as uint64_t;
            } else if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"noloop\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                options = (options as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 37 as libc::c_int) as uint64_t;
            } else if strcasecmp(
                (**((*c).argv).offset(j_0 as isize)).ptr as *const libc::c_char,
                b"prefix\0" as *const u8 as *const libc::c_char,
            ) == 0 && moreargs_0 != 0
            {
                j_0 += 1;
                prefix = zrealloc(
                    prefix as *mut libc::c_void,
                    (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                        .wrapping_mul(
                            numprefix.wrapping_add(1 as libc::c_int as libc::c_ulong),
                        ),
                ) as *mut *mut robj;
                let fresh47 = numprefix;
                numprefix = numprefix.wrapping_add(1);
                let ref mut fresh48 = *prefix.offset(fresh47 as isize);
                *fresh48 = *((*c).argv).offset(j_0 as isize);
            } else {
                zfree(prefix as *mut libc::c_void);
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            j_0 += 1;
        }
        if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"on\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if options as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 33 as libc::c_int == 0 && numprefix != 0
            {
                addReplyError(
                    c,
                    b"PREFIX option requires BCAST mode to be enabled\0" as *const u8
                        as *const libc::c_char,
                );
                zfree(prefix as *mut libc::c_void);
                return;
            }
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
            {
                let mut oldbcast: libc::c_int = ((*c).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0) as libc::c_int;
                let mut newbcast: libc::c_int = (options as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0) as libc::c_int;
                if oldbcast != newbcast {
                    addReplyError(
                        c,
                        b"You can't switch BCAST mode on/off before disabling tracking for this client, and then re-enabling it with a different mode.\0"
                            as *const u8 as *const libc::c_char,
                    );
                    zfree(prefix as *mut libc::c_void);
                    return;
                }
            }
            if options as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
                && options as libc::c_ulonglong
                    & ((1 as libc::c_ulonglong) << 34 as libc::c_int
                        | (1 as libc::c_ulonglong) << 35 as libc::c_int) != 0
            {
                addReplyError(
                    c,
                    b"OPTIN and OPTOUT are not compatible with BCAST\0" as *const u8
                        as *const libc::c_char,
                );
                zfree(prefix as *mut libc::c_void);
                return;
            }
            if options as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 34 as libc::c_int != 0
                && options as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 35 as libc::c_int != 0
            {
                addReplyError(
                    c,
                    b"You can't specify both OPTIN mode and OPTOUT mode\0" as *const u8
                        as *const libc::c_char,
                );
                zfree(prefix as *mut libc::c_void);
                return;
            }
            if options as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 34 as libc::c_int != 0
                && (*c).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 35 as libc::c_int != 0
                || options as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 35 as libc::c_int != 0
                    && (*c).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 34 as libc::c_int != 0
            {
                addReplyError(
                    c,
                    b"You can't switch OPTIN/OPTOUT mode before disabling tracking for this client, and then re-enabling it with a different mode.\0"
                        as *const u8 as *const libc::c_char,
                );
                zfree(prefix as *mut libc::c_void);
                return;
            }
            if options as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
            {
                if checkPrefixCollisionsOrReply(c, prefix, numprefix) == 0 {
                    zfree(prefix as *mut libc::c_void);
                    return;
                }
            }
            enableTracking(c, redir as uint64_t, options, prefix, numprefix);
        } else if strcasecmp(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"off\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            disableTracking(c);
        } else {
            zfree(prefix as *mut libc::c_void);
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        zfree(prefix as *mut libc::c_void);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"caching\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int
    {
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 31 as libc::c_int == 0
        {
            addReplyError(
                c,
                b"CLIENT CACHING can be called only when the client is in tracking mode with OPTIN or OPTOUT mode enabled\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        let mut opt: *mut libc::c_char = (**((*c).argv)
            .offset(2 as libc::c_int as isize))
            .ptr as *mut libc::c_char;
        if strcasecmp(opt, b"yes\0" as *const u8 as *const libc::c_char) == 0 {
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 34 as libc::c_int != 0
            {
                (*c)
                    .flags = ((*c).flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 36 as libc::c_int) as uint64_t;
            } else {
                addReplyError(
                    c,
                    b"CLIENT CACHING YES is only valid when tracking is enabled in OPTIN mode.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
        } else if strcasecmp(opt, b"no\0" as *const u8 as *const libc::c_char) == 0 {
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 35 as libc::c_int != 0
            {
                (*c)
                    .flags = ((*c).flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 36 as libc::c_int) as uint64_t;
            } else {
                addReplyError(
                    c,
                    b"CLIENT CACHING NO is only valid when tracking is enabled in OPTOUT mode.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"getredir\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
        {
            addReplyLongLong(c, (*c).client_tracking_redirection as libc::c_longlong);
        } else {
            addReplyLongLong(c, -(1 as libc::c_int) as libc::c_longlong);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"trackinginfo\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
        let mut arraylen_ptr: *mut libc::c_void = addReplyDeferredLen(c);
        let mut numflags: libc::c_int = 0 as libc::c_int;
        addReplyBulkCString(
            c,
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
            {
                b"on\0" as *const u8 as *const libc::c_char
            } else {
                b"off\0" as *const u8 as *const libc::c_char
            },
        );
        numflags += 1;
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 33 as libc::c_int != 0
        {
            addReplyBulkCString(c, b"bcast\0" as *const u8 as *const libc::c_char);
            numflags += 1;
        }
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 34 as libc::c_int != 0
        {
            addReplyBulkCString(c, b"optin\0" as *const u8 as *const libc::c_char);
            numflags += 1;
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 36 as libc::c_int != 0
            {
                addReplyBulkCString(
                    c,
                    b"caching-yes\0" as *const u8 as *const libc::c_char,
                );
                numflags += 1;
            }
        }
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 35 as libc::c_int != 0
        {
            addReplyBulkCString(c, b"optout\0" as *const u8 as *const libc::c_char);
            numflags += 1;
            if (*c).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 36 as libc::c_int != 0
            {
                addReplyBulkCString(
                    c,
                    b"caching-no\0" as *const u8 as *const libc::c_char,
                );
                numflags += 1;
            }
        }
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 37 as libc::c_int != 0
        {
            addReplyBulkCString(c, b"noloop\0" as *const u8 as *const libc::c_char);
            numflags += 1;
        }
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 32 as libc::c_int != 0
        {
            addReplyBulkCString(
                c,
                b"broken_redirect\0" as *const u8 as *const libc::c_char,
            );
            numflags += 1;
        }
        setDeferredSetLen(c, arraylen_ptr, numflags as libc::c_long);
        addReplyBulkCString(c, b"redirect\0" as *const u8 as *const libc::c_char);
        if (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
        {
            addReplyLongLong(c, (*c).client_tracking_redirection as libc::c_longlong);
        } else {
            addReplyLongLong(c, -(1 as libc::c_int) as libc::c_longlong);
        }
        addReplyBulkCString(c, b"prefixes\0" as *const u8 as *const libc::c_char);
        if !((*c).client_tracking_prefixes).is_null() {
            addReplyArrayLen(c, raxSize((*c).client_tracking_prefixes) as libc::c_long);
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
            raxStart(&mut ri, (*c).client_tracking_prefixes);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri) != 0 {
                addReplyBulkCBuffer(c, ri.key as *const libc::c_void, ri.key_len);
            }
            raxStop(&mut ri);
        } else {
            addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
        }
    } else {
        addReplySubcommandSyntaxError(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn helloCommand(mut c: *mut client) {
    let mut ver: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut next_arg: libc::c_int = 1 as libc::c_int;
    if (*c).argc >= 2 as libc::c_int {
        let fresh49 = next_arg;
        next_arg = next_arg + 1;
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(fresh49 as isize),
            &mut ver,
            b"Protocol version is not an integer or out of range\0" as *const u8
                as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if ver < 2 as libc::c_int as libc::c_longlong
            || ver > 3 as libc::c_int as libc::c_longlong
        {
            addReplyError(
                c,
                b"-NOPROTO unsupported protocol version\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
    }
    let mut j: libc::c_int = next_arg;
    while j < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        let mut opt: *const libc::c_char = (**((*c).argv).offset(j as isize)).ptr
            as *const libc::c_char;
        if strcasecmp(opt, b"AUTH\0" as *const u8 as *const libc::c_char) == 0
            && moreargs >= 2 as libc::c_int
        {
            redactClientCommandArgument(c, j + 1 as libc::c_int);
            redactClientCommandArgument(c, j + 2 as libc::c_int);
            if ACLAuthenticateUser(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                *((*c).argv).offset((j + 2 as libc::c_int) as isize),
            ) == -(1 as libc::c_int)
            {
                addReplyError(
                    c,
                    b"-WRONGPASS invalid username-password pair or user is disabled.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            j += 2 as libc::c_int;
        } else if strcasecmp(opt, b"SETNAME\0" as *const u8 as *const libc::c_char) == 0
            && moreargs != 0
        {
            if clientSetNameOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
            ) == -(1 as libc::c_int)
            {
                return;
            }
            j += 1;
        } else {
            addReplyErrorFormat(
                c,
                b"Syntax error in HELLO option '%s'\0" as *const u8
                    as *const libc::c_char,
                opt,
            );
            return;
        }
        j += 1;
    }
    if (*c).authenticated == 0 {
        addReplyError(
            c,
            b"-NOAUTH HELLO must be called with the client already authenticated, otherwise the HELLO AUTH <user> <pass> option can be used to authenticate the client and select the RESP protocol version at the same time\0"
                as *const u8 as *const libc::c_char,
        );
        return;
    }
    if ver != 0 {
        (*c).resp = ver as libc::c_int;
    }
    addReplyMapLen(
        c,
        (6 as libc::c_int + (server.sentinel_mode == 0) as libc::c_int) as libc::c_long,
    );
    addReplyBulkCString(c, b"server\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, b"redis\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, b"version\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, b"7.0.8\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, b"proto\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*c).resp as libc::c_longlong);
    addReplyBulkCString(c, b"id\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*c).id as libc::c_longlong);
    addReplyBulkCString(c, b"mode\0" as *const u8 as *const libc::c_char);
    if server.sentinel_mode != 0 {
        addReplyBulkCString(c, b"sentinel\0" as *const u8 as *const libc::c_char);
    } else if server.cluster_enabled != 0 {
        addReplyBulkCString(c, b"cluster\0" as *const u8 as *const libc::c_char);
    } else {
        addReplyBulkCString(c, b"standalone\0" as *const u8 as *const libc::c_char);
    }
    if server.sentinel_mode == 0 {
        addReplyBulkCString(c, b"role\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(
            c,
            if !(server.masterhost).is_null() {
                b"replica\0" as *const u8 as *const libc::c_char
            } else {
                b"master\0" as *const u8 as *const libc::c_char
            },
        );
    }
    addReplyBulkCString(c, b"modules\0" as *const u8 as *const libc::c_char);
    addReplyLoadedModules(c);
}
#[no_mangle]
pub unsafe extern "C" fn securityWarningCommand(mut c: *mut client) {
    static mut logged_time: time_t = 0 as libc::c_int as time_t;
    let mut now: time_t = time(0 as *mut time_t);
    if llabs((now - logged_time) as libc::c_longlong)
        > 60 as libc::c_int as libc::c_longlong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Possible SECURITY ATTACK detected. It looks like somebody is sending POST or Host: commands to Redis. This is likely due to an attacker attempting to use Cross Protocol Scripting to compromise your Redis instance. Connection aborted.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        logged_time = now;
    }
    freeClientAsync(c);
}
unsafe extern "C" fn retainOriginalCommandVector(mut c: *mut client) {
    if !((*c).original_argv).is_null() {
        return;
    }
    (*c).original_argc = (*c).argc;
    (*c)
        .original_argv = zmalloc(
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul((*c).argc as libc::c_ulong),
    ) as *mut *mut robj;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*c).argc {
        let ref mut fresh50 = *((*c).original_argv).offset(j as isize);
        *fresh50 = *((*c).argv).offset(j as isize);
        incrRefCount(*((*c).argv).offset(j as isize));
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn redactClientCommandArgument(
    mut c: *mut client,
    mut argc: libc::c_int,
) {
    retainOriginalCommandVector(c);
    if *((*c).original_argv).offset(argc as isize) == shared.redacted {
        return;
    }
    decrRefCount(*((*c).original_argv).offset(argc as isize));
    let ref mut fresh51 = *((*c).original_argv).offset(argc as isize);
    *fresh51 = shared.redacted;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteClientCommandVector(
    mut c: *mut client,
    mut argc: libc::c_int,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    let mut j: libc::c_int = 0;
    let mut argv: *mut *mut robj = 0 as *mut *mut robj;
    argv = zmalloc(
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong),
    ) as *mut *mut robj;
    ap = args.clone();
    j = 0 as libc::c_int;
    while j < argc {
        let mut a: *mut robj = 0 as *mut robj;
        a = ap.arg::<*mut robj>();
        let ref mut fresh52 = *argv.offset(j as isize);
        *fresh52 = a;
        incrRefCount(a);
        j += 1;
    }
    replaceClientCommandVector(c, argc, argv);
}
#[no_mangle]
pub unsafe extern "C" fn replaceClientCommandVector(
    mut c: *mut client,
    mut argc: libc::c_int,
    mut argv: *mut *mut robj,
) {
    let mut j: libc::c_int = 0;
    retainOriginalCommandVector(c);
    freeClientArgv(c);
    (*c).argv = argv;
    (*c).argc = argc;
    (*c).argv_len_sum = 0 as libc::c_int as size_t;
    j = 0 as libc::c_int;
    while j < (*c).argc {
        if !(*((*c).argv).offset(j as isize)).is_null() {
            (*c)
                .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
                .wrapping_add(getStringObjectLen(*((*c).argv).offset(j as isize)))
                as size_t as size_t;
        }
        j += 1;
    }
    (*c).cmd = lookupCommandOrOriginal((*c).argv, (*c).argc);
    if !((*c).cmd).is_null() {} else {
        _serverAssertWithInfo(
            c,
            0 as *const robj,
            b"c->cmd != NULL\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            3614 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn rewriteClientCommandArgument(
    mut c: *mut client,
    mut i: libc::c_int,
    mut newval: *mut robj,
) {
    let mut oldval: *mut robj = 0 as *mut robj;
    retainOriginalCommandVector(c);
    if i >= (*c).argc {
        if i >= (*c).argv_len {
            (*c)
                .argv = zrealloc(
                (*c).argv as *mut libc::c_void,
                (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                    .wrapping_mul((i + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *mut robj;
            (*c).argv_len = i + 1 as libc::c_int;
        }
        (*c).argc = i + 1 as libc::c_int;
        let ref mut fresh53 = *((*c).argv).offset(i as isize);
        *fresh53 = 0 as *mut robj;
    }
    oldval = *((*c).argv).offset(i as isize);
    if !oldval.is_null() {
        (*c)
            .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
            .wrapping_sub(getStringObjectLen(oldval)) as size_t as size_t;
    }
    if !newval.is_null() {
        (*c)
            .argv_len_sum = ((*c).argv_len_sum as libc::c_ulong)
            .wrapping_add(getStringObjectLen(newval)) as size_t as size_t;
    }
    let ref mut fresh54 = *((*c).argv).offset(i as isize);
    *fresh54 = newval;
    incrRefCount(newval);
    if !oldval.is_null() {
        decrRefCount(oldval);
    }
    if i == 0 as libc::c_int {
        (*c).cmd = lookupCommandOrOriginal((*c).argv, (*c).argc);
        if !((*c).cmd).is_null() {} else {
            _serverAssertWithInfo(
                c,
                0 as *const robj,
                b"c->cmd != NULL\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                3653 as libc::c_int,
            );
            unreachable!();
        };
    }
}
#[no_mangle]
pub unsafe extern "C" fn getClientOutputBufferMemoryUsage(mut c: *mut client) -> size_t {
    if getClientType(c) == 1 as libc::c_int {
        let mut repl_buf_size: size_t = 0 as libc::c_int as size_t;
        let mut repl_node_num: size_t = 0 as libc::c_int as size_t;
        let mut repl_node_size: size_t = (core::mem::size_of::<listNode>()
            as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<replBufBlock>() as libc::c_ulong);
        if !((*c).ref_repl_buf_node).is_null() {
            let mut last: *mut replBufBlock = (*(*server.repl_buffer_blocks).tail).value
                as *mut replBufBlock;
            let mut cur: *mut replBufBlock = (*(*c).ref_repl_buf_node).value
                as *mut replBufBlock;
            repl_buf_size = ((*last).repl_offset as libc::c_ulonglong)
                .wrapping_add((*last).size as libc::c_ulonglong)
                .wrapping_sub((*cur).repl_offset as libc::c_ulonglong) as size_t;
            repl_node_num = ((*last).id - (*cur).id
                + 1 as libc::c_int as libc::c_longlong) as size_t;
        }
        return repl_buf_size.wrapping_add(repl_node_size.wrapping_mul(repl_node_num));
    } else {
        let mut list_item_size: size_t = (core::mem::size_of::<listNode>()
            as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<clientReplyBlock>() as libc::c_ulong);
        return ((*c).reply_bytes)
            .wrapping_add(
                list_item_size.wrapping_mul((*(*c).reply).len) as libc::c_ulonglong,
            ) as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn getClientMemoryUsage(
    mut c: *mut client,
    mut output_buffer_mem_usage: *mut size_t,
) -> size_t {
    let mut mem: size_t = getClientOutputBufferMemoryUsage(c);
    if !output_buffer_mem_usage.is_null() {
        *output_buffer_mem_usage = mem;
    }
    mem = (mem as libc::c_ulong).wrapping_add(sdsZmallocSize((*c).querybuf)) as size_t
        as size_t;
    mem = (mem as libc::c_ulong)
        .wrapping_add(je_malloc_usable_size(c as *mut libc::c_void)) as size_t as size_t;
    mem = (mem as libc::c_ulong).wrapping_add((*c).buf_usable_size) as size_t as size_t;
    mem = (mem as libc::c_ulong)
        .wrapping_add(
            ((*c).argv_len_sum)
                .wrapping_add(
                    (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                        .wrapping_mul((*c).argc as libc::c_ulong),
                ),
        ) as size_t as size_t;
    mem = (mem as libc::c_ulong).wrapping_add(multiStateMemOverhead(c)) as size_t
        as size_t;
    mem = (mem as libc::c_ulong).wrapping_add(pubsubMemOverhead(c)) as size_t as size_t;
    if !((*c).client_tracking_prefixes).is_null() {
        mem = (mem as libc::c_ulong)
            .wrapping_add(
                ((*(*c).client_tracking_prefixes).numnodes)
                    .wrapping_mul(
                        (core::mem::size_of::<raxNode>() as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                            ),
                    ),
            ) as size_t as size_t;
    }
    return mem;
}
#[no_mangle]
pub unsafe extern "C" fn getClientType(mut c: *mut client) -> libc::c_int {
    if (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        return 3 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
        && (*c).flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong == 0
    {
        return 1 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0 {
        return 2 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getClientTypeByName(
    mut name: *mut libc::c_char,
) -> libc::c_int {
    if strcasecmp(name, b"normal\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int
    } else if strcasecmp(name, b"slave\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    } else if strcasecmp(name, b"replica\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    } else if strcasecmp(name, b"pubsub\0" as *const u8 as *const libc::c_char) == 0 {
        return 2 as libc::c_int
    } else if strcasecmp(name, b"master\0" as *const u8 as *const libc::c_char) == 0 {
        return 3 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn getClientTypeName(mut class: libc::c_int) -> *mut libc::c_char {
    match class {
        0 => return b"normal\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        1 => return b"slave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        2 => return b"pubsub\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        3 => return b"master\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        _ => return 0 as *mut libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkClientOutputBufferLimits(
    mut c: *mut client,
) -> libc::c_int {
    let mut soft: libc::c_int = 0 as libc::c_int;
    let mut hard: libc::c_int = 0 as libc::c_int;
    let mut class: libc::c_int = 0;
    let mut used_mem: libc::c_ulong = getClientOutputBufferMemoryUsage(c);
    class = getClientType(c);
    if class == 3 as libc::c_int {
        class = 0 as libc::c_int;
    }
    let mut hard_limit_bytes: size_t = server
        .client_obuf_limits[class as usize]
        .hard_limit_bytes as size_t;
    if class == 1 as libc::c_int && hard_limit_bytes != 0
        && (hard_limit_bytes as libc::c_longlong) < server.repl_backlog_size
    {
        hard_limit_bytes = server.repl_backlog_size as size_t;
    }
    if server.client_obuf_limits[class as usize].hard_limit_bytes != 0
        && used_mem >= hard_limit_bytes
    {
        hard = 1 as libc::c_int;
    }
    if server.client_obuf_limits[class as usize].soft_limit_bytes != 0
        && used_mem as libc::c_ulonglong
            >= server.client_obuf_limits[class as usize].soft_limit_bytes
    {
        soft = 1 as libc::c_int;
    }
    if soft != 0 {
        if (*c).obuf_soft_limit_reached_time == 0 as libc::c_int as libc::c_long {
            (*c).obuf_soft_limit_reached_time = server.unixtime as time_t;
            soft = 0 as libc::c_int;
        } else {
            let mut elapsed: time_t = server.unixtime as libc::c_long
                - (*c).obuf_soft_limit_reached_time;
            if elapsed <= server.client_obuf_limits[class as usize].soft_limit_seconds {
                soft = 0 as libc::c_int;
            }
        }
    } else {
        (*c).obuf_soft_limit_reached_time = 0 as libc::c_int as time_t;
    }
    return (soft != 0 || hard != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn closeClientOnOutputBufferLimitReached(
    mut c: *mut client,
    mut async_0: libc::c_int,
) -> libc::c_int {
    if ((*c).conn).is_null() {
        return 0 as libc::c_int;
    }
    if (*c).reply_bytes
        < (18446744073709551615 as libc::c_ulong)
            .wrapping_sub((1024 as libc::c_int * 64 as libc::c_int) as libc::c_ulong)
            as libc::c_ulonglong
    {} else {
        _serverAssert(
            b"c->reply_bytes < SIZE_MAX-(1024*64)\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            3813 as libc::c_int,
        );
        unreachable!();
    };
    if (*c).reply_bytes == 0 as libc::c_int as libc::c_ulonglong
        && getClientType(c) != 1 as libc::c_int
        || (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0
    {
        return 0 as libc::c_int;
    }
    if checkClientOutputBufferLimits(c) != 0 {
        let mut client: sds = catClientInfoString(sdsempty(), c);
        if async_0 != 0 {
            freeClientAsync(c);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Client %s scheduled to be closed ASAP for overcoming of output buffer limits.\0"
                        as *const u8 as *const libc::c_char,
                    client,
                );
            }
        } else {
            freeClient(c);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Client %s closed for overcoming of output buffer limits.\0"
                        as *const u8 as *const libc::c_char,
                    client,
                );
            }
        }
        sdsfree(client);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn flushSlavesOutputBuffers() {
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
        let mut slave: *mut client = (*ln).value as *mut client;
        let mut can_receive_writes: libc::c_int = (connHasWriteHandler((*slave).conn)
            != 0
            || (*slave).flags
                & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong != 0)
            as libc::c_int;
        if (*slave).replstate == 9 as libc::c_int && can_receive_writes != 0
            && (*slave).repl_start_cmd_stream_on_ack == 0
            && clientHasPendingReplies(slave) != 0
        {
            writeToClient(slave, 0 as libc::c_int);
        }
    };
}
unsafe extern "C" fn updateClientPauseTypeAndEndTime() {
    let mut old_type: pause_type = server.client_pause_type;
    let mut type_0: pause_type = CLIENT_PAUSE_OFF;
    let mut end: mstime_t = 0 as libc::c_int as mstime_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < NUM_PAUSE_PURPOSES as libc::c_int {
        let mut p: *mut pause_event = server.client_pause_per_purpose[i as usize];
        if !p.is_null() {
            if (*p).end < server.mstime {
                zfree(p as *mut libc::c_void);
                server.client_pause_per_purpose[i as usize] = 0 as *mut pause_event;
            } else if (*p).type_0 as libc::c_uint > type_0 as libc::c_uint {
                type_0 = (*p).type_0;
            }
        }
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < NUM_PAUSE_PURPOSES as libc::c_int {
        let mut p_0: *mut pause_event = server.client_pause_per_purpose[i_0 as usize];
        if !p_0.is_null() && (*p_0).type_0 as libc::c_uint == type_0 as libc::c_uint
            && (*p_0).end > end
        {
            end = (*p_0).end;
        }
        i_0 += 1;
    }
    server.client_pause_type = type_0;
    server.client_pause_end_time = end;
    if (type_0 as libc::c_uint) < old_type as libc::c_uint {
        unblockPostponedClients();
    }
}
#[no_mangle]
pub unsafe extern "C" fn unblockPostponedClients() {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(server.postponed_clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        unblockClient(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn pauseClients(
    mut purpose: pause_purpose,
    mut end: mstime_t,
    mut type_0: pause_type,
) {
    if (server.client_pause_per_purpose[purpose as usize]).is_null() {
        server
            .client_pause_per_purpose[purpose
            as usize] = zmalloc(core::mem::size_of::<pause_event>() as libc::c_ulong)
            as *mut pause_event;
        (*server.client_pause_per_purpose[purpose as usize]).type_0 = type_0;
        (*server.client_pause_per_purpose[purpose as usize]).end = end;
    } else {
        let mut p: *mut pause_event = server.client_pause_per_purpose[purpose as usize];
        (*p)
            .type_0 = (if (*p).type_0 as libc::c_uint > type_0 as libc::c_uint {
            (*p).type_0 as libc::c_uint
        } else {
            type_0 as libc::c_uint
        }) as pause_type;
        (*p).end = if (*p).end > end { (*p).end } else { end };
    }
    updateClientPauseTypeAndEndTime();
    if server.in_exec != 0 {
        server.client_pause_in_transaction = 1 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn unpauseClients(mut purpose: pause_purpose) {
    if (server.client_pause_per_purpose[purpose as usize]).is_null() {
        return;
    }
    zfree(server.client_pause_per_purpose[purpose as usize] as *mut libc::c_void);
    server.client_pause_per_purpose[purpose as usize] = 0 as *mut pause_event;
    updateClientPauseTypeAndEndTime();
}
#[no_mangle]
pub unsafe extern "C" fn areClientsPaused() -> libc::c_int {
    return (server.client_pause_type as libc::c_uint
        != CLIENT_PAUSE_OFF as libc::c_int as libc::c_uint) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int {
    if areClientsPaused() == 0 {
        return 0 as libc::c_int;
    }
    if server.client_pause_end_time < server.mstime {
        updateClientPauseTypeAndEndTime();
    }
    return areClientsPaused();
}
#[no_mangle]
pub unsafe extern "C" fn processEventsWhileBlocked() {
    let mut iterations: libc::c_int = 4 as libc::c_int;
    updateCachedTime(0 as libc::c_int);
    ProcessingEventsWhileBlocked += 1;
    loop {
        let fresh55 = iterations;
        iterations = iterations - 1;
        if !(fresh55 != 0) {
            break;
        }
        let mut startval: libc::c_longlong = server.events_processed_while_blocked;
        let mut ae_events: libc::c_longlong = aeProcessEvents(
            server.el,
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
        ) as libc::c_longlong;
        server.events_processed_while_blocked += ae_events;
        let mut events: libc::c_longlong = server.events_processed_while_blocked
            - startval;
        if events == 0 {
            break;
        }
    }
    whileBlockedCron();
    ProcessingEventsWhileBlocked -= 1;
    if ProcessingEventsWhileBlocked >= 0 as libc::c_int {} else {
        _serverAssert(
            b"ProcessingEventsWhileBlocked >= 0\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            4026 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub static mut io_threads: [pthread_t; 128] = [0; 128];
#[no_mangle]
pub static mut io_threads_mutex: [pthread_mutex_t; 128] = [pthread_mutex_t {
    __data: __pthread_mutex_s {
        __lock: 0,
        __count: 0,
        __owner: 0,
        __nusers: 0,
        __kind: 0,
        __spins: 0,
        __list: __pthread_list_t {
            __prev: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
            __next: 0 as *const __pthread_internal_list as *mut __pthread_internal_list,
        },
    },
}; 128];
#[no_mangle]
pub static mut io_threads_op: libc::c_int = 0;
#[no_mangle]
pub static mut io_threads_list: [*mut list; 128] = [0 as *const list as *mut list; 128];
#[inline]
unsafe extern "C" fn getIOPendingCount(mut i: libc::c_int) -> libc::c_ulong {
    let mut count: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut pending: libc::c_int = (*server.clients_pending_write).len as libc::c_int;
    count = core::intrinsics::atomic_load_seqcst(
        &mut threads_pending_cnt.0.value,
    );    
    return count;
}
#[inline]
unsafe extern "C" fn setIOPendingCount(mut i: libc::c_int, mut count: libc::c_ulong) {
    core::intrinsics::atomic_store_seqcst(
        &mut threads_pending_cnt.0.value,
        count,
    );
}
#[no_mangle]
pub unsafe extern "C" fn IOThreadMain(mut myid: *mut libc::c_void) -> *mut libc::c_void {
    let mut id: libc::c_long = myid as libc::c_ulong as libc::c_long;
    let mut thdname: [libc::c_char; 16] = [0; 16];
    snprintf(
        thdname.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
        b"io_thd_%ld\0" as *const u8 as *const libc::c_char,
        id,
    );
    pthread_setname_np(pthread_self(), thdname.as_mut_ptr());
    redisSetCpuAffinity(server.server_cpulist);
    makeThreadKillable();
    loop {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 1000000 as libc::c_int {
            if getIOPendingCount(id as libc::c_int) != 0 as libc::c_int as libc::c_ulong
            {
                break;
            }
            j += 1;
        }
        if getIOPendingCount(id as libc::c_int) == 0 as libc::c_int as libc::c_ulong {
            pthread_mutex_lock(&mut *io_threads_mutex.as_mut_ptr().offset(id as isize));
            pthread_mutex_unlock(
                &mut *io_threads_mutex.as_mut_ptr().offset(id as isize),
            );
        } else {
            if getIOPendingCount(id as libc::c_int) != 0 as libc::c_int as libc::c_ulong
            {} else {
                _serverAssert(
                    b"getIOPendingCount(id) != 0\0" as *const u8 as *const libc::c_char,
                    b"networking.c\0" as *const u8 as *const libc::c_char,
                    4090 as libc::c_int,
                );
                unreachable!();
            };
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            let mut ln: *mut listNode = 0 as *mut listNode;
            listRewind(io_threads_list[id as usize], &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut c: *mut client = (*ln).value as *mut client;
                if io_threads_op == 2 as libc::c_int {
                    writeToClient(c, 0 as libc::c_int);
                } else if io_threads_op == 1 as libc::c_int {
                    readQueryFromClient((*c).conn);
                } else {
                    _serverPanic(
                        b"networking.c\0" as *const u8 as *const libc::c_char,
                        4104 as libc::c_int,
                        b"io_threads_op value is unknown\0" as *const u8
                            as *const libc::c_char,
                    );
                    unreachable!();
                }
            }
            listEmpty(io_threads_list[id as usize]);
            setIOPendingCount(id as libc::c_int, 0 as libc::c_int as libc::c_ulong);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn initThreadedIO() {
    server.io_threads_active = 0 as libc::c_int;
    io_threads_op = 0 as libc::c_int;
    if server.io_threads_num == 1 as libc::c_int {
        return;
    }
    if server.io_threads_num > 128 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fatal: too many I/O threads configured. The maximum number is %d.\0"
                    as *const u8 as *const libc::c_char,
                128 as libc::c_int,
            );
        }
        exit(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < server.io_threads_num {
        io_threads_list[i as usize] = listCreate();
        if !(i == 0 as libc::c_int) {
            let mut tid: pthread_t = 0;
            pthread_mutex_init(
                &mut *io_threads_mutex.as_mut_ptr().offset(i as isize),
                0 as *const pthread_mutexattr_t,
            );
            setIOPendingCount(i, 0 as libc::c_int as libc::c_ulong);
            pthread_mutex_lock(&mut *io_threads_mutex.as_mut_ptr().offset(i as isize));
            if pthread_create(
                &mut tid,
                0 as *const pthread_attr_t,
                Some(
                    IOThreadMain
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                i as libc::c_long as *mut libc::c_void,
            ) != 0 as libc::c_int
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Fatal: Can't initialize IO thread.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
            io_threads[i as usize] = tid;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn killIOThreads() {
    let mut err: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < server.io_threads_num {
        if !(io_threads[j as usize] == pthread_self()) {
            if io_threads[j as usize] != 0
                && pthread_cancel(io_threads[j as usize]) == 0 as libc::c_int
            {
                err = pthread_join(io_threads[j as usize], 0 as *mut *mut libc::c_void);
                if err != 0 as libc::c_int {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"IO thread(tid:%lu) can not be joined: %s\0" as *const u8
                                as *const libc::c_char,
                            io_threads[j as usize],
                            strerror(err),
                        );
                    }
                } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                {
                    _serverLog(
                        3 as libc::c_int,
                        b"IO thread(tid:%lu) terminated\0" as *const u8
                            as *const libc::c_char,
                        io_threads[j as usize],
                    );
                }
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn startThreadedIO() {
    if server.io_threads_active == 0 as libc::c_int {} else {
        _serverAssert(
            b"server.io_threads_active == 0\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            4166 as libc::c_int,
        );
        unreachable!();
    };
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < server.io_threads_num {
        pthread_mutex_unlock(&mut *io_threads_mutex.as_mut_ptr().offset(j as isize));
        j += 1;
    }
    server.io_threads_active = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stopThreadedIO() {
    handleClientsWithPendingReadsUsingThreads();
    if server.io_threads_active == 1 as libc::c_int {} else {
        _serverAssert(
            b"server.io_threads_active == 1\0" as *const u8 as *const libc::c_char,
            b"networking.c\0" as *const u8 as *const libc::c_char,
            4176 as libc::c_int,
        );
        unreachable!();
    };
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < server.io_threads_num {
        pthread_mutex_lock(&mut *io_threads_mutex.as_mut_ptr().offset(j as isize));
        j += 1;
    }
    server.io_threads_active = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn stopThreadedIOIfNeeded() -> libc::c_int {
    let mut pending: libc::c_int = (*server.clients_pending_write).len as libc::c_int;
    if server.io_threads_num == 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if pending < server.io_threads_num * 2 as libc::c_int {
        if server.io_threads_active != 0 {
            stopThreadedIO();
        }
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn handleClientsWithPendingWritesUsingThreads() -> libc::c_int {
    let mut processed: libc::c_int = (*server.clients_pending_write).len as libc::c_int;
    if processed == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if server.io_threads_num == 1 as libc::c_int || stopThreadedIOIfNeeded() != 0 {
        return handleClientsWithPendingWrites();
    }
    if server.io_threads_active == 0 {
        startThreadedIO();
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients_pending_write, &mut li);
    let mut item_id: libc::c_int = 0 as libc::c_int;
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        (*c).flags &= !((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong;
        if (*c).flags & ((1 as libc::c_int) << 10 as libc::c_int) as libc::c_ulong != 0 {
            listDelNode(server.clients_pending_write, ln);
        } else if getClientType(c) == 1 as libc::c_int {
            listAddNodeTail(
                io_threads_list[0 as libc::c_int as usize],
                c as *mut libc::c_void,
            );
        } else {
            let mut target_id: libc::c_int = item_id % server.io_threads_num;
            listAddNodeTail(io_threads_list[target_id as usize], c as *mut libc::c_void);
            item_id += 1;
        }
    }
    io_threads_op = 2 as libc::c_int;
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < server.io_threads_num {
        let mut count: libc::c_int = (*io_threads_list[j as usize]).len as libc::c_int;
        setIOPendingCount(j, count as libc::c_ulong);
        j += 1;
    }
    listRewind(io_threads_list[0 as libc::c_int as usize], &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c_0: *mut client = (*ln).value as *mut client;
        writeToClient(c_0, 0 as libc::c_int);
    }
    listEmpty(io_threads_list[0 as libc::c_int as usize]);
    loop {
        let mut pending: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while j_0 < server.io_threads_num {
            pending = pending.wrapping_add(getIOPendingCount(j_0));
            j_0 += 1;
        }
        if pending == 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    io_threads_op = 0 as libc::c_int;
    listRewind(server.clients_pending_write, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c_1: *mut client = (*ln).value as *mut client;
        updateClientMemUsageAndBucket(c_1);
        if clientHasPendingReplies(c_1) != 0 {
            installClientWriteHandler(c_1);
        }
    }
    listEmpty(server.clients_pending_write);
    server.stat_io_writes_processed += processed as libc::c_longlong;
    return processed;
}
#[no_mangle]
pub unsafe extern "C" fn postponeClientRead(mut c: *mut client) -> libc::c_int {
    if server.io_threads_active != 0 && server.io_threads_do_reads != 0
        && ProcessingEventsWhileBlocked == 0
        && (*c).flags
            & ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong == 0
        && io_threads_op == 0 as libc::c_int
    {
        listAddNodeHead(server.clients_pending_read, c as *mut libc::c_void);
        (*c).pending_read_list_node = (*server.clients_pending_read).head;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn handleClientsWithPendingReadsUsingThreads() -> libc::c_int {
    if server.io_threads_active == 0 || server.io_threads_do_reads == 0 {
        return 0 as libc::c_int;
    }
    let mut processed: libc::c_int = (*server.clients_pending_read).len as libc::c_int;
    if processed == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients_pending_read, &mut li);
    let mut item_id: libc::c_int = 0 as libc::c_int;
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        let mut target_id: libc::c_int = item_id % server.io_threads_num;
        listAddNodeTail(io_threads_list[target_id as usize], c as *mut libc::c_void);
        item_id += 1;
    }
    io_threads_op = 1 as libc::c_int;
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < server.io_threads_num {
        let mut count: libc::c_int = (*io_threads_list[j as usize]).len as libc::c_int;
        setIOPendingCount(j, count as libc::c_ulong);
        j += 1;
    }
    listRewind(io_threads_list[0 as libc::c_int as usize], &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c_0: *mut client = (*ln).value as *mut client;
        readQueryFromClient((*c_0).conn);
    }
    listEmpty(io_threads_list[0 as libc::c_int as usize]);
    loop {
        let mut pending: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
        let mut j_0: libc::c_int = 1 as libc::c_int;
        while j_0 < server.io_threads_num {
            pending = pending.wrapping_add(getIOPendingCount(j_0));
            j_0 += 1;
        }
        if pending == 0 as libc::c_int as libc::c_ulong {
            break;
        }
    }
    io_threads_op = 0 as libc::c_int;
    while (*server.clients_pending_read).len != 0 {
        ln = (*server.clients_pending_read).head;
        let mut c_1: *mut client = (*ln).value as *mut client;
        listDelNode(server.clients_pending_read, ln);
        (*c_1).pending_read_list_node = 0 as *mut listNode;
        if (*c_1).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong == 0
        {} else {
            _serverAssert(
                b"!(c->flags & CLIENT_BLOCKED)\0" as *const u8 as *const libc::c_char,
                b"networking.c\0" as *const u8 as *const libc::c_char,
                4384 as libc::c_int,
            );
            unreachable!();
        };
        if !(beforeNextClient(c_1) == -(1 as libc::c_int)) {
            updateClientMemUsageAndBucket(c_1);
            if !(processPendingCommandAndInputBuffer(c_1) == -(1 as libc::c_int)) {
                if (*c_1).flags
                    & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong == 0
                    && clientHasPendingReplies(c_1) != 0
                {
                    putClientInPendingWriteQueue(c_1);
                }
            }
        }
    }
    server.stat_io_reads_processed += processed as libc::c_longlong;
    return processed;
}
#[no_mangle]
pub unsafe extern "C" fn getClientEvictionLimit() -> size_t {
    let mut maxmemory_clients_actual: size_t = 18446744073709551615 as libc::c_ulong;
    if server.maxmemory_clients < 0 as libc::c_int as libc::c_long
        && server.maxmemory > 0 as libc::c_int as libc::c_ulonglong
    {
        let mut maxmemory_clients_bytes: libc::c_ulonglong = (server.maxmemory
            as libc::c_double * -(server.maxmemory_clients as libc::c_double)
            / 100 as libc::c_int as libc::c_double) as libc::c_ulonglong;
        if maxmemory_clients_bytes
            <= 18446744073709551615 as libc::c_ulong as libc::c_ulonglong
        {
            maxmemory_clients_actual = maxmemory_clients_bytes as size_t;
        }
    } else if server.maxmemory_clients > 0 as libc::c_int as libc::c_long {
        maxmemory_clients_actual = server.maxmemory_clients as size_t;
    } else {
        return 0 as libc::c_int as size_t
    }
    if maxmemory_clients_actual
        < (1024 as libc::c_int * 128 as libc::c_int) as libc::c_ulong
    {
        maxmemory_clients_actual = (1024 as libc::c_int * 128 as libc::c_int) as size_t;
    }
    return maxmemory_clients_actual;
}
#[no_mangle]
pub unsafe extern "C" fn evictClients() {
    if (server.client_mem_usage_buckets).is_null() {
        return;
    }
    let mut curr_bucket: libc::c_int = 1 as libc::c_int + 33 as libc::c_int
        - 15 as libc::c_int - 1 as libc::c_int;
    let mut bucket_iter: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind(
        (*(server.client_mem_usage_buckets).offset(curr_bucket as isize)).clients,
        &mut bucket_iter,
    );
    let mut client_eviction_limit: size_t = getClientEvictionLimit();
    if client_eviction_limit == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    while (server.stat_clients_type_memory[0 as libc::c_int as usize])
        .wrapping_add(server.stat_clients_type_memory[2 as libc::c_int as usize])
        >= client_eviction_limit
    {
        let mut ln: *mut listNode = listNext(&mut bucket_iter);
        if !ln.is_null() {
            let mut c: *mut client = (*ln).value as *mut client;
            let mut ci: sds = catClientInfoString(sdsempty(), c);
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Evicting client: %s\0" as *const u8 as *const libc::c_char,
                    ci,
                );
            }
            freeClient(c);
            sdsfree(ci);
            server.stat_evictedclients += 1;
        } else {
            curr_bucket -= 1;
            if curr_bucket < 0 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Over client maxmemory after evicting all evictable clients\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                break;
            } else {
                listRewind(
                    (*(server.client_mem_usage_buckets).offset(curr_bucket as isize))
                        .clients,
                    &mut bucket_iter,
                );
            }
        }
    }
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleDefragCtx;
    pub type RedisModuleInfoCtx;
    pub type RedisModuleCommand;
    pub type dirent;
    pub type ModuleConfig;
    fn stringmatch(
        p: *const libc::c_char,
        s: *const libc::c_char,
        nocase: libc::c_int,
    ) -> libc::c_int;
    fn memtoull(p: *const libc::c_char, err: *mut libc::c_int) -> libc::c_ulonglong;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn ull2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_ulonglong,
    ) -> libc::c_int;
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    static mut SDS_NOINIT: *const libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdscmp(s1: sds, s2: sds) -> libc::c_int;
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdstolower(s: sds);
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsjoin(
        argv: *mut *mut libc::c_char,
        argc: libc::c_int,
        sep: *mut libc::c_char,
    ) -> sds;
    fn __errno_location() -> *mut libc::c_int;
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
    fn mkostemp(__template: *mut libc::c_char, __flags: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn chdir(__path: *const libc::c_char) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn aeGetSetSize(eventLoop: *mut aeEventLoop) -> libc::c_int;
    fn aeResizeSetSize(eventLoop: *mut aeEventLoop, setsize: libc::c_int) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictReplace(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn zmalloc_used_memory() -> size_t;
    fn set_jemalloc_bg_thread(enable: libc::c_int);
    fn string2d(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut libc::c_double,
    ) -> libc::c_int;
    fn trimDoubleString(buf: *mut libc::c_char, len: size_t) -> libc::c_int;
    fn pathIsBaseName(path: *mut libc::c_char) -> libc::c_int;
    fn fsyncFileDir(filename: *const libc::c_char) -> libc::c_int;
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut externalStringType: dictType;
    static mut sdsHashDictType: dictType;
    static mut modules: *mut dict;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn redisSetProcTitle(title: *mut libc::c_char) -> libc::c_int;
    fn validateProcTitleTemplate(template: *const libc::c_char) -> libc::c_int;
    fn acceptTcpHandler(
        el: *mut aeEventLoop,
        fd: libc::c_int,
        privdata: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn redactClientCommandArgument(c: *mut client, argc: libc::c_int);
    fn getClientTypeByName(name: *mut libc::c_char) -> libc::c_int;
    fn getClientTypeName(class: libc::c_int) -> *mut libc::c_char;
    fn islocalClient(c: *mut client) -> libc::c_int;
    fn updateClientMemUsageAndBucket(c: *mut client) -> libc::c_int;
    fn removeClientFromMemUsageBucket(c: *mut client, allow_eviction: libc::c_int);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn stopAppendOnly();
    fn startAppendOnly() -> libc::c_int;
    fn aofDelHistoryFiles() -> libc::c_int;
    static mut Users: *mut rax;
    fn ACLAppendUserForLoading(
        argv: *mut sds,
        argc: libc::c_int,
        argc_err: *mut libc::c_int,
    ) -> libc::c_int;
    fn ACLSetUserStringError() -> *const libc::c_char;
    fn ACLDescribeUser(u: *mut user) -> *mut robj;
    fn ACLUpdateDefaultUserPassword(password: sds);
    fn freeMemoryGetNotCountedMemory() -> size_t;
    fn setupSignalHandlers();
    fn removeSignalHandlers();
    fn changeListenPort(
        port: libc::c_int,
        sfd: *mut socketFds,
        accept_handler: Option::<aeFileProc>,
    ) -> libc::c_int;
    fn changeBindAddr() -> libc::c_int;
    fn lookupCommandBySds(s: sds) -> *mut redisCommand;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn serverLogRaw(level: libc::c_int, msg: *const libc::c_char);
    fn resetCommandTableStats(commands: *mut dict);
    fn resetErrorTableStats();
    fn adjustOpenFilesLimit();
    fn resetServerStats();
    fn refreshGoodSlavesCount();
    fn resizeReplicationBacklog();
    fn setOOMScoreAdj(process_class: libc::c_int) -> libc::c_int;
    fn keyspaceEventsStringToFlags(classes: *mut libc::c_char) -> libc::c_int;
    fn keyspaceEventsFlagsToString(flags: libc::c_int) -> sds;
    fn queueSentinelConfig(
        argv: *mut sds,
        argc: libc::c_int,
        linenum: libc::c_int,
        line: sds,
    );
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsCaseHash(key: *const libc::c_void) -> uint64_t;
    fn getModuleNumericConfig(module_config: *mut ModuleConfig) -> libc::c_longlong;
    fn setModuleNumericConfig(
        config: *mut ModuleConfig,
        val: libc::c_longlong,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn freeServerClientMemUsageBuckets();
    fn initServerClientMemUsageBuckets();
    fn startEvictionTimeProc();
    fn getModuleEnumConfig(module_config: *mut ModuleConfig) -> libc::c_int;
    fn setModuleEnumConfig(
        config: *mut ModuleConfig,
        val: libc::c_int,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn createRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn decrRefCount(o: *mut robj);
    fn getModuleStringConfig(module_config: *mut ModuleConfig) -> sds;
    fn getModuleBoolConfig(module_config: *mut ModuleConfig) -> libc::c_int;
    fn setModuleBoolConfig(
        config: *mut ModuleConfig,
        val: libc::c_int,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn addModuleConfigApply(module_configs: *mut list, module_config: *mut ModuleConfig);
    fn moduleConfigApplyConfig(
        module_configs: *mut list,
        err: *mut *const libc::c_char,
        err_arg_name: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn setModuleStringConfig(
        config: *mut ModuleConfig,
        strval: sds,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
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
    fn applyWatchdogPeriod();
    fn clusterUpdateMyselfFlags();
    fn clusterUpdateMyselfIp();
    fn clusterUpdateMyselfHostname();
    fn clusterUpdateMyselfAnnouncedPorts();
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn glob(
        __pattern: *const libc::c_char,
        __flags: libc::c_int,
        __errfunc: Option::<
            unsafe extern "C" fn(*const libc::c_char, libc::c_int) -> libc::c_int,
        >,
        __pglob: *mut glob_t,
    ) -> libc::c_int;
    fn globfree(__pglob: *mut glob_t);
    fn dictListDestructor(d: *mut dict, val: *mut libc::c_void);
    fn rewriteConfigSentinelOption(state: *mut rewriteConfigState);
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
pub type pthread_t = libc::c_ulong;
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
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleConfigChange {
    pub version: uint64_t,
    pub num_changes: uint32_t,
    pub config_names: *mut *const libc::c_char,
}
pub type RedisModuleConfigChangeV1 = RedisModuleConfigChange;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_0 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_0 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_0 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_0 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_0 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_0 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_0 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_0 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_0 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_0 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_0 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_0 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_0 = 0;
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const CLUSTER_ENDPOINT_TYPE_UNKNOWN_ENDPOINT: C2RustUnnamed_1 = 2;
pub const CLUSTER_ENDPOINT_TYPE_HOSTNAME: C2RustUnnamed_1 = 1;
pub const CLUSTER_ENDPOINT_TYPE_IP: C2RustUnnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModule {
    pub handle: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub ver: libc::c_int,
    pub apiver: libc::c_int,
    pub types: *mut list,
    pub usedby: *mut list,
    pub using: *mut list,
    pub filters: *mut list,
    pub module_configs: *mut list,
    pub configs_initialized: libc::c_int,
    pub in_call: libc::c_int,
    pub in_hook: libc::c_int,
    pub options: libc::c_int,
    pub blocked_clients: libc::c_int,
    pub info_cb: RedisModuleInfoFunc,
    pub defrag_cb: RedisModuleDefragFunc,
    pub loadmod: *mut moduleLoadQueueEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleLoadQueueEntry {
    pub path: sds,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
}
pub type robj = redisObject;
pub type RedisModuleDefragFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> (),
>;
pub type RedisModuleInfoFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleInfoCtx, libc::c_int) -> (),
>;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct configEnum {
    pub name: *mut libc::c_char,
    pub val: libc::c_int,
}
pub type configType = libc::c_uint;
pub const SPECIAL_CONFIG: configType = 5;
pub const ENUM_CONFIG: configType = 4;
pub const SDS_CONFIG: configType = 3;
pub const STRING_CONFIG: configType = 2;
pub const NUMERIC_CONFIG: configType = 1;
pub const BOOL_CONFIG: configType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deprecatedConfig {
    pub name: *const libc::c_char,
    pub argc_min: libc::c_int,
    pub argc_max: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct standardConfig {
    pub name: *const libc::c_char,
    pub alias: *const libc::c_char,
    pub flags: libc::c_uint,
    pub interface: typeInterface,
    pub data: typeData,
    pub type_0: configType,
    pub privdata: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union typeData {
    pub yesno: boolConfigData,
    pub string: stringConfigData,
    pub sds: sdsConfigData,
    pub enumd: enumConfigData,
    pub numeric: numericConfigData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct numericConfigData {
    pub config: C2RustUnnamed_10,
    pub flags: libc::c_uint,
    pub numeric_type: numericType,
    pub lower_bound: libc::c_longlong,
    pub upper_bound: libc::c_longlong,
    pub default_value: libc::c_longlong,
    pub is_valid_fn: Option::<
        unsafe extern "C" fn(libc::c_longlong, *mut *const libc::c_char) -> libc::c_int,
    >,
}
pub type numericType = libc::c_uint;
pub const NUMERIC_TYPE_TIME_T: numericType = 9;
pub const NUMERIC_TYPE_OFF_T: numericType = 8;
pub const NUMERIC_TYPE_SSIZE_T: numericType = 7;
pub const NUMERIC_TYPE_SIZE_T: numericType = 6;
pub const NUMERIC_TYPE_ULONG_LONG: numericType = 5;
pub const NUMERIC_TYPE_LONG_LONG: numericType = 4;
pub const NUMERIC_TYPE_ULONG: numericType = 3;
pub const NUMERIC_TYPE_LONG: numericType = 2;
pub const NUMERIC_TYPE_UINT: numericType = 1;
pub const NUMERIC_TYPE_INT: numericType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub i: *mut libc::c_int,
    pub ui: *mut libc::c_uint,
    pub l: *mut libc::c_long,
    pub ul: *mut libc::c_ulong,
    pub ll: *mut libc::c_longlong,
    pub ull: *mut libc::c_ulonglong,
    pub st: *mut size_t,
    pub sst: *mut ssize_t,
    pub ot: *mut off_t,
    pub tt: *mut time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct enumConfigData {
    pub config: *mut libc::c_int,
    pub enum_value: *mut configEnum,
    pub default_value: libc::c_int,
    pub is_valid_fn: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *const libc::c_char) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sdsConfigData {
    pub config: *mut sds,
    pub default_value: *mut libc::c_char,
    pub is_valid_fn: Option::<
        unsafe extern "C" fn(sds, *mut *const libc::c_char) -> libc::c_int,
    >,
    pub convert_empty_to_null: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stringConfigData {
    pub config: *mut *mut libc::c_char,
    pub default_value: *const libc::c_char,
    pub is_valid_fn: Option::<
        unsafe extern "C" fn(*mut libc::c_char, *mut *const libc::c_char) -> libc::c_int,
    >,
    pub convert_empty_to_null: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct boolConfigData {
    pub config: *mut libc::c_int,
    pub default_value: libc::c_int,
    pub is_valid_fn: Option::<
        unsafe extern "C" fn(libc::c_int, *mut *const libc::c_char) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct typeInterface {
    pub init: Option::<unsafe extern "C" fn(*mut standardConfig) -> ()>,
    pub set: Option::<
        unsafe extern "C" fn(
            *mut standardConfig,
            *mut sds,
            libc::c_int,
            *mut *const libc::c_char,
        ) -> libc::c_int,
    >,
    pub apply: apply_fn,
    pub get: Option::<unsafe extern "C" fn(*mut standardConfig) -> sds>,
    pub rewrite: Option::<
        unsafe extern "C" fn(
            *mut standardConfig,
            *const libc::c_char,
            *mut rewriteConfigState,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rewriteConfigState {
    pub option_to_line: *mut dict,
    pub rewritten: *mut dict,
    pub numlines: libc::c_int,
    pub lines: *mut sds,
    pub needs_signature: libc::c_int,
    pub force_write: libc::c_int,
}
pub type apply_fn = Option::<
    unsafe extern "C" fn(*mut *const libc::c_char) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct glob_t {
    pub gl_pathc: __size_t,
    pub gl_pathv: *mut *mut libc::c_char,
    pub gl_offs: __size_t,
    pub gl_flags: libc::c_int,
    pub gl_closedir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub gl_readdir: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut dirent>,
    pub gl_opendir: Option::<
        unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_void,
    >,
    pub gl_lstat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
    pub gl_stat: Option::<
        unsafe extern "C" fn(*const libc::c_char, *mut stat) -> libc::c_int,
    >,
}
pub type __size_t = libc::c_ulong;
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
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut maxmemory_policy_enum: [configEnum; 9] = [
    {
        let mut init = configEnum {
            name: b"volatile-lru\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (0 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"volatile-lfu\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (1 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"volatile-random\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (3 as libc::c_int) << 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"volatile-ttl\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (2 as libc::c_int) << 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"allkeys-lru\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (4 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"allkeys-lfu\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (5 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"allkeys-random\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (6 as libc::c_int) << 8 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"noeviction\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (7 as libc::c_int) << 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut syslog_facility_enum: [configEnum; 10] = [
    {
        let mut init = configEnum {
            name: b"user\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (1 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local0\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (16 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (17 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (18 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local3\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (19 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local4\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (20 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local5\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (21 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local6\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (22 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local7\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: (23 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut loglevel_enum: [configEnum; 5] = [
    {
        let mut init = configEnum {
            name: b"debug\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"verbose\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"notice\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"warning\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut supervised_mode_enum: [configEnum; 5] = [
    {
        let mut init = configEnum {
            name: b"upstart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"systemd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"auto\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut aof_fsync_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"everysec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"always\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut shutdown_on_sig_enum: [configEnum; 6] = [
    {
        let mut init = configEnum {
            name: b"default\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"save\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"nosave\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"now\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 4 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"force\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 8 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut repl_diskless_load_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"disabled\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"on-empty-db\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"swapdb\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut tls_auth_clients_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"yes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"optional\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut oom_score_adj_enum: [configEnum; 5] = [
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"yes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"relative\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"absolute\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut acl_pubsub_default_enum: [configEnum; 3] = [
    {
        let mut init = configEnum {
            name: b"allchannels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: (1 as libc::c_int) << 3 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"resetchannels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut sanitize_dump_payload_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"yes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"clients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut protected_action_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"no\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"yes\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 1 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"local\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: 2 as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut cluster_preferred_endpoint_type_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"ip\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: CLUSTER_ENDPOINT_TYPE_IP as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"hostname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: CLUSTER_ENDPOINT_TYPE_HOSTNAME as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"unknown-endpoint\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: CLUSTER_ENDPOINT_TYPE_UNKNOWN_ENDPOINT as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut propagation_error_behavior_enum: [configEnum; 4] = [
    {
        let mut init = configEnum {
            name: b"ignore\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: PROPAGATION_ERR_BEHAVIOR_IGNORE as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"panic\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            val: PROPAGATION_ERR_BEHAVIOR_PANIC as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: b"panic-on-replicas\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            val: PROPAGATION_ERR_BEHAVIOR_PANIC_ON_REPLICAS as libc::c_int,
        };
        init
    },
    {
        let mut init = configEnum {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            val: 0 as libc::c_int,
        };
        init
    },
];
#[no_mangle]
pub static mut clientBufferLimitsDefaults: [clientBufferLimitsConfig; 3] = [
    {
        let mut init = clientBufferLimitsConfig {
            hard_limit_bytes: 0 as libc::c_int as libc::c_ulonglong,
            soft_limit_bytes: 0 as libc::c_int as libc::c_ulonglong,
            soft_limit_seconds: 0 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = clientBufferLimitsConfig {
            hard_limit_bytes: (1024 as libc::c_int * 1024 as libc::c_int
                * 256 as libc::c_int) as libc::c_ulonglong,
            soft_limit_bytes: (1024 as libc::c_int * 1024 as libc::c_int
                * 64 as libc::c_int) as libc::c_ulonglong,
            soft_limit_seconds: 60 as libc::c_int as time_t,
        };
        init
    },
    {
        let mut init = clientBufferLimitsConfig {
            hard_limit_bytes: (1024 as libc::c_int * 1024 as libc::c_int
                * 32 as libc::c_int) as libc::c_ulonglong,
            soft_limit_bytes: (1024 as libc::c_int * 1024 as libc::c_int
                * 8 as libc::c_int) as libc::c_ulonglong,
            soft_limit_seconds: 60 as libc::c_int as time_t,
        };
        init
    },
];
#[no_mangle]
pub static mut configOOMScoreAdjValuesDefaults: [libc::c_int; 3] = [
    0 as libc::c_int,
    200 as libc::c_int,
    800 as libc::c_int,
];
#[no_mangle]
pub static mut configs: *mut dict = 0 as *const dict as *mut dict;
unsafe extern "C" fn lookupConfig(mut name: sds) -> *mut standardConfig {
    let mut de: *mut dictEntry = dictFind(configs, name as *const libc::c_void);
    return (if !de.is_null() { (*de).v.val } else { 0 as *mut libc::c_void })
        as *mut standardConfig;
}
#[no_mangle]
pub unsafe extern "C" fn configEnumGetValue(
    mut ce: *mut configEnum,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut bitflags: libc::c_int,
) -> libc::c_int {
    if argc == 0 as libc::c_int || bitflags == 0 && argc != 1 as libc::c_int {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    let mut values: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        let mut matched: libc::c_int = 0 as libc::c_int;
        let mut ceItem: *mut configEnum = ce;
        while !((*ceItem).name).is_null() {
            if strcasecmp(
                *argv.offset(i as isize) as *const libc::c_char,
                (*ceItem).name,
            ) == 0
            {
                values |= (*ceItem).val;
                matched = 1 as libc::c_int;
            }
            ceItem = ceItem.offset(1);
        }
        if matched == 0 {
            return -(2147483647 as libc::c_int) - 1 as libc::c_int;
        }
        i += 1;
    }
    return values;
}
unsafe extern "C" fn configEnumGetName(
    mut ce: *mut configEnum,
    mut values: libc::c_int,
    mut bitflags: libc::c_int,
) -> sds {
    let mut names: sds = 0 as sds;
    let mut unmatched: libc::c_int = values;
    while !((*ce).name).is_null() {
        if values == (*ce).val {
            sdsfree(names);
            return sdsnew((*ce).name);
        }
        if bitflags != 0 && (*ce).val != 0 && (*ce).val == unmatched & (*ce).val {
            names = if !names.is_null() {
                sdscatfmt(
                    names,
                    b" %s\0" as *const u8 as *const libc::c_char,
                    (*ce).name,
                )
            } else {
                sdsnew((*ce).name)
            };
            unmatched &= !(*ce).val;
        }
        ce = ce.offset(1);
    }
    if names.is_null() || unmatched != 0 {
        sdsfree(names);
        return sdsnew(b"unknown\0" as *const u8 as *const libc::c_char);
    }
    return names;
}
#[no_mangle]
pub unsafe extern "C" fn evictPolicyToString() -> *const libc::c_char {
    let mut ce: *mut configEnum = maxmemory_policy_enum.as_mut_ptr();
    while !((*ce).name).is_null() {
        if server.maxmemory_policy == (*ce).val {
            return (*ce).name;
        }
        ce = ce.offset(1);
    }
    _serverPanic(
        b"config.c\0" as *const u8 as *const libc::c_char,
        341 as libc::c_int,
        b"unknown eviction policy\0" as *const u8 as *const libc::c_char,
    );
    unreachable!();
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn yesnotoi(mut s: *mut libc::c_char) -> libc::c_int {
    if strcasecmp(s, b"yes\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int
    } else if strcasecmp(s, b"no\0" as *const u8 as *const libc::c_char) == 0 {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn appendServerSaveParams(
    mut seconds: time_t,
    mut changes: libc::c_int,
) {
    server
        .saveparams = zrealloc(
        server.saveparams as *mut libc::c_void,
        (core::mem::size_of::<saveparam>() as libc::c_ulong)
            .wrapping_mul((server.saveparamslen + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut saveparam;
    (*(server.saveparams).offset(server.saveparamslen as isize)).seconds = seconds;
    (*(server.saveparams).offset(server.saveparamslen as isize)).changes = changes;
    server.saveparamslen += 1;
}
#[no_mangle]
pub unsafe extern "C" fn resetServerSaveParams() {
    zfree(server.saveparams as *mut libc::c_void);
    server.saveparams = 0 as *mut saveparam;
    server.saveparamslen = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn queueLoadModule(
    mut path: sds,
    mut argv: *mut sds,
    mut argc: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut loadmod: *mut moduleLoadQueueEntry = 0 as *mut moduleLoadQueueEntry;
    loadmod = zmalloc(core::mem::size_of::<moduleLoadQueueEntry>() as libc::c_ulong)
        as *mut moduleLoadQueueEntry;
    (*loadmod)
        .argv = (if argc != 0 {
        zmalloc(
            (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                .wrapping_mul(argc as libc::c_ulong),
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut *mut robj;
    (*loadmod).path = sdsnew(path as *const libc::c_char);
    (*loadmod).argc = argc;
    i = 0 as libc::c_int;
    while i < argc {
        let ref mut fresh0 = *((*loadmod).argv).offset(i as isize);
        *fresh0 = createRawStringObject(
            *argv.offset(i as isize) as *const libc::c_char,
            sdslen(*argv.offset(i as isize)),
        );
        i += 1;
    }
    listAddNodeTail(server.loadmodule_queue, loadmod as *mut libc::c_void);
}
unsafe extern "C" fn updateClientOutputBufferLimit(
    mut args: *mut sds,
    mut arg_len: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut class: libc::c_int = 0;
    let mut hard: libc::c_ulonglong = 0;
    let mut soft: libc::c_ulonglong = 0;
    let mut hard_err: libc::c_int = 0;
    let mut soft_err: libc::c_int = 0;
    let mut soft_seconds: libc::c_int = 0;
    let mut soft_seconds_eptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut values: [clientBufferLimitsConfig; 3] = [clientBufferLimitsConfig {
        hard_limit_bytes: 0,
        soft_limit_bytes: 0,
        soft_limit_seconds: 0,
    }; 3];
    let mut classes: [libc::c_int; 3] = [0 as libc::c_int, 0, 0];
    if arg_len % 4 as libc::c_int != 0 {
        if !err.is_null() {
            *err = b"Wrong number of arguments in buffer limit configuration.\0"
                as *const u8 as *const libc::c_char;
        }
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < arg_len {
        class = getClientTypeByName(*args.offset(j as isize));
        if class == -(1 as libc::c_int) || class == 3 as libc::c_int {
            if !err.is_null() {
                *err = b"Invalid client class specified in buffer limit configuration.\0"
                    as *const u8 as *const libc::c_char;
            }
            return 0 as libc::c_int;
        }
        hard = memtoull(
            *args.offset((j + 1 as libc::c_int) as isize) as *const libc::c_char,
            &mut hard_err,
        );
        soft = memtoull(
            *args.offset((j + 2 as libc::c_int) as isize) as *const libc::c_char,
            &mut soft_err,
        );
        soft_seconds = strtoll(
            *args.offset((j + 3 as libc::c_int) as isize) as *const libc::c_char,
            &mut soft_seconds_eptr,
            10 as libc::c_int,
        ) as libc::c_int;
        if hard_err != 0 || soft_err != 0 || soft_seconds < 0 as libc::c_int
            || *soft_seconds_eptr as libc::c_int != '\0' as i32
        {
            if !err.is_null() {
                *err = b"Error in hard, soft or soft_seconds setting in buffer limit configuration.\0"
                    as *const u8 as *const libc::c_char;
            }
            return 0 as libc::c_int;
        }
        values[class as usize].hard_limit_bytes = hard;
        values[class as usize].soft_limit_bytes = soft;
        values[class as usize].soft_limit_seconds = soft_seconds as time_t;
        classes[class as usize] = 1 as libc::c_int;
        j += 4 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if classes[j as usize] != 0 {
            server.client_obuf_limits[j as usize] = values[j as usize];
        }
        j += 1;
    }
    return 1 as libc::c_int;
}
static mut reading_config_file: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn loadServerConfigFromString(mut config: *mut libc::c_char) {
    let mut current_block: u64;
    let mut deprecated_configs: [deprecatedConfig; 4] = [
        {
            let mut init = deprecatedConfig {
                name: b"list-max-ziplist-entries\0" as *const u8 as *const libc::c_char,
                argc_min: 2 as libc::c_int,
                argc_max: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = deprecatedConfig {
                name: b"list-max-ziplist-value\0" as *const u8 as *const libc::c_char,
                argc_min: 2 as libc::c_int,
                argc_max: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = deprecatedConfig {
                name: b"lua-replicate-commands\0" as *const u8 as *const libc::c_char,
                argc_min: 2 as libc::c_int,
                argc_max: 2 as libc::c_int,
            };
            init
        },
        {
            let mut init = deprecatedConfig {
                name: 0 as *const libc::c_char,
                argc_min: 0 as libc::c_int,
                argc_max: 0,
            };
            init
        },
    ];
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut linenum: libc::c_int = 0 as libc::c_int;
    let mut totlines: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lines: *mut sds = 0 as *mut sds;
    reading_config_file = 1 as libc::c_int;
    lines = sdssplitlen(
        config,
        strlen(config) as ssize_t,
        b"\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut totlines,
    );
    i = 0 as libc::c_int;
    loop {
        if !(i < totlines) {
            current_block = 7189308829251266000;
            break;
        }
        let mut argv: *mut sds = 0 as *mut sds;
        let mut argc: libc::c_int = 0;
        linenum = i + 1 as libc::c_int;
        let ref mut fresh1 = *lines.offset(i as isize);
        *fresh1 = sdstrim(
            *lines.offset(i as isize),
            b" \t\r\n\0" as *const u8 as *const libc::c_char,
        );
        if !(*(*lines.offset(i as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int == '#' as i32
            || *(*lines.offset(i as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '\0' as i32)
        {
            argv = sdssplitargs(
                *lines.offset(i as isize) as *const libc::c_char,
                &mut argc,
            );
            if argv.is_null() {
                err = b"Unbalanced quotes in configuration line\0" as *const u8
                    as *const libc::c_char;
                current_block = 4202576858725330657;
                break;
            } else if argc == 0 as libc::c_int {
                sdsfreesplitres(argv, argc);
            } else {
                sdstolower(*argv.offset(0 as libc::c_int as isize));
                let mut config_0: *mut standardConfig = lookupConfig(
                    *argv.offset(0 as libc::c_int as isize),
                );
                if !config_0.is_null() {
                    if (*config_0).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 3 as libc::c_int == 0
                        && argc != 2 as libc::c_int
                    {
                        err = b"wrong number of arguments\0" as *const u8
                            as *const libc::c_char;
                        current_block = 4202576858725330657;
                        break;
                    } else {
                        if (*config_0).flags as libc::c_ulonglong
                            & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0
                            && argc == 2 as libc::c_int
                            && sdslen(*argv.offset(1 as libc::c_int as isize)) != 0
                        {
                            let mut new_argv: *mut sds = 0 as *mut sds;
                            let mut new_argc: libc::c_int = 0;
                            new_argv = sdssplitargs(
                                *argv.offset(1 as libc::c_int as isize)
                                    as *const libc::c_char,
                                &mut new_argc,
                            );
                            if ((*config_0).interface.set)
                                .expect(
                                    "non-null function pointer",
                                )(config_0, new_argv, new_argc, &mut err) == 0
                            {
                                current_block = 4202576858725330657;
                                break;
                            }
                            sdsfreesplitres(new_argv, new_argc);
                        } else if ((*config_0).interface.set)
                            .expect(
                                "non-null function pointer",
                            )(
                            config_0,
                            &mut *argv.offset(1 as libc::c_int as isize),
                            argc - 1 as libc::c_int,
                            &mut err,
                        ) == 0
                        {
                            current_block = 4202576858725330657;
                            break;
                        }
                        sdsfreesplitres(argv, argc);
                    }
                } else {
                    let mut match_0: libc::c_int = 0 as libc::c_int;
                    let mut config_1: *mut deprecatedConfig = deprecated_configs
                        .as_mut_ptr();
                    while !((*config_1).name).is_null() {
                        if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            (*config_1).name,
                        ) == 0 && (*config_1).argc_min <= argc
                            && argc <= (*config_1).argc_max
                        {
                            match_0 = 1 as libc::c_int;
                            break;
                        } else {
                            config_1 = config_1.offset(1);
                        }
                    }
                    if match_0 != 0 {
                        sdsfreesplitres(argv, argc);
                    } else {
                        if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"include\0" as *const u8 as *const libc::c_char,
                        ) == 0 && argc == 2 as libc::c_int
                        {
                            loadServerConfig(
                                *argv.offset(1 as libc::c_int as isize),
                                0 as libc::c_int as libc::c_char,
                                0 as *mut libc::c_char,
                            );
                        } else if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"rename-command\0" as *const u8 as *const libc::c_char,
                        ) == 0 && argc == 3 as libc::c_int
                        {
                            let mut cmd: *mut redisCommand = lookupCommandBySds(
                                *argv.offset(1 as libc::c_int as isize),
                            );
                            let mut retval: libc::c_int = 0;
                            if cmd.is_null() {
                                err = b"No such command in rename-command\0" as *const u8
                                    as *const libc::c_char;
                                current_block = 4202576858725330657;
                                break;
                            } else {
                                retval = dictDelete(
                                    server.commands,
                                    *argv.offset(1 as libc::c_int as isize)
                                        as *const libc::c_void,
                                );
                                if retval == 0 as libc::c_int {} else {
                                    _serverAssert(
                                        b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
                                        b"config.c\0" as *const u8 as *const libc::c_char,
                                        543 as libc::c_int,
                                    );
                                    unreachable!();
                                };
                                if sdslen(*argv.offset(2 as libc::c_int as isize))
                                    != 0 as libc::c_int as libc::c_ulong
                                {
                                    let mut copy: sds = sdsdup(
                                        *argv.offset(2 as libc::c_int as isize),
                                    );
                                    retval = dictAdd(
                                        server.commands,
                                        copy as *mut libc::c_void,
                                        cmd as *mut libc::c_void,
                                    );
                                    if retval != 0 as libc::c_int {
                                        sdsfree(copy);
                                        err = b"Target command name already exists\0" as *const u8
                                            as *const libc::c_char;
                                        current_block = 4202576858725330657;
                                        break;
                                    }
                                }
                            }
                        } else if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"user\0" as *const u8 as *const libc::c_char,
                        ) == 0 && argc >= 2 as libc::c_int
                        {
                            let mut argc_err: libc::c_int = 0;
                            if ACLAppendUserForLoading(argv, argc, &mut argc_err)
                                == -(1 as libc::c_int)
                            {
                                let mut errmsg: *const libc::c_char = ACLSetUserStringError();
                                snprintf(
                                    buf.as_mut_ptr(),
                                    core::mem::size_of::<[libc::c_char; 1024]>()
                                        as libc::c_ulong,
                                    b"Error in user declaration '%s': %s\0" as *const u8
                                        as *const libc::c_char,
                                    *argv.offset(argc_err as isize),
                                    errmsg,
                                );
                                err = buf.as_mut_ptr();
                                current_block = 4202576858725330657;
                                break;
                            }
                        } else if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"loadmodule\0" as *const u8 as *const libc::c_char,
                        ) == 0 && argc >= 2 as libc::c_int
                        {
                            queueLoadModule(
                                *argv.offset(1 as libc::c_int as isize),
                                &mut *argv.offset(2 as libc::c_int as isize),
                                argc - 2 as libc::c_int,
                            );
                        } else if !(strchr(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            '.' as i32,
                        ))
                            .is_null()
                        {
                            if argc < 2 as libc::c_int {
                                err = b"Module config specified without value\0"
                                    as *const u8 as *const libc::c_char;
                                current_block = 4202576858725330657;
                                break;
                            } else {
                                let mut name: sds = sdsdup(
                                    *argv.offset(0 as libc::c_int as isize),
                                );
                                let mut val: sds = sdsdup(
                                    *argv.offset(1 as libc::c_int as isize),
                                );
                                let mut i_0: libc::c_int = 2 as libc::c_int;
                                while i_0 < argc {
                                    val = sdscatfmt(
                                        val,
                                        b" %S\0" as *const u8 as *const libc::c_char,
                                        *argv.offset(i_0 as isize),
                                    );
                                    i_0 += 1;
                                }
                                if dictReplace(
                                    server.module_configs_queue,
                                    name as *mut libc::c_void,
                                    val as *mut libc::c_void,
                                ) == 0
                                {
                                    sdsfree(name);
                                }
                            }
                        } else if strcasecmp(
                            *argv.offset(0 as libc::c_int as isize)
                                as *const libc::c_char,
                            b"sentinel\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            if argc != 1 as libc::c_int {
                                if server.sentinel_mode == 0 {
                                    err = b"sentinel directive while not in sentinel mode\0"
                                        as *const u8 as *const libc::c_char;
                                    current_block = 4202576858725330657;
                                    break;
                                } else {
                                    queueSentinelConfig(
                                        argv.offset(1 as libc::c_int as isize),
                                        argc - 1 as libc::c_int,
                                        linenum,
                                        *lines.offset(i as isize),
                                    );
                                }
                            }
                        } else {
                            err = b"Bad directive or wrong number of arguments\0"
                                as *const u8 as *const libc::c_char;
                            current_block = 4202576858725330657;
                            break;
                        }
                        sdsfreesplitres(argv, argc);
                    }
                }
            }
        }
        i += 1;
    }
    match current_block {
        7189308829251266000 => {
            if *(server.logfile).offset(0 as libc::c_int as isize) as libc::c_int
                != '\0' as i32
            {
                let mut logfp: *mut FILE = 0 as *mut FILE;
                logfp = fopen(
                    server.logfile,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                if logfp.is_null() {
                    err = sdscatprintf(
                        sdsempty(),
                        b"Can't open the log file: %s\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    ) as *const libc::c_char;
                    current_block = 4202576858725330657;
                } else {
                    fclose(logfp);
                    current_block = 17239133558811367971;
                }
            } else {
                current_block = 17239133558811367971;
            }
            match current_block {
                4202576858725330657 => {}
                _ => {
                    if server.cluster_enabled != 0 && !(server.masterhost).is_null() {
                        err = b"replicaof directive not allowed in cluster mode\0"
                            as *const u8 as *const libc::c_char;
                    } else {
                        if server.config_hz < 1 as libc::c_int {
                            server.config_hz = 1 as libc::c_int;
                        }
                        if server.config_hz > 500 as libc::c_int {
                            server.config_hz = 500 as libc::c_int;
                        }
                        sdsfreesplitres(lines, totlines);
                        reading_config_file = 0 as libc::c_int;
                        return;
                    }
                }
            }
        }
        _ => {}
    }
    fprintf(
        stderr,
        b"\n*** FATAL CONFIG FILE ERROR (Redis %s) ***\n\0" as *const u8
            as *const libc::c_char,
        b"7.0.8\0" as *const u8 as *const libc::c_char,
    );
    if i < totlines {
        fprintf(
            stderr,
            b"Reading the configuration file, at line %d\n\0" as *const u8
                as *const libc::c_char,
            linenum,
        );
        fprintf(
            stderr,
            b">>> '%s'\n\0" as *const u8 as *const libc::c_char,
            *lines.offset(i as isize),
        );
    }
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, err);
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn loadServerConfig(
    mut filename: *mut libc::c_char,
    mut config_from_stdin: libc::c_char,
    mut options: *mut libc::c_char,
) {
    let mut config: sds = sdsempty();
    let mut buf: [libc::c_char; 1025] = [0; 1025];
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut globbuf: glob_t = glob_t {
        gl_pathc: 0,
        gl_pathv: 0 as *mut *mut libc::c_char,
        gl_offs: 0,
        gl_flags: 0,
        gl_closedir: None,
        gl_readdir: None,
        gl_opendir: None,
        gl_lstat: None,
        gl_stat: None,
    };
    if !filename.is_null() {
        if !(strchr(filename, '*' as i32)).is_null()
            || !(strchr(filename, '?' as i32)).is_null()
            || !(strchr(filename, '[' as i32)).is_null()
        {
            if glob(filename, 0 as libc::c_int, None, &mut globbuf) == 0 as libc::c_int {
                let mut i: size_t = 0 as libc::c_int as size_t;
                while i < globbuf.gl_pathc {
                    fp = fopen(
                        *(globbuf.gl_pathv).offset(i as isize),
                        b"r\0" as *const u8 as *const libc::c_char,
                    );
                    if fp.is_null() {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Fatal error, can't open config file '%s': %s\0"
                                    as *const u8 as *const libc::c_char,
                                *(globbuf.gl_pathv).offset(i as isize),
                                strerror(*__errno_location()),
                            );
                        }
                        exit(1 as libc::c_int);
                    }
                    while !(fgets(
                        buf.as_mut_ptr(),
                        1024 as libc::c_int + 1 as libc::c_int,
                        fp,
                    ))
                        .is_null()
                    {
                        config = sdscat(config, buf.as_mut_ptr());
                    }
                    fclose(fp);
                    i = i.wrapping_add(1);
                }
                globfree(&mut globbuf);
            }
        } else {
            fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
            if fp.is_null() {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Fatal error, can't open config file '%s': %s\0" as *const u8
                            as *const libc::c_char,
                        filename,
                        strerror(*__errno_location()),
                    );
                }
                exit(1 as libc::c_int);
            }
            while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int + 1 as libc::c_int, fp))
                .is_null()
            {
                config = sdscat(config, buf.as_mut_ptr());
            }
            fclose(fp);
        }
    }
    if config_from_stdin != 0 {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Reading config from stdin\0" as *const u8 as *const libc::c_char,
            );
        }
        fp = stdin;
        while !(fgets(buf.as_mut_ptr(), 1024 as libc::c_int + 1 as libc::c_int, fp))
            .is_null()
        {
            config = sdscat(config, buf.as_mut_ptr());
        }
    }
    if !options.is_null() {
        config = sdscat(config, b"\n\0" as *const u8 as *const libc::c_char);
        config = sdscat(config, options);
    }
    loadServerConfigFromString(config);
    sdsfree(config);
}
unsafe extern "C" fn performInterfaceSet(
    mut config: *mut standardConfig,
    mut value: sds,
    mut errstr: *mut *const libc::c_char,
) -> libc::c_int {
    let mut argv: *mut sds = 0 as *mut sds;
    let mut argc: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0
    {
        argv = sdssplitlen(
            value as *const libc::c_char,
            sdslen(value) as ssize_t,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            &mut argc,
        );
    } else {
        argv = &mut value as *mut sds as *mut *mut libc::c_char;
        argc = 1 as libc::c_int;
    }
    res = ((*config).interface.set)
        .expect("non-null function pointer")(config, argv, argc, errstr);
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0
    {
        sdsfreesplitres(argv, argc);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn performModuleConfigSetFromName(
    mut name: sds,
    mut value: sds,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut config: *mut standardConfig = lookupConfig(name);
    if config.is_null()
        || (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int == 0
    {
        *err = b"Config name not found\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return performInterfaceSet(config, value, err);
}
#[no_mangle]
pub unsafe extern "C" fn performModuleConfigSetDefaultFromName(
    mut name: sds,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut config: *mut standardConfig = lookupConfig(name);
    if !config.is_null() {} else {
        _serverAssert(
            b"config\0" as *const u8 as *const libc::c_char,
            b"config.c\0" as *const u8 as *const libc::c_char,
            742 as libc::c_int,
        );
        unreachable!();
    };
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int == 0
    {
        *err = b"Config name not found\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    match (*config).type_0 as libc::c_uint {
        0 => {
            return setModuleBoolConfig(
                (*config).privdata as *mut ModuleConfig,
                (*config).data.yesno.default_value,
                err,
            );
        }
        3 => {
            return setModuleStringConfig(
                (*config).privdata as *mut ModuleConfig,
                (*config).data.sds.default_value,
                err,
            );
        }
        1 => {
            return setModuleNumericConfig(
                (*config).privdata as *mut ModuleConfig,
                (*config).data.numeric.default_value,
                err,
            );
        }
        4 => {
            return setModuleEnumConfig(
                (*config).privdata as *mut ModuleConfig,
                (*config).data.enumd.default_value,
                err,
            );
        }
        _ => {
            _serverPanic(
                b"config.c\0" as *const u8 as *const libc::c_char,
                757 as libc::c_int,
                b"Config type of module config is not allowed.\0" as *const u8
                    as *const libc::c_char,
            );
            unreachable!();
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn restoreBackupConfig(
    mut set_configs: *mut *mut standardConfig,
    mut old_values: *mut sds,
    mut count: libc::c_int,
    mut apply_fns: *mut apply_fn,
    mut module_configs: *mut list,
) {
    let mut i: libc::c_int = 0;
    let mut errstr: *const libc::c_char = b"unknown error\0" as *const u8
        as *const libc::c_char;
    i = 0 as libc::c_int;
    while i < count {
        if performInterfaceSet(
            *set_configs.offset(i as isize),
            *old_values.offset(i as isize),
            &mut errstr,
        ) == 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed restoring failed CONFIG SET command. Error setting %s to '%s': %s\0"
                        as *const u8 as *const libc::c_char,
                    (**set_configs.offset(i as isize)).name,
                    *old_values.offset(i as isize),
                    errstr,
                );
            }
        }
        i += 1;
    }
    if !apply_fns.is_null() {
        i = 0 as libc::c_int;
        while i < count && (*apply_fns.offset(i as isize)).is_some() {
            if (*apply_fns.offset(i as isize))
                .expect("non-null function pointer")(&mut errstr) == 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Failed applying restored failed CONFIG SET command: %s\0"
                            as *const u8 as *const libc::c_char,
                        errstr,
                    );
                }
            }
            i += 1;
        }
    }
    if !module_configs.is_null() {
        if moduleConfigApplyConfig(
            module_configs,
            &mut errstr,
            0 as *mut *const libc::c_char,
        ) == 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed applying restored failed CONFIG SET command: %s\0"
                        as *const u8 as *const libc::c_char,
                    errstr,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn configSetCommand(mut c: *mut client) {
    let mut cc: RedisModuleConfigChangeV1 = RedisModuleConfigChangeV1 {
        version: 0,
        num_changes: 0,
        config_names: 0 as *mut *const libc::c_char,
    };
    let mut current_block: u64;
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut invalid_arg_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut err_arg_name: *const libc::c_char = 0 as *const libc::c_char;
    let mut set_configs: *mut *mut standardConfig = 0 as *mut *mut standardConfig;
    let mut module_configs_apply: *mut list = 0 as *mut list;
    let mut config_names: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut new_values: *mut sds = 0 as *mut sds;
    let mut old_values: *mut sds = 0 as *mut sds;
    let mut apply_fns: *mut apply_fn = 0 as *mut apply_fn;
    let mut config_count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut invalid_args: libc::c_int = 0 as libc::c_int;
    let mut deny_loading_error: libc::c_int = 0 as libc::c_int;
    let mut config_map_fns: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*c).argc & 1 as libc::c_int != 0 {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    config_count = ((*c).argc - 2 as libc::c_int) / 2 as libc::c_int;
    module_configs_apply = listCreate();
    set_configs = zcalloc(
        (core::mem::size_of::<*mut standardConfig>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut *mut standardConfig;
    config_names = zcalloc(
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut *const libc::c_char;
    new_values = zmalloc(
        (core::mem::size_of::<*mut sds>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut sds;
    old_values = zcalloc(
        (core::mem::size_of::<*mut sds>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut sds;
    apply_fns = zcalloc(
        (core::mem::size_of::<apply_fn>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut apply_fn;
    config_map_fns = zmalloc(
        (core::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(config_count as libc::c_ulong),
    ) as *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < config_count {
        let mut config: *mut standardConfig = lookupConfig(
            (**((*c).argv).offset((2 as libc::c_int + i * 2 as libc::c_int) as isize))
                .ptr as sds,
        );
        if config.is_null() {
            if invalid_args == 0 {
                invalid_arg_name = (**((*c).argv)
                    .offset((2 as libc::c_int + i * 2 as libc::c_int) as isize))
                    .ptr as *const libc::c_char;
                invalid_args = 1 as libc::c_int;
            }
        } else {
            if (*config).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 1 as libc::c_int != 0
            {
                redactClientCommandArgument(
                    c,
                    2 as libc::c_int + i * 2 as libc::c_int + 1 as libc::c_int,
                );
            }
            if !(invalid_args != 0) {
                if (*config).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
                    || (*config).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 5 as libc::c_int != 0
                        && allowProtectedAction(server.enable_protected_configs, c) == 0
                {
                    errstr = if (*config).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
                    {
                        b"can't set immutable config\0" as *const u8
                            as *const libc::c_char
                    } else {
                        b"can't set protected config\0" as *const u8
                            as *const libc::c_char
                    };
                    err_arg_name = (**((*c).argv)
                        .offset((2 as libc::c_int + i * 2 as libc::c_int) as isize))
                        .ptr as *const libc::c_char;
                    invalid_args = 1 as libc::c_int;
                } else if server.loading != 0
                    && (*config).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 6 as libc::c_int != 0
                {
                    deny_loading_error = 1 as libc::c_int;
                    invalid_args = 1 as libc::c_int;
                } else {
                    j = 0 as libc::c_int;
                    while j < i {
                        if *set_configs.offset(j as isize) == config {
                            errstr = b"duplicate parameter\0" as *const u8
                                as *const libc::c_char;
                            err_arg_name = (**((*c).argv)
                                .offset((2 as libc::c_int + i * 2 as libc::c_int) as isize))
                                .ptr as *const libc::c_char;
                            invalid_args = 1 as libc::c_int;
                            break;
                        } else {
                            j += 1;
                        }
                    }
                    let ref mut fresh2 = *set_configs.offset(i as isize);
                    *fresh2 = config;
                    let ref mut fresh3 = *config_names.offset(i as isize);
                    *fresh3 = (*config).name;
                    let ref mut fresh4 = *new_values.offset(i as isize);
                    *fresh4 = (**((*c).argv)
                        .offset(
                            (2 as libc::c_int + i * 2 as libc::c_int + 1 as libc::c_int)
                                as isize,
                        ))
                        .ptr as sds;
                }
            }
        }
        i += 1;
    }
    if invalid_args != 0 {
        current_block = 8801699790749648692;
    } else {
        i = 0 as libc::c_int;
        while i < config_count {
            let ref mut fresh5 = *old_values.offset(i as isize);
            *fresh5 = ((**set_configs.offset(i as isize)).interface.get)
                .expect("non-null function pointer")(*set_configs.offset(i as isize));
            i += 1;
        }
        i = 0 as libc::c_int;
        loop {
            if !(i < config_count) {
                current_block = 8835654301469918283;
                break;
            }
            let mut res: libc::c_int = performInterfaceSet(
                *set_configs.offset(i as isize),
                *new_values.offset(i as isize),
                &mut errstr,
            );
            if res == 0 {
                restoreBackupConfig(
                    set_configs,
                    old_values,
                    i + 1 as libc::c_int,
                    0 as *mut apply_fn,
                    0 as *mut list,
                );
                err_arg_name = (**set_configs.offset(i as isize)).name;
                current_block = 8801699790749648692;
                break;
            } else {
                if res == 1 as libc::c_int {
                    if (**set_configs.offset(i as isize)).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
                    {
                        addModuleConfigApply(
                            module_configs_apply,
                            (**set_configs.offset(i as isize)).privdata
                                as *mut ModuleConfig,
                        );
                    } else if ((**set_configs.offset(i as isize)).interface.apply)
                        .is_some()
                    {
                        let mut exists: libc::c_int = 0 as libc::c_int;
                        j = 0 as libc::c_int;
                        while (*apply_fns.offset(j as isize)).is_some() && j <= i {
                            if *apply_fns.offset(j as isize)
                                == (**set_configs.offset(i as isize)).interface.apply
                            {
                                exists = 1 as libc::c_int;
                                break;
                            } else {
                                j += 1;
                            }
                        }
                        if exists == 0 {
                            let ref mut fresh6 = *apply_fns.offset(j as isize);
                            *fresh6 = (**set_configs.offset(i as isize)).interface.apply;
                            *config_map_fns.offset(j as isize) = i;
                        }
                    }
                }
                i += 1;
            }
        }
        match current_block {
            8801699790749648692 => {}
            _ => {
                i = 0 as libc::c_int;
                loop {
                    if !(i < config_count && (*apply_fns.offset(i as isize)).is_some()) {
                        current_block = 1868291631715963762;
                        break;
                    }
                    if (*apply_fns.offset(i as isize))
                        .expect("non-null function pointer")(&mut errstr) == 0
                    {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Failed applying new configuration. Possibly related to new %s setting. Restoring previous settings.\0"
                                    as *const u8 as *const libc::c_char,
                                (**set_configs
                                    .offset(*config_map_fns.offset(i as isize) as isize))
                                    .name,
                            );
                        }
                        restoreBackupConfig(
                            set_configs,
                            old_values,
                            config_count,
                            apply_fns,
                            0 as *mut list,
                        );
                        err_arg_name = (**set_configs
                            .offset(*config_map_fns.offset(i as isize) as isize))
                            .name;
                        current_block = 8801699790749648692;
                        break;
                    } else {
                        i += 1;
                    }
                }
                match current_block {
                    8801699790749648692 => {}
                    _ => {
                        if moduleConfigApplyConfig(
                            module_configs_apply,
                            &mut errstr,
                            &mut err_arg_name,
                        ) == 0
                        {
                            serverLogRaw(
                                3 as libc::c_int,
                                b"Failed applying new module configuration. Restoring previous settings.\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            restoreBackupConfig(
                                set_configs,
                                old_values,
                                config_count,
                                apply_fns,
                                module_configs_apply,
                            );
                            current_block = 8801699790749648692;
                        } else {
                            cc = {
                                let mut init = RedisModuleConfigChange {
                                    version: 0,
                                    num_changes: config_count as uint32_t,
                                    config_names: config_names,
                                };
                                init
                            };
                            moduleFireServerEvent(
                                16 as libc::c_int as uint64_t,
                                0 as libc::c_int,
                                &mut cc as *mut RedisModuleConfigChangeV1
                                    as *mut libc::c_void,
                            );
                            addReply(c, shared.ok);
                            current_block = 10095473323661844693;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        8801699790749648692 => {
            if deny_loading_error != 0 {
                addReplyErrorObject(c, shared.loadingerr);
            } else if !invalid_arg_name.is_null() {
                addReplyErrorFormat(
                    c,
                    b"Unknown option or number of arguments for CONFIG SET - '%s'\0"
                        as *const u8 as *const libc::c_char,
                    invalid_arg_name,
                );
            } else if !errstr.is_null() {
                addReplyErrorFormat(
                    c,
                    b"CONFIG SET failed (possibly related to argument '%s') - %s\0"
                        as *const u8 as *const libc::c_char,
                    err_arg_name,
                    errstr,
                );
            } else {
                addReplyErrorFormat(
                    c,
                    b"CONFIG SET failed (possibly related to argument '%s')\0"
                        as *const u8 as *const libc::c_char,
                    err_arg_name,
                );
            }
        }
        _ => {}
    }
    zfree(set_configs as *mut libc::c_void);
    zfree(config_names as *mut libc::c_void);
    zfree(new_values as *mut libc::c_void);
    i = 0 as libc::c_int;
    while i < config_count {
        sdsfree(*old_values.offset(i as isize));
        i += 1;
    }
    zfree(old_values as *mut libc::c_void);
    zfree(apply_fns as *mut libc::c_void);
    zfree(config_map_fns as *mut libc::c_void);
    listRelease(module_configs_apply);
}
#[no_mangle]
pub unsafe extern "C" fn configGetCommand(mut c: *mut client) {
    let mut i: libc::c_int = 0;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut matches: *mut dict = dictCreate(&mut externalStringType);
    i = 0 as libc::c_int;
    while i < (*c).argc - 2 as libc::c_int {
        let mut o: *mut robj = *((*c).argv).offset((2 as libc::c_int + i) as isize);
        let mut name: sds = (*o).ptr as sds;
        if (strpbrk(
            name as *const libc::c_char,
            b"[*?\0" as *const u8 as *const libc::c_char,
        ))
            .is_null()
        {
            if (dictFind(matches, name as *const libc::c_void)).is_null() {
                let mut config: *mut standardConfig = lookupConfig(name);
                if !config.is_null() {
                    dictAdd(
                        matches,
                        name as *mut libc::c_void,
                        config as *mut libc::c_void,
                    );
                }
            }
        } else {
            di = dictGetIterator(configs);
            loop {
                de = dictNext(di);
                if de.is_null() {
                    break;
                }
                let mut config_0: *mut standardConfig = (*de).v.val
                    as *mut standardConfig;
                if (*config_0).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 4 as libc::c_int != 0
                {
                    continue;
                }
                if !(dictFind(matches, (*config_0).name as *const libc::c_void))
                    .is_null()
                {
                    continue;
                }
                if stringmatch(
                    name as *const libc::c_char,
                    (*de).key as *const libc::c_char,
                    1 as libc::c_int,
                ) != 0
                {
                    dictAdd(matches, (*de).key, config_0 as *mut libc::c_void);
                }
            }
            dictReleaseIterator(di);
        }
        i += 1;
    }
    di = dictGetIterator(matches);
    addReplyMapLen(
        c,
        ((*matches).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*matches).ht_used[1 as libc::c_int as usize]) as libc::c_long,
    );
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut config_1: *mut standardConfig = (*de).v.val as *mut standardConfig;
        addReplyBulkCString(c, (*de).key as *const libc::c_char);
        addReplyBulkSds(
            c,
            ((*config_1).interface.get).expect("non-null function pointer")(config_1),
        );
    }
    dictReleaseIterator(di);
    dictRelease(matches);
}
#[no_mangle]
pub static mut optionToLineDictType: dictType = unsafe {
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
                dictListDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut optionSetDictType: dictType = unsafe {
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigReleaseState(mut state: *mut rewriteConfigState) {
    sdsfreesplitres((*state).lines, (*state).numlines);
    dictRelease((*state).option_to_line);
    dictRelease((*state).rewritten);
    zfree(state as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigCreateState() -> *mut rewriteConfigState {
    let mut state: *mut rewriteConfigState = zmalloc(
        core::mem::size_of::<rewriteConfigState>() as libc::c_ulong,
    ) as *mut rewriteConfigState;
    (*state).option_to_line = dictCreate(&mut optionToLineDictType);
    (*state).rewritten = dictCreate(&mut optionSetDictType);
    (*state).numlines = 0 as libc::c_int;
    (*state).lines = 0 as *mut sds;
    (*state).needs_signature = 1 as libc::c_int;
    (*state).force_write = 0 as libc::c_int;
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigAppendLine(
    mut state: *mut rewriteConfigState,
    mut line: sds,
) {
    (*state)
        .lines = zrealloc(
        (*state).lines as *mut libc::c_void,
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(((*state).numlines + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut sds;
    let fresh7 = (*state).numlines;
    (*state).numlines = (*state).numlines + 1;
    let ref mut fresh8 = *((*state).lines).offset(fresh7 as isize);
    *fresh8 = line;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigAddLineNumberToOption(
    mut state: *mut rewriteConfigState,
    mut option: sds,
    mut linenum: libc::c_int,
) {
    let mut l: *mut list = dictFetchValue(
        (*state).option_to_line,
        option as *const libc::c_void,
    ) as *mut list;
    if l.is_null() {
        l = listCreate();
        dictAdd(
            (*state).option_to_line,
            sdsdup(option) as *mut libc::c_void,
            l as *mut libc::c_void,
        );
    }
    listAddNodeTail(l, linenum as libc::c_long as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigMarkAsProcessed(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
) {
    let mut opt: sds = sdsnew(option);
    if dictAdd((*state).rewritten, opt as *mut libc::c_void, 0 as *mut libc::c_void)
        != 0 as libc::c_int
    {
        sdsfree(opt);
    }
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigReadOldFile(
    mut path: *mut libc::c_char,
) -> *mut rewriteConfigState {
    let mut fp: *mut FILE = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() && *__errno_location() != 2 as libc::c_int {
        return 0 as *mut rewriteConfigState;
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
    if !fp.is_null() && fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        return 0 as *mut rewriteConfigState;
    }
    let mut linenum: libc::c_int = -(1 as libc::c_int);
    let mut state: *mut rewriteConfigState = rewriteConfigCreateState();
    if fp.is_null() || sb.st_size == 0 as libc::c_int as libc::c_long {
        return state;
    }
    let mut config: sds = sdsnewlen(
        SDS_NOINIT as *const libc::c_void,
        sb.st_size as size_t,
    );
    if fread(
        config as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        sb.st_size as libc::c_ulong,
        fp,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        sdsfree(config);
        rewriteConfigReleaseState(state);
        fclose(fp);
        return 0 as *mut rewriteConfigState;
    }
    let mut i: libc::c_int = 0;
    let mut totlines: libc::c_int = 0;
    let mut lines: *mut sds = sdssplitlen(
        config as *const libc::c_char,
        sdslen(config) as ssize_t,
        b"\n\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut totlines,
    );
    i = 0 as libc::c_int;
    while i < totlines {
        let mut argc: libc::c_int = 0;
        let mut argv: *mut sds = 0 as *mut sds;
        let mut line: sds = sdstrim(
            *lines.offset(i as isize),
            b"\r\n\t \0" as *const u8 as *const libc::c_char,
        );
        let ref mut fresh9 = *lines.offset(i as isize);
        *fresh9 = 0 as sds;
        linenum += 1;
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32
            || *line.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            if (*state).needs_signature != 0
                && strcmp(
                    line as *const libc::c_char,
                    b"# Generated by CONFIG REWRITE\0" as *const u8
                        as *const libc::c_char,
                ) == 0
            {
                (*state).needs_signature = 0 as libc::c_int;
            }
            rewriteConfigAppendLine(state, line);
        } else {
            argv = sdssplitargs(line as *const libc::c_char, &mut argc);
            if argv.is_null()
                || (lookupConfig(*argv.offset(0 as libc::c_int as isize))).is_null()
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"include\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"rename-command\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"user\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"loadmodule\0" as *const u8 as *const libc::c_char,
                    ) != 0
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"sentinel\0" as *const u8 as *const libc::c_char,
                    ) != 0
            {
                let mut aux: sds = sdsnew(
                    b"# ??? \0" as *const u8 as *const libc::c_char,
                );
                aux = sdscatsds(aux, line);
                if !argv.is_null() {
                    sdsfreesplitres(argv, argc);
                }
                sdsfree(line);
                rewriteConfigAppendLine(state, aux);
            } else {
                sdstolower(*argv.offset(0 as libc::c_int as isize));
                rewriteConfigAppendLine(state, line);
                let mut s_conf: *mut standardConfig = lookupConfig(
                    *argv.offset(0 as libc::c_int as isize),
                );
                if !s_conf.is_null()
                    && (*s_conf).flags as libc::c_ulonglong
                        & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
                {
                    sdsfree(*argv.offset(0 as libc::c_int as isize));
                    let ref mut fresh10 = *argv.offset(0 as libc::c_int as isize);
                    *fresh10 = sdsnew((*s_conf).alias);
                }
                if server.sentinel_mode != 0 && argc > 1 as libc::c_int
                    && strcasecmp(
                        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                        b"sentinel\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    let mut sentinelOption: sds = sdsempty();
                    sentinelOption = sdscatfmt(
                        sentinelOption,
                        b"%S %S\0" as *const u8 as *const libc::c_char,
                        *argv.offset(0 as libc::c_int as isize),
                        *argv.offset(1 as libc::c_int as isize),
                    );
                    rewriteConfigAddLineNumberToOption(state, sentinelOption, linenum);
                    sdsfree(sentinelOption);
                } else {
                    rewriteConfigAddLineNumberToOption(
                        state,
                        *argv.offset(0 as libc::c_int as isize),
                        linenum,
                    );
                }
                sdsfreesplitres(argv, argc);
            }
        }
        i += 1;
    }
    fclose(fp);
    sdsfreesplitres(lines, totlines);
    sdsfree(config);
    return state;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigRewriteLine(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut line: sds,
    mut force: libc::c_int,
) {
    let mut o: sds = sdsnew(option);
    let mut l: *mut list = dictFetchValue(
        (*state).option_to_line,
        o as *const libc::c_void,
    ) as *mut list;
    rewriteConfigMarkAsProcessed(state, option);
    if l.is_null() && force == 0 && (*state).force_write == 0 {
        sdsfree(line);
        sdsfree(o);
        return;
    }
    if !l.is_null() {
        let mut ln: *mut listNode = (*l).head;
        let mut linenum: libc::c_int = (*ln).value as libc::c_long as libc::c_int;
        listDelNode(l, ln);
        if (*l).len == 0 as libc::c_int as libc::c_ulong {
            dictDelete((*state).option_to_line, o as *const libc::c_void);
        }
        sdsfree(*((*state).lines).offset(linenum as isize));
        let ref mut fresh11 = *((*state).lines).offset(linenum as isize);
        *fresh11 = line;
    } else {
        if (*state).needs_signature != 0 {
            rewriteConfigAppendLine(
                state,
                sdsnew(
                    b"# Generated by CONFIG REWRITE\0" as *const u8
                        as *const libc::c_char,
                ),
            );
            (*state).needs_signature = 0 as libc::c_int;
        }
        rewriteConfigAppendLine(state, line);
    }
    sdsfree(o);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigFormatMemory(
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut bytes: libc::c_longlong,
) -> libc::c_int {
    let mut gb: libc::c_int = 1024 as libc::c_int * 1024 as libc::c_int
        * 1024 as libc::c_int;
    let mut mb: libc::c_int = 1024 as libc::c_int * 1024 as libc::c_int;
    let mut kb: libc::c_int = 1024 as libc::c_int;
    if bytes != 0
        && bytes % gb as libc::c_longlong == 0 as libc::c_int as libc::c_longlong
    {
        return snprintf(
            buf,
            len,
            b"%lldgb\0" as *const u8 as *const libc::c_char,
            bytes / gb as libc::c_longlong,
        )
    } else if bytes != 0
        && bytes % mb as libc::c_longlong == 0 as libc::c_int as libc::c_longlong
    {
        return snprintf(
            buf,
            len,
            b"%lldmb\0" as *const u8 as *const libc::c_char,
            bytes / mb as libc::c_longlong,
        )
    } else if bytes != 0
        && bytes % kb as libc::c_longlong == 0 as libc::c_int as libc::c_longlong
    {
        return snprintf(
            buf,
            len,
            b"%lldkb\0" as *const u8 as *const libc::c_char,
            bytes / kb as libc::c_longlong,
        )
    } else {
        return snprintf(buf, len, b"%lld\0" as *const u8 as *const libc::c_char, bytes)
    };
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigBytesOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_longlong,
    mut defvalue: libc::c_longlong,
) {
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut force: libc::c_int = (value != defvalue) as libc::c_int;
    let mut line: sds = 0 as *mut libc::c_char;
    rewriteConfigFormatMemory(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        value,
    );
    line = sdscatprintf(
        sdsempty(),
        b"%s %s\0" as *const u8 as *const libc::c_char,
        option,
        buf.as_mut_ptr(),
    );
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigPercentOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_longlong,
    mut defvalue: libc::c_longlong,
) {
    let mut force: libc::c_int = (value != defvalue) as libc::c_int;
    let mut line: sds = sdscatprintf(
        sdsempty(),
        b"%s %lld%%\0" as *const u8 as *const libc::c_char,
        option,
        value,
    );
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigYesNoOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_int,
    mut defvalue: libc::c_int,
) {
    let mut force: libc::c_int = (value != defvalue) as libc::c_int;
    let mut line: sds = sdscatprintf(
        sdsempty(),
        b"%s %s\0" as *const u8 as *const libc::c_char,
        option,
        if value != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigStringOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: *mut libc::c_char,
    mut defvalue: *const libc::c_char,
) {
    let mut force: libc::c_int = 1 as libc::c_int;
    let mut line: sds = 0 as *mut libc::c_char;
    if value.is_null() {
        rewriteConfigMarkAsProcessed(state, option);
        return;
    }
    if !defvalue.is_null() && strcmp(value, defvalue) == 0 as libc::c_int {
        force = 0 as libc::c_int;
    }
    line = sdsnew(option);
    line = sdscatlen(
        line,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    line = sdscatrepr(line, value, strlen(value));
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigSdsOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: sds,
    mut defvalue: *const libc::c_char,
) {
    let mut force: libc::c_int = 1 as libc::c_int;
    let mut line: sds = 0 as *mut libc::c_char;
    if value.is_null() {
        rewriteConfigMarkAsProcessed(state, option);
        return;
    }
    if !defvalue.is_null()
        && strcmp(value as *const libc::c_char, defvalue) == 0 as libc::c_int
    {
        force = 0 as libc::c_int;
    }
    line = sdsnew(option);
    line = sdscatlen(
        line,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    line = sdscatrepr(line, value as *const libc::c_char, sdslen(value));
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigNumericalOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_longlong,
    mut defvalue: libc::c_longlong,
) {
    let mut force: libc::c_int = (value != defvalue) as libc::c_int;
    let mut line: sds = sdscatprintf(
        sdsempty(),
        b"%s %lld\0" as *const u8 as *const libc::c_char,
        option,
        value,
    );
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigOctalOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_longlong,
    mut defvalue: libc::c_longlong,
) {
    let mut force: libc::c_int = (value != defvalue) as libc::c_int;
    let mut line: sds = sdscatprintf(
        sdsempty(),
        b"%s %llo\0" as *const u8 as *const libc::c_char,
        option,
        value,
    );
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigEnumOption(
    mut state: *mut rewriteConfigState,
    mut option: *const libc::c_char,
    mut value: libc::c_int,
    mut config: *mut standardConfig,
) {
    let mut multiarg: libc::c_int = ((*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_int;
    let mut names: sds = configEnumGetName(
        (*config).data.enumd.enum_value,
        value,
        multiarg,
    );
    let mut line: sds = sdscatfmt(
        sdsempty(),
        b"%s %s\0" as *const u8 as *const libc::c_char,
        option,
        names,
    );
    sdsfree(names);
    let mut force: libc::c_int = (value != (*config).data.enumd.default_value)
        as libc::c_int;
    rewriteConfigRewriteLine(state, option, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigSaveOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut j: libc::c_int = 0;
    let mut line: sds = 0 as *mut libc::c_char;
    if server.sentinel_mode != 0 {
        rewriteConfigMarkAsProcessed(state, name);
        return;
    }
    if server.saveparamslen == 0 {
        rewriteConfigRewriteLine(
            state,
            name,
            sdsnew(b"save \"\"\0" as *const u8 as *const libc::c_char),
            1 as libc::c_int,
        );
    } else {
        j = 0 as libc::c_int;
        while j < server.saveparamslen {
            line = sdscatprintf(
                sdsempty(),
                b"save %ld %d\0" as *const u8 as *const libc::c_char,
                (*(server.saveparams).offset(j as isize)).seconds,
                (*(server.saveparams).offset(j as isize)).changes,
            );
            rewriteConfigRewriteLine(state, name, line, 1 as libc::c_int);
            j += 1;
        }
    }
    rewriteConfigMarkAsProcessed(state, name);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigUserOption(mut state: *mut rewriteConfigState) {
    if *(server.acl_filename).offset(0 as libc::c_int as isize) as libc::c_int
        != '\0' as i32
    {
        rewriteConfigMarkAsProcessed(
            state,
            b"user\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
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
    raxStart(&mut ri, Users);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut u: *mut user = ri.data as *mut user;
        let mut line: sds = sdsnew(b"user \0" as *const u8 as *const libc::c_char);
        line = sdscatsds(line, (*u).name);
        line = sdscatlen(
            line,
            b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        let mut descr: *mut robj = ACLDescribeUser(u);
        line = sdscatsds(line, (*descr).ptr as sds);
        decrRefCount(descr);
        rewriteConfigRewriteLine(
            state,
            b"user\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    }
    raxStop(&mut ri);
    rewriteConfigMarkAsProcessed(state, b"user\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigDirOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut cwd: [libc::c_char; 1024] = [0; 1024];
    if (getcwd(
        cwd.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    ))
        .is_null()
    {
        rewriteConfigMarkAsProcessed(state, name);
        return;
    }
    rewriteConfigStringOption(state, name, cwd.as_mut_ptr(), 0 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigReplicaOfOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut line: sds = 0 as *mut libc::c_char;
    if server.cluster_enabled != 0 || (server.masterhost).is_null() {
        rewriteConfigMarkAsProcessed(state, name);
        return;
    }
    line = sdscatprintf(
        sdsempty(),
        b"%s %s %d\0" as *const u8 as *const libc::c_char,
        name,
        server.masterhost,
        server.masterport,
    );
    rewriteConfigRewriteLine(state, name, line, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigNotifyKeyspaceEventsOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut force: libc::c_int = (server.notify_keyspace_events != 0 as libc::c_int)
        as libc::c_int;
    let mut line: sds = 0 as *mut libc::c_char;
    let mut flags: sds = 0 as *mut libc::c_char;
    flags = keyspaceEventsFlagsToString(server.notify_keyspace_events);
    line = sdsnew(name);
    line = sdscatlen(
        line,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    line = sdscatrepr(line, flags as *const libc::c_char, sdslen(flags));
    sdsfree(flags);
    rewriteConfigRewriteLine(state, name, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigClientOutputBufferLimitOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        let mut force: libc::c_int = (server
            .client_obuf_limits[j as usize]
            .hard_limit_bytes != clientBufferLimitsDefaults[j as usize].hard_limit_bytes
            || server.client_obuf_limits[j as usize].soft_limit_bytes
                != clientBufferLimitsDefaults[j as usize].soft_limit_bytes
            || server.client_obuf_limits[j as usize].soft_limit_seconds
                != clientBufferLimitsDefaults[j as usize].soft_limit_seconds)
            as libc::c_int;
        let mut line: sds = 0 as *mut libc::c_char;
        let mut hard: [libc::c_char; 64] = [0; 64];
        let mut soft: [libc::c_char; 64] = [0; 64];
        rewriteConfigFormatMemory(
            hard.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            server.client_obuf_limits[j as usize].hard_limit_bytes as libc::c_longlong,
        );
        rewriteConfigFormatMemory(
            soft.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            server.client_obuf_limits[j as usize].soft_limit_bytes as libc::c_longlong,
        );
        let mut typename: *mut libc::c_char = getClientTypeName(j);
        if strcmp(typename, b"slave\0" as *const u8 as *const libc::c_char) == 0 {
            typename = b"replica\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        line = sdscatprintf(
            sdsempty(),
            b"%s %s %s %s %ld\0" as *const u8 as *const libc::c_char,
            name,
            typename,
            hard.as_mut_ptr(),
            soft.as_mut_ptr(),
            server.client_obuf_limits[j as usize].soft_limit_seconds,
        );
        rewriteConfigRewriteLine(state, name, line, force);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigOOMScoreAdjValuesOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut force: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut line: sds = 0 as *mut libc::c_char;
    line = sdsnew(name);
    line = sdscatlen(
        line,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        if server.oom_score_adj_values[j as usize]
            != configOOMScoreAdjValuesDefaults[j as usize]
        {
            force = 1 as libc::c_int;
        }
        line = sdscatprintf(
            line,
            b"%d\0" as *const u8 as *const libc::c_char,
            server.oom_score_adj_values[j as usize],
        );
        if j + 1 as libc::c_int != 3 as libc::c_int {
            line = sdscatlen(
                line,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    rewriteConfigRewriteLine(state, name, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigBindOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut force: libc::c_int = 1 as libc::c_int;
    let mut line: sds = 0 as *mut libc::c_char;
    let mut addresses: sds = 0 as *mut libc::c_char;
    let mut is_default: libc::c_int = 0 as libc::c_int;
    if server.bindaddr_count == 2 as libc::c_int {
        is_default = 1 as libc::c_int;
        let mut default_bindaddr: [*mut libc::c_char; 2] = [
            b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"-::*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ];
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < 2 as libc::c_int {
            if strcmp(server.bindaddr[j as usize], default_bindaddr[j as usize])
                != 0 as libc::c_int
            {
                is_default = 0 as libc::c_int;
                break;
            } else {
                j += 1;
            }
        }
    }
    if is_default != 0 {
        rewriteConfigMarkAsProcessed(state, name);
        return;
    }
    if server.bindaddr_count > 0 as libc::c_int {
        addresses = sdsjoin(
            (server.bindaddr).as_mut_ptr(),
            server.bindaddr_count,
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        addresses = sdsnew(b"\"\"\0" as *const u8 as *const libc::c_char);
    }
    line = sdsnew(name);
    line = sdscatlen(
        line,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    line = sdscatsds(line, addresses);
    sdsfree(addresses);
    rewriteConfigRewriteLine(state, name, line, force);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigLoadmoduleOption(
    mut state: *mut rewriteConfigState,
) {
    let mut line: sds = 0 as *mut libc::c_char;
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        line = sdsnew(b"loadmodule \0" as *const u8 as *const libc::c_char);
        line = sdscatsds(line, (*(*module).loadmod).path);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*(*module).loadmod).argc {
            line = sdscatlen(
                line,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            line = sdscatsds(
                line,
                (**((*(*module).loadmod).argv).offset(i as isize)).ptr as sds,
            );
            i += 1;
        }
        rewriteConfigRewriteLine(
            state,
            b"loadmodule\0" as *const u8 as *const libc::c_char,
            line,
            1 as libc::c_int,
        );
    }
    dictReleaseIterator(di);
    rewriteConfigMarkAsProcessed(
        state,
        b"loadmodule\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigGetContentFromState(
    mut state: *mut rewriteConfigState,
) -> sds {
    let mut content: sds = sdsempty();
    let mut j: libc::c_int = 0;
    let mut was_empty: libc::c_int = 0 as libc::c_int;
    let mut current_block_5: u64;
    j = 0 as libc::c_int;
    while j < (*state).numlines {
        if sdslen(*((*state).lines).offset(j as isize))
            == 0 as libc::c_int as libc::c_ulong
        {
            if was_empty != 0 {
                current_block_5 = 16668937799742929182;
            } else {
                was_empty = 1 as libc::c_int;
                current_block_5 = 10879442775620481940;
            }
        } else {
            was_empty = 0 as libc::c_int;
            current_block_5 = 10879442775620481940;
        }
        match current_block_5 {
            10879442775620481940 => {
                content = sdscatsds(content, *((*state).lines).offset(j as isize));
                content = sdscatlen(
                    content,
                    b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    1 as libc::c_int as size_t,
                );
            }
            _ => {}
        }
        j += 1;
    }
    return content;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigRemoveOrphaned(
    mut state: *mut rewriteConfigState,
) {
    let mut di: *mut dictIterator = dictGetIterator((*state).option_to_line);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut l: *mut list = (*de).v.val as *mut list;
        let mut option: sds = (*de).key as sds;
        if (dictFind((*state).rewritten, option as *const libc::c_void)).is_null() {
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"Not rewritten option: %s\0" as *const u8 as *const libc::c_char,
                    option,
                );
            }
        } else {
            while (*l).len != 0 {
                let mut ln: *mut listNode = (*l).head;
                let mut linenum: libc::c_int = (*ln).value as libc::c_long
                    as libc::c_int;
                sdsfree(*((*state).lines).offset(linenum as isize));
                let ref mut fresh12 = *((*state).lines).offset(linenum as isize);
                *fresh12 = sdsempty();
                listDelNode(l, ln);
            }
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn getConfigDebugInfo() -> sds {
    let mut state: *mut rewriteConfigState = rewriteConfigCreateState();
    (*state).force_write = 1 as libc::c_int;
    (*state).needs_signature = 0 as libc::c_int;
    let mut di: *mut dictIterator = dictGetIterator(configs);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut config: *mut standardConfig = (*de).v.val as *mut standardConfig;
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 2 as libc::c_int == 0
        {
            continue;
        }
        ((*config).interface.rewrite)
            .expect("non-null function pointer")(config, (*config).name, state);
    }
    dictReleaseIterator(di);
    let mut info: sds = rewriteConfigGetContentFromState(state);
    rewriteConfigReleaseState(state);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigOverwriteFile(
    mut configfile: *mut libc::c_char,
    mut content: sds,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fd: libc::c_int = -(1 as libc::c_int);
    let mut retval: libc::c_int = -(1 as libc::c_int);
    let mut tmp_conffile: [libc::c_char; 4096] = [0; 4096];
    let mut tmp_suffix: *const libc::c_char = b".XXXXXX\0" as *const u8
        as *const libc::c_char;
    let mut offset: size_t = 0 as libc::c_int as size_t;
    let mut written_bytes: ssize_t = 0 as libc::c_int as ssize_t;
    let mut old_errno: libc::c_int = 0;
    let mut tmp_path_len: libc::c_int = snprintf(
        tmp_conffile.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
        b"%s%s\0" as *const u8 as *const libc::c_char,
        configfile,
        tmp_suffix,
    );
    if tmp_path_len <= 0 as libc::c_int
        || tmp_path_len as libc::c_uint as libc::c_ulong
            >= core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Config file full path is too long\0" as *const u8
                    as *const libc::c_char,
            );
        }
        *__errno_location() = 36 as libc::c_int;
        return retval;
    }
    fd = mkostemp(tmp_conffile.as_mut_ptr(), 0o2000000 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Could not create tmp config file (%s)\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return retval;
    }
    loop {
        if !(offset < sdslen(content)) {
            current_block = 11298138898191919651;
            break;
        }
        written_bytes = write(
            fd,
            content.offset(offset as isize) as *const libc::c_void,
            (sdslen(content)).wrapping_sub(offset),
        );
        if written_bytes <= 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed after writing (%zd) bytes to tmp config file (%s)\0"
                        as *const u8 as *const libc::c_char,
                    offset,
                    strerror(*__errno_location()),
                );
            }
            current_block = 9732621921177299880;
            break;
        } else {
            offset = (offset as libc::c_ulong)
                .wrapping_add(written_bytes as libc::c_ulong) as size_t as size_t;
        }
    }
    match current_block {
        11298138898191919651 => {
            if fsync(fd) != 0 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Could not sync tmp config file to disk (%s)\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            } else if fchmod(fd, 0o644 as libc::c_int as libc::c_uint & !server.umask)
                == -(1 as libc::c_int)
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Could not chmod config file (%s)\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            } else if rename(tmp_conffile.as_mut_ptr(), configfile)
                == -(1 as libc::c_int)
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Could not rename tmp config file (%s)\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            } else if fsyncFileDir(configfile) == -(1 as libc::c_int) {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Could not sync config file dir (%s)\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
            } else {
                retval = 0 as libc::c_int;
                if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        0 as libc::c_int,
                        b"Rewritten config file (%s) successfully\0" as *const u8
                            as *const libc::c_char,
                        configfile,
                    );
                }
            }
        }
        _ => {}
    }
    old_errno = *__errno_location();
    close(fd);
    if retval != 0 {
        unlink(tmp_conffile.as_mut_ptr());
    }
    *__errno_location() = old_errno;
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfig(
    mut path: *mut libc::c_char,
    mut force_write: libc::c_int,
) -> libc::c_int {
    let mut state: *mut rewriteConfigState = 0 as *mut rewriteConfigState;
    let mut newcontent: sds = 0 as *mut libc::c_char;
    let mut retval: libc::c_int = 0;
    state = rewriteConfigReadOldFile(path);
    if state.is_null() {
        return -(1 as libc::c_int);
    }
    if force_write != 0 {
        (*state).force_write = 1 as libc::c_int;
    }
    let mut di: *mut dictIterator = dictGetIterator(configs);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut config: *mut standardConfig = (*de).v.val as *mut standardConfig;
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
        {
            continue;
        }
        if ((*config).interface.rewrite).is_some() {
            ((*config).interface.rewrite)
                .expect(
                    "non-null function pointer",
                )(config, (*de).key as *const libc::c_char, state);
        }
    }
    dictReleaseIterator(di);
    rewriteConfigUserOption(state);
    rewriteConfigLoadmoduleOption(state);
    if server.sentinel_mode != 0 {
        rewriteConfigSentinelOption(state);
    }
    rewriteConfigRemoveOrphaned(state);
    newcontent = rewriteConfigGetContentFromState(state);
    retval = rewriteConfigOverwriteFile(server.configfile, newcontent);
    sdsfree(newcontent);
    rewriteConfigReleaseState(state);
    return retval;
}
static mut loadbuf: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn boolConfigInit(mut config: *mut standardConfig) {
    *(*config).data.yesno.config = (*config).data.yesno.default_value;
}
unsafe extern "C" fn boolConfigSet(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut yn: libc::c_int = yesnotoi(*argv.offset(0 as libc::c_int as isize));
    if yn == -(1 as libc::c_int) {
        *err = b"argument must be 'yes' or 'no'\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    if ((*config).data.yesno.is_valid_fn).is_some()
        && ((*config).data.yesno.is_valid_fn)
            .expect("non-null function pointer")(yn, err) == 0
    {
        return 0 as libc::c_int;
    }
    let mut prev: libc::c_int = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleBoolConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.yesno.config
    };
    if prev != yn {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            return setModuleBoolConfig((*config).privdata as *mut ModuleConfig, yn, err);
        }
        *(*config).data.yesno.config = yn;
        return 1 as libc::c_int;
    }
    return if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn boolConfigGet(mut config: *mut standardConfig) -> sds {
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        return sdsnew(
            if getModuleBoolConfig((*config).privdata as *mut ModuleConfig) != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    return sdsnew(
        if *(*config).data.yesno.config != 0 {
            b"yes\0" as *const u8 as *const libc::c_char
        } else {
            b"no\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn boolConfigRewrite(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut val: libc::c_int = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleBoolConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.yesno.config
    };
    rewriteConfigYesNoOption(state, name, val, (*config).data.yesno.default_value);
}
unsafe extern "C" fn stringConfigInit(mut config: *mut standardConfig) {
    *(*config)
        .data
        .string
        .config = if (*config).data.string.convert_empty_to_null != 0
        && ((*config).data.string.default_value).is_null()
    {
        0 as *mut libc::c_char
    } else {
        zstrdup((*config).data.string.default_value)
    };
}
unsafe extern "C" fn stringConfigSet(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if ((*config).data.string.is_valid_fn).is_some()
        && ((*config).data.string.is_valid_fn)
            .expect(
                "non-null function pointer",
            )(*argv.offset(0 as libc::c_int as isize), err) == 0
    {
        return 0 as libc::c_int;
    }
    let mut prev: *mut libc::c_char = *(*config).data.string.config;
    let mut new: *mut libc::c_char = if (*config).data.string.convert_empty_to_null != 0
        && *(*argv.offset(0 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            == 0
    {
        0 as sds
    } else {
        *argv.offset(0 as libc::c_int as isize)
    };
    if new != prev && (new.is_null() || prev.is_null() || strcmp(prev, new) != 0) {
        *(*config)
            .data
            .string
            .config = if !new.is_null() { zstrdup(new) } else { 0 as *mut libc::c_char };
        zfree(prev as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    return if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn stringConfigGet(mut config: *mut standardConfig) -> sds {
    return sdsnew(
        if !(*(*config).data.string.config).is_null() {
            *(*config).data.string.config as *const libc::c_char
        } else {
            b"\0" as *const u8 as *const libc::c_char
        },
    );
}
unsafe extern "C" fn stringConfigRewrite(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    rewriteConfigStringOption(
        state,
        name,
        *(*config).data.string.config,
        (*config).data.string.default_value,
    );
}
unsafe extern "C" fn sdsConfigInit(mut config: *mut standardConfig) {
    *(*config)
        .data
        .sds
        .config = if (*config).data.sds.convert_empty_to_null != 0
        && ((*config).data.sds.default_value).is_null()
    {
        0 as sds
    } else {
        sdsnew((*config).data.sds.default_value)
    };
}
unsafe extern "C" fn sdsConfigSet(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if ((*config).data.sds.is_valid_fn).is_some()
        && ((*config).data.sds.is_valid_fn)
            .expect(
                "non-null function pointer",
            )(*argv.offset(0 as libc::c_int as isize), err) == 0
    {
        return 0 as libc::c_int;
    }
    let mut prev: sds = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleStringConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.sds.config
    };
    let mut new: sds = if (*config).data.string.convert_empty_to_null != 0
        && sdslen(*argv.offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong
    {
        0 as sds
    } else {
        *argv.offset(0 as libc::c_int as isize)
    };
    if new != prev && (new.is_null() || prev.is_null() || sdscmp(prev, new) != 0) {
        sdsfree(prev);
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            return setModuleStringConfig(
                (*config).privdata as *mut ModuleConfig,
                new,
                err,
            );
        }
        *(*config).data.sds.config = if !new.is_null() { sdsdup(new) } else { 0 as sds };
        return 1 as libc::c_int;
    }
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0 && !prev.is_null()
    {
        sdsfree(prev);
    }
    return if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn sdsConfigGet(mut config: *mut standardConfig) -> sds {
    let mut val: sds = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleStringConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.sds.config
    };
    if !val.is_null() {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            return val;
        }
        return sdsdup(val);
    } else {
        return sdsnew(b"\0" as *const u8 as *const libc::c_char)
    };
}
unsafe extern "C" fn sdsConfigRewrite(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut val: sds = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleStringConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.sds.config
    };
    rewriteConfigSdsOption(state, name, val, (*config).data.sds.default_value);
    if !val.is_null()
        && (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        sdsfree(val);
    }
}
unsafe extern "C" fn enumConfigInit(mut config: *mut standardConfig) {
    *(*config).data.enumd.config = (*config).data.enumd.default_value;
}
unsafe extern "C" fn enumConfigSet(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut enumval: libc::c_int = 0;
    let mut bitflags: libc::c_int = ((*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0) as libc::c_int;
    enumval = configEnumGetValue((*config).data.enumd.enum_value, argv, argc, bitflags);
    if enumval == -(2147483647 as libc::c_int) - 1 as libc::c_int {
        let mut enumerr: sds = sdsnew(
            b"argument(s) must be one of the following: \0" as *const u8
                as *const libc::c_char,
        );
        let mut enumNode: *mut configEnum = (*config).data.enumd.enum_value;
        while !((*enumNode).name).is_null() {
            enumerr = sdscatlen(
                enumerr,
                (*enumNode).name as *const libc::c_void,
                strlen((*enumNode).name),
            );
            enumerr = sdscatlen(
                enumerr,
                b", \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            );
            enumNode = enumNode.offset(1);
        }
        sdsrange(enumerr, 0 as libc::c_int as ssize_t, -(3 as libc::c_int) as ssize_t);
        strncpy(
            loadbuf.as_mut_ptr(),
            enumerr as *const libc::c_char,
            256 as libc::c_int as libc::c_ulong,
        );
        loadbuf[(256 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
        sdsfree(enumerr);
        *err = loadbuf.as_mut_ptr();
        return 0 as libc::c_int;
    }
    if ((*config).data.enumd.is_valid_fn).is_some()
        && ((*config).data.enumd.is_valid_fn)
            .expect("non-null function pointer")(enumval, err) == 0
    {
        return 0 as libc::c_int;
    }
    let mut prev: libc::c_int = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleEnumConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.enumd.config
    };
    if prev != enumval {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            return setModuleEnumConfig(
                (*config).privdata as *mut ModuleConfig,
                enumval,
                err,
            );
        }
        *(*config).data.enumd.config = enumval;
        return 1 as libc::c_int;
    }
    return if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn enumConfigGet(mut config: *mut standardConfig) -> sds {
    let mut val: libc::c_int = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleEnumConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.enumd.config
    };
    let mut bitflags: libc::c_int = ((*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0) as libc::c_int;
    return configEnumGetName((*config).data.enumd.enum_value, val, bitflags);
}
unsafe extern "C" fn enumConfigRewrite(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut val: libc::c_int = if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        getModuleEnumConfig((*config).privdata as *mut ModuleConfig)
    } else {
        *(*config).data.enumd.config
    };
    rewriteConfigEnumOption(state, name, val, config);
}
#[no_mangle]
pub unsafe extern "C" fn setNumericType(
    mut config: *mut standardConfig,
    mut val: libc::c_longlong,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_INT as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.i = val as libc::c_int;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_UINT as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.ui = val as libc::c_uint;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.l = val as libc::c_long;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.ul = val as libc::c_ulong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG_LONG as libc::c_int as libc::c_uint
    {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            return setModuleNumericConfig(
                (*config).privdata as *mut ModuleConfig,
                val,
                err,
            )
        } else {
            *(*config).data.numeric.config.ll = val;
        }
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG_LONG as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.ull = val as libc::c_ulonglong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SIZE_T as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.st = val as size_t;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SSIZE_T as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.sst = val as ssize_t;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_OFF_T as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.ot = val as off_t;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_TIME_T as libc::c_int as libc::c_uint
    {
        *(*config).data.numeric.config.tt = val as time_t;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn numericConfigInit(mut config: *mut standardConfig) {
    setNumericType(
        config,
        (*config).data.numeric.default_value,
        0 as *mut *const libc::c_char,
    );
}
unsafe extern "C" fn numericBoundaryCheck(
    mut config: *mut standardConfig,
    mut ll: libc::c_longlong,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG_LONG as libc::c_int as libc::c_uint
        || (*config).data.numeric.numeric_type as libc::c_uint
            == NUMERIC_TYPE_UINT as libc::c_int as libc::c_uint
        || (*config).data.numeric.numeric_type as libc::c_uint
            == NUMERIC_TYPE_SIZE_T as libc::c_int as libc::c_uint
    {
        let mut ull: libc::c_ulonglong = ll as libc::c_ulonglong;
        let mut upper_bound: libc::c_ulonglong = (*config).data.numeric.upper_bound
            as libc::c_ulonglong;
        let mut lower_bound: libc::c_ulonglong = (*config).data.numeric.lower_bound
            as libc::c_ulonglong;
        if ull > upper_bound || ull < lower_bound {
            if (*config).data.numeric.flags
                & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
            {
                snprintf(
                    loadbuf.as_mut_ptr(),
                    256 as libc::c_int as libc::c_ulong,
                    b"argument must be between %llo and %llo inclusive\0" as *const u8
                        as *const libc::c_char,
                    lower_bound,
                    upper_bound,
                );
            } else {
                snprintf(
                    loadbuf.as_mut_ptr(),
                    256 as libc::c_int as libc::c_ulong,
                    b"argument must be between %llu and %llu inclusive\0" as *const u8
                        as *const libc::c_char,
                    lower_bound,
                    upper_bound,
                );
            }
            *err = loadbuf.as_mut_ptr();
            return 0 as libc::c_int;
        }
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        && ll < 0 as libc::c_int as libc::c_longlong
    {
        if ll < (*config).data.numeric.lower_bound {
            snprintf(
                loadbuf.as_mut_ptr(),
                256 as libc::c_int as libc::c_ulong,
                b"percentage argument must be less or equal to %lld\0" as *const u8
                    as *const libc::c_char,
                -(*config).data.numeric.lower_bound,
            );
            *err = loadbuf.as_mut_ptr();
            return 0 as libc::c_int;
        }
    } else if ll > (*config).data.numeric.upper_bound
        || ll < (*config).data.numeric.lower_bound
    {
        snprintf(
            loadbuf.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"argument must be between %lld and %lld inclusive\0" as *const u8
                as *const libc::c_char,
            (*config).data.numeric.lower_bound,
            (*config).data.numeric.upper_bound,
        );
        *err = loadbuf.as_mut_ptr();
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn numericParseString(
    mut config: *mut standardConfig,
    mut value: sds,
    mut err: *mut *const libc::c_char,
    mut res: *mut libc::c_longlong,
) -> libc::c_int {
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
    {
        let mut memerr: libc::c_int = 0;
        *res = memtoull(value as *const libc::c_char, &mut memerr) as libc::c_longlong;
        if memerr == 0 {
            return 1 as libc::c_int;
        }
    }
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        && sdslen(value) > 1 as libc::c_int as libc::c_ulong
        && *value
            .offset(
                (sdslen(value)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int == '%' as i32
        && string2ll(
            value as *const libc::c_char,
            (sdslen(value)).wrapping_sub(1 as libc::c_int as libc::c_ulong),
            res,
        ) != 0 && *res >= 0 as libc::c_int as libc::c_longlong
    {
        *res = -*res;
        return 1 as libc::c_int;
    }
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        *res = strtoll(value as *const libc::c_char, &mut endptr, 8 as libc::c_int);
        if *__errno_location() == 0 as libc::c_int
            && *endptr as libc::c_int == '\0' as i32
        {
            return 1 as libc::c_int;
        }
    }
    if (*config).data.numeric.flags == 0
        && string2ll(value as *const libc::c_char, sdslen(value), res) != 0
    {
        return 1 as libc::c_int;
    }
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
        && (*config).data.numeric.flags
            & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
    {
        *err = b"argument must be a memory or percent value\0" as *const u8
            as *const libc::c_char;
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
    {
        *err = b"argument must be a memory value\0" as *const u8 as *const libc::c_char;
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        *err = b"argument couldn't be parsed as an octal number\0" as *const u8
            as *const libc::c_char;
    } else {
        *err = b"argument couldn't be parsed into an integer\0" as *const u8
            as *const libc::c_char;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn numericConfigSet(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut ll: libc::c_longlong = 0;
    let mut prev: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if numericParseString(config, *argv.offset(0 as libc::c_int as isize), err, &mut ll)
        == 0
    {
        return 0 as libc::c_int;
    }
    if numericBoundaryCheck(config, ll, err) == 0 {
        return 0 as libc::c_int;
    }
    if ((*config).data.numeric.is_valid_fn).is_some()
        && ((*config).data.numeric.is_valid_fn)
            .expect("non-null function pointer")(ll, err) == 0
    {
        return 0 as libc::c_int;
    }
    if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_INT as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.i as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_UINT as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.ui as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.l as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.ul as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG_LONG as libc::c_int as libc::c_uint
    {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            prev = getModuleNumericConfig((*config).privdata as *mut ModuleConfig);
        } else {
            prev = *(*config).data.numeric.config.ll;
        }
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG_LONG as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.ull as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SIZE_T as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.st as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SSIZE_T as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.sst as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_OFF_T as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.ot as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_TIME_T as libc::c_int as libc::c_uint
    {
        prev = *(*config).data.numeric.config.tt as libc::c_longlong;
    }
    if prev != ll {
        return setNumericType(config, ll, err);
    }
    return if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        2 as libc::c_int
    };
}
unsafe extern "C" fn numericConfigGet(mut config: *mut standardConfig) -> sds {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut value: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_INT as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.i as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_UINT as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ui as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.l as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ul as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG_LONG as libc::c_int as libc::c_uint
    {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            value = getModuleNumericConfig((*config).privdata as *mut ModuleConfig);
        } else {
            value = *(*config).data.numeric.config.ll;
        }
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG_LONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ull as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SIZE_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.st as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SSIZE_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.sst as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_OFF_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ot as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_TIME_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.tt as libc::c_longlong;
    }
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        && value < 0 as libc::c_int as libc::c_longlong
    {
        let mut len: libc::c_int = ll2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            -value,
        );
        buf[len as usize] = '%' as i32 as libc::c_char;
        buf[(len + 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
    {
        ull2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            value as libc::c_ulonglong,
        );
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"%llo\0" as *const u8 as *const libc::c_char,
            value,
        );
    } else {
        ll2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            value,
        );
    }
    return sdsnew(buf.as_mut_ptr());
}
unsafe extern "C" fn numericConfigRewrite(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut value: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_INT as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.i as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_UINT as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ui as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.l as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ul as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_LONG_LONG as libc::c_int as libc::c_uint
    {
        if (*config).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
        {
            value = getModuleNumericConfig((*config).privdata as *mut ModuleConfig);
        } else {
            value = *(*config).data.numeric.config.ll;
        }
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_ULONG_LONG as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ull as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SIZE_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.st as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_SSIZE_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.sst as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_OFF_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.ot as libc::c_longlong;
    } else if (*config).data.numeric.numeric_type as libc::c_uint
        == NUMERIC_TYPE_TIME_T as libc::c_int as libc::c_uint
    {
        value = *(*config).data.numeric.config.tt as libc::c_longlong;
    }
    if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0
        && value < 0 as libc::c_int as libc::c_longlong
    {
        rewriteConfigPercentOption(
            state,
            name,
            -value,
            (*config).data.numeric.default_value,
        );
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint != 0
    {
        rewriteConfigBytesOption(
            state,
            name,
            value,
            (*config).data.numeric.default_value,
        );
    } else if (*config).data.numeric.flags
        & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_uint != 0
    {
        rewriteConfigOctalOption(
            state,
            name,
            value,
            (*config).data.numeric.default_value,
        );
    } else {
        rewriteConfigNumericalOption(
            state,
            name,
            value,
            (*config).data.numeric.default_value,
        );
    };
}
unsafe extern "C" fn isValidActiveDefrag(
    mut val: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidDBfilename(
    mut val: *mut libc::c_char,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if pathIsBaseName(val) == 0 {
        *err = b"dbfilename can't be a path, just a filename\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidAOFfilename(
    mut val: *mut libc::c_char,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if strcmp(val, b"\0" as *const u8 as *const libc::c_char) == 0 {
        *err = b"appendfilename can't be empty\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    if pathIsBaseName(val) == 0 {
        *err = b"appendfilename can't be a path, just a filename\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidAOFdirname(
    mut val: *mut libc::c_char,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if strcmp(val, b"\0" as *const u8 as *const libc::c_char) == 0 {
        *err = b"appenddirname can't be empty\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    if pathIsBaseName(val) == 0 {
        *err = b"appenddirname can't be a path, just a dirname\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidShutdownOnSigFlags(
    mut val: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if val & 2 as libc::c_int != 0 && val & 1 as libc::c_int != 0 {
        *err = b"shutdown options SAVE and NOSAVE can't be used simultaneously\0"
            as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidAnnouncedHostname(
    mut val: *mut libc::c_char,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if strlen(val) >= 256 as libc::c_int as libc::c_ulong {
        *err = b"Hostnames must be less than 256 characters\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut c: libc::c_char = 0;
    loop {
        c = *val.offset(i as isize);
        if !(c != 0) {
            break;
        }
        if !(c as libc::c_int >= 'A' as i32 && c as libc::c_int <= 'Z' as i32
            || c as libc::c_int >= 'a' as i32 && c as libc::c_int <= 'z' as i32
            || c as libc::c_int >= '0' as i32 && c as libc::c_int <= '9' as i32
            || c as libc::c_int == '-' as i32 || c as libc::c_int == '.' as i32)
        {
            *err = b"Hostnames may only contain alphanumeric characters, hyphens or dots\0"
                as *const u8 as *const libc::c_char;
            return 0 as libc::c_int;
        }
        let fresh13 = i;
        i = i + 1;
        c = *val.offset(fresh13 as isize);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn isValidProcTitleTemplate(
    mut val: *mut libc::c_char,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if validateProcTitleTemplate(val) == 0 {
        *err = b"template format is invalid or contains unknown variables\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateProcTitleTemplate(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if redisSetProcTitle(0 as *mut libc::c_char) == -(1 as libc::c_int) {
        *err = b"failed to set process title\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateHZ(mut err: *mut *const libc::c_char) -> libc::c_int {
    if server.config_hz < 1 as libc::c_int {
        server.config_hz = 1 as libc::c_int;
    }
    if server.config_hz > 500 as libc::c_int {
        server.config_hz = 500 as libc::c_int;
    }
    server.hz = server.config_hz;
    return 1 as libc::c_int;
}
unsafe extern "C" fn updatePort(mut err: *mut *const libc::c_char) -> libc::c_int {
    if changeListenPort(
        server.port,
        &mut server.ipfd,
        Some(
            acceptTcpHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) == -(1 as libc::c_int)
    {
        *err = b"Unable to listen on this port. Check server logs.\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateJemallocBgThread(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    set_jemalloc_bg_thread(server.jemalloc_bg_thread);
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateReplBacklogSize(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    resizeReplicationBacklog();
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateMaxmemory(mut err: *mut *const libc::c_char) -> libc::c_int {
    if server.maxmemory != 0 {
        let mut used: size_t = (zmalloc_used_memory())
            .wrapping_sub(freeMemoryGetNotCountedMemory());
        if server.maxmemory < used as libc::c_ulonglong {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"WARNING: the new maxmemory value set via CONFIG SET (%llu) is smaller than the current memory usage (%zu). This will result in key eviction and/or the inability to accept new write commands depending on the maxmemory-policy.\0"
                        as *const u8 as *const libc::c_char,
                    server.maxmemory,
                    used,
                );
            }
        }
        startEvictionTimeProc();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateGoodSlaves(mut err: *mut *const libc::c_char) -> libc::c_int {
    refreshGoodSlavesCount();
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateWatchdogPeriod(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    applyWatchdogPeriod();
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateAppendonly(mut err: *mut *const libc::c_char) -> libc::c_int {
    if server.aof_enabled == 0 && server.aof_state != 0 as libc::c_int {
        stopAppendOnly();
    } else if server.aof_enabled != 0 && server.aof_state == 0 as libc::c_int {
        if startAppendOnly() == -(1 as libc::c_int) {
            *err = b"Unable to turn on AOF. Check server logs.\0" as *const u8
                as *const libc::c_char;
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateAofAutoGCEnabled(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if server.aof_disable_auto_gc == 0 {
        aofDelHistoryFiles();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateSighandlerEnabled(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if server.crashlog_enabled != 0 {
        setupSignalHandlers();
    } else {
        removeSignalHandlers();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateMaxclients(mut err: *mut *const libc::c_char) -> libc::c_int {
    let mut new_maxclients: libc::c_uint = server.maxclients;
    adjustOpenFilesLimit();
    if server.maxclients != new_maxclients {
        static mut msg: [libc::c_char; 128] = [0; 128];
        sprintf(
            msg.as_mut_ptr(),
            b"The operating system is not able to handle the specified number of clients, try with %d\0"
                as *const u8 as *const libc::c_char,
            server.maxclients,
        );
        *err = msg.as_mut_ptr();
        return 0 as libc::c_int;
    }
    if (aeGetSetSize(server.el) as libc::c_uint)
        < (server.maxclients)
            .wrapping_add((32 as libc::c_int + 96 as libc::c_int) as libc::c_uint)
    {
        if aeResizeSetSize(
            server.el,
            (server.maxclients)
                .wrapping_add((32 as libc::c_int + 96 as libc::c_int) as libc::c_uint)
                as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            *err = b"The event loop API used by Redis is not able to handle the specified number of clients\0"
                as *const u8 as *const libc::c_char;
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateOOMScoreAdj(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if setOOMScoreAdj(-(1 as libc::c_int)) == -(1 as libc::c_int) {
        *err = b"Failed to set current oom_score_adj. Check server logs.\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateRequirePass(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    ACLUpdateDefaultUserPassword(server.requirepass);
    return 1 as libc::c_int;
}
unsafe extern "C" fn applyBind(mut err: *mut *const libc::c_char) -> libc::c_int {
    if changeBindAddr() == -(1 as libc::c_int) {
        *err = b"Failed to bind to specified addresses.\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateClusterFlags(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    clusterUpdateMyselfFlags();
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateClusterAnnouncedPort(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    clusterUpdateMyselfAnnouncedPorts();
    return 1 as libc::c_int;
}
unsafe extern "C" fn updateClusterIp(mut err: *mut *const libc::c_char) -> libc::c_int {
    clusterUpdateMyselfIp();
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateClusterHostname(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    clusterUpdateMyselfHostname();
    return 1 as libc::c_int;
}
unsafe extern "C" fn setConfigDirOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if argc != 1 as libc::c_int {
        *err = b"wrong number of arguments\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    if chdir(*argv.offset(0 as libc::c_int as isize) as *const libc::c_char)
        == -(1 as libc::c_int)
    {
        *err = strerror(*__errno_location());
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn getConfigDirOption(mut config: *mut standardConfig) -> sds {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if (getcwd(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
    ))
        .is_null()
    {
        buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    return sdsnew(buf.as_mut_ptr());
}
unsafe extern "C" fn setConfigSaveOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if argc == 1 as libc::c_int
        && strcasecmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        resetServerSaveParams();
        argc = 0 as libc::c_int;
    }
    if argc & 1 as libc::c_int != 0 {
        *err = b"Invalid save parameters\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < argc {
        let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: libc::c_long = 0;
        val = strtoll(
            *argv.offset(j as isize) as *const libc::c_char,
            &mut eptr,
            10 as libc::c_int,
        ) as libc::c_long;
        if *eptr.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32
            || j & 1 as libc::c_int == 0 as libc::c_int
                && val < 1 as libc::c_int as libc::c_long
            || j & 1 as libc::c_int == 1 as libc::c_int
                && val < 0 as libc::c_int as libc::c_long
        {
            *err = b"Invalid save parameters\0" as *const u8 as *const libc::c_char;
            return 0 as libc::c_int;
        }
        j += 1;
    }
    if reading_config_file == 0 {
        resetServerSaveParams();
    } else {
        static mut save_loaded: libc::c_int = 0 as libc::c_int;
        if save_loaded == 0 {
            save_loaded = 1 as libc::c_int;
            resetServerSaveParams();
        }
    }
    j = 0 as libc::c_int;
    while j < argc {
        let mut seconds: time_t = 0;
        let mut changes: libc::c_int = 0;
        seconds = strtoll(
            *argv.offset(j as isize) as *const libc::c_char,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as time_t;
        changes = strtoll(
            *argv.offset((j + 1 as libc::c_int) as isize) as *const libc::c_char,
            0 as *mut *mut libc::c_char,
            10 as libc::c_int,
        ) as libc::c_int;
        appendServerSaveParams(seconds, changes);
        j += 2 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn getConfigSaveOption(mut config: *mut standardConfig) -> sds {
    let mut buf: sds = sdsempty();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < server.saveparamslen {
        buf = sdscatprintf(
            buf,
            b"%jd %d\0" as *const u8 as *const libc::c_char,
            (*(server.saveparams).offset(j as isize)).seconds,
            (*(server.saveparams).offset(j as isize)).changes,
        );
        if j != server.saveparamslen - 1 as libc::c_int {
            buf = sdscatlen(
                buf,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    return buf;
}
unsafe extern "C" fn setConfigClientOutputBufferLimitOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    return updateClientOutputBufferLimit(argv, argc, err);
}
unsafe extern "C" fn getConfigClientOutputBufferLimitOption(
    mut config: *mut standardConfig,
) -> sds {
    let mut buf: sds = sdsempty();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        buf = sdscatprintf(
            buf,
            b"%s %llu %llu %ld\0" as *const u8 as *const libc::c_char,
            getClientTypeName(j),
            server.client_obuf_limits[j as usize].hard_limit_bytes,
            server.client_obuf_limits[j as usize].soft_limit_bytes,
            server.client_obuf_limits[j as usize].soft_limit_seconds,
        );
        if j != 3 as libc::c_int - 1 as libc::c_int {
            buf = sdscatlen(
                buf,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    return buf;
}
unsafe extern "C" fn setConfigOOMScoreAdjValuesOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut values: [libc::c_int; 3] = [0; 3];
    let mut change: libc::c_int = 0 as libc::c_int;
    if argc != 3 as libc::c_int {
        *err = b"wrong number of arguments\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        let mut eptr: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut val: libc::c_longlong = strtoll(
            *argv.offset(i as isize) as *const libc::c_char,
            &mut eptr,
            10 as libc::c_int,
        );
        if *eptr as libc::c_int != '\0' as i32
            || val < -(2000 as libc::c_int) as libc::c_longlong
            || val > 2000 as libc::c_int as libc::c_longlong
        {
            if !err.is_null() {
                *err = b"Invalid oom-score-adj-values, elements must be between -2000 and 2000.\0"
                    as *const u8 as *const libc::c_char;
            }
            return 0 as libc::c_int;
        }
        values[i as usize] = val as libc::c_int;
        i += 1;
    }
    if values[1 as libc::c_int as usize] < values[0 as libc::c_int as usize]
        || values[2 as libc::c_int as usize] < values[1 as libc::c_int as usize]
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"The oom-score-adj-values configuration may not work for non-privileged processes! Please consult the documentation.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    i = 0 as libc::c_int;
    while i < 3 as libc::c_int {
        if server.oom_score_adj_values[i as usize] != values[i as usize] {
            server.oom_score_adj_values[i as usize] = values[i as usize];
            change = 1 as libc::c_int;
        }
        i += 1;
    }
    return if change != 0 { 1 as libc::c_int } else { 2 as libc::c_int };
}
unsafe extern "C" fn getConfigOOMScoreAdjValuesOption(
    mut config: *mut standardConfig,
) -> sds {
    let mut buf: sds = sdsempty();
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        buf = sdscatprintf(
            buf,
            b"%d\0" as *const u8 as *const libc::c_char,
            server.oom_score_adj_values[j as usize],
        );
        if j != 3 as libc::c_int - 1 as libc::c_int {
            buf = sdscatlen(
                buf,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    return buf;
}
unsafe extern "C" fn setConfigNotifyKeyspaceEventsOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if argc != 1 as libc::c_int {
        *err = b"wrong number of arguments\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    let mut flags: libc::c_int = keyspaceEventsStringToFlags(
        *argv.offset(0 as libc::c_int as isize),
    );
    if flags == -(1 as libc::c_int) {
        *err = b"Invalid event class character. Use 'Ag$lshzxeKEtmdn'.\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    server.notify_keyspace_events = flags;
    return 1 as libc::c_int;
}
unsafe extern "C" fn getConfigNotifyKeyspaceEventsOption(
    mut config: *mut standardConfig,
) -> sds {
    return keyspaceEventsFlagsToString(server.notify_keyspace_events);
}
unsafe extern "C" fn setConfigBindOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if argc > 16 as libc::c_int {
        *err = b"Too many bind addresses specified.\0" as *const u8
            as *const libc::c_char;
        return 0 as libc::c_int;
    }
    if argc == 1 as libc::c_int
        && sdslen(*argv.offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong
    {
        argc = 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < server.bindaddr_count {
        zfree(server.bindaddr[j as usize] as *mut libc::c_void);
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < argc {
        server
            .bindaddr[j
            as usize] = zstrdup(*argv.offset(j as isize) as *const libc::c_char);
        j += 1;
    }
    server.bindaddr_count = argc;
    return 1 as libc::c_int;
}
unsafe extern "C" fn setConfigReplicaOfOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    if argc != 2 as libc::c_int {
        *err = b"wrong number of arguments\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    sdsfree(server.masterhost);
    server.masterhost = 0 as *mut libc::c_char;
    if strcasecmp(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
        b"no\0" as *const u8 as *const libc::c_char,
    ) == 0
        && strcasecmp(
            *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
            b"one\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        return 1 as libc::c_int;
    }
    let mut ptr: *mut libc::c_char = 0 as *mut libc::c_char;
    server
        .masterport = strtol(
        *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
        &mut ptr,
        10 as libc::c_int,
    ) as libc::c_int;
    if server.masterport < 0 as libc::c_int || server.masterport > 65535 as libc::c_int
        || *ptr as libc::c_int != '\0' as i32
    {
        *err = b"Invalid master port\0" as *const u8 as *const libc::c_char;
        return 0 as libc::c_int;
    }
    server
        .masterhost = sdsnew(
        *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
    );
    server.repl_state = REPL_STATE_CONNECT as libc::c_int;
    return 1 as libc::c_int;
}
unsafe extern "C" fn getConfigBindOption(mut config: *mut standardConfig) -> sds {
    return sdsjoin(
        (server.bindaddr).as_mut_ptr(),
        server.bindaddr_count,
        b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn getConfigReplicaOfOption(mut config: *mut standardConfig) -> sds {
    let mut buf: [libc::c_char; 256] = [0; 256];
    if !(server.masterhost).is_null() {
        snprintf(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
            b"%s %d\0" as *const u8 as *const libc::c_char,
            server.masterhost,
            server.masterport,
        );
    } else {
        buf[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    return sdsnew(buf.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn allowProtectedAction(
    mut config: libc::c_int,
    mut c: *mut client,
) -> libc::c_int {
    return (config == 1 as libc::c_int
        || config == 2 as libc::c_int && islocalClient(c) != 0) as libc::c_int;
}
unsafe extern "C" fn setConfigLatencyTrackingInfoPercentilesOutputOption(
    mut config: *mut standardConfig,
    mut argv: *mut sds,
    mut argc: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    zfree(server.latency_tracking_info_percentiles as *mut libc::c_void);
    server.latency_tracking_info_percentiles = 0 as *mut libc::c_double;
    server.latency_tracking_info_percentiles_len = argc;
    if argc == 1 as libc::c_int
        && sdslen(*argv.offset(0 as libc::c_int as isize))
            == 0 as libc::c_int as libc::c_ulong
    {
        server.latency_tracking_info_percentiles_len = 0 as libc::c_int;
    } else {
        server
            .latency_tracking_info_percentiles = zmalloc(
            (core::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(argc as libc::c_ulong),
        ) as *mut libc::c_double;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    loop {
        if !(j < server.latency_tracking_info_percentiles_len) {
            current_block = 7651349459974463963;
            break;
        }
        let mut percentile: libc::c_double = 0.;
        if string2d(
            *argv.offset(j as isize) as *const libc::c_char,
            sdslen(*argv.offset(j as isize)),
            &mut percentile,
        ) == 0
        {
            *err = b"Invalid latency-tracking-info-percentiles parameters\0" as *const u8
                as *const libc::c_char;
            current_block = 13511339451016918291;
            break;
        } else if percentile > 100.0f64 || percentile < 0.0f64 {
            *err = b"latency-tracking-info-percentiles parameters should sit between [0.0,100.0]\0"
                as *const u8 as *const libc::c_char;
            current_block = 13511339451016918291;
            break;
        } else {
            *(server.latency_tracking_info_percentiles).offset(j as isize) = percentile;
            j += 1;
        }
    }
    match current_block {
        7651349459974463963 => return 1 as libc::c_int,
        _ => {
            zfree(server.latency_tracking_info_percentiles as *mut libc::c_void);
            server.latency_tracking_info_percentiles = 0 as *mut libc::c_double;
            server.latency_tracking_info_percentiles_len = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
    };
}
unsafe extern "C" fn getConfigLatencyTrackingInfoPercentilesOutputOption(
    mut config: *mut standardConfig,
) -> sds {
    let mut buf: sds = sdsempty();
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < server.latency_tracking_info_percentiles_len {
        let mut fbuf: [libc::c_char; 128] = [0; 128];
        let mut len: size_t = sprintf(
            fbuf.as_mut_ptr(),
            b"%f\0" as *const u8 as *const libc::c_char,
            *(server.latency_tracking_info_percentiles).offset(j as isize),
        ) as size_t;
        len = trimDoubleString(fbuf.as_mut_ptr(), len) as size_t;
        buf = sdscatlen(buf, fbuf.as_mut_ptr() as *const libc::c_void, len);
        if j != server.latency_tracking_info_percentiles_len - 1 as libc::c_int {
            buf = sdscatlen(
                buf,
                b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteConfigLatencyTrackingInfoPercentilesOutputOption(
    mut config: *mut standardConfig,
    mut name: *const libc::c_char,
    mut state: *mut rewriteConfigState,
) {
    let mut line: sds = sdsnew(name);
    if server.latency_tracking_info_percentiles_len == 0 {
        line = sdscat(line, b" \"\"\0" as *const u8 as *const libc::c_char);
    } else {
        let mut j: libc::c_int = 0 as libc::c_int;
        while j < server.latency_tracking_info_percentiles_len {
            let mut fbuf: [libc::c_char; 128] = [0; 128];
            let mut len: size_t = sprintf(
                fbuf.as_mut_ptr(),
                b" %f\0" as *const u8 as *const libc::c_char,
                *(server.latency_tracking_info_percentiles).offset(j as isize),
            ) as size_t;
            len = trimDoubleString(fbuf.as_mut_ptr(), len) as size_t;
            line = sdscatlen(line, fbuf.as_mut_ptr() as *const libc::c_void, len);
            j += 1;
        }
    }
    rewriteConfigRewriteLine(state, name, line, 1 as libc::c_int);
}
unsafe extern "C" fn applyClientMaxMemoryUsage(
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    if server.maxmemory_clients != 0 as libc::c_int as libc::c_long
        && !(server.client_mem_usage_buckets).is_null()
    {
        return 1 as libc::c_int;
    }
    if server.maxmemory_clients != 0 as libc::c_int as libc::c_long {
        initServerClientMemUsageBuckets();
    }
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        if server.maxmemory_clients == 0 as libc::c_int as libc::c_long {
            removeClientFromMemUsageBucket(c, 0 as libc::c_int);
        } else {
            updateClientMemUsageAndBucket(c);
        }
    }
    if server.maxmemory_clients == 0 as libc::c_int as libc::c_long {
        freeServerClientMemUsageBuckets();
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut static_configs: [standardConfig; 159] = [standardConfig {
    name: 0 as *const libc::c_char,
    alias: 0 as *const libc::c_char,
    flags: 0,
    interface: typeInterface {
        init: None,
        set: None,
        apply: None,
        get: None,
        rewrite: None,
    },
    data: typeData {
        yesno: boolConfigData {
            config: 0 as *mut libc::c_int,
            default_value: 0,
            is_valid_fn: None,
        },
    },
    type_0: BOOL_CONFIG,
    privdata: 0 as *mut libc::c_void,
}; 159];
#[no_mangle]
pub unsafe extern "C" fn registerConfigValue(
    mut name: *const libc::c_char,
    mut config: *const standardConfig,
    mut alias: libc::c_int,
) -> libc::c_int {
    let mut new: *mut standardConfig = zmalloc(
        core::mem::size_of::<standardConfig>() as libc::c_ulong,
    ) as *mut standardConfig;
    memcpy(
        new as *mut libc::c_void,
        config as *const libc::c_void,
        core::mem::size_of::<standardConfig>() as libc::c_ulong,
    );
    if alias != 0 {
        (*new)
            .flags = ((*new).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 7 as libc::c_int) as libc::c_uint;
        (*new).name = (*config).alias;
        (*new).alias = (*config).name;
    }
    return (dictAdd(configs, sdsnew(name) as *mut libc::c_void, new as *mut libc::c_void)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initConfigValues() {
    configs = dictCreate(&mut sdsHashDictType);
    dictExpand(
        configs,
        (core::mem::size_of::<[standardConfig; 159]>() as libc::c_ulong)
            .wrapping_div(core::mem::size_of::<standardConfig>() as libc::c_ulong),
    );
    let mut config: *mut standardConfig = static_configs.as_mut_ptr();
    while !((*config).name).is_null() {
        if ((*config).interface.init).is_some() {
            ((*config).interface.init).expect("non-null function pointer")(config);
        }
        let mut ret: libc::c_int = registerConfigValue(
            (*config).name,
            config,
            0 as libc::c_int,
        );
        if ret != 0 {} else {
            _serverAssert(
                b"ret\0" as *const u8 as *const libc::c_char,
                b"config.c\0" as *const u8 as *const libc::c_char,
                3199 as libc::c_int,
            );
            unreachable!();
        };
        if !((*config).alias).is_null() {
            let mut ret_0: libc::c_int = registerConfigValue(
                (*config).alias,
                config,
                ((1 as libc::c_ulonglong) << 7 as libc::c_int) as libc::c_int,
            );
            if ret_0 != 0 {} else {
                _serverAssert(
                    b"ret\0" as *const u8 as *const libc::c_char,
                    b"config.c\0" as *const u8 as *const libc::c_char,
                    3205 as libc::c_int,
                );
                unreachable!();
            };
        }
        config = config.offset(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn removeConfig(mut name: sds) {
    let mut config: *mut standardConfig = lookupConfig(name);
    if config.is_null() {
        return;
    }
    if (*config).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        sdsfree((*config).name as sds);
        if (*config).type_0 as libc::c_uint == ENUM_CONFIG as libc::c_int as libc::c_uint
        {
            let mut enumNode: *mut configEnum = (*config).data.enumd.enum_value;
            while !((*enumNode).name).is_null() {
                zfree((*enumNode).name as *mut libc::c_void);
                enumNode = enumNode.offset(1);
            }
            zfree((*config).data.enumd.enum_value as *mut libc::c_void);
        } else if (*config).type_0 as libc::c_uint
            == SDS_CONFIG as libc::c_int as libc::c_uint
        {
            if !((*config).data.sds.default_value).is_null() {
                sdsfree((*config).data.sds.default_value);
            }
        }
    }
    dictDelete(configs, name as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn addModuleBoolConfig(
    mut module_name: *const libc::c_char,
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut default_val: libc::c_int,
) {
    let mut config_name: sds = sdscatfmt(
        sdsempty(),
        b"%s.%s\0" as *const u8 as *const libc::c_char,
        module_name,
        name,
    );
    let mut config_dummy_address: libc::c_int = 0;
    let mut module_config: standardConfig = {
        let mut init = standardConfig {
            name: config_name as *const libc::c_char,
            alias: 0 as *const libc::c_char,
            flags: (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 8 as libc::c_int) as libc::c_uint,
            interface: {
                let mut init = typeInterface {
                    init: Some(
                        boolConfigInit as unsafe extern "C" fn(*mut standardConfig) -> (),
                    ),
                    set: Some(
                        boolConfigSet
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *mut sds,
                                libc::c_int,
                                *mut *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                    apply: None,
                    get: Some(
                        boolConfigGet as unsafe extern "C" fn(*mut standardConfig) -> sds,
                    ),
                    rewrite: Some(
                        boolConfigRewrite
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *const libc::c_char,
                                *mut rewriteConfigState,
                            ) -> (),
                    ),
                };
                init
            },
            data: typeData {
                yesno: {
                    let mut init = boolConfigData {
                        config: &mut config_dummy_address,
                        default_value: default_val,
                        is_valid_fn: None,
                    };
                    init
                },
            },
            type_0: BOOL_CONFIG,
            privdata: 0 as *mut libc::c_void,
        };
        init
    };
    module_config.data.yesno.config = 0 as *mut libc::c_int;
    module_config.privdata = privdata;
    registerConfigValue(
        config_name as *const libc::c_char,
        &mut module_config,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addModuleStringConfig(
    mut module_name: *const libc::c_char,
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut default_val: sds,
) {
    let mut config_name: sds = sdscatfmt(
        sdsempty(),
        b"%s.%s\0" as *const u8 as *const libc::c_char,
        module_name,
        name,
    );
    let mut config_dummy_address: sds = 0 as *mut libc::c_char;
    let mut module_config: standardConfig = {
        let mut init = standardConfig {
            name: config_name as *const libc::c_char,
            alias: 0 as *const libc::c_char,
            flags: (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 8 as libc::c_int) as libc::c_uint,
            interface: {
                let mut init = typeInterface {
                    init: Some(
                        sdsConfigInit as unsafe extern "C" fn(*mut standardConfig) -> (),
                    ),
                    set: Some(
                        sdsConfigSet
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *mut sds,
                                libc::c_int,
                                *mut *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                    apply: None,
                    get: Some(
                        sdsConfigGet as unsafe extern "C" fn(*mut standardConfig) -> sds,
                    ),
                    rewrite: Some(
                        sdsConfigRewrite
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *const libc::c_char,
                                *mut rewriteConfigState,
                            ) -> (),
                    ),
                };
                init
            },
            data: typeData {
                sds: {
                    let mut init = sdsConfigData {
                        config: &mut config_dummy_address,
                        default_value: default_val,
                        is_valid_fn: None,
                        convert_empty_to_null: 0 as libc::c_int,
                    };
                    init
                },
            },
            type_0: SDS_CONFIG,
            privdata: 0 as *mut libc::c_void,
        };
        init
    };
    module_config.data.sds.config = 0 as *mut sds;
    module_config.privdata = privdata;
    registerConfigValue(
        config_name as *const libc::c_char,
        &mut module_config,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addModuleEnumConfig(
    mut module_name: *const libc::c_char,
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut default_val: libc::c_int,
    mut enum_vals: *mut configEnum,
) {
    let mut config_name: sds = sdscatfmt(
        sdsempty(),
        b"%s.%s\0" as *const u8 as *const libc::c_char,
        module_name,
        name,
    );
    let mut config_dummy_address: libc::c_int = 0;
    let mut module_config: standardConfig = {
        let mut init = standardConfig {
            name: config_name as *const libc::c_char,
            alias: 0 as *const libc::c_char,
            flags: (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 8 as libc::c_int) as libc::c_uint,
            interface: {
                let mut init = typeInterface {
                    init: Some(
                        enumConfigInit as unsafe extern "C" fn(*mut standardConfig) -> (),
                    ),
                    set: Some(
                        enumConfigSet
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *mut sds,
                                libc::c_int,
                                *mut *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                    apply: None,
                    get: Some(
                        enumConfigGet as unsafe extern "C" fn(*mut standardConfig) -> sds,
                    ),
                    rewrite: Some(
                        enumConfigRewrite
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *const libc::c_char,
                                *mut rewriteConfigState,
                            ) -> (),
                    ),
                };
                init
            },
            data: typeData {
                enumd: {
                    let mut init = enumConfigData {
                        config: &mut config_dummy_address,
                        enum_value: enum_vals,
                        default_value: default_val,
                        is_valid_fn: None,
                    };
                    init
                },
            },
            type_0: ENUM_CONFIG,
            privdata: 0 as *mut libc::c_void,
        };
        init
    };
    module_config.data.enumd.config = 0 as *mut libc::c_int;
    module_config.privdata = privdata;
    registerConfigValue(
        config_name as *const libc::c_char,
        &mut module_config,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn addModuleNumericConfig(
    mut module_name: *const libc::c_char,
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut default_val: libc::c_longlong,
    mut conf_flags: libc::c_int,
    mut lower: libc::c_longlong,
    mut upper: libc::c_longlong,
) {
    let mut config_name: sds = sdscatfmt(
        sdsempty(),
        b"%s.%s\0" as *const u8 as *const libc::c_char,
        module_name,
        name,
    );
    let mut config_dummy_address: libc::c_longlong = 0;
    let mut module_config: standardConfig = {
        let mut init = standardConfig {
            name: config_name as *const libc::c_char,
            alias: 0 as *const libc::c_char,
            flags: (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 8 as libc::c_int) as libc::c_uint,
            interface: {
                let mut init = typeInterface {
                    init: Some(
                        numericConfigInit
                            as unsafe extern "C" fn(*mut standardConfig) -> (),
                    ),
                    set: Some(
                        numericConfigSet
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *mut sds,
                                libc::c_int,
                                *mut *const libc::c_char,
                            ) -> libc::c_int,
                    ),
                    apply: None,
                    get: Some(
                        numericConfigGet
                            as unsafe extern "C" fn(*mut standardConfig) -> sds,
                    ),
                    rewrite: Some(
                        numericConfigRewrite
                            as unsafe extern "C" fn(
                                *mut standardConfig,
                                *const libc::c_char,
                                *mut rewriteConfigState,
                            ) -> (),
                    ),
                };
                init
            },
            data: typeData {
                numeric: {
                    let mut init = numericConfigData {
                        config: C2RustUnnamed_10 {
                            ll: &mut config_dummy_address,
                        },
                        flags: conf_flags as libc::c_uint,
                        numeric_type: NUMERIC_TYPE_LONG_LONG,
                        lower_bound: lower,
                        upper_bound: upper,
                        default_value: default_val,
                        is_valid_fn: None,
                    };
                    init
                },
            },
            type_0: NUMERIC_CONFIG,
            privdata: 0 as *mut libc::c_void,
        };
        init
    };
    module_config.data.numeric.config.ll = 0 as *mut libc::c_longlong;
    module_config.privdata = privdata;
    registerConfigValue(
        config_name as *const libc::c_char,
        &mut module_config,
        0 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn configHelpCommand(mut c: *mut client) {
    let mut help: [*const libc::c_char; 9] = [
        b"GET <pattern>\0" as *const u8 as *const libc::c_char,
        b"    Return parameters matching the glob-like <pattern> and their values.\0"
            as *const u8 as *const libc::c_char,
        b"SET <directive> <value>\0" as *const u8 as *const libc::c_char,
        b"    Set the configuration <directive> to <value>.\0" as *const u8
            as *const libc::c_char,
        b"RESETSTAT\0" as *const u8 as *const libc::c_char,
        b"    Reset statistics reported by the INFO command.\0" as *const u8
            as *const libc::c_char,
        b"REWRITE\0" as *const u8 as *const libc::c_char,
        b"    Rewrite the configuration file.\0" as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    addReplyHelp(c, help.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn configResetStatCommand(mut c: *mut client) {
    resetServerStats();
    resetCommandTableStats(server.commands);
    resetErrorTableStats();
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn configRewriteCommand(mut c: *mut client) {
    if (server.configfile).is_null() {
        addReplyError(
            c,
            b"The server is running without a config file\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if rewriteConfig(server.configfile, 0 as libc::c_int) == -(1 as libc::c_int) {
        let mut err: libc::c_int = *__errno_location();
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"CONFIG REWRITE failed: %s\0" as *const u8 as *const libc::c_char,
                strerror(err),
            );
        }
        addReplyErrorFormat(
            c,
            b"Rewriting config file: %s\0" as *const u8 as *const libc::c_char,
            strerror(err),
        );
    } else {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"CONFIG REWRITE executed with success.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        addReply(c, shared.ok);
    };
}
unsafe extern "C" fn run_static_initializers() {
    static_configs = [
        {
            let mut init = standardConfig {
                name: b"rdbchecksum\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.rdb_checksum,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"daemonize\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.daemonize,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"io-threads-do-reads\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | (1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.io_threads_do_reads,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"always-show-logo\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.always_show_logo,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"protected-mode\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.protected_mode,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"rdbcompression\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.rdb_compression,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"rdb-del-sync-files\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.rdb_del_sync_files,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"activerehashing\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.activerehashing,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"stop-writes-on-bgsave-error\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.stop_writes_on_bgsave_err,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"set-proc-title\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.set_proc_title,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"dynamic-hz\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.dynamic_hz,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lazyfree-lazy-eviction\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.lazyfree_lazy_eviction,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lazyfree-lazy-expire\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.lazyfree_lazy_expire,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lazyfree-lazy-server-del\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.lazyfree_lazy_server_del,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lazyfree-lazy-user-del\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.lazyfree_lazy_user_del,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lazyfree-lazy-user-flush\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.lazyfree_lazy_user_flush,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-disable-tcp-nodelay\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_disable_tcp_nodelay,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-diskless-sync\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_diskless_sync,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof-rewrite-incremental-fsync\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_rewrite_incremental_fsync,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"no-appendfsync-on-rewrite\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_no_fsync_on_rewrite,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-require-full-coverage\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_require_full_coverage,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"rdb-save-incremental-fsync\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.rdb_save_incremental_fsync,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof-load-truncated\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_load_truncated,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof-use-rdb-preamble\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_use_rdb_preamble,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof-timestamp-enabled\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_timestamp_enabled,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-replica-no-failover\0" as *const u8
                    as *const libc::c_char,
                alias: b"cluster-slave-no-failover\0" as *const u8
                    as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterFlags
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_slave_no_failover,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-lazy-flush\0" as *const u8 as *const libc::c_char,
                alias: b"slave-lazy-flush\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_slave_lazy_flush,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-serve-stale-data\0" as *const u8 as *const libc::c_char,
                alias: b"slave-serve-stale-data\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_serve_stale_data,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-read-only\0" as *const u8 as *const libc::c_char,
                alias: b"slave-read-only\0" as *const u8 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_slave_ro,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-ignore-maxmemory\0" as *const u8 as *const libc::c_char,
                alias: b"slave-ignore-maxmemory\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_slave_ignore_maxmemory,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"jemalloc-bg-thread\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateJemallocBgThread
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.jemalloc_bg_thread,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"activedefrag\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.active_defrag_enabled,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: Some(
                                isValidActiveDefrag
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"syslog-enabled\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.syslog_enabled,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-enabled\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_enabled,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"appendonly\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 6 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateAppendonly
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_enabled,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-allow-reads-when-down\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_allow_reads_when_down,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-allow-pubsubshard-when-down\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_allow_pubsubshard_when_down,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"crash-log-enabled\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateSighandlerEnabled
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.crashlog_enabled,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"crash-memcheck-enabled\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.memcheck_enabled,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"use-exit-on-panic\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.use_exit_on_panic,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"disable-thp\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.disable_thp,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-allow-replica-migration\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.cluster_allow_replica_migration,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-announced\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.replica_announced,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"latency-tracking\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.latency_tracking_enabled,
                            default_value: 1 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof-disable-auto-gc\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateAofAutoGCEnabled
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.aof_disable_auto_gc,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-ignore-disk-write-errors\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            boolConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            boolConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            boolConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            boolConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: {
                        let mut init = boolConfigData {
                            config: &mut server.repl_ignore_disk_write_error,
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aclfile\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.acl_filename,
                            default_value: b"\0" as *const u8 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"unixsocket\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.unixsocket,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"pidfile\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.pidfile,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-announce-ip\0" as *const u8 as *const libc::c_char,
                alias: b"slave-announce-ip\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.slave_announce_ip,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"masteruser\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 1 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.masteruser,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-announce-ip\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterIp
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.cluster_announce_ip,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-config-file\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.cluster_configfile,
                            default_value: b"nodes.conf\0" as *const u8
                                as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-announce-hostname\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterHostname
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.cluster_announce_hostname,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: Some(
                                isValidAnnouncedHostname
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"syslog-ident\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.syslog_ident,
                            default_value: b"redis\0" as *const u8
                                as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"dbfilename\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.rdb_filename,
                            default_value: b"dump.rdb\0" as *const u8
                                as *const libc::c_char,
                            is_valid_fn: Some(
                                isValidDBfilename
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"appendfilename\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.aof_filename,
                            default_value: b"appendonly.aof\0" as *const u8
                                as *const libc::c_char,
                            is_valid_fn: Some(
                                isValidAOFfilename
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"appenddirname\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.aof_dirname,
                            default_value: b"appendonlydir\0" as *const u8
                                as *const libc::c_char,
                            is_valid_fn: Some(
                                isValidAOFdirname
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"server_cpulist\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.server_cpulist,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"bio_cpulist\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.bio_cpulist,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"aof_rewrite_cpulist\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.aof_rewrite_cpulist,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"bgsave_cpulist\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.bgsave_cpulist,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"ignore-warnings\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.ignore_warnings,
                            default_value: b"\0" as *const u8 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"proc-title-template\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateProcTitleTemplate
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.proc_title_template,
                            default_value: b"{title} {listen-addr} {server-mode}\0"
                                as *const u8 as *const libc::c_char,
                            is_valid_fn: Some(
                                isValidProcTitleTemplate
                                    as unsafe extern "C" fn(
                                        *mut libc::c_char,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"bind-source-addr\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.bind_source_addr,
                            default_value: 0 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"logfile\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            stringConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            stringConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            stringConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            stringConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    string: {
                        let mut init = stringConfigData {
                            config: &mut server.logfile,
                            default_value: b"\0" as *const u8 as *const libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 0 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: STRING_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"masterauth\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 1 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            sdsConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            sdsConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            sdsConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            sdsConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    sds: {
                        let mut init = sdsConfigData {
                            config: &mut server.masterauth,
                            default_value: 0 as *mut libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: SDS_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"requirepass\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 1 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            sdsConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            sdsConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateRequirePass
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            sdsConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            sdsConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    sds: {
                        let mut init = sdsConfigData {
                            config: &mut server.requirepass,
                            default_value: 0 as *mut libc::c_char,
                            is_valid_fn: None,
                            convert_empty_to_null: 1 as libc::c_int,
                        };
                        init
                    },
                },
                type_0: SDS_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"supervised\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.supervised_mode,
                            enum_value: supervised_mode_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"syslog-facility\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.syslog_facility,
                            enum_value: syslog_facility_enum.as_mut_ptr(),
                            default_value: (16 as libc::c_int) << 3 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-diskless-load\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 6 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.repl_diskless_load,
                            enum_value: repl_diskless_load_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"loglevel\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.verbosity,
                            enum_value: loglevel_enum.as_mut_ptr(),
                            default_value: 2 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxmemory-policy\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.maxmemory_policy,
                            enum_value: maxmemory_policy_enum.as_mut_ptr(),
                            default_value: (7 as libc::c_int) << 8 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"appendfsync\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.aof_fsync,
                            enum_value: aof_fsync_enum.as_mut_ptr(),
                            default_value: 2 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"oom-score-adj\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateOOMScoreAdj
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.oom_score_adj,
                            enum_value: oom_score_adj_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"acl-pubsub-default\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.acl_pubsub_default,
                            enum_value: acl_pubsub_default_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"sanitize-dump-payload\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.sanitize_dump_payload,
                            enum_value: sanitize_dump_payload_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"enable-protected-configs\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.enable_protected_configs,
                            enum_value: protected_action_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"enable-debug-command\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.enable_debug_cmd,
                            enum_value: protected_action_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"enable-module-command\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.enable_module_cmd,
                            enum_value: protected_action_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-preferred-endpoint-type\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.cluster_preferred_endpoint_type,
                            enum_value: cluster_preferred_endpoint_type_enum
                                .as_mut_ptr(),
                            default_value: CLUSTER_ENDPOINT_TYPE_IP as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"propagation-error-behavior\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.propagation_error_behavior,
                            enum_value: propagation_error_behavior_enum.as_mut_ptr(),
                            default_value: PROPAGATION_ERR_BEHAVIOR_IGNORE
                                as libc::c_int,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"shutdown-on-sigint\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.shutdown_on_sigint,
                            enum_value: shutdown_on_sig_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: Some(
                                isValidShutdownOnSigFlags
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"shutdown-on-sigterm\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            enumConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            enumConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            enumConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            enumConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    enumd: {
                        let mut init = enumConfigData {
                            config: &mut server.shutdown_on_sigterm,
                            enum_value: shutdown_on_sig_enum.as_mut_ptr(),
                            default_value: 0 as libc::c_int,
                            is_valid_fn: Some(
                                isValidShutdownOnSigFlags
                                    as unsafe extern "C" fn(
                                        libc::c_int,
                                        *mut *const libc::c_char,
                                    ) -> libc::c_int,
                            ),
                        };
                        init
                    },
                },
                type_0: ENUM_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"databases\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.dbnum,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 16 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"port\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updatePort
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 6379 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"io-threads\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | (1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.io_threads_num,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 128 as libc::c_int as libc::c_longlong,
                            default_value: 1 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"auto-aof-rewrite-percentage\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.aof_rewrite_perc,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 100 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-replica-validity-factor\0" as *const u8
                    as *const libc::c_char,
                alias: b"cluster-slave-validity-factor\0" as *const u8
                    as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_slave_validity_factor,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"list-max-listpack-size\0" as *const u8 as *const libc::c_char,
                alias: b"list-max-ziplist-size\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.list_max_listpack_size,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: (-(2147483647 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: -(2 as libc::c_int) as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"tcp-keepalive\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.tcpkeepalive,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 300 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-migration-barrier\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_migration_barrier,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 1 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-cycle-min\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.active_defrag_cycle_min,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 99 as libc::c_int as libc::c_longlong,
                            default_value: 1 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-cycle-max\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.active_defrag_cycle_max,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 99 as libc::c_int as libc::c_longlong,
                            default_value: 25 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-threshold-lower\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.active_defrag_threshold_lower,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 1000 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-threshold-upper\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.active_defrag_threshold_upper,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 1000 as libc::c_int as libc::c_longlong,
                            default_value: 100 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lfu-log-factor\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.lfu_log_factor,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"lfu-decay-time\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.lfu_decay_time,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 1 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-priority\0" as *const u8 as *const libc::c_char,
                alias: b"slave-priority\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.slave_priority,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 100 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-diskless-sync-delay\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_diskless_sync_delay,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 5 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxmemory-samples\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.maxmemory_samples,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 5 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxmemory-eviction-tenacity\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.maxmemory_eviction_tenacity,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 100 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"timeout\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.maxidletime,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replica-announce-port\0" as *const u8 as *const libc::c_char,
                alias: b"slave-announce-port\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.slave_announce_port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"tcp-backlog\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.tcp_backlog,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 511 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-port\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-announce-bus-port\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterAnnouncedPort
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_announce_bus_port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-announce-port\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterAnnouncedPort
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_announce_port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-announce-tls-port\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateClusterAnnouncedPort
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.cluster_announce_tls_port,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 65535 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-timeout\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_timeout,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 60 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-ping-replica-period\0" as *const u8 as *const libc::c_char,
                alias: b"repl-ping-slave-period\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_ping_slave_period,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"list-compress-depth\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.list_compress_depth,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"rdb-key-save-delay\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.rdb_key_save_delay,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: (-(2147483647 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"key-load-delay\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.key_load_delay,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: (-(2147483647 as libc::c_int)
                                - 1 as libc::c_int) as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-expire-effort\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.active_expire_effort,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 10 as libc::c_int as libc::c_longlong,
                            default_value: 1 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"hz\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateHZ
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.config_hz,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"min-replicas-to-write\0" as *const u8 as *const libc::c_char,
                alias: b"min-slaves-to-write\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateGoodSlaves
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_min_slaves_to_write,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"min-replicas-max-lag\0" as *const u8 as *const libc::c_char,
                alias: b"min-slaves-max-lag\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateGoodSlaves
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_min_slaves_max_lag,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"watchdog-period\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateWatchdogPeriod
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.watchdog_period,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"shutdown-timeout\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.shutdown_timeout,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 10 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-diskless-sync-max-replicas\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                i: &mut server.repl_diskless_sync_max_replicas,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_INT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxclients\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateMaxclients
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ui: &mut server.maxclients,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_UINT,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
                            default_value: 10000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"unixsocketperm\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ui: &mut server.unixsocketperm,
                            },
                            flags: ((1 as libc::c_int) << 2 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_UINT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 0o777 as libc::c_int as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"socket-mark-id\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ui: &mut server.socket_mark_id,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_UINT,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: (2147483647 as libc::c_int as libc::c_uint)
                                .wrapping_mul(2 as libc::c_uint)
                                .wrapping_add(1 as libc::c_uint) as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-max-scan-fields\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ul: &mut server.active_defrag_max_scan_fields,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_ULONG,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 1000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"slowlog-max-len\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ul: &mut server.slowlog_max_len,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_ULONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 128 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"acllog-max-len\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ul: &mut server.acllog_max_len,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_ULONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 128 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"busy-reply-threshold\0" as *const u8 as *const libc::c_char,
                alias: b"lua-time-limit\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.busy_reply_threshold,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 5000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-node-timeout\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.cluster_node_timeout,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: 15000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"slowlog-log-slower-than\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.slowlog_log_slower_than,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: -(1 as libc::c_int) as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: 10000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"latency-monitor-threshold\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.latency_monitor_threshold,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"proto-max-bulk-len\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.proto_max_bulk_len,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: (1024 as libc::c_int * 1024 as libc::c_int)
                                as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 512 as libc::c_longlong
                                * 1024 as libc::c_int as libc::c_longlong
                                * 1024 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"stream-node-max-entries\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.stream_node_max_entries,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: 100 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-backlog-size\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateReplBacklogSize
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ll: &mut server.repl_backlog_size,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_LONG_LONG,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: (1024 as libc::c_int * 1024 as libc::c_int)
                                as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxmemory\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateMaxmemory
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ull: &mut server.maxmemory,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_ULONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong) as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"cluster-link-sendbuf-limit\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ull: &mut server.cluster_link_sendbuf_limit_bytes,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_ULONG_LONG,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: (9223372036854775807 as libc::c_longlong
                                as libc::c_ulonglong)
                                .wrapping_mul(2 as libc::c_ulonglong)
                                .wrapping_add(1 as libc::c_ulonglong) as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"hash-max-listpack-entries\0" as *const u8 as *const libc::c_char,
                alias: b"hash-max-ziplist-entries\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.hash_max_listpack_entries,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 512 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"set-max-intset-entries\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.set_max_intset_entries,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 512 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"zset-max-listpack-entries\0" as *const u8 as *const libc::c_char,
                alias: b"zset-max-ziplist-entries\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.zset_max_listpack_entries,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 128 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"active-defrag-ignore-bytes\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.active_defrag_ignore_bytes,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 1 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: ((100 as libc::c_int) << 20 as libc::c_int)
                                as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"hash-max-listpack-value\0" as *const u8 as *const libc::c_char,
                alias: b"hash-max-ziplist-value\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.hash_max_listpack_value,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 64 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"stream-node-max-bytes\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.stream_node_max_bytes,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 4096 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"zset-max-listpack-value\0" as *const u8 as *const libc::c_char,
                alias: b"zset-max-ziplist-value\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.zset_max_listpack_value,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 64 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"hll-sparse-max-bytes\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.hll_sparse_max_bytes,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 3000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"tracking-table-max-keys\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.tracking_table_max_keys,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 1000000 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"client-query-buffer-limit\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 2 as libc::c_int
                    | 0 as libc::c_int as libc::c_ulonglong) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                st: &mut server.client_max_querybuf_len,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SIZE_T,
                            lower_bound: (1024 as libc::c_int * 1024 as libc::c_int)
                                as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: (1024 as libc::c_int * 1024 as libc::c_int
                                * 1024 as libc::c_int) as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"maxmemory-clients\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            applyClientMaxMemoryUsage
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                sst: &mut server.maxmemory_clients,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_SSIZE_T,
                            lower_bound: -(100 as libc::c_int) as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: 0 as libc::c_int as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"repl-backlog-ttl\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                tt: &mut server.repl_backlog_time_limit,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_TIME_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_long
                                as libc::c_longlong,
                            default_value: (60 as libc::c_int * 60 as libc::c_int)
                                as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"auto-aof-rewrite-min-size\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ot: &mut server.aof_rewrite_min_size,
                            },
                            flags: ((1 as libc::c_int) << 0 as libc::c_int)
                                as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_OFF_T,
                            lower_bound: 0 as libc::c_int as libc::c_longlong,
                            upper_bound: 9223372036854775807 as libc::c_longlong,
                            default_value: (64 as libc::c_int * 1024 as libc::c_int
                                * 1024 as libc::c_int) as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"loading-process-events-interval-bytes\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: Some(
                            numericConfigInit
                                as unsafe extern "C" fn(*mut standardConfig) -> (),
                        ),
                        set: Some(
                            numericConfigSet
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            numericConfigGet
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            numericConfigRewrite
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    numeric: {
                        let mut init = numericConfigData {
                            config: C2RustUnnamed_10 {
                                ot: &mut server.loading_process_events_interval_bytes,
                            },
                            flags: 0 as libc::c_int as libc::c_uint,
                            numeric_type: NUMERIC_TYPE_OFF_T,
                            lower_bound: 1024 as libc::c_int as libc::c_longlong,
                            upper_bound: 2147483647 as libc::c_int as libc::c_longlong,
                            default_value: (1024 as libc::c_int * 1024 as libc::c_int
                                * 2 as libc::c_int) as libc::c_longlong,
                            is_valid_fn: None,
                        };
                        init
                    },
                },
                type_0: NUMERIC_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"dir\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 5 as libc::c_int
                    | (1 as libc::c_ulonglong) << 6 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigDirOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigDirOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigDirOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"save\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigSaveOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigSaveOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigSaveOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"client-output-buffer-limit\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigClientOutputBufferLimitOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigClientOutputBufferLimitOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigClientOutputBufferLimitOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"oom-score-adj-values\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigOOMScoreAdjValuesOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            updateOOMScoreAdj
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            getConfigOOMScoreAdjValuesOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigOOMScoreAdjValuesOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"notify-keyspace-events\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0 as libc::c_int as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigNotifyKeyspaceEventsOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigNotifyKeyspaceEventsOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigNotifyKeyspaceEventsOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"bind\0" as *const u8 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigBindOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: Some(
                            applyBind
                                as unsafe extern "C" fn(
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        get: Some(
                            getConfigBindOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigBindOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"replicaof\0" as *const u8 as *const libc::c_char,
                alias: b"slaveof\0" as *const u8 as *const libc::c_char,
                flags: ((1 as libc::c_ulonglong) << 0 as libc::c_int
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigReplicaOfOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigReplicaOfOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigReplicaOfOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: b"latency-tracking-info-percentiles\0" as *const u8
                    as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: (0 as libc::c_int as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint,
                interface: {
                    let mut init = typeInterface {
                        init: None,
                        set: Some(
                            setConfigLatencyTrackingInfoPercentilesOutputOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *mut sds,
                                    libc::c_int,
                                    *mut *const libc::c_char,
                                ) -> libc::c_int,
                        ),
                        apply: None,
                        get: Some(
                            getConfigLatencyTrackingInfoPercentilesOutputOption
                                as unsafe extern "C" fn(*mut standardConfig) -> sds,
                        ),
                        rewrite: Some(
                            rewriteConfigLatencyTrackingInfoPercentilesOutputOption
                                as unsafe extern "C" fn(
                                    *mut standardConfig,
                                    *const libc::c_char,
                                    *mut rewriteConfigState,
                                ) -> (),
                        ),
                    };
                    init
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: SPECIAL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = standardConfig {
                name: 0 as *const libc::c_char,
                alias: 0 as *const libc::c_char,
                flags: 0,
                interface: typeInterface {
                    init: None,
                    set: None,
                    apply: None,
                    get: None,
                    rewrite: None,
                },
                data: typeData {
                    yesno: boolConfigData {
                        config: 0 as *mut libc::c_int,
                        default_value: 0,
                        is_valid_fn: None,
                    },
                },
                type_0: BOOL_CONFIG,
                privdata: 0 as *mut libc::c_void,
            };
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];

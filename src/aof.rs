extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use aof::core::cell::UnsafeCell;
use std::convert::TryInto;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCtx;
    pub type RedisModuleDefragCtx;
    pub type RedisModuleInfoCtx;
    pub type RedisModuleKeyOptCtx;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
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
    static mut SDS_NOINIT: *const libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdsclear(s: sds);
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsneedsrepr(s: sds) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn rioInitWithFile(r: *mut rio, fp: *mut FILE);
    fn rioWriteBulkCount(
        r: *mut rio,
        prefix: libc::c_char,
        count: libc::c_long,
    ) -> size_t;
    fn rioWriteBulkString(r: *mut rio, buf: *const libc::c_char, len: size_t) -> size_t;
    fn rioWriteBulkLongLong(r: *mut rio, l: libc::c_longlong) -> size_t;
    fn rioWriteBulkDouble(r: *mut rio, d: libc::c_double) -> size_t;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn rioSetAutoSync(r: *mut rio, bytes: off_t);
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off64_t, __whence: libc::c_int) -> __off64_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn truncate(__file: *const libc::c_char, __length: __off64_t) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeHead(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listDup(orig: *mut list) -> *mut list;
    fn listIndex(list: *mut list, index: libc::c_long) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn listRewindTail(list: *mut list, li: *mut listIter);
    fn zfree(ptr: *mut libc::c_void);
    fn intsetGet(is: *mut intset, pos: uint32_t, value: *mut int64_t) -> uint8_t;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn pathIsBaseName(path: *mut libc::c_char) -> libc::c_int;
    fn dirCreateIfMissing(dname: *mut libc::c_char) -> libc::c_int;
    fn dirExists(dname: *mut libc::c_char) -> libc::c_int;
    fn fileExist(filename: *mut libc::c_char) -> libc::c_int;
    fn makePath(path: *mut libc::c_char, filename: *mut libc::c_char) -> sds;
    fn fsyncFileDir(filename: *const libc::c_char) -> libc::c_int;
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn quicklistGetIterator(
        quicklist: *mut quicklist,
        direction: libc::c_int,
    ) -> *mut quicklistIter;
    fn quicklistNext(
        iter: *mut quicklistIter,
        entry: *mut quicklistEntry,
    ) -> libc::c_int;
    fn quicklistReleaseIterator(iter: *mut quicklistIter);
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
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpSeek(lp: *mut libc::c_uchar, index: libc::c_long) -> *mut libc::c_uchar;
    fn streamIteratorStart(
        si: *mut streamIterator,
        s: *mut stream,
        start: *mut streamID,
        end: *mut streamID,
        rev: libc::c_int,
    );
    fn streamIteratorGetID(
        si: *mut streamIterator,
        id: *mut streamID,
        numfields: *mut int64_t,
    ) -> libc::c_int;
    fn streamIteratorGetField(
        si: *mut streamIterator,
        fieldptr: *mut *mut libc::c_uchar,
        valueptr: *mut *mut libc::c_uchar,
        fieldlen: *mut int64_t,
        valuelen: *mut int64_t,
    );
    fn streamIteratorStop(si: *mut streamIterator);
    fn streamDecodeID(buf: *mut libc::c_void, id: *mut streamID);
    static mut server: redisServer;
    fn moduleFreeContext(ctx: *mut RedisModuleCtx);
    fn processModuleLoadingProgressEvent(is_aof: libc::c_int);
    fn ustime() -> libc::c_longlong;
    fn mstime() -> libc::c_longlong;
    fn exitFromChild(retcode: libc::c_int);
    fn redisSetProcTitle(title: *mut libc::c_char) -> libc::c_int;
    fn redisSetCpuAffinity(cpulist: *const libc::c_char);
    fn createClient(conn: *mut connection) -> *mut client;
    fn freeClient(c: *mut client);
    fn freeClientArgv(c: *mut client);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn processEventsWhileBlocked();
    fn listTypeLength(subject: *const robj) -> libc::c_ulong;
    fn queueMultiCommand(c: *mut client, cmd_flags: uint64_t);
    fn decrRefCount(o: *mut robj);
    fn dismissObject(o: *mut robj, dump_size: size_t);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn startLoading(size: size_t, rdbflags: libc::c_int, async_0: libc::c_int);
    fn loadingAbsProgress(pos: off_t);
    fn loadingIncrProgress(size: off_t);
    fn stopLoading(success: libc::c_int);
    fn updateLoadingFileName(filename: *mut libc::c_char);
    fn startSaving(rdbflags: libc::c_int);
    fn stopSaving(success: libc::c_int);
    fn rdbLoadRio(
        rdb: *mut rio,
        rdbflags: libc::c_int,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn rdbSaveRio(
        req: libc::c_int,
        rdb: *mut rio,
        error: *mut libc::c_int,
        rdbflags: libc::c_int,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn bg_unlink(filename: *const libc::c_char) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn hasActiveChildProcess() -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn sendChildCowInfo(info_type: childInfoType, pname: *mut libc::c_char);
    fn debugDelay(usec: libc::c_int);
    fn sendChildInfo(info_type: childInfoType, keys: size_t, pname: *mut libc::c_char);
    fn hashTypeReleaseIterator(hi: *mut hashTypeIterator);
    fn hashTypeLength(o: *const robj) -> libc::c_ulong;
    fn hashTypeCurrentFromHashTable(hi: *mut hashTypeIterator, what: libc::c_int) -> sds;
    fn hashTypeCurrentFromListpack(
        hi: *mut hashTypeIterator,
        what: libc::c_int,
        vstr: *mut *mut libc::c_uchar,
        vlen: *mut libc::c_uint,
        vll: *mut libc::c_longlong,
    );
    fn hashTypeNext(hi: *mut hashTypeIterator) -> libc::c_int;
    fn hashTypeInitIterator(subject: *mut robj) -> *mut hashTypeIterator;
    fn zsetLength(zobj: *const robj) -> libc::c_ulong;
    fn zzlNext(
        zl: *mut libc::c_uchar,
        eptr: *mut *mut libc::c_uchar,
        sptr: *mut *mut libc::c_uchar,
    );
    fn zzlGetScore(sptr: *mut libc::c_uchar) -> libc::c_double;
    fn setTypeSize(subject: *const robj) -> libc::c_ulong;
    fn getExpire(db: *mut redisDb, key: *mut robj) -> libc::c_longlong;
    fn redisFork(purpose: libc::c_int) -> libc::c_int;
    fn execCommand(c: *mut client);
    fn multiCommand(c: *mut client);
    fn lookupCommand(argv: *mut *mut robj, argc: libc::c_int) -> *mut redisCommand;
    fn resetChildState();
    fn bioPendingJobsOfType(type_0: libc::c_int) -> libc::c_ulonglong;
    fn bioCreateCloseJob(fd: libc::c_int, need_fsync: libc::c_int);
    fn bioCreateFsyncJob(fd: libc::c_int);
    fn functionsLibGet() -> *mut dict;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
}
pub type __int8_t = libc::c_schar;
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
pub type __useconds_t = libc::c_uint;
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
pub type int8_t = __int8_t;
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
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type robj = redisObject;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intset {
    pub encoding: uint32_t,
    pub length: uint32_t,
    pub contents: [int8_t; 0],
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct quicklistNode {
    pub prev: *mut quicklistNode,
    pub next: *mut quicklistNode,
    pub entry: *mut libc::c_uchar,
    pub sz: size_t,
    #[bitfield(name = "count", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "16..=17")]
    #[bitfield(name = "container", ty = "libc::c_uint", bits = "18..=19")]
    #[bitfield(name = "recompress", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "attempted_compress", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "dont_compress", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "extra", ty = "libc::c_uint", bits = "23..=31")]
    pub count_encoding_container_recompress_attempted_compress_dont_compress_extra: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistBookmark {
    pub node: *mut quicklistNode,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct quicklist {
    pub head: *mut quicklistNode,
    pub tail: *mut quicklistNode,
    pub count: libc::c_ulong,
    pub len: libc::c_ulong,
    #[bitfield(name = "fill", ty = "libc::c_int", bits = "0..=15")]
    #[bitfield(name = "compress", ty = "libc::c_uint", bits = "16..=31")]
    #[bitfield(name = "bookmark_count", ty = "libc::c_uint", bits = "32..=35")]
    pub fill_compress_bookmark_count: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub bookmarks: [quicklistBookmark; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistIter {
    pub quicklist: *mut quicklist,
    pub current: *mut quicklistNode,
    pub zi: *mut libc::c_uchar,
    pub offset: libc::c_long,
    pub direction: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistEntry {
    pub quicklist: *const quicklist,
    pub node: *mut quicklistNode,
    pub zi: *mut libc::c_uchar,
    pub value: *mut libc::c_uchar,
    pub longval: libc::c_longlong,
    pub sz: size_t,
    pub offset: libc::c_int,
}
#[derive(Copy, Clone, c2rust_bitfields::BitfieldStruct)]
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
pub type RedisModuleDefragFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> (),
>;
pub type RedisModuleInfoFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleInfoCtx, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleIO {
    pub bytes: size_t,
    pub rio: *mut rio,
    pub type_0: *mut moduleType,
    pub error: libc::c_int,
    pub ver: libc::c_int,
    pub ctx: *mut RedisModuleCtx,
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleType = RedisModuleType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleType {
    pub id: uint64_t,
    pub module: *mut RedisModule,
    pub rdb_load: moduleTypeLoadFunc,
    pub rdb_save: moduleTypeSaveFunc,
    pub aof_rewrite: moduleTypeRewriteFunc,
    pub mem_usage: moduleTypeMemUsageFunc,
    pub digest: moduleTypeDigestFunc,
    pub free: moduleTypeFreeFunc,
    pub free_effort: moduleTypeFreeEffortFunc,
    pub unlink: moduleTypeUnlinkFunc,
    pub copy: moduleTypeCopyFunc,
    pub defrag: moduleTypeDefragFunc,
    pub aux_load: moduleTypeAuxLoadFunc,
    pub aux_save: moduleTypeAuxSaveFunc,
    pub mem_usage2: moduleTypeMemUsageFunc2,
    pub free_effort2: moduleTypeFreeEffortFunc2,
    pub unlink2: moduleTypeUnlinkFunc2,
    pub copy2: moduleTypeCopyFunc2,
    pub aux_save_triggers: libc::c_int,
    pub name: [libc::c_char; 10],
}
pub type moduleTypeCopyFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type moduleTypeUnlinkFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *const libc::c_void) -> size_t,
>;
pub type moduleTypeMemUsageFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
        size_t,
    ) -> size_t,
>;
pub type moduleTypeAuxSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> (),
>;
pub type moduleTypeAuxLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type moduleTypeDefragFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleDefragCtx,
        *mut redisObject,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type moduleTypeCopyFunc = Option::<
    unsafe extern "C" fn(
        *mut redisObject,
        *mut redisObject,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type moduleTypeUnlinkFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *const libc::c_void) -> size_t,
>;
pub type moduleTypeFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type moduleTypeDigestFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDigest, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDigest {
    pub o: [libc::c_uchar; 20],
    pub x: [libc::c_uchar; 20],
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleTypeMemUsageFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> size_t,
>;
pub type moduleTypeRewriteFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
>;
pub type moduleTypeLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
>;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleValue {
    pub type_0: *mut moduleType,
    pub value: *mut libc::c_void,
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
    pub bs: C2RustUnnamed_8,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub range: C2RustUnnamed_7,
    pub keynum: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
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
pub union C2RustUnnamed_8 {
    pub index: C2RustUnnamed_10,
    pub keyword: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub struct zskiplistNode {
    pub ele: sds,
    pub score: libc::c_double,
    pub backward: *mut zskiplistNode,
    pub level: [zskiplistLevel; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplistLevel {
    pub forward: *mut zskiplistNode,
    pub span: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplist {
    pub header: *mut zskiplistNode,
    pub tail: *mut zskiplistNode,
    pub length: libc::c_ulong,
    pub level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zset {
    pub dict: *mut dict,
    pub zsl: *mut zskiplist,
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
pub type childInfoType = libc::c_uint;
pub const CHILD_INFO_TYPE_MODULE_COW_SIZE: childInfoType = 3;
pub const CHILD_INFO_TYPE_RDB_COW_SIZE: childInfoType = 2;
pub const CHILD_INFO_TYPE_AOF_COW_SIZE: childInfoType = 1;
pub const CHILD_INFO_TYPE_CURRENT_INFO: childInfoType = 0;
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
    pub inst_metric: [C2RustUnnamed_11; 5],
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
pub struct C2RustUnnamed_11 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hashTypeIterator {
    pub subject: *mut robj,
    pub encoding: libc::c_int,
    pub fptr: *mut libc::c_uchar,
    pub vptr: *mut libc::c_uchar,
    pub di: *mut dictIterator,
    pub de: *mut dictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamID {
    pub ms: uint64_t,
    pub seq: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream {
    pub rax: *mut rax,
    pub length: uint64_t,
    pub last_id: streamID,
    pub first_id: streamID,
    pub max_deleted_entry_id: streamID,
    pub entries_added: uint64_t,
    pub cgroups: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamIterator {
    pub stream: *mut stream,
    pub master_id: streamID,
    pub master_fields_count: uint64_t,
    pub master_fields_start: *mut libc::c_uchar,
    pub master_fields_ptr: *mut libc::c_uchar,
    pub entry_flags: libc::c_int,
    pub rev: libc::c_int,
    pub skip_tombstones: libc::c_int,
    pub start_key: [uint64_t; 2],
    pub end_key: [uint64_t; 2],
    pub ri: raxIterator,
    pub lp: *mut libc::c_uchar,
    pub lp_ele: *mut libc::c_uchar,
    pub lp_flags: *mut libc::c_uchar,
    pub field_buf: [libc::c_uchar; 21],
    pub value_buf: [libc::c_uchar; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamCG {
    pub last_id: streamID,
    pub entries_read: libc::c_longlong,
    pub pel: *mut rax,
    pub consumers: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamConsumer {
    pub seen_time: mstime_t,
    pub name: sds,
    pub pel: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamNACK {
    pub delivery_time: mstime_t,
    pub delivery_count: uint64_t,
    pub consumer: *mut streamConsumer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct functionLibInfo {
    pub name: sds,
    pub functions: *mut dict,
    pub ei: *mut engineInfo,
    pub code: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct engineInfo {
    pub name: sds,
    pub engine: *mut engine,
    pub c: *mut client,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct engine {
    pub engine_ctx: *mut libc::c_void,
    pub create: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut functionLibInfo,
            sds,
            *mut sds,
        ) -> libc::c_int,
    >,
    pub call: Option::<
        unsafe extern "C" fn(
            *mut scriptRunCtx,
            *mut libc::c_void,
            *mut libc::c_void,
            *mut *mut robj,
            size_t,
            *mut *mut robj,
            size_t,
        ) -> (),
    >,
    pub get_used_memory: Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>,
    pub get_function_memory_overhead: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    >,
    pub get_engine_memory_overhead: Option::<
        unsafe extern "C" fn(*mut libc::c_void) -> size_t,
    >,
    pub free_function: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scriptRunCtx {
    pub funcname: *const libc::c_char,
    pub c: *mut client,
    pub original_client: *mut client,
    pub flags: libc::c_int,
    pub repl_flags: libc::c_int,
    pub start_time: monotime,
    pub snapshot_time: mstime_t,
}
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
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atoll(mut __nptr: *const libc::c_char) -> libc::c_longlong {
    return strtoll(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(0 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub unsafe extern "C" fn aofInfoCreate() -> *mut aofInfo {
    return zcalloc(core::mem::size_of::<aofInfo>() as libc::c_ulong) as *mut aofInfo;
}
#[no_mangle]
pub unsafe extern "C" fn aofInfoFree(mut ai: *mut aofInfo) {
    if !ai.is_null() {} else {
        _serverAssert(
            b"ai != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
        );
        unreachable!();
    };
    if !((*ai).file_name).is_null() {
        sdsfree((*ai).file_name);
    }
    zfree(ai as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn aofInfoDup(mut orig: *mut aofInfo) -> *mut aofInfo {
    if !orig.is_null() {} else {
        _serverAssert(
            b"orig != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            112 as libc::c_int,
        );
        unreachable!();
    };
    let mut ai: *mut aofInfo = aofInfoCreate();
    (*ai).file_name = sdsdup((*orig).file_name);
    (*ai).file_seq = (*orig).file_seq;
    (*ai).file_type = (*orig).file_type;
    return ai;
}
#[no_mangle]
pub unsafe extern "C" fn aofInfoFormat(mut buf: sds, mut ai: *mut aofInfo) -> sds {
    let mut filename_repr: sds = 0 as sds;
    if sdsneedsrepr((*ai).file_name) != 0 {
        filename_repr = sdscatrepr(
            sdsempty(),
            (*ai).file_name as *const libc::c_char,
            sdslen((*ai).file_name),
        );
    }
    let mut ret: sds = sdscatprintf(
        buf,
        b"%s %s %s %lld %s %c\n\0" as *const u8 as *const libc::c_char,
        b"file\0" as *const u8 as *const libc::c_char,
        if !filename_repr.is_null() { filename_repr } else { (*ai).file_name },
        b"seq\0" as *const u8 as *const libc::c_char,
        (*ai).file_seq,
        b"type\0" as *const u8 as *const libc::c_char,
        (*ai).file_type as libc::c_uint,
    );
    sdsfree(filename_repr);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn aofListFree(mut item: *mut libc::c_void) {
    let mut ai: *mut aofInfo = item as *mut aofInfo;
    aofInfoFree(ai);
}
#[no_mangle]
pub unsafe extern "C" fn aofListDup(mut item: *mut libc::c_void) -> *mut libc::c_void {
    return aofInfoDup(item as *mut aofInfo) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn aofManifestCreate() -> *mut aofManifest {
    let mut am: *mut aofManifest = zcalloc(
        core::mem::size_of::<aofManifest>() as libc::c_ulong,
    ) as *mut aofManifest;
    (*am).incr_aof_list = listCreate();
    (*am).history_aof_list = listCreate();
    (*(*am).incr_aof_list)
        .free = Some(aofListFree as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*am).incr_aof_list)
        .dup = Some(
        aofListDup as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    );
    (*(*am).history_aof_list)
        .free = Some(aofListFree as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*(*am).history_aof_list)
        .dup = Some(
        aofListDup as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
    );
    return am;
}
#[no_mangle]
pub unsafe extern "C" fn aofManifestFree(mut am: *mut aofManifest) {
    if !((*am).base_aof_info).is_null() {
        aofInfoFree((*am).base_aof_info);
    }
    if !((*am).incr_aof_list).is_null() {
        listRelease((*am).incr_aof_list);
    }
    if !((*am).history_aof_list).is_null() {
        listRelease((*am).history_aof_list);
    }
    zfree(am as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn getAofManifestFileName() -> sds {
    return sdscatprintf(
        sdsempty(),
        b"%s%s\0" as *const u8 as *const libc::c_char,
        server.aof_filename,
        b".manifest\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getTempAofManifestFileName() -> sds {
    return sdscatprintf(
        sdsempty(),
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        b"temp-\0" as *const u8 as *const libc::c_char,
        server.aof_filename,
        b".manifest\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getAofManifestAsString(mut am: *mut aofManifest) -> sds {
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
        );
        unreachable!();
    };
    let mut buf: sds = sdsempty();
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    if !((*am).base_aof_info).is_null() {
        buf = aofInfoFormat(buf, (*am).base_aof_info);
    }
    listRewind((*am).history_aof_list, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
        buf = aofInfoFormat(buf, ai);
    }
    listRewind((*am).incr_aof_list, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut ai_0: *mut aofInfo = (*ln).value as *mut aofInfo;
        buf = aofInfoFormat(buf, ai_0);
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn aofLoadManifestFromDisk() {
    server.aof_manifest = aofManifestCreate();
    if dirExists(server.aof_dirname) == 0 {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"The AOF directory %s doesn't exist\0" as *const u8
                    as *const libc::c_char,
                server.aof_dirname,
            );
        }
        return;
    }
    let mut am_name: sds = getAofManifestFileName();
    let mut am_filepath: sds = makePath(server.aof_dirname, am_name);
    if fileExist(am_filepath) == 0 {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"The AOF manifest file %s doesn't exist\0" as *const u8
                    as *const libc::c_char,
                am_name,
            );
        }
        sdsfree(am_name);
        sdsfree(am_filepath);
        return;
    }
    let mut am: *mut aofManifest = aofLoadManifestFromFile(am_filepath);
    if !am.is_null() {
        aofManifestFreeAndUpdate(am);
    }
    sdsfree(am_name);
    sdsfree(am_filepath);
}
#[no_mangle]
pub unsafe extern "C" fn aofLoadManifestFromFile(
    mut am_filepath: sds,
) -> *mut aofManifest {
    let mut current_block: u64;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    let mut maxseq: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut am: *mut aofManifest = aofManifestCreate();
    let mut fp: *mut FILE = fopen(
        am_filepath as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fatal error: can't open the AOF manifest file %s for reading: %s\0"
                    as *const u8 as *const libc::c_char,
                am_filepath,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut buf: [libc::c_char; 1025] = [0; 1025];
    let mut argv: *mut sds = 0 as *mut sds;
    let mut argc: libc::c_int = 0;
    let mut ai: *mut aofInfo = 0 as *mut aofInfo;
    let mut line: sds = 0 as sds;
    let mut linenum: libc::c_int = 0 as libc::c_int;
    's_51: loop {
        if (fgets(buf.as_mut_ptr(), 1024 as libc::c_int + 1 as libc::c_int, fp))
            .is_null()
        {
            if feof(fp) != 0 {
                if !(linenum == 0 as libc::c_int) {
                    current_block = 2520131295878969859;
                    break;
                }
                err = b"Found an empty AOF manifest\0" as *const u8
                    as *const libc::c_char;
                current_block = 10149789293494588487;
                break;
            } else {
                err = b"Read AOF manifest failed\0" as *const u8 as *const libc::c_char;
                current_block = 10149789293494588487;
                break;
            }
        } else {
            linenum += 1;
            if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                continue;
            }
            if (strchr(buf.as_mut_ptr(), '\n' as i32)).is_null() {
                err = b"The AOF manifest file contains too long line\0" as *const u8
                    as *const libc::c_char;
                current_block = 10149789293494588487;
                break;
            } else {
                line = sdstrim(
                    sdsnew(buf.as_mut_ptr()),
                    b" \t\r\n\0" as *const u8 as *const libc::c_char,
                );
                if sdslen(line) == 0 {
                    err = b"Invalid AOF manifest file format\0" as *const u8
                        as *const libc::c_char;
                    current_block = 10149789293494588487;
                    break;
                } else {
                    argv = sdssplitargs(line as *const libc::c_char, &mut argc);
                    if argv.is_null() || argc < 6 as libc::c_int
                        || argc % 2 as libc::c_int != 0
                    {
                        err = b"Invalid AOF manifest file format\0" as *const u8
                            as *const libc::c_char;
                        current_block = 10149789293494588487;
                        break;
                    } else {
                        ai = aofInfoCreate();
                        let mut i: libc::c_int = 0 as libc::c_int;
                        while i < argc {
                            if strcasecmp(
                                *argv.offset(i as isize) as *const libc::c_char,
                                b"file\0" as *const u8 as *const libc::c_char,
                            ) == 0
                            {
                                (*ai)
                                    .file_name = sdsnew(
                                    *argv.offset((i + 1 as libc::c_int) as isize)
                                        as *const libc::c_char,
                                );
                                if pathIsBaseName((*ai).file_name) == 0 {
                                    err = b"File can't be a path, just a filename\0"
                                        as *const u8 as *const libc::c_char;
                                    current_block = 10149789293494588487;
                                    break 's_51;
                                }
                            } else if strcasecmp(
                                *argv.offset(i as isize) as *const libc::c_char,
                                b"seq\0" as *const u8 as *const libc::c_char,
                            ) == 0
                            {
                                (*ai)
                                    .file_seq = atoll(
                                    *argv.offset((i + 1 as libc::c_int) as isize)
                                        as *const libc::c_char,
                                );
                            } else if strcasecmp(
                                *argv.offset(i as isize) as *const libc::c_char,
                                b"type\0" as *const u8 as *const libc::c_char,
                            ) == 0
                            {
                                (*ai)
                                    .file_type = *(*argv
                                    .offset((i + 1 as libc::c_int) as isize))
                                    .offset(0 as libc::c_int as isize) as aof_file_type;
                            }
                            i += 2 as libc::c_int;
                        }
                        if ((*ai).file_name).is_null() || (*ai).file_seq == 0
                            || (*ai).file_type as u64 == 0
                        {
                            err = b"Invalid AOF manifest file format\0" as *const u8
                                as *const libc::c_char;
                            current_block = 10149789293494588487;
                            break;
                        } else {
                            sdsfreesplitres(argv, argc);
                            argv = 0 as *mut sds;
                            if (*ai).file_type as libc::c_uint
                                == AOF_FILE_TYPE_BASE as libc::c_int as libc::c_uint
                            {
                                if !((*am).base_aof_info).is_null() {
                                    err = b"Found duplicate base file information\0"
                                        as *const u8 as *const libc::c_char;
                                    current_block = 10149789293494588487;
                                    break;
                                } else {
                                    (*am).base_aof_info = ai;
                                    (*am).curr_base_file_seq = (*ai).file_seq;
                                }
                            } else if (*ai).file_type as libc::c_uint
                                == AOF_FILE_TYPE_HIST as libc::c_int as libc::c_uint
                            {
                                listAddNodeTail(
                                    (*am).history_aof_list,
                                    ai as *mut libc::c_void,
                                );
                            } else if (*ai).file_type as libc::c_uint
                                == AOF_FILE_TYPE_INCR as libc::c_int as libc::c_uint
                            {
                                if (*ai).file_seq <= maxseq {
                                    err = b"Found a non-monotonic sequence number\0"
                                        as *const u8 as *const libc::c_char;
                                    current_block = 10149789293494588487;
                                    break;
                                } else {
                                    listAddNodeTail(
                                        (*am).incr_aof_list,
                                        ai as *mut libc::c_void,
                                    );
                                    (*am).curr_incr_file_seq = (*ai).file_seq;
                                    maxseq = (*ai).file_seq;
                                }
                            } else {
                                err = b"Unknown AOF file type\0" as *const u8
                                    as *const libc::c_char;
                                current_block = 10149789293494588487;
                                break;
                            }
                            sdsfree(line);
                            line = 0 as sds;
                            ai = 0 as *mut aofInfo;
                        }
                    }
                }
            }
        }
    }
    match current_block {
        10149789293494588487 => {
            if !argv.is_null() {
                sdsfreesplitres(argv, argc);
            }
            if !ai.is_null() {
                aofInfoFree(ai);
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"\n*** FATAL AOF MANIFEST FILE ERROR ***\n\0" as *const u8
                        as *const libc::c_char,
                );
            }
            if !line.is_null() {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Reading the manifest file, at line %d\n\0" as *const u8
                            as *const libc::c_char,
                        linenum,
                    );
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b">>> '%s'\n\0" as *const u8 as *const libc::c_char,
                        line,
                    );
                }
            }
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"%s\n\0" as *const u8 as *const libc::c_char,
                    err,
                );
            }
            exit(1 as libc::c_int);
        }
        _ => {
            fclose(fp);
            return am;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn aofManifestDup(mut orig: *mut aofManifest) -> *mut aofManifest {
    if !orig.is_null() {} else {
        _serverAssert(
            b"orig != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            392 as libc::c_int,
        );
        unreachable!();
    };
    let mut am: *mut aofManifest = zcalloc(
        core::mem::size_of::<aofManifest>() as libc::c_ulong,
    ) as *mut aofManifest;
    (*am).curr_base_file_seq = (*orig).curr_base_file_seq;
    (*am).curr_incr_file_seq = (*orig).curr_incr_file_seq;
    (*am).dirty = (*orig).dirty;
    if !((*orig).base_aof_info).is_null() {
        (*am).base_aof_info = aofInfoDup((*orig).base_aof_info);
    }
    (*am).incr_aof_list = listDup((*orig).incr_aof_list);
    (*am).history_aof_list = listDup((*orig).history_aof_list);
    if !((*am).incr_aof_list).is_null() {} else {
        _serverAssert(
            b"am->incr_aof_list != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            405 as libc::c_int,
        );
        unreachable!();
    };
    if !((*am).history_aof_list).is_null() {} else {
        _serverAssert(
            b"am->history_aof_list != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int,
        );
        unreachable!();
    };
    return am;
}
#[no_mangle]
pub unsafe extern "C" fn aofManifestFreeAndUpdate(mut am: *mut aofManifest) {
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            413 as libc::c_int,
        );
        unreachable!();
    };
    if !(server.aof_manifest).is_null() {
        aofManifestFree(server.aof_manifest);
    }
    server.aof_manifest = am;
}
#[no_mangle]
pub unsafe extern "C" fn getNewBaseFileNameAndMarkPreAsHistory(
    mut am: *mut aofManifest,
) -> sds {
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            428 as libc::c_int,
        );
        unreachable!();
    };
    if !((*am).base_aof_info).is_null() {
        if (*(*am).base_aof_info).file_type as libc::c_uint
            == AOF_FILE_TYPE_BASE as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"am->base_aof_info->file_type == AOF_FILE_TYPE_BASE\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                430 as libc::c_int,
            );
            unreachable!();
        };
        (*(*am).base_aof_info).file_type = AOF_FILE_TYPE_HIST;
        listAddNodeHead(
            (*am).history_aof_list,
            (*am).base_aof_info as *mut libc::c_void,
        );
    }
    let mut format_suffix: *mut libc::c_char = (if server.aof_use_rdb_preamble != 0 {
        b".rdb\0" as *const u8 as *const libc::c_char
    } else {
        b".aof\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut ai: *mut aofInfo = aofInfoCreate();
    (*am).curr_base_file_seq += 1;
    (*ai)
        .file_name = sdscatprintf(
        sdsempty(),
        b"%s.%lld%s%s\0" as *const u8 as *const libc::c_char,
        server.aof_filename,
        (*am).curr_base_file_seq,
        b".base\0" as *const u8 as *const libc::c_char,
        format_suffix,
    );
    (*ai).file_seq = (*am).curr_base_file_seq;
    (*ai).file_type = AOF_FILE_TYPE_BASE;
    (*am).base_aof_info = ai;
    (*am).dirty = 1 as libc::c_int;
    return (*(*am).base_aof_info).file_name;
}
#[no_mangle]
pub unsafe extern "C" fn getNewIncrAofName(mut am: *mut aofManifest) -> sds {
    let mut ai: *mut aofInfo = aofInfoCreate();
    (*ai).file_type = AOF_FILE_TYPE_INCR;
    (*am).curr_incr_file_seq += 1;
    (*ai)
        .file_name = sdscatprintf(
        sdsempty(),
        b"%s.%lld%s%s\0" as *const u8 as *const libc::c_char,
        server.aof_filename,
        (*am).curr_incr_file_seq,
        b".incr\0" as *const u8 as *const libc::c_char,
        b".aof\0" as *const u8 as *const libc::c_char,
    );
    (*ai).file_seq = (*am).curr_incr_file_seq;
    listAddNodeTail((*am).incr_aof_list, ai as *mut libc::c_void);
    (*am).dirty = 1 as libc::c_int;
    return (*ai).file_name;
}
#[no_mangle]
pub unsafe extern "C" fn getTempIncrAofName() -> sds {
    return sdscatprintf(
        sdsempty(),
        b"%s%s%s\0" as *const u8 as *const libc::c_char,
        b"temp-\0" as *const u8 as *const libc::c_char,
        server.aof_filename,
        b".incr\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn getLastIncrAofName(mut am: *mut aofManifest) -> sds {
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            474 as libc::c_int,
        );
        unreachable!();
    };
    if (*(*am).incr_aof_list).len == 0 {
        return getNewIncrAofName(am);
    }
    let mut lastnode: *mut listNode = listIndex(
        (*am).incr_aof_list,
        -(1 as libc::c_int) as libc::c_long,
    );
    let mut ai: *mut aofInfo = (*lastnode).value as *mut aofInfo;
    return (*ai).file_name;
}
#[no_mangle]
pub unsafe extern "C" fn markRewrittenIncrAofAsHistory(mut am: *mut aofManifest) {
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            493 as libc::c_int,
        );
        unreachable!();
    };
    if (*(*am).incr_aof_list).len == 0 {
        return;
    }
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewindTail((*am).incr_aof_list, &mut li);
    if server.aof_fd != -(1 as libc::c_int) {
        ln = listNext(&mut li);
        if !ln.is_null() {} else {
            _serverAssert(
                b"ln != NULL\0" as *const u8 as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                507 as libc::c_int,
            );
            unreachable!();
        };
    }
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
        if (*ai).file_type as libc::c_uint
            == AOF_FILE_TYPE_INCR as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"ai->file_type == AOF_FILE_TYPE_INCR\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                513 as libc::c_int,
            );
            unreachable!();
        };
        let mut hai: *mut aofInfo = aofInfoDup(ai);
        (*hai).file_type = AOF_FILE_TYPE_HIST;
        listAddNodeHead((*am).history_aof_list, hai as *mut libc::c_void);
        listDelNode((*am).incr_aof_list, ln);
    }
    (*am).dirty = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn writeAofManifestFile(mut buf: sds) -> libc::c_int {
    let mut current_block: u64;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut nwritten: ssize_t = 0;
    let mut len: libc::c_int = 0;
    let mut am_name: sds = getAofManifestFileName();
    let mut am_filepath: sds = makePath(server.aof_dirname, am_name);
    let mut tmp_am_name: sds = getTempAofManifestFileName();
    let mut tmp_am_filepath: sds = makePath(server.aof_dirname, tmp_am_name);
    let mut fd: libc::c_int = open(
        tmp_am_filepath as *const libc::c_char,
        0o1 as libc::c_int | 0o1000 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open the AOF manifest file %s: %s\0" as *const u8
                    as *const libc::c_char,
                tmp_am_name,
                strerror(*__errno_location()),
            );
        }
        ret = -(1 as libc::c_int);
    } else {
        len = sdslen(buf) as libc::c_int;
        loop {
            if !(len != 0) {
                current_block = 6057473163062296781;
                break;
            }
            nwritten = write(fd, buf as *const libc::c_void, len as size_t);
            if nwritten < 0 as libc::c_int as libc::c_long {
                if *__errno_location() == 4 as libc::c_int {
                    continue;
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Error trying to write the temporary AOF manifest file %s: %s\0"
                            as *const u8 as *const libc::c_char,
                        tmp_am_name,
                        strerror(*__errno_location()),
                    );
                }
                ret = -(1 as libc::c_int);
                current_block = 3442109089460671254;
                break;
            } else {
                len = (len as libc::c_long - nwritten) as libc::c_int;
                buf = buf.offset(nwritten as isize);
            }
        }
        match current_block {
            3442109089460671254 => {}
            _ => {
                if fdatasync(fd) == -(1 as libc::c_int) {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Fail to fsync the temp AOF file %s: %s.\0" as *const u8
                                as *const libc::c_char,
                            tmp_am_name,
                            strerror(*__errno_location()),
                        );
                    }
                    ret = -(1 as libc::c_int);
                } else if rename(
                    tmp_am_filepath as *const libc::c_char,
                    am_filepath as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error trying to rename the temporary AOF manifest file %s into %s: %s\0"
                                as *const u8 as *const libc::c_char,
                            tmp_am_name,
                            am_name,
                            strerror(*__errno_location()),
                        );
                    }
                    ret = -(1 as libc::c_int);
                } else if fsyncFileDir(am_filepath as *const libc::c_char)
                    == -(1 as libc::c_int)
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Fail to fsync AOF directory %s: %s.\0" as *const u8
                                as *const libc::c_char,
                            am_filepath,
                            strerror(*__errno_location()),
                        );
                    }
                    ret = -(1 as libc::c_int);
                }
            }
        }
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    sdsfree(am_name);
    sdsfree(am_filepath);
    sdsfree(tmp_am_name);
    sdsfree(tmp_am_filepath);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn persistAofManifest(mut am: *mut aofManifest) -> libc::c_int {
    if (*am).dirty == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut amstr: sds = getAofManifestAsString(am);
    let mut ret: libc::c_int = writeAofManifestFile(amstr);
    sdsfree(amstr);
    if ret == 0 as libc::c_int {
        (*am).dirty = 0 as libc::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn aofUpgradePrepare(mut am: *mut aofManifest) {
    if aofFileExist(server.aof_filename) == 0 {} else {
        _serverAssert(
            b"!aofFileExist(server.aof_filename)\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            621 as libc::c_int,
        );
        unreachable!();
    };
    if dirCreateIfMissing(server.aof_dirname) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open or create append-only dir %s: %s\0" as *const u8
                    as *const libc::c_char,
                server.aof_dirname,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    if !((*am).base_aof_info).is_null() {
        aofInfoFree((*am).base_aof_info);
    }
    let mut ai: *mut aofInfo = aofInfoCreate();
    (*ai).file_name = sdsnew(server.aof_filename);
    (*ai).file_seq = 1 as libc::c_int as libc::c_longlong;
    (*ai).file_type = AOF_FILE_TYPE_BASE;
    (*am).base_aof_info = ai;
    (*am).curr_base_file_seq = 1 as libc::c_int as libc::c_longlong;
    (*am).dirty = 1 as libc::c_int;
    if persistAofManifest(am) != 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    let mut aof_filepath: sds = makePath(server.aof_dirname, server.aof_filename);
    if rename(server.aof_filename, aof_filepath as *const libc::c_char)
        == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error trying to move the old AOF file %s into dir %s: %s\0"
                    as *const u8 as *const libc::c_char,
                server.aof_filename,
                server.aof_dirname,
                strerror(*__errno_location()),
            );
        }
        sdsfree(aof_filepath);
        exit(1 as libc::c_int);
    }
    sdsfree(aof_filepath);
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Successfully migrated an old-style AOF file (%s) into the AOF directory (%s).\0"
                as *const u8 as *const libc::c_char,
            server.aof_filename,
            server.aof_dirname,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn aofDelHistoryFiles() -> libc::c_int {
    if (server.aof_manifest).is_null() || server.aof_disable_auto_gc == 1 as libc::c_int
        || (*(*server.aof_manifest).history_aof_list).len == 0
    {
        return 0 as libc::c_int;
    }
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    listRewind((*server.aof_manifest).history_aof_list, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
        if (*ai).file_type as libc::c_uint
            == AOF_FILE_TYPE_HIST as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"ai->file_type == AOF_FILE_TYPE_HIST\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                682 as libc::c_int,
            );
            unreachable!();
        };
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Removing the history file %s in the background\0" as *const u8
                    as *const libc::c_char,
                (*ai).file_name,
            );
        }
        let mut aof_filepath: sds = makePath(server.aof_dirname, (*ai).file_name);
        bg_unlink(aof_filepath as *const libc::c_char);
        sdsfree(aof_filepath);
        listDelNode((*server.aof_manifest).history_aof_list, ln);
    }
    (*server.aof_manifest).dirty = 1 as libc::c_int;
    return persistAofManifest(server.aof_manifest);
}
#[no_mangle]
pub unsafe extern "C" fn aofDelTempIncrAofFile() {
    let mut aof_filename: sds = getTempIncrAofName();
    let mut aof_filepath: sds = makePath(server.aof_dirname, aof_filename);
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Removing the temp incr aof file %s in the background\0" as *const u8
                as *const libc::c_char,
            aof_filename,
        );
    }
    bg_unlink(aof_filepath as *const libc::c_char);
    sdsfree(aof_filepath);
    sdsfree(aof_filename);
}
#[no_mangle]
pub unsafe extern "C" fn aofOpenIfNeededOnServerStart() {
    if server.aof_state != 1 as libc::c_int {
        return;
    }
    if !(server.aof_manifest).is_null() {} else {
        _serverAssert(
            b"server.aof_manifest != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            718 as libc::c_int,
        );
        unreachable!();
    };
    if server.aof_fd == -(1 as libc::c_int) {} else {
        _serverAssert(
            b"server.aof_fd == -1\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            719 as libc::c_int,
        );
        unreachable!();
    };
    if dirCreateIfMissing(server.aof_dirname) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open or create append-only dir %s: %s\0" as *const u8
                    as *const libc::c_char,
                server.aof_dirname,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut incr_aof_len: size_t = (*(*server.aof_manifest).incr_aof_list).len;
    if ((*server.aof_manifest).base_aof_info).is_null() && incr_aof_len == 0 {
        let mut base_name: sds = getNewBaseFileNameAndMarkPreAsHistory(
            server.aof_manifest,
        );
        let mut base_filepath: sds = makePath(server.aof_dirname, base_name);
        if rewriteAppendOnlyFile(base_filepath) != 0 as libc::c_int {
            exit(1 as libc::c_int);
        }
        sdsfree(base_filepath);
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Creating AOF base file %s on server start\0" as *const u8
                    as *const libc::c_char,
                base_name,
            );
        }
    }
    let mut aof_name: sds = getLastIncrAofName(server.aof_manifest);
    let mut aof_filepath: sds = makePath(server.aof_dirname, aof_name);
    server
        .aof_fd = open(
        aof_filepath as *const libc::c_char,
        0o1 as libc::c_int | 0o2000 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    sdsfree(aof_filepath);
    if server.aof_fd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open the append-only file %s: %s\0" as *const u8
                    as *const libc::c_char,
                aof_name,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    let mut ret: libc::c_int = persistAofManifest(server.aof_manifest);
    if ret != 0 as libc::c_int {
        exit(1 as libc::c_int);
    }
    server.aof_last_incr_size = getAppendOnlyFileSize(aof_name, 0 as *mut libc::c_int);
    if incr_aof_len != 0 {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Opening AOF incr file %s on server start\0" as *const u8
                    as *const libc::c_char,
                aof_name,
            );
        }
    } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Creating AOF incr file %s on server start\0" as *const u8
                as *const libc::c_char,
            aof_name,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn aofFileExist(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut file_path: sds = makePath(server.aof_dirname, filename);
    let mut ret: libc::c_int = fileExist(file_path);
    sdsfree(file_path);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn openNewIncrAofForAppend() -> libc::c_int {
    let mut current_block: u64;
    if !(server.aof_manifest).is_null() {} else {
        _serverAssert(
            b"server.aof_manifest != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            790 as libc::c_int,
        );
        unreachable!();
    };
    let mut newfd: libc::c_int = -(1 as libc::c_int);
    let mut temp_am: *mut aofManifest = 0 as *mut aofManifest;
    let mut new_aof_name: sds = 0 as sds;
    if server.aof_state == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if server.aof_state == 2 as libc::c_int {
        new_aof_name = getTempIncrAofName();
    } else {
        temp_am = aofManifestDup(server.aof_manifest);
        new_aof_name = sdsdup(getNewIncrAofName(temp_am));
    }
    let mut new_aof_filepath: sds = makePath(server.aof_dirname, new_aof_name);
    newfd = open(
        new_aof_filepath as *const libc::c_char,
        0o1 as libc::c_int | 0o1000 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    sdsfree(new_aof_filepath);
    if newfd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open the append-only file %s: %s\0" as *const u8
                    as *const libc::c_char,
                new_aof_name,
                strerror(*__errno_location()),
            );
        }
    } else {
        if !temp_am.is_null() {
            if persistAofManifest(temp_am) == -(1 as libc::c_int) {
                current_block = 9561948109637317291;
            } else {
                current_block = 5143058163439228106;
            }
        } else {
            current_block = 5143058163439228106;
        }
        match current_block {
            9561948109637317291 => {}
            _ => {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Creating AOF incr file %s on background rewrite\0" as *const u8
                            as *const libc::c_char,
                        new_aof_name,
                    );
                }
                sdsfree(new_aof_name);
                if server.aof_fd != -(1 as libc::c_int) {
                    aof_background_fsync_and_close(server.aof_fd);
                    server.aof_fsync_offset = server.aof_current_size;
                    server.aof_last_fsync = server.unixtime as time_t;
                }
                server.aof_fd = newfd;
                server.aof_last_incr_size = 0 as libc::c_int as off_t;
                if !temp_am.is_null() {
                    aofManifestFreeAndUpdate(temp_am);
                }
                return 0 as libc::c_int;
            }
        }
    }
    if !new_aof_name.is_null() {
        sdsfree(new_aof_name);
    }
    if newfd != -(1 as libc::c_int) {
        close(newfd);
    }
    if !temp_am.is_null() {
        aofManifestFree(temp_am);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn aofRewriteLimited() -> libc::c_int {
    static mut next_delay_minutes: libc::c_int = 0 as libc::c_int;
    static mut next_rewrite_time: time_t = 0 as libc::c_int as time_t;
    if server.stat_aofrw_consecutive_failures < 3 as libc::c_int as libc::c_longlong {
        next_delay_minutes = 0 as libc::c_int;
        next_rewrite_time = 0 as libc::c_int as time_t;
        return 0 as libc::c_int;
    }
    if next_rewrite_time != 0 as libc::c_int as libc::c_long {
        if (server.unixtime as libc::c_long) < next_rewrite_time {
            return 1 as libc::c_int
        } else {
            next_rewrite_time = 0 as libc::c_int as time_t;
            return 0 as libc::c_int;
        }
    }
    next_delay_minutes = if next_delay_minutes == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        next_delay_minutes * 2 as libc::c_int
    };
    if next_delay_minutes > 60 as libc::c_int {
        next_delay_minutes = 60 as libc::c_int;
    }
    next_rewrite_time = (server.unixtime + next_delay_minutes * 60 as libc::c_int)
        as time_t;
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Background AOF rewrite has repeatedly failed and triggered the limit, will retry in %d minutes\0"
                as *const u8 as *const libc::c_char,
            next_delay_minutes,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aofFsyncInProgress() -> libc::c_int {
    return (bioPendingJobsOfType(1 as libc::c_int)
        != 0 as libc::c_int as libc::c_ulonglong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aof_background_fsync(mut fd: libc::c_int) {
    bioCreateFsyncJob(fd);
}
#[no_mangle]
pub unsafe extern "C" fn aof_background_fsync_and_close(mut fd: libc::c_int) {
    bioCreateCloseJob(fd, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn killAppendOnlyChild() {
    let mut statloc: libc::c_int = 0;
    if server.child_type != 2 as libc::c_int {
        return;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Killing running AOF rewrite child: %ld\0" as *const u8
                as *const libc::c_char,
            server.child_pid as libc::c_long,
        );
    }
    if kill(server.child_pid, 10 as libc::c_int) != -(1 as libc::c_int) {
        while waitpid(-(1 as libc::c_int), &mut statloc, 0 as libc::c_int)
            != server.child_pid
        {}
    }
    aofRemoveTempFile(server.child_pid);
    resetChildState();
    server.aof_rewrite_time_start = -(1 as libc::c_int) as time_t;
}
#[no_mangle]
pub unsafe extern "C" fn stopAppendOnly() {
    if server.aof_state != 0 as libc::c_int {} else {
        _serverAssert(
            b"server.aof_state != AOF_OFF\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            950 as libc::c_int,
        );
        unreachable!();
    };
    flushAppendOnlyFile(1 as libc::c_int);
    if fdatasync(server.aof_fd) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fail to fsync the AOF file: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else {
        server.aof_fsync_offset = server.aof_current_size;
        server.aof_last_fsync = server.unixtime as time_t;
    }
    close(server.aof_fd);
    server.aof_fd = -(1 as libc::c_int);
    server.aof_selected_db = -(1 as libc::c_int);
    server.aof_state = 0 as libc::c_int;
    server.aof_rewrite_scheduled = 0 as libc::c_int;
    server.aof_last_incr_size = 0 as libc::c_int as off_t;
    killAppendOnlyChild();
    sdsfree(server.aof_buf);
    server.aof_buf = sdsempty();
}
#[no_mangle]
pub unsafe extern "C" fn startAppendOnly() -> libc::c_int {
    if server.aof_state == 0 as libc::c_int {} else {
        _serverAssert(
            b"server.aof_state == AOF_OFF\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            973 as libc::c_int,
        );
        unreachable!();
    };
    server.aof_state = 2 as libc::c_int;
    if hasActiveChildProcess() != 0 && server.child_type != 2 as libc::c_int {
        server.aof_rewrite_scheduled = 1 as libc::c_int;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"AOF was enabled but there is already another background operation. An AOF background was scheduled to start when possible.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if server.in_exec != 0 {
        server.aof_rewrite_scheduled = 1 as libc::c_int;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"AOF was enabled during a transaction. An AOF background was scheduled to start when possible.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else {
        if server.child_type == 2 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"AOF was enabled but there is already an AOF rewriting in background. Stopping background AOF and starting a rewrite now.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            killAppendOnlyChild();
        }
        if rewriteAppendOnlyFileBackground() == -(1 as libc::c_int) {
            server.aof_state = 0 as libc::c_int;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Redis needs to enable the AOF but can't trigger a background AOF rewrite operation. Check the above logs for more info about the error.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            return -(1 as libc::c_int);
        }
    }
    server.aof_last_fsync = server.unixtime as time_t;
    let mut aof_bio_fsync_status: libc::c_int = 0;
    aof_bio_fsync_status = core::intrinsics::atomic_load_relaxed(
        &mut server.aof_bio_fsync_status,
    );
    if aof_bio_fsync_status == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"AOF reopen, just ignore the AOF fsync error in bio job\0" as *const u8
                    as *const libc::c_char,
            );
        }
        core::intrinsics::atomic_store_relaxed(
            &mut server.aof_bio_fsync_status,
            0 as libc::c_int,
        );
    }
    if server.aof_last_write_status == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"AOF reopen, just ignore the last error.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        server.aof_last_write_status = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aofWrite(
    mut fd: libc::c_int,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> ssize_t {
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    let mut totwritten: ssize_t = 0 as libc::c_int as ssize_t;
    while len != 0 {
        nwritten = write(fd, buf as *const libc::c_void, len);
        if nwritten < 0 as libc::c_int as libc::c_long {
            if *__errno_location() == 4 as libc::c_int {
                continue;
            }
            return if totwritten != 0 {
                totwritten
            } else {
                -(1 as libc::c_int) as libc::c_long
            };
        } else {
            len = (len as libc::c_ulong).wrapping_sub(nwritten as libc::c_ulong)
                as size_t as size_t;
            buf = buf.offset(nwritten as isize);
            totwritten += nwritten;
        }
    }
    return totwritten;
}
#[no_mangle]
pub unsafe extern "C" fn flushAppendOnlyFile(mut force: libc::c_int) {
    let mut nwritten: ssize_t = 0;
    let mut sync_in_progress: libc::c_int = 0 as libc::c_int;
    let mut latency: mstime_t = 0;
    if sdslen(server.aof_buf) == 0 as libc::c_int as libc::c_ulong {
        if !(server.aof_fsync == 2 as libc::c_int
            && server.aof_fsync_offset != server.aof_current_size
            && server.unixtime as libc::c_long > server.aof_last_fsync
            && {
                sync_in_progress = aofFsyncInProgress();
                sync_in_progress == 0
            })
        {
            return;
        }
    } else {
        if server.aof_fsync == 2 as libc::c_int {
            sync_in_progress = aofFsyncInProgress();
        }
        if server.aof_fsync == 2 as libc::c_int && force == 0 {
            if sync_in_progress != 0 {
                if server.aof_flush_postponed_start == 0 as libc::c_int as libc::c_long {
                    server.aof_flush_postponed_start = server.unixtime as time_t;
                    return;
                } else {
                    if server.unixtime as libc::c_long - server.aof_flush_postponed_start
                        < 2 as libc::c_int as libc::c_long
                    {
                        return;
                    }
                }
                server.aof_delayed_fsync = (server.aof_delayed_fsync).wrapping_add(1);
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Asynchronous AOF fsync is taking too long (disk is busy?). Writing the AOF buffer without waiting for fsync to complete, this may slow down Redis.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        if server.aof_flush_sleep != 0 && sdslen(server.aof_buf) != 0 {
            usleep(server.aof_flush_sleep as __useconds_t);
        }
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        nwritten = aofWrite(
            server.aof_fd,
            server.aof_buf as *const libc::c_char,
            sdslen(server.aof_buf),
        );
        if server.latency_monitor_threshold != 0 {
            latency = mstime() - latency;
        }
        if sync_in_progress != 0 {
            if server.latency_monitor_threshold != 0
                && latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"aof-write-pending-fsync\0" as *const u8 as *const libc::c_char,
                    latency,
                );
            }
        } else if hasActiveChildProcess() != 0 {
            if server.latency_monitor_threshold != 0
                && latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"aof-write-active-child\0" as *const u8 as *const libc::c_char,
                    latency,
                );
            }
        } else if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"aof-write-alone\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
        if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"aof-write\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
        server.aof_flush_postponed_start = 0 as libc::c_int as time_t;
        if nwritten != sdslen(server.aof_buf) as ssize_t {
            static mut last_write_error_log: time_t = 0 as libc::c_int as time_t;
            let mut can_log: libc::c_int = 0 as libc::c_int;
            if server.unixtime as libc::c_long - last_write_error_log
                > 30 as libc::c_int as libc::c_long
            {
                can_log = 1 as libc::c_int;
                last_write_error_log = server.unixtime as time_t;
            }
            if nwritten == -(1 as libc::c_int) as libc::c_long {
                if can_log != 0 {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error writing to the AOF file: %s\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                    }
                }
                server.aof_last_write_errno = *__errno_location();
            } else {
                if can_log != 0 {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Short write while writing to the AOF file: (nwritten=%lld, expected=%lld)\0"
                                as *const u8 as *const libc::c_char,
                            nwritten as libc::c_longlong,
                            sdslen(server.aof_buf) as libc::c_longlong,
                        );
                    }
                }
                if ftruncate(server.aof_fd, server.aof_last_incr_size)
                    == -(1 as libc::c_int)
                {
                    if can_log != 0 {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Could not remove short write from the append-only file.  Redis may refuse to load the AOF the next time it starts.  ftruncate: %s\0"
                                    as *const u8 as *const libc::c_char,
                                strerror(*__errno_location()),
                            );
                        }
                    }
                } else {
                    nwritten = -(1 as libc::c_int) as ssize_t;
                }
                server.aof_last_write_errno = 28 as libc::c_int;
            }
            if server.aof_fsync == 1 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Can't recover from AOF write error when the AOF fsync policy is 'always'. Exiting...\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            } else {
                server.aof_last_write_status = -(1 as libc::c_int);
                if nwritten > 0 as libc::c_int as libc::c_long {
                    server.aof_current_size += nwritten;
                    server.aof_last_incr_size += nwritten;
                    sdsrange(server.aof_buf, nwritten, -(1 as libc::c_int) as ssize_t);
                }
                return;
            }
        } else {
            if server.aof_last_write_status == -(1 as libc::c_int) {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"AOF write error looks solved, Redis can write again.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                server.aof_last_write_status = 0 as libc::c_int;
            }
        }
        server.aof_current_size += nwritten;
        server.aof_last_incr_size += nwritten;
        if (sdslen(server.aof_buf)).wrapping_add(sdsavail(server.aof_buf))
            < 4000 as libc::c_int as libc::c_ulong
        {
            sdsclear(server.aof_buf);
        } else {
            sdsfree(server.aof_buf);
            server.aof_buf = sdsempty();
        }
    }
    if server.aof_no_fsync_on_rewrite != 0 && hasActiveChildProcess() != 0 {
        return;
    }
    if server.aof_fsync == 1 as libc::c_int {
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        if fdatasync(server.aof_fd) == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't persist AOF for fsync error when the AOF fsync policy is 'always': %s. Exiting...\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            exit(1 as libc::c_int);
        }
        if server.latency_monitor_threshold != 0 {
            latency = mstime() - latency;
        }
        if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"aof-fsync-always\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
        server.aof_fsync_offset = server.aof_current_size;
        server.aof_last_fsync = server.unixtime as time_t;
    } else if server.aof_fsync == 2 as libc::c_int
        && server.unixtime as libc::c_long > server.aof_last_fsync
    {
        if sync_in_progress == 0 {
            aof_background_fsync(server.aof_fd);
            server.aof_fsync_offset = server.aof_current_size;
        }
        server.aof_last_fsync = server.unixtime as time_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn catAppendOnlyGenericCommand(
    mut dst: sds,
    mut argc: libc::c_int,
    mut argv: *mut *mut robj,
) -> sds {
    let mut buf: [libc::c_char; 32] = [0; 32];
    let mut len: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut o: *mut robj = 0 as *mut robj;
    buf[0 as libc::c_int as usize] = '*' as i32 as libc::c_char;
    len = 1 as libc::c_int
        + ll2string(
            buf.as_mut_ptr().offset(1 as libc::c_int as isize),
            (core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            argc as libc::c_longlong,
        );
    let fresh0 = len;
    len = len + 1;
    buf[fresh0 as usize] = '\r' as i32 as libc::c_char;
    let fresh1 = len;
    len = len + 1;
    buf[fresh1 as usize] = '\n' as i32 as libc::c_char;
    dst = sdscatlen(dst, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
    j = 0 as libc::c_int;
    while j < argc {
        o = getDecodedObject(*argv.offset(j as isize));
        buf[0 as libc::c_int as usize] = '$' as i32 as libc::c_char;
        len = 1 as libc::c_int
            + ll2string(
                buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                (core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                sdslen((*o).ptr as sds) as libc::c_longlong,
            );
        let fresh2 = len;
        len = len + 1;
        buf[fresh2 as usize] = '\r' as i32 as libc::c_char;
        let fresh3 = len;
        len = len + 1;
        buf[fresh3 as usize] = '\n' as i32 as libc::c_char;
        dst = sdscatlen(dst, buf.as_mut_ptr() as *const libc::c_void, len as size_t);
        dst = sdscatlen(dst, (*o).ptr, sdslen((*o).ptr as sds));
        dst = sdscatlen(
            dst,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        decrRefCount(o);
        j += 1;
    }
    return dst;
}
#[no_mangle]
pub unsafe extern "C" fn genAofTimestampAnnotationIfNeeded(
    mut force: libc::c_int,
) -> sds {
    let mut ts: sds = 0 as sds;
    if force != 0 || server.aof_cur_timestamp < server.unixtime as libc::c_long {
        server
            .aof_cur_timestamp = if force != 0 {
            time(0 as *mut time_t)
        } else {
            server.unixtime as libc::c_long
        };
        ts = sdscatfmt(
            sdsempty(),
            b"#TS:%I\r\n\0" as *const u8 as *const libc::c_char,
            server.aof_cur_timestamp,
        );
        if sdslen(ts) <= 1024 as libc::c_int as libc::c_ulong {} else {
            _serverAssert(
                b"sdslen(ts) <= AOF_ANNOTATION_LINE_MAX_LEN\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                1293 as libc::c_int,
            );
            unreachable!();
        };
    }
    return ts;
}
#[no_mangle]
pub unsafe extern "C" fn feedAppendOnlyFile(
    mut dictid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) {
    let mut buf: sds = sdsempty();
    if dictid >= 0 as libc::c_int && dictid < server.dbnum {} else {
        _serverAssert(
            b"dictid >= 0 && dictid < server.dbnum\0" as *const u8
                as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1301 as libc::c_int,
        );
        unreachable!();
    };
    if server.aof_timestamp_enabled != 0 {
        let mut ts: sds = genAofTimestampAnnotationIfNeeded(0 as libc::c_int);
        if !ts.is_null() {
            buf = sdscatsds(buf, ts);
            sdsfree(ts);
        }
    }
    if dictid != server.aof_selected_db {
        let mut seldb: [libc::c_char; 64] = [0; 64];
        snprintf(
            seldb.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%d\0" as *const u8 as *const libc::c_char,
            dictid,
        );
        buf = sdscatprintf(
            buf,
            b"*2\r\n$6\r\nSELECT\r\n$%lu\r\n%s\r\n\0" as *const u8
                as *const libc::c_char,
            strlen(seldb.as_mut_ptr()),
            seldb.as_mut_ptr(),
        );
        server.aof_selected_db = dictid;
    }
    buf = catAppendOnlyGenericCommand(buf, argc, argv);
    if server.aof_state == 1 as libc::c_int
        || server.aof_state == 2 as libc::c_int && server.child_type == 2 as libc::c_int
    {
        server
            .aof_buf = sdscatlen(
            server.aof_buf,
            buf as *const libc::c_void,
            sdslen(buf),
        );
    }
    sdsfree(buf);
}
#[no_mangle]
pub unsafe extern "C" fn createAOFClient() -> *mut client {
    let mut c: *mut client = createClient(0 as *mut connection);
    (*c).id = 18446744073709551615 as libc::c_ulong;
    (*c).flags = ((1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
    (*c).replstate = 6 as libc::c_int;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn loadSingleAppendOnlyFile(
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fakeClient: *mut client = 0 as *mut client;
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
    let mut old_aof_state: libc::c_int = server.aof_state;
    let mut loops: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut valid_up_to: off_t = 0 as libc::c_int as off_t;
    let mut valid_before_multi: off_t = 0 as libc::c_int as off_t;
    let mut last_progress_report_size: off_t = 0 as libc::c_int as off_t;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut aof_filepath: sds = makePath(server.aof_dirname, filename);
    let mut fp: *mut FILE = fopen(
        aof_filepath as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        let mut en: libc::c_int = *__errno_location();
        if stat(aof_filepath as *const libc::c_char, &mut sb) == 0 as libc::c_int
            || *__errno_location() != 2 as libc::c_int
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Fatal error: can't open the append log file %s for reading: %s\0"
                        as *const u8 as *const libc::c_char,
                    filename,
                    strerror(en),
                );
            }
            sdsfree(aof_filepath);
            return 3 as libc::c_int;
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"The append log file %s doesn't exist: %s\0" as *const u8
                        as *const libc::c_char,
                    filename,
                    strerror(*__errno_location()),
                );
            }
            sdsfree(aof_filepath);
            return 1 as libc::c_int;
        }
    }
    if !fp.is_null() && fstat(fileno(fp), &mut sb) != -(1 as libc::c_int)
        && sb.st_size == 0 as libc::c_int as libc::c_long
    {
        fclose(fp);
        sdsfree(aof_filepath);
        return 2 as libc::c_int;
    }
    server.aof_state = 0 as libc::c_int;
    let mut old_client: *mut client = server.current_client;
    server.current_client = createAOFClient();
    fakeClient = server.current_client;
    let mut sig: [libc::c_char; 5] = [0; 5];
    if fread(
        sig.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        5 as libc::c_int as libc::c_ulong,
        fp,
    ) != 5 as libc::c_int as libc::c_ulong
        || memcmp(
            sig.as_mut_ptr() as *const libc::c_void,
            b"REDIS\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        if fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
            == -(1 as libc::c_int)
        {
            current_block = 8304579957269340979;
        } else {
            current_block = 2516253395664191498;
        }
    } else {
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
        let mut old_style: libc::c_int = (strcmp(filename, server.aof_filename) == 0)
            as libc::c_int;
        if old_style != 0 {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Reading RDB preamble from AOF file...\0" as *const u8
                        as *const libc::c_char,
                );
            }
        } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Reading RDB base file on AOF loading...\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if fseek(fp, 0 as libc::c_int as libc::c_long, 0 as libc::c_int)
            == -(1 as libc::c_int)
        {
            current_block = 8304579957269340979;
        } else {
            rioInitWithFile(&mut rdb, fp);
            if rdbLoadRio(
                &mut rdb,
                (1 as libc::c_int) << 0 as libc::c_int,
                0 as *mut rdbSaveInfo,
            ) != 0 as libc::c_int
            {
                if old_style != 0 {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error reading the RDB preamble of the AOF file %s, AOF loading aborted\0"
                                as *const u8 as *const libc::c_char,
                            filename,
                        );
                    }
                } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                {
                    _serverLog(
                        3 as libc::c_int,
                        b"Error reading the RDB base file %s, AOF loading aborted\0"
                            as *const u8 as *const libc::c_char,
                        filename,
                    );
                }
                current_block = 8304579957269340979;
            } else {
                loadingAbsProgress(ftello(fp));
                last_progress_report_size = ftello(fp);
                if old_style != 0 {
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"Reading the remaining AOF tail...\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                }
                current_block = 2516253395664191498;
            }
        }
    }
    '_readerr: loop {
        match current_block {
            2516253395664191498 => {
                let mut argc: libc::c_int = 0;
                let mut j: libc::c_int = 0;
                let mut len: libc::c_ulong = 0;
                let mut argv: *mut *mut robj = 0 as *mut *mut robj;
                let mut buf: [libc::c_char; 1024] = [0; 1024];
                let mut argsds: sds = 0 as *mut libc::c_char;
                let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
                let fresh4 = loops;
                loops = loops + 1;
                if fresh4 % 1024 as libc::c_int as libc::c_long == 0 {
                    let mut progress_delta: off_t = ftello(fp)
                        - last_progress_report_size;
                    loadingIncrProgress(progress_delta);
                    last_progress_report_size += progress_delta;
                    processEventsWhileBlocked();
                    processModuleLoadingProgressEvent(1 as libc::c_int);
                }
                if (fgets(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                        as libc::c_int,
                    fp,
                ))
                    .is_null()
                {
                    if !(feof(fp) != 0) {
                        current_block = 8304579957269340979;
                        continue;
                    }
                    if (*fakeClient).flags
                        & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
                    {
                        current_block = 5265702136860997526;
                        break;
                    } else {
                        current_block = 852400248170036051;
                        break;
                    }
                } else {
                    if buf[0 as libc::c_int as usize] as libc::c_int == '#' as i32 {
                        current_block = 2516253395664191498;
                        continue;
                    }
                    if buf[0 as libc::c_int as usize] as libc::c_int != '*' as i32 {
                        current_block = 9904301676592638684;
                        break;
                    }
                    if buf[1 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
                        current_block = 8304579957269340979;
                        continue;
                    }
                    argc = atoi(buf.as_mut_ptr().offset(1 as libc::c_int as isize));
                    if argc < 1 as libc::c_int {
                        current_block = 9904301676592638684;
                        break;
                    }
                    if argc as size_t
                        > (18446744073709551615 as libc::c_ulong)
                            .wrapping_div(
                                core::mem::size_of::<*mut robj>() as libc::c_ulong,
                            )
                    {
                        current_block = 9904301676592638684;
                        break;
                    }
                    argv = zmalloc(
                        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                            .wrapping_mul(argc as libc::c_ulong),
                    ) as *mut *mut robj;
                    (*fakeClient).argc = argc;
                    (*fakeClient).argv = argv;
                    (*fakeClient).argv_len = argc;
                    j = 0 as libc::c_int;
                    while j < argc {
                        let mut readres: *mut libc::c_char = fgets(
                            buf.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong as libc::c_int,
                            fp,
                        );
                        if readres.is_null()
                            || buf[0 as libc::c_int as usize] as libc::c_int
                                != '$' as i32
                        {
                            (*fakeClient).argc = j;
                            freeClientArgv(fakeClient);
                            if readres.is_null() {
                                current_block = 8304579957269340979;
                                continue '_readerr;
                            } else {
                                current_block = 9904301676592638684;
                                break '_readerr;
                            }
                        } else {
                            len = strtol(
                                buf.as_mut_ptr().offset(1 as libc::c_int as isize),
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            ) as libc::c_ulong;
                            argsds = sdsnewlen(SDS_NOINIT as *const libc::c_void, len);
                            if len != 0
                                && fread(
                                    argsds as *mut libc::c_void,
                                    len,
                                    1 as libc::c_int as libc::c_ulong,
                                    fp,
                                ) == 0 as libc::c_int as libc::c_ulong
                            {
                                sdsfree(argsds);
                                (*fakeClient).argc = j;
                                freeClientArgv(fakeClient);
                                current_block = 8304579957269340979;
                                continue '_readerr;
                            } else {
                                let ref mut fresh5 = *argv.offset(j as isize);
                                *fresh5 = createObject(
                                    0 as libc::c_int,
                                    argsds as *mut libc::c_void,
                                );
                                if fread(
                                    buf.as_mut_ptr() as *mut libc::c_void,
                                    2 as libc::c_int as libc::c_ulong,
                                    1 as libc::c_int as libc::c_ulong,
                                    fp,
                                ) == 0 as libc::c_int as libc::c_ulong
                                {
                                    (*fakeClient).argc = j + 1 as libc::c_int;
                                    freeClientArgv(fakeClient);
                                    current_block = 8304579957269340979;
                                    continue '_readerr;
                                } else {
                                    j += 1;
                                }
                            }
                        }
                    }
                    cmd = lookupCommand(argv, argc);
                    if cmd.is_null() {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Unknown command '%s' reading the append only file %s\0"
                                    as *const u8 as *const libc::c_char,
                                (**argv.offset(0 as libc::c_int as isize)).ptr
                                    as *mut libc::c_char,
                                filename,
                            );
                        }
                        freeClientArgv(fakeClient);
                        ret = 4 as libc::c_int;
                        current_block = 5212373964289612464;
                        break;
                    } else {
                        if (*cmd).proc_0
                            == Some(
                                multiCommand as unsafe extern "C" fn(*mut client) -> (),
                            )
                        {
                            valid_before_multi = valid_up_to;
                        }
                        (*fakeClient).lastcmd = cmd;
                        (*fakeClient).cmd = (*fakeClient).lastcmd;
                        if (*fakeClient).flags
                            & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
                            != 0
                            && (*(*fakeClient).cmd).proc_0
                                != Some(
                                    execCommand as unsafe extern "C" fn(*mut client) -> (),
                                )
                        {
                            queueMultiCommand(fakeClient, (*cmd).flags);
                        } else {
                            ((*cmd).proc_0)
                                .expect("non-null function pointer")(fakeClient);
                        }
                        if (*fakeClient).bufpos == 0 as libc::c_int
                            && (*(*fakeClient).reply).len
                                == 0 as libc::c_int as libc::c_ulong
                        {} else {
                            _serverAssert(
                                b"fakeClient->bufpos == 0 && listLength(fakeClient->reply) == 0\0"
                                    as *const u8 as *const libc::c_char,
                                b"aof.c\0" as *const u8 as *const libc::c_char,
                                1540 as libc::c_int,
                            );
                            unreachable!();
                        };
                        if (*fakeClient).flags
                            & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong
                            == 0 as libc::c_int as libc::c_ulong
                        {} else {
                            _serverAssert(
                                b"(fakeClient->flags & CLIENT_BLOCKED) == 0\0" as *const u8
                                    as *const libc::c_char,
                                b"aof.c\0" as *const u8 as *const libc::c_char,
                                1543 as libc::c_int,
                            );
                            unreachable!();
                        };
                        freeClientArgv(fakeClient);
                        if server.aof_load_truncated != 0 {
                            valid_up_to = ftello(fp);
                        }
                        if server.key_load_delay != 0 {
                            debugDelay(server.key_load_delay);
                        }
                        current_block = 2516253395664191498;
                    }
                }
            }
            _ => {
                if !(feof(fp) == 0) {
                    current_block = 9752757855149897415;
                    break;
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Unrecoverable error reading the append only file %s: %s\0"
                            as *const u8 as *const libc::c_char,
                        filename,
                        strerror(*__errno_location()),
                    );
                }
                ret = 4 as libc::c_int;
                current_block = 5212373964289612464;
                break;
            }
        }
    }
    match current_block {
        5265702136860997526 => {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Revert incomplete MULTI/EXEC transaction in AOF file %s\0"
                        as *const u8 as *const libc::c_char,
                    filename,
                );
            }
            valid_up_to = valid_before_multi;
            current_block = 9752757855149897415;
        }
        9904301676592638684 => {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Bad file format reading the append only file %s: make a backup of your AOF file, then use ./redis-check-aof --fix <filename.manifest>\0"
                        as *const u8 as *const libc::c_char,
                    filename,
                );
            }
            ret = 4 as libc::c_int;
            current_block = 5212373964289612464;
        }
        _ => {}
    }
    match current_block {
        9752757855149897415 => {
            if server.aof_load_truncated != 0 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"!!! Warning: short read while loading the AOF file %s!!!\0"
                            as *const u8 as *const libc::c_char,
                        filename,
                    );
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"!!! Truncating the AOF %s at offset %llu !!!\0" as *const u8
                            as *const libc::c_char,
                        filename,
                        valid_up_to as libc::c_ulonglong,
                    );
                }
                if valid_up_to == -(1 as libc::c_int) as libc::c_long
                    || truncate(aof_filepath as *const libc::c_char, valid_up_to)
                        == -(1 as libc::c_int)
                {
                    if valid_up_to == -(1 as libc::c_int) as libc::c_long {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Last valid command offset is invalid\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                    } else if !((3 as libc::c_int & 0xff as libc::c_int)
                        < server.verbosity)
                    {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error truncating the AOF file %s: %s\0" as *const u8
                                as *const libc::c_char,
                            filename,
                            strerror(*__errno_location()),
                        );
                    }
                    current_block = 13810333397648094191;
                } else if server.aof_fd != -(1 as libc::c_int)
                    && lseek(
                        server.aof_fd,
                        0 as libc::c_int as __off64_t,
                        2 as libc::c_int,
                    ) == -(1 as libc::c_int) as libc::c_long
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Can't seek the end of the AOF file %s: %s\0" as *const u8
                                as *const libc::c_char,
                            filename,
                            strerror(*__errno_location()),
                        );
                    }
                    current_block = 13810333397648094191;
                } else {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"AOF %s loaded anyway because aof-load-truncated is enabled\0"
                                as *const u8 as *const libc::c_char,
                            filename,
                        );
                    }
                    ret = 5 as libc::c_int;
                    current_block = 852400248170036051;
                }
            } else {
                current_block = 13810333397648094191;
            }
            match current_block {
                852400248170036051 => {}
                _ => {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Unexpected end of file reading the append only file %s. You can: 1) Make a backup of your AOF file, then use ./redis-check-aof --fix <filename.manifest>. 2) Alternatively you can set the 'aof-load-truncated' configuration option to yes and restart the server.\0"
                                as *const u8 as *const libc::c_char,
                            filename,
                        );
                    }
                    ret = 4 as libc::c_int;
                    current_block = 5212373964289612464;
                }
            }
        }
        _ => {}
    }
    match current_block {
        852400248170036051 => {
            loadingIncrProgress(ftello(fp) - last_progress_report_size);
            server.aof_state = old_aof_state;
        }
        _ => {}
    }
    if !fakeClient.is_null() {
        freeClient(fakeClient);
    }
    server.current_client = old_client;
    fclose(fp);
    sdsfree(aof_filepath);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn loadAppendOnlyFiles(mut am: *mut aofManifest) -> libc::c_int {
    let mut current_block: u64;
    if !am.is_null() {} else {
        _serverAssert(
            b"am != NULL\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1624 as libc::c_int,
        );
        unreachable!();
    };
    let mut status: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_longlong = 0;
    let mut total_size: off_t = 0 as libc::c_int as off_t;
    let mut base_size: off_t = 0 as libc::c_int as off_t;
    let mut aof_name: sds = 0 as *mut libc::c_char;
    let mut total_num: libc::c_int = 0;
    let mut aof_num: libc::c_int = 0 as libc::c_int;
    let mut last_file: libc::c_int = 0;
    if fileExist(server.aof_filename) != 0 {
        if dirExists(server.aof_dirname) == 0
            || ((*am).base_aof_info).is_null()
                && (*(*am).incr_aof_list).len == 0 as libc::c_int as libc::c_ulong
            || !((*am).base_aof_info).is_null()
                && (*(*am).incr_aof_list).len == 0 as libc::c_int as libc::c_ulong
                && strcmp(
                    (*(*am).base_aof_info).file_name as *const libc::c_char,
                    server.aof_filename,
                ) == 0 && aofFileExist(server.aof_filename) == 0
        {
            aofUpgradePrepare(am);
        }
    }
    if ((*am).base_aof_info).is_null()
        && (*(*am).incr_aof_list).len == 0 as libc::c_int as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    total_num = getBaseAndIncrAppendOnlyFilesNum(am);
    if total_num > 0 as libc::c_int {} else {
        _serverAssert(
            b"total_num > 0\0" as *const u8 as *const libc::c_char,
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1655 as libc::c_int,
        );
        unreachable!();
    };
    total_size = getBaseAndIncrAppendOnlyFilesSize(am, &mut status);
    if status != 0 as libc::c_int {
        if status == 1 as libc::c_int {
            status = 4 as libc::c_int;
        }
        return status;
    } else {
        if total_size == 0 as libc::c_int as libc::c_long {
            return 2 as libc::c_int;
        }
    }
    startLoading(
        total_size as size_t,
        (1 as libc::c_int) << 0 as libc::c_int,
        0 as libc::c_int,
    );
    if !((*am).base_aof_info).is_null() {
        if (*(*am).base_aof_info).file_type as libc::c_uint
            == AOF_FILE_TYPE_BASE as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"am->base_aof_info->file_type == AOF_FILE_TYPE_BASE\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                1673 as libc::c_int,
            );
            unreachable!();
        };
        aof_name = (*(*am).base_aof_info).file_name;
        updateLoadingFileName(aof_name);
        base_size = getAppendOnlyFileSize(aof_name, 0 as *mut libc::c_int);
        aof_num += 1;
        last_file = (aof_num == total_num) as libc::c_int;
        start = ustime();
        ret = loadSingleAppendOnlyFile(aof_name);
        if ret == 0 as libc::c_int || ret == 5 as libc::c_int && last_file != 0 {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"DB loaded from base file %s: %.3f seconds\0" as *const u8
                        as *const libc::c_char,
                    aof_name,
                    ((ustime() - start) as libc::c_float
                        / 1000000 as libc::c_int as libc::c_float) as libc::c_double,
                );
            }
        }
        if ret == 5 as libc::c_int && last_file == 0 {
            ret = 4 as libc::c_int;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Fatal error: the truncated file is not the last file\0"
                        as *const u8 as *const libc::c_char,
                );
            }
        }
        if ret == 3 as libc::c_int || ret == 4 as libc::c_int {
            current_block = 2717692205864117062;
        } else {
            current_block = 9520865839495247062;
        }
    } else {
        current_block = 9520865839495247062;
    }
    match current_block {
        9520865839495247062 => {
            if (*(*am).incr_aof_list).len != 0 {
                let mut ln: *mut listNode = 0 as *mut listNode;
                let mut li: listIter = listIter {
                    next: 0 as *mut listNode,
                    direction: 0,
                };
                listRewind((*am).incr_aof_list, &mut li);
                loop {
                    ln = listNext(&mut li);
                    if ln.is_null() {
                        current_block = 6072622540298447352;
                        break;
                    }
                    let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
                    if (*ai).file_type as libc::c_uint
                        == AOF_FILE_TYPE_INCR as libc::c_int as libc::c_uint
                    {} else {
                        _serverAssert(
                            b"ai->file_type == AOF_FILE_TYPE_INCR\0" as *const u8
                                as *const libc::c_char,
                            b"aof.c\0" as *const u8 as *const libc::c_char,
                            1704 as libc::c_int,
                        );
                        unreachable!();
                    };
                    aof_name = (*ai).file_name;
                    updateLoadingFileName(aof_name);
                    aof_num += 1;
                    last_file = (aof_num == total_num) as libc::c_int;
                    start = ustime();
                    ret = loadSingleAppendOnlyFile(aof_name);
                    if ret == 0 as libc::c_int
                        || ret == 5 as libc::c_int && last_file != 0
                    {
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"DB loaded from incr file %s: %.3f seconds\0" as *const u8
                                    as *const libc::c_char,
                                aof_name,
                                ((ustime() - start) as libc::c_float
                                    / 1000000 as libc::c_int as libc::c_float) as libc::c_double,
                            );
                        }
                    }
                    if ret == 2 as libc::c_int {
                        ret = 0 as libc::c_int;
                    }
                    if ret == 5 as libc::c_int && last_file == 0 {
                        ret = 4 as libc::c_int;
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Fatal error: the truncated file is not the last file\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                    }
                    if ret == 3 as libc::c_int || ret == 4 as libc::c_int {
                        current_block = 2717692205864117062;
                        break;
                    }
                }
            } else {
                current_block = 6072622540298447352;
            }
            match current_block {
                2717692205864117062 => {}
                _ => {
                    server.aof_current_size = total_size;
                    server.aof_rewrite_base_size = base_size;
                    server.aof_fsync_offset = server.aof_current_size;
                }
            }
        }
        _ => {}
    }
    stopLoading((ret == 0 as libc::c_int || ret == 5 as libc::c_int) as libc::c_int);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkObject(
    mut r: *mut rio,
    mut obj: *mut robj,
) -> libc::c_int {
    if (*obj).encoding() as libc::c_int == 1 as libc::c_int {
        return rioWriteBulkLongLong(r, (*obj).ptr as libc::c_long as libc::c_longlong)
            as libc::c_int
    } else {
        if (*obj).encoding() as libc::c_int == 0 as libc::c_int
            || (*obj).encoding() as libc::c_int == 8 as libc::c_int
        {
            return rioWriteBulkString(
                r,
                (*obj).ptr as *const libc::c_char,
                sdslen((*obj).ptr as sds),
            ) as libc::c_int
        } else {
            _serverPanic(
                b"aof.c\0" as *const u8 as *const libc::c_char,
                1763 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn rewriteListObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
) -> libc::c_int {
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut items: libc::c_longlong = listTypeLength(o) as libc::c_longlong;
    if (*o).encoding() as libc::c_int == 9 as libc::c_int {
        let mut list: *mut quicklist = (*o).ptr as *mut quicklist;
        let mut li: *mut quicklistIter = quicklistGetIterator(list, 0 as libc::c_int);
        let mut entry: quicklistEntry = quicklistEntry {
            quicklist: 0 as *const quicklist,
            node: 0 as *mut quicklistNode,
            zi: 0 as *mut libc::c_uchar,
            value: 0 as *mut libc::c_uchar,
            longval: 0,
            sz: 0,
            offset: 0,
        };
        while quicklistNext(li, &mut entry) != 0 {
            if count == 0 as libc::c_int as libc::c_longlong {
                let mut cmd_items: libc::c_int = (if items
                    > 64 as libc::c_int as libc::c_longlong
                {
                    64 as libc::c_int as libc::c_longlong
                } else {
                    items
                }) as libc::c_int;
                if rioWriteBulkCount(
                    r,
                    '*' as i32 as libc::c_char,
                    (2 as libc::c_int + cmd_items) as libc::c_long,
                ) == 0
                    || rioWriteBulkString(
                        r,
                        b"RPUSH\0" as *const u8 as *const libc::c_char,
                        5 as libc::c_int as size_t,
                    ) == 0 || rioWriteBulkObject(r, key) == 0
                {
                    quicklistReleaseIterator(li);
                    return 0 as libc::c_int;
                }
            }
            if !(entry.value).is_null() {
                if rioWriteBulkString(r, entry.value as *mut libc::c_char, entry.sz) == 0
                {
                    quicklistReleaseIterator(li);
                    return 0 as libc::c_int;
                }
            } else if rioWriteBulkLongLong(r, entry.longval) == 0 {
                quicklistReleaseIterator(li);
                return 0 as libc::c_int;
            }
            count += 1;
            if count == 64 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
            items -= 1;
        }
        quicklistReleaseIterator(li);
    } else {
        _serverPanic(
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1806 as libc::c_int,
            b"Unknown list encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteSetObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
) -> libc::c_int {
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut items: libc::c_longlong = setTypeSize(o) as libc::c_longlong;
    if (*o).encoding() as libc::c_int == 6 as libc::c_int {
        let mut ii: libc::c_int = 0 as libc::c_int;
        let mut llval: int64_t = 0;
        loop {
            let fresh6 = ii;
            ii = ii + 1;
            if !(intsetGet((*o).ptr as *mut intset, fresh6 as uint32_t, &mut llval) != 0)
            {
                break;
            }
            if count == 0 as libc::c_int as libc::c_longlong {
                let mut cmd_items: libc::c_int = (if items
                    > 64 as libc::c_int as libc::c_longlong
                {
                    64 as libc::c_int as libc::c_longlong
                } else {
                    items
                }) as libc::c_int;
                if rioWriteBulkCount(
                    r,
                    '*' as i32 as libc::c_char,
                    (2 as libc::c_int + cmd_items) as libc::c_long,
                ) == 0
                    || rioWriteBulkString(
                        r,
                        b"SADD\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    ) == 0 || rioWriteBulkObject(r, key) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            if rioWriteBulkLongLong(r, llval as libc::c_longlong) == 0 {
                return 0 as libc::c_int;
            }
            count += 1;
            if count == 64 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
            items -= 1;
        }
    } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
        let mut di: *mut dictIterator = dictGetIterator((*o).ptr as *mut dict);
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut ele: sds = (*de).key as sds;
            if count == 0 as libc::c_int as libc::c_longlong {
                let mut cmd_items_0: libc::c_int = (if items
                    > 64 as libc::c_int as libc::c_longlong
                {
                    64 as libc::c_int as libc::c_longlong
                } else {
                    items
                }) as libc::c_int;
                if rioWriteBulkCount(
                    r,
                    '*' as i32 as libc::c_char,
                    (2 as libc::c_int + cmd_items_0) as libc::c_long,
                ) == 0
                    || rioWriteBulkString(
                        r,
                        b"SADD\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    ) == 0 || rioWriteBulkObject(r, key) == 0
                {
                    dictReleaseIterator(di);
                    return 0 as libc::c_int;
                }
            }
            if rioWriteBulkString(r, ele as *const libc::c_char, sdslen(ele)) == 0 {
                dictReleaseIterator(di);
                return 0 as libc::c_int;
            }
            count += 1;
            if count == 64 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
            items -= 1;
        }
        dictReleaseIterator(di);
    } else {
        _serverPanic(
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1863 as libc::c_int,
            b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteSortedSetObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
) -> libc::c_int {
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut items: libc::c_longlong = zsetLength(o) as libc::c_longlong;
    if (*o).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*o).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vll: libc::c_longlong = 0;
        let mut score: libc::c_double = 0.;
        eptr = lpSeek(zl, 0 as libc::c_int as libc::c_long);
        if !eptr.is_null() {} else {
            _serverAssert(
                b"eptr != NULL\0" as *const u8 as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                1882 as libc::c_int,
            );
            unreachable!();
        };
        sptr = lpNext(zl, eptr);
        if !sptr.is_null() {} else {
            _serverAssert(
                b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                1884 as libc::c_int,
            );
            unreachable!();
        };
        while !eptr.is_null() {
            vstr = lpGetValue(eptr, &mut vlen, &mut vll);
            score = zzlGetScore(sptr);
            if count == 0 as libc::c_int as libc::c_longlong {
                let mut cmd_items: libc::c_int = (if items
                    > 64 as libc::c_int as libc::c_longlong
                {
                    64 as libc::c_int as libc::c_longlong
                } else {
                    items
                }) as libc::c_int;
                if rioWriteBulkCount(
                    r,
                    '*' as i32 as libc::c_char,
                    (2 as libc::c_int + cmd_items * 2 as libc::c_int) as libc::c_long,
                ) == 0
                    || rioWriteBulkString(
                        r,
                        b"ZADD\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    ) == 0 || rioWriteBulkObject(r, key) == 0
                {
                    return 0 as libc::c_int;
                }
            }
            if rioWriteBulkDouble(r, score) == 0 {
                return 0 as libc::c_int;
            }
            if !vstr.is_null() {
                if rioWriteBulkString(r, vstr as *mut libc::c_char, vlen as size_t) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if rioWriteBulkLongLong(r, vll) == 0 {
                return 0 as libc::c_int
            }
            zzlNext(zl, &mut eptr, &mut sptr);
            count += 1;
            if count == 64 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
            items -= 1;
        }
    } else if (*o).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*o).ptr as *mut zset;
        let mut di: *mut dictIterator = dictGetIterator((*zs).dict);
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut ele: sds = (*de).key as sds;
            let mut score_0: *mut libc::c_double = (*de).v.val as *mut libc::c_double;
            if count == 0 as libc::c_int as libc::c_longlong {
                let mut cmd_items_0: libc::c_int = (if items
                    > 64 as libc::c_int as libc::c_longlong
                {
                    64 as libc::c_int as libc::c_longlong
                } else {
                    items
                }) as libc::c_int;
                if rioWriteBulkCount(
                    r,
                    '*' as i32 as libc::c_char,
                    (2 as libc::c_int + cmd_items_0 * 2 as libc::c_int) as libc::c_long,
                ) == 0
                    || rioWriteBulkString(
                        r,
                        b"ZADD\0" as *const u8 as *const libc::c_char,
                        4 as libc::c_int as size_t,
                    ) == 0 || rioWriteBulkObject(r, key) == 0
                {
                    dictReleaseIterator(di);
                    return 0 as libc::c_int;
                }
            }
            if rioWriteBulkDouble(r, *score_0) == 0
                || rioWriteBulkString(r, ele as *const libc::c_char, sdslen(ele)) == 0
            {
                dictReleaseIterator(di);
                return 0 as libc::c_int;
            }
            count += 1;
            if count == 64 as libc::c_int as libc::c_longlong {
                count = 0 as libc::c_int as libc::c_longlong;
            }
            items -= 1;
        }
        dictReleaseIterator(di);
    } else {
        _serverPanic(
            b"aof.c\0" as *const u8 as *const libc::c_char,
            1943 as libc::c_int,
            b"Unknown sorted zset encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn rioWriteHashIteratorCursor(
    mut r: *mut rio,
    mut hi: *mut hashTypeIterator,
    mut what: libc::c_int,
) -> libc::c_int {
    if (*hi).encoding == 11 as libc::c_int {
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = (2147483647 as libc::c_int as libc::c_uint)
            .wrapping_mul(2 as libc::c_uint)
            .wrapping_add(1 as libc::c_uint);
        let mut vll: libc::c_longlong = 9223372036854775807 as libc::c_longlong;
        hashTypeCurrentFromListpack(hi, what, &mut vstr, &mut vlen, &mut vll);
        if !vstr.is_null() {
            return rioWriteBulkString(r, vstr as *mut libc::c_char, vlen as size_t)
                as libc::c_int
        } else {
            return rioWriteBulkLongLong(r, vll) as libc::c_int
        }
    } else {
        if (*hi).encoding == 2 as libc::c_int {
            let mut value: sds = hashTypeCurrentFromHashTable(hi, what);
            return rioWriteBulkString(r, value as *const libc::c_char, sdslen(value))
                as libc::c_int;
        }
    }
    _serverPanic(
        b"aof.c\0" as *const u8 as *const libc::c_char,
        1970 as libc::c_int,
        b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
    );
    unreachable!();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteHashObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
) -> libc::c_int {
    let mut hi: *mut hashTypeIterator = 0 as *mut hashTypeIterator;
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut items: libc::c_longlong = hashTypeLength(o) as libc::c_longlong;
    hi = hashTypeInitIterator(o);
    while hashTypeNext(hi) != -(1 as libc::c_int) {
        if count == 0 as libc::c_int as libc::c_longlong {
            let mut cmd_items: libc::c_int = (if items
                > 64 as libc::c_int as libc::c_longlong
            {
                64 as libc::c_int as libc::c_longlong
            } else {
                items
            }) as libc::c_int;
            if rioWriteBulkCount(
                r,
                '*' as i32 as libc::c_char,
                (2 as libc::c_int + cmd_items * 2 as libc::c_int) as libc::c_long,
            ) == 0
                || rioWriteBulkString(
                    r,
                    b"HMSET\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int as size_t,
                ) == 0 || rioWriteBulkObject(r, key) == 0
            {
                hashTypeReleaseIterator(hi);
                return 0 as libc::c_int;
            }
        }
        if rioWriteHashIteratorCursor(r, hi, 1 as libc::c_int) == 0
            || rioWriteHashIteratorCursor(r, hi, 2 as libc::c_int) == 0
        {
            hashTypeReleaseIterator(hi);
            return 0 as libc::c_int;
        }
        count += 1;
        if count == 64 as libc::c_int as libc::c_longlong {
            count = 0 as libc::c_int as libc::c_longlong;
        }
        items -= 1;
    }
    hashTypeReleaseIterator(hi);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteBulkStreamID(
    mut r: *mut rio,
    mut id: *mut streamID,
) -> libc::c_int {
    let mut retval: libc::c_int = 0;
    let mut replyid: sds = sdscatfmt(
        sdsempty(),
        b"%U-%U\0" as *const u8 as *const libc::c_char,
        (*id).ms,
        (*id).seq,
    );
    retval = rioWriteBulkString(r, replyid as *const libc::c_char, sdslen(replyid))
        as libc::c_int;
    sdsfree(replyid);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteStreamPendingEntry(
    mut r: *mut rio,
    mut key: *mut robj,
    mut groupname: *const libc::c_char,
    mut groupname_len: size_t,
    mut consumer: *mut streamConsumer,
    mut rawid: *mut libc::c_uchar,
    mut nack: *mut streamNACK,
) -> libc::c_int {
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    streamDecodeID(rawid as *mut libc::c_void, &mut id);
    if rioWriteBulkCount(
        r,
        '*' as i32 as libc::c_char,
        12 as libc::c_int as libc::c_long,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"XCLAIM\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkObject(r, key) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(r, groupname, groupname_len)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        (*consumer).name as *const libc::c_char,
        sdslen((*consumer).name),
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"0\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkStreamID(r, &mut id) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"TIME\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkLongLong(r, (*nack).delivery_time)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"RETRYCOUNT\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkLongLong(r, (*nack).delivery_count as libc::c_longlong)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"JUSTID\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"FORCE\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rioWriteStreamEmptyConsumer(
    mut r: *mut rio,
    mut key: *mut robj,
    mut groupname: *const libc::c_char,
    mut groupname_len: size_t,
    mut consumer: *mut streamConsumer,
) -> libc::c_int {
    if rioWriteBulkCount(r, '*' as i32 as libc::c_char, 5 as libc::c_int as libc::c_long)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"XGROUP\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        b"CREATECONSUMER\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkObject(r, key) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(r, groupname, groupname_len)
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    if rioWriteBulkString(
        r,
        (*consumer).name as *const libc::c_char,
        sdslen((*consumer).name),
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteStreamObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
) -> libc::c_int {
    let mut s: *mut stream = (*o).ptr as *mut stream;
    let mut si: streamIterator = streamIterator {
        stream: 0 as *mut stream,
        master_id: streamID { ms: 0, seq: 0 },
        master_fields_count: 0,
        master_fields_start: 0 as *mut libc::c_uchar,
        master_fields_ptr: 0 as *mut libc::c_uchar,
        entry_flags: 0,
        rev: 0,
        skip_tombstones: 0,
        start_key: [0; 2],
        end_key: [0; 2],
        ri: raxIterator {
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
        },
        lp: 0 as *mut libc::c_uchar,
        lp_ele: 0 as *mut libc::c_uchar,
        lp_flags: 0 as *mut libc::c_uchar,
        field_buf: [0; 21],
        value_buf: [0; 21],
    };
    streamIteratorStart(
        &mut si,
        s,
        0 as *mut streamID,
        0 as *mut streamID,
        0 as libc::c_int,
    );
    let mut id: streamID = streamID { ms: 0, seq: 0 };
    let mut numfields: int64_t = 0;
    if (*s).length != 0 {
        while streamIteratorGetID(&mut si, &mut id, &mut numfields) != 0 {
            if rioWriteBulkCount(
                r,
                '*' as i32 as libc::c_char,
                3 as libc::c_int as libc::c_long
                    + numfields * 2 as libc::c_int as libc::c_long,
            ) == 0
                || rioWriteBulkString(
                    r,
                    b"XADD\0" as *const u8 as *const libc::c_char,
                    4 as libc::c_int as size_t,
                ) == 0 || rioWriteBulkObject(r, key) == 0
                || rioWriteBulkStreamID(r, &mut id) == 0
            {
                streamIteratorStop(&mut si);
                return 0 as libc::c_int;
            }
            loop {
                let fresh7 = numfields;
                numfields = numfields - 1;
                if !(fresh7 != 0) {
                    break;
                }
                let mut field: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut field_len: int64_t = 0;
                let mut value_len: int64_t = 0;
                streamIteratorGetField(
                    &mut si,
                    &mut field,
                    &mut value,
                    &mut field_len,
                    &mut value_len,
                );
                if rioWriteBulkString(r, field as *mut libc::c_char, field_len as size_t)
                    == 0
                    || rioWriteBulkString(
                        r,
                        value as *mut libc::c_char,
                        value_len as size_t,
                    ) == 0
                {
                    streamIteratorStop(&mut si);
                    return 0 as libc::c_int;
                }
            }
        }
    } else {
        id.ms = 0 as libc::c_int as uint64_t;
        id.seq = 1 as libc::c_int as uint64_t;
        if rioWriteBulkCount(
            r,
            '*' as i32 as libc::c_char,
            7 as libc::c_int as libc::c_long,
        ) == 0
            || rioWriteBulkString(
                r,
                b"XADD\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            ) == 0 || rioWriteBulkObject(r, key) == 0
            || rioWriteBulkString(
                r,
                b"MAXLEN\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            ) == 0
            || rioWriteBulkString(
                r,
                b"0\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) == 0 || rioWriteBulkStreamID(r, &mut id) == 0
            || rioWriteBulkString(
                r,
                b"x\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) == 0
            || rioWriteBulkString(
                r,
                b"y\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int as size_t,
            ) == 0
        {
            streamIteratorStop(&mut si);
            return 0 as libc::c_int;
        }
    }
    if rioWriteBulkCount(r, '*' as i32 as libc::c_char, 7 as libc::c_int as libc::c_long)
        == 0
        || rioWriteBulkString(
            r,
            b"XSETID\0" as *const u8 as *const libc::c_char,
            6 as libc::c_int as size_t,
        ) == 0 || rioWriteBulkObject(r, key) == 0
        || rioWriteBulkStreamID(r, &mut (*s).last_id) == 0
        || rioWriteBulkString(
            r,
            b"ENTRIESADDED\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        ) == 0 || rioWriteBulkLongLong(r, (*s).entries_added as libc::c_longlong) == 0
        || rioWriteBulkString(
            r,
            b"MAXDELETEDID\0" as *const u8 as *const libc::c_char,
            12 as libc::c_int as size_t,
        ) == 0 || rioWriteBulkStreamID(r, &mut (*s).max_deleted_entry_id) == 0
    {
        streamIteratorStop(&mut si);
        return 0 as libc::c_int;
    }
    if !((*s).cgroups).is_null() {
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
        raxStart(&mut ri, (*s).cgroups);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri) != 0 {
            let mut group: *mut streamCG = ri.data as *mut streamCG;
            if rioWriteBulkCount(
                r,
                '*' as i32 as libc::c_char,
                7 as libc::c_int as libc::c_long,
            ) == 0
                || rioWriteBulkString(
                    r,
                    b"XGROUP\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as size_t,
                ) == 0
                || rioWriteBulkString(
                    r,
                    b"CREATE\0" as *const u8 as *const libc::c_char,
                    6 as libc::c_int as size_t,
                ) == 0 || rioWriteBulkObject(r, key) == 0
                || rioWriteBulkString(r, ri.key as *mut libc::c_char, ri.key_len) == 0
                || rioWriteBulkStreamID(r, &mut (*group).last_id) == 0
                || rioWriteBulkString(
                    r,
                    b"ENTRIESREAD\0" as *const u8 as *const libc::c_char,
                    11 as libc::c_int as size_t,
                ) == 0 || rioWriteBulkLongLong(r, (*group).entries_read) == 0
            {
                raxStop(&mut ri);
                streamIteratorStop(&mut si);
                return 0 as libc::c_int;
            }
            let mut ri_cons: raxIterator = raxIterator {
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
            raxStart(&mut ri_cons, (*group).consumers);
            raxSeek(
                &mut ri_cons,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri_cons) != 0 {
                let mut consumer: *mut streamConsumer = ri_cons.data
                    as *mut streamConsumer;
                if raxSize((*consumer).pel) == 0 as libc::c_int as libc::c_ulong {
                    if rioWriteStreamEmptyConsumer(
                        r,
                        key,
                        ri.key as *mut libc::c_char,
                        ri.key_len,
                        consumer,
                    ) == 0 as libc::c_int
                    {
                        raxStop(&mut ri_cons);
                        raxStop(&mut ri);
                        streamIteratorStop(&mut si);
                        return 0 as libc::c_int;
                    }
                } else {
                    let mut ri_pel: raxIterator = raxIterator {
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
                    raxStart(&mut ri_pel, (*consumer).pel);
                    raxSeek(
                        &mut ri_pel,
                        b"^\0" as *const u8 as *const libc::c_char,
                        0 as *mut libc::c_uchar,
                        0 as libc::c_int as size_t,
                    );
                    while raxNext(&mut ri_pel) != 0 {
                        let mut nack: *mut streamNACK = ri_pel.data as *mut streamNACK;
                        if rioWriteStreamPendingEntry(
                            r,
                            key,
                            ri.key as *mut libc::c_char,
                            ri.key_len,
                            consumer,
                            ri_pel.key,
                            nack,
                        ) == 0 as libc::c_int
                        {
                            raxStop(&mut ri_pel);
                            raxStop(&mut ri_cons);
                            raxStop(&mut ri);
                            streamIteratorStop(&mut si);
                            return 0 as libc::c_int;
                        }
                    }
                    raxStop(&mut ri_pel);
                }
            }
            raxStop(&mut ri_cons);
        }
        raxStop(&mut ri);
    }
    streamIteratorStop(&mut si);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rewriteModuleObject(
    mut r: *mut rio,
    mut key: *mut robj,
    mut o: *mut robj,
    mut dbid: libc::c_int,
) -> libc::c_int {
    let mut io: RedisModuleIO = RedisModuleIO {
        bytes: 0,
        rio: 0 as *mut rio,
        type_0: 0 as *mut moduleType,
        error: 0,
        ver: 0,
        ctx: 0 as *mut RedisModuleCtx,
        key: 0 as *mut redisObject,
        dbid: 0,
    };
    let mut mv: *mut moduleValue = (*o).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    io.rio = r;
    io.type_0 = mt;
    io.bytes = 0 as libc::c_int as size_t;
    io.error = 0 as libc::c_int;
    io.ver = 0 as libc::c_int;
    io.key = key;
    io.dbid = dbid;
    io.ctx = 0 as *mut RedisModuleCtx;
    ((*mt).aof_rewrite).expect("non-null function pointer")(&mut io, key, (*mv).value);
    if !(io.ctx).is_null() {
        moduleFreeContext(io.ctx);
        zfree(io.ctx as *mut libc::c_void);
    }
    return if io.error != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
unsafe extern "C" fn rewriteFunctions(mut aof: *mut rio) -> libc::c_int {
    let mut current_block: u64;
    let mut functions: *mut dict = functionsLibGet();
    let mut iter: *mut dictIterator = dictGetIterator(functions);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            current_block = 6483416627284290920;
            break;
        }
        let mut li: *mut functionLibInfo = (*entry).v.val as *mut functionLibInfo;
        if rioWrite(
            aof,
            b"*3\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            current_block = 11212911460225021231;
            break;
        }
        let mut functionload: [u8; 25] = UnsafeCell::new([
            b'$', b'8', b'\r', b'\n', b'F', b'U', b'N', b'C', b'T', b'I', b'O', b'N',
            b'\r', b'\n', b'$', b'4', b'\r', b'\n', b'L', b'O', b'A', b'D', b'\r', b'\n', b'\0'
        ]).into_inner();
        if rioWrite(
            aof,
            functionload.as_mut_ptr() as *const libc::c_void,
            (core::mem::size_of::<[libc::c_char; 25]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int as libc::c_ulong
        {
            current_block = 11212911460225021231;
            break;
        }
        if rioWriteBulkString(aof, (*li).code as *const libc::c_char, sdslen((*li).code))
            == 0 as libc::c_int as libc::c_ulong
        {
            current_block = 11212911460225021231;
            break;
        }
    }
    match current_block {
        11212911460225021231 => {
            dictReleaseIterator(iter);
            return 0 as libc::c_int;
        }
        _ => {
            dictReleaseIterator(iter);
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rewriteAppendOnlyFileRio(mut aof: *mut rio) -> libc::c_int {
    let mut current_block: u64;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut j: libc::c_int = 0;
    let mut key_count: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut updated_time: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if server.aof_timestamp_enabled != 0 {
        let mut ts: sds = genAofTimestampAnnotationIfNeeded(1 as libc::c_int);
        if rioWrite(aof, ts as *const libc::c_void, sdslen(ts))
            == 0 as libc::c_int as libc::c_ulong
        {
            sdsfree(ts);
            current_block = 12222187880575376610;
        } else {
            sdsfree(ts);
            current_block = 15619007995458559411;
        }
    } else {
        current_block = 15619007995458559411;
    }
    match current_block {
        15619007995458559411 => {
            if !(rewriteFunctions(aof) == 0 as libc::c_int) {
                j = 0 as libc::c_int;
                's_39: loop {
                    if !(j < server.dbnum) {
                        current_block = 8835654301469918283;
                        break;
                    }
                    let mut selectcmd: UnsafeCell<[libc::c_char; 17]> = UnsafeCell::new([
                        b'*'.try_into().unwrap(), b'2'.try_into().unwrap(), b'\r'.try_into().unwrap(), b'\n'.try_into().unwrap(), b'$'.try_into().unwrap(), b'6'.try_into().unwrap(), b'\r'.try_into().unwrap(), b'\n'.try_into().unwrap(), b'S'.try_into().unwrap(), b'E'.try_into().unwrap(), b'L'.try_into().unwrap(),
                        b'E'.try_into().unwrap(), b'C'.try_into().unwrap(), b'T'.try_into().unwrap(), b'\r'.try_into().unwrap(), b'\n'.try_into().unwrap(), b'\0'.try_into().unwrap(),
                    ]);
                    let mut db: *mut redisDb = (server.db).offset(j as isize);
                    let mut d: *mut dict = (*db).dict;
                    if !(((*d).ht_used[0 as libc::c_int as usize])
                        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
                        == 0 as libc::c_int as libc::c_ulong)
                    {
                        di = dictGetSafeIterator(d);
                        if rioWrite(
                            aof,
                            selectcmd.get() as *const libc::c_void,
                            (core::mem::size_of::<[libc::c_char; 17]>()
                              as libc::c_ulong)
                              .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                          ) == 0 as libc::c_int as libc::c_ulong
                          {
                            current_block = 12222187880575376610;
                            break;
                          }
                        if rioWriteBulkLongLong(aof, j as libc::c_longlong)
                            == 0 as libc::c_int as libc::c_ulong
                        {
                            current_block = 12222187880575376610;
                            break;
                        }
                        loop {
                            de = dictNext(di);
                            if de.is_null() {
                                break;
                            }
                            let mut keystr: sds = 0 as *mut libc::c_char;
                            let mut key: robj = robj {
                                type_0_encoding_lru: [0; 4],
                                refcount: 0,
                                ptr: 0 as *mut libc::c_void,
                            };
                            let mut o: *mut robj = 0 as *mut robj;
                            let mut expiretime: libc::c_longlong = 0;
                            let mut aof_bytes_before_key: size_t = (*aof)
                                .processed_bytes;
                            keystr = (*de).key as sds;
                            o = (*de).v.val as *mut robj;
                            key.refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
                            key.set_type_0(0 as libc::c_int as libc::c_uint);
                            key.set_encoding(0 as libc::c_int as libc::c_uint);
                            key.ptr = keystr as *mut libc::c_void;
                            expiretime = getExpire(db, &mut key);
                            if (*o).type_0() as libc::c_int == 0 as libc::c_int {
                                let mut cmd: [libc::c_char; 14] = *unsafe {
                                    std::cell::UnsafeCell::new(b"*3\r\n$3\r\nSET\r\n\0").get()
                                        as *mut [libc::c_char; 14]
                                };
                                if rioWrite(
                                    aof,
                                    cmd.as_mut_ptr() as *const libc::c_void,
                                    (core::mem::size_of::<[libc::c_char; 14]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) == 0 as libc::c_int as libc::c_ulong
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                                if rioWriteBulkObject(aof, &mut key) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                                if rioWriteBulkObject(aof, o) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 1 as libc::c_int {
                                if rewriteListObject(aof, &mut key, o) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
                                if rewriteSetObject(aof, &mut key, o) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
                                if rewriteSortedSetObject(aof, &mut key, o)
                                    == 0 as libc::c_int
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
                                if rewriteHashObject(aof, &mut key, o) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 6 as libc::c_int {
                                if rewriteStreamObject(aof, &mut key, o) == 0 as libc::c_int
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else if (*o).type_0() as libc::c_int == 5 as libc::c_int {
                                if rewriteModuleObject(aof, &mut key, o, j)
                                    == 0 as libc::c_int
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            } else {
                                _serverPanic(
                                    b"aof.c\0" as *const u8 as *const libc::c_char,
                                    2298 as libc::c_int,
                                    b"Unknown object type\0" as *const u8 as *const libc::c_char,
                                );
                                unreachable!();
                            }
                            let mut dump_size: size_t = ((*aof).processed_bytes)
                                .wrapping_sub(aof_bytes_before_key);
                            if server.in_fork_child != 0 {
                                dismissObject(o, dump_size);
                            }
                            if expiretime != -(1 as libc::c_int) as libc::c_longlong {
                                let mut cmd_0: [libc::c_char; 14] = *unsafe {
                                    std::cell::UnsafeCell::new(b"*3\r\n$3\r\nSET\r\n\0").get()
                                        as *mut [libc::c_char; 14]
                                };
                                if rioWrite(
                                    aof,
                                    cmd_0.as_mut_ptr() as *const libc::c_void,
                                    (core::mem::size_of::<[libc::c_char; 20]>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                                ) == 0 as libc::c_int as libc::c_ulong
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                                if rioWriteBulkObject(aof, &mut key) == 0 as libc::c_int {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                                if rioWriteBulkLongLong(aof, expiretime)
                                    == 0 as libc::c_int as libc::c_ulong
                                {
                                    current_block = 12222187880575376610;
                                    break 's_39;
                                }
                            }
                            let fresh8 = key_count;
                            key_count = key_count + 1;
                            if fresh8 & 1023 as libc::c_int as libc::c_long
                                == 0 as libc::c_int as libc::c_long
                            {
                                let mut now: libc::c_longlong = mstime();
                                if now - updated_time
                                    >= 1000 as libc::c_int as libc::c_longlong
                                {
                                    sendChildInfo(
                                        CHILD_INFO_TYPE_CURRENT_INFO,
                                        key_count as size_t,
                                        b"AOF rewrite\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                    );
                                    updated_time = now;
                                }
                            }
                            if server.rdb_key_save_delay != 0 {
                                debugDelay(server.rdb_key_save_delay);
                            }
                        }
                        dictReleaseIterator(di);
                        di = 0 as *mut dictIterator;
                    }
                    j += 1;
                }
                match current_block {
                    12222187880575376610 => {}
                    _ => return 0 as libc::c_int,
                }
            }
        }
        _ => {}
    }
    if !di.is_null() {
        dictReleaseIterator(di);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteAppendOnlyFile(
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut aof: rio = rio {
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
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut tmpfile: [libc::c_char; 256] = [0; 256];
    snprintf(
        tmpfile.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"temp-rewriteaof-%d.aof\0" as *const u8 as *const libc::c_char,
        getpid(),
    );
    fp = fopen(tmpfile.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Opening the temp file for AOF rewrite in rewriteAppendOnlyFile(): %s\0"
                    as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    rioInitWithFile(&mut aof, fp);
    if server.aof_rewrite_incremental_fsync != 0 {
        rioSetAutoSync(
            &mut aof,
            (1024 as libc::c_int * 1024 as libc::c_int * 4 as libc::c_int) as off_t,
        );
    }
    startSaving((1 as libc::c_int) << 0 as libc::c_int);
    if server.aof_use_rdb_preamble != 0 {
        let mut error: libc::c_int = 0;
        if rdbSaveRio(
            0 as libc::c_int,
            &mut aof,
            &mut error,
            (1 as libc::c_int) << 0 as libc::c_int,
            0 as *mut rdbSaveInfo,
        ) == -(1 as libc::c_int)
        {
            *__errno_location() = error;
            current_block = 18217938798896821909;
        } else {
            current_block = 1054647088692577877;
        }
    } else if rewriteAppendOnlyFileRio(&mut aof) == -(1 as libc::c_int) {
        current_block = 18217938798896821909;
    } else {
        current_block = 1054647088692577877;
    }
    match current_block {
        1054647088692577877 => {
            if !(fflush(fp) != 0) {
                if !(fsync(fileno(fp)) != 0) {
                    if fclose(fp) != 0 {
                        fp = 0 as *mut FILE;
                    } else {
                        fp = 0 as *mut FILE;
                        if rename(tmpfile.as_mut_ptr(), filename) == -(1 as libc::c_int)
                        {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"Error moving temp append only file on the final destination: %s\0"
                                        as *const u8 as *const libc::c_char,
                                    strerror(*__errno_location()),
                                );
                            }
                            unlink(tmpfile.as_mut_ptr());
                            stopSaving(0 as libc::c_int);
                            return -(1 as libc::c_int);
                        }
                        stopSaving(1 as libc::c_int);
                        return 0 as libc::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Write error writing append only file on disk: %s\0" as *const u8
                as *const libc::c_char,
            strerror(*__errno_location()),
        );
    }
    if !fp.is_null() {
        fclose(fp);
    }
    unlink(tmpfile.as_mut_ptr());
    stopSaving(0 as libc::c_int);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rewriteAppendOnlyFileBackground() -> libc::c_int {
    let mut childpid: pid_t = 0;
    if hasActiveChildProcess() != 0 {
        return -(1 as libc::c_int);
    }
    if dirCreateIfMissing(server.aof_dirname) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open or create append-only dir %s: %s\0" as *const u8
                    as *const libc::c_char,
                server.aof_dirname,
                strerror(*__errno_location()),
            );
        }
        server.aof_lastbgrewrite_status = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    server.aof_selected_db = -(1 as libc::c_int);
    flushAppendOnlyFile(1 as libc::c_int);
    if openNewIncrAofForAppend() != 0 as libc::c_int {
        server.aof_lastbgrewrite_status = -(1 as libc::c_int);
        return -(1 as libc::c_int);
    }
    server.stat_aof_rewrites += 1;
    childpid = redisFork(2 as libc::c_int);
    if childpid == 0 as libc::c_int {
        let mut tmpfile: [libc::c_char; 256] = [0; 256];
        redisSetProcTitle(
            b"redis-aof-rewrite\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        redisSetCpuAffinity(server.aof_rewrite_cpulist);
        snprintf(
            tmpfile.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"temp-rewriteaof-bg-%d.aof\0" as *const u8 as *const libc::c_char,
            getpid(),
        );
        if rewriteAppendOnlyFile(tmpfile.as_mut_ptr()) == 0 as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Successfully created the temporary AOF base file %s\0" as *const u8
                        as *const libc::c_char,
                    tmpfile.as_mut_ptr(),
                );
            }
            sendChildCowInfo(
                CHILD_INFO_TYPE_AOF_COW_SIZE,
                b"AOF rewrite\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            exitFromChild(0 as libc::c_int);
        } else {
            exitFromChild(1 as libc::c_int);
        }
    } else {
        if childpid == -(1 as libc::c_int) {
            server.aof_lastbgrewrite_status = -(1 as libc::c_int);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't rewrite append only file in background: fork: %s\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Background append only file rewriting started by pid %ld\0"
                    as *const u8 as *const libc::c_char,
                childpid as libc::c_long,
            );
        }
        server.aof_rewrite_scheduled = 0 as libc::c_int;
        server.aof_rewrite_time_start = time(0 as *mut time_t);
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bgrewriteaofCommand(mut c: *mut client) {
    if server.child_type == 2 as libc::c_int {
        addReplyError(
            c,
            b"Background append only file rewriting already in progress\0" as *const u8
                as *const libc::c_char,
        );
    } else if hasActiveChildProcess() != 0 || server.in_exec != 0 {
        server.aof_rewrite_scheduled = 1 as libc::c_int;
        server.stat_aofrw_consecutive_failures = 0 as libc::c_int as libc::c_longlong;
        addReplyStatus(
            c,
            b"Background append only file rewriting scheduled\0" as *const u8
                as *const libc::c_char,
        );
    } else if rewriteAppendOnlyFileBackground() == 0 as libc::c_int {
        addReplyStatus(
            c,
            b"Background append only file rewriting started\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        addReplyError(
            c,
            b"Can't execute an AOF background rewriting. Please check the server logs for more information.\0"
                as *const u8 as *const libc::c_char,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn aofRemoveTempFile(mut childpid: pid_t) {
    let mut tmpfile: [libc::c_char; 256] = [0; 256];
    snprintf(
        tmpfile.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"temp-rewriteaof-bg-%d.aof\0" as *const u8 as *const libc::c_char,
        childpid,
    );
    bg_unlink(tmpfile.as_mut_ptr());
    snprintf(
        tmpfile.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"temp-rewriteaof-%d.aof\0" as *const u8 as *const libc::c_char,
        childpid,
    );
    bg_unlink(tmpfile.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn getAppendOnlyFileSize(
    mut filename: sds,
    mut status: *mut libc::c_int,
) -> off_t {
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
    let mut size: off_t = 0;
    let mut latency: mstime_t = 0;
    let mut aof_filepath: sds = makePath(server.aof_dirname, filename);
    if server.latency_monitor_threshold != 0 {
        latency = mstime();
    } else {
        latency = 0 as libc::c_int as mstime_t;
    }
    if stat(aof_filepath as *const libc::c_char, &mut sb) == -(1 as libc::c_int) {
        if !status.is_null() {
            *status = if *__errno_location() == 2 as libc::c_int {
                1 as libc::c_int
            } else {
                3 as libc::c_int
            };
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unable to obtain the AOF file %s length. stat: %s\0" as *const u8
                    as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
        }
        size = 0 as libc::c_int as off_t;
    } else {
        if !status.is_null() {
            *status = 0 as libc::c_int;
        }
        size = sb.st_size;
    }
    if server.latency_monitor_threshold != 0 {
        latency = mstime() - latency;
    }
    if server.latency_monitor_threshold != 0
        && latency >= server.latency_monitor_threshold
    {
        latencyAddSample(b"aof-fstat\0" as *const u8 as *const libc::c_char, latency);
    }
    sdsfree(aof_filepath);
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn getBaseAndIncrAppendOnlyFilesSize(
    mut am: *mut aofManifest,
    mut status: *mut libc::c_int,
) -> off_t {
    let mut size: off_t = 0 as libc::c_int as off_t;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    if !((*am).base_aof_info).is_null() {
        if (*(*am).base_aof_info).file_type as libc::c_uint
            == AOF_FILE_TYPE_BASE as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"am->base_aof_info->file_type == AOF_FILE_TYPE_BASE\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                2536 as libc::c_int,
            );
            unreachable!();
        };
        size += getAppendOnlyFileSize((*(*am).base_aof_info).file_name, status);
        if *status != 0 as libc::c_int {
            return 0 as libc::c_int as off_t;
        }
    }
    listRewind((*am).incr_aof_list, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut ai: *mut aofInfo = (*ln).value as *mut aofInfo;
        if (*ai).file_type as libc::c_uint
            == AOF_FILE_TYPE_INCR as libc::c_int as libc::c_uint
        {} else {
            _serverAssert(
                b"ai->file_type == AOF_FILE_TYPE_INCR\0" as *const u8
                    as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                2545 as libc::c_int,
            );
            unreachable!();
        };
        size += getAppendOnlyFileSize((*ai).file_name, status);
        if *status != 0 as libc::c_int {
            return 0 as libc::c_int as off_t;
        }
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn getBaseAndIncrAppendOnlyFilesNum(
    mut am: *mut aofManifest,
) -> libc::c_int {
    let mut num: libc::c_int = 0 as libc::c_int;
    if !((*am).base_aof_info).is_null() {
        num += 1;
    }
    if !((*am).incr_aof_list).is_null() {
        num = (num as libc::c_ulong).wrapping_add((*(*am).incr_aof_list).len)
            as libc::c_int as libc::c_int;
    }
    return num;
}
#[no_mangle]
pub unsafe extern "C" fn backgroundRewriteDoneHandler(
    mut exitcode: libc::c_int,
    mut bysignal: libc::c_int,
) {
    let mut current_block: u64;
    if bysignal == 0 && exitcode == 0 as libc::c_int {
        let mut tmpfile: [libc::c_char; 256] = [0; 256];
        let mut now: libc::c_longlong = ustime();
        let mut new_base_filepath: sds = 0 as sds;
        let mut new_incr_filepath: sds = 0 as sds;
        let mut temp_am: *mut aofManifest = 0 as *mut aofManifest;
        let mut latency: mstime_t = 0;
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Background AOF rewrite terminated with success\0" as *const u8
                    as *const libc::c_char,
            );
        }
        snprintf(
            tmpfile.as_mut_ptr(),
            256 as libc::c_int as libc::c_ulong,
            b"temp-rewriteaof-bg-%d.aof\0" as *const u8 as *const libc::c_char,
            server.child_pid,
        );
        if !(server.aof_manifest).is_null() {} else {
            _serverAssert(
                b"server.aof_manifest != NULL\0" as *const u8 as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                2577 as libc::c_int,
            );
            unreachable!();
        };
        temp_am = aofManifestDup(server.aof_manifest);
        let mut new_base_filename: sds = getNewBaseFileNameAndMarkPreAsHistory(temp_am);
        if !new_base_filename.is_null() {} else {
            _serverAssert(
                b"new_base_filename != NULL\0" as *const u8 as *const libc::c_char,
                b"aof.c\0" as *const u8 as *const libc::c_char,
                2585 as libc::c_int,
            );
            unreachable!();
        };
        new_base_filepath = makePath(server.aof_dirname, new_base_filename);
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        if rename(tmpfile.as_mut_ptr(), new_base_filepath as *const libc::c_char)
            == -(1 as libc::c_int)
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Error trying to rename the temporary AOF base file %s into %s: %s\0"
                        as *const u8 as *const libc::c_char,
                    tmpfile.as_mut_ptr(),
                    new_base_filepath,
                    strerror(*__errno_location()),
                );
            }
            aofManifestFree(temp_am);
            sdsfree(new_base_filepath);
            server.aof_lastbgrewrite_status = -(1 as libc::c_int);
            server.stat_aofrw_consecutive_failures += 1;
        } else {
            if server.latency_monitor_threshold != 0 {
                latency = mstime() - latency;
            }
            if server.latency_monitor_threshold != 0
                && latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"aof-rename\0" as *const u8 as *const libc::c_char,
                    latency,
                );
            }
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Successfully renamed the temporary AOF base file %s into %s\0"
                        as *const u8 as *const libc::c_char,
                    tmpfile.as_mut_ptr(),
                    new_base_filename,
                );
            }
            if server.aof_state == 2 as libc::c_int {
                let mut temp_incr_aof_name: sds = getTempIncrAofName();
                let mut temp_incr_filepath: sds = makePath(
                    server.aof_dirname,
                    temp_incr_aof_name,
                );
                let mut new_incr_filename: sds = getNewIncrAofName(temp_am);
                new_incr_filepath = makePath(server.aof_dirname, new_incr_filename);
                if server.latency_monitor_threshold != 0 {
                    latency = mstime();
                } else {
                    latency = 0 as libc::c_int as mstime_t;
                }
                if rename(
                    temp_incr_filepath as *const libc::c_char,
                    new_incr_filepath as *const libc::c_char,
                ) == -(1 as libc::c_int)
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Error trying to rename the temporary AOF incr file %s into %s: %s\0"
                                as *const u8 as *const libc::c_char,
                            temp_incr_filepath,
                            new_incr_filepath,
                            strerror(*__errno_location()),
                        );
                    }
                    bg_unlink(new_base_filepath as *const libc::c_char);
                    sdsfree(new_base_filepath);
                    aofManifestFree(temp_am);
                    sdsfree(temp_incr_filepath);
                    sdsfree(new_incr_filepath);
                    sdsfree(temp_incr_aof_name);
                    server.aof_lastbgrewrite_status = -(1 as libc::c_int);
                    server.stat_aofrw_consecutive_failures += 1;
                    current_block = 14400478195970198768;
                } else {
                    if server.latency_monitor_threshold != 0 {
                        latency = mstime() - latency;
                    }
                    if server.latency_monitor_threshold != 0
                        && latency >= server.latency_monitor_threshold
                    {
                        latencyAddSample(
                            b"aof-rename\0" as *const u8 as *const libc::c_char,
                            latency,
                        );
                    }
                    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            2 as libc::c_int,
                            b"Successfully renamed the temporary AOF incr file %s into %s\0"
                                as *const u8 as *const libc::c_char,
                            temp_incr_aof_name,
                            new_incr_filename,
                        );
                    }
                    sdsfree(temp_incr_filepath);
                    sdsfree(temp_incr_aof_name);
                    current_block = 5235537862154438448;
                }
            } else {
                current_block = 5235537862154438448;
            }
            match current_block {
                14400478195970198768 => {}
                _ => {
                    markRewrittenIncrAofAsHistory(temp_am);
                    if persistAofManifest(temp_am) == -(1 as libc::c_int) {
                        bg_unlink(new_base_filepath as *const libc::c_char);
                        aofManifestFree(temp_am);
                        sdsfree(new_base_filepath);
                        if !new_incr_filepath.is_null() {
                            bg_unlink(new_incr_filepath as *const libc::c_char);
                            sdsfree(new_incr_filepath);
                        }
                        server.aof_lastbgrewrite_status = -(1 as libc::c_int);
                        server.stat_aofrw_consecutive_failures += 1;
                    } else {
                        sdsfree(new_base_filepath);
                        if !new_incr_filepath.is_null() {
                            sdsfree(new_incr_filepath);
                        }
                        aofManifestFreeAndUpdate(temp_am);
                        if server.aof_fd != -(1 as libc::c_int) {
                            server.aof_selected_db = -(1 as libc::c_int);
                            server
                                .aof_current_size = getAppendOnlyFileSize(
                                new_base_filename,
                                0 as *mut libc::c_int,
                            ) + server.aof_last_incr_size;
                            server.aof_rewrite_base_size = server.aof_current_size;
                            server.aof_fsync_offset = server.aof_current_size;
                            server.aof_last_fsync = server.unixtime as time_t;
                        }
                        aofDelHistoryFiles();
                        server.aof_lastbgrewrite_status = 0 as libc::c_int;
                        server
                            .stat_aofrw_consecutive_failures = 0 as libc::c_int
                            as libc::c_longlong;
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"Background AOF rewrite finished successfully\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        if server.aof_state == 2 as libc::c_int {
                            server.aof_state = 1 as libc::c_int;
                        }
                        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                1 as libc::c_int,
                                b"Background AOF rewrite signal handler took %lldus\0"
                                    as *const u8 as *const libc::c_char,
                                ustime() - now,
                            );
                        }
                    }
                }
            }
        }
    } else if bysignal == 0 && exitcode != 0 as libc::c_int {
        server.aof_lastbgrewrite_status = -(1 as libc::c_int);
        server.stat_aofrw_consecutive_failures += 1;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Background AOF rewrite terminated with error\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        if bysignal != 10 as libc::c_int {
            server.aof_lastbgrewrite_status = -(1 as libc::c_int);
            server.stat_aofrw_consecutive_failures += 1;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Background AOF rewrite terminated by signal %d\0" as *const u8
                    as *const libc::c_char,
                bysignal,
            );
        }
    }
    aofRemoveTempFile(server.child_pid);
    if server.aof_state == 2 as libc::c_int {
        sdsfree(server.aof_buf);
        server.aof_buf = sdsempty();
        aofDelTempIncrAofFile();
    }
    server
        .aof_rewrite_time_last = time(0 as *mut time_t) - server.aof_rewrite_time_start;
    server.aof_rewrite_time_start = -(1 as libc::c_int) as time_t;
    if server.aof_state == 2 as libc::c_int {
        server.aof_rewrite_scheduled = 1 as libc::c_int;
    }
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use debug::core::cell::UnsafeCell;
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
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdsmapchars(
        s: sds,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> sds;
    fn __errno_location() -> *mut libc::c_int;
    fn connGetInfo(
        conn: *mut connection,
        buf: *mut libc::c_char,
        buf_len: size_t,
    ) -> *const libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_self() -> pthread_t;
    fn pthread_cancel(__th: pthread_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoul(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulong;
    fn rand() -> libc::c_int;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn _exit(__status: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn setitimer(
        __which: __itimer_which_t,
        __new: *const itimerval,
        __old: *mut itimerval,
    ) -> libc::c_int;
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGetStats(buf: *mut libc::c_char, bufsize: size_t, d: *mut dict);
    fn je_mallctl(
        name: *const libc::c_char,
        oldp: *mut libc::c_void,
        oldlenp: *mut size_t,
        newp: *mut libc::c_void,
        newlen: size_t,
    ) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn stringmatchlen_fuzz_test() -> libc::c_int;
    fn memtoull(p: *const libc::c_char, err: *mut libc::c_int) -> libc::c_ulonglong;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn quicklistRepr(ql: *mut libc::c_uchar, full: libc::c_int);
    fn quicklistisSetPackedThreshold(sz: size_t) -> libc::c_int;
    fn SHA1Init(context: *mut SHA1_CTX);
    fn SHA1Update(context: *mut SHA1_CTX, data: *const libc::c_uchar, len: uint32_t);
    fn SHA1Final(digest: *mut libc::c_uchar, context: *mut SHA1_CTX);
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpSeek(lp: *mut libc::c_uchar, index: libc::c_long) -> *mut libc::c_uchar;
    fn lpRepr(lp: *mut libc::c_uchar);
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
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn modulesCollectInfo(
        info: sds,
        sections_dict: *mut dict,
        for_crash_report: libc::c_int,
        sections: libc::c_int,
    ) -> sds;
    fn addReplyNull(c: *mut client);
    fn addReplyBool(c: *mut client, b: libc::c_int);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplySds(c: *mut client, s: sds);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn addReplyBigNum(c: *mut client, num: *const libc::c_char, len: size_t);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplySetLen(c: *mut client, length: libc::c_long);
    fn addReplyAttributeLen(c: *mut client, length: libc::c_long);
    fn addReplyPushLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn sdsZmallocSize(s: sds) -> size_t;
    fn getStringObjectSdsUsedMemory(o: *mut robj) -> size_t;
    fn catClientInfoString(s: sds, client: *mut client) -> sds;
    fn getAllClientsInfoString(type_0: libc::c_int) -> sds;
    fn rdbPopulateSaveInfo(rsi: *mut rdbSaveInfo) -> *mut rdbSaveInfo;
    fn rdbSavedObjectLen(o: *mut robj, key: *mut robj, dbid: libc::c_int) -> size_t;
    fn rdbSave(
        req: libc::c_int,
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn rdbLoad(
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
        rdbflags: libc::c_int,
    ) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn getLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_long,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getPositiveLongFromObjectOrReply(
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
    fn strEncoding(encoding: libc::c_int) -> *mut libc::c_char;
    fn estimateObjectIdleTime(o: *mut robj) -> libc::c_ulonglong;
    fn replicationFeedSlaves(
        slaves: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn changeReplicationId();
    fn clearReplicationId2();
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn decrRefCount(o: *mut robj);
    fn flushAppendOnlyFile(force: libc::c_int);
    fn loadAppendOnlyFiles(am: *mut aofManifest) -> libc::c_int;
    fn aofLoadManifestFromDisk();
    fn aofManifestFree(am: *mut aofManifest);
    fn aofDelHistoryFiles() -> libc::c_int;
    fn zzlGetScore(sptr: *mut libc::c_uchar) -> libc::c_double;
    fn zzlNext(
        zl: *mut libc::c_uchar,
        eptr: *mut *mut libc::c_uchar,
        sptr: *mut *mut libc::c_uchar,
    );
    fn removeSignalHandlers();
    fn protectClient(c: *mut client);
    fn unprotectClient(c: *mut client);
    fn addReplyStatusFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn listTypeInitIterator(
        subject: *mut robj,
        index: libc::c_long,
        direction: libc::c_uchar,
    ) -> *mut listTypeIterator;
    fn listTypeReleaseIterator(li: *mut listTypeIterator);
    fn listTypeNext(li: *mut listTypeIterator, entry: *mut listTypeEntry) -> libc::c_int;
    fn listTypeGet(entry: *mut listTypeEntry) -> *mut robj;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn rewriteConfig(path: *mut libc::c_char, force_write: libc::c_int) -> libc::c_int;
    fn getConfigDebugInfo() -> sds;
    fn getExpire(db: *mut redisDb, key: *mut robj) -> libc::c_longlong;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn objectCommandLookupOrReply(
        c: *mut client,
        key: *mut robj,
        reply: *mut robj,
    ) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn emptyData(
        dbnum: libc::c_int,
        flags: libc::c_int,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    ) -> libc::c_longlong;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn serverLogRaw(level: libc::c_int, msg: *const libc::c_char);
    fn serverLogFromHandler(level: libc::c_int, msg: *const libc::c_char);
    fn restartServer(flags: libc::c_int, delay: mstime_t) -> libc::c_int;
    fn setTypeInitIterator(subject: *mut robj) -> *mut setTypeIterator;
    fn setTypeReleaseIterator(si: *mut setTypeIterator);
    fn setTypeNextObject(si: *mut setTypeIterator) -> sds;
    fn hashTypeInitIterator(subject: *mut robj) -> *mut hashTypeIterator;
    fn hashTypeReleaseIterator(hi: *mut hashTypeIterator);
    fn hashTypeNext(hi: *mut hashTypeIterator) -> libc::c_int;
    fn hashTypeCurrentObjectNewSds(hi: *mut hashTypeIterator, what: libc::c_int) -> sds;
    fn genRedisInfoString(
        section_dict: *mut dict,
        all_sections: libc::c_int,
        everything: libc::c_int,
    ) -> sds;
    fn releaseInfoSectionDict(sec: *mut dict);
    fn genInfoSectionDict(
        argv: *mut *mut robj,
        argc: libc::c_int,
        defaults: *mut *mut libc::c_char,
        out_all: *mut libc::c_int,
        out_everything: *mut libc::c_int,
    ) -> *mut dict;
    fn memtest_preserving_test(
        m: *mut libc::c_ulong,
        bytes: size_t,
        passes: libc::c_int,
    ) -> libc::c_int;
    fn killIOThreads();
    fn bioKillThreads();
    fn dladdr(__address: *const libc::c_void, __info: *mut Dl_info) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn backtrace(__array: *mut *mut libc::c_void, __size: libc::c_int) -> libc::c_int;
    fn backtrace_symbols_fd(
        __array: *const *mut libc::c_void,
        __size: libc::c_int,
        __fd: libc::c_int,
    );
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
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
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
pub struct sdshdr5 {
    pub flags: libc::c_uchar,
    pub buf: [libc::c_char; 0],
}
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_4 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_4 = 0;
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type C2RustUnnamed_5 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_5 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_5 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_5 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_5 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_5 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_5 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_5 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_5 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_5 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_5 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_5 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_5 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_5 = 236;
pub const _SC_IPV6: C2RustUnnamed_5 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_5 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_5 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_5 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_5 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_5 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_5 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_5 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_5 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_5 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_5 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_5 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_5 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_5 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_5 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_5 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_5 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_5 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_5 = 182;
pub const _SC_TRACE: C2RustUnnamed_5 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_5 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_5 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_5 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_5 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_5 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_5 = 175;
pub const _SC_STREAMS: C2RustUnnamed_5 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_5 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_5 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_5 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_5 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_5 = 169;
pub const _SC_2_PBS: C2RustUnnamed_5 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_5 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_5 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_5 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_5 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_5 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_5 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_5 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_5 = 160;
pub const _SC_SPAWN: C2RustUnnamed_5 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_5 = 158;
pub const _SC_SHELL: C2RustUnnamed_5 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_5 = 156;
pub const _SC_REGEXP: C2RustUnnamed_5 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_5 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_5 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_5 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_5 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_5 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_5 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_5 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_5 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_5 = 146;
pub const _SC_PIPE: C2RustUnnamed_5 = 145;
pub const _SC_FIFO: C2RustUnnamed_5 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_5 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_5 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_5 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_5 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_5 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_5 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_5 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_5 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_5 = 135;
pub const _SC_BASE: C2RustUnnamed_5 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_5 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_5 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_5 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_5 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_5 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_5 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_5 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_5 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_5 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_5 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_5 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_5 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_5 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_5 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_5 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_5 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_5 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_5 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_5 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_5 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_5 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_5 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_5 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_5 = 110;
pub const _SC_NZERO: C2RustUnnamed_5 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_5 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_5 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_5 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_5 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_5 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_5 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_5 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_5 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_5 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_5 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_5 = 98;
pub const _SC_2_UPE: C2RustUnnamed_5 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_5 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_5 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_5 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_5 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_5 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_5 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_5 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_5 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_5 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_5 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_5 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_5 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_5 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_5 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_5 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_5 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_5 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_5 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_5 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_5 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_5 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_5 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_5 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_5 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_5 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_5 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_5 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_5 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_5 = 68;
pub const _SC_THREADS: C2RustUnnamed_5 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_5 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_5 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_5 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_5 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_5 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_5 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_5 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_5 = 60;
pub const _SC_SELECT: C2RustUnnamed_5 = 59;
pub const _SC_POLL: C2RustUnnamed_5 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_5 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_5 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_5 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_5 = 54;
pub const _SC_PII: C2RustUnnamed_5 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_5 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_5 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_5 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_5 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_5 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_5 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_5 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_5 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_5 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_5 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_5 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_5 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_5 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_5 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_5 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_5 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_5 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_5 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_5 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_5 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_5 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_5 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_5 = 30;
pub const _SC_VERSION: C2RustUnnamed_5 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_5 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_5 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_5 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_5 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_5 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_5 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_5 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_5 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_5 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_5 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_5 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_5 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_5 = 16;
pub const _SC_FSYNC: C2RustUnnamed_5 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_5 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_5 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_5 = 12;
pub const _SC_TIMERS: C2RustUnnamed_5 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_5 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_5 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_5 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_5 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_5 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_5 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_5 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_5 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_5 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_5 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_5 = 0;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_15,
    pub _timer: C2RustUnnamed_14,
    pub _rt: C2RustUnnamed_13,
    pub _sigchld: C2RustUnnamed_12,
    pub _sigfault: C2RustUnnamed_9,
    pub _sigpoll: C2RustUnnamed_8,
    pub _sigsys: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub _addr_bnd: C2RustUnnamed_11,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type C2RustUnnamed_16 = libc::c_int;
pub const SI_KERNEL: C2RustUnnamed_16 = 128;
pub const SI_USER: C2RustUnnamed_16 = 0;
pub const SI_QUEUE: C2RustUnnamed_16 = -1;
pub const SI_TIMER: C2RustUnnamed_16 = -2;
pub const SI_MESGQ: C2RustUnnamed_16 = -3;
pub const SI_ASYNCIO: C2RustUnnamed_16 = -4;
pub const SI_SIGIO: C2RustUnnamed_16 = -5;
pub const SI_TKILL: C2RustUnnamed_16 = -6;
pub const SI_DETHREAD: C2RustUnnamed_16 = -7;
pub const SI_ASYNCNL: C2RustUnnamed_16 = -60;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_17,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_17 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub type __itimer_which = libc::c_uint;
pub const ITIMER_PROF: __itimer_which = 2;
pub const ITIMER_VIRTUAL: __itimer_which = 1;
pub const ITIMER_REAL: __itimer_which = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
pub type __itimer_which_t = __itimer_which;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub fault_address: libc::c_ulonglong,
    pub regs: [libc::c_ulonglong; 31],
    pub sp: libc::c_ulonglong,
    pub pc: libc::c_ulonglong,
    pub pstate: libc::c_ulonglong,
    pub __reserved: [libc::c_uchar; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: libc::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}
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
    pub v: C2RustUnnamed_18,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_18 {
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
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
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
pub struct SHA1_CTX {
    pub state: [uint32_t; 5],
    pub count: [uint32_t; 2],
    pub buffer: [libc::c_uchar; 64],
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
pub type robj = redisObject;
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
    pub bs: C2RustUnnamed_22,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_19 {
    pub range: C2RustUnnamed_21,
    pub keynum: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
pub union C2RustUnnamed_22 {
    pub index: C2RustUnnamed_24,
    pub keyword: C2RustUnnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
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
    pub inst_metric: [C2RustUnnamed_25; 5],
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
pub struct C2RustUnnamed_25 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listTypeIterator {
    pub subject: *mut robj,
    pub encoding: libc::c_uchar,
    pub direction: libc::c_uchar,
    pub iter: *mut quicklistIter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listTypeEntry {
    pub li: *mut listTypeIterator,
    pub entry: quicklistEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct setTypeIterator {
    pub subject: *mut robj,
    pub encoding: libc::c_int,
    pub ii: libc::c_int,
    pub di: *mut dictIterator,
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
pub struct Dl_info {
    pub dli_fname: *const libc::c_char,
    pub dli_fbase: *mut libc::c_void,
    pub dli_sname: *const libc::c_char,
    pub dli_saddr: *mut libc::c_void,
}
pub type invalidFunctionWasCalledType = Option::<unsafe extern "C" fn() -> ()>;
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut bug_report_start: libc::c_int = 0 as libc::c_int;
static mut bug_report_start_mutex: pthread_mutex_t = pthread_mutex_t {
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
#[no_mangle]
pub unsafe extern "C" fn xorDigest(
    mut digest: *mut libc::c_uchar,
    mut ptr: *const libc::c_void,
    mut len: size_t,
) {
    let mut ctx: SHA1_CTX = SHA1_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    let mut hash: [libc::c_uchar; 20] = [0; 20];
    let mut j: libc::c_int = 0;
    SHA1Init(&mut ctx);
    SHA1Update(&mut ctx, ptr as *const libc::c_uchar, len as uint32_t);
    SHA1Final(hash.as_mut_ptr(), &mut ctx);
    j = 0 as libc::c_int;
    while j < 20 as libc::c_int {
        let ref mut fresh0 = *digest.offset(j as isize);
        *fresh0 = (*fresh0 as libc::c_int ^ hash[j as usize] as libc::c_int)
            as libc::c_uchar;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xorStringObjectDigest(
    mut digest: *mut libc::c_uchar,
    mut o: *mut robj,
) {
    o = getDecodedObject(o);
    xorDigest(digest, (*o).ptr, sdslen((*o).ptr as sds));
    decrRefCount(o);
}
#[no_mangle]
pub unsafe extern "C" fn mixDigest(
    mut digest: *mut libc::c_uchar,
    mut ptr: *const libc::c_void,
    mut len: size_t,
) {
    let mut ctx: SHA1_CTX = SHA1_CTX {
        state: [0; 5],
        count: [0; 2],
        buffer: [0; 64],
    };
    xorDigest(digest, ptr, len);
    SHA1Init(&mut ctx);
    SHA1Update(&mut ctx, digest, 20 as libc::c_int as uint32_t);
    SHA1Final(digest, &mut ctx);
}
#[no_mangle]
pub unsafe extern "C" fn mixStringObjectDigest(
    mut digest: *mut libc::c_uchar,
    mut o: *mut robj,
) {
    o = getDecodedObject(o);
    mixDigest(digest, (*o).ptr, sdslen((*o).ptr as sds));
    decrRefCount(o);
}
#[no_mangle]
pub unsafe extern "C" fn xorObjectDigest(
    mut db: *mut redisDb,
    mut keyobj: *mut robj,
    mut digest: *mut libc::c_uchar,
    mut o: *mut robj,
) {
    let mut aux: uint32_t = __bswap_32((*o).type_0());
    mixDigest(
        digest,
        &mut aux as *mut uint32_t as *const libc::c_void,
        core::mem::size_of::<uint32_t>() as libc::c_ulong,
    );
    let mut expiretime: libc::c_longlong = getExpire(db, keyobj);
    let mut buf: [libc::c_char; 128] = [0; 128];
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {
        mixStringObjectDigest(digest, o);
    } else if (*o).type_0() as libc::c_int == 1 as libc::c_int {
        let mut li: *mut listTypeIterator = listTypeInitIterator(
            o,
            0 as libc::c_int as libc::c_long,
            1 as libc::c_int as libc::c_uchar,
        );
        let mut entry: listTypeEntry = listTypeEntry {
            li: 0 as *mut listTypeIterator,
            entry: quicklistEntry {
                quicklist: 0 as *const quicklist,
                node: 0 as *mut quicklistNode,
                zi: 0 as *mut libc::c_uchar,
                value: 0 as *mut libc::c_uchar,
                longval: 0,
                sz: 0,
                offset: 0,
            },
        };
        while listTypeNext(li, &mut entry) != 0 {
            let mut eleobj: *mut robj = listTypeGet(&mut entry);
            mixStringObjectDigest(digest, eleobj);
            decrRefCount(eleobj);
        }
        listTypeReleaseIterator(li);
    } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        let mut si: *mut setTypeIterator = setTypeInitIterator(o);
        let mut sdsele: sds = 0 as *mut libc::c_char;
        loop {
            sdsele = setTypeNextObject(si);
            if sdsele.is_null() {
                break;
            }
            xorDigest(digest, sdsele as *const libc::c_void, sdslen(sdsele));
            sdsfree(sdsele);
        }
        setTypeReleaseIterator(si);
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        let mut eledigest: [libc::c_uchar; 20] = [0; 20];
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
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    176 as libc::c_int,
                );
                unreachable!();
            };
            sptr = lpNext(zl, eptr);
            if !sptr.is_null() {} else {
                _serverAssert(
                    b"sptr != NULL\0" as *const u8 as *const libc::c_char,
                    b"debug.c\0" as *const u8 as *const libc::c_char,
                    178 as libc::c_int,
                );
                unreachable!();
            };
            while !eptr.is_null() {
                vstr = lpGetValue(eptr, &mut vlen, &mut vll);
                score = zzlGetScore(sptr);
                memset(
                    eledigest.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    20 as libc::c_int as libc::c_ulong,
                );
                if !vstr.is_null() {
                    mixDigest(
                        eledigest.as_mut_ptr(),
                        vstr as *const libc::c_void,
                        vlen as size_t,
                    );
                } else {
                    ll2string(
                        buf.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                        vll,
                    );
                    mixDigest(
                        eledigest.as_mut_ptr(),
                        buf.as_mut_ptr() as *const libc::c_void,
                        strlen(buf.as_mut_ptr()),
                    );
                }
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"%.17g\0" as *const u8 as *const libc::c_char,
                    score,
                );
                mixDigest(
                    eledigest.as_mut_ptr(),
                    buf.as_mut_ptr() as *const libc::c_void,
                    strlen(buf.as_mut_ptr()),
                );
                xorDigest(
                    digest,
                    eledigest.as_mut_ptr() as *const libc::c_void,
                    20 as libc::c_int as size_t,
                );
                zzlNext(zl, &mut eptr, &mut sptr);
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
                let mut sdsele_0: sds = (*de).key as sds;
                let mut score_0: *mut libc::c_double = (*de).v.val
                    as *mut libc::c_double;
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"%.17g\0" as *const u8 as *const libc::c_char,
                    *score_0,
                );
                memset(
                    eledigest.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    20 as libc::c_int as libc::c_ulong,
                );
                mixDigest(
                    eledigest.as_mut_ptr(),
                    sdsele_0 as *const libc::c_void,
                    sdslen(sdsele_0),
                );
                mixDigest(
                    eledigest.as_mut_ptr(),
                    buf.as_mut_ptr() as *const libc::c_void,
                    strlen(buf.as_mut_ptr()),
                );
                xorDigest(
                    digest,
                    eledigest.as_mut_ptr() as *const libc::c_void,
                    20 as libc::c_int as size_t,
                );
            }
            dictReleaseIterator(di);
        } else {
            _serverPanic(
                b"debug.c\0" as *const u8 as *const libc::c_char,
                214 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        let mut hi: *mut hashTypeIterator = hashTypeInitIterator(o);
        while hashTypeNext(hi) != -(1 as libc::c_int) {
            let mut eledigest_0: [libc::c_uchar; 20] = [0; 20];
            let mut sdsele_1: sds = 0 as *mut libc::c_char;
            memset(
                eledigest_0.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                20 as libc::c_int as libc::c_ulong,
            );
            sdsele_1 = hashTypeCurrentObjectNewSds(hi, 1 as libc::c_int);
            mixDigest(
                eledigest_0.as_mut_ptr(),
                sdsele_1 as *const libc::c_void,
                sdslen(sdsele_1),
            );
            sdsfree(sdsele_1);
            sdsele_1 = hashTypeCurrentObjectNewSds(hi, 2 as libc::c_int);
            mixDigest(
                eledigest_0.as_mut_ptr(),
                sdsele_1 as *const libc::c_void,
                sdslen(sdsele_1),
            );
            sdsfree(sdsele_1);
            xorDigest(
                digest,
                eledigest_0.as_mut_ptr() as *const libc::c_void,
                20 as libc::c_int as size_t,
            );
        }
        hashTypeReleaseIterator(hi);
    } else if (*o).type_0() as libc::c_int == 6 as libc::c_int {
        let mut si_0: streamIterator = streamIterator {
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
            &mut si_0,
            (*o).ptr as *mut stream,
            0 as *mut streamID,
            0 as *mut streamID,
            0 as libc::c_int,
        );
        let mut id: streamID = streamID { ms: 0, seq: 0 };
        let mut numfields: int64_t = 0;
        while streamIteratorGetID(&mut si_0, &mut id, &mut numfields) != 0 {
            let mut itemid: sds = sdscatfmt(
                sdsempty(),
                b"%U.%U\0" as *const u8 as *const libc::c_char,
                id.ms,
                id.seq,
            );
            mixDigest(digest, itemid as *const libc::c_void, sdslen(itemid));
            sdsfree(itemid);
            loop {
                let fresh1 = numfields;
                numfields = numfields - 1;
                if !(fresh1 != 0) {
                    break;
                }
                let mut field: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                let mut field_len: int64_t = 0;
                let mut value_len: int64_t = 0;
                streamIteratorGetField(
                    &mut si_0,
                    &mut field,
                    &mut value,
                    &mut field_len,
                    &mut value_len,
                );
                mixDigest(digest, field as *const libc::c_void, field_len as size_t);
                mixDigest(digest, value as *const libc::c_void, value_len as size_t);
            }
        }
        streamIteratorStop(&mut si_0);
    } else if (*o).type_0() as libc::c_int == 5 as libc::c_int {
        let mut md: RedisModuleDigest = {
            let mut init = RedisModuleDigest {
                o: [
                    0 as libc::c_int as libc::c_uchar,
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
                ],
                x: [
                    0 as libc::c_int as libc::c_uchar,
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
                ],
                key: keyobj,
                dbid: (*db).id,
            };
            init
        };
        let mut mv: *mut moduleValue = (*o).ptr as *mut moduleValue;
        let mut mt: *mut moduleType = (*mv).type_0;
        memset(
            (md.o).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
        );
        memset(
            (md.x).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
        );
        if ((*mt).digest).is_some() {
            ((*mt).digest).expect("non-null function pointer")(&mut md, (*mv).value);
            xorDigest(
                digest,
                (md.x).as_mut_ptr() as *const libc::c_void,
                core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
            );
        }
    } else {
        _serverPanic(
            b"debug.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            b"Unknown object type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    if expiretime != -(1 as libc::c_int) as libc::c_longlong {
        xorDigest(
            digest,
            b"!!expire!!\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            10 as libc::c_int as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn computeDatasetDigest(mut final_0: *mut libc::c_uchar) {
    let mut digest: [libc::c_uchar; 20] = [0; 20];
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut j: libc::c_int = 0;
    let mut aux: uint32_t = 0;
    memset(
        final_0 as *mut libc::c_void,
        0 as libc::c_int,
        20 as libc::c_int as libc::c_ulong,
    );
    j = 0 as libc::c_int;
    while j < server.dbnum {
        let mut db: *mut redisDb = (server.db).offset(j as isize);
        if !(((*(*db).dict).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize])
            == 0 as libc::c_int as libc::c_ulong)
        {
            di = dictGetSafeIterator((*db).dict);
            aux = __bswap_32(j as __uint32_t);
            mixDigest(
                final_0,
                &mut aux as *mut uint32_t as *const libc::c_void,
                core::mem::size_of::<uint32_t>() as libc::c_ulong,
            );
            loop {
                de = dictNext(di);
                if de.is_null() {
                    break;
                }
                let mut key: sds = 0 as *mut libc::c_char;
                let mut keyobj: *mut robj = 0 as *mut robj;
                let mut o: *mut robj = 0 as *mut robj;
                memset(
                    digest.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    20 as libc::c_int as libc::c_ulong,
                );
                key = (*de).key as sds;
                keyobj = createStringObject(key as *const libc::c_char, sdslen(key));
                mixDigest(digest.as_mut_ptr(), key as *const libc::c_void, sdslen(key));
                o = (*de).v.val as *mut robj;
                xorObjectDigest(db, keyobj, digest.as_mut_ptr(), o);
                xorDigest(
                    final_0,
                    digest.as_mut_ptr() as *const libc::c_void,
                    20 as libc::c_int as size_t,
                );
                decrRefCount(keyobj);
            }
            dictReleaseIterator(di);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn mallctl_int(
    mut c: *mut client,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) {
    let mut ret: libc::c_int = 0;
    let mut old: int64_t = 0 as libc::c_int as int64_t;
    let mut val: int64_t = 0;
    if argc > 1 as libc::c_int {
        let mut ll: libc::c_longlong = 0;
        if getLongLongFromObjectOrReply(
            c,
            *argv.offset(1 as libc::c_int as isize),
            &mut ll,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        val = ll as int64_t;
    }
    let mut sz: size_t = core::mem::size_of::<int64_t>() as libc::c_ulong;
    while sz > 0 as libc::c_int as libc::c_ulong {
        ret = je_mallctl(
            (**argv.offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
            &mut old as *mut int64_t as *mut libc::c_void,
            &mut sz,
            (if argc > 1 as libc::c_int {
                &mut val as *mut int64_t
            } else {
                0 as *mut int64_t
            }) as *mut libc::c_void,
            if argc > 1 as libc::c_int { sz } else { 0 as libc::c_int as libc::c_ulong },
        );
        if ret != 0 {
            if ret == 1 as libc::c_int && argc > 1 as libc::c_int {
                ret = je_mallctl(
                    (**argv.offset(0 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    0 as *mut libc::c_void,
                    0 as *mut size_t,
                    &mut val as *mut int64_t as *mut libc::c_void,
                    sz,
                );
                if ret == 0 {
                    addReply(c, shared.ok);
                    return;
                }
            }
            if ret == 22 as libc::c_int {
                sz = (sz as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
            } else {
                addReplyErrorFormat(
                    c,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    strerror(ret),
                );
                return;
            }
        } else {
            addReplyLongLong(c, old as libc::c_longlong);
            return;
        }
    }
    addReplyErrorFormat(
        c,
        b"%s\0" as *const u8 as *const libc::c_char,
        strerror(22 as libc::c_int),
    );
}
#[no_mangle]
pub unsafe extern "C" fn mallctl_string(
    mut c: *mut client,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) {
    let mut rret: libc::c_int = 0;
    let mut wret: libc::c_int = 0;
    let mut old: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sz: size_t = core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong;
    rret = je_mallctl(
        (**argv.offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
        &mut old as *mut *mut libc::c_char as *mut libc::c_void,
        &mut sz,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    if rret != 0 {
        if !(rret == 1 as libc::c_int && argc > 1 as libc::c_int) {
            addReplyErrorFormat(
                c,
                b"%s\0" as *const u8 as *const libc::c_char,
                strerror(rret),
            );
            return;
        }
    }
    if argc > 1 as libc::c_int {
        let mut val: *mut libc::c_char = (**argv.offset(1 as libc::c_int as isize)).ptr
            as *mut libc::c_char;
        let mut valref: *mut *mut libc::c_char = &mut val;
        if strcmp(val, b"VOID\0" as *const u8 as *const libc::c_char) == 0 {
            valref = 0 as *mut *mut libc::c_char;
            sz = 0 as libc::c_int as size_t;
        }
        wret = je_mallctl(
            (**argv.offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
            0 as *mut libc::c_void,
            0 as *mut size_t,
            valref as *mut libc::c_void,
            sz,
        );
    }
    if rret == 0 {
        addReplyBulkCString(c, old);
    } else if wret != 0 {
        addReplyErrorFormat(
            c,
            b"%s\0" as *const u8 as *const libc::c_char,
            strerror(wret),
        );
    } else {
        addReply(c, shared.ok);
    };
}
#[no_mangle]
pub unsafe extern "C" fn debugCommand(mut c: *mut client) {
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 102] = [
            b"AOF-FLUSH-SLEEP <microsec>\0" as *const u8 as *const libc::c_char,
            b"    Server will sleep before flushing the AOF, this is used for testing.\0"
                as *const u8 as *const libc::c_char,
            b"ASSERT\0" as *const u8 as *const libc::c_char,
            b"    Crash by assertion failed.\0" as *const u8 as *const libc::c_char,
            b"CHANGE-REPL-ID\0" as *const u8 as *const libc::c_char,
            b"    Change the replication IDs of the instance.\0" as *const u8
                as *const libc::c_char,
            b"    Dangerous: should be used only for testing the replication subsystem.\0"
                as *const u8 as *const libc::c_char,
            b"CONFIG-REWRITE-FORCE-ALL\0" as *const u8 as *const libc::c_char,
            b"    Like CONFIG REWRITE but writes all configuration options, including\0"
                as *const u8 as *const libc::c_char,
            b"    keywords not listed in original configuration file or default values.\0"
                as *const u8 as *const libc::c_char,
            b"CRASH-AND-RECOVER [<milliseconds>]\0" as *const u8 as *const libc::c_char,
            b"    Hard crash and restart after a <milliseconds> delay (default 0).\0"
                as *const u8 as *const libc::c_char,
            b"DIGEST\0" as *const u8 as *const libc::c_char,
            b"    Output a hex signature representing the current DB content.\0"
                as *const u8 as *const libc::c_char,
            b"DIGEST-VALUE <key> [<key> ...]\0" as *const u8 as *const libc::c_char,
            b"    Output a hex signature of the values of all the specified keys.\0"
                as *const u8 as *const libc::c_char,
            b"ERROR <string>\0" as *const u8 as *const libc::c_char,
            b"    Return a Redis protocol error with <string> as message. Useful for clients\0"
                as *const u8 as *const libc::c_char,
            b"    unit tests to simulate Redis errors.\0" as *const u8
                as *const libc::c_char,
            b"LEAK <string>\0" as *const u8 as *const libc::c_char,
            b"    Create a memory leak of the input string.\0" as *const u8
                as *const libc::c_char,
            b"LOG <message>\0" as *const u8 as *const libc::c_char,
            b"    Write <message> to the server log.\0" as *const u8
                as *const libc::c_char,
            b"HTSTATS <dbid>\0" as *const u8 as *const libc::c_char,
            b"    Return hash table statistics of the specified Redis database.\0"
                as *const u8 as *const libc::c_char,
            b"HTSTATS-KEY <key>\0" as *const u8 as *const libc::c_char,
            b"    Like HTSTATS but for the hash table stored at <key>'s value.\0"
                as *const u8 as *const libc::c_char,
            b"LOADAOF\0" as *const u8 as *const libc::c_char,
            b"    Flush the AOF buffers on disk and reload the AOF in memory.\0"
                as *const u8 as *const libc::c_char,
            b"REPLICATE <string>\0" as *const u8 as *const libc::c_char,
            b"    Replicates the provided string to replicas, allowing data divergence.\0"
                as *const u8 as *const libc::c_char,
            b"MALLCTL <key> [<val>]\0" as *const u8 as *const libc::c_char,
            b"    Get or set a malloc tuning integer.\0" as *const u8
                as *const libc::c_char,
            b"MALLCTL-STR <key> [<val>]\0" as *const u8 as *const libc::c_char,
            b"    Get or set a malloc tuning string.\0" as *const u8
                as *const libc::c_char,
            b"OBJECT <key>\0" as *const u8 as *const libc::c_char,
            b"    Show low level info about `key` and associated value.\0" as *const u8
                as *const libc::c_char,
            b"DROP-CLUSTER-PACKET-FILTER <packet-type>\0" as *const u8
                as *const libc::c_char,
            b"    Drop all packets that match the filtered type. Set to -1 allow all packets.\0"
                as *const u8 as *const libc::c_char,
            b"OOM\0" as *const u8 as *const libc::c_char,
            b"    Crash the server simulating an out-of-memory error.\0" as *const u8
                as *const libc::c_char,
            b"PANIC\0" as *const u8 as *const libc::c_char,
            b"    Crash the server simulating a panic.\0" as *const u8
                as *const libc::c_char,
            b"POPULATE <count> [<prefix>] [<size>]\0" as *const u8
                as *const libc::c_char,
            b"    Create <count> string keys named key:<num>. If <prefix> is specified then\0"
                as *const u8 as *const libc::c_char,
            b"    it is used instead of the 'key' prefix. These are not propagated to\0"
                as *const u8 as *const libc::c_char,
            b"    replicas. Cluster slots are not respected so keys not belonging to the\0"
                as *const u8 as *const libc::c_char,
            b"    current node can be created in cluster mode.\0" as *const u8
                as *const libc::c_char,
            b"PROTOCOL <type>\0" as *const u8 as *const libc::c_char,
            b"    Reply with a test value of the specified type. <type> can be: string,\0"
                as *const u8 as *const libc::c_char,
            b"    integer, double, bignum, null, array, set, map, attrib, push, verbatim,\0"
                as *const u8 as *const libc::c_char,
            b"    true, false.\0" as *const u8 as *const libc::c_char,
            b"RELOAD [option ...]\0" as *const u8 as *const libc::c_char,
            b"    Save the RDB on disk and reload it back to memory. Valid <option> values:\0"
                as *const u8 as *const libc::c_char,
            b"    * MERGE: conflicting keys will be loaded from RDB.\0" as *const u8
                as *const libc::c_char,
            b"    * NOFLUSH: the existing database will not be removed before load, but\0"
                as *const u8 as *const libc::c_char,
            b"      conflicting keys will generate an exception and kill the server.\0"
                as *const u8 as *const libc::c_char,
            b"    * NOSAVE: the database will be loaded from an existing RDB file.\0"
                as *const u8 as *const libc::c_char,
            b"    Examples:\0" as *const u8 as *const libc::c_char,
            b"    * DEBUG RELOAD: verify that the server is able to persist, flush and reload\0"
                as *const u8 as *const libc::c_char,
            b"      the database.\0" as *const u8 as *const libc::c_char,
            b"    * DEBUG RELOAD NOSAVE: replace the current database with the contents of an\0"
                as *const u8 as *const libc::c_char,
            b"      existing RDB file.\0" as *const u8 as *const libc::c_char,
            b"    * DEBUG RELOAD NOSAVE NOFLUSH MERGE: add the contents of an existing RDB\0"
                as *const u8 as *const libc::c_char,
            b"      file to the database.\0" as *const u8 as *const libc::c_char,
            b"RESTART [<milliseconds>]\0" as *const u8 as *const libc::c_char,
            b"    Graceful restart: save config, db, restart after a <milliseconds> delay (default 0).\0"
                as *const u8 as *const libc::c_char,
            b"SDSLEN <key>\0" as *const u8 as *const libc::c_char,
            b"    Show low level SDS string info representing `key` and value.\0"
                as *const u8 as *const libc::c_char,
            b"SEGFAULT\0" as *const u8 as *const libc::c_char,
            b"    Crash the server with sigsegv.\0" as *const u8 as *const libc::c_char,
            b"SET-ACTIVE-EXPIRE <0|1>\0" as *const u8 as *const libc::c_char,
            b"    Setting it to 0 disables expiring keys in background when they are not\0"
                as *const u8 as *const libc::c_char,
            b"    accessed (otherwise the Redis behavior). Setting it to 1 reenables back the\0"
                as *const u8 as *const libc::c_char,
            b"    default.\0" as *const u8 as *const libc::c_char,
            b"QUICKLIST-PACKED-THRESHOLD <size>\0" as *const u8 as *const libc::c_char,
            b"    Sets the threshold for elements to be inserted as plain vs packed nodes\0"
                as *const u8 as *const libc::c_char,
            b"    Default value is 1GB, allows values up to 4GB. Setting to 0 restores to default.\0"
                as *const u8 as *const libc::c_char,
            b"SET-SKIP-CHECKSUM-VALIDATION <0|1>\0" as *const u8 as *const libc::c_char,
            b"    Enables or disables checksum checks for RDB files and RESTORE's payload.\0"
                as *const u8 as *const libc::c_char,
            b"SLEEP <seconds>\0" as *const u8 as *const libc::c_char,
            b"    Stop the server for <seconds>. Decimals allowed.\0" as *const u8
                as *const libc::c_char,
            b"STRINGMATCH-TEST\0" as *const u8 as *const libc::c_char,
            b"    Run a fuzz tester against the stringmatchlen() function.\0"
                as *const u8 as *const libc::c_char,
            b"STRUCTSIZE\0" as *const u8 as *const libc::c_char,
            b"    Return the size of different Redis core C structures.\0" as *const u8
                as *const libc::c_char,
            b"LISTPACK <key>\0" as *const u8 as *const libc::c_char,
            b"    Show low level info about the listpack encoding of <key>.\0"
                as *const u8 as *const libc::c_char,
            b"QUICKLIST <key> [<0|1>]\0" as *const u8 as *const libc::c_char,
            b"    Show low level info about the quicklist encoding of <key>.\0"
                as *const u8 as *const libc::c_char,
            b"    The optional argument (0 by default) sets the level of detail\0"
                as *const u8 as *const libc::c_char,
            b"CLIENT-EVICTION\0" as *const u8 as *const libc::c_char,
            b"    Show low level client eviction pools info (maxmemory-clients).\0"
                as *const u8 as *const libc::c_char,
            b"PAUSE-CRON <0|1>\0" as *const u8 as *const libc::c_char,
            b"    Stop periodic cron job processing.\0" as *const u8
                as *const libc::c_char,
            b"REPLYBUFFER PEAK-RESET-TIME <NEVER||RESET|time>\0" as *const u8
                as *const libc::c_char,
            b"    Sets the time (in milliseconds) to wait between client reply buffer peak resets.\0"
                as *const u8 as *const libc::c_char,
            b"    In case NEVER is provided the last observed peak will never be reset\0"
                as *const u8 as *const libc::c_char,
            b"    In case RESET is provided the peak reset time will be restored to the default value\0"
                as *const u8 as *const libc::c_char,
            b"REPLYBUFFER RESIZING <0|1>\0" as *const u8 as *const libc::c_char,
            b"    Enable or disable the replay buffer resize cron job\0" as *const u8
                as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"segfault\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut p: *mut libc::c_char = mmap(
            0 as *mut libc::c_void,
            4096 as libc::c_int as size_t,
            0x1 as libc::c_int,
            0x2 as libc::c_int | 0x20 as libc::c_int,
            -(1 as libc::c_int),
            0 as libc::c_int as __off64_t,
        ) as *mut libc::c_char;
        *p = 'x' as i32 as libc::c_char;
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"panic\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        _serverPanic(
            b"debug.c\0" as *const u8 as *const libc::c_char,
            503 as libc::c_int,
            b"DEBUG PANIC called at Unix time %lld\0" as *const u8
                as *const libc::c_char,
            time(0 as *mut time_t) as libc::c_longlong,
        );
        unreachable!();
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"restart\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"crash-and-recover\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut delay: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        if (*c).argc >= 3 as libc::c_int {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset(2 as libc::c_int as isize),
                &mut delay,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if delay < 0 as libc::c_int as libc::c_longlong {
                delay = 0 as libc::c_int as libc::c_longlong;
            }
        }
        let mut flags: libc::c_int = if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"restart\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        restartServer(flags, delay);
        addReplyError(
            c,
            b"failed to restart the server. Check server logs.\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"oom\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut ptr: *mut libc::c_void = zmalloc(
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong),
        );
        zfree(ptr);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"assert\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if 1 as libc::c_int == 2 as libc::c_int {} else {
            _serverAssertWithInfo(
                c,
                *((*c).argv).offset(0 as libc::c_int as isize),
                b"1 == 2\0" as *const u8 as *const libc::c_char,
                b"debug.c\0" as *const u8 as *const libc::c_char,
                523 as libc::c_int,
            );
            unreachable!();
        };
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"log\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"DEBUG LOG: %s\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"leak\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        sdsdup((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"reload\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        let mut flush: libc::c_int = 1 as libc::c_int;
        let mut save: libc::c_int = 1 as libc::c_int;
        let mut flags_0: libc::c_int = 0 as libc::c_int;
        let mut j: libc::c_int = 2 as libc::c_int;
        while j < (*c).argc {
            let mut opt: *mut libc::c_char = (**((*c).argv).offset(j as isize)).ptr
                as *mut libc::c_char;
            if strcasecmp(opt, b"MERGE\0" as *const u8 as *const libc::c_char) == 0 {
                flags_0 |= (1 as libc::c_int) << 2 as libc::c_int;
            } else if strcasecmp(opt, b"NOFLUSH\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flush = 0 as libc::c_int;
            } else if strcasecmp(opt, b"NOSAVE\0" as *const u8 as *const libc::c_char)
                == 0
            {
                save = 0 as libc::c_int;
            } else {
                addReplyError(
                    c,
                    b"DEBUG RELOAD only supports the MERGE, NOFLUSH and NOSAVE options.\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            j += 1;
        }
        if save != 0 {
            let mut rsi: rdbSaveInfo = rdbSaveInfo {
                repl_stream_db: 0,
                repl_id_is_set: 0,
                repl_id: [0; 41],
                repl_offset: 0,
            };
            let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
            rsiptr = rdbPopulateSaveInfo(&mut rsi);
            if rdbSave(0 as libc::c_int, server.rdb_filename, rsiptr) != 0 as libc::c_int
            {
                addReplyErrorObject(c, shared.err);
                return;
            }
        }
        if flush != 0 {
            emptyData(-(1 as libc::c_int), 0 as libc::c_int, None);
        }
        protectClient(c);
        let mut ret: libc::c_int = rdbLoad(
            server.rdb_filename,
            0 as *mut rdbSaveInfo,
            flags_0,
        );
        unprotectClient(c);
        if ret != 0 as libc::c_int {
            addReplyError(
                c,
                b"Error trying to load the RDB dump\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"DB reloaded by DEBUG RELOAD\0" as *const u8 as *const libc::c_char,
            );
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"loadaof\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if server.aof_state != 0 as libc::c_int {
            flushAppendOnlyFile(1 as libc::c_int);
        }
        emptyData(-(1 as libc::c_int), 0 as libc::c_int, None);
        protectClient(c);
        if !(server.aof_manifest).is_null() {
            aofManifestFree(server.aof_manifest);
        }
        aofLoadManifestFromDisk();
        aofDelHistoryFiles();
        let mut ret_0: libc::c_int = loadAppendOnlyFiles(server.aof_manifest);
        if ret_0 != 0 as libc::c_int && ret_0 != 2 as libc::c_int {
            exit(1 as libc::c_int);
        }
        unprotectClient(c);
        server.dirty = 0 as libc::c_int as libc::c_longlong;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Append Only File loaded by DEBUG LOADAOF\0" as *const u8
                    as *const libc::c_char,
            );
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"drop-cluster-packet-filter\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut packet_type: libc::c_long = 0;
        if getLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut packet_type,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        server.cluster_drop_packet_filter = packet_type as libc::c_int;
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"object\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut val: *mut robj = 0 as *mut robj;
        let mut strenc: *mut libc::c_char = 0 as *mut libc::c_char;
        de = dictFind(
            (*(*c).db).dict,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
        );
        if de.is_null() {
            addReplyErrorObject(c, shared.nokeyerr);
            return;
        }
        val = (*de).v.val as *mut robj;
        strenc = strEncoding((*val).encoding() as libc::c_int);
        let mut extra: [libc::c_char; 138] = [
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
        if (*val).encoding() as libc::c_int == 9 as libc::c_int {
            let mut nextra: *mut libc::c_char = extra.as_mut_ptr();
            let mut remaining: libc::c_int = core::mem::size_of::<
                [libc::c_char; 138],
            >() as libc::c_ulong as libc::c_int;
            let mut ql: *mut quicklist = (*val).ptr as *mut quicklist;
            let mut used: libc::c_int = snprintf(
                nextra,
                remaining as libc::c_ulong,
                b" ql_nodes:%lu\0" as *const u8 as *const libc::c_char,
                (*ql).len,
            );
            nextra = nextra.offset(used as isize);
            remaining -= used;
            let mut avg: libc::c_double = (*ql).count as libc::c_double
                / (*ql).len as libc::c_double;
            used = snprintf(
                nextra,
                remaining as libc::c_ulong,
                b" ql_avg_node:%.2f\0" as *const u8 as *const libc::c_char,
                avg,
            );
            nextra = nextra.offset(used as isize);
            remaining -= used;
            used = snprintf(
                nextra,
                remaining as libc::c_ulong,
                b" ql_listpack_max:%d\0" as *const u8 as *const libc::c_char,
                (*ql).fill(),
            );
            nextra = nextra.offset(used as isize);
            remaining -= used;
            let mut compressed: libc::c_int = ((*ql).compress() as libc::c_int
                != 0 as libc::c_int) as libc::c_int;
            used = snprintf(
                nextra,
                remaining as libc::c_ulong,
                b" ql_compressed:%d\0" as *const u8 as *const libc::c_char,
                compressed,
            );
            nextra = nextra.offset(used as isize);
            remaining -= used;
            let mut sz: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
            let mut node: *mut quicklistNode = (*ql).head;
            while !node.is_null() {
                sz = sz.wrapping_add((*node).sz);
                node = (*node).next;
            }
            used = snprintf(
                nextra,
                remaining as libc::c_ulong,
                b" ql_uncompressed_size:%lu\0" as *const u8 as *const libc::c_char,
                sz,
            );
            nextra = nextra.offset(used as isize);
            remaining -= used;
        }
        addReplyStatusFormat(
            c,
            b"Value at:%p refcount:%d encoding:%s serializedlength:%zu lru:%d lru_seconds_idle:%llu%s\0"
                as *const u8 as *const libc::c_char,
            val as *mut libc::c_void,
            (*val).refcount,
            strenc,
            rdbSavedObjectLen(
                val,
                *((*c).argv).offset(2 as libc::c_int as isize),
                (*(*c).db).id,
            ),
            (*val).lru() as libc::c_int,
            (estimateObjectIdleTime(val))
                .wrapping_div(1000 as libc::c_int as libc::c_ulonglong),
            extra.as_mut_ptr(),
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"sdslen\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
        let mut val_0: *mut robj = 0 as *mut robj;
        let mut key: sds = 0 as *mut libc::c_char;
        de_0 = dictFind(
            (*(*c).db).dict,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
        );
        if de_0.is_null() {
            addReplyErrorObject(c, shared.nokeyerr);
            return;
        }
        val_0 = (*de_0).v.val as *mut robj;
        key = (*de_0).key as sds;
        if (*val_0).type_0() as libc::c_int != 0 as libc::c_int
            || !((*val_0).encoding() as libc::c_int == 0 as libc::c_int
                || (*val_0).encoding() as libc::c_int == 8 as libc::c_int)
        {
            addReplyError(
                c,
                b"Not an sds encoded string.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            addReplyStatusFormat(
                c,
                b"key_sds_len:%lld, key_sds_avail:%lld, key_zmalloc: %lld, val_sds_len:%lld, val_sds_avail:%lld, val_zmalloc: %lld\0"
                    as *const u8 as *const libc::c_char,
                sdslen(key) as libc::c_longlong,
                sdsavail(key) as libc::c_longlong,
                sdsZmallocSize(key) as libc::c_longlong,
                sdslen((*val_0).ptr as sds) as libc::c_longlong,
                sdsavail((*val_0).ptr as sds) as libc::c_longlong,
                getStringObjectSdsUsedMemory(val_0) as libc::c_longlong,
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"listpack\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut o: *mut robj = 0 as *mut robj;
        o = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.nokeyerr,
        );
        if o.is_null() {
            return;
        }
        if (*o).encoding() as libc::c_int != 11 as libc::c_int {
            addReplyError(
                c,
                b"Not a listpack encoded object.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            lpRepr((*o).ptr as *mut libc::c_uchar);
            addReplyStatus(
                c,
                b"Listpack structure printed on stdout\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"quicklist\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 3 as libc::c_int || (*c).argc == 4 as libc::c_int)
    {
        let mut o_0: *mut robj = 0 as *mut robj;
        o_0 = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.nokeyerr,
        );
        if o_0.is_null() {
            return;
        }
        let mut full: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 4 as libc::c_int {
            full = atoi(
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
            );
        }
        if (*o_0).encoding() as libc::c_int != 9 as libc::c_int {
            addReplyError(
                c,
                b"Not a quicklist encoded object.\0" as *const u8 as *const libc::c_char,
            );
        } else {
            quicklistRepr((*o_0).ptr as *mut libc::c_uchar, full);
            addReplyStatus(
                c,
                b"Quicklist structure printed on stdout\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"populate\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int && (*c).argc <= 5 as libc::c_int
    {
        let mut keys: libc::c_long = 0;
        let mut j_0: libc::c_long = 0;
        let mut key_0: *mut robj = 0 as *mut robj;
        let mut val_1: *mut robj = 0 as *mut robj;
        let mut buf: [libc::c_char; 128] = [0; 128];
        if getPositiveLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut keys,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        dictExpand((*(*c).db).dict, keys as libc::c_ulong);
        let mut valsize: libc::c_long = 0 as libc::c_int as libc::c_long;
        if (*c).argc == 5 as libc::c_int
            && getPositiveLongFromObjectOrReply(
                c,
                *((*c).argv).offset(4 as libc::c_int as isize),
                &mut valsize,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
        {
            return;
        }
        j_0 = 0 as libc::c_int as libc::c_long;
        while j_0 < keys {
            snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"%s:%lu\0" as *const u8 as *const libc::c_char,
                if (*c).argc == 3 as libc::c_int {
                    b"key\0" as *const u8 as *const libc::c_char
                } else {
                    (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                        as *mut libc::c_char as *const libc::c_char
                },
                j_0,
            );
            key_0 = createStringObject(buf.as_mut_ptr(), strlen(buf.as_mut_ptr()));
            if !(lookupKeyWrite((*c).db, key_0)).is_null() {
                decrRefCount(key_0);
            } else {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    b"value:%lu\0" as *const u8 as *const libc::c_char,
                    j_0,
                );
                if valsize == 0 as libc::c_int as libc::c_long {
                    val_1 = createStringObject(
                        buf.as_mut_ptr(),
                        strlen(buf.as_mut_ptr()),
                    );
                } else {
                    let mut buflen: libc::c_int = strlen(buf.as_mut_ptr())
                        as libc::c_int;
                    val_1 = createStringObject(
                        0 as *const libc::c_char,
                        valsize as size_t,
                    );
                    memcpy(
                        (*val_1).ptr,
                        buf.as_mut_ptr() as *const libc::c_void,
                        (if valsize <= buflen as libc::c_long {
                            valsize
                        } else {
                            buflen as libc::c_long
                        }) as libc::c_ulong,
                    );
                }
                dbAdd((*c).db, key_0, val_1);
                signalModifiedKey(c, (*c).db, key_0);
                decrRefCount(key_0);
            }
            j_0 += 1;
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"digest\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut digest: [libc::c_uchar; 20] = [0; 20];
        let mut d: sds = sdsempty();
        computeDatasetDigest(digest.as_mut_ptr());
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 20 as libc::c_int {
            d = sdscatprintf(
                d,
                b"%02x\0" as *const u8 as *const libc::c_char,
                digest[i as usize] as libc::c_int,
            );
            i += 1;
        }
        addReplyStatus(c, d as *const libc::c_char);
        sdsfree(d);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"digest-value\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 2 as libc::c_int
    {
        addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
        let mut j_1: libc::c_int = 2 as libc::c_int;
        while j_1 < (*c).argc {
            let mut digest_0: [libc::c_uchar; 20] = [0; 20];
            memset(
                digest_0.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                20 as libc::c_int as libc::c_ulong,
            );
            let mut de_1: *mut dictEntry = 0 as *mut dictEntry;
            de_1 = dictFind((*(*c).db).dict, (**((*c).argv).offset(j_1 as isize)).ptr);
            let mut o_1: *mut robj = (if de_1.is_null() {
                0 as *mut libc::c_void
            } else {
                (*de_1).v.val
            }) as *mut robj;
            if !o_1.is_null() {
                xorObjectDigest(
                    (*c).db,
                    *((*c).argv).offset(j_1 as isize),
                    digest_0.as_mut_ptr(),
                    o_1,
                );
            }
            let mut d_0: sds = sdsempty();
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < 20 as libc::c_int {
                d_0 = sdscatprintf(
                    d_0,
                    b"%02x\0" as *const u8 as *const libc::c_char,
                    digest_0[i_0 as usize] as libc::c_int,
                );
                i_0 += 1;
            }
            addReplyStatus(c, d_0 as *const libc::c_char);
            sdsfree(d_0);
            j_1 += 1;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"protocol\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut name: *mut libc::c_char = (**((*c).argv)
            .offset(2 as libc::c_int as isize))
            .ptr as *mut libc::c_char;
        if strcasecmp(name, b"string\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyBulkCString(c, b"Hello World\0" as *const u8 as *const libc::c_char);
        } else if strcasecmp(name, b"integer\0" as *const u8 as *const libc::c_char) == 0
        {
            addReplyLongLong(c, 12345 as libc::c_int as libc::c_longlong);
        } else if strcasecmp(name, b"double\0" as *const u8 as *const libc::c_char) == 0
        {
            addReplyDouble(c, 3.141f64);
        } else if strcasecmp(name, b"bignum\0" as *const u8 as *const libc::c_char) == 0
        {
            addReplyBigNum(
                c,
                b"1234567999999999999999999999999999999\0" as *const u8
                    as *const libc::c_char,
                37 as libc::c_int as size_t,
            );
        } else if strcasecmp(name, b"null\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyNull(c);
        } else if strcasecmp(name, b"array\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyArrayLen(c, 3 as libc::c_int as libc::c_long);
            let mut j_2: libc::c_int = 0 as libc::c_int;
            while j_2 < 3 as libc::c_int {
                addReplyLongLong(c, j_2 as libc::c_longlong);
                j_2 += 1;
            }
        } else if strcasecmp(name, b"set\0" as *const u8 as *const libc::c_char) == 0 {
            addReplySetLen(c, 3 as libc::c_int as libc::c_long);
            let mut j_3: libc::c_int = 0 as libc::c_int;
            while j_3 < 3 as libc::c_int {
                addReplyLongLong(c, j_3 as libc::c_longlong);
                j_3 += 1;
            }
        } else if strcasecmp(name, b"map\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
            let mut j_4: libc::c_int = 0 as libc::c_int;
            while j_4 < 3 as libc::c_int {
                addReplyLongLong(c, j_4 as libc::c_longlong);
                addReplyBool(c, (j_4 == 1 as libc::c_int) as libc::c_int);
                j_4 += 1;
            }
        } else if strcasecmp(name, b"attrib\0" as *const u8 as *const libc::c_char) == 0
        {
            if (*c).resp >= 3 as libc::c_int {
                addReplyAttributeLen(c, 1 as libc::c_int as libc::c_long);
                addReplyBulkCString(
                    c,
                    b"key-popularity\0" as *const u8 as *const libc::c_char,
                );
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"key:123\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(c, 90 as libc::c_int as libc::c_longlong);
            }
            addReplyBulkCString(
                c,
                b"Some real reply following the attribute\0" as *const u8
                    as *const libc::c_char,
            );
        } else if strcasecmp(name, b"push\0" as *const u8 as *const libc::c_char) == 0 {
            if (*c).resp < 3 as libc::c_int {
                addReplyError(
                    c,
                    b"RESP2 is not supported by this command\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            addReplyPushLen(c, 2 as libc::c_int as libc::c_long);
            addReplyBulkCString(
                c,
                b"server-cpu-usage\0" as *const u8 as *const libc::c_char,
            );
            addReplyLongLong(c, 42 as libc::c_int as libc::c_longlong);
            addReplyBulkCString(
                c,
                b"Some real reply following the push reply\0" as *const u8
                    as *const libc::c_char,
            );
        } else if strcasecmp(name, b"true\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyBool(c, 1 as libc::c_int);
        } else if strcasecmp(name, b"false\0" as *const u8 as *const libc::c_char) == 0 {
            addReplyBool(c, 0 as libc::c_int);
        } else if strcasecmp(name, b"verbatim\0" as *const u8 as *const libc::c_char)
            == 0
        {
            addReplyVerbatim(
                c,
                b"This is a verbatim\nstring\0" as *const u8 as *const libc::c_char,
                25 as libc::c_int as size_t,
                b"txt\0" as *const u8 as *const libc::c_char,
            );
        } else {
            addReplyError(
                c,
                b"Wrong protocol type name. Please use one of the following: string|integer|double|bignum|null|array|set|map|attrib|push|verbatim|true|false\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"sleep\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut dtime: libc::c_double = strtod(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            0 as *mut *mut libc::c_char,
        );
        let mut utime: libc::c_longlong = (dtime
            * 1000000 as libc::c_int as libc::c_double) as libc::c_longlong;
        let mut tv: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
        tv.tv_sec = (utime / 1000000 as libc::c_int as libc::c_longlong) as __time_t;
        tv
            .tv_nsec = (utime % 1000000 as libc::c_int as libc::c_longlong
            * 1000 as libc::c_int as libc::c_longlong) as __syscall_slong_t;
        nanosleep(&mut tv, 0 as *mut timespec);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"set-active-expire\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        server
            .active_expire_enabled = atoi(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"quicklist-packed-threshold\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut memerr: libc::c_int = 0;
        let mut sz_0: libc::c_ulonglong = memtoull(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            &mut memerr,
        );
        if memerr != 0 || quicklistisSetPackedThreshold(sz_0 as size_t) == 0 {
            addReplyError(
                c,
                b"argument must be a memory value bigger than 1 and smaller than 4gb\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            addReply(c, shared.ok);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"set-skip-checksum-validation\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        server
            .skip_checksum_validation = atoi(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"aof-flush-sleep\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        server
            .aof_flush_sleep = atoi(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"replicate\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int
    {
        replicationFeedSlaves(
            server.slaves,
            server.slaveseldb,
            ((*c).argv).offset(2 as libc::c_int as isize),
            (*c).argc - 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"error\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut errstr: sds = sdsnewlen(
            b"-\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        errstr = sdscatsds(
            errstr,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        );
        errstr = sdsmapchars(
            errstr,
            b"\n\r\0" as *const u8 as *const libc::c_char,
            b"  \0" as *const u8 as *const libc::c_char,
            2 as libc::c_int as size_t,
        );
        errstr = sdscatlen(
            errstr,
            b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            2 as libc::c_int as size_t,
        );
        addReplySds(c, errstr);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"structsize\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut sizes: sds = sdsempty();
        sizes = sdscatprintf(
            sizes,
            b"bits:%d \0" as *const u8 as *const libc::c_char,
            if core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                == 8 as libc::c_int as libc::c_ulong
            {
                64 as libc::c_int
            } else {
                32 as libc::c_int
            },
        );
        sizes = sdscatprintf(
            sizes,
            b"robj:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<robj>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"dictentry:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<dictEntry>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"sdshdr5:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<sdshdr5>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"sdshdr8:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<sdshdr8>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"sdshdr16:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<sdshdr16>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"sdshdr32:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<sdshdr32>() as libc::c_ulong as libc::c_int,
        );
        sizes = sdscatprintf(
            sizes,
            b"sdshdr64:%d \0" as *const u8 as *const libc::c_char,
            core::mem::size_of::<sdshdr64>() as libc::c_ulong as libc::c_int,
        );
        addReplyBulkSds(c, sizes);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"htstats\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut dbid: libc::c_long = 0;
        let mut stats: sds = sdsempty();
        let mut buf_0: [libc::c_char; 4096] = [0; 4096];
        if getLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut dbid,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            sdsfree(stats);
            return;
        }
        if dbid < 0 as libc::c_int as libc::c_long
            || dbid >= server.dbnum as libc::c_long
        {
            sdsfree(stats);
            addReplyError(
                c,
                b"Out of range database\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        stats = sdscatprintf(
            stats,
            b"[Dictionary HT]\n\0" as *const u8 as *const libc::c_char,
        );
        dictGetStats(
            buf_0.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            (*(server.db).offset(dbid as isize)).dict,
        );
        stats = sdscat(stats, buf_0.as_mut_ptr());
        stats = sdscatprintf(
            stats,
            b"[Expires HT]\n\0" as *const u8 as *const libc::c_char,
        );
        dictGetStats(
            buf_0.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
            (*(server.db).offset(dbid as isize)).expires,
        );
        stats = sdscat(stats, buf_0.as_mut_ptr());
        addReplyVerbatim(
            c,
            stats as *const libc::c_char,
            sdslen(stats),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(stats);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"htstats-key\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut o_2: *mut robj = 0 as *mut robj;
        let mut ht: *mut dict = 0 as *mut dict;
        o_2 = objectCommandLookupOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            shared.nokeyerr,
        );
        if o_2.is_null() {
            return;
        }
        match (*o_2).encoding() as libc::c_int {
            7 => {
                let mut zs: *mut zset = (*o_2).ptr as *mut zset;
                ht = (*zs).dict;
            }
            2 => {
                ht = (*o_2).ptr as *mut dict;
            }
            _ => {}
        }
        if ht.is_null() {
            addReplyError(
                c,
                b"The value stored at the specified key is not represented using an hash table\0"
                    as *const u8 as *const libc::c_char,
            );
        } else {
            let mut buf_1: [libc::c_char; 4096] = [0; 4096];
            dictGetStats(
                buf_1.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong,
                ht,
            );
            addReplyVerbatim(
                c,
                buf_1.as_mut_ptr(),
                strlen(buf_1.as_mut_ptr()),
                b"txt\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"change-repl-id\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Changing replication IDs after receiving DEBUG change-repl-id\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        changeReplicationId();
        clearReplicationId2();
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"stringmatch-test\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        stringmatchlen_fuzz_test();
        addReplyStatus(
            c,
            b"Apparently Redis did not crash: test passed\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"set-disable-deny-scripts\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        server
            .script_disable_deny_script = atoi(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"config-rewrite-force-all\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if rewriteConfig(server.configfile, 1 as libc::c_int) == -(1 as libc::c_int) {
            addReplyErrorFormat(
                c,
                b"CONFIG-REWRITE-FORCE-ALL failed: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        } else {
            addReply(c, shared.ok);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"client-eviction\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if (server.client_mem_usage_buckets).is_null() {
            addReplyError(
                c,
                b"maxmemory-clients is disabled.\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let mut bucket_info: sds = sdsempty();
        let mut j_5: libc::c_int = 0 as libc::c_int;
        while j_5 < 1 as libc::c_int + 33 as libc::c_int - 15 as libc::c_int {
            if j_5 == 0 as libc::c_int {
                bucket_info = sdscatprintf(
                    bucket_info,
                    b"bucket          0\0" as *const u8 as *const libc::c_char,
                );
            } else {
                bucket_info = sdscatprintf(
                    bucket_info,
                    b"bucket %10zu\0" as *const u8 as *const libc::c_char,
                    (1 as libc::c_int as size_t)
                        << j_5 - 1 as libc::c_int + 15 as libc::c_int,
                );
            }
            if j_5
                == 1 as libc::c_int + 33 as libc::c_int - 15 as libc::c_int
                    - 1 as libc::c_int
            {
                bucket_info = sdscatprintf(
                    bucket_info,
                    b"+            : \0" as *const u8 as *const libc::c_char,
                );
            } else {
                bucket_info = sdscatprintf(
                    bucket_info,
                    b" - %10zu: \0" as *const u8 as *const libc::c_char,
                    ((1 as libc::c_int as size_t) << j_5 + 15 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
            }
            bucket_info = sdscatprintf(
                bucket_info,
                b"tot-mem: %10zu, clients: %lu\n\0" as *const u8 as *const libc::c_char,
                (*(server.client_mem_usage_buckets).offset(j_5 as isize)).mem_usage_sum,
                (*(*(server.client_mem_usage_buckets).offset(j_5 as isize)).clients).len,
            );
            j_5 += 1;
        }
        addReplyVerbatim(
            c,
            bucket_info as *const libc::c_char,
            sdslen(bucket_info),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(bucket_info);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"mallctl\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 3 as libc::c_int
    {
        mallctl_int(
            c,
            ((*c).argv).offset(2 as libc::c_int as isize),
            (*c).argc - 2 as libc::c_int,
        );
        return;
    } else {
        if strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"mallctl-str\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc >= 3 as libc::c_int
        {
            mallctl_string(
                c,
                ((*c).argv).offset(2 as libc::c_int as isize),
                (*c).argc - 2 as libc::c_int,
            );
            return;
        } else {
            if strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"pause-cron\0" as *const u8 as *const libc::c_char,
            ) == 0 && (*c).argc == 3 as libc::c_int
            {
                server
                    .pause_cron = atoi(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                );
                addReply(c, shared.ok);
            } else if strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"replybuffer\0" as *const u8 as *const libc::c_char,
            ) == 0 && (*c).argc == 4 as libc::c_int
            {
                if strcasecmp(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    b"peak-reset-time\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if strcasecmp(
                        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                            as *const libc::c_char,
                        b"never\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        server
                            .reply_buffer_peak_reset_time = -(1 as libc::c_int)
                            as libc::c_long;
                    } else if strcasecmp(
                        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                            as *const libc::c_char,
                        b"reset\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        server
                            .reply_buffer_peak_reset_time = 5000 as libc::c_int
                            as libc::c_long;
                    } else if getLongFromObjectOrReply(
                        c,
                        *((*c).argv).offset(3 as libc::c_int as isize),
                        &mut server.reply_buffer_peak_reset_time,
                        0 as *const libc::c_char,
                    ) != 0 as libc::c_int
                    {
                        return
                    }
                } else if strcasecmp(
                    (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                        as *const libc::c_char,
                    b"resizing\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    server
                        .reply_buffer_resizing_enabled = atoi(
                        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                            as *const libc::c_char,
                    );
                } else {
                    addReplySubcommandSyntaxError(c);
                    return;
                }
                addReply(c, shared.ok);
            } else {
                addReplySubcommandSyntaxError(c);
                return;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn _serverAssert(
    mut estr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    bugReportStart();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"=== ASSERTION FAILED ===\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"==> %s:%d '%s' is not true\0" as *const u8 as *const libc::c_char,
            file,
            line,
            estr,
        );
    }
    if server.crashlog_enabled != 0 {
        logStackTrace(0 as *mut libc::c_void, 1 as libc::c_int);
        printCrashReport();
    }
    removeSignalHandlers();
    bugReportEnd(0 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _serverAssertPrintClientInfo(mut c: *const client) {
    let mut j: libc::c_int = 0;
    let mut conninfo: [libc::c_char; 32] = [0; 32];
    bugReportStart();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"=== ASSERTION FAILED CLIENT CONTEXT ===\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"client->flags = %llu\0" as *const u8 as *const libc::c_char,
            (*c).flags as libc::c_ulonglong,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"client->conn = %s\0" as *const u8 as *const libc::c_char,
            connGetInfo(
                (*c).conn,
                conninfo.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            ),
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"client->argc = %d\0" as *const u8 as *const libc::c_char,
            (*c).argc,
        );
    }
    j = 0 as libc::c_int;
    while j < (*c).argc {
        let mut buf: [libc::c_char; 128] = [0; 128];
        let mut arg: *mut libc::c_char = 0 as *mut libc::c_char;
        if (**((*c).argv).offset(j as isize)).type_0() as libc::c_int == 0 as libc::c_int
            && ((**((*c).argv).offset(j as isize)).encoding() as libc::c_int
                == 0 as libc::c_int
                || (**((*c).argv).offset(j as isize)).encoding() as libc::c_int
                    == 8 as libc::c_int)
        {
            arg = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
        } else {
            snprintf(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                b"Object type: %u, encoding: %u\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(j as isize)).type_0() as libc::c_int,
                (**((*c).argv).offset(j as isize)).encoding() as libc::c_int,
            );
            arg = buf.as_mut_ptr();
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"client->argv[%d] = \"%s\" (refcount: %d)\0" as *const u8
                    as *const libc::c_char,
                j,
                arg,
                (**((*c).argv).offset(j as isize)).refcount,
            );
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn serverLogObjectDebugInfo(mut o: *const robj) {
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Object type: %u\0" as *const u8 as *const libc::c_char,
            (*o).type_0() as libc::c_int,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Object encoding: %u\0" as *const u8 as *const libc::c_char,
            (*o).encoding() as libc::c_int,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Object refcount: %d\0" as *const u8 as *const libc::c_char,
            (*o).refcount,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _serverAssertPrintObject(mut o: *const robj) {
    bugReportStart();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"=== ASSERTION FAILED OBJECT CONTEXT ===\0" as *const u8
                as *const libc::c_char,
        );
    }
    serverLogObjectDebugInfo(o);
}
#[no_mangle]
pub unsafe extern "C" fn _serverAssertWithInfo(
    mut c: *const client,
    mut o: *const robj,
    mut estr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    if !c.is_null() {
        _serverAssertPrintClientInfo(c);
    }
    if !o.is_null() {
        _serverAssertPrintObject(o);
    }
    _serverAssert(estr, file, line);
}
#[no_mangle]
pub unsafe extern "C" fn _serverPanic(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut msg: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    let mut fmtmsg: [libc::c_char; 256] = [0; 256];
    vsnprintf(
        fmtmsg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong,
        msg,
        ap.as_va_list(),
    );
    bugReportStart();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"------------------------------------------------\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"!!! Software Failure. Press left mouse button to continue\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Guru Meditation: %s #%s:%d\0" as *const u8 as *const libc::c_char,
            fmtmsg.as_mut_ptr(),
            file,
            line,
        );
    }
    if server.crashlog_enabled != 0 {
        logStackTrace(0 as *mut libc::c_void, 1 as libc::c_int);
        printCrashReport();
    }
    removeSignalHandlers();
    bugReportEnd(0 as libc::c_int, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn bugReportStart() {
    pthread_mutex_lock(&mut bug_report_start_mutex);
    if bug_report_start == 0 as libc::c_int {
        serverLogRaw(
            3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
            b"\n\n=== REDIS BUG REPORT START: Cut & paste starting from here ===\n\0"
                as *const u8 as *const libc::c_char,
        );
        bug_report_start = 1 as libc::c_int;
    }
    pthread_mutex_unlock(&mut bug_report_start_mutex);
}
unsafe extern "C" fn getAndSetMcontextEip(
    mut uc: *mut ucontext_t,
    mut eip: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut old_val: *mut libc::c_void = (*uc).uc_mcontext.pc as *mut libc::c_void;
    if !eip.is_null() {
        let mut temp: *mut *mut libc::c_void = &mut (*uc).uc_mcontext.pc
            as *mut libc::c_ulonglong as *mut *mut libc::c_void;
        *temp = eip;
    }
    return old_val;
}
#[no_mangle]
pub unsafe extern "C" fn logStackContent(mut sp: *mut *mut libc::c_void) {
    let mut i: libc::c_int = 0;
    i = 15 as libc::c_int;
    while i >= 0 as libc::c_int {
        let mut addr: libc::c_ulong = (sp as libc::c_ulong)
            .wrapping_add(i as libc::c_ulong);
        let mut val: libc::c_ulong = *sp.offset(i as isize) as libc::c_ulong;
        if core::mem::size_of::<libc::c_long>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"(%08lx) -> %08lx\0" as *const u8 as *const libc::c_char,
                    addr,
                    val,
                );
            }
        } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"(%016lx) -> %016lx\0" as *const u8 as *const libc::c_char,
                addr,
                val,
            );
        }
        i -= 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn logRegisters(mut uc: *mut ucontext_t) {
    if !(((3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int)
        & 0xff as libc::c_int) < server.verbosity)
    {
        _serverLog(
            3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
            b"\n------ REGISTERS ------\n\0" as *const u8 as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"\nX18:%016lx X19:%016lx\nX20:%016lx X21:%016lx\nX22:%016lx X23:%016lx\nX24:%016lx X25:%016lx\nX26:%016lx X27:%016lx\nX28:%016lx X29:%016lx\nX30:%016lx\npc:%016lx sp:%016lx\npstate:%016lx fault_address:%016lx\n\0"
                as *const u8 as *const libc::c_char,
            (*uc).uc_mcontext.regs[18 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[19 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[20 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[21 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[22 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[23 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[24 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[25 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[26 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[27 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[28 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[29 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.regs[30 as libc::c_int as usize] as libc::c_ulong,
            (*uc).uc_mcontext.pc as libc::c_ulong,
            (*uc).uc_mcontext.sp as libc::c_ulong,
            (*uc).uc_mcontext.pstate as libc::c_ulong,
            (*uc).uc_mcontext.fault_address as libc::c_ulong,
        );
    }
    logStackContent((*uc).uc_mcontext.sp as *mut *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn openDirectLogFiledes() -> libc::c_int {
    let mut log_to_stdout: libc::c_int = (*(server.logfile)
        .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32) as libc::c_int;
    let mut fd: libc::c_int = if log_to_stdout != 0 {
        1 as libc::c_int
    } else {
        open(
            server.logfile,
            0o2000 as libc::c_int | 0o100 as libc::c_int | 0o1 as libc::c_int,
            0o644 as libc::c_int,
        )
    };
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn closeDirectLogFiledes(mut fd: libc::c_int) {
    let mut log_to_stdout: libc::c_int = (*(server.logfile)
        .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32) as libc::c_int;
    if log_to_stdout == 0 {
        close(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn logStackTrace(
    mut eip: *mut libc::c_void,
    mut uplevel: libc::c_int,
) {
    let mut trace: [*mut libc::c_void; 100] = [0 as *mut libc::c_void; 100];
    let mut trace_size: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = openDirectLogFiledes();
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    uplevel += 1;
    if fd == -(1 as libc::c_int) {
        return;
    }
    trace_size = backtrace(trace.as_mut_ptr(), 100 as libc::c_int);
    msg = b"\n------ STACK TRACE ------\n\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char;
    write(fd, msg as *const libc::c_void, strlen(msg))
        == -(1 as libc::c_int) as libc::c_long;
    if !eip.is_null() {
        msg = b"EIP:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        write(fd, msg as *const libc::c_void, strlen(msg))
            == -(1 as libc::c_int) as libc::c_long;
        backtrace_symbols_fd(&mut eip, 1 as libc::c_int, fd);
    }
    msg = b"\nBacktrace:\n\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    write(fd, msg as *const libc::c_void, strlen(msg))
        == -(1 as libc::c_int) as libc::c_long;
    backtrace_symbols_fd(
        trace.as_mut_ptr().offset(uplevel as isize),
        trace_size - uplevel,
        fd,
    );
    closeDirectLogFiledes(fd);
}
#[no_mangle]
pub unsafe extern "C" fn logServerInfo() {
    let mut infostring: sds = 0 as *mut libc::c_char;
    let mut clients: sds = 0 as *mut libc::c_char;
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n------ INFO OUTPUT ------\n\0" as *const u8 as *const libc::c_char,
    );
    let mut all: libc::c_int = 0 as libc::c_int;
    let mut everything: libc::c_int = 0 as libc::c_int;
    let mut argv: [*mut robj; 1] = [0 as *mut robj; 1];
    argv[0 as libc::c_int
        as usize] = createStringObject(
        b"all\0" as *const u8 as *const libc::c_char,
        strlen(b"all\0" as *const u8 as *const libc::c_char),
    );
    let mut section_dict: *mut dict = genInfoSectionDict(
        argv.as_mut_ptr(),
        1 as libc::c_int,
        0 as *mut *mut libc::c_char,
        &mut all,
        &mut everything,
    );
    infostring = genRedisInfoString(section_dict, all, everything);
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        infostring as *const libc::c_char,
    );
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n------ CLIENT LIST OUTPUT ------\n\0" as *const u8 as *const libc::c_char,
    );
    clients = getAllClientsInfoString(-(1 as libc::c_int));
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        clients as *const libc::c_char,
    );
    sdsfree(infostring);
    sdsfree(clients);
    releaseInfoSectionDict(section_dict);
    decrRefCount(argv[0 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn logConfigDebugInfo() {
    let mut configstring: sds = 0 as *mut libc::c_char;
    configstring = getConfigDebugInfo();
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n------ CONFIG DEBUG OUTPUT ------\n\0" as *const u8 as *const libc::c_char,
    );
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        configstring as *const libc::c_char,
    );
    sdsfree(configstring);
}
#[no_mangle]
pub unsafe extern "C" fn logModulesInfo() {
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n------ MODULES INFO OUTPUT ------\n\0" as *const u8 as *const libc::c_char,
    );
    let mut infostring: sds = modulesCollectInfo(
        sdsempty(),
        0 as *mut dict,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        infostring as *const libc::c_char,
    );
    sdsfree(infostring);
}
#[no_mangle]
pub unsafe extern "C" fn logCurrentClient() {
    if (server.current_client).is_null() {
        return;
    }
    let mut cc: *mut client = server.current_client;
    let mut client: sds = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n------ CURRENT CLIENT INFO ------\n\0" as *const u8 as *const libc::c_char,
    );
    client = catClientInfoString(sdsempty(), cc);
    if !(((3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int)
        & 0xff as libc::c_int) < server.verbosity)
    {
        _serverLog(
            3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            client,
        );
    }
    sdsfree(client);
    j = 0 as libc::c_int;
    while j < (*cc).argc {
        let mut decoded: *mut robj = 0 as *mut robj;
        decoded = getDecodedObject(*((*cc).argv).offset(j as isize));
        let mut repr: sds = sdscatrepr(
            sdsempty(),
            (*decoded).ptr as *const libc::c_char,
            if sdslen((*decoded).ptr as sds) < 128 as libc::c_int as libc::c_ulong {
                sdslen((*decoded).ptr as sds)
            } else {
                128 as libc::c_int as libc::c_ulong
            },
        );
        if !(((3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int)
            & 0xff as libc::c_int) < server.verbosity)
        {
            _serverLog(
                3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
                b"argv[%d]: '%s'\n\0" as *const u8 as *const libc::c_char,
                j,
                repr,
            );
        }
        sdsfree(repr);
        decrRefCount(decoded);
        j += 1;
    }
    if (*cc).argc > 1 as libc::c_int {
        let mut val: *mut robj = 0 as *mut robj;
        let mut key: *mut robj = 0 as *mut robj;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        key = getDecodedObject(*((*cc).argv).offset(1 as libc::c_int as isize));
        de = dictFind((*(*cc).db).dict, (*key).ptr);
        if !de.is_null() {
            val = (*de).v.val as *mut robj;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"key '%s' found in DB containing the following object:\0"
                        as *const u8 as *const libc::c_char,
                    (*key).ptr as *mut libc::c_char,
                );
            }
            serverLogObjectDebugInfo(val);
        }
        decrRefCount(key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn memtest_test_linux_anonymous_maps() -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut logbuf: [libc::c_char; 1024] = [0; 1024];
    let mut start_addr: size_t = 0;
    let mut end_addr: size_t = 0;
    let mut size: size_t = 0;
    let mut start_vect: [size_t; 128] = [0; 128];
    let mut size_vect: [size_t; 128] = [0; 128];
    let mut regions: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut fd: libc::c_int = openDirectLogFiledes();
    if fd == 0 {
        return 0 as libc::c_int;
    }
    fp = fopen(
        b"/proc/self/maps\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        closeDirectLogFiledes(fd);
        return 0 as libc::c_int;
    }
    while !(fgets(
        line.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        let mut start: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut end: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut p: *mut libc::c_char = line.as_mut_ptr();
        start = p;
        p = strchr(p, '-' as i32);
        if p.is_null() {
            continue;
        }
        let fresh2 = p;
        p = p.offset(1);
        *fresh2 = '\0' as i32 as libc::c_char;
        end = p;
        p = strchr(p, ' ' as i32);
        if p.is_null() {
            continue;
        }
        let fresh3 = p;
        p = p.offset(1);
        *fresh3 = '\0' as i32 as libc::c_char;
        if !(strstr(p, b"stack\0" as *const u8 as *const libc::c_char)).is_null()
            || !(strstr(p, b"vdso\0" as *const u8 as *const libc::c_char)).is_null()
            || !(strstr(p, b"vsyscall\0" as *const u8 as *const libc::c_char)).is_null()
        {
            continue;
        }
        if (strstr(p, b"00:00\0" as *const u8 as *const libc::c_char)).is_null() {
            continue;
        }
        if (strstr(p, b"rw\0" as *const u8 as *const libc::c_char)).is_null() {
            continue;
        }
        start_addr = strtoul(start, 0 as *mut *mut libc::c_char, 16 as libc::c_int);
        end_addr = strtoul(end, 0 as *mut *mut libc::c_char, 16 as libc::c_int);
        size = end_addr.wrapping_sub(start_addr);
        start_vect[regions as usize] = start_addr;
        size_vect[regions as usize] = size;
        snprintf(
            logbuf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
            b"*** Preparing to test memory region %lx (%lu bytes)\n\0" as *const u8
                as *const libc::c_char,
            start_vect[regions as usize],
            size_vect[regions as usize],
        );
        write(
            fd,
            logbuf.as_mut_ptr() as *const libc::c_void,
            strlen(logbuf.as_mut_ptr()),
        ) == -(1 as libc::c_int) as libc::c_long;
        regions += 1;
    }
    let mut errors: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < regions {
        write(
            fd,
            b".\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long;
        errors
            += memtest_preserving_test(
                start_vect[j as usize] as *mut libc::c_void as *mut libc::c_ulong,
                size_vect[j as usize],
                1 as libc::c_int,
            );
        write(
            fd,
            (if errors != 0 {
                b"E\0" as *const u8 as *const libc::c_char
            } else {
                b"O\0" as *const u8 as *const libc::c_char
            }) as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long;
        j += 1;
    }
    write(
        fd,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long;
    fclose(fp);
    closeDirectLogFiledes(fd);
    return errors;
}
unsafe extern "C" fn killMainThread() {
    let mut err: libc::c_int = 0;
    if pthread_self() != server.main_thread_id
        && pthread_cancel(server.main_thread_id) == 0 as libc::c_int
    {
        err = pthread_join(server.main_thread_id, 0 as *mut *mut libc::c_void);
        if err != 0 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"main thread can not be joined: %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(err),
                );
            }
        } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"main thread terminated\0" as *const u8 as *const libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn killThreads() {
    killMainThread();
    bioKillThreads();
    killIOThreads();
}
#[no_mangle]
pub unsafe extern "C" fn doFastMemoryTest() {
    if server.memcheck_enabled != 0 {
        serverLogRaw(
            3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
            b"\n------ FAST MEMORY TEST ------\n\0" as *const u8 as *const libc::c_char,
        );
        killThreads();
        if memtest_test_linux_anonymous_maps() != 0 {
            serverLogRaw(
                3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
                b"!!! MEMORY ERROR DETECTED! Check your memory ASAP !!!\n\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            serverLogRaw(
                3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
                b"Fast memory test PASSED, however your memory can still be broken. Please run a memory test for several hours if possible.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dumpX86Calls(mut addr: *mut libc::c_void, mut len: size_t) {
    let mut j: size_t = 0;
    let mut p: *mut libc::c_uchar = addr as *mut libc::c_uchar;
    let mut info: Dl_info = Dl_info {
        dli_fname: 0 as *const libc::c_char,
        dli_fbase: 0 as *mut libc::c_void,
        dli_sname: 0 as *const libc::c_char,
        dli_saddr: 0 as *mut libc::c_void,
    };
    let mut ht: [libc::c_ulong; 256] = [
        0 as libc::c_int as libc::c_ulong,
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
    if len < 5 as libc::c_int as libc::c_ulong {
        return;
    }
    j = 0 as libc::c_int as size_t;
    while j < len.wrapping_sub(4 as libc::c_int as libc::c_ulong) {
        if !(*p.offset(j as isize) as libc::c_int != 0xe8 as libc::c_int) {
            let mut target: libc::c_ulong = (addr as libc::c_ulong)
                .wrapping_add(j)
                .wrapping_add(5 as libc::c_int as libc::c_ulong);
            let mut tmp: uint32_t = 0;
            memcpy(
                &mut tmp as *mut uint32_t as *mut libc::c_void,
                p.offset(j as isize).offset(1 as libc::c_int as isize)
                    as *const libc::c_void,
                core::mem::size_of::<uint32_t>() as libc::c_ulong,
            );
            target = target.wrapping_add(tmp as libc::c_ulong);
            if dladdr(target as *mut libc::c_void, &mut info) != 0 as libc::c_int
                && !(info.dli_sname).is_null()
            {
                if ht[(target & 0xff as libc::c_int as libc::c_ulong) as usize] != target
                {
                    printf(
                        b"Function at 0x%lx is %s\n\0" as *const u8
                            as *const libc::c_char,
                        target,
                        info.dli_sname,
                    );
                    ht[(target & 0xff as libc::c_int as libc::c_ulong)
                        as usize] = target;
                }
                j = (j as libc::c_ulong).wrapping_add(4 as libc::c_int as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        j = j.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dumpCodeAroundEIP(mut eip: *mut libc::c_void) {
    let mut info: Dl_info = Dl_info {
        dli_fname: 0 as *const libc::c_char,
        dli_fbase: 0 as *mut libc::c_void,
        dli_sname: 0 as *const libc::c_char,
        dli_saddr: 0 as *mut libc::c_void,
    };
    if dladdr(eip, &mut info) != 0 as libc::c_int {
        if !(((3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int)
            & 0xff as libc::c_int) < server.verbosity)
        {
            _serverLog(
                3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
                b"\n------ DUMPING CODE AROUND EIP ------\nSymbol: %s (base: %p)\nModule: %s (base %p)\n$ xxd -r -p /tmp/dump.hex /tmp/dump.bin\n$ objdump --adjust-vma=%p -D -b binary -m i386:x86-64 /tmp/dump.bin\n------\n\0"
                    as *const u8 as *const libc::c_char,
                info.dli_sname,
                info.dli_saddr,
                info.dli_fname,
                info.dli_fbase,
                info.dli_saddr,
            );
        }
        let mut len: size_t = (eip as libc::c_long - info.dli_saddr as libc::c_long)
            as size_t;
        let mut sz: libc::c_ulong = sysconf(_SC_PAGESIZE as libc::c_int)
            as libc::c_ulong;
        if len < ((1 as libc::c_int) << 13 as libc::c_int) as libc::c_ulong {
            let mut base: *mut libc::c_void = info.dli_saddr;
            let mut next: libc::c_ulong = (eip as libc::c_ulong).wrapping_add(sz)
                & !sz.wrapping_sub(1 as libc::c_int as libc::c_ulong);
            let mut end: libc::c_ulong = (eip as libc::c_ulong)
                .wrapping_add(128 as libc::c_int as libc::c_ulong);
            if end > next {
                end = next;
            }
            len = end.wrapping_sub(base as libc::c_ulong);
            serverLogHexDump(
                3 as libc::c_int,
                b"dump of function\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                base,
                len,
            );
            dumpX86Calls(base, len);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn invalidFunctionWasCalled() {}
#[no_mangle]
pub unsafe extern "C" fn sigsegvHandler(
    mut sig: libc::c_int,
    mut info: *mut siginfo_t,
    mut secret: *mut libc::c_void,
) {
    bugReportStart();
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Redis %s crashed by signal: %d, si_code: %d\0" as *const u8
                as *const libc::c_char,
            b"7.0.8\0" as *const u8 as *const libc::c_char,
            sig,
            (*info).si_code,
        );
    }
    if sig == 11 as libc::c_int || sig == 7 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Accessing address: %p\0" as *const u8 as *const libc::c_char,
                (*info)._sifields._sigfault.si_addr,
            );
        }
    }
    if (*info).si_code == SI_USER as libc::c_int
        && (*info)._sifields._kill.si_pid != -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Killed by PID: %ld, UID: %d\0" as *const u8 as *const libc::c_char,
                (*info)._sifields._kill.si_pid as libc::c_long,
                (*info)._sifields._kill.si_uid,
            );
        }
    }
    let mut uc: *mut ucontext_t = secret as *mut ucontext_t;
    let mut eip: *mut libc::c_void = getAndSetMcontextEip(uc, 0 as *mut libc::c_void);
    if !eip.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Crashed running the instruction at: %p\0" as *const u8
                    as *const libc::c_char,
                eip,
            );
        }
    }
    if eip == (*info)._sifields._sigfault.si_addr {
        let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut ptr_ptr: *mut invalidFunctionWasCalledType = &mut ptr
            as *mut *mut libc::c_void as *mut invalidFunctionWasCalledType;
        *ptr_ptr = Some(
            core::mem::transmute::<
                unsafe extern "C" fn() -> (),
                unsafe extern "C" fn() -> (),
            >(invalidFunctionWasCalled),
        );
        getAndSetMcontextEip(uc, ptr);
    }
    logStackTrace(eip, 1 as libc::c_int);
    if eip == (*info)._sifields._sigfault.si_addr {
        getAndSetMcontextEip(uc, eip);
    }
    logRegisters(uc);
    printCrashReport();
    if !eip.is_null() {
        dumpCodeAroundEIP(eip);
    }
    bugReportEnd(1 as libc::c_int, sig);
}
#[no_mangle]
pub unsafe extern "C" fn printCrashReport() {
    logServerInfo();
    logCurrentClient();
    logModulesInfo();
    logConfigDebugInfo();
    doFastMemoryTest();
}
#[no_mangle]
pub unsafe extern "C" fn bugReportEnd(
    mut killViaSignal: libc::c_int,
    mut sig: libc::c_int,
) {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_17 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    serverLogRaw(
        3 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n=== REDIS BUG REPORT END. Make sure to include from START to END. ===\n\n       Please report the crash by opening an issue on github:\n\n           http://github.com/redis/redis/issues\n\n  If a Redis module was involved, please open in the module's repo instead.\n\n  Suspect RAM error? Use redis-server --test-memory to verify it.\n\n  Some other issues could be detected by redis-server --check-system\n\0"
            as *const u8 as *const libc::c_char,
    );
    if server.daemonize != 0 && server.supervised == 0 as libc::c_int
        && !(server.pidfile).is_null()
    {
        unlink(server.pidfile);
    }
    if killViaSignal == 0 {
        if server.use_exit_on_panic != 0 {
            fflush(stdout);
            _exit(1 as libc::c_int);
        }
        abort();
    }
    sigemptyset(&mut act.sa_mask);
    act
        .sa_flags = ((0x40000000 as libc::c_int | 0x8000000 as libc::c_int)
        as libc::c_uint | 0x80000000 as libc::c_uint) as libc::c_int;
    act.__sigaction_handler.sa_handler = None;
    sigaction(sig, &mut act, 0 as *mut sigaction);
    kill(getpid(), sig);
}
#[no_mangle]
pub unsafe extern "C" fn serverLogHexDump(
    mut level: libc::c_int,
    mut descr: *mut libc::c_char,
    mut value: *mut libc::c_void,
    mut len: size_t,
) {
    let mut buf: [libc::c_char; 65] = [0; 65];
    let mut b: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: *mut libc::c_uchar = value as *mut libc::c_uchar;
    /*let mut charset: [libc::c_char; 17] = *core::mem::transmute::<
        &[u8; 17],
        &mut [libc::c_char; 17],
    >(b"0123456789abcdef\0");*/
    let charset: UnsafeCell<[libc::c_char; 17]> = UnsafeCell::new([0; 17]);
    unsafe {
        let charset_slice: &mut [u8] = std::slice::from_raw_parts_mut(
            charset.get() as *mut u8, 17
        );
        charset_slice.copy_from_slice(b"0123456789abcdef\0");
    }
    if !((level & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            level,
            b"%s (hexdump of %zu bytes):\0" as *const u8 as *const libc::c_char,
            descr,
            len,
        );
    }
    b = buf.as_mut_ptr();
    while len != 0 {
        *b
            .offset(
                0 as libc::c_int as isize,
            ) = unsafe { &mut *charset.get() }[(*v as libc::c_int >> 4 as libc::c_int) as usize];
        *b
            .offset(
                1 as libc::c_int as isize,
            ) = unsafe { &mut *charset.get() }[(*v as libc::c_int & 0xf as libc::c_int) as usize];
        *b.offset(2 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        b = b.offset(2 as libc::c_int as isize);
        len = len.wrapping_sub(1);
        v = v.offset(1);
        if b.offset_from(buf.as_mut_ptr()) as libc::c_long
            == 64 as libc::c_int as libc::c_long
            || len == 0 as libc::c_int as libc::c_ulong
        {
            serverLogRaw(
                level | (1 as libc::c_int) << 10 as libc::c_int,
                buf.as_mut_ptr(),
            );
            b = buf.as_mut_ptr();
        }
    }
    serverLogRaw(
        level | (1 as libc::c_int) << 10 as libc::c_int,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn watchdogSignalHandler(
    mut sig: libc::c_int,
    mut info: *mut siginfo_t,
    mut secret: *mut libc::c_void,
) {
    let mut uc: *mut ucontext_t = secret as *mut ucontext_t;
    serverLogFromHandler(
        3 as libc::c_int,
        b"\n--- WATCHDOG TIMER EXPIRED ---\0" as *const u8 as *const libc::c_char,
    );
    logStackTrace(getAndSetMcontextEip(uc, 0 as *mut libc::c_void), 1 as libc::c_int);
    serverLogFromHandler(
        3 as libc::c_int,
        b"--------\n\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn watchdogScheduleSignal(mut period: libc::c_int) {
    let mut it: itimerval = itimerval {
        it_interval: timeval { tv_sec: 0, tv_usec: 0 },
        it_value: timeval { tv_sec: 0, tv_usec: 0 },
    };
    it.it_value.tv_sec = (period / 1000 as libc::c_int) as __time_t;
    it
        .it_value
        .tv_usec = (period % 1000 as libc::c_int * 1000 as libc::c_int) as __suseconds_t;
    it.it_interval.tv_sec = 0 as libc::c_int as __time_t;
    it.it_interval.tv_usec = 0 as libc::c_int as __suseconds_t;
    setitimer(ITIMER_REAL, &mut it, 0 as *mut itimerval);
}
#[no_mangle]
pub unsafe extern "C" fn applyWatchdogPeriod() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_17 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    if server.watchdog_period == 0 as libc::c_int {
        watchdogScheduleSignal(0 as libc::c_int);
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = 0 as libc::c_int;
        act
            .__sigaction_handler
            .sa_handler = core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t);
        sigaction(14 as libc::c_int, &mut act, 0 as *mut sigaction);
    } else {
        sigemptyset(&mut act.sa_mask);
        act.sa_flags = 4 as libc::c_int;
        act
            .__sigaction_handler
            .sa_sigaction = Some(
            watchdogSignalHandler
                as unsafe extern "C" fn(
                    libc::c_int,
                    *mut siginfo_t,
                    *mut libc::c_void,
                ) -> (),
        );
        sigaction(14 as libc::c_int, &mut act, 0 as *mut sigaction);
        let mut min_period: libc::c_int = 1000 as libc::c_int / server.hz
            * 2 as libc::c_int;
        if server.watchdog_period < min_period {
            server.watchdog_period = min_period;
        }
        watchdogScheduleSignal(server.watchdog_period);
    };
}
#[no_mangle]
pub unsafe extern "C" fn debugDelay(mut usec: libc::c_int) {
    if usec < 0 as libc::c_int {
        usec = if rand() % -usec == 0 as libc::c_int {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if usec != 0 {
        usleep(usec as __useconds_t);
    }
}

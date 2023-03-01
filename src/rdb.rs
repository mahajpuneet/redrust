extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
use std::cell::UnsafeCell;
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
    pub type functionsLibCtx;
    pub type clusterState;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut R_PosInf: libc::c_double;
    static mut R_NegInf: libc::c_double;
    static mut R_Nan: libc::c_double;
    static mut hashDictType: dictType;
    fn moduleTypeLookupModuleByID(id: uint64_t) -> *mut moduleType;
    fn moduleTypeNameByID(name: *mut libc::c_char, moduleid: uint64_t);
    fn moduleTypeModuleName(mt: *mut moduleType) -> *const libc::c_char;
    fn moduleFreeContext(ctx: *mut RedisModuleCtx);
    fn moduleNotifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *const libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn rdbSaveModulesAux(rdb: *mut rio, when: libc::c_int) -> ssize_t;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn processModuleLoadingProgressEvent(is_aof: libc::c_int);
    fn mstime() -> libc::c_longlong;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn exitFromChild(retcode: libc::c_int);
    fn redisSetProcTitle(title: *mut libc::c_char) -> libc::c_int;
    fn redisSetCpuAffinity(cpulist: *const libc::c_char);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn processEventsWhileBlocked();
    fn blockingOperationStarts();
    fn blockingOperationEnds();
    fn decrRefCount(o: *mut robj);
    fn dismissObject(o: *mut robj, dump_size: size_t);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn tryCreateRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn tryCreateStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn isSdsRepresentableAsLongLong(s: sds, llval: *mut libc::c_longlong) -> libc::c_int;
    fn tryObjectEncoding(o: *mut robj) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn createStringObjectFromLongLongForValue(value: libc::c_longlong) -> *mut robj;
    fn createQuicklistObject() -> *mut robj;
    fn createSetObject() -> *mut robj;
    fn createIntsetObject() -> *mut robj;
    fn createHashObject() -> *mut robj;
    fn createZsetObject() -> *mut robj;
    fn createStreamObject() -> *mut robj;
    fn createModuleObject(mt: *mut moduleType, value: *mut libc::c_void) -> *mut robj;
    fn rioInitWithFile(r: *mut rio, fp: *mut FILE);
    fn rioInitWithFd(r: *mut rio, fd: libc::c_int);
    fn rioFreeFd(r: *mut rio);
    fn rioGenericUpdateChecksum(r: *mut rio, buf: *const libc::c_void, len: size_t);
    fn rioSetAutoSync(r: *mut rio, bytes: off_t);
    fn rioCheckType(r: *mut rio) -> uint8_t;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn getpid() -> __pid_t;
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
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictTryExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn ztrymalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zmalloc_used_memory() -> size_t;
    fn anetPipe(
        fds: *mut libc::c_int,
        read_flags: libc::c_int,
        write_flags: libc::c_int,
    ) -> libc::c_int;
    fn intsetAdd(is: *mut intset, value: int64_t, success: *mut uint8_t) -> *mut intset;
    fn intsetLen(is: *const intset) -> uint32_t;
    fn intsetBlobLen(is: *mut intset) -> size_t;
    fn intsetValidateIntegrity(
        is: *const libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
    ) -> libc::c_int;
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
    fn double2ll(d: libc::c_double, out: *mut libc::c_longlong) -> libc::c_int;
    fn fsyncFileDir(filename: *const libc::c_char) -> libc::c_int;
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn quicklistNew(fill: libc::c_int, compress: libc::c_int) -> *mut quicklist;
    fn quicklistSetOptions(
        quicklist: *mut quicklist,
        fill: libc::c_int,
        depth: libc::c_int,
    );
    fn quicklistRelease(quicklist: *mut quicklist);
    fn quicklistPushTail(
        quicklist: *mut quicklist,
        value: *mut libc::c_void,
        sz: size_t,
    ) -> libc::c_int;
    fn quicklistAppendListpack(quicklist: *mut quicklist, zl: *mut libc::c_uchar);
    fn quicklistAppendPlainNode(
        quicklist: *mut quicklist,
        data: *mut libc::c_uchar,
        sz: size_t,
    );
    fn quicklistCount(ql: *const quicklist) -> libc::c_ulong;
    fn quicklistGetLzf(
        node: *const quicklistNode,
        data: *mut *mut libc::c_void,
    ) -> size_t;
    static mut raxNotFound: *mut libc::c_void;
    fn raxTryInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
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
    fn zipmapRewind(zm: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn zipmapNext(
        zm: *mut libc::c_uchar,
        key: *mut *mut libc::c_uchar,
        klen: *mut libc::c_uint,
        value: *mut *mut libc::c_uchar,
        vlen: *mut libc::c_uint,
    ) -> *mut libc::c_uchar;
    fn zipmapValidateIntegrity(
        zm: *mut libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
    ) -> libc::c_int;
    fn ziplistGet(
        p: *mut libc::c_uchar,
        sval: *mut *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> libc::c_uint;
    fn ziplistValidateIntegrity(
        zl: *mut libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
        entry_cb: ziplistValidateEntryCB,
        cb_userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn intrev64(v: uint64_t) -> uint64_t;
    fn __errno_location() -> *mut libc::c_int;
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdsfree(s: sds);
    fn sdsdup(s: sds) -> sds;
    fn sdsempty() -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdstrynewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    static mut SDS_NOINIT: *const libc::c_char;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn lpNew(capacity: size_t) -> *mut libc::c_uchar;
    fn lpShrinkToFit(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpAppend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpAppendInteger(
        lp: *mut libc::c_uchar,
        lval: libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpLength(lp: *mut libc::c_uchar) -> libc::c_ulong;
    fn lpGet(
        p: *mut libc::c_uchar,
        count: *mut int64_t,
        intbuf: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
    fn lpValidateIntegrity(
        lp: *mut libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
        entry_cb: listpackValidateEntryCB,
        cb_userdata: *mut libc::c_void,
    ) -> libc::c_int;
    fn lpSafeToAdd(lp: *mut libc::c_uchar, add: size_t) -> libc::c_int;
    fn streamCreateConsumer(
        cg: *mut streamCG,
        name: sds,
        key: *mut robj,
        dbid: libc::c_int,
        flags: libc::c_int,
    ) -> *mut streamConsumer;
    fn streamCreateCG(
        s: *mut stream,
        name: *mut libc::c_char,
        namelen: size_t,
        id: *mut streamID,
        entries_read: libc::c_longlong,
    ) -> *mut streamCG;
    fn streamCreateNACK(consumer: *mut streamConsumer) -> *mut streamNACK;
    fn streamFreeNACK(na: *mut streamNACK);
    fn streamValidateListpackIntegrity(
        lp: *mut libc::c_uchar,
        size: size_t,
        deep: libc::c_int,
    ) -> libc::c_int;
    fn streamGetEdgeID(
        s: *mut stream,
        first: libc::c_int,
        skip_tombstones: libc::c_int,
        edge_id: *mut streamID,
    );
    fn streamEstimateDistanceFromFirstEverEntry(
        s: *mut stream,
        id: *mut streamID,
    ) -> libc::c_longlong;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn estimateObjectIdleTime(o: *mut robj) -> libc::c_ulonglong;
    fn replicationFeedSlaves(
        slaves: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn updateSlavesWaitingBgsave(bgsaveerr: libc::c_int, type_0: libc::c_int);
    fn replicationSendNewlineToMaster();
    fn getPsyncInitialOffset() -> libc::c_longlong;
    fn replicationSetupSlaveForFullResync(
        slave: *mut client,
        offset: libc::c_longlong,
    ) -> libc::c_int;
    fn rdbPipeReadHandler(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        clientData: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn redis_check_rdb_main(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        fp: *mut FILE,
    ) -> libc::c_int;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn debugDelay(usec: libc::c_int);
    fn LRU_CLOCK() -> libc::c_uint;
    fn objectSetLRUOrLFU(
        val: *mut robj,
        lfu_freq: libc::c_longlong,
        lru_idle: libc::c_longlong,
        lru_clock: libc::c_longlong,
        lru_multiplier: libc::c_int,
    ) -> libc::c_int;
    fn setExpire(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        when: libc::c_longlong,
    );
    fn dbAddRDBLoad(db: *mut redisDb, key: sds, val: *mut robj) -> libc::c_int;
    fn dbSyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn iAmMaster() -> libc::c_int;
    fn hashTypeConvert(o: *mut robj, enc: libc::c_int);
    fn hashTypeLength(o: *const robj) -> libc::c_ulong;
    fn zsetConvert(zobj: *mut robj, encoding: libc::c_int);
    fn zsetLength(zobj: *const robj) -> libc::c_ulong;
    fn setTypeConvert(subject: *mut robj, enc: libc::c_int);
    fn zslInsert(
        zsl: *mut zskiplist,
        score: libc::c_double,
        ele: sds,
    ) -> *mut zskiplistNode;
    fn sendChildCowInfo(info_type: childInfoType, pname: *mut libc::c_char);
    fn sendChildInfo(info_type: childInfoType, keys: size_t, pname: *mut libc::c_char);
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn LFUDecrAndReturn(o: *mut robj) -> libc::c_ulong;
    fn getExpire(db: *mut redisDb, key: *mut robj) -> libc::c_longlong;
    fn redisFork(purpose: libc::c_int) -> libc::c_int;
    fn hasActiveChildProcess() -> libc::c_int;
    fn bg_unlink(filename: *const libc::c_char) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn lzf_compress(
        in_data: *const libc::c_void,
        in_len: size_t,
        out_data: *mut libc::c_void,
        out_len: size_t,
    ) -> size_t;
    fn lzf_decompress(
        in_data: *const libc::c_void,
        in_len: size_t,
        out_data: *mut libc::c_void,
        out_len: size_t,
    ) -> size_t;
    fn functionsCreateWithLibraryCtx(
        code: sds,
        replace: libc::c_int,
        err: *mut sds,
        lib_ctx: *mut functionsLibCtx,
    ) -> sds;
    fn functionsLibGet() -> *mut dict;
    fn functionsLibCtxGetCurrent() -> *mut functionsLibCtx;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut rdbCheckMode: libc::c_int;
    fn rdbCheckError(fmt: *const libc::c_char, _: ...);
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
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
pub type ziplistValidateEntryCB = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
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
    pub bs: C2RustUnnamed_9,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub range: C2RustUnnamed_8,
    pub keynum: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
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
pub union C2RustUnnamed_9 {
    pub index: C2RustUnnamed_11,
    pub keyword: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
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
    pub inst_metric: [C2RustUnnamed_12; 5],
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
pub struct C2RustUnnamed_12 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
pub type listpackValidateEntryCB = Option::<
    unsafe extern "C" fn(
        *mut libc::c_uchar,
        libc::c_uint,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
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
pub struct C2RustUnnamed_13 {
    pub count: libc::c_long,
    pub fields: *mut dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub count: libc::c_long,
    pub fields: *mut dict,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub count: libc::c_long,
    pub fields: *mut dict,
    pub lp: *mut *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub count: libc::c_long,
    pub fields: *mut dict,
    pub lp: *mut *mut libc::c_uchar,
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
unsafe extern "C" fn rioFlush(mut r: *mut rio) -> libc::c_int {
    return ((*r).flush).expect("non-null function pointer")(r);
}
#[inline]
unsafe extern "C" fn rioGetReadError(mut r: *mut rio) -> libc::c_int {
    return ((*r).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong
        != 0 as libc::c_int as libc::c_ulong) as libc::c_int;
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
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut rdbFileBeingLoaded: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn rdbReportError(
    mut corruption_error: libc::c_int,
    mut linenum: libc::c_int,
    mut reason: *mut libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut len: libc::c_int = 0;
    len = snprintf(
        msg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"Internal error in RDB reading offset %llu, function at rdb.c:%d -> \0"
            as *const u8 as *const libc::c_char,
        server.loading_loaded_bytes as libc::c_ulonglong,
        linenum,
    );
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr().offset(len as isize),
        (core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(len as libc::c_ulong),
        reason,
        ap.as_va_list(),
    );
    if if (server.current_client).is_null()
        || (*server.current_client).id == 18446744073709551615 as libc::c_ulong
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    } != 0
    {
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"%s\0" as *const u8 as *const libc::c_char,
                msg.as_mut_ptr(),
            );
        }
        return;
    } else {
        if rdbCheckMode != 0 {
            rdbCheckError(b"%s\0" as *const u8 as *const libc::c_char, msg.as_mut_ptr());
        } else if !rdbFileBeingLoaded.is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    msg.as_mut_ptr(),
                );
            }
            let mut argv: [*mut libc::c_char; 2] = [
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                rdbFileBeingLoaded,
            ];
            redis_check_rdb_main(2 as libc::c_int, argv.as_mut_ptr(), 0 as *mut FILE);
        } else if corruption_error != 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"%s. Failure loading rdb format\0" as *const u8
                        as *const libc::c_char,
                    msg.as_mut_ptr(),
                );
            }
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"%s. Failure loading rdb format from socket, assuming connection error, resuming operation.\0"
                        as *const u8 as *const libc::c_char,
                    msg.as_mut_ptr(),
                );
            }
            return;
        }
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Terminating server after rdb file reading failure.\0" as *const u8
                as *const libc::c_char,
        );
    }
    exit(1 as libc::c_int);
}
unsafe extern "C" fn rdbWriteRaw(
    mut rdb: *mut rio,
    mut p: *mut libc::c_void,
    mut len: size_t,
) -> ssize_t {
    if !rdb.is_null() && rioWrite(rdb, p, len) == 0 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int) as ssize_t;
    }
    return len as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveType(
    mut rdb: *mut rio,
    mut type_0: libc::c_uchar,
) -> libc::c_int {
    return rdbWriteRaw(
        rdb,
        &mut type_0 as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadType(mut rdb: *mut rio) -> libc::c_int {
    let mut type_0: libc::c_uchar = 0;
    if rioRead(
        rdb,
        &mut type_0 as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return type_0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadTime(mut rdb: *mut rio) -> time_t {
    let mut t32: int32_t = 0;
    if rioRead(
        rdb,
        &mut t32 as *mut int32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int) as time_t;
    }
    return t32 as time_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveMillisecondTime(
    mut rdb: *mut rio,
    mut t: libc::c_longlong,
) -> libc::c_int {
    let mut t64: int64_t = t as int64_t;
    return rdbWriteRaw(
        rdb,
        &mut t64 as *mut int64_t as *mut libc::c_void,
        8 as libc::c_int as size_t,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadMillisecondTime(
    mut rdb: *mut rio,
    mut rdbver: libc::c_int,
) -> libc::c_longlong {
    let mut t64: int64_t = 0;
    if rioRead(
        rdb,
        &mut t64 as *mut int64_t as *mut libc::c_void,
        8 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return 9223372036854775807 as libc::c_longlong;
    }
    rdbver >= 9 as libc::c_int;
    return t64 as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveLen(
    mut rdb: *mut rio,
    mut len: uint64_t,
) -> libc::c_int {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    let mut nwritten: size_t = 0;
    if len < ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong {
        buf[0 as libc::c_int
            as usize] = (len & 0xff as libc::c_int as libc::c_ulong
            | ((0 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
            as libc::c_uchar;
        if rdbWriteRaw(
            rdb,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        nwritten = 1 as libc::c_int as size_t;
    } else if len < ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong {
        buf[0 as libc::c_int
            as usize] = (len >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_ulong
            | ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong)
            as libc::c_uchar;
        buf[1 as libc::c_int
            as usize] = (len & 0xff as libc::c_int as libc::c_ulong) as libc::c_uchar;
        if rdbWriteRaw(
            rdb,
            buf.as_mut_ptr() as *mut libc::c_void,
            2 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        nwritten = 2 as libc::c_int as size_t;
    } else if len <= 4294967295 as libc::c_uint as libc::c_ulong {
        buf[0 as libc::c_int as usize] = 0x80 as libc::c_int as libc::c_uchar;
        if rdbWriteRaw(
            rdb,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        let mut len32: uint32_t = __bswap_32(len as __uint32_t);
        if rdbWriteRaw(
            rdb,
            &mut len32 as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        nwritten = (1 as libc::c_int + 4 as libc::c_int) as size_t;
    } else {
        buf[0 as libc::c_int as usize] = 0x81 as libc::c_int as libc::c_uchar;
        if rdbWriteRaw(
            rdb,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        len = intrev64(len);
        if rdbWriteRaw(
            rdb,
            &mut len as *mut uint64_t as *mut libc::c_void,
            8 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        nwritten = (1 as libc::c_int + 8 as libc::c_int) as size_t;
    }
    return nwritten as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadLenByRef(
    mut rdb: *mut rio,
    mut isencoded: *mut libc::c_int,
    mut lenptr: *mut uint64_t,
) -> libc::c_int {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    let mut type_0: libc::c_int = 0;
    if !isencoded.is_null() {
        *isencoded = 0 as libc::c_int;
    }
    if rioRead(rdb, buf.as_mut_ptr() as *mut libc::c_void, 1 as libc::c_int as size_t)
        == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    type_0 = (buf[0 as libc::c_int as usize] as libc::c_int & 0xc0 as libc::c_int)
        >> 6 as libc::c_int;
    if type_0 == 3 as libc::c_int {
        if !isencoded.is_null() {
            *isencoded = 1 as libc::c_int;
        }
        *lenptr = (buf[0 as libc::c_int as usize] as libc::c_int & 0x3f as libc::c_int)
            as uint64_t;
    } else if type_0 == 0 as libc::c_int {
        *lenptr = (buf[0 as libc::c_int as usize] as libc::c_int & 0x3f as libc::c_int)
            as uint64_t;
    } else if type_0 == 1 as libc::c_int {
        if rioRead(
            rdb,
            buf.as_mut_ptr().offset(1 as libc::c_int as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        *lenptr = ((buf[0 as libc::c_int as usize] as libc::c_int & 0x3f as libc::c_int)
            << 8 as libc::c_int | buf[1 as libc::c_int as usize] as libc::c_int)
            as uint64_t;
    } else if buf[0 as libc::c_int as usize] as libc::c_int == 0x80 as libc::c_int {
        let mut len: uint32_t = 0;
        if rioRead(
            rdb,
            &mut len as *mut uint32_t as *mut libc::c_void,
            4 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        *lenptr = __bswap_32(len) as uint64_t;
    } else if buf[0 as libc::c_int as usize] as libc::c_int == 0x81 as libc::c_int {
        let mut len_0: uint64_t = 0;
        if rioRead(
            rdb,
            &mut len_0 as *mut uint64_t as *mut libc::c_void,
            8 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return -(1 as libc::c_int);
        }
        *lenptr = intrev64(len_0);
    } else {
        rdbReportError(
            1 as libc::c_int,
            237 as libc::c_int,
            b"Unknown length encoding %d in rdbLoadLen()\0" as *const u8
                as *const libc::c_char as *mut libc::c_char,
            type_0,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadLen(
    mut rdb: *mut rio,
    mut isencoded: *mut libc::c_int,
) -> uint64_t {
    let mut len: uint64_t = 0;
    if rdbLoadLenByRef(rdb, isencoded, &mut len) == -(1 as libc::c_int) {
        return 18446744073709551615 as libc::c_ulong;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn rdbEncodeInteger(
    mut value: libc::c_longlong,
    mut enc: *mut libc::c_uchar,
) -> libc::c_int {
    if value >= -((1 as libc::c_int) << 7 as libc::c_int) as libc::c_longlong
        && value
            <= (((1 as libc::c_int) << 7 as libc::c_int) - 1 as libc::c_int)
                as libc::c_longlong
    {
        *enc
            .offset(
                0 as libc::c_int as isize,
            ) = ((3 as libc::c_int) << 6 as libc::c_int | 0 as libc::c_int)
            as libc::c_uchar;
        *enc
            .offset(
                1 as libc::c_int as isize,
            ) = (value & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
        return 2 as libc::c_int;
    } else if value >= -((1 as libc::c_int) << 15 as libc::c_int) as libc::c_longlong
        && value
            <= (((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int)
                as libc::c_longlong
    {
        *enc
            .offset(
                0 as libc::c_int as isize,
            ) = ((3 as libc::c_int) << 6 as libc::c_int | 1 as libc::c_int)
            as libc::c_uchar;
        *enc
            .offset(
                1 as libc::c_int as isize,
            ) = (value & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
        *enc
            .offset(
                2 as libc::c_int as isize,
            ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        return 3 as libc::c_int;
    } else if value >= -((1 as libc::c_int as libc::c_longlong) << 31 as libc::c_int)
        && value
            <= ((1 as libc::c_int as libc::c_longlong) << 31 as libc::c_int)
                - 1 as libc::c_int as libc::c_longlong
    {
        *enc
            .offset(
                0 as libc::c_int as isize,
            ) = ((3 as libc::c_int) << 6 as libc::c_int | 2 as libc::c_int)
            as libc::c_uchar;
        *enc
            .offset(
                1 as libc::c_int as isize,
            ) = (value & 0xff as libc::c_int as libc::c_longlong) as libc::c_uchar;
        *enc
            .offset(
                2 as libc::c_int as isize,
            ) = (value >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        *enc
            .offset(
                3 as libc::c_int as isize,
            ) = (value >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        *enc
            .offset(
                4 as libc::c_int as isize,
            ) = (value >> 24 as libc::c_int & 0xff as libc::c_int as libc::c_longlong)
            as libc::c_uchar;
        return 5 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadIntegerObject(
    mut rdb: *mut rio,
    mut enctype: libc::c_int,
    mut flags: libc::c_int,
    mut lenptr: *mut size_t,
) -> *mut libc::c_void {
    let mut plain: libc::c_int = flags & (1 as libc::c_int) << 1 as libc::c_int;
    let mut sds: libc::c_int = flags & (1 as libc::c_int) << 2 as libc::c_int;
    let mut encode: libc::c_int = flags & (1 as libc::c_int) << 0 as libc::c_int;
    let mut enc: [libc::c_uchar; 4] = [0; 4];
    let mut val: libc::c_longlong = 0;
    if enctype == 0 as libc::c_int {
        if rioRead(
            rdb,
            enc.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as *mut libc::c_void;
        }
        val = enc[0 as libc::c_int as usize] as libc::c_schar as libc::c_longlong;
    } else if enctype == 1 as libc::c_int {
        let mut v: uint16_t = 0;
        if rioRead(
            rdb,
            enc.as_mut_ptr() as *mut libc::c_void,
            2 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as *mut libc::c_void;
        }
        v = (enc[0 as libc::c_int as usize] as uint32_t
            | (enc[1 as libc::c_int as usize] as uint32_t) << 8 as libc::c_int)
            as uint16_t;
        val = v as int16_t as libc::c_longlong;
    } else if enctype == 2 as libc::c_int {
        let mut v_0: uint32_t = 0;
        if rioRead(
            rdb,
            enc.as_mut_ptr() as *mut libc::c_void,
            4 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong
        {
            return 0 as *mut libc::c_void;
        }
        v_0 = enc[0 as libc::c_int as usize] as uint32_t
            | (enc[1 as libc::c_int as usize] as uint32_t) << 8 as libc::c_int
            | (enc[2 as libc::c_int as usize] as uint32_t) << 16 as libc::c_int
            | (enc[3 as libc::c_int as usize] as uint32_t) << 24 as libc::c_int;
        val = v_0 as int32_t as libc::c_longlong;
    } else {
        rdbReportError(
            1 as libc::c_int,
            308 as libc::c_int,
            b"Unknown RDB integer encoding type %d\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            enctype,
        );
        return 0 as *mut libc::c_void;
    }
    if plain != 0 || sds != 0 {
        let mut buf: [libc::c_char; 21] = [0; 21];
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: libc::c_int = ll2string(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
            val,
        );
        if !lenptr.is_null() {
            *lenptr = len as size_t;
        }
        p = (if plain != 0 {
            zmalloc(len as size_t)
        } else {
            sdsnewlen(SDS_NOINIT as *const libc::c_void, len as size_t)
                as *mut libc::c_void
        }) as *mut libc::c_char;
        memcpy(
            p as *mut libc::c_void,
            buf.as_mut_ptr() as *const libc::c_void,
            len as libc::c_ulong,
        );
        return p as *mut libc::c_void;
    } else if encode != 0 {
        return createStringObjectFromLongLongForValue(val) as *mut libc::c_void
    } else {
        return createObject(0 as libc::c_int, sdsfromlonglong(val) as *mut libc::c_void)
            as *mut libc::c_void
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbTryIntegerEncoding(
    mut s: *mut libc::c_char,
    mut len: size_t,
    mut enc: *mut libc::c_uchar,
) -> libc::c_int {
    let mut value: libc::c_longlong = 0;
    if string2ll(s, len, &mut value) != 0 {
        return rdbEncodeInteger(value, enc)
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveLzfBlob(
    mut rdb: *mut rio,
    mut data: *mut libc::c_void,
    mut compress_len: size_t,
    mut original_len: size_t,
) -> ssize_t {
    let mut byte: libc::c_uchar = 0;
    let mut n: ssize_t = 0;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    byte = ((3 as libc::c_int) << 6 as libc::c_int | 3 as libc::c_int) as libc::c_uchar;
    n = rdbWriteRaw(
        rdb,
        &mut byte as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as size_t,
    );
    if !(n == -(1 as libc::c_int) as libc::c_long) {
        nwritten += n;
        n = rdbSaveLen(rdb, compress_len) as ssize_t;
        if !(n == -(1 as libc::c_int) as libc::c_long) {
            nwritten += n;
            n = rdbSaveLen(rdb, original_len) as ssize_t;
            if !(n == -(1 as libc::c_int) as libc::c_long) {
                nwritten += n;
                n = rdbWriteRaw(rdb, data, compress_len);
                if !(n == -(1 as libc::c_int) as libc::c_long) {
                    nwritten += n;
                    return nwritten;
                }
            }
        }
    }
    return -(1 as libc::c_int) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveLzfStringObject(
    mut rdb: *mut rio,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
) -> ssize_t {
    let mut comprlen: size_t = 0;
    let mut outlen: size_t = 0;
    let mut out: *mut libc::c_void = 0 as *mut libc::c_void;
    if len <= 4 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as ssize_t;
    }
    outlen = len.wrapping_sub(4 as libc::c_int as libc::c_ulong);
    out = zmalloc(outlen.wrapping_add(1 as libc::c_int as libc::c_ulong));
    if out.is_null() {
        return 0 as libc::c_int as ssize_t;
    }
    comprlen = lzf_compress(s as *const libc::c_void, len, out, outlen);
    if comprlen == 0 as libc::c_int as libc::c_ulong {
        zfree(out);
        return 0 as libc::c_int as ssize_t;
    }
    let mut nwritten: ssize_t = rdbSaveLzfBlob(rdb, out, comprlen, len);
    zfree(out);
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadLzfStringObject(
    mut rdb: *mut rio,
    mut flags: libc::c_int,
    mut lenptr: *mut size_t,
) -> *mut libc::c_void {
    let mut plain: libc::c_int = flags & (1 as libc::c_int) << 1 as libc::c_int;
    let mut sds: libc::c_int = flags & (1 as libc::c_int) << 2 as libc::c_int;
    let mut len: uint64_t = 0;
    let mut clen: uint64_t = 0;
    let mut c: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    clen = rdbLoadLen(rdb, 0 as *mut libc::c_int);
    if clen == 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    len = rdbLoadLen(rdb, 0 as *mut libc::c_int);
    if len == 18446744073709551615 as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    c = ztrymalloc(clen) as *mut libc::c_uchar;
    if c.is_null() {
        if !(((if (server.current_client).is_null()
            || (*server.current_client).id == 18446744073709551615 as libc::c_ulong
        {
            0 as libc::c_int
        } else {
            (if 1 as libc::c_int != 0 { 1 as libc::c_int } else { 3 as libc::c_int })
        }) & 0xff as libc::c_int) < server.verbosity)
        {
            _serverLog(
                if (server.current_client).is_null()
                    || (*server.current_client).id
                        == 18446744073709551615 as libc::c_ulong
                {
                    0 as libc::c_int
                } else if 1 as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    3 as libc::c_int
                },
                b"rdbLoadLzfStringObject failed allocating %llu bytes\0" as *const u8
                    as *const libc::c_char,
                clen as libc::c_ulonglong,
            );
        }
    } else {
        if plain != 0 {
            val = ztrymalloc(len) as *mut libc::c_char;
        } else {
            val = sdstrynewlen(SDS_NOINIT as *const libc::c_void, len);
        }
        if val.is_null() {
            if !(((if (server.current_client).is_null()
                || (*server.current_client).id == 18446744073709551615 as libc::c_ulong
            {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 1 as libc::c_int } else { 3 as libc::c_int })
            }) & 0xff as libc::c_int) < server.verbosity)
            {
                _serverLog(
                    if (server.current_client).is_null()
                        || (*server.current_client).id
                            == 18446744073709551615 as libc::c_ulong
                    {
                        0 as libc::c_int
                    } else if 1 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        3 as libc::c_int
                    },
                    b"rdbLoadLzfStringObject failed allocating %llu bytes\0" as *const u8
                        as *const libc::c_char,
                    len as libc::c_ulonglong,
                );
            }
        } else {
            if !lenptr.is_null() {
                *lenptr = len;
            }
            if !(rioRead(rdb, c as *mut libc::c_void, clen)
                == 0 as libc::c_int as libc::c_ulong)
            {
                if lzf_decompress(
                    c as *const libc::c_void,
                    clen,
                    val as *mut libc::c_void,
                    len,
                ) != len
                {
                    rdbReportError(
                        1 as libc::c_int,
                        413 as libc::c_int,
                        b"Invalid LZF compressed string\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                } else {
                    zfree(c as *mut libc::c_void);
                    if plain != 0 || sds != 0 {
                        return val as *mut libc::c_void
                    } else {
                        return createObject(0 as libc::c_int, val as *mut libc::c_void)
                            as *mut libc::c_void
                    }
                }
            }
        }
    }
    zfree(c as *mut libc::c_void);
    if plain != 0 {
        zfree(val as *mut libc::c_void);
    } else {
        sdsfree(val);
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveRawString(
    mut rdb: *mut rio,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
) -> ssize_t {
    let mut enclen: libc::c_int = 0;
    let mut n: ssize_t = 0;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    if len <= 11 as libc::c_int as libc::c_ulong {
        let mut buf: [libc::c_uchar; 5] = [0; 5];
        enclen = rdbTryIntegerEncoding(s as *mut libc::c_char, len, buf.as_mut_ptr());
        if enclen > 0 as libc::c_int {
            if rdbWriteRaw(rdb, buf.as_mut_ptr() as *mut libc::c_void, enclen as size_t)
                == -(1 as libc::c_int) as libc::c_long
            {
                return -(1 as libc::c_int) as ssize_t;
            }
            return enclen as ssize_t;
        }
    }
    if server.rdb_compression != 0 && len > 20 as libc::c_int as libc::c_ulong {
        n = rdbSaveLzfStringObject(rdb, s, len);
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        if n > 0 as libc::c_int as libc::c_long {
            return n;
        }
    }
    n = rdbSaveLen(rdb, len) as ssize_t;
    if n == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    nwritten += n;
    if len > 0 as libc::c_int as libc::c_ulong {
        if rdbWriteRaw(rdb, s as *mut libc::c_void, len)
            == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten = (nwritten as libc::c_ulong).wrapping_add(len) as ssize_t as ssize_t;
    }
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveLongLongAsStringObject(
    mut rdb: *mut rio,
    mut value: libc::c_longlong,
) -> ssize_t {
    let mut buf: [libc::c_uchar; 32] = [0; 32];
    let mut n: ssize_t = 0;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    let mut enclen: libc::c_int = rdbEncodeInteger(value, buf.as_mut_ptr());
    if enclen > 0 as libc::c_int {
        return rdbWriteRaw(rdb, buf.as_mut_ptr() as *mut libc::c_void, enclen as size_t)
    } else {
        enclen = ll2string(
            buf.as_mut_ptr() as *mut libc::c_char,
            32 as libc::c_int as size_t,
            value,
        );
        if enclen < 32 as libc::c_int {} else {
            _serverAssert(
                b"enclen < 32\0" as *const u8 as *const libc::c_char,
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                476 as libc::c_int,
            );
            unreachable!();
        };
        n = rdbSaveLen(rdb, enclen as uint64_t) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbWriteRaw(rdb, buf.as_mut_ptr() as *mut libc::c_void, enclen as size_t);
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
    }
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveStringObject(
    mut rdb: *mut rio,
    mut obj: *mut robj,
) -> ssize_t {
    if (*obj).encoding() as libc::c_int == 1 as libc::c_int {
        return rdbSaveLongLongAsStringObject(
            rdb,
            (*obj).ptr as libc::c_long as libc::c_longlong,
        )
    } else {
        if (*obj).encoding() as libc::c_int == 0 as libc::c_int
            || (*obj).encoding() as libc::c_int == 8 as libc::c_int
        {} else {
            _serverAssertWithInfo(
                0 as *const client,
                obj,
                b"sdsEncodedObject(obj)\0" as *const u8 as *const libc::c_char,
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                492 as libc::c_int,
            );
            unreachable!();
        };
        return rdbSaveRawString(
            rdb,
            (*obj).ptr as *mut libc::c_uchar,
            sdslen((*obj).ptr as sds),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbGenericLoadStringObject(
    mut rdb: *mut rio,
    mut flags: libc::c_int,
    mut lenptr: *mut size_t,
) -> *mut libc::c_void {
    let mut encode: libc::c_int = flags & (1 as libc::c_int) << 0 as libc::c_int;
    let mut plain: libc::c_int = flags & (1 as libc::c_int) << 1 as libc::c_int;
    let mut sds: libc::c_int = flags & (1 as libc::c_int) << 2 as libc::c_int;
    let mut isencoded: libc::c_int = 0;
    let mut len: libc::c_ulonglong = 0;
    len = rdbLoadLen(rdb, &mut isencoded) as libc::c_ulonglong;
    if len == 18446744073709551615 as libc::c_ulong as libc::c_ulonglong {
        return 0 as *mut libc::c_void;
    }
    if isencoded != 0 {
        match len {
            0 | 1 | 2 => {
                return rdbLoadIntegerObject(rdb, len as libc::c_int, flags, lenptr);
            }
            3 => return rdbLoadLzfStringObject(rdb, flags, lenptr),
            _ => {
                rdbReportError(
                    1 as libc::c_int,
                    529 as libc::c_int,
                    b"Unknown RDB string encoding type %llu\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    len,
                );
                return 0 as *mut libc::c_void;
            }
        }
    }
    if plain != 0 || sds != 0 {
        let mut buf: *mut libc::c_void = if plain != 0 {
            ztrymalloc(len as size_t)
        } else {
            sdstrynewlen(SDS_NOINIT as *const libc::c_void, len as size_t)
                as *mut libc::c_void
        };
        if buf.is_null() {
            if !(((if (server.current_client).is_null()
                || (*server.current_client).id == 18446744073709551615 as libc::c_ulong
            {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 1 as libc::c_int } else { 3 as libc::c_int })
            }) & 0xff as libc::c_int) < server.verbosity)
            {
                _serverLog(
                    if (server.current_client).is_null()
                        || (*server.current_client).id
                            == 18446744073709551615 as libc::c_ulong
                    {
                        0 as libc::c_int
                    } else if 1 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        3 as libc::c_int
                    },
                    b"rdbGenericLoadStringObject failed allocating %llu bytes\0"
                        as *const u8 as *const libc::c_char,
                    len,
                );
            }
            return 0 as *mut libc::c_void;
        }
        if !lenptr.is_null() {
            *lenptr = len as size_t;
        }
        if len != 0
            && rioRead(rdb, buf, len as size_t) == 0 as libc::c_int as libc::c_ulong
        {
            if plain != 0 {
                zfree(buf);
            } else {
                sdsfree(buf as sds);
            }
            return 0 as *mut libc::c_void;
        }
        return buf;
    } else {
        let mut o: *mut robj = if encode != 0 {
            tryCreateStringObject(SDS_NOINIT, len as size_t)
        } else {
            tryCreateRawStringObject(SDS_NOINIT, len as size_t)
        };
        if o.is_null() {
            if !(((if (server.current_client).is_null()
                || (*server.current_client).id == 18446744073709551615 as libc::c_ulong
            {
                0 as libc::c_int
            } else {
                (if 1 as libc::c_int != 0 { 1 as libc::c_int } else { 3 as libc::c_int })
            }) & 0xff as libc::c_int) < server.verbosity)
            {
                _serverLog(
                    if (server.current_client).is_null()
                        || (*server.current_client).id
                            == 18446744073709551615 as libc::c_ulong
                    {
                        0 as libc::c_int
                    } else if 1 as libc::c_int != 0 {
                        1 as libc::c_int
                    } else {
                        3 as libc::c_int
                    },
                    b"rdbGenericLoadStringObject failed allocating %llu bytes\0"
                        as *const u8 as *const libc::c_char,
                    len,
                );
            }
            return 0 as *mut libc::c_void;
        }
        if len != 0
            && rioRead(rdb, (*o).ptr, len as size_t) == 0 as libc::c_int as libc::c_ulong
        {
            decrRefCount(o);
            return 0 as *mut libc::c_void;
        }
        return o as *mut libc::c_void;
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadStringObject(mut rdb: *mut rio) -> *mut robj {
    return rdbGenericLoadStringObject(rdb, 0 as libc::c_int, 0 as *mut size_t)
        as *mut robj;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadEncodedStringObject(mut rdb: *mut rio) -> *mut robj {
    return rdbGenericLoadStringObject(
        rdb,
        (1 as libc::c_int) << 0 as libc::c_int,
        0 as *mut size_t,
    ) as *mut robj;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveDoubleValue(
    mut rdb: *mut rio,
    mut val: libc::c_double,
) -> libc::c_int {
    let mut buf: [libc::c_uchar; 128] = [0; 128];
    let mut len: libc::c_int = 0;
    if val.is_nan() as i32 != 0 {
        buf[0 as libc::c_int as usize] = 253 as libc::c_int as libc::c_uchar;
        len = 1 as libc::c_int;
    } else if val.is_finite() as i32 == 0 {
        len = 1 as libc::c_int;
        buf[0 as libc::c_int
            as usize] = (if val < 0 as libc::c_int as libc::c_double {
            255 as libc::c_int
        } else {
            254 as libc::c_int
        }) as libc::c_uchar;
    } else {
        let mut lvalue: libc::c_longlong = 0;
        if double2ll(val, &mut lvalue) != 0 {
            ll2string(
                (buf.as_mut_ptr() as *mut libc::c_char)
                    .offset(1 as libc::c_int as isize),
                (core::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                lvalue,
            );
        } else {
            snprintf(
                (buf.as_mut_ptr() as *mut libc::c_char)
                    .offset(1 as libc::c_int as isize),
                (core::mem::size_of::<[libc::c_uchar; 128]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                b"%.17g\0" as *const u8 as *const libc::c_char,
                val,
            );
        }
        buf[0 as libc::c_int
            as usize] = strlen(
            (buf.as_mut_ptr() as *mut libc::c_char).offset(1 as libc::c_int as isize),
        ) as libc::c_uchar;
        len = buf[0 as libc::c_int as usize] as libc::c_int + 1 as libc::c_int;
    }
    return rdbWriteRaw(rdb, buf.as_mut_ptr() as *mut libc::c_void, len as size_t)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadDoubleValue(
    mut rdb: *mut rio,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut len: libc::c_uchar = 0;
    if rioRead(
        rdb,
        &mut len as *mut libc::c_uchar as *mut libc::c_void,
        1 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    match len as libc::c_int {
        255 => {
            *val = R_NegInf;
            return 0 as libc::c_int;
        }
        254 => {
            *val = R_PosInf;
            return 0 as libc::c_int;
        }
        253 => {
            *val = R_Nan;
            return 0 as libc::c_int;
        }
        _ => {
            if rioRead(rdb, buf.as_mut_ptr() as *mut libc::c_void, len as size_t)
                == 0 as libc::c_int as libc::c_ulong
            {
                return -(1 as libc::c_int);
            }
            buf[len as usize] = '\0' as i32 as libc::c_char;
            if sscanf(
                buf.as_mut_ptr(),
                b"%lg\0" as *const u8 as *const libc::c_char,
                val,
            ) != 1 as libc::c_int
            {
                return -(1 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveBinaryDoubleValue(
    mut rdb: *mut rio,
    mut val: libc::c_double,
) -> libc::c_int {
    return rdbWriteRaw(
        rdb,
        &mut val as *mut libc::c_double as *mut libc::c_void,
        core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadBinaryDoubleValue(
    mut rdb: *mut rio,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    if rioRead(
        rdb,
        val as *mut libc::c_void,
        core::mem::size_of::<libc::c_double>() as libc::c_ulong,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveBinaryFloatValue(
    mut rdb: *mut rio,
    mut val: libc::c_float,
) -> libc::c_int {
    return rdbWriteRaw(
        rdb,
        &mut val as *mut libc::c_float as *mut libc::c_void,
        core::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadBinaryFloatValue(
    mut rdb: *mut rio,
    mut val: *mut libc::c_float,
) -> libc::c_int {
    if rioRead(
        rdb,
        val as *mut libc::c_void,
        core::mem::size_of::<libc::c_float>() as libc::c_ulong,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveObjectType(
    mut rdb: *mut rio,
    mut o: *mut robj,
) -> libc::c_int {
    's_100: {
        let mut current_block_21: u64;
        match (*o).type_0() as libc::c_int {
            0 => return rdbSaveType(rdb, 0 as libc::c_int as libc::c_uchar),
            1 => {
                if (*o).encoding() as libc::c_int == 9 as libc::c_int {
                    return rdbSaveType(rdb, 18 as libc::c_int as libc::c_uchar)
                } else {
                    _serverPanic(
                        b"rdb.c\0" as *const u8 as *const libc::c_char,
                        661 as libc::c_int,
                        b"Unknown list encoding\0" as *const u8 as *const libc::c_char,
                    );
                    unreachable!();
                }
                current_block_21 = 9205717943706311272;
            }
            2 => {
                current_block_21 = 9205717943706311272;
            }
            3 => {
                current_block_21 = 1760194286177680343;
            }
            4 => {
                current_block_21 = 459104737412379174;
            }
            6 => {
                current_block_21 = 1017447862132263635;
            }
            5 => return rdbSaveType(rdb, 7 as libc::c_int as libc::c_uchar),
            _ => {
                _serverPanic(
                    b"rdb.c\0" as *const u8 as *const libc::c_char,
                    688 as libc::c_int,
                    b"Unknown object type\0" as *const u8 as *const libc::c_char,
                );
                unreachable!();
                break 's_100;
            }
        }
        match current_block_21 {
            9205717943706311272 => {
                if (*o).encoding() as libc::c_int == 6 as libc::c_int {
                    return rdbSaveType(rdb, 11 as libc::c_int as libc::c_uchar)
                } else {
                    if (*o).encoding() as libc::c_int == 2 as libc::c_int {
                        return rdbSaveType(rdb, 2 as libc::c_int as libc::c_uchar)
                    } else {
                        _serverPanic(
                            b"rdb.c\0" as *const u8 as *const libc::c_char,
                            668 as libc::c_int,
                            b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
                        );
                        unreachable!();
                    }
                }
                current_block_21 = 1760194286177680343;
            }
            _ => {}
        }
        match current_block_21 {
            1760194286177680343 => {
                if (*o).encoding() as libc::c_int == 11 as libc::c_int {
                    return rdbSaveType(rdb, 17 as libc::c_int as libc::c_uchar)
                } else {
                    if (*o).encoding() as libc::c_int == 7 as libc::c_int {
                        return rdbSaveType(rdb, 5 as libc::c_int as libc::c_uchar)
                    } else {
                        _serverPanic(
                            b"rdb.c\0" as *const u8 as *const libc::c_char,
                            675 as libc::c_int,
                            b"Unknown sorted set encoding\0" as *const u8
                                as *const libc::c_char,
                        );
                        unreachable!();
                    }
                }
                current_block_21 = 459104737412379174;
            }
            _ => {}
        }
        match current_block_21 {
            459104737412379174 => {
                if (*o).encoding() as libc::c_int == 11 as libc::c_int {
                    return rdbSaveType(rdb, 16 as libc::c_int as libc::c_uchar)
                } else {
                    if (*o).encoding() as libc::c_int == 2 as libc::c_int {
                        return rdbSaveType(rdb, 4 as libc::c_int as libc::c_uchar)
                    } else {
                        _serverPanic(
                            b"rdb.c\0" as *const u8 as *const libc::c_char,
                            682 as libc::c_int,
                            b"Unknown hash encoding\0" as *const u8
                                as *const libc::c_char,
                        );
                        unreachable!();
                    }
                }
            }
            _ => {}
        }
        return rdbSaveType(rdb, 19 as libc::c_int as libc::c_uchar);
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadObjectType(mut rdb: *mut rio) -> libc::c_int {
    let mut type_0: libc::c_int = 0;
    type_0 = rdbLoadType(rdb);
    if type_0 == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if !(type_0 >= 0 as libc::c_int && type_0 <= 7 as libc::c_int
        || type_0 >= 9 as libc::c_int && type_0 <= 19 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    return type_0;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveStreamPEL(
    mut rdb: *mut rio,
    mut pel: *mut rax,
    mut nacks: libc::c_int,
) -> ssize_t {
    let mut n: ssize_t = 0;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    n = rdbSaveLen(rdb, raxSize(pel)) as ssize_t;
    if n == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    nwritten += n;
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
    raxStart(&mut ri, pel);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        n = rdbWriteRaw(
            rdb,
            ri.key as *mut libc::c_void,
            core::mem::size_of::<streamID>() as libc::c_ulong,
        );
        if n == -(1 as libc::c_int) as libc::c_long {
            raxStop(&mut ri);
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        if nacks != 0 {
            let mut nack: *mut streamNACK = ri.data as *mut streamNACK;
            n = rdbSaveMillisecondTime(rdb, (*nack).delivery_time) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                raxStop(&mut ri);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            n = rdbSaveLen(rdb, (*nack).delivery_count) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                raxStop(&mut ri);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
        }
    }
    raxStop(&mut ri);
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveStreamConsumers(
    mut rdb: *mut rio,
    mut cg: *mut streamCG,
) -> size_t {
    let mut n: ssize_t = 0;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    n = rdbSaveLen(rdb, raxSize((*cg).consumers)) as ssize_t;
    if n == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as size_t;
    }
    nwritten += n;
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
    raxStart(&mut ri, (*cg).consumers);
    raxSeek(
        &mut ri,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut ri) != 0 {
        let mut consumer: *mut streamConsumer = ri.data as *mut streamConsumer;
        n = rdbSaveRawString(rdb, ri.key, ri.key_len);
        if n == -(1 as libc::c_int) as libc::c_long {
            raxStop(&mut ri);
            return -(1 as libc::c_int) as size_t;
        }
        nwritten += n;
        n = rdbSaveMillisecondTime(rdb, (*consumer).seen_time) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            raxStop(&mut ri);
            return -(1 as libc::c_int) as size_t;
        }
        nwritten += n;
        n = rdbSaveStreamPEL(rdb, (*consumer).pel, 0 as libc::c_int);
        if n == -(1 as libc::c_int) as libc::c_long {
            raxStop(&mut ri);
            return -(1 as libc::c_int) as size_t;
        }
        nwritten += n;
    }
    raxStop(&mut ri);
    return nwritten as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveObject(
    mut rdb: *mut rio,
    mut o: *mut robj,
    mut key: *mut robj,
    mut dbid: libc::c_int,
) -> ssize_t {
    let mut n: ssize_t = 0 as libc::c_int as ssize_t;
    let mut nwritten: ssize_t = 0 as libc::c_int as ssize_t;
    if (*o).type_0() as libc::c_int == 0 as libc::c_int {
        n = rdbSaveStringObject(rdb, o);
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
    } else if (*o).type_0() as libc::c_int == 1 as libc::c_int {
        if (*o).encoding() as libc::c_int == 9 as libc::c_int {
            let mut ql: *mut quicklist = (*o).ptr as *mut quicklist;
            let mut node: *mut quicklistNode = (*ql).head;
            n = rdbSaveLen(rdb, (*ql).len) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            while !node.is_null() {
                n = rdbSaveLen(rdb, (*node).container() as uint64_t) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                if (*node).encoding() as libc::c_int == 2 as libc::c_int {
                    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
                    let mut compress_len: size_t = quicklistGetLzf(node, &mut data);
                    n = rdbSaveLzfBlob(rdb, data, compress_len, (*node).sz);
                    if n == -(1 as libc::c_int) as libc::c_long {
                        return -(1 as libc::c_int) as ssize_t;
                    }
                    nwritten += n;
                } else {
                    n = rdbSaveRawString(rdb, (*node).entry, (*node).sz);
                    if n == -(1 as libc::c_int) as libc::c_long {
                        return -(1 as libc::c_int) as ssize_t;
                    }
                    nwritten += n;
                }
                node = (*node).next;
            }
        } else {
            _serverPanic(
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                829 as libc::c_int,
                b"Unknown list encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            let mut set: *mut dict = (*o).ptr as *mut dict;
            let mut di: *mut dictIterator = dictGetIterator(set);
            let mut de: *mut dictEntry = 0 as *mut dictEntry;
            n = rdbSaveLen(
                rdb,
                ((*set).ht_used[0 as libc::c_int as usize])
                    .wrapping_add((*set).ht_used[1 as libc::c_int as usize]),
            ) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                dictReleaseIterator(di);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            loop {
                de = dictNext(di);
                if de.is_null() {
                    break;
                }
                let mut ele: sds = (*de).key as sds;
                n = rdbSaveRawString(rdb, ele as *mut libc::c_uchar, sdslen(ele));
                if n == -(1 as libc::c_int) as libc::c_long {
                    dictReleaseIterator(di);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
            }
            dictReleaseIterator(di);
        } else if (*o).encoding() as libc::c_int == 6 as libc::c_int {
            let mut l: size_t = intsetBlobLen((*o).ptr as *mut intset);
            n = rdbSaveRawString(rdb, (*o).ptr as *mut libc::c_uchar, l);
            if n == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
        } else {
            _serverPanic(
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                861 as libc::c_int,
                b"Unknown set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        if (*o).encoding() as libc::c_int == 11 as libc::c_int {
            let mut l_0: size_t = lpBytes((*o).ptr as *mut libc::c_uchar);
            n = rdbSaveRawString(rdb, (*o).ptr as *mut libc::c_uchar, l_0);
            if n == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
        } else if (*o).encoding() as libc::c_int == 7 as libc::c_int {
            let mut zs: *mut zset = (*o).ptr as *mut zset;
            let mut zsl: *mut zskiplist = (*zs).zsl;
            n = rdbSaveLen(rdb, (*zsl).length) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            let mut zn: *mut zskiplistNode = (*zsl).tail;
            while !zn.is_null() {
                n = rdbSaveRawString(
                    rdb,
                    (*zn).ele as *mut libc::c_uchar,
                    sdslen((*zn).ele),
                );
                if n == -(1 as libc::c_int) as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveBinaryDoubleValue(rdb, (*zn).score) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                zn = (*zn).backward;
            }
        } else {
            _serverPanic(
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                897 as libc::c_int,
                b"Unknown sorted set encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        if (*o).encoding() as libc::c_int == 11 as libc::c_int {
            let mut l_1: size_t = lpBytes((*o).ptr as *mut libc::c_uchar);
            n = rdbSaveRawString(rdb, (*o).ptr as *mut libc::c_uchar, l_1);
            if n == -(1 as libc::c_int) as libc::c_long {
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
        } else if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            let mut di_0: *mut dictIterator = dictGetIterator((*o).ptr as *mut dict);
            let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
            n = rdbSaveLen(
                rdb,
                ((*((*o).ptr as *mut dict)).ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*((*o).ptr as *mut dict)).ht_used[1 as libc::c_int as usize],
                    ),
            ) as ssize_t;
            if n == -(1 as libc::c_int) as libc::c_long {
                dictReleaseIterator(di_0);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            loop {
                de_0 = dictNext(di_0);
                if de_0.is_null() {
                    break;
                }
                let mut field: sds = (*de_0).key as sds;
                let mut value: sds = (*de_0).v.val as sds;
                n = rdbSaveRawString(rdb, field as *mut libc::c_uchar, sdslen(field));
                if n == -(1 as libc::c_int) as libc::c_long {
                    dictReleaseIterator(di_0);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveRawString(rdb, value as *mut libc::c_uchar, sdslen(value));
                if n == -(1 as libc::c_int) as libc::c_long {
                    dictReleaseIterator(di_0);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
            }
            dictReleaseIterator(di_0);
        } else {
            _serverPanic(
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                937 as libc::c_int,
                b"Unknown hash encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    } else if (*o).type_0() as libc::c_int == 6 as libc::c_int {
        let mut s: *mut stream = (*o).ptr as *mut stream;
        let mut rax: *mut rax = (*s).rax;
        n = rdbSaveLen(rdb, raxSize(rax)) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
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
        raxStart(&mut ri, rax);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        while raxNext(&mut ri) != 0 {
            let mut lp: *mut libc::c_uchar = ri.data as *mut libc::c_uchar;
            let mut lp_bytes: size_t = lpBytes(lp);
            n = rdbSaveRawString(rdb, ri.key, ri.key_len);
            if n == -(1 as libc::c_int) as libc::c_long {
                raxStop(&mut ri);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
            n = rdbSaveRawString(rdb, lp, lp_bytes);
            if n == -(1 as libc::c_int) as libc::c_long {
                raxStop(&mut ri);
                return -(1 as libc::c_int) as ssize_t;
            }
            nwritten += n;
        }
        raxStop(&mut ri);
        n = rdbSaveLen(rdb, (*s).length) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).last_id.ms) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).last_id.seq) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).first_id.ms) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).first_id.seq) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).max_deleted_entry_id.ms) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).max_deleted_entry_id.seq) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        n = rdbSaveLen(rdb, (*s).entries_added) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        let mut num_cgroups: size_t = if !((*s).cgroups).is_null() {
            raxSize((*s).cgroups)
        } else {
            0 as libc::c_int as libc::c_ulong
        };
        n = rdbSaveLen(rdb, num_cgroups) as ssize_t;
        if n == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        nwritten += n;
        if num_cgroups != 0 {
            raxStart(&mut ri, (*s).cgroups);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            while raxNext(&mut ri) != 0 {
                let mut cg: *mut streamCG = ri.data as *mut streamCG;
                n = rdbSaveRawString(rdb, ri.key, ri.key_len);
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveLen(rdb, (*cg).last_id.ms) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveLen(rdb, (*cg).last_id.seq) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveLen(rdb, (*cg).entries_read as uint64_t) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveStreamPEL(rdb, (*cg).pel, 1 as libc::c_int);
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
                n = rdbSaveStreamConsumers(rdb, cg) as ssize_t;
                if n == -(1 as libc::c_int) as libc::c_long {
                    raxStop(&mut ri);
                    return -(1 as libc::c_int) as ssize_t;
                }
                nwritten += n;
            }
            raxStop(&mut ri);
        }
    } else if (*o).type_0() as libc::c_int == 5 as libc::c_int {
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
        let mut retval: libc::c_int = rdbSaveLen(rdb, (*mt).id);
        if retval == -(1 as libc::c_int) {
            return -(1 as libc::c_int) as ssize_t;
        }
        io.rio = rdb;
        io.type_0 = mt;
        io.bytes = 0 as libc::c_int as size_t;
        io.error = 0 as libc::c_int;
        io.ver = 0 as libc::c_int;
        io.key = key;
        io.dbid = dbid;
        io.ctx = 0 as *mut RedisModuleCtx;
        io
            .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        ((*mt).rdb_save).expect("non-null function pointer")(&mut io, (*mv).value);
        retval = rdbSaveLen(rdb, 0 as libc::c_int as uint64_t);
        if retval == -(1 as libc::c_int) {
            io.error = 1 as libc::c_int;
        } else {
            io
                .bytes = (io.bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
        }
        if !(io.ctx).is_null() {
            moduleFreeContext(io.ctx);
            zfree(io.ctx as *mut libc::c_void);
        }
        return if io.error != 0 {
            -(1 as libc::c_int) as libc::c_long
        } else {
            io.bytes as ssize_t
        };
    } else {
        _serverPanic(
            b"rdb.c\0" as *const u8 as *const libc::c_char,
            1076 as libc::c_int,
            b"Unknown object type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    return nwritten;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSavedObjectLen(
    mut o: *mut robj,
    mut key: *mut robj,
    mut dbid: libc::c_int,
) -> size_t {
    let mut len: ssize_t = rdbSaveObject(0 as *mut rio, o, key, dbid);
    if len != -(1 as libc::c_int) as libc::c_long {} else {
        _serverAssertWithInfo(
            0 as *const client,
            o,
            b"len != -1\0" as *const u8 as *const libc::c_char,
            b"rdb.c\0" as *const u8 as *const libc::c_char,
            1087 as libc::c_int,
        );
        unreachable!();
    };
    return len as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveKeyValuePair(
    mut rdb: *mut rio,
    mut key: *mut robj,
    mut val: *mut robj,
    mut expiretime: libc::c_longlong,
    mut dbid: libc::c_int,
) -> libc::c_int {
    let mut savelru: libc::c_int = server.maxmemory_policy
        & (1 as libc::c_int) << 0 as libc::c_int;
    let mut savelfu: libc::c_int = server.maxmemory_policy
        & (1 as libc::c_int) << 1 as libc::c_int;
    if expiretime != -(1 as libc::c_int) as libc::c_longlong {
        if rdbSaveType(rdb, 252 as libc::c_int as libc::c_uchar) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if rdbSaveMillisecondTime(rdb, expiretime) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if savelru != 0 {
        let mut idletime: uint64_t = estimateObjectIdleTime(val) as uint64_t;
        idletime = (idletime as libc::c_ulong)
            .wrapping_div(1000 as libc::c_int as libc::c_ulong) as uint64_t as uint64_t;
        if rdbSaveType(rdb, 248 as libc::c_int as libc::c_uchar) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if rdbSaveLen(rdb, idletime) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if savelfu != 0 {
        let mut buf: [uint8_t; 1] = [0; 1];
        buf[0 as libc::c_int as usize] = LFUDecrAndReturn(val) as uint8_t;
        if rdbSaveType(rdb, 249 as libc::c_int as libc::c_uchar) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if rdbWriteRaw(
            rdb,
            buf.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
    }
    if rdbSaveObjectType(rdb, val) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if rdbSaveStringObject(rdb, key) == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int);
    }
    if rdbSaveObject(rdb, val, key, dbid) == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int);
    }
    if server.rdb_key_save_delay != 0 {
        debugDelay(server.rdb_key_save_delay);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveAuxField(
    mut rdb: *mut rio,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
    mut val: *mut libc::c_void,
    mut vallen: size_t,
) -> ssize_t {
    let mut ret: ssize_t = 0;
    let mut len: ssize_t = 0 as libc::c_int as ssize_t;
    ret = rdbSaveType(rdb, 250 as libc::c_int as libc::c_uchar) as ssize_t;
    if ret == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    len += ret;
    ret = rdbSaveRawString(rdb, key as *mut libc::c_uchar, keylen);
    if ret == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    len += ret;
    ret = rdbSaveRawString(rdb, val as *mut libc::c_uchar, vallen);
    if ret == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as ssize_t;
    }
    len += ret;
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveAuxFieldStrStr(
    mut rdb: *mut rio,
    mut key: *mut libc::c_char,
    mut val: *mut libc::c_char,
) -> ssize_t {
    return rdbSaveAuxField(
        rdb,
        key as *mut libc::c_void,
        strlen(key),
        val as *mut libc::c_void,
        strlen(val),
    );
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveAuxFieldStrInt(
    mut rdb: *mut rio,
    mut key: *mut libc::c_char,
    mut val: libc::c_longlong,
) -> ssize_t {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut vlen: libc::c_int = ll2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
        val,
    );
    return rdbSaveAuxField(
        rdb,
        key as *mut libc::c_void,
        strlen(key),
        buf.as_mut_ptr() as *mut libc::c_void,
        vlen as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveInfoAuxFields(
    mut rdb: *mut rio,
    mut rdbflags: libc::c_int,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut redis_bits: libc::c_int = if core::mem::size_of::<*mut libc::c_void>()
        as libc::c_ulong == 8 as libc::c_int as libc::c_ulong
    {
        64 as libc::c_int
    } else {
        32 as libc::c_int
    };
    let mut aof_base: libc::c_int = (rdbflags & (1 as libc::c_int) << 0 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    if rdbSaveAuxFieldStrStr(
        rdb,
        b"redis-ver\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"7.0.8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if rdbSaveAuxFieldStrInt(
        rdb,
        b"redis-bits\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        redis_bits as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if rdbSaveAuxFieldStrInt(
        rdb,
        b"ctime\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        time(0 as *mut time_t) as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if rdbSaveAuxFieldStrInt(
        rdb,
        b"used-mem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        zmalloc_used_memory() as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    if !rsi.is_null() {
        if rdbSaveAuxFieldStrInt(
            rdb,
            b"repl-stream-db\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (*rsi).repl_stream_db as libc::c_longlong,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        if rdbSaveAuxFieldStrStr(
            rdb,
            b"repl-id\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            (server.replid).as_mut_ptr(),
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
        if rdbSaveAuxFieldStrInt(
            rdb,
            b"repl-offset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            server.master_repl_offset,
        ) == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int);
        }
    }
    if rdbSaveAuxFieldStrInt(
        rdb,
        b"aof-base\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        aof_base as libc::c_longlong,
    ) == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveSingleModuleAux(
    mut rdb: *mut rio,
    mut when: libc::c_int,
    mut mt: *mut moduleType,
) -> ssize_t {
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
    let mut retval: libc::c_int = rdbSaveType(rdb, 247 as libc::c_int as libc::c_uchar);
    if retval == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as ssize_t;
    }
    io.rio = rdb;
    io.type_0 = mt;
    io.bytes = 0 as libc::c_int as size_t;
    io.error = 0 as libc::c_int;
    io.ver = 0 as libc::c_int;
    io.key = 0 as *mut redisObject;
    io.dbid = -(1 as libc::c_int);
    io.ctx = 0 as *mut RedisModuleCtx;
    io
        .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
        as size_t as size_t;
    retval = rdbSaveLen(rdb, (*mt).id);
    if retval == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as ssize_t;
    }
    io
        .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
        as size_t as size_t;
    retval = rdbSaveLen(rdb, 2 as libc::c_int as uint64_t);
    if retval == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as ssize_t;
    }
    io
        .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
        as size_t as size_t;
    retval = rdbSaveLen(rdb, when as uint64_t);
    if retval == -(1 as libc::c_int) {
        return -(1 as libc::c_int) as ssize_t;
    }
    io
        .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
        as size_t as size_t;
    ((*mt).aux_save).expect("non-null function pointer")(&mut io, when);
    retval = rdbSaveLen(rdb, 0 as libc::c_int as uint64_t);
    if retval == -(1 as libc::c_int) {
        io.error = 1 as libc::c_int;
    } else {
        io
            .bytes = (io.bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
    }
    if !(io.ctx).is_null() {
        moduleFreeContext(io.ctx);
        zfree(io.ctx as *mut libc::c_void);
    }
    if io.error != 0 {
        return -(1 as libc::c_int) as ssize_t;
    }
    return io.bytes as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveFunctions(mut rdb: *mut rio) -> ssize_t {
    let mut current_block: u64;
    let mut functions: *mut dict = functionsLibGet();
    let mut iter: *mut dictIterator = dictGetIterator(functions);
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut written: ssize_t = 0 as libc::c_int as ssize_t;
    let mut ret: ssize_t = 0;
    loop {
        entry = dictNext(iter);
        if entry.is_null() {
            current_block = 10879442775620481940;
            break;
        }
        ret = rdbSaveType(rdb, 245 as libc::c_int as libc::c_uchar) as ssize_t;
        if ret < 0 as libc::c_int as libc::c_long {
            current_block = 17565661220666970011;
            break;
        }
        written += ret;
        let mut li: *mut functionLibInfo = (*entry).v.val as *mut functionLibInfo;
        ret = rdbSaveRawString(
            rdb,
            (*li).code as *mut libc::c_uchar,
            sdslen((*li).code),
        );
        if ret < 0 as libc::c_int as libc::c_long {
            current_block = 17565661220666970011;
            break;
        }
        written += ret;
    }
    match current_block {
        17565661220666970011 => {
            dictReleaseIterator(iter);
            return -(1 as libc::c_int) as ssize_t;
        }
        _ => {
            dictReleaseIterator(iter);
            return written;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveDb(
    mut rdb: *mut rio,
    mut dbid: libc::c_int,
    mut rdbflags: libc::c_int,
    mut key_counter: *mut libc::c_long,
) -> ssize_t {
    let mut db_size: uint64_t = 0;
    let mut expires_size: uint64_t = 0;
    let mut current_block: u64;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut written: ssize_t = 0 as libc::c_int as ssize_t;
    let mut res: ssize_t = 0;
    static mut info_updated_time: libc::c_longlong = 0 as libc::c_int
        as libc::c_longlong;
    let mut pname: *mut libc::c_char = (if rdbflags
        & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        b"AOF rewrite\0" as *const u8 as *const libc::c_char
    } else {
        b"RDB\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    let mut db: *mut redisDb = (server.db).offset(dbid as isize);
    let mut d: *mut dict = (*db).dict;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as ssize_t;
    }
    di = dictGetSafeIterator(d);
    res = rdbSaveType(rdb, 254 as libc::c_int as libc::c_uchar) as ssize_t;
    if !(res < 0 as libc::c_int as libc::c_long) {
        written += res;
        res = rdbSaveLen(rdb, dbid as uint64_t) as ssize_t;
        if !(res < 0 as libc::c_int as libc::c_long) {
            written += res;
            db_size = 0;
            expires_size = 0;
            db_size = ((*(*db).dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*db).dict).ht_used[1 as libc::c_int as usize]);
            expires_size = ((*(*db).expires).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*db).expires).ht_used[1 as libc::c_int as usize]);
            res = rdbSaveType(rdb, 251 as libc::c_int as libc::c_uchar) as ssize_t;
            if !(res < 0 as libc::c_int as libc::c_long) {
                written += res;
                res = rdbSaveLen(rdb, db_size) as ssize_t;
                if !(res < 0 as libc::c_int as libc::c_long) {
                    written += res;
                    res = rdbSaveLen(rdb, expires_size) as ssize_t;
                    if !(res < 0 as libc::c_int as libc::c_long) {
                        written += res;
                        loop {
                            de = dictNext(di);
                            if de.is_null() {
                                current_block = 15897653523371991391;
                                break;
                            }
                            let mut keystr: sds = (*de).key as sds;
                            let mut key: robj = robj {
                                type_0_encoding_lru: [0; 4],
                                refcount: 0,
                                ptr: 0 as *mut libc::c_void,
                            };
                            let mut o: *mut robj = (*de).v.val as *mut robj;
                            let mut expire: libc::c_longlong = 0;
                            let mut rdb_bytes_before_key: size_t = (*rdb)
                                .processed_bytes;
                            key.refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
                            key.set_type_0(0 as libc::c_int as libc::c_uint);
                            key.set_encoding(0 as libc::c_int as libc::c_uint);
                            key.ptr = keystr as *mut libc::c_void;
                            expire = getExpire(db, &mut key);
                            res = rdbSaveKeyValuePair(rdb, &mut key, o, expire, dbid)
                                as ssize_t;
                            if res < 0 as libc::c_int as libc::c_long {
                                current_block = 15205125493042018371;
                                break;
                            }
                            written += res;
                            let mut dump_size: size_t = ((*rdb).processed_bytes)
                                .wrapping_sub(rdb_bytes_before_key);
                            if server.in_fork_child != 0 {
                                dismissObject(o, dump_size);
                            }
                            let fresh0 = *key_counter;
                            *key_counter = *key_counter + 1;
                            if fresh0 & 1023 as libc::c_int as libc::c_long
                                == 0 as libc::c_int as libc::c_long
                            {
                                let mut now: libc::c_longlong = mstime();
                                if now - info_updated_time
                                    >= 1000 as libc::c_int as libc::c_longlong
                                {
                                    sendChildInfo(
                                        CHILD_INFO_TYPE_CURRENT_INFO,
                                        *key_counter as size_t,
                                        pname,
                                    );
                                    info_updated_time = now;
                                }
                            }
                        }
                        match current_block {
                            15205125493042018371 => {}
                            _ => {
                                dictReleaseIterator(di);
                                return written;
                            }
                        }
                    }
                }
            }
        }
    }
    dictReleaseIterator(di);
    return -(1 as libc::c_int) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveRio(
    mut req: libc::c_int,
    mut rdb: *mut rio,
    mut error: *mut libc::c_int,
    mut rdbflags: libc::c_int,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut current_block: u64;
    let mut magic: [libc::c_char; 10] = [0; 10];
    let mut cksum: uint64_t = 0;
    let mut key_counter: libc::c_long = 0 as libc::c_int as libc::c_long;
    let mut j: libc::c_int = 0;
    if server.rdb_checksum != 0 {
        (*rdb)
            .update_cksum = Some(
            rioGenericUpdateChecksum
                as unsafe extern "C" fn(*mut rio, *const libc::c_void, size_t) -> (),
        );
    }
    snprintf(
        magic.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
        b"REDIS%04d\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
    );
    if !(rdbWriteRaw(
        rdb,
        magic.as_mut_ptr() as *mut libc::c_void,
        9 as libc::c_int as size_t,
    ) == -(1 as libc::c_int) as libc::c_long)
    {
        if !(rdbSaveInfoAuxFields(rdb, rdbflags, rsi) == -(1 as libc::c_int)) {
            if !(req & (1 as libc::c_int) << 0 as libc::c_int == 0
                && rdbSaveModulesAux(rdb, (1 as libc::c_int) << 0 as libc::c_int)
                    == -(1 as libc::c_int) as libc::c_long)
            {
                if !(req & (1 as libc::c_int) << 1 as libc::c_int == 0
                    && rdbSaveFunctions(rdb) == -(1 as libc::c_int) as libc::c_long)
                {
                    if req & (1 as libc::c_int) << 0 as libc::c_int == 0 {
                        j = 0 as libc::c_int;
                        loop {
                            if !(j < server.dbnum) {
                                current_block = 1856101646708284338;
                                break;
                            }
                            if rdbSaveDb(rdb, j, rdbflags, &mut key_counter)
                                == -(1 as libc::c_int) as libc::c_long
                            {
                                current_block = 17116435445121809148;
                                break;
                            }
                            j += 1;
                        }
                    } else {
                        current_block = 1856101646708284338;
                    }
                    match current_block {
                        17116435445121809148 => {}
                        _ => {
                            if !(req & (1 as libc::c_int) << 0 as libc::c_int == 0
                                && rdbSaveModulesAux(
                                    rdb,
                                    (1 as libc::c_int) << 1 as libc::c_int,
                                ) == -(1 as libc::c_int) as libc::c_long)
                            {
                                if !(rdbSaveType(rdb, 255 as libc::c_int as libc::c_uchar)
                                    == -(1 as libc::c_int))
                                {
                                    cksum = (*rdb).cksum;
                                    if !(rioWrite(
                                        rdb,
                                        &mut cksum as *mut uint64_t as *const libc::c_void,
                                        8 as libc::c_int as size_t,
                                    ) == 0 as libc::c_int as libc::c_ulong)
                                    {
                                        return 0 as libc::c_int;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !error.is_null() {
        *error = *__errno_location();
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveRioWithEOFMark(
    mut req: libc::c_int,
    mut rdb: *mut rio,
    mut error: *mut libc::c_int,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut eofmark: [libc::c_char; 40] = [0; 40];
    startSaving((1 as libc::c_int) << 1 as libc::c_int);
    getRandomHexChars(eofmark.as_mut_ptr(), 40 as libc::c_int as size_t);
    if !error.is_null() {
        *error = 0 as libc::c_int;
    }
    if !(rioWrite(
        rdb,
        b"$EOF:\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        5 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong)
    {
        if !(rioWrite(
            rdb,
            eofmark.as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
        ) == 0 as libc::c_int as libc::c_ulong)
        {
            if !(rioWrite(
                rdb,
                b"\r\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                2 as libc::c_int as size_t,
            ) == 0 as libc::c_int as libc::c_ulong)
            {
                if !(rdbSaveRio(req, rdb, error, 0 as libc::c_int, rsi)
                    == -(1 as libc::c_int))
                {
                    if !(rioWrite(
                        rdb,
                        eofmark.as_mut_ptr() as *const libc::c_void,
                        40 as libc::c_int as size_t,
                    ) == 0 as libc::c_int as libc::c_ulong)
                    {
                        stopSaving(1 as libc::c_int);
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if !error.is_null() && *error == 0 as libc::c_int {
        *error = *__errno_location();
    }
    stopSaving(0 as libc::c_int);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbSave(
    mut req: libc::c_int,
    mut filename: *mut libc::c_char,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut tmpfile: [libc::c_char; 256] = [0; 256];
    let mut cwd: [libc::c_char; 4096] = [0; 4096];
    let mut fp: *mut FILE = 0 as *mut FILE;
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
    let mut error: libc::c_int = 0 as libc::c_int;
    let mut err_op: *mut libc::c_char = 0 as *mut libc::c_char;
    snprintf(
        tmpfile.as_mut_ptr(),
        256 as libc::c_int as libc::c_ulong,
        b"temp-%d.rdb\0" as *const u8 as *const libc::c_char,
        getpid(),
    );
    fp = fopen(tmpfile.as_mut_ptr(), b"w\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        let mut str_err: *mut libc::c_char = strerror(*__errno_location());
        let mut cwdp: *mut libc::c_char = getcwd(
            cwd.as_mut_ptr(),
            4096 as libc::c_int as size_t,
        );
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed opening the temp RDB file %s (in server root dir %s) for saving: %s\0"
                    as *const u8 as *const libc::c_char,
                tmpfile.as_mut_ptr(),
                if !cwdp.is_null() {
                    cwdp as *const libc::c_char
                } else {
                    b"unknown\0" as *const u8 as *const libc::c_char
                },
                str_err,
            );
        }
        return -(1 as libc::c_int);
    }
    rioInitWithFile(&mut rdb, fp);
    startSaving(0 as libc::c_int);
    if server.rdb_save_incremental_fsync != 0 {
        rioSetAutoSync(
            &mut rdb,
            (1024 as libc::c_int * 1024 as libc::c_int * 4 as libc::c_int) as off_t,
        );
    }
    if rdbSaveRio(req, &mut rdb, &mut error, 0 as libc::c_int, rsi)
        == -(1 as libc::c_int)
    {
        *__errno_location() = error;
        err_op = b"rdbSaveRio\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
    } else if fflush(fp) != 0 {
        err_op = b"fflush\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if fsync(fileno(fp)) != 0 {
        err_op = b"fsync\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if fclose(fp) != 0 {
        fp = 0 as *mut FILE;
        err_op = b"fclose\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        fp = 0 as *mut FILE;
        if rename(tmpfile.as_mut_ptr(), filename) == -(1 as libc::c_int) {
            let mut str_err_0: *mut libc::c_char = strerror(*__errno_location());
            let mut cwdp_0: *mut libc::c_char = getcwd(
                cwd.as_mut_ptr(),
                4096 as libc::c_int as size_t,
            );
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Error moving temp DB file %s on the final destination %s (in server root dir %s): %s\0"
                        as *const u8 as *const libc::c_char,
                    tmpfile.as_mut_ptr(),
                    filename,
                    if !cwdp_0.is_null() {
                        cwdp_0 as *const libc::c_char
                    } else {
                        b"unknown\0" as *const u8 as *const libc::c_char
                    },
                    str_err_0,
                );
            }
            unlink(tmpfile.as_mut_ptr());
            stopSaving(0 as libc::c_int);
            return -(1 as libc::c_int);
        }
        if fsyncFileDir(filename) == -(1 as libc::c_int) {
            err_op = b"fsyncFileDir\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"DB saved on disk\0" as *const u8 as *const libc::c_char,
                );
            }
            server.dirty = 0 as libc::c_int as libc::c_longlong;
            server.lastsave = time(0 as *mut time_t);
            server.lastbgsave_status = 0 as libc::c_int;
            stopSaving(1 as libc::c_int);
            return 0 as libc::c_int;
        }
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Write error saving DB on disk(%s): %s\0" as *const u8
                as *const libc::c_char,
            err_op,
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
pub unsafe extern "C" fn rdbSaveBackground(
    mut req: libc::c_int,
    mut filename: *mut libc::c_char,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut childpid: pid_t = 0;
    if hasActiveChildProcess() != 0 {
        return -(1 as libc::c_int);
    }
    server.stat_rdb_saves += 1;
    server.dirty_before_bgsave = server.dirty;
    server.lastbgsave_try = time(0 as *mut time_t);
    childpid = redisFork(1 as libc::c_int);
    if childpid == 0 as libc::c_int {
        let mut retval: libc::c_int = 0;
        redisSetProcTitle(
            b"redis-rdb-bgsave\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        redisSetCpuAffinity(server.bgsave_cpulist);
        retval = rdbSave(req, filename, rsi);
        if retval == 0 as libc::c_int {
            sendChildCowInfo(
                CHILD_INFO_TYPE_RDB_COW_SIZE,
                b"RDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        exitFromChild(
            if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
        );
    } else {
        if childpid == -(1 as libc::c_int) {
            server.lastbgsave_status = -(1 as libc::c_int);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't save in background: fork: %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            return -(1 as libc::c_int);
        }
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Background saving started by pid %ld\0" as *const u8
                    as *const libc::c_char,
                childpid as libc::c_long,
            );
        }
        server.rdb_save_time_start = time(0 as *mut time_t);
        server.rdb_child_type = 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbRemoveTempFile(
    mut childpid: pid_t,
    mut from_signal: libc::c_int,
) {
    let mut tmpfile: [libc::c_char; 256] = [0; 256];
    let mut pid: [libc::c_char; 32] = [0; 32];
    let mut pid_len: libc::c_int = ll2string(
        pid.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
        childpid as libc::c_longlong,
    );
    strcpy(tmpfile.as_mut_ptr(), b"temp-\0" as *const u8 as *const libc::c_char);
    strncpy(
        tmpfile.as_mut_ptr().offset(5 as libc::c_int as isize),
        pid.as_mut_ptr(),
        pid_len as libc::c_ulong,
    );
    strcpy(
        tmpfile.as_mut_ptr().offset(5 as libc::c_int as isize).offset(pid_len as isize),
        b".rdb\0" as *const u8 as *const libc::c_char,
    );
    if from_signal != 0 {
        let mut fd: libc::c_int = open(
            tmpfile.as_mut_ptr(),
            0 as libc::c_int | 0o4000 as libc::c_int,
        );
        unlink(tmpfile.as_mut_ptr());
    } else {
        bg_unlink(tmpfile.as_mut_ptr());
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadCheckModuleValue(
    mut rdb: *mut rio,
    mut modulename: *mut libc::c_char,
) -> *mut robj {
    let mut opcode: uint64_t = 0;
    loop {
        opcode = rdbLoadLen(rdb, 0 as *mut libc::c_int);
        if !(opcode != 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        if opcode == 1 as libc::c_int as libc::c_ulong
            || opcode == 2 as libc::c_int as libc::c_ulong
        {
            let mut len: uint64_t = 0;
            if rdbLoadLenByRef(rdb, 0 as *mut libc::c_int, &mut len)
                == -(1 as libc::c_int)
            {
                rdbReportError(
                    1 as libc::c_int,
                    1540 as libc::c_int,
                    b"Error reading integer from module %s value\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    modulename,
                );
            }
        } else if opcode == 5 as libc::c_int as libc::c_ulong {
            let mut o: *mut robj = rdbGenericLoadStringObject(
                rdb,
                0 as libc::c_int,
                0 as *mut size_t,
            ) as *mut robj;
            if o.is_null() {
                rdbReportError(
                    1 as libc::c_int,
                    1546 as libc::c_int,
                    b"Error reading string from module %s value\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    modulename,
                );
            }
            decrRefCount(o);
        } else if opcode == 3 as libc::c_int as libc::c_ulong {
            let mut val: libc::c_float = 0.;
            if rdbLoadBinaryFloatValue(rdb, &mut val) == -(1 as libc::c_int) {
                rdbReportError(
                    1 as libc::c_int,
                    1553 as libc::c_int,
                    b"Error reading float from module %s value\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    modulename,
                );
            }
        } else if opcode == 4 as libc::c_int as libc::c_ulong {
            let mut val_0: libc::c_double = 0.;
            if rdbLoadBinaryDoubleValue(rdb, &mut val_0) == -(1 as libc::c_int) {
                rdbReportError(
                    1 as libc::c_int,
                    1559 as libc::c_int,
                    b"Error reading double from module %s value\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                    modulename,
                );
            }
        }
    }
    return createStringObject(
        b"module-dummy-value\0" as *const u8 as *const libc::c_char,
        18 as libc::c_int as size_t,
    );
}
unsafe extern "C" fn _ziplistPairsEntryConvertAndValidate(
    mut p: *mut libc::c_uchar,
    mut head_count: libc::c_uint,
    mut userdata: *mut libc::c_void,
) -> libc::c_int {
    let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut slen: libc::c_uint = 0;
    let mut vll: libc::c_longlong = 0;
    let mut data: *mut C2RustUnnamed_16 = userdata as *mut C2RustUnnamed_16;
    if ((*data).fields).is_null() {
        (*data).fields = dictCreate(&mut hashDictType);
        dictExpand(
            (*data).fields,
            head_count.wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
    }
    if ziplistGet(p, &mut str, &mut slen, &mut vll) == 0 {
        return 0 as libc::c_int;
    }
    if (*data).count & 1 as libc::c_int as libc::c_long
        == 0 as libc::c_int as libc::c_long
    {
        let mut field: sds = if !str.is_null() {
            sdsnewlen(str as *const libc::c_void, slen as size_t)
        } else {
            sdsfromlonglong(vll)
        };
        if dictAdd((*data).fields, field as *mut libc::c_void, 0 as *mut libc::c_void)
            != 0 as libc::c_int
        {
            sdsfree(field);
            return 0 as libc::c_int;
        }
    }
    if !str.is_null() {
        *(*data).lp = lpAppend(*(*data).lp, str, slen);
    } else {
        *(*data).lp = lpAppendInteger(*(*data).lp, vll);
    }
    (*data).count += 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ziplistPairsConvertAndValidateIntegrity(
    mut zl: *mut libc::c_uchar,
    mut size: size_t,
    mut lp: *mut *mut libc::c_uchar,
) -> libc::c_int {
    let mut data: C2RustUnnamed_15 = {
        let mut init = C2RustUnnamed_15 {
            count: 0 as libc::c_int as libc::c_long,
            fields: 0 as *mut dict,
            lp: lp,
        };
        init
    };
    let mut ret: libc::c_int = ziplistValidateIntegrity(
        zl,
        size,
        1 as libc::c_int,
        Some(
            _ziplistPairsEntryConvertAndValidate
                as unsafe extern "C" fn(
                    *mut libc::c_uchar,
                    libc::c_uint,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut data as *mut C2RustUnnamed_15 as *mut libc::c_void,
    );
    if data.count & 1 as libc::c_int as libc::c_long != 0 {
        ret = 0 as libc::c_int;
    }
    if !(data.fields).is_null() {
        dictRelease(data.fields);
    }
    return ret;
}
unsafe extern "C" fn _ziplistEntryConvertAndValidate(
    mut p: *mut libc::c_uchar,
    mut head_count: libc::c_uint,
    mut userdata: *mut libc::c_void,
) -> libc::c_int {
    let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut slen: libc::c_uint = 0;
    let mut vll: libc::c_longlong = 0;
    let mut lp: *mut *mut libc::c_uchar = userdata as *mut *mut libc::c_uchar;
    if ziplistGet(p, &mut str, &mut slen, &mut vll) == 0 {
        return 0 as libc::c_int;
    }
    if !str.is_null() {
        *lp = lpAppend(*lp, str, slen);
    } else {
        *lp = lpAppendInteger(*lp, vll);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn _listZiplistEntryConvertAndValidate(
    mut p: *mut libc::c_uchar,
    mut head_count: libc::c_uint,
    mut userdata: *mut libc::c_void,
) -> libc::c_int {
    let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut slen: libc::c_uint = 0;
    let mut vll: libc::c_longlong = 0;
    let mut longstr: [libc::c_char; 32] = [
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
    ];
    let mut ql: *mut quicklist = userdata as *mut quicklist;
    if ziplistGet(p, &mut str, &mut slen, &mut vll) == 0 {
        return 0 as libc::c_int;
    }
    if str.is_null() {
        slen = ll2string(
            longstr.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            vll,
        ) as libc::c_uint;
        str = longstr.as_mut_ptr() as *mut libc::c_uchar;
    }
    quicklistPushTail(ql, str as *mut libc::c_void, slen as size_t);
    return 1 as libc::c_int;
}
unsafe extern "C" fn _lpPairsEntryValidation(
    mut p: *mut libc::c_uchar,
    mut head_count: libc::c_uint,
    mut userdata: *mut libc::c_void,
) -> libc::c_int {
    let mut data: *mut C2RustUnnamed_14 = userdata as *mut C2RustUnnamed_14;
    if ((*data).fields).is_null() {
        (*data).fields = dictCreate(&mut hashDictType);
        dictExpand(
            (*data).fields,
            head_count.wrapping_div(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
    }
    if (*data).count & 1 as libc::c_int as libc::c_long
        == 0 as libc::c_int as libc::c_long
    {
        let mut str: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut slen: int64_t = 0;
        let mut buf: [libc::c_uchar; 21] = [0; 21];
        str = lpGet(p, &mut slen, buf.as_mut_ptr());
        let mut field: sds = sdsnewlen(str as *const libc::c_void, slen as size_t);
        if dictAdd((*data).fields, field as *mut libc::c_void, 0 as *mut libc::c_void)
            != 0 as libc::c_int
        {
            sdsfree(field);
            return 0 as libc::c_int;
        }
    }
    (*data).count += 1;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn lpPairsValidateIntegrityAndDups(
    mut lp: *mut libc::c_uchar,
    mut size: size_t,
    mut deep: libc::c_int,
) -> libc::c_int {
    if deep == 0 {
        return lpValidateIntegrity(
            lp,
            size,
            0 as libc::c_int,
            None,
            0 as *mut libc::c_void,
        );
    }
    let mut data: C2RustUnnamed_13 = {
        let mut init = C2RustUnnamed_13 {
            count: 0 as libc::c_int as libc::c_long,
            fields: 0 as *mut dict,
        };
        init
    };
    let mut ret: libc::c_int = lpValidateIntegrity(
        lp,
        size,
        1 as libc::c_int,
        Some(
            _lpPairsEntryValidation
                as unsafe extern "C" fn(
                    *mut libc::c_uchar,
                    libc::c_uint,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        &mut data as *mut C2RustUnnamed_13 as *mut libc::c_void,
    );
    if data.count & 1 as libc::c_int as libc::c_long != 0 {
        ret = 0 as libc::c_int;
    }
    if !(data.fields).is_null() {
        dictRelease(data.fields);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadObject(
    mut rdbtype: libc::c_int,
    mut rdb: *mut rio,
    mut key: sds,
    mut dbid: libc::c_int,
    mut error: *mut libc::c_int,
) -> *mut robj {
    let mut current_block: u64;
    let mut o: *mut robj = 0 as *mut robj;
    let mut ele: *mut robj = 0 as *mut robj;
    let mut dec: *mut robj = 0 as *mut robj;
    let mut len: uint64_t = 0;
    let mut i: libc::c_uint = 0;
    if !error.is_null() {
        *error = 2 as libc::c_int;
    }
    let mut deep_integrity_validation: libc::c_int = (server.sanitize_dump_payload
        == 1 as libc::c_int) as libc::c_int;
    if server.sanitize_dump_payload == 2 as libc::c_int {
        let mut skip: libc::c_int = (server.loading != 0
            || !(server.current_client).is_null()
                && (*server.current_client).flags
                    & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0)
            as libc::c_int;
        if skip == 0 && !(server.current_client).is_null()
            && !((*server.current_client).user).is_null()
        {
            skip = ((*(*server.current_client).user).flags
                & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_uint != 0)
                as libc::c_int;
        }
        deep_integrity_validation = (skip == 0) as libc::c_int;
    }
    if rdbtype == 0 as libc::c_int {
        o = rdbLoadEncodedStringObject(rdb);
        if o.is_null() {
            return 0 as *mut robj;
        }
        o = tryObjectEncoding(o);
    } else {
        if rdbtype == 1 as libc::c_int {
            len = rdbLoadLen(rdb, 0 as *mut libc::c_int);
            if len == 18446744073709551615 as libc::c_ulong {
                return 0 as *mut robj;
            }
            if len == 0 as libc::c_int as libc::c_ulong {
                current_block = 11297075595689639996;
            } else {
                o = createQuicklistObject();
                quicklistSetOptions(
                    (*o).ptr as *mut quicklist,
                    server.list_max_listpack_size,
                    server.list_compress_depth,
                );
                loop {
                    let fresh1 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh1 != 0) {
                        break;
                    }
                    ele = rdbLoadEncodedStringObject(rdb);
                    if ele.is_null() {
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    dec = getDecodedObject(ele);
                    let mut len_0: size_t = sdslen((*dec).ptr as sds);
                    quicklistPushTail((*o).ptr as *mut quicklist, (*dec).ptr, len_0);
                    decrRefCount(dec);
                    decrRefCount(ele);
                }
                current_block = 10856689599696957676;
            }
        } else if rdbtype == 2 as libc::c_int {
            len = rdbLoadLen(rdb, 0 as *mut libc::c_int);
            if len == 18446744073709551615 as libc::c_ulong {
                return 0 as *mut robj;
            }
            if len == 0 as libc::c_int as libc::c_ulong {
                current_block = 11297075595689639996;
            } else {
                let mut max_entries: size_t = server.set_max_intset_entries;
                if max_entries
                    >= ((1 as libc::c_int) << 30 as libc::c_int) as libc::c_ulong
                {
                    max_entries = ((1 as libc::c_int) << 30 as libc::c_int) as size_t;
                }
                if len > max_entries {
                    o = createSetObject();
                    if len > ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
                        && dictTryExpand((*o).ptr as *mut dict, len) != 0 as libc::c_int
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            1784 as libc::c_int,
                            b"OOM in dictTryExpand %llu\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            len as libc::c_ulonglong,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                } else {
                    o = createIntsetObject();
                }
                i = 0 as libc::c_int as libc::c_uint;
                while (i as libc::c_ulong) < len {
                    let mut llval: libc::c_longlong = 0;
                    let mut sdsele: sds = 0 as *mut libc::c_char;
                    sdsele = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if sdsele.is_null() {
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    if (*o).encoding() as libc::c_int == 6 as libc::c_int {
                        if isSdsRepresentableAsLongLong(sdsele, &mut llval)
                            == 0 as libc::c_int
                        {
                            let mut success: uint8_t = 0;
                            (*o)
                                .ptr = intsetAdd(
                                (*o).ptr as *mut intset,
                                llval as int64_t,
                                &mut success,
                            ) as *mut libc::c_void;
                            if success == 0 {
                                rdbReportError(
                                    1 as libc::c_int,
                                    1808 as libc::c_int,
                                    b"Duplicate set members detected\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                sdsfree(sdsele);
                                return 0 as *mut robj;
                            }
                        } else {
                            setTypeConvert(o, 2 as libc::c_int);
                            if dictTryExpand((*o).ptr as *mut dict, len)
                                != 0 as libc::c_int
                            {
                                rdbReportError(
                                    1 as libc::c_int,
                                    1816 as libc::c_int,
                                    b"OOM in dictTryExpand %llu\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                    len as libc::c_ulonglong,
                                );
                                sdsfree(sdsele);
                                decrRefCount(o);
                                return 0 as *mut robj;
                            }
                        }
                    }
                    if (*o).encoding() as libc::c_int == 2 as libc::c_int {
                        if dictAdd(
                            (*o).ptr as *mut dict,
                            sdsele as *mut libc::c_void,
                            0 as *mut libc::c_void,
                        ) != 0 as libc::c_int
                        {
                            rdbReportError(
                                1 as libc::c_int,
                                1828 as libc::c_int,
                                b"Duplicate set members detected\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            sdsfree(sdsele);
                            return 0 as *mut robj;
                        }
                    } else {
                        sdsfree(sdsele);
                    }
                    i = i.wrapping_add(1);
                }
                current_block = 10856689599696957676;
            }
        } else if rdbtype == 5 as libc::c_int || rdbtype == 3 as libc::c_int {
            let mut zsetlen: uint64_t = 0;
            let mut maxelelen: size_t = 0 as libc::c_int as size_t;
            let mut totelelen: size_t = 0 as libc::c_int as size_t;
            let mut zs: *mut zset = 0 as *mut zset;
            zsetlen = rdbLoadLen(rdb, 0 as *mut libc::c_int);
            if zsetlen == 18446744073709551615 as libc::c_ulong {
                return 0 as *mut robj;
            }
            if zsetlen == 0 as libc::c_int as libc::c_ulong {
                current_block = 11297075595689639996;
            } else {
                o = createZsetObject();
                zs = (*o).ptr as *mut zset;
                if zsetlen > ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
                    && dictTryExpand((*zs).dict, zsetlen) != 0 as libc::c_int
                {
                    rdbReportError(
                        1 as libc::c_int,
                        1850 as libc::c_int,
                        b"OOM in dictTryExpand %llu\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        zsetlen as libc::c_ulonglong,
                    );
                    decrRefCount(o);
                    return 0 as *mut robj;
                }
                loop {
                    let fresh2 = zsetlen;
                    zsetlen = zsetlen.wrapping_sub(1);
                    if !(fresh2 != 0) {
                        break;
                    }
                    let mut sdsele_0: sds = 0 as *mut libc::c_char;
                    let mut score: libc::c_double = 0.;
                    let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
                    sdsele_0 = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if sdsele_0.is_null() {
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    if rdbtype == 5 as libc::c_int {
                        if rdbLoadBinaryDoubleValue(rdb, &mut score)
                            == -(1 as libc::c_int)
                        {
                            decrRefCount(o);
                            sdsfree(sdsele_0);
                            return 0 as *mut robj;
                        }
                    } else if rdbLoadDoubleValue(rdb, &mut score) == -(1 as libc::c_int)
                    {
                        decrRefCount(o);
                        sdsfree(sdsele_0);
                        return 0 as *mut robj;
                    }
                    if score.is_nan() as i32 != 0 {
                        rdbReportError(
                            1 as libc::c_int,
                            1881 as libc::c_int,
                            b"Zset with NAN score detected\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        sdsfree(sdsele_0);
                        return 0 as *mut robj;
                    }
                    if sdslen(sdsele_0) > maxelelen {
                        maxelelen = sdslen(sdsele_0);
                    }
                    totelelen = (totelelen as libc::c_ulong)
                        .wrapping_add(sdslen(sdsele_0)) as size_t as size_t;
                    znode = zslInsert((*zs).zsl, score, sdsele_0);
                    if dictAdd(
                        (*zs).dict,
                        sdsele_0 as *mut libc::c_void,
                        &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
                    ) != 0 as libc::c_int
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            1893 as libc::c_int,
                            b"Duplicate zset fields detected\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                }
                if zsetLength(o) <= server.zset_max_listpack_entries
                    && maxelelen <= server.zset_max_listpack_value
                    && lpSafeToAdd(0 as *mut libc::c_uchar, totelelen) != 0
                {
                    zsetConvert(o, 11 as libc::c_int);
                }
                current_block = 10856689599696957676;
            }
        } else if rdbtype == 4 as libc::c_int {
            let mut len_1: uint64_t = 0;
            let mut ret: libc::c_int = 0;
            let mut field: sds = 0 as *mut libc::c_char;
            let mut value: sds = 0 as *mut libc::c_char;
            let mut dupSearchDict: *mut dict = 0 as *mut dict;
            len_1 = rdbLoadLen(rdb, 0 as *mut libc::c_int);
            if len_1 == 18446744073709551615 as libc::c_ulong {
                return 0 as *mut robj;
            }
            if len_1 == 0 as libc::c_int as libc::c_ulong {
                current_block = 11297075595689639996;
            } else {
                o = createHashObject();
                if len_1 > server.hash_max_listpack_entries {
                    hashTypeConvert(o, 2 as libc::c_int);
                } else if deep_integrity_validation != 0 {
                    dupSearchDict = dictCreate(&mut hashDictType);
                }
                while (*o).encoding() as libc::c_int == 11 as libc::c_int
                    && len_1 > 0 as libc::c_int as libc::c_ulong
                {
                    len_1 = len_1.wrapping_sub(1);
                    field = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if field.is_null() {
                        decrRefCount(o);
                        if !dupSearchDict.is_null() {
                            dictRelease(dupSearchDict);
                        }
                        return 0 as *mut robj;
                    }
                    value = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if value.is_null() {
                        sdsfree(field);
                        decrRefCount(o);
                        if !dupSearchDict.is_null() {
                            dictRelease(dupSearchDict);
                        }
                        return 0 as *mut robj;
                    }
                    if !dupSearchDict.is_null() {
                        let mut field_dup: sds = sdsdup(field);
                        if dictAdd(
                            dupSearchDict,
                            field_dup as *mut libc::c_void,
                            0 as *mut libc::c_void,
                        ) != 0 as libc::c_int
                        {
                            rdbReportError(
                                1 as libc::c_int,
                                1950 as libc::c_int,
                                b"Hash with dup elements\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            dictRelease(dupSearchDict);
                            decrRefCount(o);
                            sdsfree(field_dup);
                            sdsfree(field);
                            sdsfree(value);
                            return 0 as *mut robj;
                        }
                    }
                    if sdslen(field) > server.hash_max_listpack_value
                        || sdslen(value) > server.hash_max_listpack_value
                        || lpSafeToAdd(
                            (*o).ptr as *mut libc::c_uchar,
                            (sdslen(field)).wrapping_add(sdslen(value)),
                        ) == 0
                    {
                        hashTypeConvert(o, 2 as libc::c_int);
                        ret = dictAdd(
                            (*o).ptr as *mut dict,
                            field as *mut libc::c_void,
                            value as *mut libc::c_void,
                        );
                        if ret == 1 as libc::c_int {
                            rdbReportError(
                                1 as libc::c_int,
                                1968 as libc::c_int,
                                b"Duplicate hash fields detected\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            if !dupSearchDict.is_null() {
                                dictRelease(dupSearchDict);
                            }
                            sdsfree(value);
                            sdsfree(field);
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        break;
                    } else {
                        (*o)
                            .ptr = lpAppend(
                            (*o).ptr as *mut libc::c_uchar,
                            field as *mut libc::c_uchar,
                            sdslen(field) as uint32_t,
                        ) as *mut libc::c_void;
                        (*o)
                            .ptr = lpAppend(
                            (*o).ptr as *mut libc::c_uchar,
                            value as *mut libc::c_uchar,
                            sdslen(value) as uint32_t,
                        ) as *mut libc::c_void;
                        sdsfree(field);
                        sdsfree(value);
                    }
                }
                if !dupSearchDict.is_null() {
                    dictRelease(dupSearchDict);
                    dupSearchDict = 0 as *mut dict;
                }
                if (*o).encoding() as libc::c_int == 2 as libc::c_int
                    && len_1 > ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong
                {
                    if dictTryExpand((*o).ptr as *mut dict, len_1) != 0 as libc::c_int {
                        rdbReportError(
                            1 as libc::c_int,
                            1995 as libc::c_int,
                            b"OOM in dictTryExpand %llu\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            len_1 as libc::c_ulonglong,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                }
                while (*o).encoding() as libc::c_int == 2 as libc::c_int
                    && len_1 > 0 as libc::c_int as libc::c_ulong
                {
                    len_1 = len_1.wrapping_sub(1);
                    field = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if field.is_null() {
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    value = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if value.is_null() {
                        sdsfree(field);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    ret = dictAdd(
                        (*o).ptr as *mut dict,
                        field as *mut libc::c_void,
                        value as *mut libc::c_void,
                    );
                    if ret == 1 as libc::c_int {
                        rdbReportError(
                            1 as libc::c_int,
                            2018 as libc::c_int,
                            b"Duplicate hash fields detected\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(value);
                        sdsfree(field);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                }
                if len_1 == 0 as libc::c_int as libc::c_ulong {} else {
                    _serverAssert(
                        b"len == 0\0" as *const u8 as *const libc::c_char,
                        b"rdb.c\0" as *const u8 as *const libc::c_char,
                        2027 as libc::c_int,
                    );
                    unreachable!();
                };
                current_block = 10856689599696957676;
            }
        } else if rdbtype == 14 as libc::c_int || rdbtype == 18 as libc::c_int {
            len = rdbLoadLen(rdb, 0 as *mut libc::c_int);
            if len == 18446744073709551615 as libc::c_ulong {
                return 0 as *mut robj;
            }
            if len == 0 as libc::c_int as libc::c_ulong {
                current_block = 11297075595689639996;
            } else {
                o = createQuicklistObject();
                quicklistSetOptions(
                    (*o).ptr as *mut quicklist,
                    server.list_max_listpack_size,
                    server.list_compress_depth,
                );
                let mut container: uint64_t = 2 as libc::c_int as uint64_t;
                loop {
                    let fresh3 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh3 != 0) {
                        break;
                    }
                    let mut lp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut encoded_len: size_t = 0;
                    if rdbtype == 18 as libc::c_int {
                        container = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                        if container == 18446744073709551615 as libc::c_ulong {
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        if container != 2 as libc::c_int as libc::c_ulong
                            && container != 1 as libc::c_int as libc::c_ulong
                        {
                            rdbReportError(
                                1 as libc::c_int,
                                2047 as libc::c_int,
                                b"Quicklist integrity check failed.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                    }
                    let mut data: *mut libc::c_uchar = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 1 as libc::c_int,
                        &mut encoded_len,
                    ) as *mut libc::c_uchar;
                    if data.is_null() || encoded_len == 0 as libc::c_int as libc::c_ulong
                    {
                        zfree(data as *mut libc::c_void);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    if container == 1 as libc::c_int as libc::c_ulong {
                        quicklistAppendPlainNode(
                            (*o).ptr as *mut quicklist,
                            data,
                            encoded_len,
                        );
                    } else {
                        if rdbtype == 18 as libc::c_int {
                            lp = data;
                            if deep_integrity_validation != 0 {
                                server.stat_dump_payload_sanitizations += 1;
                            }
                            if lpValidateIntegrity(
                                lp,
                                encoded_len,
                                deep_integrity_validation,
                                None,
                                0 as *mut libc::c_void,
                            ) == 0
                            {
                                rdbReportError(
                                    1 as libc::c_int,
                                    2070 as libc::c_int,
                                    b"Listpack integrity check failed.\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                zfree(lp as *mut libc::c_void);
                                return 0 as *mut robj;
                            }
                        } else {
                            lp = lpNew(encoded_len);
                            if ziplistValidateIntegrity(
                                data,
                                encoded_len,
                                1 as libc::c_int,
                                Some(
                                    _ziplistEntryConvertAndValidate
                                        as unsafe extern "C" fn(
                                            *mut libc::c_uchar,
                                            libc::c_uint,
                                            *mut libc::c_void,
                                        ) -> libc::c_int,
                                ),
                                &mut lp as *mut *mut libc::c_uchar as *mut libc::c_void,
                            ) == 0
                            {
                                rdbReportError(
                                    1 as libc::c_int,
                                    2080 as libc::c_int,
                                    b"Ziplist integrity check failed.\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                zfree(data as *mut libc::c_void);
                                zfree(lp as *mut libc::c_void);
                                return 0 as *mut robj;
                            }
                            zfree(data as *mut libc::c_void);
                            lp = lpShrinkToFit(lp);
                        }
                        if lpLength(lp) == 0 as libc::c_int as libc::c_ulong {
                            zfree(lp as *mut libc::c_void);
                        } else {
                            quicklistAppendListpack((*o).ptr as *mut quicklist, lp);
                        }
                    }
                }
                if quicklistCount((*o).ptr as *const quicklist)
                    == 0 as libc::c_int as libc::c_ulong
                {
                    decrRefCount(o);
                    current_block = 11297075595689639996;
                } else {
                    current_block = 10856689599696957676;
                }
            }
        } else if rdbtype == 9 as libc::c_int || rdbtype == 10 as libc::c_int
            || rdbtype == 11 as libc::c_int || rdbtype == 12 as libc::c_int
            || rdbtype == 17 as libc::c_int || rdbtype == 13 as libc::c_int
            || rdbtype == 16 as libc::c_int
        {
            let mut encoded_len_0: size_t = 0;
            let mut encoded: *mut libc::c_uchar = rdbGenericLoadStringObject(
                rdb,
                (1 as libc::c_int) << 1 as libc::c_int,
                &mut encoded_len_0,
            ) as *mut libc::c_uchar;
            if encoded.is_null() {
                return 0 as *mut robj;
            }
            o = createObject(0 as libc::c_int, encoded as *mut libc::c_void);
            match rdbtype {
                9 => {
                    if zipmapValidateIntegrity(encoded, encoded_len_0, 1 as libc::c_int)
                        == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2129 as libc::c_int,
                            b"Zipmap integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    let mut lp_0: *mut libc::c_uchar = lpNew(0 as libc::c_int as size_t);
                    let mut zi: *mut libc::c_uchar = zipmapRewind(
                        (*o).ptr as *mut libc::c_uchar,
                    );
                    let mut fstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
                    let mut flen: libc::c_uint = 0;
                    let mut vlen: libc::c_uint = 0;
                    let mut maxlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
                    let mut dupSearchDict_0: *mut dict = dictCreate(&mut hashDictType);
                    loop {
                        zi = zipmapNext(zi, &mut fstr, &mut flen, &mut vstr, &mut vlen);
                        if zi.is_null() {
                            break;
                        }
                        if flen > maxlen {
                            maxlen = flen;
                        }
                        if vlen > maxlen {
                            maxlen = vlen;
                        }
                        let mut field_0: sds = sdstrynewlen(
                            fstr as *const libc::c_void,
                            flen as size_t,
                        );
                        if field_0.is_null()
                            || dictAdd(
                                dupSearchDict_0,
                                field_0 as *mut libc::c_void,
                                0 as *mut libc::c_void,
                            ) != 0 as libc::c_int
                            || lpSafeToAdd(
                                lp_0,
                                (flen as size_t).wrapping_add(vlen as libc::c_ulong),
                            ) == 0
                        {
                            rdbReportError(
                                1 as libc::c_int,
                                2153 as libc::c_int,
                                b"Hash zipmap with dup elements, or big length (%u)\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                                flen,
                            );
                            dictRelease(dupSearchDict_0);
                            sdsfree(field_0);
                            zfree(encoded as *mut libc::c_void);
                            (*o).ptr = 0 as *mut libc::c_void;
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        lp_0 = lpAppend(lp_0, fstr, flen);
                        lp_0 = lpAppend(lp_0, vstr, vlen);
                    }
                    dictRelease(dupSearchDict_0);
                    zfree((*o).ptr);
                    (*o).ptr = lp_0 as *mut libc::c_void;
                    (*o).set_type_0(4 as libc::c_int as libc::c_uint);
                    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
                    if hashTypeLength(o) > server.hash_max_listpack_entries
                        || maxlen as libc::c_ulong > server.hash_max_listpack_value
                    {
                        hashTypeConvert(o, 2 as libc::c_int);
                    }
                    current_block = 10856689599696957676;
                }
                10 => {
                    let mut ql: *mut quicklist = quicklistNew(
                        server.list_max_listpack_size,
                        server.list_compress_depth,
                    );
                    if ziplistValidateIntegrity(
                        encoded,
                        encoded_len_0,
                        1 as libc::c_int,
                        Some(
                            _listZiplistEntryConvertAndValidate
                                as unsafe extern "C" fn(
                                    *mut libc::c_uchar,
                                    libc::c_uint,
                                    *mut libc::c_void,
                                ) -> libc::c_int,
                        ),
                        ql as *mut libc::c_void,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2187 as libc::c_int,
                            b"List ziplist integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        quicklistRelease(ql);
                        return 0 as *mut robj;
                    }
                    if (*ql).len == 0 as libc::c_int as libc::c_ulong {
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        quicklistRelease(ql);
                        current_block = 11297075595689639996;
                    } else {
                        zfree(encoded as *mut libc::c_void);
                        (*o).set_type_0(1 as libc::c_int as libc::c_uint);
                        (*o).ptr = ql as *mut libc::c_void;
                        (*o).set_encoding(9 as libc::c_int as libc::c_uint);
                        current_block = 10856689599696957676;
                    }
                }
                11 => {
                    if deep_integrity_validation != 0 {
                        server.stat_dump_payload_sanitizations += 1;
                    }
                    if intsetValidateIntegrity(
                        encoded,
                        encoded_len_0,
                        deep_integrity_validation,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2212 as libc::c_int,
                            b"Intset integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    (*o).set_type_0(2 as libc::c_int as libc::c_uint);
                    (*o).set_encoding(6 as libc::c_int as libc::c_uint);
                    if intsetLen((*o).ptr as *const intset) as libc::c_ulong
                        > server.set_max_intset_entries
                    {
                        setTypeConvert(o, 2 as libc::c_int);
                    }
                    current_block = 10856689599696957676;
                }
                12 => {
                    let mut lp_1: *mut libc::c_uchar = lpNew(encoded_len_0);
                    if ziplistPairsConvertAndValidateIntegrity(
                        encoded,
                        encoded_len_0,
                        &mut lp_1,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2227 as libc::c_int,
                            b"Zset ziplist integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(lp_1 as *mut libc::c_void);
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    zfree((*o).ptr);
                    (*o).set_type_0(3 as libc::c_int as libc::c_uint);
                    (*o).ptr = lp_1 as *mut libc::c_void;
                    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
                    if zsetLength(o) == 0 as libc::c_int as libc::c_ulong {
                        decrRefCount(o);
                        current_block = 11297075595689639996;
                    } else {
                        if zsetLength(o) > server.zset_max_listpack_entries {
                            zsetConvert(o, 7 as libc::c_int);
                        } else {
                            (*o)
                                .ptr = lpShrinkToFit((*o).ptr as *mut libc::c_uchar)
                                as *mut libc::c_void;
                        }
                        current_block = 10856689599696957676;
                    }
                }
                17 => {
                    if deep_integrity_validation != 0 {
                        server.stat_dump_payload_sanitizations += 1;
                    }
                    if lpPairsValidateIntegrityAndDups(
                        encoded,
                        encoded_len_0,
                        deep_integrity_validation,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2253 as libc::c_int,
                            b"Zset listpack integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    (*o).set_type_0(3 as libc::c_int as libc::c_uint);
                    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
                    if zsetLength(o) == 0 as libc::c_int as libc::c_ulong {
                        decrRefCount(o);
                        current_block = 11297075595689639996;
                    } else {
                        if zsetLength(o) > server.zset_max_listpack_entries {
                            zsetConvert(o, 7 as libc::c_int);
                        }
                        current_block = 10856689599696957676;
                    }
                }
                13 => {
                    let mut lp_2: *mut libc::c_uchar = lpNew(encoded_len_0);
                    if ziplistPairsConvertAndValidateIntegrity(
                        encoded,
                        encoded_len_0,
                        &mut lp_2,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2273 as libc::c_int,
                            b"Hash ziplist integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(lp_2 as *mut libc::c_void);
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    zfree((*o).ptr);
                    (*o).ptr = lp_2 as *mut libc::c_void;
                    (*o).set_type_0(4 as libc::c_int as libc::c_uint);
                    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
                    if hashTypeLength(o) == 0 as libc::c_int as libc::c_ulong {
                        decrRefCount(o);
                        current_block = 11297075595689639996;
                    } else {
                        if hashTypeLength(o) > server.hash_max_listpack_entries {
                            hashTypeConvert(o, 2 as libc::c_int);
                        } else {
                            (*o)
                                .ptr = lpShrinkToFit((*o).ptr as *mut libc::c_uchar)
                                as *mut libc::c_void;
                        }
                        current_block = 10856689599696957676;
                    }
                }
                16 => {
                    if deep_integrity_validation != 0 {
                        server.stat_dump_payload_sanitizations += 1;
                    }
                    if lpPairsValidateIntegrityAndDups(
                        encoded,
                        encoded_len_0,
                        deep_integrity_validation,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2299 as libc::c_int,
                            b"Hash listpack integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        zfree(encoded as *mut libc::c_void);
                        (*o).ptr = 0 as *mut libc::c_void;
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    (*o).set_type_0(4 as libc::c_int as libc::c_uint);
                    (*o).set_encoding(11 as libc::c_int as libc::c_uint);
                    if hashTypeLength(o) == 0 as libc::c_int as libc::c_ulong {
                        decrRefCount(o);
                        current_block = 11297075595689639996;
                    } else {
                        if hashTypeLength(o) > server.hash_max_listpack_entries {
                            hashTypeConvert(o, 2 as libc::c_int);
                        }
                        current_block = 10856689599696957676;
                    }
                }
                _ => {
                    rdbReportError(
                        1 as libc::c_int,
                        2317 as libc::c_int,
                        b"Unknown RDB encoding type %d\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                        rdbtype,
                    );
                    current_block = 10856689599696957676;
                }
            }
        } else {
            if rdbtype == 15 as libc::c_int || rdbtype == 19 as libc::c_int {
                o = createStreamObject();
                let mut s: *mut stream = (*o).ptr as *mut stream;
                let mut listpacks: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if listpacks == 18446744073709551615 as libc::c_ulong {
                    rdbReportError(
                        0 as libc::c_int,
                        2325 as libc::c_int,
                        b"Stream listpacks len loading failed.\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    decrRefCount(o);
                    return 0 as *mut robj;
                }
                loop {
                    let fresh4 = listpacks;
                    listpacks = listpacks.wrapping_sub(1);
                    if !(fresh4 != 0) {
                        break;
                    }
                    let mut nodekey: sds = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if nodekey.is_null() {
                        rdbReportError(
                            0 as libc::c_int,
                            2336 as libc::c_int,
                            b"Stream master ID loading failed: invalid encoding or I/O error.\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    if sdslen(nodekey)
                        != core::mem::size_of::<streamID>() as libc::c_ulong
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2342 as libc::c_int,
                            b"Stream node key entry is not the size of a stream ID\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(nodekey);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    let mut lp_size: size_t = 0;
                    let mut lp_3: *mut libc::c_uchar = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 1 as libc::c_int,
                        &mut lp_size,
                    ) as *mut libc::c_uchar;
                    if lp_3.is_null() {
                        rdbReportError(
                            0 as libc::c_int,
                            2353 as libc::c_int,
                            b"Stream listpacks loading failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(nodekey);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    if deep_integrity_validation != 0 {
                        server.stat_dump_payload_sanitizations += 1;
                    }
                    if streamValidateListpackIntegrity(
                        lp_3,
                        lp_size,
                        deep_integrity_validation,
                    ) == 0
                    {
                        rdbReportError(
                            1 as libc::c_int,
                            2360 as libc::c_int,
                            b"Stream listpack integrity check failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(nodekey);
                        decrRefCount(o);
                        zfree(lp_3 as *mut libc::c_void);
                        return 0 as *mut robj;
                    }
                    let mut first: *mut libc::c_uchar = lpFirst(lp_3);
                    if first.is_null() {
                        rdbReportError(
                            1 as libc::c_int,
                            2372 as libc::c_int,
                            b"Empty listpack inside stream\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(nodekey);
                        decrRefCount(o);
                        zfree(lp_3 as *mut libc::c_void);
                        return 0 as *mut robj;
                    }
                    let mut retval: libc::c_int = raxTryInsert(
                        (*s).rax,
                        nodekey as *mut libc::c_uchar,
                        core::mem::size_of::<streamID>() as libc::c_ulong,
                        lp_3 as *mut libc::c_void,
                        0 as *mut *mut libc::c_void,
                    );
                    sdsfree(nodekey);
                    if retval == 0 {
                        rdbReportError(
                            1 as libc::c_int,
                            2384 as libc::c_int,
                            b"Listpack re-added with existing key\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        zfree(lp_3 as *mut libc::c_void);
                        return 0 as *mut robj;
                    }
                }
                (*s).length = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                (*s).last_id.ms = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                (*s).last_id.seq = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if rdbtype == 19 as libc::c_int {
                    (*s).first_id.ms = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    (*s).first_id.seq = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    (*s)
                        .max_deleted_entry_id
                        .ms = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    (*s)
                        .max_deleted_entry_id
                        .seq = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    (*s).entries_added = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                } else {
                    (*s).max_deleted_entry_id.ms = 0 as libc::c_int as uint64_t;
                    (*s).max_deleted_entry_id.seq = 0 as libc::c_int as uint64_t;
                    (*s).entries_added = (*s).length;
                    streamGetEdgeID(
                        s,
                        1 as libc::c_int,
                        1 as libc::c_int,
                        &mut (*s).first_id,
                    );
                }
                if rioGetReadError(rdb) != 0 {
                    rdbReportError(
                        0 as libc::c_int,
                        2422 as libc::c_int,
                        b"Stream object metadata loading failed.\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    decrRefCount(o);
                    return 0 as *mut robj;
                }
                if (*s).length != 0 && raxSize((*s).rax) == 0 {
                    rdbReportError(
                        1 as libc::c_int,
                        2428 as libc::c_int,
                        b"Stream length inconsistent with rax entries\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    decrRefCount(o);
                    return 0 as *mut robj;
                }
                let mut cgroups_count: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if cgroups_count == 18446744073709551615 as libc::c_ulong {
                    rdbReportError(
                        0 as libc::c_int,
                        2436 as libc::c_int,
                        b"Stream cgroup count loading failed.\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    decrRefCount(o);
                    return 0 as *mut robj;
                }
                loop {
                    let fresh5 = cgroups_count;
                    cgroups_count = cgroups_count.wrapping_sub(1);
                    if !(fresh5 != 0) {
                        break;
                    }
                    let mut cg_id: streamID = streamID { ms: 0, seq: 0 };
                    let mut cgname: sds = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if cgname.is_null() {
                        rdbReportError(
                            0 as libc::c_int,
                            2448 as libc::c_int,
                            b"Error reading the consumer group name from Stream\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    cg_id.ms = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    cg_id.seq = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    if rioGetReadError(rdb) != 0 {
                        rdbReportError(
                            0 as libc::c_int,
                            2456 as libc::c_int,
                            b"Stream cgroup ID loading failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        sdsfree(cgname);
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    let mut cg_offset: uint64_t = 0;
                    if rdbtype == 19 as libc::c_int {
                        cg_offset = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                        if rioGetReadError(rdb) != 0 {
                            rdbReportError(
                                0 as libc::c_int,
                                2467 as libc::c_int,
                                b"Stream cgroup offset loading failed.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            sdsfree(cgname);
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                    } else {
                        cg_offset = streamEstimateDistanceFromFirstEverEntry(
                            s,
                            &mut cg_id,
                        ) as uint64_t;
                    }
                    let mut cgroup: *mut streamCG = streamCreateCG(
                        s,
                        cgname,
                        sdslen(cgname),
                        &mut cg_id,
                        cg_offset as libc::c_longlong,
                    );
                    if cgroup.is_null() {
                        rdbReportError(
                            1 as libc::c_int,
                            2479 as libc::c_int,
                            b"Duplicated consumer group name %s\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                            cgname,
                        );
                        decrRefCount(o);
                        sdsfree(cgname);
                        return 0 as *mut robj;
                    }
                    sdsfree(cgname);
                    let mut pel_size: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    if pel_size == 18446744073709551615 as libc::c_ulong {
                        rdbReportError(
                            0 as libc::c_int,
                            2493 as libc::c_int,
                            b"Stream PEL size loading failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    loop {
                        let fresh6 = pel_size;
                        pel_size = pel_size.wrapping_sub(1);
                        if !(fresh6 != 0) {
                            break;
                        }
                        let mut rawid: [libc::c_uchar; 16] = [0; 16];
                        if rioRead(
                            rdb,
                            rawid.as_mut_ptr() as *mut libc::c_void,
                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                as libc::c_ulong,
                        ) == 0 as libc::c_int as libc::c_ulong
                        {
                            rdbReportError(
                                0 as libc::c_int,
                                2500 as libc::c_int,
                                b"Stream PEL ID loading failed.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        let mut nack: *mut streamNACK = streamCreateNACK(
                            0 as *mut streamConsumer,
                        );
                        (*nack)
                            .delivery_time = rdbLoadMillisecondTime(
                            rdb,
                            10 as libc::c_int,
                        );
                        (*nack).delivery_count = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                        if rioGetReadError(rdb) != 0 {
                            rdbReportError(
                                0 as libc::c_int,
                                2508 as libc::c_int,
                                b"Stream PEL NACK loading failed.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            streamFreeNACK(nack);
                            return 0 as *mut robj;
                        }
                        if raxTryInsert(
                            (*cgroup).pel,
                            rawid.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_uchar; 16]>()
                                as libc::c_ulong,
                            nack as *mut libc::c_void,
                            0 as *mut *mut libc::c_void,
                        ) == 0
                        {
                            rdbReportError(
                                1 as libc::c_int,
                                2515 as libc::c_int,
                                b"Duplicated global PEL entry loading stream consumer group\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            streamFreeNACK(nack);
                            return 0 as *mut robj;
                        }
                    }
                    let mut consumers_num: uint64_t = rdbLoadLen(
                        rdb,
                        0 as *mut libc::c_int,
                    );
                    if consumers_num == 18446744073709551615 as libc::c_ulong {
                        rdbReportError(
                            0 as libc::c_int,
                            2526 as libc::c_int,
                            b"Stream consumers num loading failed.\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        decrRefCount(o);
                        return 0 as *mut robj;
                    }
                    loop {
                        let fresh7 = consumers_num;
                        consumers_num = consumers_num.wrapping_sub(1);
                        if !(fresh7 != 0) {
                            break;
                        }
                        let mut cname: sds = rdbGenericLoadStringObject(
                            rdb,
                            (1 as libc::c_int) << 2 as libc::c_int,
                            0 as *mut size_t,
                        ) as sds;
                        if cname.is_null() {
                            rdbReportError(
                                0 as libc::c_int,
                                2534 as libc::c_int,
                                b"Error reading the consumer name from Stream group.\0"
                                    as *const u8 as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        let mut consumer: *mut streamConsumer = streamCreateConsumer(
                            cgroup,
                            cname,
                            0 as *mut robj,
                            0 as libc::c_int,
                            (1 as libc::c_int) << 0 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int,
                        );
                        sdsfree(cname);
                        if consumer.is_null() {
                            rdbReportError(
                                1 as libc::c_int,
                                2542 as libc::c_int,
                                b"Duplicate stream consumer detected.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        (*consumer)
                            .seen_time = rdbLoadMillisecondTime(rdb, 10 as libc::c_int);
                        if rioGetReadError(rdb) != 0 {
                            rdbReportError(
                                0 as libc::c_int,
                                2548 as libc::c_int,
                                b"Stream short read reading seen time.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        pel_size = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                        if pel_size == 18446744073709551615 as libc::c_ulong {
                            rdbReportError(
                                0 as libc::c_int,
                                2558 as libc::c_int,
                                b"Stream consumer PEL num loading failed.\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            decrRefCount(o);
                            return 0 as *mut robj;
                        }
                        loop {
                            let fresh8 = pel_size;
                            pel_size = pel_size.wrapping_sub(1);
                            if !(fresh8 != 0) {
                                break;
                            }
                            let mut rawid_0: [libc::c_uchar; 16] = [0; 16];
                            if rioRead(
                                rdb,
                                rawid_0.as_mut_ptr() as *mut libc::c_void,
                                core::mem::size_of::<[libc::c_uchar; 16]>()
                                    as libc::c_ulong,
                            ) == 0 as libc::c_int as libc::c_ulong
                            {
                                rdbReportError(
                                    0 as libc::c_int,
                                    2566 as libc::c_int,
                                    b"Stream short read reading PEL streamID.\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                return 0 as *mut robj;
                            }
                            let mut nack_0: *mut streamNACK = raxFind(
                                (*cgroup).pel,
                                rawid_0.as_mut_ptr(),
                                core::mem::size_of::<[libc::c_uchar; 16]>()
                                    as libc::c_ulong,
                            ) as *mut streamNACK;
                            if nack_0 == raxNotFound as *mut streamNACK {
                                rdbReportError(
                                    1 as libc::c_int,
                                    2573 as libc::c_int,
                                    b"Consumer entry not found in group global PEL\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                return 0 as *mut robj;
                            }
                            (*nack_0).consumer = consumer;
                            if raxTryInsert(
                                (*consumer).pel,
                                rawid_0.as_mut_ptr(),
                                core::mem::size_of::<[libc::c_uchar; 16]>()
                                    as libc::c_ulong,
                                nack_0 as *mut libc::c_void,
                                0 as *mut *mut libc::c_void,
                            ) == 0
                            {
                                rdbReportError(
                                    1 as libc::c_int,
                                    2585 as libc::c_int,
                                    b"Duplicated consumer PEL entry  loading a stream consumer group\0"
                                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                streamFreeNACK(nack_0);
                                return 0 as *mut robj;
                            }
                        }
                    }
                    if deep_integrity_validation != 0 {
                        let mut ri_cg_pel: raxIterator = raxIterator {
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
                        raxStart(&mut ri_cg_pel, (*cgroup).pel);
                        raxSeek(
                            &mut ri_cg_pel,
                            b"^\0" as *const u8 as *const libc::c_char,
                            0 as *mut libc::c_uchar,
                            0 as libc::c_int as size_t,
                        );
                        while raxNext(&mut ri_cg_pel) != 0 {
                            let mut nack_1: *mut streamNACK = ri_cg_pel.data
                                as *mut streamNACK;
                            if ((*nack_1).consumer).is_null() {
                                raxStop(&mut ri_cg_pel);
                                rdbReportError(
                                    1 as libc::c_int,
                                    2602 as libc::c_int,
                                    b"Stream CG PEL entry without consumer\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                decrRefCount(o);
                                return 0 as *mut robj;
                            }
                        }
                        raxStop(&mut ri_cg_pel);
                    }
                }
            } else if rdbtype == 6 as libc::c_int || rdbtype == 7 as libc::c_int {
                let mut moduleid: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if rioGetReadError(rdb) != 0 {
                    rdbReportError(
                        0 as libc::c_int,
                        2613 as libc::c_int,
                        b"Short read module id\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    return 0 as *mut robj;
                }
                let mut mt: *mut moduleType = moduleTypeLookupModuleByID(moduleid);
                if rdbCheckMode != 0 && rdbtype == 7 as libc::c_int {
                    let mut name: [libc::c_char; 10] = [0; 10];
                    moduleTypeNameByID(name.as_mut_ptr(), moduleid);
                    return rdbLoadCheckModuleValue(rdb, name.as_mut_ptr());
                }
                if mt.is_null() {
                    let mut name_0: [libc::c_char; 10] = [0; 10];
                    moduleTypeNameByID(name_0.as_mut_ptr(), moduleid);
                    rdbReportError(
                        1 as libc::c_int,
                        2627 as libc::c_int,
                        b"The RDB file contains module data I can't load: no matching module type '%s'\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        name_0.as_mut_ptr(),
                    );
                    return 0 as *mut robj;
                }
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
                let mut keyobj: robj = robj {
                    type_0_encoding_lru: [0; 4],
                    refcount: 0,
                    ptr: 0 as *mut libc::c_void,
                };
                keyobj.refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
                keyobj.set_type_0(0 as libc::c_int as libc::c_uint);
                keyobj.set_encoding(0 as libc::c_int as libc::c_uint);
                keyobj.ptr = key as *mut libc::c_void;
                io.rio = rdb;
                io.type_0 = mt;
                io.bytes = 0 as libc::c_int as size_t;
                io.error = 0 as libc::c_int;
                io.ver = 0 as libc::c_int;
                io.key = &mut keyobj;
                io.dbid = dbid;
                io.ctx = 0 as *mut RedisModuleCtx;
                io
                    .ver = if rdbtype == 6 as libc::c_int {
                    1 as libc::c_int
                } else {
                    2 as libc::c_int
                };
                let mut ptr: *mut libc::c_void = ((*mt).rdb_load)
                    .expect(
                        "non-null function pointer",
                    )(
                    &mut io,
                    (moduleid & 1023 as libc::c_int as libc::c_ulong) as libc::c_int,
                );
                if !(io.ctx).is_null() {
                    moduleFreeContext(io.ctx);
                    zfree(io.ctx as *mut libc::c_void);
                }
                if io.ver == 2 as libc::c_int {
                    let mut eof: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                    if eof == 18446744073709551615 as libc::c_ulong {
                        if !ptr.is_null() {
                            o = createModuleObject(mt, ptr);
                            decrRefCount(o);
                        }
                        return 0 as *mut robj;
                    }
                    if eof != 0 as libc::c_int as libc::c_ulong {
                        rdbReportError(
                            1 as libc::c_int,
                            2655 as libc::c_int,
                            b"The RDB file contains module data for the module '%s' that is not terminated by the proper module value EOF marker\0"
                                as *const u8 as *const libc::c_char as *mut libc::c_char,
                            moduleTypeModuleName(mt),
                        );
                        if !ptr.is_null() {
                            o = createModuleObject(mt, ptr);
                            decrRefCount(o);
                        }
                        return 0 as *mut robj;
                    }
                }
                if ptr.is_null() {
                    rdbReportError(
                        1 as libc::c_int,
                        2667 as libc::c_int,
                        b"The RDB file contains module data for the module type '%s', that the responsible module is not able to load. Check for modules log above for additional clues.\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                        moduleTypeModuleName(mt),
                    );
                    return 0 as *mut robj;
                }
                o = createModuleObject(mt, ptr);
            } else {
                rdbReportError(
                    0 as libc::c_int,
                    2672 as libc::c_int,
                    b"Unknown RDB encoding type %d\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    rdbtype,
                );
                return 0 as *mut robj;
            }
            current_block = 10856689599696957676;
        }
        match current_block {
            10856689599696957676 => {}
            _ => {
                if !error.is_null() {
                    *error = 1 as libc::c_int;
                }
                return 0 as *mut robj;
            }
        }
    }
    if !error.is_null() {
        *error = 0 as libc::c_int;
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn startLoading(
    mut size: size_t,
    mut rdbflags: libc::c_int,
    mut async_0: libc::c_int,
) {
    core::ptr::write_volatile(
        &mut server.loading as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    if async_0 == 1 as libc::c_int {
        core::ptr::write_volatile(
            &mut server.async_loading as *mut sig_atomic_t,
            1 as libc::c_int,
        );
    }
    server.loading_start_time = time(0 as *mut time_t);
    server.loading_loaded_bytes = 0 as libc::c_int as off_t;
    server.loading_total_bytes = size as off_t;
    server.loading_rdb_used_mem = 0 as libc::c_int as off_t;
    server.rdb_last_load_keys_expired = 0 as libc::c_int as libc::c_longlong;
    server.rdb_last_load_keys_loaded = 0 as libc::c_int as libc::c_longlong;
    blockingOperationStarts();
    let mut subevent: libc::c_int = 0;
    if rdbflags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        subevent = 1 as libc::c_int;
    } else if rdbflags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        subevent = 2 as libc::c_int;
    } else {
        subevent = 0 as libc::c_int;
    }
    moduleFireServerEvent(
        3 as libc::c_int as uint64_t,
        subevent,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn startLoadingFile(
    mut size: size_t,
    mut filename: *mut libc::c_char,
    mut rdbflags: libc::c_int,
) {
    rdbFileBeingLoaded = filename;
    startLoading(size, rdbflags, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn loadingAbsProgress(mut pos: off_t) {
    server.loading_loaded_bytes = pos;
    if server.stat_peak_memory < zmalloc_used_memory() {
        server.stat_peak_memory = zmalloc_used_memory();
    }
}
#[no_mangle]
pub unsafe extern "C" fn loadingIncrProgress(mut size: off_t) {
    server.loading_loaded_bytes += size;
    if server.stat_peak_memory < zmalloc_used_memory() {
        server.stat_peak_memory = zmalloc_used_memory();
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateLoadingFileName(mut filename: *mut libc::c_char) {
    rdbFileBeingLoaded = filename;
}
#[no_mangle]
pub unsafe extern "C" fn stopLoading(mut success: libc::c_int) {
    core::ptr::write_volatile(
        &mut server.loading as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    core::ptr::write_volatile(
        &mut server.async_loading as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    blockingOperationEnds();
    rdbFileBeingLoaded = 0 as *mut libc::c_char;
    moduleFireServerEvent(
        3 as libc::c_int as uint64_t,
        if success != 0 { 3 as libc::c_int } else { 4 as libc::c_int },
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn startSaving(mut rdbflags: libc::c_int) {
    let mut subevent: libc::c_int = 0;
    if rdbflags & (1 as libc::c_int) << 0 as libc::c_int != 0 && getpid() != server.pid {
        subevent = 1 as libc::c_int;
    } else if rdbflags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        subevent = 5 as libc::c_int;
    } else if getpid() != server.pid {
        subevent = 0 as libc::c_int;
    } else {
        subevent = 2 as libc::c_int;
    }
    moduleFireServerEvent(
        1 as libc::c_int as uint64_t,
        subevent,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn stopSaving(mut success: libc::c_int) {
    moduleFireServerEvent(
        1 as libc::c_int as uint64_t,
        if success != 0 { 3 as libc::c_int } else { 4 as libc::c_int },
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadProgressCallback(
    mut r: *mut rio,
    mut buf: *const libc::c_void,
    mut len: size_t,
) {
    if server.rdb_checksum != 0 {
        rioGenericUpdateChecksum(r, buf, len);
    }
    if server.loading_process_events_interval_bytes != 0
        && ((*r).processed_bytes)
            .wrapping_add(len)
            .wrapping_div(server.loading_process_events_interval_bytes as libc::c_ulong)
            > ((*r).processed_bytes)
                .wrapping_div(
                    server.loading_process_events_interval_bytes as libc::c_ulong,
                )
    {
        if !(server.masterhost).is_null()
            && server.repl_state == REPL_STATE_TRANSFER as libc::c_int
        {
            replicationSendNewlineToMaster();
        }
        loadingAbsProgress((*r).processed_bytes as off_t);
        processEventsWhileBlocked();
        processModuleLoadingProgressEvent(0 as libc::c_int);
    }
    if server.repl_state == REPL_STATE_TRANSFER as libc::c_int
        && rioCheckType(r) as libc::c_int == (1 as libc::c_int) << 2 as libc::c_int
    {
        let fresh9 = &mut server.stat_net_repl_input_bytes;
        let fresh10 = len as libc::c_longlong;
        core::intrinsics::atomic_xadd_relaxed(fresh9, fresh10) + fresh10;
    }
}
#[no_mangle]
pub unsafe extern "C" fn rdbFunctionLoad(
    mut rdb: *mut rio,
    mut ver: libc::c_int,
    mut lib_ctx: *mut functionsLibCtx,
    mut type_0: libc::c_int,
    mut rdbflags: libc::c_int,
    mut err: *mut sds,
) -> libc::c_int {
    let mut current_block: u64;
    let mut error: sds = 0 as sds;
    let mut final_payload: sds = 0 as sds;
    let mut res: libc::c_int = -(1 as libc::c_int);
    if type_0 == 246 as libc::c_int {
        let mut name: sds = 0 as sds;
        let mut engine_name: sds = 0 as sds;
        let mut desc: sds = 0 as sds;
        let mut blob: sds = 0 as sds;
        let mut has_desc: uint64_t = 0;
        name = rdbGenericLoadStringObject(
            rdb,
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as *mut size_t,
        ) as sds;
        if name.is_null() {
            error = sdsnew(
                b"Failed loading library name\0" as *const u8 as *const libc::c_char,
            );
        } else {
            engine_name = rdbGenericLoadStringObject(
                rdb,
                (1 as libc::c_int) << 2 as libc::c_int,
                0 as *mut size_t,
            ) as sds;
            if engine_name.is_null() {
                error = sdsnew(
                    b"Failed loading engine name\0" as *const u8 as *const libc::c_char,
                );
            } else {
                has_desc = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if has_desc == 18446744073709551615 as libc::c_ulong {
                    error = sdsnew(
                        b"Failed loading library description indicator\0" as *const u8
                            as *const libc::c_char,
                    );
                } else if has_desc != 0
                    && {
                        desc = rdbGenericLoadStringObject(
                            rdb,
                            (1 as libc::c_int) << 2 as libc::c_int,
                            0 as *mut size_t,
                        ) as sds;
                        desc.is_null()
                    }
                {
                    error = sdsnew(
                        b"Failed loading library description\0" as *const u8
                            as *const libc::c_char,
                    );
                } else {
                    blob = rdbGenericLoadStringObject(
                        rdb,
                        (1 as libc::c_int) << 2 as libc::c_int,
                        0 as *mut size_t,
                    ) as sds;
                    if blob.is_null() {
                        error = sdsnew(
                            b"Failed loading library blob\0" as *const u8
                                as *const libc::c_char,
                        );
                    } else {
                        final_payload = sdscatfmt(
                            sdsempty(),
                            b"#!%s name=%s\n%s\0" as *const u8 as *const libc::c_char,
                            engine_name,
                            name,
                            blob,
                        );
                    }
                }
            }
        }
        if !name.is_null() {
            sdsfree(name);
        }
        if !engine_name.is_null() {
            sdsfree(engine_name);
        }
        if !desc.is_null() {
            sdsfree(desc);
        }
        if !blob.is_null() {
            sdsfree(blob);
        }
        if !error.is_null() {
            current_block = 6387762410704612043;
        } else {
            current_block = 1608152415753874203;
        }
    } else if type_0 == 245 as libc::c_int {
        final_payload = rdbGenericLoadStringObject(
            rdb,
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as *mut size_t,
        ) as sds;
        if final_payload.is_null() {
            error = sdsnew(
                b"Failed loading library payload\0" as *const u8 as *const libc::c_char,
            );
            current_block = 6387762410704612043;
        } else {
            current_block = 1608152415753874203;
        }
    } else {
        _serverPanic(
            b"rdb.c\0" as *const u8 as *const libc::c_char,
            2854 as libc::c_int,
            b"Bad function type was given to rdbFunctionLoad\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
        current_block = 1608152415753874203;
    }
    match current_block {
        1608152415753874203 => {
            if !lib_ctx.is_null() {
                let mut library_name: sds = 0 as sds;
                library_name = functionsCreateWithLibraryCtx(
                    final_payload,
                    rdbflags & (1 as libc::c_int) << 2 as libc::c_int,
                    &mut error,
                    lib_ctx,
                );
                if library_name.is_null() {
                    if error.is_null() {
                        error = sdsnew(
                            b"Failed creating the library\0" as *const u8
                                as *const libc::c_char,
                        );
                    }
                    current_block = 6387762410704612043;
                } else {
                    sdsfree(library_name);
                    current_block = 9520865839495247062;
                }
            } else {
                current_block = 9520865839495247062;
            }
            match current_block {
                6387762410704612043 => {}
                _ => {
                    res = 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    if !final_payload.is_null() {
        sdsfree(final_payload);
    }
    if !error.is_null() {
        if !err.is_null() {
            *err = error;
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed creating function, %s\0" as *const u8
                        as *const libc::c_char,
                    error,
                );
            }
            sdsfree(error);
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadRio(
    mut rdb: *mut rio,
    mut rdbflags: libc::c_int,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut functions_lib_ctx: *mut functionsLibCtx = functionsLibCtxGetCurrent();
    let mut loading_ctx: rdbLoadingCtx = {
        let mut init = rdbLoadingCtx {
            dbarray: server.db,
            functions_lib_ctx: functions_lib_ctx,
        };
        init
    };
    let mut retval: libc::c_int = rdbLoadRioWithLoadingCtx(
        rdb,
        rdbflags,
        rsi,
        &mut loading_ctx,
    );
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoadRioWithLoadingCtx(
    mut rdb: *mut rio,
    mut rdbflags: libc::c_int,
    mut rsi: *mut rdbSaveInfo,
    mut rdb_loading_ctx: *mut rdbLoadingCtx,
) -> libc::c_int {
    let mut lru_idle: libc::c_longlong = 0;
    let mut lfu_freq: libc::c_longlong = 0;
    let mut expiretime: libc::c_longlong = 0;
    let mut now: libc::c_longlong = 0;
    let mut lru_clock: libc::c_longlong = 0;
    let mut current_block: u64;
    let mut dbid: uint64_t = 0 as libc::c_int as uint64_t;
    let mut type_0: libc::c_int = 0;
    let mut rdbver: libc::c_int = 0;
    let mut db: *mut redisDb = ((*rdb_loading_ctx).dbarray)
        .offset(0 as libc::c_int as isize);
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut error: libc::c_int = 0;
    let mut empty_keys_skipped: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    (*rdb)
        .update_cksum = Some(
        rdbLoadProgressCallback
            as unsafe extern "C" fn(*mut rio, *const libc::c_void, size_t) -> (),
    );
    (*rdb).max_processing_chunk = server.loading_process_events_interval_bytes as size_t;
    if !(rioRead(rdb, buf.as_mut_ptr() as *mut libc::c_void, 9 as libc::c_int as size_t)
        == 0 as libc::c_int as libc::c_ulong)
    {
        buf[9 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        if memcmp(
            buf.as_mut_ptr() as *const libc::c_void,
            b"REDIS\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Wrong signature trying to load DB from file\0" as *const u8
                        as *const libc::c_char,
                );
            }
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        rdbver = atoi(buf.as_mut_ptr().offset(5 as libc::c_int as isize));
        if rdbver < 1 as libc::c_int || rdbver > 10 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't handle RDB format version %d\0" as *const u8
                        as *const libc::c_char,
                    rdbver,
                );
            }
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        lru_idle = -(1 as libc::c_int) as libc::c_longlong;
        lfu_freq = -(1 as libc::c_int) as libc::c_longlong;
        expiretime = -(1 as libc::c_int) as libc::c_longlong;
        now = mstime();
        lru_clock = LRU_CLOCK() as libc::c_longlong;
        loop {
            let mut key: sds = 0 as *mut libc::c_char;
            let mut val: *mut robj = 0 as *mut robj;
            type_0 = rdbLoadType(rdb);
            if type_0 == -(1 as libc::c_int) {
                current_block = 17392992105600767133;
                break;
            }
            if type_0 == 253 as libc::c_int {
                expiretime = rdbLoadTime(rdb) as libc::c_longlong;
                expiretime *= 1000 as libc::c_int as libc::c_longlong;
                if rioGetReadError(rdb) != 0 {
                    current_block = 17392992105600767133;
                    break;
                }
            } else if type_0 == 252 as libc::c_int {
                expiretime = rdbLoadMillisecondTime(rdb, rdbver);
                if rioGetReadError(rdb) != 0 {
                    current_block = 17392992105600767133;
                    break;
                }
            } else if type_0 == 249 as libc::c_int {
                let mut byte: uint8_t = 0;
                if rioRead(
                    rdb,
                    &mut byte as *mut uint8_t as *mut libc::c_void,
                    1 as libc::c_int as size_t,
                ) == 0 as libc::c_int as libc::c_ulong
                {
                    current_block = 17392992105600767133;
                    break;
                }
                lfu_freq = byte as libc::c_longlong;
            } else if type_0 == 248 as libc::c_int {
                let mut qword: uint64_t = 0;
                qword = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if qword == 18446744073709551615 as libc::c_ulong {
                    current_block = 17392992105600767133;
                    break;
                }
                lru_idle = qword as libc::c_longlong;
            } else if type_0 == 255 as libc::c_int {
                current_block = 162359820444412279;
                break;
            } else if type_0 == 254 as libc::c_int {
                dbid = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if dbid == 18446744073709551615 as libc::c_ulong {
                    current_block = 17392992105600767133;
                    break;
                }
                if dbid >= server.dbnum as libc::c_uint as libc::c_ulong {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"FATAL: Data file was created with a Redis server configured to handle more than %d databases. Exiting\n\0"
                                as *const u8 as *const libc::c_char,
                            server.dbnum,
                        );
                    }
                    exit(1 as libc::c_int);
                }
                db = ((*rdb_loading_ctx).dbarray).offset(dbid as isize);
            } else if type_0 == 251 as libc::c_int {
                let mut db_size: uint64_t = 0;
                let mut expires_size: uint64_t = 0;
                db_size = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if db_size == 18446744073709551615 as libc::c_ulong {
                    current_block = 17392992105600767133;
                    break;
                }
                expires_size = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                if expires_size == 18446744073709551615 as libc::c_ulong {
                    current_block = 17392992105600767133;
                    break;
                }
                dictExpand((*db).dict, db_size);
                dictExpand((*db).expires, expires_size);
            } else if type_0 == 250 as libc::c_int {
                let mut auxkey: *mut robj = 0 as *mut robj;
                let mut auxval: *mut robj = 0 as *mut robj;
                auxkey = rdbLoadStringObject(rdb);
                if auxkey.is_null() {
                    current_block = 17392992105600767133;
                    break;
                }
                auxval = rdbLoadStringObject(rdb);
                if auxval.is_null() {
                    decrRefCount(auxkey);
                    current_block = 17392992105600767133;
                    break;
                } else {
                    if *((*auxkey).ptr as *mut libc::c_char)
                        .offset(0 as libc::c_int as isize) as libc::c_int == '%' as i32
                    {
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"RDB '%s': %s\0" as *const u8 as *const libc::c_char,
                                (*auxkey).ptr as *mut libc::c_char,
                                (*auxval).ptr as *mut libc::c_char,
                            );
                        }
                    } else if strcasecmp(
                        (*auxkey).ptr as *const libc::c_char,
                        b"repl-stream-db\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if !rsi.is_null() {
                            (*rsi)
                                .repl_stream_db = atoi(
                                (*auxval).ptr as *const libc::c_char,
                            );
                        }
                    } else if strcasecmp(
                        (*auxkey).ptr as *const libc::c_char,
                        b"repl-id\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if !rsi.is_null()
                            && sdslen((*auxval).ptr as sds)
                                == 40 as libc::c_int as libc::c_ulong
                        {
                            memcpy(
                                ((*rsi).repl_id).as_mut_ptr() as *mut libc::c_void,
                                (*auxval).ptr,
                                (40 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                            );
                            (*rsi).repl_id_is_set = 1 as libc::c_int;
                        }
                    } else if strcasecmp(
                        (*auxkey).ptr as *const libc::c_char,
                        b"repl-offset\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        if !rsi.is_null() {
                            (*rsi)
                                .repl_offset = strtoll(
                                (*auxval).ptr as *const libc::c_char,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                        }
                    } else if !(strcasecmp(
                        (*auxkey).ptr as *const libc::c_char,
                        b"lua\0" as *const u8 as *const libc::c_char,
                    ) == 0)
                    {
                        if strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"redis-ver\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    2 as libc::c_int,
                                    b"Loading RDB produced by version %s\0" as *const u8
                                        as *const libc::c_char,
                                    (*auxval).ptr as *mut libc::c_char,
                                );
                            }
                        } else if strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"ctime\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            let mut age: time_t = time(0 as *mut time_t)
                                - strtol(
                                    (*auxval).ptr as *const libc::c_char,
                                    0 as *mut *mut libc::c_char,
                                    10 as libc::c_int,
                                );
                            if age < 0 as libc::c_int as libc::c_long {
                                age = 0 as libc::c_int as time_t;
                            }
                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    2 as libc::c_int,
                                    b"RDB age %ld seconds\0" as *const u8
                                        as *const libc::c_char,
                                    age as libc::c_ulong,
                                );
                            }
                        } else if strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"used-mem\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            let mut usedmem: libc::c_longlong = strtoll(
                                (*auxval).ptr as *const libc::c_char,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                            if !((2 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    2 as libc::c_int,
                                    b"RDB memory usage when created %.2f Mb\0" as *const u8
                                        as *const libc::c_char,
                                    usedmem as libc::c_double
                                        / (1024 as libc::c_int * 1024 as libc::c_int)
                                            as libc::c_double,
                                );
                            }
                            server.loading_rdb_used_mem = usedmem as off_t;
                        } else if strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"aof-preamble\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            let mut haspreamble: libc::c_longlong = strtoll(
                                (*auxval).ptr as *const libc::c_char,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                            if haspreamble != 0 {
                                if !((2 as libc::c_int & 0xff as libc::c_int)
                                    < server.verbosity)
                                {
                                    _serverLog(
                                        2 as libc::c_int,
                                        b"RDB has an AOF tail\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                        } else if strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"aof-base\0" as *const u8 as *const libc::c_char,
                        ) == 0
                        {
                            let mut isbase: libc::c_longlong = strtoll(
                                (*auxval).ptr as *const libc::c_char,
                                0 as *mut *mut libc::c_char,
                                10 as libc::c_int,
                            );
                            if isbase != 0 {
                                if !((2 as libc::c_int & 0xff as libc::c_int)
                                    < server.verbosity)
                                {
                                    _serverLog(
                                        2 as libc::c_int,
                                        b"RDB is base AOF\0" as *const u8 as *const libc::c_char,
                                    );
                                }
                            }
                        } else if !(strcasecmp(
                            (*auxkey).ptr as *const libc::c_char,
                            b"redis-bits\0" as *const u8 as *const libc::c_char,
                        ) == 0)
                        {
                            if !((0 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    0 as libc::c_int,
                                    b"Unrecognized RDB AUX field: '%s'\0" as *const u8
                                        as *const libc::c_char,
                                    (*auxkey).ptr as *mut libc::c_char,
                                );
                            }
                        }
                    }
                    decrRefCount(auxkey);
                    decrRefCount(auxval);
                }
            } else if type_0 == 247 as libc::c_int {
                let mut moduleid: uint64_t = rdbLoadLen(rdb, 0 as *mut libc::c_int);
                let mut when_opcode: libc::c_int = rdbLoadLen(rdb, 0 as *mut libc::c_int)
                    as libc::c_int;
                let mut when: libc::c_int = rdbLoadLen(rdb, 0 as *mut libc::c_int)
                    as libc::c_int;
                if rioGetReadError(rdb) != 0 {
                    current_block = 17392992105600767133;
                    break;
                }
                if when_opcode != 2 as libc::c_int {
                    rdbReportError(
                        0 as libc::c_int,
                        3057 as libc::c_int,
                        b"bad when_opcode\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                    current_block = 17392992105600767133;
                    break;
                } else {
                    let mut mt: *mut moduleType = moduleTypeLookupModuleByID(moduleid);
                    let mut name: [libc::c_char; 10] = [0; 10];
                    moduleTypeNameByID(name.as_mut_ptr(), moduleid);
                    if rdbCheckMode == 0 && mt.is_null() {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"The RDB file contains AUX module data I can't load: no matching module '%s'\0"
                                    as *const u8 as *const libc::c_char,
                                name.as_mut_ptr(),
                            );
                        }
                        exit(1 as libc::c_int);
                    } else if rdbCheckMode == 0 && !mt.is_null() {
                        if ((*mt).aux_load).is_none() {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"The RDB file contains module AUX data, but the module '%s' doesn't seem to support it.\0"
                                        as *const u8 as *const libc::c_char,
                                    name.as_mut_ptr(),
                                );
                            }
                            exit(1 as libc::c_int);
                        }
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
                        io.rio = rdb;
                        io.type_0 = mt;
                        io.bytes = 0 as libc::c_int as size_t;
                        io.error = 0 as libc::c_int;
                        io.ver = 0 as libc::c_int;
                        io.key = 0 as *mut redisObject;
                        io.dbid = -(1 as libc::c_int);
                        io.ctx = 0 as *mut RedisModuleCtx;
                        io.ver = 2 as libc::c_int;
                        let mut rc: libc::c_int = ((*mt).aux_load)
                            .expect(
                                "non-null function pointer",
                            )(
                            &mut io,
                            (moduleid & 1023 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            when,
                        );
                        if !(io.ctx).is_null() {
                            moduleFreeContext(io.ctx);
                            zfree(io.ctx as *mut libc::c_void);
                        }
                        if rc != 0 as libc::c_int || io.error != 0 {
                            moduleTypeNameByID(name.as_mut_ptr(), moduleid);
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"The RDB file contains module AUX data for the module type '%s', that the responsible module is not able to load. Check for modules log above for additional clues.\0"
                                        as *const u8 as *const libc::c_char,
                                    name.as_mut_ptr(),
                                );
                            }
                            current_block = 17392992105600767133;
                            break;
                        } else {
                            let mut eof: uint64_t = rdbLoadLen(
                                rdb,
                                0 as *mut libc::c_int,
                            );
                            if !(eof != 0 as libc::c_int as libc::c_ulong) {
                                continue;
                            }
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"The RDB file contains module AUX data for the module '%s' that is not terminated by the proper module value EOF marker\0"
                                        as *const u8 as *const libc::c_char,
                                    name.as_mut_ptr(),
                                );
                            }
                            current_block = 17392992105600767133;
                            break;
                        }
                    } else {
                        let mut aux: *mut robj = rdbLoadCheckModuleValue(
                            rdb,
                            name.as_mut_ptr(),
                        );
                        decrRefCount(aux);
                    }
                }
            } else if type_0 == 246 as libc::c_int || type_0 == 245 as libc::c_int {
                let mut err: sds = 0 as sds;
                if !(rdbFunctionLoad(
                    rdb,
                    rdbver,
                    (*rdb_loading_ctx).functions_lib_ctx,
                    type_0,
                    rdbflags,
                    &mut err,
                ) != 0 as libc::c_int)
                {
                    continue;
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Failed loading library, %s\0" as *const u8
                            as *const libc::c_char,
                        err,
                    );
                }
                sdsfree(err);
                current_block = 17392992105600767133;
                break;
            } else {
                key = rdbGenericLoadStringObject(
                    rdb,
                    (1 as libc::c_int) << 2 as libc::c_int,
                    0 as *mut size_t,
                ) as sds;
                if key.is_null() {
                    current_block = 17392992105600767133;
                    break;
                }
                val = rdbLoadObject(type_0, rdb, key, (*db).id, &mut error);
                if val.is_null() {
                    if error == 1 as libc::c_int {
                        let fresh11 = empty_keys_skipped;
                        empty_keys_skipped = empty_keys_skipped + 1;
                        if fresh11 < 10 as libc::c_int as libc::c_longlong {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"rdbLoadObject skipping empty key: %s\0" as *const u8
                                        as *const libc::c_char,
                                    key,
                                );
                            }
                        }
                        sdsfree(key);
                    } else {
                        sdsfree(key);
                        current_block = 17392992105600767133;
                        break;
                    }
                } else if iAmMaster() != 0
                    && rdbflags & (1 as libc::c_int) << 0 as libc::c_int == 0
                    && expiretime != -(1 as libc::c_int) as libc::c_longlong
                    && expiretime < now
                {
                    if rdbflags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                        if !(server.repl_backlog).is_null()
                            && (*server.slaves).len == 0 as libc::c_int as libc::c_ulong
                        {} else {
                            _serverAssert(
                                b"server.repl_backlog != NULL && listLength(server.slaves) == 0\0"
                                    as *const u8 as *const libc::c_char,
                                b"rdb.c\0" as *const u8 as *const libc::c_char,
                                3147 as libc::c_int,
                            );
                            unreachable!();
                        };
                        let mut keyobj: robj = robj {
                            type_0_encoding_lru: [0; 4],
                            refcount: 0,
                            ptr: 0 as *mut libc::c_void,
                        };
                        keyobj.refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
                        keyobj.set_type_0(0 as libc::c_int as libc::c_uint);
                        keyobj.set_encoding(0 as libc::c_int as libc::c_uint);
                        keyobj.ptr = key as *mut libc::c_void;
                        let mut argv: [*mut robj; 2] = [0 as *mut robj; 2];
                        argv[0 as libc::c_int
                            as usize] = if server.lazyfree_lazy_expire != 0 {
                            shared.unlink
                        } else {
                            shared.del
                        };
                        argv[1 as libc::c_int as usize] = &mut keyobj;
                        replicationFeedSlaves(
                            server.slaves,
                            dbid as libc::c_int,
                            argv.as_mut_ptr(),
                            2 as libc::c_int,
                        );
                    }
                    sdsfree(key);
                    decrRefCount(val);
                    server.rdb_last_load_keys_expired += 1;
                } else {
                    let mut keyobj_0: robj = robj {
                        type_0_encoding_lru: [0; 4],
                        refcount: 0,
                        ptr: 0 as *mut libc::c_void,
                    };
                    keyobj_0.refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
                    keyobj_0.set_type_0(0 as libc::c_int as libc::c_uint);
                    keyobj_0.set_encoding(0 as libc::c_int as libc::c_uint);
                    keyobj_0.ptr = key as *mut libc::c_void;
                    let mut added: libc::c_int = dbAddRDBLoad(db, key, val);
                    server.rdb_last_load_keys_loaded += 1;
                    if added == 0 {
                        if rdbflags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                            dbSyncDelete(db, &mut keyobj_0);
                            dbAddRDBLoad(db, key, val);
                        } else {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"RDB has duplicated key '%s' in DB %d\0" as *const u8
                                        as *const libc::c_char,
                                    key,
                                    (*db).id,
                                );
                            }
                            _serverPanic(
                                b"rdb.c\0" as *const u8 as *const libc::c_char,
                                3175 as libc::c_int,
                                b"Duplicated key found in RDB file\0" as *const u8
                                    as *const libc::c_char,
                            );
                            unreachable!();
                        }
                    }
                    if expiretime != -(1 as libc::c_int) as libc::c_longlong {
                        setExpire(0 as *mut client, db, &mut keyobj_0, expiretime);
                    }
                    objectSetLRUOrLFU(
                        val,
                        lfu_freq,
                        lru_idle,
                        lru_clock,
                        1000 as libc::c_int,
                    );
                    moduleNotifyKeyspaceEvent(
                        (1 as libc::c_int) << 12 as libc::c_int,
                        b"loaded\0" as *const u8 as *const libc::c_char,
                        &mut keyobj_0,
                        (*db).id,
                    );
                }
                if server.key_load_delay != 0 {
                    debugDelay(server.key_load_delay);
                }
                expiretime = -(1 as libc::c_int) as libc::c_longlong;
                lfu_freq = -(1 as libc::c_int) as libc::c_longlong;
                lru_idle = -(1 as libc::c_int) as libc::c_longlong;
            }
        }
        match current_block {
            17392992105600767133 => {}
            _ => {
                if rdbver >= 5 as libc::c_int {
                    let mut cksum: uint64_t = 0;
                    let mut expected: uint64_t = (*rdb).cksum;
                    if rioRead(
                        rdb,
                        &mut cksum as *mut uint64_t as *mut libc::c_void,
                        8 as libc::c_int as size_t,
                    ) == 0 as libc::c_int as libc::c_ulong
                    {
                        current_block = 17392992105600767133;
                    } else {
                        if server.rdb_checksum != 0
                            && server.skip_checksum_validation == 0
                        {
                            if cksum == 0 as libc::c_int as libc::c_ulong {
                                if !((3 as libc::c_int & 0xff as libc::c_int)
                                    < server.verbosity)
                                {
                                    _serverLog(
                                        3 as libc::c_int,
                                        b"RDB file was saved with checksum disabled: no check performed.\0"
                                            as *const u8 as *const libc::c_char,
                                    );
                                }
                            } else if cksum != expected {
                                if !((3 as libc::c_int & 0xff as libc::c_int)
                                    < server.verbosity)
                                {
                                    _serverLog(
                                        3 as libc::c_int,
                                        b"Wrong RDB checksum expected: (%llx) but got (%llx). Aborting now.\0"
                                            as *const u8 as *const libc::c_char,
                                        expected as libc::c_ulonglong,
                                        cksum as libc::c_ulonglong,
                                    );
                                }
                                rdbReportError(
                                    1 as libc::c_int,
                                    3216 as libc::c_int,
                                    b"RDB CRC error\0" as *const u8 as *const libc::c_char
                                        as *mut libc::c_char,
                                );
                                return -(1 as libc::c_int);
                            }
                        }
                        current_block = 16286683003977321678;
                    }
                } else {
                    current_block = 16286683003977321678;
                }
                match current_block {
                    17392992105600767133 => {}
                    _ => {
                        if empty_keys_skipped != 0 {
                            if !((3 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    3 as libc::c_int,
                                    b"Done loading RDB, keys loaded: %lld, keys expired: %lld, empty keys skipped: %lld.\0"
                                        as *const u8 as *const libc::c_char,
                                    server.rdb_last_load_keys_loaded,
                                    server.rdb_last_load_keys_expired,
                                    empty_keys_skipped,
                                );
                            }
                        } else if !((2 as libc::c_int & 0xff as libc::c_int)
                            < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"Done loading RDB, keys loaded: %lld, keys expired: %lld.\0"
                                    as *const u8 as *const libc::c_char,
                                server.rdb_last_load_keys_loaded,
                                server.rdb_last_load_keys_expired,
                            );
                        }
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Short read or OOM loading DB. Unrecoverable error, aborting now.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    rdbReportError(
        0 as libc::c_int,
        3240 as libc::c_int,
        b"Unexpected EOF reading RDB file\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbLoad(
    mut filename: *mut libc::c_char,
    mut rsi: *mut rdbSaveInfo,
    mut rdbflags: libc::c_int,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
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
    let mut retval: libc::c_int = 0;
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
    fp = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        return -(1 as libc::c_int);
    }
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        sb.st_size = 0 as libc::c_int as __off64_t;
    }
    startLoadingFile(sb.st_size as size_t, filename, rdbflags);
    rioInitWithFile(&mut rdb, fp);
    retval = rdbLoadRio(&mut rdb, rdbflags, rsi);
    fclose(fp);
    stopLoading((retval == 0 as libc::c_int) as libc::c_int);
    return retval;
}
unsafe extern "C" fn backgroundSaveDoneHandlerDisk(
    mut exitcode: libc::c_int,
    mut bysignal: libc::c_int,
) {
    if bysignal == 0 && exitcode == 0 as libc::c_int {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Background saving terminated with success\0" as *const u8
                    as *const libc::c_char,
            );
        }
        server.dirty = server.dirty - server.dirty_before_bgsave;
        server.lastsave = time(0 as *mut time_t);
        server.lastbgsave_status = 0 as libc::c_int;
    } else if bysignal == 0 && exitcode != 0 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Background saving error\0" as *const u8 as *const libc::c_char,
            );
        }
        server.lastbgsave_status = -(1 as libc::c_int);
    } else {
        let mut latency: mstime_t = 0;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Background saving terminated by signal %d\0" as *const u8
                    as *const libc::c_char,
                bysignal,
            );
        }
        if server.latency_monitor_threshold != 0 {
            latency = mstime();
        } else {
            latency = 0 as libc::c_int as mstime_t;
        }
        rdbRemoveTempFile(server.child_pid, 0 as libc::c_int);
        if server.latency_monitor_threshold != 0 {
            latency = mstime() - latency;
        }
        if server.latency_monitor_threshold != 0
            && latency >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"rdb-unlink-temp-file\0" as *const u8 as *const libc::c_char,
                latency,
            );
        }
        if bysignal != 10 as libc::c_int {
            server.lastbgsave_status = -(1 as libc::c_int);
        }
    };
}
unsafe extern "C" fn backgroundSaveDoneHandlerSocket(
    mut exitcode: libc::c_int,
    mut bysignal: libc::c_int,
) {
    if bysignal == 0 && exitcode == 0 as libc::c_int {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Background RDB transfer terminated with success\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if bysignal == 0 && exitcode != 0 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Background transfer error\0" as *const u8 as *const libc::c_char,
            );
        }
    } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Background transfer terminated by signal %d\0" as *const u8
                as *const libc::c_char,
            bysignal,
        );
    }
    if server.rdb_child_exit_pipe != -(1 as libc::c_int) {
        close(server.rdb_child_exit_pipe);
    }
    aeDeleteFileEvent(server.el, server.rdb_pipe_read, 1 as libc::c_int);
    close(server.rdb_pipe_read);
    server.rdb_child_exit_pipe = -(1 as libc::c_int);
    server.rdb_pipe_read = -(1 as libc::c_int);
    zfree(server.rdb_pipe_conns as *mut libc::c_void);
    server.rdb_pipe_conns = 0 as *mut *mut connection;
    server.rdb_pipe_numconns = 0 as libc::c_int;
    server.rdb_pipe_numconns_writing = 0 as libc::c_int;
    zfree(server.rdb_pipe_buff as *mut libc::c_void);
    server.rdb_pipe_buff = 0 as *mut libc::c_char;
    server.rdb_pipe_bufflen = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn backgroundSaveDoneHandler(
    mut exitcode: libc::c_int,
    mut bysignal: libc::c_int,
) {
    let mut type_0: libc::c_int = server.rdb_child_type;
    match server.rdb_child_type {
        1 => {
            backgroundSaveDoneHandlerDisk(exitcode, bysignal);
        }
        2 => {
            backgroundSaveDoneHandlerSocket(exitcode, bysignal);
        }
        _ => {
            _serverPanic(
                b"rdb.c\0" as *const u8 as *const libc::c_char,
                3339 as libc::c_int,
                b"Unknown RDB child type.\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    server.rdb_child_type = 0 as libc::c_int;
    server.rdb_save_time_last = time(0 as *mut time_t) - server.rdb_save_time_start;
    server.rdb_save_time_start = -(1 as libc::c_int) as time_t;
    updateSlavesWaitingBgsave(
        if bysignal == 0 && exitcode == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        },
        type_0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn killRDBChild() {
    kill(server.child_pid, 10 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveToSlavesSockets(
    mut req: libc::c_int,
    mut rsi: *mut rdbSaveInfo,
) -> libc::c_int {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut childpid: pid_t = 0;
    let mut pipefds: [libc::c_int; 2] = [0; 2];
    let mut rdb_pipe_write: libc::c_int = 0;
    let mut safe_to_exit_pipe: libc::c_int = 0;
    if hasActiveChildProcess() != 0 {
        return -(1 as libc::c_int);
    }
    if !(server.rdb_pipe_conns).is_null() {
        return -(1 as libc::c_int);
    }
    if anetPipe(pipefds.as_mut_ptr(), 0o4000 as libc::c_int, 0 as libc::c_int)
        == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    server.rdb_pipe_read = pipefds[0 as libc::c_int as usize];
    rdb_pipe_write = pipefds[1 as libc::c_int as usize];
    if anetPipe(pipefds.as_mut_ptr(), 0 as libc::c_int, 0 as libc::c_int)
        == -(1 as libc::c_int)
    {
        close(rdb_pipe_write);
        close(server.rdb_pipe_read);
        return -(1 as libc::c_int);
    }
    safe_to_exit_pipe = pipefds[0 as libc::c_int as usize];
    server.rdb_child_exit_pipe = pipefds[1 as libc::c_int as usize];
    server
        .rdb_pipe_conns = zmalloc(
        (core::mem::size_of::<*mut connection>() as libc::c_ulong)
            .wrapping_mul((*server.slaves).len),
    ) as *mut *mut connection;
    server.rdb_pipe_numconns = 0 as libc::c_int;
    server.rdb_pipe_numconns_writing = 0 as libc::c_int;
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
        if (*slave).slave_req != req {
            continue;
        }
        let fresh12 = server.rdb_pipe_numconns;
        server.rdb_pipe_numconns = server.rdb_pipe_numconns + 1;
        let ref mut fresh13 = *(server.rdb_pipe_conns).offset(fresh12 as isize);
        *fresh13 = (*slave).conn;
        replicationSetupSlaveForFullResync(slave, getPsyncInitialOffset());
    }
    childpid = redisFork(1 as libc::c_int);
    if childpid == 0 as libc::c_int {
        let mut retval: libc::c_int = 0;
        let mut dummy: libc::c_int = 0;
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
        rioInitWithFd(&mut rdb, rdb_pipe_write);
        close(server.rdb_pipe_read);
        redisSetProcTitle(
            b"redis-rdb-to-slaves\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        redisSetCpuAffinity(server.bgsave_cpulist);
        retval = rdbSaveRioWithEOFMark(req, &mut rdb, 0 as *mut libc::c_int, rsi);
        if retval == 0 as libc::c_int && rioFlush(&mut rdb) == 0 as libc::c_int {
            retval = -(1 as libc::c_int);
        }
        if retval == 0 as libc::c_int {
            sendChildCowInfo(
                CHILD_INFO_TYPE_RDB_COW_SIZE,
                b"RDB\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        rioFreeFd(&mut rdb);
        close(rdb_pipe_write);
        close(server.rdb_child_exit_pipe);
        dummy = read(
            safe_to_exit_pipe,
            pipefds.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        exitFromChild(
            if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
        );
    } else {
        if childpid == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't save in background: fork: %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            listRewind(server.slaves, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut slave_0: *mut client = (*ln).value as *mut client;
                if (*slave_0).replstate == 7 as libc::c_int {
                    (*slave_0).replstate = 6 as libc::c_int;
                }
            }
            close(rdb_pipe_write);
            close(server.rdb_pipe_read);
            zfree(server.rdb_pipe_conns as *mut libc::c_void);
            server.rdb_pipe_conns = 0 as *mut *mut connection;
            server.rdb_pipe_numconns = 0 as libc::c_int;
            server.rdb_pipe_numconns_writing = 0 as libc::c_int;
        } else {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Background RDB transfer started by pid %ld\0" as *const u8
                        as *const libc::c_char,
                    childpid as libc::c_long,
                );
            }
            server.rdb_save_time_start = time(0 as *mut time_t);
            server.rdb_child_type = 2 as libc::c_int;
            close(rdb_pipe_write);
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
                    b"rdb.c\0" as *const u8 as *const libc::c_char,
                    3474 as libc::c_int,
                    b"Unrecoverable error creating server.rdb_pipe_read file event.\0"
                        as *const u8 as *const libc::c_char,
                );
                unreachable!();
            }
        }
        close(safe_to_exit_pipe);
        return if childpid == -(1 as libc::c_int) {
            -(1 as libc::c_int)
        } else {
            0 as libc::c_int
        };
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn saveCommand(mut c: *mut client) {
    if server.child_type == 1 as libc::c_int {
        addReplyError(
            c,
            b"Background save already in progress\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    server.stat_rdb_saves += 1;
    let mut rsi: rdbSaveInfo = rdbSaveInfo {
        repl_stream_db: 0,
        repl_id_is_set: 0,
        repl_id: [0; 41],
        repl_offset: 0,
    };
    let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
    rsiptr = rdbPopulateSaveInfo(&mut rsi);
    if rdbSave(0 as libc::c_int, server.rdb_filename, rsiptr) == 0 as libc::c_int {
        addReply(c, shared.ok);
    } else {
        addReplyErrorObject(c, shared.err);
    };
}
#[no_mangle]
pub unsafe extern "C" fn bgsaveCommand(mut c: *mut client) {
    let mut schedule: libc::c_int = 0 as libc::c_int;
    if (*c).argc > 1 as libc::c_int {
        if (*c).argc == 2 as libc::c_int
            && strcasecmp(
                (**((*c).argv).offset(1 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"schedule\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            schedule = 1 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
    }
    let mut rsi: rdbSaveInfo = rdbSaveInfo {
        repl_stream_db: 0,
        repl_id_is_set: 0,
        repl_id: [0; 41],
        repl_offset: 0,
    };
    let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
    rsiptr = rdbPopulateSaveInfo(&mut rsi);
    if server.child_type == 1 as libc::c_int {
        addReplyError(
            c,
            b"Background save already in progress\0" as *const u8 as *const libc::c_char,
        );
    } else if hasActiveChildProcess() != 0 || server.in_exec != 0 {
        if schedule != 0 || server.in_exec != 0 {
            server.rdb_bgsave_scheduled = 1 as libc::c_int;
            addReplyStatus(
                c,
                b"Background saving scheduled\0" as *const u8 as *const libc::c_char,
            );
        } else {
            addReplyError(
                c,
                b"Another child process is active (AOF?): can't BGSAVE right now. Use BGSAVE SCHEDULE in order to schedule a BGSAVE whenever possible.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if rdbSaveBackground(0 as libc::c_int, server.rdb_filename, rsiptr)
        == 0 as libc::c_int
    {
        addReplyStatus(
            c,
            b"Background saving started\0" as *const u8 as *const libc::c_char,
        );
    } else {
        addReplyErrorObject(c, shared.err);
    };
}
#[no_mangle]
pub unsafe extern "C" fn rdbPopulateSaveInfo(
    mut rsi: *mut rdbSaveInfo,
) -> *mut rdbSaveInfo {
    let mut rsi_init: rdbSaveInfo = {
    let mut init = rdbSaveInfo {
        repl_stream_db: -(1 as libc::c_int),
        repl_id_is_set: 0 as libc::c_int,
        repl_id: {
            let id_bytes: &[u8; 41] = b"00000000000000000000000000000000000000000";
            let id_chars: &mut [libc::c_char; 41] = unsafe {
                &mut *(&id_bytes as *const _ as *mut [libc::c_char; 41])
            };
            let id_cell = UnsafeCell::new(id_chars);
            unsafe { **id_cell.get() }
        },
        repl_offset: -(1 as libc::c_int) as libc::c_longlong,
    };

        init
    };
    *rsi = rsi_init;
    if (server.masterhost).is_null() && !(server.repl_backlog).is_null() {
        (*rsi)
            .repl_stream_db = if server.slaveseldb == -(1 as libc::c_int) {
            0 as libc::c_int
        } else {
            server.slaveseldb
        };
        return rsi;
    }
    if !(server.master).is_null() {
        (*rsi).repl_stream_db = (*(*server.master).db).id;
        return rsi;
    }
    if !(server.cached_master).is_null() {
        (*rsi).repl_stream_db = (*(*server.cached_master).db).id;
        return rsi;
    }
    return 0 as *mut rdbSaveInfo;
}

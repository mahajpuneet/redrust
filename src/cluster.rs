extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn moduleClientIsBlockedOnKeys(c: *mut client) -> libc::c_int;
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
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplySds(c: *mut client, s: sds);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorSds(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyErrorArity(c: *mut client);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn rewriteClientCommandVector(c: *mut client, argc: libc::c_int, _: ...);
    fn rewriteClientCommandArgument(c: *mut client, i: libc::c_int, newval: *mut robj);
    fn replaceClientCommandVector(
        c: *mut client,
        argc: libc::c_int,
        argv: *mut *mut robj,
    );
    fn redactClientCommandArgument(c: *mut client, argc: libc::c_int);
    fn listenToPort(port: libc::c_int, fds: *mut socketFds) -> libc::c_int;
    fn pauseClients(purpose: pause_purpose, end: mstime_t, type_0: pause_type);
    fn unpauseClients(purpose: pause_purpose);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn createStringObjectFromLongLong(value: libc::c_longlong) -> *mut robj;
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
    fn equalStringObjects(a: *mut robj, b: *mut robj) -> libc::c_int;
    fn replicationSetMaster(ip: *mut libc::c_char, port: libc::c_int);
    fn replicationUnsetMaster();
    fn replicationGetSlaveOffset() -> libc::c_longlong;
    fn connCreateSocket() -> *mut connection;
    fn connCreateAcceptedSocket(fd: libc::c_int) -> *mut connection;
    fn connCreateTLS() -> *mut connection;
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
    fn connSockName(
        conn: *mut connection,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn rioInitWithBuffer(r: *mut rio, s: sds);
    fn rioWriteBulkCount(
        r: *mut rio,
        prefix: libc::c_char,
        count: libc::c_long,
    ) -> size_t;
    fn rioWriteBulkString(r: *mut rio, buf: *const libc::c_char, len: size_t) -> size_t;
    fn rioWriteBulkLongLong(r: *mut rio, l: libc::c_longlong) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoull(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_ulonglong;
    fn random() -> libc::c_long;
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
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn fsync(__fd: libc::c_int) -> libc::c_int;
    fn ftruncate(__fd: libc::c_int, __length: __off64_t) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGetRandomKey(d: *mut dict) -> *mut dictEntry;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn anetTcpAccept(
        err: *mut libc::c_char,
        serversock: libc::c_int,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
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
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn intrev64(v: uint64_t) -> uint64_t;
    fn crc64(crc: uint64_t, s: *const libc::c_uchar, l: uint64_t) -> uint64_t;
    fn __errno_location() -> *mut libc::c_int;
    fn sdsRemoveFreeSpace(s: sds) -> sds;
    fn sdsIncrLen(s: sds, incr: ssize_t);
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdsclear(s: sds);
    fn sdsrange(s: sds, start: ssize_t, end: ssize_t);
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscpy(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatsds(s: sds, t: sds) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsgrowzero(s: sds, len: size_t) -> sds;
    fn sdsfree(s: sds);
    fn sdsdup(s: sds) -> sds;
    fn sdsempty() -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn rdbSaveObjectType(rdb: *mut rio, o: *mut robj) -> libc::c_int;
    fn rdbLoadObjectType(rdb: *mut rio) -> libc::c_int;
    fn rdbSaveObject(
        rdb: *mut rio,
        o: *mut robj,
        key: *mut robj,
        dbid: libc::c_int,
    ) -> ssize_t;
    fn rdbLoadObject(
        type_0: libc::c_int,
        rdb: *mut rio,
        key: sds,
        dbid: libc::c_int,
        error: *mut libc::c_int,
    ) -> *mut robj;
    fn getCommandFlags(c: *mut client) -> uint64_t;
    fn createSocketAcceptHandler(
        sfd: *mut socketFds,
        accept_handler: Option::<aeFileProc>,
    ) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn LRU_CLOCK() -> libc::c_uint;
    fn pubsubUnsubscribeShardChannels(channels: *mut *mut robj, count: libc::c_uint);
    fn pubsubPublishMessage(
        channel: *mut robj,
        message: *mut robj,
        sharded: libc::c_int,
    ) -> libc::c_int;
    fn serverPubsubSubscriptionCount() -> libc::c_int;
    fn serverPubsubShardSubscriptionCount() -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn getExpire(db: *mut redisDb, key: *mut robj) -> libc::c_longlong;
    fn setExpire(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        when: libc::c_longlong,
    );
    fn checkAlreadyExpired(when: libc::c_longlong) -> libc::c_int;
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyWrite(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyReadWithFlags(
        db: *mut redisDb,
        key: *mut robj,
        flags: libc::c_int,
    ) -> *mut robj;
    fn objectSetLRUOrLFU(
        val: *mut robj,
        lfu_freq: libc::c_longlong,
        lru_idle: libc::c_longlong,
        lru_clock: libc::c_longlong,
        lru_multiplier: libc::c_int,
    ) -> libc::c_int;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn emptyData(
        dbnum: libc::c_int,
        flags: libc::c_int,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    ) -> libc::c_longlong;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn getKeysFromCommand(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn getKeysFreeResult(result: *mut getKeysResult);
    fn crc16(buf: *const libc::c_char, len: libc::c_int) -> libc::c_ushort;
    fn dictSdsHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsCaseHash(key: *const libc::c_void) -> uint64_t;
    fn dictSdsKeyCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn execCommand(c: *mut client);
    fn spublishCommand(c: *mut client);
    fn ssubscribeCommand(c: *mut client);
    fn sunsubscribeCommand(c: *mut client);
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
    fn _serverAssertWithInfo(
        c: *const client,
        o: *const robj,
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn inet_pton(
        __af: libc::c_int,
        __cp: *const libc::c_char,
        __buf: *mut libc::c_void,
    ) -> libc::c_int;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn moduleCallClusterReceivers(
        sender_id: *const libc::c_char,
        module_id: uint64_t,
        type_0: uint8_t,
        payload: *const libc::c_uchar,
        len: uint32_t,
    );
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
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
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
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
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
    pub v: C2RustUnnamed_5,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
pub type C2RustUnnamed_6 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_6 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_6 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_6 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_6 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_6 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_6 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_6 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_6 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_6 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_6 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_6 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_6 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_6 = 0;
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
pub type C2RustUnnamed_7 = libc::c_uint;
pub const CLUSTER_ENDPOINT_TYPE_UNKNOWN_ENDPOINT: C2RustUnnamed_7 = 2;
pub const CLUSTER_ENDPOINT_TYPE_HOSTNAME: C2RustUnnamed_7 = 1;
pub const CLUSTER_ENDPOINT_TYPE_IP: C2RustUnnamed_7 = 0;
pub type robj = redisObject;
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
    pub bs: C2RustUnnamed_11,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub range: C2RustUnnamed_10,
    pub keynum: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
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
pub union C2RustUnnamed_11 {
    pub index: C2RustUnnamed_13,
    pub keyword: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
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
    pub inst_metric: [C2RustUnnamed_14; 5],
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
pub struct C2RustUnnamed_14 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterNodeFailReport {
    pub node: *mut clusterNode,
    pub time: mstime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsg {
    pub sig: [libc::c_char; 4],
    pub totlen: uint32_t,
    pub ver: uint16_t,
    pub port: uint16_t,
    pub type_0: uint16_t,
    pub count: uint16_t,
    pub currentEpoch: uint64_t,
    pub configEpoch: uint64_t,
    pub offset: uint64_t,
    pub sender: [libc::c_char; 40],
    pub myslots: [libc::c_uchar; 2048],
    pub slaveof: [libc::c_char; 40],
    pub myip: [libc::c_char; 46],
    pub extensions: uint16_t,
    pub notused1: [libc::c_char; 30],
    pub pport: uint16_t,
    pub cport: uint16_t,
    pub flags: uint16_t,
    pub state: libc::c_uchar,
    pub mflags: [libc::c_uchar; 3],
    pub data: clusterMsgData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union clusterMsgData {
    pub ping: C2RustUnnamed_19,
    pub fail: C2RustUnnamed_18,
    pub publish: C2RustUnnamed_17,
    pub update: C2RustUnnamed_16,
    pub module: C2RustUnnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub msg: clusterMsgModule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgModule {
    pub module_id: uint64_t,
    pub len: uint32_t,
    pub type_0: uint8_t,
    pub bulk_data: [libc::c_uchar; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub nodecfg: clusterMsgDataUpdate,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgDataUpdate {
    pub configEpoch: uint64_t,
    pub nodename: [libc::c_char; 40],
    pub slots: [libc::c_uchar; 2048],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub msg: clusterMsgDataPublish,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgDataPublish {
    pub channel_len: uint32_t,
    pub message_len: uint32_t,
    pub bulk_data: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub about: clusterMsgDataFail,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgDataFail {
    pub nodename: [libc::c_char; 40],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub gossip: [clusterMsgDataGossip; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgDataGossip {
    pub nodename: [libc::c_char; 40],
    pub ping_sent: uint32_t,
    pub pong_received: uint32_t,
    pub ip: [libc::c_char; 46],
    pub port: uint16_t,
    pub cport: uint16_t,
    pub flags: uint16_t,
    pub pport: uint16_t,
    pub notused1: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgPingExt {
    pub length: uint32_t,
    pub type_0: uint16_t,
    pub unused: uint16_t,
    pub ext: [C2RustUnnamed_20; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub hostname: clusterMsgPingExtHostname,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterMsgPingExtHostname {
    pub hostname: [libc::c_char; 1],
}
pub const CLUSTERMSG_EXT_TYPE_HOSTNAME: C2RustUnnamed_21 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisNodeFlags {
    pub flag: uint16_t,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterDictEntryMetadata {
    pub prev: *mut dictEntry,
    pub next: *mut dictEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct migrateCachedSocket {
    pub conn: *mut connection,
    pub last_dbid: libc::c_long,
    pub last_use_time: time_t,
}
pub type C2RustUnnamed_21 = libc::c_uint;
#[inline]
unsafe extern "C" fn connAccept(
    mut conn: *mut connection,
    mut accept_handler: ConnectionCallbackFunc,
) -> libc::c_int {
    return ((*(*conn).type_0).accept)
        .expect("non-null function pointer")(conn, accept_handler);
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
unsafe extern "C" fn connBlockingConnect(
    mut conn: *mut connection,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut timeout: libc::c_longlong,
) -> libc::c_int {
    return ((*(*conn).type_0).blocking_connect)
        .expect("non-null function pointer")(conn, addr, port, timeout);
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
unsafe extern "C" fn connGetType(mut conn: *mut connection) -> libc::c_int {
    return ((*(*conn).type_0).get_type).expect("non-null function pointer")(conn);
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
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut myself: *mut clusterNode = 0 as *const clusterNode as *mut clusterNode;
#[no_mangle]
pub static mut clusterNodesDictType: dictType = unsafe {
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
pub static mut clusterNodesBlackListDictType: dictType = unsafe {
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
pub unsafe extern "C" fn clusterLoadConfig(
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fp: *mut FILE = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
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
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxline: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if fp.is_null() {
        if *__errno_location() == 2 as libc::c_int {
            return -(1 as libc::c_int)
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Loading the cluster node config from %s: %s\0" as *const u8
                        as *const libc::c_char,
                    filename,
                    strerror(*__errno_location()),
                );
            }
            exit(1 as libc::c_int);
        }
    }
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unable to obtain the cluster node config file stat %s: %s\0"
                    as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    if sb.st_size == 0 as libc::c_int as libc::c_long {
        fclose(fp);
        return -(1 as libc::c_int);
    }
    maxline = 1024 as libc::c_int + 16384 as libc::c_int * 128 as libc::c_int;
    line = zmalloc(maxline as size_t) as *mut libc::c_char;
    's_95: loop {
        if (fgets(line, maxline, fp)).is_null() {
            current_block = 5431927413890720344;
            break;
        }
        let mut argc: libc::c_int = 0;
        let mut argv: *mut sds = 0 as *mut sds;
        let mut n: *mut clusterNode = 0 as *mut clusterNode;
        let mut master: *mut clusterNode = 0 as *mut clusterNode;
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
        if *line.offset(0 as libc::c_int as isize) as libc::c_int == '\n' as i32
            || *line.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
        {
            continue;
        }
        argv = sdssplitargs(line, &mut argc);
        if argv.is_null() {
            current_block = 8085191762609820871;
            break;
        }
        if strcasecmp(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            b"vars\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if argc % 2 as libc::c_int == 0 {
                current_block = 8085191762609820871;
                break;
            }
            j = 1 as libc::c_int;
            while j < argc {
                if strcasecmp(
                    *argv.offset(j as isize) as *const libc::c_char,
                    b"currentEpoch\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*server.cluster)
                        .currentEpoch = strtoull(
                        *argv.offset((j + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as uint64_t;
                } else if strcasecmp(
                    *argv.offset(j as isize) as *const libc::c_char,
                    b"lastVoteEpoch\0" as *const u8 as *const libc::c_char,
                ) == 0 as libc::c_int
                {
                    (*server.cluster)
                        .lastVoteEpoch = strtoull(
                        *argv.offset((j + 1 as libc::c_int) as isize)
                            as *const libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    ) as uint64_t;
                } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                {
                    _serverLog(
                        3 as libc::c_int,
                        b"Skipping unknown cluster config variable '%s'\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(j as isize),
                    );
                }
                j += 2 as libc::c_int;
            }
            sdsfreesplitres(argv, argc);
        } else if argc < 8 as libc::c_int {
            sdsfreesplitres(argv, argc);
            current_block = 8085191762609820871;
            break;
        } else if verifyClusterNodeId(
            *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
            sdslen(*argv.offset(0 as libc::c_int as isize)) as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            sdsfreesplitres(argv, argc);
            current_block = 8085191762609820871;
            break;
        } else {
            n = clusterLookupNode(
                *argv.offset(0 as libc::c_int as isize) as *const libc::c_char,
                sdslen(*argv.offset(0 as libc::c_int as isize)) as libc::c_int,
            );
            if n.is_null() {
                n = createClusterNode(
                    *argv.offset(0 as libc::c_int as isize),
                    0 as libc::c_int,
                );
                clusterAddNode(n);
            }
            let mut hostname: *mut libc::c_char = strchr(
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                ',' as i32,
            );
            if !hostname.is_null() {
                *hostname = '\0' as i32 as libc::c_char;
                hostname = hostname.offset(1);
                (*n).hostname = sdscpy((*n).hostname, hostname);
            } else if sdslen((*n).hostname) != 0 as libc::c_int as libc::c_ulong {
                sdsclear((*n).hostname);
            }
            p = strrchr(
                *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                ':' as i32,
            );
            if p.is_null() {
                sdsfreesplitres(argv, argc);
                current_block = 8085191762609820871;
                break;
            } else {
                *p = '\0' as i32 as libc::c_char;
                memcpy(
                    ((*n).ip).as_mut_ptr() as *mut libc::c_void,
                    *argv.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    (strlen(
                        *argv.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                let mut port: *mut libc::c_char = p.offset(1 as libc::c_int as isize);
                let mut busp: *mut libc::c_char = strchr(port, '@' as i32);
                if !busp.is_null() {
                    *busp = '\0' as i32 as libc::c_char;
                    busp = busp.offset(1);
                }
                (*n).port = atoi(port);
                (*n)
                    .cport = if !busp.is_null() {
                    atoi(busp)
                } else {
                    (*n).port + 10000 as libc::c_int
                };
                s = *argv.offset(2 as libc::c_int as isize);
                p = s;
                while !p.is_null() {
                    p = strchr(s, ',' as i32);
                    if !p.is_null() {
                        *p = '\0' as i32 as libc::c_char;
                    }
                    if strcasecmp(s, b"myself\0" as *const u8 as *const libc::c_char)
                        == 0
                    {
                        if ((*server.cluster).myself).is_null() {} else {
                            _serverAssert(
                                b"server.cluster->myself == NULL\0" as *const u8
                                    as *const libc::c_char,
                                b"cluster.c\0" as *const u8 as *const libc::c_char,
                                265 as libc::c_int,
                            );
                            unreachable!();
                        };
                        (*server.cluster).myself = n;
                        myself = (*server.cluster).myself;
                        (*n).flags |= 16 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"master\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 1 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"slave\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 2 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"fail?\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 4 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"fail\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 8 as libc::c_int;
                        (*n).fail_time = mstime();
                    } else if strcasecmp(
                        s,
                        b"handshake\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 32 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"noaddr\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 64 as libc::c_int;
                    } else if strcasecmp(
                        s,
                        b"nofailover\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    {
                        (*n).flags |= 512 as libc::c_int;
                    } else if !(strcasecmp(
                        s,
                        b"noflags\0" as *const u8 as *const libc::c_char,
                    ) == 0)
                    {
                        _serverPanic(
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            286 as libc::c_int,
                            b"Unknown flag in redis cluster config file\0" as *const u8
                                as *const libc::c_char,
                        );
                        unreachable!();
                    }
                    if !p.is_null() {
                        s = p.offset(1 as libc::c_int as isize);
                    }
                }
                if *(*argv.offset(3 as libc::c_int as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32
                {
                    if verifyClusterNodeId(
                        *argv.offset(3 as libc::c_int as isize) as *const libc::c_char,
                        sdslen(*argv.offset(3 as libc::c_int as isize)) as libc::c_int,
                    ) == -(1 as libc::c_int)
                    {
                        sdsfreesplitres(argv, argc);
                        current_block = 8085191762609820871;
                        break;
                    } else {
                        master = clusterLookupNode(
                            *argv.offset(3 as libc::c_int as isize)
                                as *const libc::c_char,
                            sdslen(*argv.offset(3 as libc::c_int as isize))
                                as libc::c_int,
                        );
                        if master.is_null() {
                            master = createClusterNode(
                                *argv.offset(3 as libc::c_int as isize),
                                0 as libc::c_int,
                            );
                            clusterAddNode(master);
                        }
                        (*n).slaveof = master;
                        clusterNodeAddSlave(master, n);
                    }
                }
                if atoi(*argv.offset(4 as libc::c_int as isize) as *const libc::c_char)
                    != 0
                {
                    (*n).ping_sent = mstime();
                }
                if atoi(*argv.offset(5 as libc::c_int as isize) as *const libc::c_char)
                    != 0
                {
                    (*n).pong_received = mstime();
                }
                (*n)
                    .configEpoch = (if (*n).flags & 2 as libc::c_int != 0
                    && !((*n).slaveof).is_null()
                {
                    0 as libc::c_int as libc::c_ulonglong
                } else {
                    strtoull(
                        *argv.offset(6 as libc::c_int as isize) as *const libc::c_char,
                        0 as *mut *mut libc::c_char,
                        10 as libc::c_int,
                    )
                }) as uint64_t;
                j = 8 as libc::c_int;
                while j < argc {
                    let mut start: libc::c_int = 0;
                    let mut stop: libc::c_int = 0;
                    if *(*argv.offset(j as isize)).offset(0 as libc::c_int as isize)
                        as libc::c_int == '[' as i32
                    {
                        let mut slot: libc::c_int = 0;
                        let mut direction: libc::c_char = 0;
                        let mut cn: *mut clusterNode = 0 as *mut clusterNode;
                        p = strchr(
                            *argv.offset(j as isize) as *const libc::c_char,
                            '-' as i32,
                        );
                        if !p.is_null() {} else {
                            _serverAssert(
                                b"p != NULL\0" as *const u8 as *const libc::c_char,
                                b"cluster.c\0" as *const u8 as *const libc::c_char,
                                327 as libc::c_int,
                            );
                            unreachable!();
                        };
                        *p = '\0' as i32 as libc::c_char;
                        direction = *p.offset(1 as libc::c_int as isize);
                        slot = atoi(
                            (*argv.offset(j as isize)).offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                        );
                        if slot < 0 as libc::c_int || slot >= 16384 as libc::c_int {
                            sdsfreesplitres(argv, argc);
                            current_block = 8085191762609820871;
                            break 's_95;
                        } else {
                            p = p.offset(3 as libc::c_int as isize);
                            let mut pr: *mut libc::c_char = strchr(p, ']' as i32);
                            let mut node_len: size_t = pr.offset_from(p) as libc::c_long
                                as size_t;
                            if pr.is_null()
                                || verifyClusterNodeId(p, node_len as libc::c_int)
                                    == -(1 as libc::c_int)
                            {
                                sdsfreesplitres(argv, argc);
                                current_block = 8085191762609820871;
                                break 's_95;
                            } else {
                                cn = clusterLookupNode(p, 40 as libc::c_int);
                                if cn.is_null() {
                                    cn = createClusterNode(p, 0 as libc::c_int);
                                    clusterAddNode(cn);
                                }
                                if direction as libc::c_int == '>' as i32 {
                                    (*server.cluster).migrating_slots_to[slot as usize] = cn;
                                } else {
                                    (*server.cluster).importing_slots_from[slot as usize] = cn;
                                }
                            }
                        }
                    } else {
                        p = strchr(
                            *argv.offset(j as isize) as *const libc::c_char,
                            '-' as i32,
                        );
                        if !p.is_null() {
                            *p = '\0' as i32 as libc::c_char;
                            start = atoi(
                                *argv.offset(j as isize) as *const libc::c_char,
                            );
                            stop = atoi(p.offset(1 as libc::c_int as isize));
                        } else {
                            stop = atoi(*argv.offset(j as isize) as *const libc::c_char);
                            start = stop;
                        }
                        if start < 0 as libc::c_int || start >= 16384 as libc::c_int
                            || stop < 0 as libc::c_int || stop >= 16384 as libc::c_int
                        {
                            sdsfreesplitres(argv, argc);
                            current_block = 8085191762609820871;
                            break 's_95;
                        } else {
                            while start <= stop {
                                let fresh0 = start;
                                start = start + 1;
                                clusterAddSlot(n, fresh0);
                            }
                        }
                    }
                    j += 1;
                }
                sdsfreesplitres(argv, argc);
            }
        }
    }
    match current_block {
        5431927413890720344 => {
            if !((*server.cluster).myself).is_null() {
                zfree(line as *mut libc::c_void);
                fclose(fp);
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Node configuration loaded, I'm %.40s\0" as *const u8
                            as *const libc::c_char,
                        ((*myself).name).as_mut_ptr(),
                    );
                }
                if clusterGetMaxEpoch() > (*server.cluster).currentEpoch {
                    (*server.cluster).currentEpoch = clusterGetMaxEpoch();
                }
                return 0 as libc::c_int;
            }
        }
        _ => {}
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Unrecoverable error: corrupted cluster config file.\0" as *const u8
                as *const libc::c_char,
        );
    }
    zfree(line as *mut libc::c_void);
    if !fp.is_null() {
        fclose(fp);
    }
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn clusterSaveConfig(mut do_fsync: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut ci: sds = 0 as *mut libc::c_char;
    let mut content_size: size_t = 0;
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
    let mut fd: libc::c_int = 0;
    (*server.cluster).todo_before_sleep &= !((1 as libc::c_int) << 2 as libc::c_int);
    ci = clusterGenNodesDescription(32 as libc::c_int, 0 as libc::c_int);
    ci = sdscatprintf(
        ci,
        b"vars currentEpoch %llu lastVoteEpoch %llu\n\0" as *const u8
            as *const libc::c_char,
        (*server.cluster).currentEpoch as libc::c_ulonglong,
        (*server.cluster).lastVoteEpoch as libc::c_ulonglong,
    );
    content_size = sdslen(ci);
    fd = open(
        server.cluster_configfile,
        0o1 as libc::c_int | 0o100 as libc::c_int,
        0o644 as libc::c_int,
    );
    if !(fd == -(1 as libc::c_int)) {
        if !(fstat(fd, &mut sb) == -(1 as libc::c_int)) {
            if sb.st_size > content_size as off_t {
                ci = sdsgrowzero(ci, sb.st_size as size_t);
                memset(
                    ci.offset(content_size as isize) as *mut libc::c_void,
                    '\n' as i32,
                    (sb.st_size as libc::c_ulong).wrapping_sub(content_size),
                );
            }
            if !(write(fd, ci as *const libc::c_void, sdslen(ci))
                != sdslen(ci) as ssize_t)
            {
                if do_fsync != 0 {
                    (*server.cluster).todo_before_sleep
                        &= !((1 as libc::c_int) << 3 as libc::c_int);
                    if fsync(fd) == -(1 as libc::c_int) {
                        current_block = 4890479531577211101;
                    } else {
                        current_block = 2979737022853876585;
                    }
                } else {
                    current_block = 2979737022853876585;
                }
                match current_block {
                    4890479531577211101 => {}
                    _ => {
                        content_size != sdslen(ci)
                            && ftruncate(fd, content_size as __off64_t)
                                == -(1 as libc::c_int);
                        close(fd);
                        sdsfree(ci);
                        return 0 as libc::c_int;
                    }
                }
            }
        }
    }
    if fd != -(1 as libc::c_int) {
        close(fd);
    }
    sdsfree(ci);
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn clusterSaveConfigOrDie(mut do_fsync: libc::c_int) {
    if clusterSaveConfig(do_fsync) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fatal: can't update cluster config file.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterLockConfig(
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut fd: libc::c_int = open(
        filename,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o2000000 as libc::c_int,
        0o644 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't open %s in order to acquire a lock: %s\0" as *const u8
                    as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    if flock(fd, 2 as libc::c_int | 4 as libc::c_int) == -(1 as libc::c_int) {
        if *__errno_location() == 11 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Sorry, the cluster configuration file %s is already used by a different Redis Cluster node. Please make sure that different nodes use different cluster configuration files.\0"
                        as *const u8 as *const libc::c_char,
                    filename,
                );
            }
        } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Impossible to lock %s: %s\0" as *const u8 as *const libc::c_char,
                filename,
                strerror(*__errno_location()),
            );
        }
        close(fd);
        return -(1 as libc::c_int);
    }
    server.cluster_config_file_lock_fd = fd;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn deriveAnnouncedPorts(
    mut announced_port: *mut libc::c_int,
    mut announced_pport: *mut libc::c_int,
    mut announced_cport: *mut libc::c_int,
) {
    let mut port: libc::c_int = if server.tls_cluster != 0 {
        server.tls_port
    } else {
        server.port
    };
    *announced_port = port;
    *announced_pport = if server.tls_cluster != 0 {
        server.port
    } else {
        0 as libc::c_int
    };
    *announced_cport = if server.cluster_port != 0 {
        server.cluster_port
    } else {
        port + 10000 as libc::c_int
    };
    if server.tls_cluster != 0 && server.cluster_announce_tls_port != 0 {
        *announced_port = server.cluster_announce_tls_port;
        *announced_pport = server.cluster_announce_port;
    } else if server.cluster_announce_port != 0 {
        *announced_port = server.cluster_announce_port;
    }
    if server.cluster_announce_bus_port != 0 {
        *announced_cport = server.cluster_announce_bus_port;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateMyselfFlags() {
    if myself.is_null() {
        return;
    }
    let mut oldflags: libc::c_int = (*myself).flags;
    let mut nofailover: libc::c_int = if server.cluster_slave_no_failover != 0 {
        512 as libc::c_int
    } else {
        0 as libc::c_int
    };
    (*myself).flags &= !(512 as libc::c_int);
    (*myself).flags |= nofailover;
    if (*myself).flags != oldflags {
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateMyselfAnnouncedPorts() {
    if myself.is_null() {
        return;
    }
    deriveAnnouncedPorts(
        &mut (*myself).port,
        &mut (*myself).pport,
        &mut (*myself).cport,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateMyselfIp() {
    if myself.is_null() {
        return;
    }
    static mut prev_ip: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut curr_ip: *mut libc::c_char = server.cluster_announce_ip;
    let mut changed: libc::c_int = 0 as libc::c_int;
    if prev_ip.is_null() && !curr_ip.is_null() {
        changed = 1 as libc::c_int;
    } else if !prev_ip.is_null() && curr_ip.is_null() {
        changed = 1 as libc::c_int;
    } else if !prev_ip.is_null() && !curr_ip.is_null() && strcmp(prev_ip, curr_ip) != 0 {
        changed = 1 as libc::c_int;
    }
    if changed != 0 {
        if !prev_ip.is_null() {
            zfree(prev_ip as *mut libc::c_void);
        }
        prev_ip = curr_ip;
        if !curr_ip.is_null() {
            prev_ip = zstrdup(prev_ip);
            strncpy(
                ((*myself).ip).as_mut_ptr(),
                server.cluster_announce_ip,
                (46 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
            );
            (*myself)
                .ip[(46 as libc::c_int - 1 as libc::c_int)
                as usize] = '\0' as i32 as libc::c_char;
        } else {
            (*myself).ip[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        }
    }
}
unsafe extern "C" fn updateAnnouncedHostname(
    mut node: *mut clusterNode,
    mut new: *mut libc::c_char,
) {
    if !new.is_null() && strcmp(new, (*node).hostname as *const libc::c_char) == 0 {
        return;
    }
    if !new.is_null() {
        (*node).hostname = sdscpy((*node).hostname, new);
    } else if sdslen((*node).hostname) != 0 as libc::c_int as libc::c_ulong {
        sdsclear((*node).hostname);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateMyselfHostname() {
    if myself.is_null() {
        return;
    }
    updateAnnouncedHostname(myself, server.cluster_announce_hostname);
}
#[no_mangle]
pub unsafe extern "C" fn clusterInit() {
    let mut saveconf: libc::c_int = 0 as libc::c_int;
    server
        .cluster = zmalloc(core::mem::size_of::<clusterState>() as libc::c_ulong)
        as *mut clusterState;
    (*server.cluster).myself = 0 as *mut clusterNode;
    (*server.cluster).currentEpoch = 0 as libc::c_int as uint64_t;
    (*server.cluster).state = 1 as libc::c_int;
    (*server.cluster).size = 1 as libc::c_int;
    (*server.cluster).todo_before_sleep = 0 as libc::c_int;
    (*server.cluster).nodes = dictCreate(&mut clusterNodesDictType);
    (*server.cluster).nodes_black_list = dictCreate(&mut clusterNodesBlackListDictType);
    (*server.cluster).failover_auth_time = 0 as libc::c_int as mstime_t;
    (*server.cluster).failover_auth_count = 0 as libc::c_int;
    (*server.cluster).failover_auth_rank = 0 as libc::c_int;
    (*server.cluster).failover_auth_epoch = 0 as libc::c_int as uint64_t;
    (*server.cluster).cant_failover_reason = 0 as libc::c_int;
    (*server.cluster).lastVoteEpoch = 0 as libc::c_int as uint64_t;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 11 as libc::c_int {
        (*server.cluster)
            .stats_bus_messages_sent[i as usize] = 0 as libc::c_int as libc::c_longlong;
        (*server.cluster)
            .stats_bus_messages_received[i
            as usize] = 0 as libc::c_int as libc::c_longlong;
        i += 1;
    }
    (*server.cluster).stats_pfail_nodes = 0 as libc::c_int as libc::c_longlong;
    (*server.cluster)
        .stat_cluster_links_buffer_limit_exceeded = 0 as libc::c_int
        as libc::c_ulonglong;
    memset(
        ((*server.cluster).slots).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[*mut clusterNode; 16384]>() as libc::c_ulong,
    );
    clusterCloseAllSlots();
    server.cluster_config_file_lock_fd = -(1 as libc::c_int);
    if clusterLockConfig(server.cluster_configfile) == -(1 as libc::c_int) {
        exit(1 as libc::c_int);
    }
    if clusterLoadConfig(server.cluster_configfile) == -(1 as libc::c_int) {
        (*server.cluster)
            .myself = createClusterNode(
            0 as *mut libc::c_char,
            16 as libc::c_int | 1 as libc::c_int,
        );
        myself = (*server.cluster).myself;
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"No cluster configuration found, I'm %.40s\0" as *const u8
                    as *const libc::c_char,
                ((*myself).name).as_mut_ptr(),
            );
        }
        clusterAddNode(myself);
        saveconf = 1 as libc::c_int;
    }
    if saveconf != 0 {
        clusterSaveConfigOrDie(1 as libc::c_int);
    }
    server.cfd.count = 0 as libc::c_int;
    let mut port: libc::c_int = if server.tls_cluster != 0 {
        server.tls_port
    } else {
        server.port
    };
    if server.cluster_port == 0 && port > 65535 as libc::c_int - 10000 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Redis port number too high. Cluster communication port is 10,000 port numbers higher than your Redis port. Your Redis port number must be 55535 or less.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    if server.bindaddr_count == 0 {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"No bind address is configured, but it is required for the Cluster bus.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    let mut cport: libc::c_int = if server.cluster_port != 0 {
        server.cluster_port
    } else {
        port + 10000 as libc::c_int
    };
    if listenToPort(cport, &mut server.cfd) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed listening on port %u (cluster), aborting.\0" as *const u8
                    as *const libc::c_char,
                cport,
            );
        }
        exit(1 as libc::c_int);
    }
    if createSocketAcceptHandler(
        &mut server.cfd,
        Some(
            clusterAcceptHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) != 0 as libc::c_int
    {
        _serverPanic(
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            691 as libc::c_int,
            b"Unrecoverable error creating Redis Cluster socket accept handler.\0"
                as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    slotToKeyInit(server.db);
    (*server.cluster).slots_to_channels = raxNew();
    deriveAnnouncedPorts(
        &mut (*myself).port,
        &mut (*myself).pport,
        &mut (*myself).cport,
    );
    (*server.cluster).mf_end = 0 as libc::c_int as mstime_t;
    (*server.cluster).mf_slave = 0 as *mut clusterNode;
    resetManualFailover();
    clusterUpdateMyselfFlags();
    clusterUpdateMyselfIp();
    clusterUpdateMyselfHostname();
}
#[no_mangle]
pub unsafe extern "C" fn clusterReset(mut hard: libc::c_int) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut j: libc::c_int = 0;
    if (*myself).flags & 2 as libc::c_int != 0 {
        clusterSetNodeAsMaster(myself);
        replicationUnsetMaster();
        emptyData(-(1 as libc::c_int), 0 as libc::c_int, None);
    }
    clusterCloseAllSlots();
    resetManualFailover();
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        clusterDelSlot(j);
        j += 1;
    }
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if node == myself {
            continue;
        }
        clusterDelNode(node);
    }
    dictReleaseIterator(di);
    if hard != 0 {
        let mut oldname: sds = 0 as *mut libc::c_char;
        (*server.cluster).currentEpoch = 0 as libc::c_int as uint64_t;
        (*server.cluster).lastVoteEpoch = 0 as libc::c_int as uint64_t;
        (*myself).configEpoch = 0 as libc::c_int as uint64_t;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"configEpoch set to 0 via CLUSTER RESET HARD\0" as *const u8
                    as *const libc::c_char,
            );
        }
        oldname = sdsnewlen(
            ((*myself).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
        );
        dictDelete((*server.cluster).nodes, oldname as *const libc::c_void);
        sdsfree(oldname);
        getRandomHexChars(((*myself).name).as_mut_ptr(), 40 as libc::c_int as size_t);
        clusterAddNode(myself);
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Node hard reset, now I'm %.40s\0" as *const u8 as *const libc::c_char,
                ((*myself).name).as_mut_ptr(),
            );
        }
    }
    clusterDoBeforeSleep(
        (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn createClusterLink(
    mut node: *mut clusterNode,
) -> *mut clusterLink {
    let mut link: *mut clusterLink = zmalloc(
        core::mem::size_of::<clusterLink>() as libc::c_ulong,
    ) as *mut clusterLink;
    (*link).ctime = mstime();
    (*link).sndbuf = sdsempty();
    (*link).rcvbuf_alloc = 1024 as libc::c_int as size_t;
    (*link).rcvbuf = zmalloc((*link).rcvbuf_alloc) as *mut libc::c_char;
    (*link).rcvbuf_len = 0 as libc::c_int as size_t;
    (*link).conn = 0 as *mut connection;
    (*link).node = node;
    (*link)
        .inbound = (node == 0 as *mut libc::c_void as *mut clusterNode) as libc::c_int;
    if (*link).inbound == 0 {
        (*node).link = link;
    }
    return link;
}
#[no_mangle]
pub unsafe extern "C" fn freeClusterLink(mut link: *mut clusterLink) {
    if !((*link).conn).is_null() {
        connClose((*link).conn);
        (*link).conn = 0 as *mut connection;
    }
    sdsfree((*link).sndbuf);
    zfree((*link).rcvbuf as *mut libc::c_void);
    if !((*link).node).is_null() {
        if (*(*link).node).link == link {
            if (*link).inbound == 0 {} else {
                _serverAssert(
                    b"!link->inbound\0" as *const u8 as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    807 as libc::c_int,
                );
                unreachable!();
            };
            (*(*link).node).link = 0 as *mut clusterLink;
        } else if (*(*link).node).inbound_link == link {
            if (*link).inbound != 0 {} else {
                _serverAssert(
                    b"link->inbound\0" as *const u8 as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    810 as libc::c_int,
                );
                unreachable!();
            };
            (*(*link).node).inbound_link = 0 as *mut clusterLink;
        }
    }
    zfree(link as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn setClusterNodeToInboundClusterLink(
    mut node: *mut clusterNode,
    mut link: *mut clusterLink,
) {
    if ((*link).node).is_null() {} else {
        _serverAssert(
            b"!link->node\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            818 as libc::c_int,
        );
        unreachable!();
    };
    if (*link).inbound != 0 {} else {
        _serverAssert(
            b"link->inbound\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            819 as libc::c_int,
        );
        unreachable!();
    };
    if !((*node).inbound_link).is_null() {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"Replacing inbound link fd %d from node %.40s with fd %d\0" as *const u8
                    as *const libc::c_char,
                (*(*(*node).inbound_link).conn).fd,
                ((*node).name).as_mut_ptr(),
                (*(*link).conn).fd,
            );
        }
        freeClusterLink((*node).inbound_link);
    }
    if ((*node).inbound_link).is_null() {} else {
        _serverAssert(
            b"!node->inbound_link\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            832 as libc::c_int,
        );
        unreachable!();
    };
    (*node).inbound_link = link;
    (*link).node = node;
}
unsafe extern "C" fn clusterConnAcceptHandler(mut conn: *mut connection) {
    let mut link: *mut clusterLink = 0 as *mut clusterLink;
    if connGetState(conn) != CONN_STATE_CONNECTED as libc::c_int {
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Error accepting cluster node connection: %s\0" as *const u8
                    as *const libc::c_char,
                connGetLastError(conn),
            );
        }
        connClose(conn);
        return;
    }
    link = createClusterLink(0 as *mut clusterNode);
    (*link).conn = conn;
    connSetPrivateData(conn, link as *mut libc::c_void);
    connSetReadHandler(
        conn,
        Some(clusterReadHandler as unsafe extern "C" fn(*mut connection) -> ()),
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterAcceptHandler(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut cport: libc::c_int = 0;
    let mut cfd: libc::c_int = 0;
    let mut max: libc::c_int = 1000 as libc::c_int;
    let mut cip: [libc::c_char; 46] = [0; 46];
    if (server.masterhost).is_null() && server.loading != 0 {
        return;
    }
    loop {
        let fresh1 = max;
        max = max - 1;
        if !(fresh1 != 0) {
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
                if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        1 as libc::c_int,
                        b"Error accepting cluster node: %s\0" as *const u8
                            as *const libc::c_char,
                        (server.neterr).as_mut_ptr(),
                    );
                }
            }
            return;
        }
        let mut conn: *mut connection = if server.tls_cluster != 0 {
            connCreateAcceptedTLS(cfd, 1 as libc::c_int)
        } else {
            connCreateAcceptedSocket(cfd)
        };
        if connGetState(conn) != CONN_STATE_ACCEPTING as libc::c_int {
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Error creating an accepting connection for cluster node: %s\0"
                        as *const u8 as *const libc::c_char,
                    connGetLastError(conn),
                );
            }
            connClose(conn);
            return;
        }
        connEnableTcpNoDelay(conn);
        connKeepAlive(
            conn,
            (server.cluster_node_timeout * 2 as libc::c_int as libc::c_longlong)
                as libc::c_int,
        );
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Accepting cluster node connection from %s:%d\0" as *const u8
                    as *const libc::c_char,
                cip.as_mut_ptr(),
                cport,
            );
        }
        if connAccept(
            conn,
            Some(clusterConnAcceptHandler as unsafe extern "C" fn(*mut connection) -> ()),
        ) == -(1 as libc::c_int)
        {
            if connGetState(conn) == CONN_STATE_ERROR as libc::c_int {
                if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        1 as libc::c_int,
                        b"Error accepting cluster node connection: %s\0" as *const u8
                            as *const libc::c_char,
                        connGetLastError(conn),
                    );
                }
            }
            connClose(conn);
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn getClusterConnectionsCount() -> libc::c_ulong {
    return if server.cluster_enabled != 0 {
        ((*(*server.cluster).nodes).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize])
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
#[no_mangle]
pub unsafe extern "C" fn keyHashSlot(
    mut key: *mut libc::c_char,
    mut keylen: libc::c_int,
) -> libc::c_uint {
    let mut s: libc::c_int = 0;
    let mut e: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s < keylen {
        if *key.offset(s as isize) as libc::c_int == '{' as i32 {
            break;
        }
        s += 1;
    }
    if s == keylen {
        return (crc16(key, keylen) as libc::c_int & 0x3fff as libc::c_int)
            as libc::c_uint;
    }
    e = s + 1 as libc::c_int;
    while e < keylen {
        if *key.offset(e as isize) as libc::c_int == '}' as i32 {
            break;
        }
        e += 1;
    }
    if e == keylen || e == s + 1 as libc::c_int {
        return (crc16(key, keylen) as libc::c_int & 0x3fff as libc::c_int)
            as libc::c_uint;
    }
    return (crc16(
        key.offset(s as isize).offset(1 as libc::c_int as isize),
        e - s - 1 as libc::c_int,
    ) as libc::c_int & 0x3fff as libc::c_int) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn createClusterNode(
    mut nodename: *mut libc::c_char,
    mut flags: libc::c_int,
) -> *mut clusterNode {
    let mut node: *mut clusterNode = zmalloc(
        core::mem::size_of::<clusterNode>() as libc::c_ulong,
    ) as *mut clusterNode;
    if !nodename.is_null() {
        memcpy(
            ((*node).name).as_mut_ptr() as *mut libc::c_void,
            nodename as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
    } else {
        getRandomHexChars(((*node).name).as_mut_ptr(), 40 as libc::c_int as size_t);
    }
    (*node).ctime = mstime();
    (*node).configEpoch = 0 as libc::c_int as uint64_t;
    (*node).flags = flags;
    memset(
        ((*node).slots).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong,
    );
    (*node).slot_info_pairs = 0 as *mut uint16_t;
    (*node).slot_info_pairs_count = 0 as libc::c_int;
    (*node).numslots = 0 as libc::c_int;
    (*node).numslaves = 0 as libc::c_int;
    (*node).slaves = 0 as *mut *mut clusterNode;
    (*node).slaveof = 0 as *mut clusterNode;
    (*node).last_in_ping_gossip = 0 as libc::c_int as libc::c_ulonglong;
    (*node).pong_received = 0 as libc::c_int as mstime_t;
    (*node).ping_sent = (*node).pong_received;
    (*node).data_received = 0 as libc::c_int as mstime_t;
    (*node).fail_time = 0 as libc::c_int as mstime_t;
    (*node).link = 0 as *mut clusterLink;
    (*node).inbound_link = 0 as *mut clusterLink;
    memset(
        ((*node).ip).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
    );
    (*node).hostname = sdsempty();
    (*node).port = 0 as libc::c_int;
    (*node).cport = 0 as libc::c_int;
    (*node).pport = 0 as libc::c_int;
    (*node).fail_reports = listCreate();
    (*node).voted_time = 0 as libc::c_int as mstime_t;
    (*node).orphaned_time = 0 as libc::c_int as mstime_t;
    (*node).repl_offset_time = 0 as libc::c_int as mstime_t;
    (*node).repl_offset = 0 as libc::c_int as libc::c_longlong;
    (*(*node).fail_reports)
        .free = Some(zfree as unsafe extern "C" fn(*mut libc::c_void) -> ());
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeAddFailureReport(
    mut failing: *mut clusterNode,
    mut sender: *mut clusterNode,
) -> libc::c_int {
    let mut l: *mut list = (*failing).fail_reports;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut fr: *mut clusterNodeFailReport = 0 as *mut clusterNodeFailReport;
    listRewind(l, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        fr = (*ln).value as *mut clusterNodeFailReport;
        if (*fr).node == sender {
            (*fr).time = mstime();
            return 0 as libc::c_int;
        }
    }
    fr = zmalloc(core::mem::size_of::<clusterNodeFailReport>() as libc::c_ulong)
        as *mut clusterNodeFailReport;
    (*fr).node = sender;
    (*fr).time = mstime();
    listAddNodeTail(l, fr as *mut libc::c_void);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeCleanupFailureReports(mut node: *mut clusterNode) {
    let mut l: *mut list = (*node).fail_reports;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut fr: *mut clusterNodeFailReport = 0 as *mut clusterNodeFailReport;
    let mut maxtime: mstime_t = server.cluster_node_timeout
        * 2 as libc::c_int as libc::c_longlong;
    let mut now: mstime_t = mstime();
    listRewind(l, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        fr = (*ln).value as *mut clusterNodeFailReport;
        if now - (*fr).time > maxtime {
            listDelNode(l, ln);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeDelFailureReport(
    mut node: *mut clusterNode,
    mut sender: *mut clusterNode,
) -> libc::c_int {
    let mut l: *mut list = (*node).fail_reports;
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut fr: *mut clusterNodeFailReport = 0 as *mut clusterNodeFailReport;
    listRewind(l, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        fr = (*ln).value as *mut clusterNodeFailReport;
        if (*fr).node == sender {
            break;
        }
    }
    if ln.is_null() {
        return 0 as libc::c_int;
    }
    listDelNode(l, ln);
    clusterNodeCleanupFailureReports(node);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeFailureReportsCount(
    mut node: *mut clusterNode,
) -> libc::c_int {
    clusterNodeCleanupFailureReports(node);
    return (*(*node).fail_reports).len as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeRemoveSlave(
    mut master: *mut clusterNode,
    mut slave: *mut clusterNode,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*master).numslaves {
        if *((*master).slaves).offset(j as isize) == slave {
            if (j + 1 as libc::c_int) < (*master).numslaves {
                let mut remaining_slaves: libc::c_int = (*master).numslaves - j
                    - 1 as libc::c_int;
                memmove(
                    ((*master).slaves).offset(j as isize) as *mut libc::c_void,
                    ((*master).slaves).offset((j + 1 as libc::c_int) as isize)
                        as *const libc::c_void,
                    (core::mem::size_of::<*mut clusterNode>() as libc::c_ulong)
                        .wrapping_mul(remaining_slaves as libc::c_ulong),
                );
            }
            (*master).numslaves -= 1;
            if (*master).numslaves == 0 as libc::c_int {
                (*master).flags &= !(256 as libc::c_int);
            }
            return 0 as libc::c_int;
        }
        j += 1;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeAddSlave(
    mut master: *mut clusterNode,
    mut slave: *mut clusterNode,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*master).numslaves {
        if *((*master).slaves).offset(j as isize) == slave {
            return -(1 as libc::c_int);
        }
        j += 1;
    }
    (*master)
        .slaves = zrealloc(
        (*master).slaves as *mut libc::c_void,
        (core::mem::size_of::<*mut clusterNode>() as libc::c_ulong)
            .wrapping_mul(((*master).numslaves + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut clusterNode;
    let ref mut fresh2 = *((*master).slaves).offset((*master).numslaves as isize);
    *fresh2 = slave;
    (*master).numslaves += 1;
    (*master).flags |= 256 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterCountNonFailingSlaves(
    mut n: *mut clusterNode,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut okslaves: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < (*n).numslaves {
        if (**((*n).slaves).offset(j as isize)).flags & 8 as libc::c_int == 0 {
            okslaves += 1;
        }
        j += 1;
    }
    return okslaves;
}
#[no_mangle]
pub unsafe extern "C" fn freeClusterNode(mut n: *mut clusterNode) {
    let mut nodename: sds = 0 as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*n).numslaves {
        let ref mut fresh3 = (**((*n).slaves).offset(j as isize)).slaveof;
        *fresh3 = 0 as *mut clusterNode;
        j += 1;
    }
    if (*n).flags & 2 as libc::c_int != 0 && !((*n).slaveof).is_null() {
        clusterNodeRemoveSlave((*n).slaveof, n);
    }
    nodename = sdsnewlen(
        ((*n).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    if dictDelete((*server.cluster).nodes, nodename as *const libc::c_void)
        == 0 as libc::c_int
    {} else {
        _serverAssert(
            b"dictDelete(server.cluster->nodes,nodename) == DICT_OK\0" as *const u8
                as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            1153 as libc::c_int,
        );
        unreachable!();
    };
    sdsfree(nodename);
    sdsfree((*n).hostname);
    if !((*n).link).is_null() {
        freeClusterLink((*n).link);
    }
    if !((*n).inbound_link).is_null() {
        freeClusterLink((*n).inbound_link);
    }
    listRelease((*n).fail_reports);
    zfree((*n).slaves as *mut libc::c_void);
    zfree(n as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn clusterAddNode(mut node: *mut clusterNode) {
    let mut retval: libc::c_int = 0;
    retval = dictAdd(
        (*server.cluster).nodes,
        sdsnewlen(
            ((*node).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
        ) as *mut libc::c_void,
        node as *mut libc::c_void,
    );
    if retval == 0 as libc::c_int {} else {
        _serverAssert(
            b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            1171 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterDelNode(mut delnode: *mut clusterNode) {
    let mut j: libc::c_int = 0;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if (*server.cluster).importing_slots_from[j as usize] == delnode {
            (*server.cluster).importing_slots_from[j as usize] = 0 as *mut clusterNode;
        }
        if (*server.cluster).migrating_slots_to[j as usize] == delnode {
            (*server.cluster).migrating_slots_to[j as usize] = 0 as *mut clusterNode;
        }
        if (*server.cluster).slots[j as usize] == delnode {
            clusterDelSlot(j);
        }
        j += 1;
    }
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if node == delnode {
            continue;
        }
        clusterNodeDelFailureReport(node, delnode);
    }
    dictReleaseIterator(di);
    freeClusterNode(delnode);
}
#[no_mangle]
pub unsafe extern "C" fn verifyClusterNodeId(
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> libc::c_int {
    if length != 40 as libc::c_int {
        return -(1 as libc::c_int);
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < length {
        if !(*name.offset(i as isize) as libc::c_int >= 'a' as i32
            && *name.offset(i as isize) as libc::c_int <= 'z' as i32)
        {
            if !(*name.offset(i as isize) as libc::c_int >= '0' as i32
                && *name.offset(i as isize) as libc::c_int <= '9' as i32)
            {
                return -(1 as libc::c_int);
            }
        }
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterLookupNode(
    mut name: *const libc::c_char,
    mut length: libc::c_int,
) -> *mut clusterNode {
    if verifyClusterNodeId(name, length) != 0 as libc::c_int {
        return 0 as *mut clusterNode;
    }
    let mut s: sds = sdsnewlen(name as *const libc::c_void, length as size_t);
    let mut de: *mut dictEntry = dictFind(
        (*server.cluster).nodes,
        s as *const libc::c_void,
    );
    sdsfree(s);
    if de.is_null() {
        return 0 as *mut clusterNode;
    }
    return (*de).v.val as *mut clusterNode;
}
#[no_mangle]
pub unsafe extern "C" fn clusterGetNodesServingMySlots(
    mut node: *mut clusterNode,
) -> *mut list {
    let mut nodes_for_slot: *mut list = listCreate();
    let mut my_primary: *mut clusterNode = if (*node).flags & 1 as libc::c_int != 0 {
        node
    } else {
        (*node).slaveof
    };
    if !my_primary.is_null() {} else {
        _serverAssert(
            b"my_primary\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            1243 as libc::c_int,
        );
        unreachable!();
    };
    listAddNodeTail(nodes_for_slot, my_primary as *mut libc::c_void);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*my_primary).numslaves {
        listAddNodeTail(
            nodes_for_slot,
            *((*my_primary).slaves).offset(i as isize) as *mut libc::c_void,
        );
        i += 1;
    }
    return nodes_for_slot;
}
#[no_mangle]
pub unsafe extern "C" fn clusterRenameNode(
    mut node: *mut clusterNode,
    mut newname: *mut libc::c_char,
) {
    let mut retval: libc::c_int = 0;
    let mut s: sds = sdsnewlen(
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"Renaming node %.40s into %.40s\0" as *const u8 as *const libc::c_char,
            ((*node).name).as_mut_ptr(),
            newname,
        );
    }
    retval = dictDelete((*server.cluster).nodes, s as *const libc::c_void);
    sdsfree(s);
    if retval == 0 as libc::c_int {} else {
        _serverAssert(
            b"retval == DICT_OK\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            1263 as libc::c_int,
        );
        unreachable!();
    };
    memcpy(
        ((*node).name).as_mut_ptr() as *mut libc::c_void,
        newname as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    );
    clusterAddNode(node);
}
#[no_mangle]
pub unsafe extern "C" fn clusterGetMaxEpoch() -> uint64_t {
    let mut max: uint64_t = 0 as libc::c_int as uint64_t;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).configEpoch > max {
            max = (*node).configEpoch;
        }
    }
    dictReleaseIterator(di);
    if max < (*server.cluster).currentEpoch {
        max = (*server.cluster).currentEpoch;
    }
    return max;
}
#[no_mangle]
pub unsafe extern "C" fn clusterBumpConfigEpochWithoutConsensus() -> libc::c_int {
    let mut maxEpoch: uint64_t = clusterGetMaxEpoch();
    if (*myself).configEpoch == 0 as libc::c_int as libc::c_ulong
        || (*myself).configEpoch != maxEpoch
    {
        (*server.cluster)
            .currentEpoch = ((*server.cluster).currentEpoch).wrapping_add(1);
        (*myself).configEpoch = (*server.cluster).currentEpoch;
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"New configEpoch set to %llu\0" as *const u8 as *const libc::c_char,
                (*myself).configEpoch as libc::c_ulonglong,
            );
        }
        return 0 as libc::c_int;
    } else {
        return -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterHandleConfigEpochCollision(
    mut sender: *mut clusterNode,
) {
    if (*sender).configEpoch != (*myself).configEpoch
        || (*sender).flags & 1 as libc::c_int == 0
        || (*myself).flags & 1 as libc::c_int == 0
    {
        return;
    }
    if memcmp(
        ((*sender).name).as_mut_ptr() as *const libc::c_void,
        ((*myself).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    ) <= 0 as libc::c_int
    {
        return;
    }
    (*server.cluster).currentEpoch = ((*server.cluster).currentEpoch).wrapping_add(1);
    (*myself).configEpoch = (*server.cluster).currentEpoch;
    clusterSaveConfigOrDie(1 as libc::c_int);
    if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            1 as libc::c_int,
            b"WARNING: configEpoch collision with node %.40s. configEpoch set to %llu\0"
                as *const u8 as *const libc::c_char,
            ((*sender).name).as_mut_ptr(),
            (*myself).configEpoch as libc::c_ulonglong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterBlacklistCleanup() {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator((*server.cluster).nodes_black_list);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut expire: int64_t = (*de).v.u64_0 as int64_t;
        if expire < server.unixtime as libc::c_long {
            dictDelete((*server.cluster).nodes_black_list, (*de).key);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn clusterBlacklistAddNode(mut node: *mut clusterNode) {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut id: sds = sdsnewlen(
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    clusterBlacklistCleanup();
    if dictAdd(
        (*server.cluster).nodes_black_list,
        id as *mut libc::c_void,
        0 as *mut libc::c_void,
    ) == 0 as libc::c_int
    {
        id = sdsdup(id);
    }
    de = dictFind((*server.cluster).nodes_black_list, id as *const libc::c_void);
    (*de)
        .v
        .u64_0 = (time(0 as *mut time_t) + 60 as libc::c_int as libc::c_long)
        as uint64_t;
    sdsfree(id);
}
#[no_mangle]
pub unsafe extern "C" fn clusterBlacklistExists(
    mut nodeid: *mut libc::c_char,
) -> libc::c_int {
    let mut id: sds = sdsnewlen(
        nodeid as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    let mut retval: libc::c_int = 0;
    clusterBlacklistCleanup();
    retval = (dictFind((*server.cluster).nodes_black_list, id as *const libc::c_void)
        != 0 as *mut libc::c_void as *mut dictEntry) as libc::c_int;
    sdsfree(id);
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn markNodeAsFailingIfNeeded(mut node: *mut clusterNode) {
    let mut failures: libc::c_int = 0;
    let mut needed_quorum: libc::c_int = (*server.cluster).size / 2 as libc::c_int
        + 1 as libc::c_int;
    if (*node).flags & 4 as libc::c_int == 0 {
        return;
    }
    if (*node).flags & 8 as libc::c_int != 0 {
        return;
    }
    failures = clusterNodeFailureReportsCount(node);
    if (*myself).flags & 1 as libc::c_int != 0 {
        failures += 1;
    }
    if failures < needed_quorum {
        return;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Marking node %.40s as failing (quorum reached).\0" as *const u8
                as *const libc::c_char,
            ((*node).name).as_mut_ptr(),
        );
    }
    (*node).flags &= !(4 as libc::c_int);
    (*node).flags |= 8 as libc::c_int;
    (*node).fail_time = mstime();
    clusterSendFail(((*node).name).as_mut_ptr());
    clusterDoBeforeSleep(
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clearNodeFailureIfNeeded(mut node: *mut clusterNode) {
    let mut now: mstime_t = mstime();
    if (*node).flags & 8 as libc::c_int != 0 {} else {
        _serverAssert(
            b"nodeFailed(node)\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            1534 as libc::c_int,
        );
        unreachable!();
    };
    if (*node).flags & 2 as libc::c_int != 0 || (*node).numslots == 0 as libc::c_int {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Clear FAIL state for node %.40s: %s is reachable again.\0" as *const u8
                    as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
                if (*node).flags & 2 as libc::c_int != 0 {
                    b"replica\0" as *const u8 as *const libc::c_char
                } else {
                    b"master without slots\0" as *const u8 as *const libc::c_char
                },
            );
        }
        (*node).flags &= !(8 as libc::c_int);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
    }
    if (*node).flags & 1 as libc::c_int != 0 && (*node).numslots > 0 as libc::c_int
        && now - (*node).fail_time
            > server.cluster_node_timeout * 2 as libc::c_int as libc::c_longlong
    {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Clear FAIL state for node %.40s: is reachable again and nobody is serving its slots after some time.\0"
                    as *const u8 as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
            );
        }
        (*node).flags &= !(8 as libc::c_int);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterHandshakeInProgress(
    mut ip: *mut libc::c_char,
    mut port: libc::c_int,
    mut cport: libc::c_int,
) -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).flags & 32 as libc::c_int == 0 {
            continue;
        }
        if strcasecmp(((*node).ip).as_mut_ptr(), ip) == 0 && (*node).port == port
            && (*node).cport == cport
        {
            break;
        }
    }
    dictReleaseIterator(di);
    return (de != 0 as *mut libc::c_void as *mut dictEntry) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterStartHandshake(
    mut ip: *mut libc::c_char,
    mut port: libc::c_int,
    mut cport: libc::c_int,
) -> libc::c_int {
    let mut n: *mut clusterNode = 0 as *mut clusterNode;
    let mut norm_ip: [libc::c_char; 46] = [0; 46];
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    if inet_pton(
        2 as libc::c_int,
        ip,
        &mut (*(&mut sa as *mut sockaddr_storage as *mut sockaddr_in)).sin_addr
            as *mut in_addr as *mut libc::c_void,
    ) != 0
    {
        sa.ss_family = 2 as libc::c_int as sa_family_t;
    } else if inet_pton(
        10 as libc::c_int,
        ip,
        &mut (*(&mut sa as *mut sockaddr_storage as *mut sockaddr_in6)).sin6_addr
            as *mut in6_addr as *mut libc::c_void,
    ) != 0
    {
        sa.ss_family = 10 as libc::c_int as sa_family_t;
    } else {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if port <= 0 as libc::c_int || port > 65535 as libc::c_int
        || cport <= 0 as libc::c_int || cport > 65535 as libc::c_int
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    memset(
        norm_ip.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        46 as libc::c_int as libc::c_ulong,
    );
    if sa.ss_family as libc::c_int == 2 as libc::c_int {
        inet_ntop(
            2 as libc::c_int,
            &mut (*(&mut sa as *mut sockaddr_storage as *mut sockaddr_in)).sin_addr
                as *mut in_addr as *mut libc::c_void,
            norm_ip.as_mut_ptr(),
            46 as libc::c_int as socklen_t,
        );
    } else {
        inet_ntop(
            10 as libc::c_int,
            &mut (*(&mut sa as *mut sockaddr_storage as *mut sockaddr_in6)).sin6_addr
                as *mut in6_addr as *mut libc::c_void,
            norm_ip.as_mut_ptr(),
            46 as libc::c_int as socklen_t,
        );
    }
    if clusterHandshakeInProgress(norm_ip.as_mut_ptr(), port, cport) != 0 {
        *__errno_location() = 11 as libc::c_int;
        return 0 as libc::c_int;
    }
    n = createClusterNode(
        0 as *mut libc::c_char,
        32 as libc::c_int | 128 as libc::c_int,
    );
    memcpy(
        ((*n).ip).as_mut_ptr() as *mut libc::c_void,
        norm_ip.as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
    );
    (*n).port = port;
    (*n).cport = cport;
    clusterAddNode(n);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterProcessGossipSection(
    mut hdr: *mut clusterMsg,
    mut link: *mut clusterLink,
) {
    let mut count: uint16_t = __bswap_16((*hdr).count);
    let mut g: *mut clusterMsgDataGossip = ((*hdr).data.ping.gossip).as_mut_ptr();
    let mut sender: *mut clusterNode = if !((*link).node).is_null() {
        (*link).node
    } else {
        clusterLookupNode(((*hdr).sender).as_mut_ptr(), 40 as libc::c_int)
    };
    loop {
        let fresh4 = count;
        count = count.wrapping_sub(1);
        if !(fresh4 != 0) {
            break;
        }
        let mut flags: uint16_t = __bswap_16((*g).flags);
        let mut node: *mut clusterNode = 0 as *mut clusterNode;
        let mut ci: sds = 0 as *mut libc::c_char;
        if server.verbosity == 0 as libc::c_int {
            ci = representClusterNodeFlags(sdsempty(), flags);
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"GOSSIP %.40s %s:%d@%d %s\0" as *const u8 as *const libc::c_char,
                    ((*g).nodename).as_mut_ptr(),
                    ((*g).ip).as_mut_ptr(),
                    __bswap_16((*g).port) as libc::c_int,
                    __bswap_16((*g).cport) as libc::c_int,
                    ci,
                );
            }
            sdsfree(ci);
        }
        node = clusterLookupNode(((*g).nodename).as_mut_ptr(), 40 as libc::c_int);
        if !node.is_null() {
            if !sender.is_null() && (*sender).flags & 1 as libc::c_int != 0
                && node != myself
            {
                if flags as libc::c_int & (8 as libc::c_int | 4 as libc::c_int) != 0 {
                    if clusterNodeAddFailureReport(node, sender) != 0 {
                        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                1 as libc::c_int,
                                b"Node %.40s reported node %.40s as not reachable.\0"
                                    as *const u8 as *const libc::c_char,
                                ((*sender).name).as_mut_ptr(),
                                ((*node).name).as_mut_ptr(),
                            );
                        }
                    }
                    markNodeAsFailingIfNeeded(node);
                } else if clusterNodeDelFailureReport(node, sender) != 0 {
                    if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            1 as libc::c_int,
                            b"Node %.40s reported node %.40s is back online.\0"
                                as *const u8 as *const libc::c_char,
                            ((*sender).name).as_mut_ptr(),
                            ((*node).name).as_mut_ptr(),
                        );
                    }
                }
            }
            if flags as libc::c_int & (8 as libc::c_int | 4 as libc::c_int) == 0
                && (*node).ping_sent == 0 as libc::c_int as libc::c_longlong
                && clusterNodeFailureReportsCount(node) == 0 as libc::c_int
            {
                let mut pongtime: mstime_t = __bswap_32((*g).pong_received) as mstime_t;
                pongtime *= 1000 as libc::c_int as libc::c_longlong;
                if pongtime <= server.mstime + 500 as libc::c_int as libc::c_longlong
                    && pongtime > (*node).pong_received
                {
                    (*node).pong_received = pongtime;
                }
            }
            if (*node).flags & (8 as libc::c_int | 4 as libc::c_int) != 0
                && flags as libc::c_int & 64 as libc::c_int == 0
                && flags as libc::c_int & (8 as libc::c_int | 4 as libc::c_int) == 0
                && (strcasecmp(((*node).ip).as_mut_ptr(), ((*g).ip).as_mut_ptr()) != 0
                    || (*node).port != __bswap_16((*g).port) as libc::c_int
                    || (*node).cport != __bswap_16((*g).cport) as libc::c_int)
            {
                if !((*node).link).is_null() {
                    freeClusterLink((*node).link);
                }
                memcpy(
                    ((*node).ip).as_mut_ptr() as *mut libc::c_void,
                    ((*g).ip).as_mut_ptr() as *const libc::c_void,
                    46 as libc::c_int as libc::c_ulong,
                );
                (*node).port = __bswap_16((*g).port) as libc::c_int;
                (*node).pport = __bswap_16((*g).pport) as libc::c_int;
                (*node).cport = __bswap_16((*g).cport) as libc::c_int;
                (*node).flags &= !(64 as libc::c_int);
            }
        } else if !sender.is_null() && flags as libc::c_int & 64 as libc::c_int == 0
            && clusterBlacklistExists(((*g).nodename).as_mut_ptr()) == 0
        {
            let mut node_0: *mut clusterNode = 0 as *mut clusterNode;
            node_0 = createClusterNode(
                ((*g).nodename).as_mut_ptr(),
                flags as libc::c_int,
            );
            memcpy(
                ((*node_0).ip).as_mut_ptr() as *mut libc::c_void,
                ((*g).ip).as_mut_ptr() as *const libc::c_void,
                46 as libc::c_int as libc::c_ulong,
            );
            (*node_0).port = __bswap_16((*g).port) as libc::c_int;
            (*node_0).pport = __bswap_16((*g).pport) as libc::c_int;
            (*node_0).cport = __bswap_16((*g).cport) as libc::c_int;
            clusterAddNode(node_0);
        }
        g = g.offset(1);
    };
}
#[no_mangle]
pub unsafe extern "C" fn nodeIp2String(
    mut buf: *mut libc::c_char,
    mut link: *mut clusterLink,
    mut announced_ip: *mut libc::c_char,
) -> libc::c_int {
    if *announced_ip.offset(0 as libc::c_int as isize) as libc::c_int != '\0' as i32 {
        memcpy(
            buf as *mut libc::c_void,
            announced_ip as *const libc::c_void,
            46 as libc::c_int as libc::c_ulong,
        );
        *buf
            .offset(
                (46 as libc::c_int - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        return 0 as libc::c_int;
    } else {
        if connPeerToString(
            (*link).conn,
            buf,
            46 as libc::c_int as size_t,
            0 as *mut libc::c_int,
        ) == -(1 as libc::c_int)
        {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Error converting peer IP to string: %s\0" as *const u8
                        as *const libc::c_char,
                    if !((*link).conn).is_null() {
                        connGetLastError((*link).conn)
                    } else {
                        b"no link\0" as *const u8 as *const libc::c_char
                    },
                );
            }
            return -(1 as libc::c_int);
        }
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn nodeUpdateAddressIfNeeded(
    mut node: *mut clusterNode,
    mut link: *mut clusterLink,
    mut hdr: *mut clusterMsg,
) -> libc::c_int {
    let mut ip: [libc::c_char; 46] = [
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
    ];
    let mut port: libc::c_int = __bswap_16((*hdr).port) as libc::c_int;
    let mut pport: libc::c_int = __bswap_16((*hdr).pport) as libc::c_int;
    let mut cport: libc::c_int = __bswap_16((*hdr).cport) as libc::c_int;
    if link == (*node).link {
        return 0 as libc::c_int;
    }
    if nodeIp2String(ip.as_mut_ptr(), link, ((*hdr).myip).as_mut_ptr())
        == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if (*node).port == port && (*node).cport == cport && (*node).pport == pport
        && strcmp(ip.as_mut_ptr(), ((*node).ip).as_mut_ptr()) == 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    memcpy(
        ((*node).ip).as_mut_ptr() as *mut libc::c_void,
        ip.as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
    );
    (*node).port = port;
    (*node).pport = pport;
    (*node).cport = cport;
    if !((*node).link).is_null() {
        freeClusterLink((*node).link);
    }
    (*node).flags &= !(64 as libc::c_int);
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Address updated for node %.40s, now %s:%d\0" as *const u8
                as *const libc::c_char,
            ((*node).name).as_mut_ptr(),
            ((*node).ip).as_mut_ptr(),
            (*node).port,
        );
    }
    if (*myself).flags & 2 as libc::c_int != 0 && (*myself).slaveof == node {
        replicationSetMaster(((*node).ip).as_mut_ptr(), (*node).port);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterSetNodeAsMaster(mut n: *mut clusterNode) {
    if (*n).flags & 1 as libc::c_int != 0 {
        return;
    }
    if !((*n).slaveof).is_null() {
        clusterNodeRemoveSlave((*n).slaveof, n);
        if n != myself {
            (*n).flags |= 256 as libc::c_int;
        }
    }
    (*n).flags &= !(2 as libc::c_int);
    (*n).flags |= 1 as libc::c_int;
    (*n).slaveof = 0 as *mut clusterNode;
    clusterDoBeforeSleep(
        (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateSlotsConfigWith(
    mut sender: *mut clusterNode,
    mut senderConfigEpoch: uint64_t,
    mut slots: *mut libc::c_uchar,
) {
    let mut j: libc::c_int = 0;
    let mut curmaster: *mut clusterNode = 0 as *mut clusterNode;
    let mut newmaster: *mut clusterNode = 0 as *mut clusterNode;
    let mut dirty_slots: [uint16_t; 16384] = [0; 16384];
    let mut dirty_slots_count: libc::c_int = 0 as libc::c_int;
    let mut sender_slots: libc::c_int = 0 as libc::c_int;
    let mut migrated_our_slots: libc::c_int = 0 as libc::c_int;
    curmaster = if (*myself).flags & 1 as libc::c_int != 0 {
        myself
    } else {
        (*myself).slaveof
    };
    if sender == myself {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Discarding UPDATE message about myself.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return;
    }
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if bitmapTestBit(slots, j) != 0 {
            sender_slots += 1;
            if !((*server.cluster).slots[j as usize] == sender) {
                if ((*server.cluster).importing_slots_from[j as usize]).is_null() {
                    if ((*server.cluster).slots[j as usize]).is_null()
                        || (*(*server.cluster).slots[j as usize]).configEpoch
                            < senderConfigEpoch
                    {
                        if (*server.cluster).slots[j as usize] == myself
                            && countKeysInSlot(j as libc::c_uint) != 0
                            && sender != myself
                        {
                            dirty_slots[dirty_slots_count as usize] = j as uint16_t;
                            dirty_slots_count += 1;
                        }
                        if (*server.cluster).slots[j as usize] == curmaster {
                            newmaster = sender;
                            migrated_our_slots += 1;
                        }
                        clusterDelSlot(j);
                        clusterAddSlot(sender, j);
                        clusterDoBeforeSleep(
                            (1 as libc::c_int) << 2 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int
                                | (1 as libc::c_int) << 3 as libc::c_int,
                        );
                    }
                }
            }
        }
        j += 1;
    }
    if server.cluster_module_flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return;
    }
    if !newmaster.is_null() && (*curmaster).numslots == 0 as libc::c_int
        && (server.cluster_allow_replica_migration != 0
            || sender_slots == migrated_our_slots)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Configuration change detected. Reconfiguring myself as a replica of %.40s\0"
                    as *const u8 as *const libc::c_char,
                ((*sender).name).as_mut_ptr(),
            );
        }
        clusterSetMaster(sender);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
    } else if !((*myself).slaveof).is_null() && !((*(*myself).slaveof).slaveof).is_null()
        && (*(*myself).slaveof).slaveof != myself
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"I'm a sub-replica! Reconfiguring myself as a replica of grandmaster %.40s\0"
                    as *const u8 as *const libc::c_char,
                ((*(*(*myself).slaveof).slaveof).name).as_mut_ptr(),
            );
        }
        clusterSetMaster((*(*myself).slaveof).slaveof);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
    } else if dirty_slots_count != 0 {
        j = 0 as libc::c_int;
        while j < dirty_slots_count {
            delKeysInSlot(dirty_slots[j as usize] as libc::c_uint);
            j += 1;
        }
    }
}
unsafe extern "C" fn getPingExtLength(mut ext: *mut clusterMsgPingExt) -> uint32_t {
    return __bswap_32((*ext).length);
}
unsafe extern "C" fn getInitialPingExt(
    mut hdr: *mut clusterMsg,
    mut count: uint16_t,
) -> *mut clusterMsgPingExt {
    let mut initial: *mut clusterMsgPingExt = &mut *((*hdr).data.ping.gossip)
        .as_mut_ptr()
        .offset(count as isize) as *mut clusterMsgDataGossip as *mut clusterMsgPingExt;
    return initial;
}
unsafe extern "C" fn getNextPingExt(
    mut ext: *mut clusterMsgPingExt,
) -> *mut clusterMsgPingExt {
    let mut next: *mut clusterMsgPingExt = (ext as *mut libc::c_char)
        .offset(getPingExtLength(ext) as isize) as *mut clusterMsgPingExt;
    return next;
}
#[no_mangle]
pub unsafe extern "C" fn getHostnamePingExtSize() -> libc::c_int {
    if sdslen((*myself).hostname) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut totlen: libc::c_int = (core::mem::size_of::<clusterMsgPingExt>()
        as libc::c_ulong)
        .wrapping_add(
            (sdslen((*myself).hostname))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong),
        ) as libc::c_int;
    return totlen;
}
#[no_mangle]
pub unsafe extern "C" fn writeHostnamePingExt(
    mut cursor: *mut *mut clusterMsgPingExt,
) -> libc::c_int {
    if sdslen((*myself).hostname) == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut ext: *mut clusterMsgPingExtHostname = &mut (*((**cursor).ext)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize))
        .hostname;
    memcpy(
        ((*ext).hostname).as_mut_ptr() as *mut libc::c_void,
        (*myself).hostname as *const libc::c_void,
        sdslen((*myself).hostname),
    );
    let mut extension_size: uint32_t = getHostnamePingExtSize() as uint32_t;
    (**cursor)
        .type_0 = __bswap_16(CLUSTERMSG_EXT_TYPE_HOSTNAME as libc::c_int as __uint16_t);
    (**cursor).length = __bswap_32(extension_size);
    *cursor = ((*ext).hostname)
        .as_mut_ptr()
        .offset(
            (sdslen((*myself).hostname))
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                .wrapping_add(7 as libc::c_int as libc::c_ulong)
                .wrapping_div(8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong) as isize,
        ) as *mut clusterMsgPingExt;
    return extension_size as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterProcessPingExtensions(
    mut hdr: *mut clusterMsg,
    mut link: *mut clusterLink,
) {
    let mut sender: *mut clusterNode = if !((*link).node).is_null() {
        (*link).node
    } else {
        clusterLookupNode(((*hdr).sender).as_mut_ptr(), 40 as libc::c_int)
    };
    let mut ext_hostname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut extensions: uint16_t = __bswap_16((*hdr).extensions);
    let mut ext: *mut clusterMsgPingExt = getInitialPingExt(
        hdr,
        __bswap_16((*hdr).count),
    );
    loop {
        let fresh5 = extensions;
        extensions = extensions.wrapping_sub(1);
        if !(fresh5 != 0) {
            break;
        }
        let mut type_0: uint16_t = __bswap_16((*ext).type_0);
        if type_0 as libc::c_int == CLUSTERMSG_EXT_TYPE_HOSTNAME as libc::c_int {
            let mut hostname_ext: *mut clusterMsgPingExtHostname = &mut (*((*ext).ext)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .hostname as *mut clusterMsgPingExtHostname;
            ext_hostname = ((*hostname_ext).hostname).as_mut_ptr();
        } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Received unknown extension type %d\0" as *const u8
                    as *const libc::c_char,
                type_0 as libc::c_int,
            );
        }
        ext = getNextPingExt(ext);
    }
    updateAnnouncedHostname(sender, ext_hostname);
}
unsafe extern "C" fn getNodeFromLinkAndMsg(
    mut link: *mut clusterLink,
    mut hdr: *mut clusterMsg,
) -> *mut clusterNode {
    let mut sender: *mut clusterNode = 0 as *mut clusterNode;
    if !((*link).node).is_null() && (*(*link).node).flags & 32 as libc::c_int == 0 {
        sender = (*link).node;
    } else {
        sender = clusterLookupNode(((*hdr).sender).as_mut_ptr(), 40 as libc::c_int);
        if !sender.is_null() && ((*link).node).is_null() {
            setClusterNodeToInboundClusterLink(sender, link);
        }
    }
    return sender;
}
#[no_mangle]
pub unsafe extern "C" fn clusterProcessPacket(
    mut link: *mut clusterLink,
) -> libc::c_int {
    let mut hdr: *mut clusterMsg = (*link).rcvbuf as *mut clusterMsg;
    let mut totlen: uint32_t = __bswap_32((*hdr).totlen);
    let mut type_0: uint16_t = __bswap_16((*hdr).type_0);
    let mut now: mstime_t = mstime();
    if (type_0 as libc::c_int) < 11 as libc::c_int {
        (*server.cluster).stats_bus_messages_received[type_0 as usize] += 1;
    }
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"--- Processing packet of type %s, %lu bytes\0" as *const u8
                as *const libc::c_char,
            clusterGetMessageTypeString(type_0 as libc::c_int),
            totlen as libc::c_ulong,
        );
    }
    if totlen < 16 as libc::c_int as libc::c_uint {
        return 1 as libc::c_int;
    }
    if totlen as libc::c_ulong > (*link).rcvbuf_len {
        return 1 as libc::c_int;
    }
    if __bswap_16((*hdr).ver) as libc::c_int != 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    if type_0 as libc::c_int == server.cluster_drop_packet_filter {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Dropping packet that matches debug drop filter\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 1 as libc::c_int;
    }
    let mut flags: uint16_t = __bswap_16((*hdr).flags);
    let mut extensions: uint16_t = __bswap_16((*hdr).extensions);
    let mut senderCurrentEpoch: uint64_t = 0 as libc::c_int as uint64_t;
    let mut senderConfigEpoch: uint64_t = 0 as libc::c_int as uint64_t;
    let mut explen: uint32_t = 0;
    let mut sender: *mut clusterNode = 0 as *mut clusterNode;
    if type_0 as libc::c_int == 0 as libc::c_int
        || type_0 as libc::c_int == 1 as libc::c_int
        || type_0 as libc::c_int == 2 as libc::c_int
    {
        let mut count: uint16_t = __bswap_16((*hdr).count);
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
        explen = (explen as libc::c_ulong)
            .wrapping_add(
                (core::mem::size_of::<clusterMsgDataGossip>() as libc::c_ulong)
                    .wrapping_mul(count as libc::c_ulong),
            ) as uint32_t as uint32_t;
        if (*hdr).mflags[0 as libc::c_int as usize] as libc::c_int
            & (1 as libc::c_int) << 2 as libc::c_int != 0
        {
            let mut ext: *mut clusterMsgPingExt = getInitialPingExt(hdr, count);
            loop {
                let fresh6 = extensions;
                extensions = extensions.wrapping_sub(1);
                if !(fresh6 != 0) {
                    break;
                }
                let mut extlen: uint16_t = getPingExtLength(ext) as uint16_t;
                if extlen as libc::c_int % 8 as libc::c_int != 0 as libc::c_int {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Received a %s packet without proper padding (%d bytes)\0"
                                as *const u8 as *const libc::c_char,
                            clusterGetMessageTypeString(type_0 as libc::c_int),
                            extlen as libc::c_int,
                        );
                    }
                    return 1 as libc::c_int;
                }
                if totlen.wrapping_sub(explen) < extlen as libc::c_uint {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Received invalid %s packet with extension data that exceeds total packet length (%lld)\0"
                                as *const u8 as *const libc::c_char,
                            clusterGetMessageTypeString(type_0 as libc::c_int),
                            totlen as libc::c_ulonglong,
                        );
                    }
                    return 1 as libc::c_int;
                }
                explen = (explen as libc::c_uint).wrapping_add(extlen as libc::c_uint)
                    as uint32_t as uint32_t;
                ext = getNextPingExt(ext);
            }
        }
    } else if type_0 as libc::c_int == 3 as libc::c_int {
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
        explen = (explen as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<clusterMsgDataFail>() as libc::c_ulong)
            as uint32_t as uint32_t;
    } else if type_0 as libc::c_int == 4 as libc::c_int
        || type_0 as libc::c_int == 10 as libc::c_int
    {
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
        explen = (explen as libc::c_ulong)
            .wrapping_add(
                (core::mem::size_of::<clusterMsgDataPublish>() as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        __bswap_32((*hdr).data.publish.msg.channel_len) as libc::c_ulong,
                    )
                    .wrapping_add(
                        __bswap_32((*hdr).data.publish.msg.message_len) as libc::c_ulong,
                    ),
            ) as uint32_t as uint32_t;
    } else if type_0 as libc::c_int == 5 as libc::c_int
        || type_0 as libc::c_int == 6 as libc::c_int
        || type_0 as libc::c_int == 8 as libc::c_int
    {
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
    } else if type_0 as libc::c_int == 7 as libc::c_int {
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
        explen = (explen as libc::c_ulong)
            .wrapping_add(
                core::mem::size_of::<clusterMsgDataUpdate>() as libc::c_ulong,
            ) as uint32_t as uint32_t;
    } else if type_0 as libc::c_int == 9 as libc::c_int {
        explen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as uint32_t;
        explen = (explen as libc::c_ulong)
            .wrapping_add(
                (core::mem::size_of::<clusterMsgModule>() as libc::c_ulong)
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        __bswap_32((*hdr).data.module.msg.len) as libc::c_ulong,
                    ),
            ) as uint32_t as uint32_t;
    } else {
        explen = totlen;
    }
    if totlen != explen {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Received invalid %s packet of length %lld but expected length %lld\0"
                    as *const u8 as *const libc::c_char,
                clusterGetMessageTypeString(type_0 as libc::c_int),
                totlen as libc::c_ulonglong,
                explen as libc::c_ulonglong,
            );
        }
        return 1 as libc::c_int;
    }
    sender = getNodeFromLinkAndMsg(link, hdr);
    if !sender.is_null() {
        (*sender).data_received = now;
    }
    if !sender.is_null() && (*sender).flags & 32 as libc::c_int == 0 {
        senderCurrentEpoch = intrev64((*hdr).currentEpoch);
        senderConfigEpoch = intrev64((*hdr).configEpoch);
        if senderCurrentEpoch > (*server.cluster).currentEpoch {
            (*server.cluster).currentEpoch = senderCurrentEpoch;
        }
        if senderConfigEpoch > (*sender).configEpoch {
            (*sender).configEpoch = senderConfigEpoch;
            clusterDoBeforeSleep(
                (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int,
            );
        }
        (*sender).repl_offset = intrev64((*hdr).offset) as libc::c_longlong;
        (*sender).repl_offset_time = now;
        if (*server.cluster).mf_end != 0 && (*myself).flags & 2 as libc::c_int != 0
            && (*myself).slaveof == sender
            && (*hdr).mflags[0 as libc::c_int as usize] as libc::c_int
                & (1 as libc::c_int) << 0 as libc::c_int != 0
            && (*server.cluster).mf_master_offset
                == -(1 as libc::c_int) as libc::c_longlong
        {
            (*server.cluster).mf_master_offset = (*sender).repl_offset;
            clusterDoBeforeSleep((1 as libc::c_int) << 4 as libc::c_int);
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Received replication offset for paused master manual failover: %lld\0"
                        as *const u8 as *const libc::c_char,
                    (*server.cluster).mf_master_offset,
                );
            }
        }
    }
    if type_0 as libc::c_int == 0 as libc::c_int
        || type_0 as libc::c_int == 2 as libc::c_int
    {
        if (type_0 as libc::c_int == 2 as libc::c_int
            || (*myself).ip[0 as libc::c_int as usize] as libc::c_int == '\0' as i32)
            && (server.cluster_announce_ip).is_null()
        {
            let mut ip: [libc::c_char; 46] = [0; 46];
            if connSockName(
                (*link).conn,
                ip.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                0 as *mut libc::c_int,
            ) != -(1 as libc::c_int)
                && strcmp(ip.as_mut_ptr(), ((*myself).ip).as_mut_ptr()) != 0
            {
                memcpy(
                    ((*myself).ip).as_mut_ptr() as *mut libc::c_void,
                    ip.as_mut_ptr() as *const libc::c_void,
                    46 as libc::c_int as libc::c_ulong,
                );
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"IP address for this node updated to %s\0" as *const u8
                            as *const libc::c_char,
                        ((*myself).ip).as_mut_ptr(),
                    );
                }
                clusterDoBeforeSleep((1 as libc::c_int) << 2 as libc::c_int);
            }
        }
        if sender.is_null() && type_0 as libc::c_int == 2 as libc::c_int {
            let mut node: *mut clusterNode = 0 as *mut clusterNode;
            node = createClusterNode(0 as *mut libc::c_char, 32 as libc::c_int);
            if nodeIp2String(((*node).ip).as_mut_ptr(), link, ((*hdr).myip).as_mut_ptr())
                == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"nodeIp2String(node->ip,link,hdr->myip) == C_OK\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    2269 as libc::c_int,
                );
                unreachable!();
            };
            (*node).port = __bswap_16((*hdr).port) as libc::c_int;
            (*node).pport = __bswap_16((*hdr).pport) as libc::c_int;
            (*node).cport = __bswap_16((*hdr).cport) as libc::c_int;
            clusterAddNode(node);
            clusterDoBeforeSleep((1 as libc::c_int) << 2 as libc::c_int);
        }
        if sender.is_null() && type_0 as libc::c_int == 2 as libc::c_int {
            clusterProcessGossipSection(hdr, link);
        }
        clusterSendPing(link, 1 as libc::c_int);
    }
    if type_0 as libc::c_int == 0 as libc::c_int
        || type_0 as libc::c_int == 1 as libc::c_int
        || type_0 as libc::c_int == 2 as libc::c_int
    {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"%s packet received: %.40s\0" as *const u8 as *const libc::c_char,
                clusterGetMessageTypeString(type_0 as libc::c_int),
                if !((*link).node).is_null() {
                    ((*(*link).node).name).as_mut_ptr() as *const libc::c_char
                } else {
                    b"NULL\0" as *const u8 as *const libc::c_char
                },
            );
        }
        if (*link).inbound == 0 {
            if (*(*link).node).flags & 32 as libc::c_int != 0 {
                if !sender.is_null() {
                    if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            1 as libc::c_int,
                            b"Handshake: we already know node %.40s, updating the address if needed.\0"
                                as *const u8 as *const libc::c_char,
                            ((*sender).name).as_mut_ptr(),
                        );
                    }
                    if nodeUpdateAddressIfNeeded(sender, link, hdr) != 0 {
                        clusterDoBeforeSleep(
                            (1 as libc::c_int) << 2 as libc::c_int
                                | (1 as libc::c_int) << 1 as libc::c_int,
                        );
                    }
                    clusterDelNode((*link).node);
                    return 0 as libc::c_int;
                }
                clusterRenameNode((*link).node, ((*hdr).sender).as_mut_ptr());
                if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        0 as libc::c_int,
                        b"Handshake with node %.40s completed.\0" as *const u8
                            as *const libc::c_char,
                        ((*(*link).node).name).as_mut_ptr(),
                    );
                }
                (*(*link).node).flags &= !(32 as libc::c_int);
                (*(*link).node).flags
                    |= flags as libc::c_int & (1 as libc::c_int | 2 as libc::c_int);
                clusterDoBeforeSleep((1 as libc::c_int) << 2 as libc::c_int);
            } else if memcmp(
                ((*(*link).node).name).as_mut_ptr() as *const libc::c_void,
                ((*hdr).sender).as_mut_ptr() as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
            ) != 0 as libc::c_int
            {
                if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        0 as libc::c_int,
                        b"PONG contains mismatching sender ID. About node %.40s added %d ms ago, having flags %d\0"
                            as *const u8 as *const libc::c_char,
                        ((*(*link).node).name).as_mut_ptr(),
                        (now - (*(*link).node).ctime) as libc::c_int,
                        (*(*link).node).flags,
                    );
                }
                (*(*link).node).flags |= 64 as libc::c_int;
                (*(*link).node)
                    .ip[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                (*(*link).node).port = 0 as libc::c_int;
                (*(*link).node).pport = 0 as libc::c_int;
                (*(*link).node).cport = 0 as libc::c_int;
                freeClusterLink(link);
                clusterDoBeforeSleep((1 as libc::c_int) << 2 as libc::c_int);
                return 0 as libc::c_int;
            }
        }
        if !sender.is_null() {
            let mut nofailover: libc::c_int = flags as libc::c_int & 512 as libc::c_int;
            (*sender).flags &= !(512 as libc::c_int);
            (*sender).flags |= nofailover;
        }
        if !sender.is_null() && type_0 as libc::c_int == 0 as libc::c_int
            && (*sender).flags & 32 as libc::c_int == 0
            && nodeUpdateAddressIfNeeded(sender, link, hdr) != 0
        {
            clusterDoBeforeSleep(
                (1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 1 as libc::c_int,
            );
        }
        if (*link).inbound == 0 && type_0 as libc::c_int == 1 as libc::c_int {
            (*(*link).node).pong_received = now;
            (*(*link).node).ping_sent = 0 as libc::c_int as mstime_t;
            if (*(*link).node).flags & 4 as libc::c_int != 0 {
                (*(*link).node).flags &= !(4 as libc::c_int);
                clusterDoBeforeSleep(
                    (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int,
                );
            } else if (*(*link).node).flags & 8 as libc::c_int != 0 {
                clearNodeFailureIfNeeded((*link).node);
            }
        }
        if !sender.is_null() {
            if memcmp(
                ((*hdr).slaveof).as_mut_ptr() as *const libc::c_void,
                b"\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"
                    as *const u8 as *const libc::c_char as *const libc::c_void,
                core::mem::size_of::<[libc::c_char; 40]>() as libc::c_ulong,
            ) == 0
            {
                clusterSetNodeAsMaster(sender);
            } else {
                let mut master: *mut clusterNode = clusterLookupNode(
                    ((*hdr).slaveof).as_mut_ptr(),
                    40 as libc::c_int,
                );
                if (*sender).flags & 1 as libc::c_int != 0 {
                    clusterDelNodeSlots(sender);
                    (*sender).flags &= !(1 as libc::c_int | 256 as libc::c_int);
                    (*sender).flags |= 2 as libc::c_int;
                    clusterDoBeforeSleep(
                        (1 as libc::c_int) << 2 as libc::c_int
                            | (1 as libc::c_int) << 1 as libc::c_int,
                    );
                }
                if !master.is_null() && (*sender).slaveof != master {
                    if !((*sender).slaveof).is_null() {
                        clusterNodeRemoveSlave((*sender).slaveof, sender);
                    }
                    clusterNodeAddSlave(master, sender);
                    (*sender).slaveof = master;
                    clusterDoBeforeSleep((1 as libc::c_int) << 2 as libc::c_int);
                }
            }
        }
        let mut sender_master: *mut clusterNode = 0 as *mut clusterNode;
        let mut dirty_slots: libc::c_int = 0 as libc::c_int;
        if !sender.is_null() {
            sender_master = if (*sender).flags & 1 as libc::c_int != 0 {
                sender
            } else {
                (*sender).slaveof
            };
            if !sender_master.is_null() {
                dirty_slots = (memcmp(
                    ((*sender_master).slots).as_mut_ptr() as *const libc::c_void,
                    ((*hdr).myslots).as_mut_ptr() as *const libc::c_void,
                    core::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong,
                ) != 0 as libc::c_int) as libc::c_int;
            }
        }
        if !sender.is_null() && (*sender).flags & 1 as libc::c_int != 0
            && dirty_slots != 0
        {
            clusterUpdateSlotsConfigWith(
                sender,
                senderConfigEpoch,
                ((*hdr).myslots).as_mut_ptr(),
            );
        }
        if !sender.is_null() && dirty_slots != 0 {
            let mut j: libc::c_int = 0;
            j = 0 as libc::c_int;
            while j < 16384 as libc::c_int {
                if bitmapTestBit(((*hdr).myslots).as_mut_ptr(), j) != 0 {
                    if !((*server.cluster).slots[j as usize] == sender
                        || ((*server.cluster).slots[j as usize]).is_null())
                    {
                        if (*(*server.cluster).slots[j as usize]).configEpoch
                            > senderConfigEpoch
                        {
                            if !((1 as libc::c_int & 0xff as libc::c_int)
                                < server.verbosity)
                            {
                                _serverLog(
                                    1 as libc::c_int,
                                    b"Node %.40s has old slots configuration, sending an UPDATE message about %.40s\0"
                                        as *const u8 as *const libc::c_char,
                                    ((*sender).name).as_mut_ptr(),
                                    ((*(*server.cluster).slots[j as usize]).name).as_mut_ptr(),
                                );
                            }
                            clusterSendUpdate(
                                (*sender).link,
                                (*server.cluster).slots[j as usize],
                            );
                            break;
                        }
                    }
                }
                j += 1;
            }
        }
        if !sender.is_null() && (*myself).flags & 1 as libc::c_int != 0
            && (*sender).flags & 1 as libc::c_int != 0
            && senderConfigEpoch == (*myself).configEpoch
        {
            clusterHandleConfigEpochCollision(sender);
        }
        if !sender.is_null() {
            clusterProcessGossipSection(hdr, link);
            clusterProcessPingExtensions(hdr, link);
        }
    } else if type_0 as libc::c_int == 3 as libc::c_int {
        let mut failing: *mut clusterNode = 0 as *mut clusterNode;
        if !sender.is_null() {
            failing = clusterLookupNode(
                ((*hdr).data.fail.about.nodename).as_mut_ptr(),
                40 as libc::c_int,
            );
            if !failing.is_null()
                && (*failing).flags & (8 as libc::c_int | 16 as libc::c_int) == 0
            {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"FAIL message received from %.40s about %.40s\0" as *const u8
                            as *const libc::c_char,
                        ((*hdr).sender).as_mut_ptr(),
                        ((*hdr).data.fail.about.nodename).as_mut_ptr(),
                    );
                }
                (*failing).flags |= 8 as libc::c_int;
                (*failing).fail_time = now;
                (*failing).flags &= !(4 as libc::c_int);
                clusterDoBeforeSleep(
                    (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int,
                );
            }
        } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Ignoring FAIL message from unknown node %.40s about %.40s\0"
                    as *const u8 as *const libc::c_char,
                ((*hdr).sender).as_mut_ptr(),
                ((*hdr).data.fail.about.nodename).as_mut_ptr(),
            );
        }
    } else if type_0 as libc::c_int == 4 as libc::c_int
        || type_0 as libc::c_int == 10 as libc::c_int
    {
        if sender.is_null() {
            return 1 as libc::c_int;
        }
        let mut channel: *mut robj = 0 as *mut robj;
        let mut message: *mut robj = 0 as *mut robj;
        let mut channel_len: uint32_t = 0;
        let mut message_len: uint32_t = 0;
        if type_0 as libc::c_int == 4 as libc::c_int
            && serverPubsubSubscriptionCount() > 0 as libc::c_int
            || type_0 as libc::c_int == 10 as libc::c_int
                && serverPubsubShardSubscriptionCount() > 0 as libc::c_int
        {
            channel_len = __bswap_32((*hdr).data.publish.msg.channel_len);
            message_len = __bswap_32((*hdr).data.publish.msg.message_len);
            channel = createStringObject(
                ((*hdr).data.publish.msg.bulk_data).as_mut_ptr() as *mut libc::c_char,
                channel_len as size_t,
            );
            message = createStringObject(
                (((*hdr).data.publish.msg.bulk_data).as_mut_ptr() as *mut libc::c_char)
                    .offset(channel_len as isize),
                message_len as size_t,
            );
            pubsubPublishMessage(
                channel,
                message,
                (type_0 as libc::c_int == 10 as libc::c_int) as libc::c_int,
            );
            decrRefCount(channel);
            decrRefCount(message);
        }
    } else if type_0 as libc::c_int == 5 as libc::c_int {
        if sender.is_null() {
            return 1 as libc::c_int;
        }
        clusterSendFailoverAuthIfNeeded(sender, hdr);
    } else if type_0 as libc::c_int == 6 as libc::c_int {
        if sender.is_null() {
            return 1 as libc::c_int;
        }
        if (*sender).flags & 1 as libc::c_int != 0
            && (*sender).numslots > 0 as libc::c_int
            && senderCurrentEpoch >= (*server.cluster).failover_auth_epoch
        {
            (*server.cluster).failover_auth_count += 1;
            clusterDoBeforeSleep((1 as libc::c_int) << 0 as libc::c_int);
        }
    } else if type_0 as libc::c_int == 8 as libc::c_int {
        if sender.is_null() || (*sender).slaveof != myself {
            return 1 as libc::c_int;
        }
        resetManualFailover();
        (*server.cluster).mf_end = now + 5000 as libc::c_int as libc::c_longlong;
        (*server.cluster).mf_slave = sender;
        pauseClients(
            PAUSE_DURING_FAILOVER,
            now + (5000 as libc::c_int * 2 as libc::c_int) as libc::c_longlong,
            CLIENT_PAUSE_WRITE,
        );
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Manual failover requested by replica %.40s.\0" as *const u8
                    as *const libc::c_char,
                ((*sender).name).as_mut_ptr(),
            );
        }
        clusterSendPing(link, 0 as libc::c_int);
    } else if type_0 as libc::c_int == 7 as libc::c_int {
        let mut n: *mut clusterNode = 0 as *mut clusterNode;
        let mut reportedConfigEpoch: uint64_t = intrev64(
            (*hdr).data.update.nodecfg.configEpoch,
        );
        if sender.is_null() {
            return 1 as libc::c_int;
        }
        n = clusterLookupNode(
            ((*hdr).data.update.nodecfg.nodename).as_mut_ptr(),
            40 as libc::c_int,
        );
        if n.is_null() {
            return 1 as libc::c_int;
        }
        if (*n).configEpoch >= reportedConfigEpoch {
            return 1 as libc::c_int;
        }
        if (*n).flags & 2 as libc::c_int != 0 {
            clusterSetNodeAsMaster(n);
        }
        (*n).configEpoch = reportedConfigEpoch;
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
        clusterUpdateSlotsConfigWith(
            n,
            reportedConfigEpoch,
            ((*hdr).data.update.nodecfg.slots).as_mut_ptr(),
        );
    } else if type_0 as libc::c_int == 9 as libc::c_int {
        if sender.is_null() {
            return 1 as libc::c_int;
        }
        let mut module_id: uint64_t = (*hdr).data.module.msg.module_id;
        let mut len: uint32_t = __bswap_32((*hdr).data.module.msg.len);
        let mut type_1: uint8_t = (*hdr).data.module.msg.type_0;
        let mut payload: *mut libc::c_uchar = ((*hdr).data.module.msg.bulk_data)
            .as_mut_ptr();
        moduleCallClusterReceivers(
            ((*sender).name).as_mut_ptr(),
            module_id,
            type_1,
            payload,
            len,
        );
    } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Received unknown packet type: %d\0" as *const u8 as *const libc::c_char,
            type_0 as libc::c_int,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn handleLinkIOError(mut link: *mut clusterLink) {
    freeClusterLink(link);
}
#[no_mangle]
pub unsafe extern "C" fn clusterWriteHandler(mut conn: *mut connection) {
    let mut link: *mut clusterLink = connGetPrivateData(conn) as *mut clusterLink;
    let mut nwritten: ssize_t = 0;
    nwritten = connWrite(
        conn,
        (*link).sndbuf as *const libc::c_void,
        sdslen((*link).sndbuf),
    ) as ssize_t;
    if nwritten <= 0 as libc::c_int as libc::c_long {
        if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                0 as libc::c_int,
                b"I/O error writing to node link: %s\0" as *const u8
                    as *const libc::c_char,
                if nwritten == -(1 as libc::c_int) as libc::c_long {
                    connGetLastError(conn)
                } else {
                    b"short write\0" as *const u8 as *const libc::c_char
                },
            );
        }
        handleLinkIOError(link);
        return;
    }
    sdsrange((*link).sndbuf, nwritten, -(1 as libc::c_int) as ssize_t);
    if sdslen((*link).sndbuf) == 0 as libc::c_int as libc::c_ulong {
        connSetWriteHandler((*link).conn, None);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterLinkConnectHandler(mut conn: *mut connection) {
    let mut link: *mut clusterLink = connGetPrivateData(conn) as *mut clusterLink;
    let mut node: *mut clusterNode = (*link).node;
    if connGetState(conn) != CONN_STATE_CONNECTED as libc::c_int {
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Connection with Node %.40s at %s:%d failed: %s\0" as *const u8
                    as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
                ((*node).ip).as_mut_ptr(),
                (*node).cport,
                connGetLastError(conn),
            );
        }
        freeClusterLink(link);
        return;
    }
    connSetReadHandler(
        conn,
        Some(clusterReadHandler as unsafe extern "C" fn(*mut connection) -> ()),
    );
    let mut old_ping_sent: mstime_t = (*node).ping_sent;
    clusterSendPing(
        link,
        if (*node).flags & 128 as libc::c_int != 0 {
            2 as libc::c_int
        } else {
            0 as libc::c_int
        },
    );
    if old_ping_sent != 0 {
        (*node).ping_sent = old_ping_sent;
    }
    (*node).flags &= !(128 as libc::c_int);
    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            0 as libc::c_int,
            b"Connecting with Node %.40s at %s:%d\0" as *const u8 as *const libc::c_char,
            ((*node).name).as_mut_ptr(),
            ((*node).ip).as_mut_ptr(),
            (*node).cport,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterReadHandler(mut conn: *mut connection) {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut nread: ssize_t = 0;
    let mut hdr: *mut clusterMsg = 0 as *mut clusterMsg;
    let mut link: *mut clusterLink = connGetPrivateData(conn) as *mut clusterLink;
    let mut readlen: libc::c_uint = 0;
    let mut rcvbuflen: libc::c_uint = 0;
    loop {
        rcvbuflen = (*link).rcvbuf_len as libc::c_uint;
        if rcvbuflen < 8 as libc::c_int as libc::c_uint {
            readlen = (8 as libc::c_int as libc::c_uint).wrapping_sub(rcvbuflen);
        } else {
            hdr = (*link).rcvbuf as *mut clusterMsg;
            if rcvbuflen == 8 as libc::c_int as libc::c_uint {
                if memcmp(
                    ((*hdr).sig).as_mut_ptr() as *const libc::c_void,
                    b"RCmb\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    4 as libc::c_int as libc::c_ulong,
                ) != 0 as libc::c_int
                    || (__bswap_32((*hdr).totlen) as libc::c_ulong)
                        < (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
                            .wrapping_sub(
                                core::mem::size_of::<clusterMsgData>() as libc::c_ulong,
                            )
                {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Bad message length or signature received from Cluster bus.\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                    handleLinkIOError(link);
                    return;
                }
            }
            readlen = (__bswap_32((*hdr).totlen)).wrapping_sub(rcvbuflen);
            if readlen as libc::c_ulong
                > core::mem::size_of::<[clusterMsg; 1]>() as libc::c_ulong
            {
                readlen = core::mem::size_of::<[clusterMsg; 1]>() as libc::c_ulong
                    as libc::c_uint;
            }
        }
        nread = connRead(conn, buf.as_mut_ptr() as *mut libc::c_void, readlen as size_t)
            as ssize_t;
        if nread == -(1 as libc::c_int) as libc::c_long
            && connGetState(conn) == CONN_STATE_CONNECTED as libc::c_int
        {
            return;
        }
        if nread <= 0 as libc::c_int as libc::c_long {
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"I/O error reading from node link: %s\0" as *const u8
                        as *const libc::c_char,
                    if nread == 0 as libc::c_int as libc::c_long {
                        b"connection closed\0" as *const u8 as *const libc::c_char
                    } else {
                        connGetLastError(conn)
                    },
                );
            }
            handleLinkIOError(link);
            return;
        } else {
            let mut unused: size_t = ((*link).rcvbuf_alloc)
                .wrapping_sub((*link).rcvbuf_len);
            if nread as size_t > unused {
                let mut required: size_t = ((*link).rcvbuf_len)
                    .wrapping_add(nread as libc::c_ulong);
                (*link)
                    .rcvbuf_alloc = if required
                    < ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
                {
                    required.wrapping_mul(2 as libc::c_int as libc::c_ulong)
                } else {
                    required
                        .wrapping_add(
                            ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong,
                        )
                };
                (*link)
                    .rcvbuf = zrealloc(
                    (*link).rcvbuf as *mut libc::c_void,
                    (*link).rcvbuf_alloc,
                ) as *mut libc::c_char;
            }
            memcpy(
                ((*link).rcvbuf).offset((*link).rcvbuf_len as isize)
                    as *mut libc::c_void,
                buf.as_mut_ptr() as *const libc::c_void,
                nread as libc::c_ulong,
            );
            (*link)
                .rcvbuf_len = ((*link).rcvbuf_len as libc::c_ulong)
                .wrapping_add(nread as libc::c_ulong) as size_t as size_t;
            hdr = (*link).rcvbuf as *mut clusterMsg;
            rcvbuflen = (rcvbuflen as libc::c_long + nread) as libc::c_uint;
        }
        if rcvbuflen >= 8 as libc::c_int as libc::c_uint
            && rcvbuflen == __bswap_32((*hdr).totlen)
        {
            if clusterProcessPacket(link) != 0 {
                if (*link).rcvbuf_alloc > 1024 as libc::c_int as libc::c_ulong {
                    zfree((*link).rcvbuf as *mut libc::c_void);
                    (*link).rcvbuf_alloc = 1024 as libc::c_int as size_t;
                    (*link).rcvbuf = zmalloc((*link).rcvbuf_alloc) as *mut libc::c_char;
                }
                (*link).rcvbuf_len = 0 as libc::c_int as size_t;
            } else {
                return
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendMessage(
    mut link: *mut clusterLink,
    mut msg: *mut libc::c_uchar,
    mut msglen: size_t,
) {
    if sdslen((*link).sndbuf) == 0 as libc::c_int as libc::c_ulong
        && msglen != 0 as libc::c_int as libc::c_ulong
    {
        connSetWriteHandlerWithBarrier(
            (*link).conn,
            Some(clusterWriteHandler as unsafe extern "C" fn(*mut connection) -> ()),
            1 as libc::c_int,
        );
    }
    (*link).sndbuf = sdscatlen((*link).sndbuf, msg as *const libc::c_void, msglen);
    let mut hdr: *mut clusterMsg = msg as *mut clusterMsg;
    let mut type_0: uint16_t = __bswap_16((*hdr).type_0);
    if (type_0 as libc::c_int) < 11 as libc::c_int {
        (*server.cluster).stats_bus_messages_sent[type_0 as usize] += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterBroadcastMessage(
    mut buf: *mut libc::c_void,
    mut len: size_t,
) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if ((*node).link).is_null() {
            continue;
        }
        if (*node).flags & (16 as libc::c_int | 32 as libc::c_int) != 0 {
            continue;
        }
        clusterSendMessage((*node).link, buf as *mut libc::c_uchar, len);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn clusterBuildMessageHdr(
    mut hdr: *mut clusterMsg,
    mut type_0: libc::c_int,
) {
    let mut totlen: libc::c_int = 0 as libc::c_int;
    let mut offset: uint64_t = 0;
    let mut master: *mut clusterNode = 0 as *mut clusterNode;
    master = if (*myself).flags & 2 as libc::c_int != 0 && !((*myself).slaveof).is_null()
    {
        (*myself).slaveof
    } else {
        myself
    };
    memset(
        hdr as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<clusterMsg>() as libc::c_ulong,
    );
    (*hdr).ver = __bswap_16(1 as libc::c_int as __uint16_t);
    (*hdr).sig[0 as libc::c_int as usize] = 'R' as i32 as libc::c_char;
    (*hdr).sig[1 as libc::c_int as usize] = 'C' as i32 as libc::c_char;
    (*hdr).sig[2 as libc::c_int as usize] = 'm' as i32 as libc::c_char;
    (*hdr).sig[3 as libc::c_int as usize] = 'b' as i32 as libc::c_char;
    (*hdr).type_0 = __bswap_16(type_0 as __uint16_t);
    memcpy(
        ((*hdr).sender).as_mut_ptr() as *mut libc::c_void,
        ((*myself).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    );
    memset(
        ((*hdr).myip).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        46 as libc::c_int as libc::c_ulong,
    );
    if !(server.cluster_announce_ip).is_null() {
        strncpy(
            ((*hdr).myip).as_mut_ptr(),
            server.cluster_announce_ip,
            (46 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
        );
        (*hdr)
            .myip[(46 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
    }
    let mut announced_port: libc::c_int = 0;
    let mut announced_pport: libc::c_int = 0;
    let mut announced_cport: libc::c_int = 0;
    deriveAnnouncedPorts(
        &mut announced_port,
        &mut announced_pport,
        &mut announced_cport,
    );
    memcpy(
        ((*hdr).myslots).as_mut_ptr() as *mut libc::c_void,
        ((*master).slots).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong,
    );
    memset(
        ((*hdr).slaveof).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        40 as libc::c_int as libc::c_ulong,
    );
    if !((*myself).slaveof).is_null() {
        memcpy(
            ((*hdr).slaveof).as_mut_ptr() as *mut libc::c_void,
            ((*(*myself).slaveof).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
    }
    (*hdr).port = __bswap_16(announced_port as __uint16_t);
    (*hdr).pport = __bswap_16(announced_pport as __uint16_t);
    (*hdr).cport = __bswap_16(announced_cport as __uint16_t);
    (*hdr).flags = __bswap_16((*myself).flags as __uint16_t);
    (*hdr).state = (*server.cluster).state as libc::c_uchar;
    (*hdr).currentEpoch = intrev64((*server.cluster).currentEpoch);
    (*hdr).configEpoch = intrev64((*master).configEpoch);
    if (*myself).flags & 2 as libc::c_int != 0 {
        offset = replicationGetSlaveOffset() as uint64_t;
    } else {
        offset = server.master_repl_offset as uint64_t;
    }
    (*hdr).offset = intrev64(offset);
    if (*myself).flags & 1 as libc::c_int != 0 && (*server.cluster).mf_end != 0 {
        (*hdr)
            .mflags[0 as libc::c_int
            as usize] = ((*hdr).mflags[0 as libc::c_int as usize] as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_uchar;
    }
    if type_0 == 3 as libc::c_int {
        totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as libc::c_int;
        totlen = (totlen as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<clusterMsgDataFail>() as libc::c_ulong)
            as libc::c_int as libc::c_int;
    } else if type_0 == 7 as libc::c_int {
        totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
            .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
            as libc::c_int;
        totlen = (totlen as libc::c_ulong)
            .wrapping_add(
                core::mem::size_of::<clusterMsgDataUpdate>() as libc::c_ulong,
            ) as libc::c_int as libc::c_int;
    }
    (*hdr).totlen = __bswap_32(totlen as __uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn clusterSetGossipEntry(
    mut hdr: *mut clusterMsg,
    mut i: libc::c_int,
    mut n: *mut clusterNode,
) {
    let mut gossip: *mut clusterMsgDataGossip = 0 as *mut clusterMsgDataGossip;
    gossip = &mut *((*hdr).data.ping.gossip).as_mut_ptr().offset(i as isize)
        as *mut clusterMsgDataGossip;
    memcpy(
        ((*gossip).nodename).as_mut_ptr() as *mut libc::c_void,
        ((*n).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    );
    (*gossip)
        .ping_sent = __bswap_32(
        ((*n).ping_sent / 1000 as libc::c_int as libc::c_longlong) as __uint32_t,
    );
    (*gossip)
        .pong_received = __bswap_32(
        ((*n).pong_received / 1000 as libc::c_int as libc::c_longlong) as __uint32_t,
    );
    memcpy(
        ((*gossip).ip).as_mut_ptr() as *mut libc::c_void,
        ((*n).ip).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
    );
    (*gossip).port = __bswap_16((*n).port as __uint16_t);
    (*gossip).cport = __bswap_16((*n).cport as __uint16_t);
    (*gossip).flags = __bswap_16((*n).flags as __uint16_t);
    (*gossip).pport = __bswap_16((*n).pport as __uint16_t);
    (*gossip).notused1 = 0 as libc::c_int as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendPing(
    mut link: *mut clusterLink,
    mut type_0: libc::c_int,
) {
    static mut cluster_pings_sent: libc::c_ulonglong = 0 as libc::c_int
        as libc::c_ulonglong;
    cluster_pings_sent = cluster_pings_sent.wrapping_add(1);
    let mut buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hdr: *mut clusterMsg = 0 as *mut clusterMsg;
    let mut gossipcount: libc::c_int = 0 as libc::c_int;
    let mut wanted: libc::c_int = 0;
    let mut estlen: libc::c_int = 0;
    let mut freshnodes: libc::c_int = ((*(*server.cluster).nodes)
        .ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize])
        .wrapping_sub(2 as libc::c_int as libc::c_ulong) as libc::c_int;
    wanted = floor(
        ((*(*server.cluster).nodes).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize])
            .wrapping_div(10 as libc::c_int as libc::c_ulong) as libc::c_double,
    ) as libc::c_int;
    if wanted < 3 as libc::c_int {
        wanted = 3 as libc::c_int;
    }
    if wanted > freshnodes {
        wanted = freshnodes;
    }
    let mut pfail_wanted: libc::c_int = (*server.cluster).stats_pfail_nodes
        as libc::c_int;
    estlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as libc::c_int;
    estlen = (estlen as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<clusterMsgDataGossip>() as libc::c_ulong)
                .wrapping_mul((wanted + pfail_wanted) as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    estlen += getHostnamePingExtSize();
    if estlen < core::mem::size_of::<clusterMsg>() as libc::c_ulong as libc::c_int {
        estlen = core::mem::size_of::<clusterMsg>() as libc::c_ulong as libc::c_int;
    }
    buf = zcalloc(estlen as size_t) as *mut libc::c_uchar;
    hdr = buf as *mut clusterMsg;
    if (*link).inbound == 0 && type_0 == 0 as libc::c_int {
        (*(*link).node).ping_sent = mstime();
    }
    clusterBuildMessageHdr(hdr, type_0);
    let mut maxiterations: libc::c_int = wanted * 3 as libc::c_int;
    while freshnodes > 0 as libc::c_int && gossipcount < wanted
        && {
            let fresh7 = maxiterations;
            maxiterations = maxiterations - 1;
            fresh7 != 0
        }
    {
        let mut de: *mut dictEntry = dictGetRandomKey((*server.cluster).nodes);
        let mut this: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if this == myself {
            continue;
        }
        if (*this).flags & 4 as libc::c_int != 0 {
            continue;
        }
        if (*this).flags & (32 as libc::c_int | 64 as libc::c_int) != 0
            || ((*this).link).is_null() && (*this).numslots == 0 as libc::c_int
        {
            freshnodes -= 1;
        } else {
            if (*this).last_in_ping_gossip == cluster_pings_sent {
                continue;
            }
            clusterSetGossipEntry(hdr, gossipcount, this);
            (*this).last_in_ping_gossip = cluster_pings_sent;
            freshnodes -= 1;
            gossipcount += 1;
        }
    }
    if pfail_wanted != 0 {
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de_0: *mut dictEntry = 0 as *mut dictEntry;
        di = dictGetSafeIterator((*server.cluster).nodes);
        loop {
            de_0 = dictNext(di);
            if !(!de_0.is_null() && pfail_wanted > 0 as libc::c_int) {
                break;
            }
            let mut node: *mut clusterNode = (*de_0).v.val as *mut clusterNode;
            if (*node).flags & 32 as libc::c_int != 0 {
                continue;
            }
            if (*node).flags & 64 as libc::c_int != 0 {
                continue;
            }
            if (*node).flags & 4 as libc::c_int == 0 {
                continue;
            }
            clusterSetGossipEntry(hdr, gossipcount, node);
            gossipcount += 1;
            pfail_wanted -= 1;
        }
        dictReleaseIterator(di);
    }
    let mut totlen: libc::c_int = 0 as libc::c_int;
    let mut extensions: libc::c_int = 0 as libc::c_int;
    let mut cursor: *mut clusterMsgPingExt = getInitialPingExt(
        hdr,
        gossipcount as uint16_t,
    );
    if sdslen((*myself).hostname) != 0 as libc::c_int as libc::c_ulong {
        (*hdr)
            .mflags[0 as libc::c_int
            as usize] = ((*hdr).mflags[0 as libc::c_int as usize] as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_uchar;
        totlen += writeHostnamePingExt(&mut cursor);
        extensions += 1;
    }
    totlen = (totlen as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
                .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    totlen = (totlen as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<clusterMsgDataGossip>() as libc::c_ulong)
                .wrapping_mul(gossipcount as libc::c_ulong),
        ) as libc::c_int as libc::c_int;
    (*hdr).count = __bswap_16(gossipcount as __uint16_t);
    (*hdr).extensions = __bswap_16(extensions as __uint16_t);
    (*hdr).totlen = __bswap_32(totlen as __uint32_t);
    clusterSendMessage(link, buf, totlen as size_t);
    zfree(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn clusterBroadcastPong(mut target: libc::c_int) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if ((*node).link).is_null() {
            continue;
        }
        if node == myself || (*node).flags & 32 as libc::c_int != 0 {
            continue;
        }
        if target == 1 as libc::c_int {
            let mut local_slave: libc::c_int = ((*node).flags & 2 as libc::c_int != 0
                && !((*node).slaveof).is_null()
                && ((*node).slaveof == myself || (*node).slaveof == (*myself).slaveof))
                as libc::c_int;
            if local_slave == 0 {
                continue;
            }
        }
        clusterSendPing((*node).link, 1 as libc::c_int);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendPublish(
    mut link: *mut clusterLink,
    mut channel: *mut robj,
    mut message: *mut robj,
    mut type_0: uint16_t,
) {
    let mut payload: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    let mut totlen: uint32_t = 0;
    let mut channel_len: uint32_t = 0;
    let mut message_len: uint32_t = 0;
    channel = getDecodedObject(channel);
    message = getDecodedObject(message);
    channel_len = sdslen((*channel).ptr as sds) as uint32_t;
    message_len = sdslen((*message).ptr as sds) as uint32_t;
    clusterBuildMessageHdr(hdr, type_0 as libc::c_int);
    totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as uint32_t;
    totlen = (totlen as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<clusterMsgDataPublish>() as libc::c_ulong)
                .wrapping_sub(8 as libc::c_int as libc::c_ulong)
                .wrapping_add(channel_len as libc::c_ulong)
                .wrapping_add(message_len as libc::c_ulong),
        ) as uint32_t as uint32_t;
    (*hdr).data.publish.msg.channel_len = __bswap_32(channel_len);
    (*hdr).data.publish.msg.message_len = __bswap_32(message_len);
    (*hdr).totlen = __bswap_32(totlen);
    if (totlen as libc::c_ulong)
        < core::mem::size_of::<[clusterMsg; 1]>() as libc::c_ulong
    {
        payload = buf.as_mut_ptr() as *mut libc::c_uchar;
    } else {
        payload = zmalloc(totlen as size_t) as *mut libc::c_uchar;
        memcpy(
            payload as *mut libc::c_void,
            hdr as *const libc::c_void,
            core::mem::size_of::<clusterMsg>() as libc::c_ulong,
        );
        hdr = payload as *mut clusterMsg;
    }
    memcpy(
        ((*hdr).data.publish.msg.bulk_data).as_mut_ptr() as *mut libc::c_void,
        (*channel).ptr,
        sdslen((*channel).ptr as sds),
    );
    memcpy(
        ((*hdr).data.publish.msg.bulk_data)
            .as_mut_ptr()
            .offset(sdslen((*channel).ptr as sds) as isize) as *mut libc::c_void,
        (*message).ptr,
        sdslen((*message).ptr as sds),
    );
    if !link.is_null() {
        clusterSendMessage(link, payload, totlen as size_t);
    } else {
        clusterBroadcastMessage(payload as *mut libc::c_void, totlen as size_t);
    }
    decrRefCount(channel);
    decrRefCount(message);
    if payload != buf.as_mut_ptr() as *mut libc::c_uchar {
        zfree(payload as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendFail(mut nodename: *mut libc::c_char) {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    clusterBuildMessageHdr(hdr, 3 as libc::c_int);
    memcpy(
        ((*hdr).data.fail.about.nodename).as_mut_ptr() as *mut libc::c_void,
        nodename as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    );
    clusterBroadcastMessage(
        buf.as_mut_ptr() as *mut libc::c_void,
        __bswap_32((*hdr).totlen) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendUpdate(
    mut link: *mut clusterLink,
    mut node: *mut clusterNode,
) {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    if link.is_null() {
        return;
    }
    clusterBuildMessageHdr(hdr, 7 as libc::c_int);
    memcpy(
        ((*hdr).data.update.nodecfg.nodename).as_mut_ptr() as *mut libc::c_void,
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as libc::c_ulong,
    );
    (*hdr).data.update.nodecfg.configEpoch = intrev64((*node).configEpoch);
    memcpy(
        ((*hdr).data.update.nodecfg.slots).as_mut_ptr() as *mut libc::c_void,
        ((*node).slots).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_uchar; 2048]>() as libc::c_ulong,
    );
    clusterSendMessage(
        link,
        buf.as_mut_ptr() as *mut libc::c_uchar,
        __bswap_32((*hdr).totlen) as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendModule(
    mut link: *mut clusterLink,
    mut module_id: uint64_t,
    mut type_0: uint8_t,
    mut payload: *const libc::c_char,
    mut len: uint32_t,
) {
    let mut heapbuf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    let mut totlen: uint32_t = 0;
    clusterBuildMessageHdr(hdr, 9 as libc::c_int);
    totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as uint32_t;
    totlen = (totlen as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<clusterMsgModule>() as libc::c_ulong)
                .wrapping_sub(3 as libc::c_int as libc::c_ulong)
                .wrapping_add(len as libc::c_ulong),
        ) as uint32_t as uint32_t;
    (*hdr).data.module.msg.module_id = module_id;
    (*hdr).data.module.msg.type_0 = type_0;
    (*hdr).data.module.msg.len = __bswap_32(len);
    (*hdr).totlen = __bswap_32(totlen);
    if (totlen as libc::c_ulong)
        < core::mem::size_of::<[clusterMsg; 1]>() as libc::c_ulong
    {
        heapbuf = buf.as_mut_ptr() as *mut libc::c_uchar;
    } else {
        heapbuf = zmalloc(totlen as size_t) as *mut libc::c_uchar;
        memcpy(
            heapbuf as *mut libc::c_void,
            hdr as *const libc::c_void,
            core::mem::size_of::<clusterMsg>() as libc::c_ulong,
        );
        hdr = heapbuf as *mut clusterMsg;
    }
    memcpy(
        ((*hdr).data.module.msg.bulk_data).as_mut_ptr() as *mut libc::c_void,
        payload as *const libc::c_void,
        len as libc::c_ulong,
    );
    if !link.is_null() {
        clusterSendMessage(link, heapbuf, totlen as size_t);
    } else {
        clusterBroadcastMessage(heapbuf as *mut libc::c_void, totlen as size_t);
    }
    if heapbuf != buf.as_mut_ptr() as *mut libc::c_uchar {
        zfree(heapbuf as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendModuleMessageToTarget(
    mut target: *const libc::c_char,
    mut module_id: uint64_t,
    mut type_0: uint8_t,
    mut payload: *const libc::c_char,
    mut len: uint32_t,
) -> libc::c_int {
    let mut node: *mut clusterNode = 0 as *mut clusterNode;
    if !target.is_null() {
        node = clusterLookupNode(target, strlen(target) as libc::c_int);
        if node.is_null() || ((*node).link).is_null() {
            return -(1 as libc::c_int);
        }
    }
    clusterSendModule(
        if !target.is_null() { (*node).link } else { 0 as *mut clusterLink },
        module_id,
        type_0,
        payload,
        len,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterPropagatePublish(
    mut channel: *mut robj,
    mut message: *mut robj,
    mut sharded: libc::c_int,
) {
    if sharded == 0 {
        clusterSendPublish(
            0 as *mut clusterLink,
            channel,
            message,
            4 as libc::c_int as uint16_t,
        );
        return;
    }
    let mut nodes_for_slot: *mut list = clusterGetNodesServingMySlots(
        (*server.cluster).myself,
    );
    if (*nodes_for_slot).len != 0 as libc::c_int as libc::c_ulong {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(nodes_for_slot, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut node: *mut clusterNode = (*ln).value as *mut clusterNode;
            if node != myself {
                clusterSendPublish(
                    (*node).link,
                    channel,
                    message,
                    10 as libc::c_int as uint16_t,
                );
            }
        }
    }
    listRelease(nodes_for_slot);
}
#[no_mangle]
pub unsafe extern "C" fn clusterRequestFailoverAuth() {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    let mut totlen: uint32_t = 0;
    clusterBuildMessageHdr(hdr, 5 as libc::c_int);
    if (*server.cluster).mf_end != 0 {
        (*hdr)
            .mflags[0 as libc::c_int
            as usize] = ((*hdr).mflags[0 as libc::c_int as usize] as libc::c_int
            | (1 as libc::c_int) << 1 as libc::c_int) as libc::c_uchar;
    }
    totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as uint32_t;
    (*hdr).totlen = __bswap_32(totlen);
    clusterBroadcastMessage(buf.as_mut_ptr() as *mut libc::c_void, totlen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendFailoverAuth(mut node: *mut clusterNode) {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    let mut totlen: uint32_t = 0;
    if ((*node).link).is_null() {
        return;
    }
    clusterBuildMessageHdr(hdr, 6 as libc::c_int);
    totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as uint32_t;
    (*hdr).totlen = __bswap_32(totlen);
    clusterSendMessage(
        (*node).link,
        buf.as_mut_ptr() as *mut libc::c_uchar,
        totlen as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendMFStart(mut node: *mut clusterNode) {
    let mut buf: [clusterMsg; 1] = [clusterMsg {
        sig: [0; 4],
        totlen: 0,
        ver: 0,
        port: 0,
        type_0: 0,
        count: 0,
        currentEpoch: 0,
        configEpoch: 0,
        offset: 0,
        sender: [0; 40],
        myslots: [0; 2048],
        slaveof: [0; 40],
        myip: [0; 46],
        extensions: 0,
        notused1: [0; 30],
        pport: 0,
        cport: 0,
        flags: 0,
        state: 0,
        mflags: [0; 3],
        data: clusterMsgData {
            ping: C2RustUnnamed_19 {
                gossip: [clusterMsgDataGossip {
                    nodename: [0; 40],
                    ping_sent: 0,
                    pong_received: 0,
                    ip: [0; 46],
                    port: 0,
                    cport: 0,
                    flags: 0,
                    pport: 0,
                    notused1: 0,
                }; 1],
            },
        },
    }; 1];
    let mut hdr: *mut clusterMsg = buf.as_mut_ptr();
    let mut totlen: uint32_t = 0;
    if ((*node).link).is_null() {
        return;
    }
    clusterBuildMessageHdr(hdr, 8 as libc::c_int);
    totlen = (core::mem::size_of::<clusterMsg>() as libc::c_ulong)
        .wrapping_sub(core::mem::size_of::<clusterMsgData>() as libc::c_ulong)
        as uint32_t;
    (*hdr).totlen = __bswap_32(totlen);
    clusterSendMessage(
        (*node).link,
        buf.as_mut_ptr() as *mut libc::c_uchar,
        totlen as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterSendFailoverAuthIfNeeded(
    mut node: *mut clusterNode,
    mut request: *mut clusterMsg,
) {
    let mut master: *mut clusterNode = (*node).slaveof;
    let mut requestCurrentEpoch: uint64_t = intrev64((*request).currentEpoch);
    let mut requestConfigEpoch: uint64_t = intrev64((*request).configEpoch);
    let mut claimed_slots: *mut libc::c_uchar = ((*request).myslots).as_mut_ptr();
    let mut force_ack: libc::c_int = (*request).mflags[0 as libc::c_int as usize]
        as libc::c_int & (1 as libc::c_int) << 1 as libc::c_int;
    let mut j: libc::c_int = 0;
    if (*myself).flags & 2 as libc::c_int != 0 || (*myself).numslots == 0 as libc::c_int
    {
        return;
    }
    if requestCurrentEpoch < (*server.cluster).currentEpoch {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failover auth denied to %.40s: reqEpoch (%llu) < curEpoch(%llu)\0"
                    as *const u8 as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
                requestCurrentEpoch as libc::c_ulonglong,
                (*server.cluster).currentEpoch as libc::c_ulonglong,
            );
        }
        return;
    }
    if (*server.cluster).lastVoteEpoch == (*server.cluster).currentEpoch {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failover auth denied to %.40s: already voted for epoch %llu\0"
                    as *const u8 as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
                (*server.cluster).currentEpoch as libc::c_ulonglong,
            );
        }
        return;
    }
    if (*node).flags & 1 as libc::c_int != 0 || master.is_null()
        || (*master).flags & 8 as libc::c_int == 0 && force_ack == 0
    {
        if (*node).flags & 1 as libc::c_int != 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failover auth denied to %.40s: it is a master node\0" as *const u8
                        as *const libc::c_char,
                    ((*node).name).as_mut_ptr(),
                );
            }
        } else if master.is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failover auth denied to %.40s: I don't know its master\0"
                        as *const u8 as *const libc::c_char,
                    ((*node).name).as_mut_ptr(),
                );
            }
        } else if (*master).flags & 8 as libc::c_int == 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failover auth denied to %.40s: its master is up\0" as *const u8
                        as *const libc::c_char,
                    ((*node).name).as_mut_ptr(),
                );
            }
        }
        return;
    }
    if mstime() - (*(*node).slaveof).voted_time
        < server.cluster_node_timeout * 2 as libc::c_int as libc::c_longlong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failover auth denied to %.40s: can't vote about this master before %lld milliseconds\0"
                    as *const u8 as *const libc::c_char,
                ((*node).name).as_mut_ptr(),
                server.cluster_node_timeout * 2 as libc::c_int as libc::c_longlong
                    - (mstime() - (*(*node).slaveof).voted_time),
            );
        }
        return;
    }
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if !(bitmapTestBit(claimed_slots, j) == 0 as libc::c_int) {
            if !(((*server.cluster).slots[j as usize]).is_null()
                || (*(*server.cluster).slots[j as usize]).configEpoch
                    <= requestConfigEpoch)
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Failover auth denied to %.40s: slot %d epoch (%llu) > reqEpoch (%llu)\0"
                            as *const u8 as *const libc::c_char,
                        ((*node).name).as_mut_ptr(),
                        j,
                        (*(*server.cluster).slots[j as usize]).configEpoch
                            as libc::c_ulonglong,
                        requestConfigEpoch as libc::c_ulonglong,
                    );
                }
                return;
            }
        }
        j += 1;
    }
    (*server.cluster).lastVoteEpoch = (*server.cluster).currentEpoch;
    (*(*node).slaveof).voted_time = mstime();
    clusterDoBeforeSleep(
        (1 as libc::c_int) << 2 as libc::c_int | (1 as libc::c_int) << 3 as libc::c_int,
    );
    clusterSendFailoverAuth(node);
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Failover auth granted to %.40s for epoch %llu\0" as *const u8
                as *const libc::c_char,
            ((*node).name).as_mut_ptr(),
            (*server.cluster).currentEpoch as libc::c_ulonglong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterGetSlaveRank() -> libc::c_int {
    let mut myoffset: libc::c_longlong = 0;
    let mut j: libc::c_int = 0;
    let mut rank: libc::c_int = 0 as libc::c_int;
    let mut master: *mut clusterNode = 0 as *mut clusterNode;
    if (*myself).flags & 2 as libc::c_int != 0 {} else {
        _serverAssert(
            b"nodeIsSlave(myself)\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            3424 as libc::c_int,
        );
        unreachable!();
    };
    master = (*myself).slaveof;
    if master.is_null() {
        return 0 as libc::c_int;
    }
    myoffset = replicationGetSlaveOffset();
    j = 0 as libc::c_int;
    while j < (*master).numslaves {
        if *((*master).slaves).offset(j as isize) != myself
            && (**((*master).slaves).offset(j as isize)).flags & 512 as libc::c_int == 0
            && (**((*master).slaves).offset(j as isize)).repl_offset > myoffset
        {
            rank += 1;
        }
        j += 1;
    }
    return rank;
}
#[no_mangle]
pub unsafe extern "C" fn clusterLogCantFailover(mut reason: libc::c_int) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut lastlog_time: time_t = 0 as libc::c_int as time_t;
    let mut nolog_fail_time: mstime_t = server.cluster_node_timeout
        + 5000 as libc::c_int as libc::c_longlong;
    if reason == (*server.cluster).cant_failover_reason
        && time(0 as *mut time_t) - lastlog_time
            < (60 as libc::c_int * 5 as libc::c_int) as libc::c_long
    {
        return;
    }
    (*server.cluster).cant_failover_reason = reason;
    if !((*myself).slaveof).is_null()
        && (*(*myself).slaveof).flags & 8 as libc::c_int != 0
        && mstime() - (*(*myself).slaveof).fail_time < nolog_fail_time
    {
        return;
    }
    match reason {
        1 => {
            msg = b"Disconnected from master for longer than allowed. Please check the 'cluster-replica-validity-factor' configuration option.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        2 => {
            msg = b"Waiting the delay before I can start a new failover.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        3 => {
            msg = b"Failover attempt expired.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        4 => {
            msg = b"Waiting for votes, but majority still not reached.\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            msg = b"Unknown reason code.\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
    }
    lastlog_time = time(0 as *mut time_t);
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Currently unable to failover: %s\0" as *const u8 as *const libc::c_char,
            msg,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterFailoverReplaceYourMaster() {
    let mut j: libc::c_int = 0;
    let mut oldmaster: *mut clusterNode = (*myself).slaveof;
    if (*myself).flags & 1 as libc::c_int != 0 || oldmaster.is_null() {
        return;
    }
    clusterSetNodeAsMaster(myself);
    replicationUnsetMaster();
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if clusterNodeGetSlotBit(oldmaster, j) != 0 {
            clusterDelSlot(j);
            clusterAddSlot(myself, j);
        }
        j += 1;
    }
    clusterUpdateState();
    clusterSaveConfigOrDie(1 as libc::c_int);
    clusterBroadcastPong(0 as libc::c_int);
    resetManualFailover();
}
#[no_mangle]
pub unsafe extern "C" fn clusterHandleSlaveFailover() {
    let mut data_age: mstime_t = 0;
    let mut auth_age: mstime_t = mstime() - (*server.cluster).failover_auth_time;
    let mut needed_quorum: libc::c_int = (*server.cluster).size / 2 as libc::c_int
        + 1 as libc::c_int;
    let mut manual_failover: libc::c_int = ((*server.cluster).mf_end
        != 0 as libc::c_int as libc::c_longlong && (*server.cluster).mf_can_start != 0)
        as libc::c_int;
    let mut auth_timeout: mstime_t = 0;
    let mut auth_retry_time: mstime_t = 0;
    (*server.cluster).todo_before_sleep &= !((1 as libc::c_int) << 0 as libc::c_int);
    auth_timeout = server.cluster_node_timeout * 2 as libc::c_int as libc::c_longlong;
    if auth_timeout < 2000 as libc::c_int as libc::c_longlong {
        auth_timeout = 2000 as libc::c_int as mstime_t;
    }
    auth_retry_time = auth_timeout * 2 as libc::c_int as libc::c_longlong;
    if (*myself).flags & 1 as libc::c_int != 0 || ((*myself).slaveof).is_null()
        || (*(*myself).slaveof).flags & 8 as libc::c_int == 0 && manual_failover == 0
        || server.cluster_slave_no_failover != 0 && manual_failover == 0
        || (*(*myself).slaveof).numslots == 0 as libc::c_int
    {
        (*server.cluster).cant_failover_reason = 0 as libc::c_int;
        return;
    }
    if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
        data_age = (server.unixtime as libc::c_long - (*server.master).lastinteraction)
            as mstime_t * 1000 as libc::c_int as libc::c_longlong;
    } else {
        data_age = (server.unixtime as libc::c_long - server.repl_down_since) as mstime_t
            * 1000 as libc::c_int as libc::c_longlong;
    }
    if data_age > server.cluster_node_timeout {
        data_age -= server.cluster_node_timeout;
    }
    if server.cluster_slave_validity_factor != 0
        && data_age
            > server.repl_ping_slave_period as mstime_t
                * 1000 as libc::c_int as libc::c_longlong
                + server.cluster_node_timeout
                    * server.cluster_slave_validity_factor as libc::c_longlong
    {
        if manual_failover == 0 {
            clusterLogCantFailover(1 as libc::c_int);
            return;
        }
    }
    if auth_age > auth_retry_time {
        (*server.cluster)
            .failover_auth_time = mstime() + 500 as libc::c_int as libc::c_longlong
            + (random() % 500 as libc::c_int as libc::c_long) as libc::c_longlong;
        (*server.cluster).failover_auth_count = 0 as libc::c_int;
        (*server.cluster).failover_auth_sent = 0 as libc::c_int;
        (*server.cluster).failover_auth_rank = clusterGetSlaveRank();
        (*server.cluster).failover_auth_time
            += ((*server.cluster).failover_auth_rank * 1000 as libc::c_int)
                as libc::c_longlong;
        if (*server.cluster).mf_end != 0 {
            (*server.cluster).failover_auth_time = mstime();
            (*server.cluster).failover_auth_rank = 0 as libc::c_int;
            clusterDoBeforeSleep((1 as libc::c_int) << 0 as libc::c_int);
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Start of election delayed for %lld milliseconds (rank #%d, offset %lld).\0"
                    as *const u8 as *const libc::c_char,
                (*server.cluster).failover_auth_time - mstime(),
                (*server.cluster).failover_auth_rank,
                replicationGetSlaveOffset(),
            );
        }
        clusterBroadcastPong(1 as libc::c_int);
        return;
    }
    if (*server.cluster).failover_auth_sent == 0 as libc::c_int
        && (*server.cluster).mf_end == 0 as libc::c_int as libc::c_longlong
    {
        let mut newrank: libc::c_int = clusterGetSlaveRank();
        if newrank > (*server.cluster).failover_auth_rank {
            let mut added_delay: libc::c_longlong = ((newrank
                - (*server.cluster).failover_auth_rank) * 1000 as libc::c_int)
                as libc::c_longlong;
            (*server.cluster).failover_auth_time += added_delay;
            (*server.cluster).failover_auth_rank = newrank;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Replica rank updated to #%d, added %lld milliseconds of delay.\0"
                        as *const u8 as *const libc::c_char,
                    newrank,
                    added_delay,
                );
            }
        }
    }
    if mstime() < (*server.cluster).failover_auth_time {
        clusterLogCantFailover(2 as libc::c_int);
        return;
    }
    if auth_age > auth_timeout {
        clusterLogCantFailover(3 as libc::c_int);
        return;
    }
    if (*server.cluster).failover_auth_sent == 0 as libc::c_int {
        (*server.cluster)
            .currentEpoch = ((*server.cluster).currentEpoch).wrapping_add(1);
        (*server.cluster).failover_auth_epoch = (*server.cluster).currentEpoch;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Starting a failover election for epoch %llu.\0" as *const u8
                    as *const libc::c_char,
                (*server.cluster).currentEpoch as libc::c_ulonglong,
            );
        }
        clusterRequestFailoverAuth();
        (*server.cluster).failover_auth_sent = 1 as libc::c_int;
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int,
        );
        return;
    }
    if (*server.cluster).failover_auth_count >= needed_quorum {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failover election won: I'm the new master.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        if (*myself).configEpoch < (*server.cluster).failover_auth_epoch {
            (*myself).configEpoch = (*server.cluster).failover_auth_epoch;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"configEpoch set to %llu after successful failover\0" as *const u8
                        as *const libc::c_char,
                    (*myself).configEpoch as libc::c_ulonglong,
                );
            }
        }
        clusterFailoverReplaceYourMaster();
    } else {
        clusterLogCantFailover(4 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterHandleSlaveMigration(mut max_slaves: libc::c_int) {
    let mut j: libc::c_int = 0;
    let mut okslaves: libc::c_int = 0 as libc::c_int;
    let mut mymaster: *mut clusterNode = (*myself).slaveof;
    let mut target: *mut clusterNode = 0 as *mut clusterNode;
    let mut candidate: *mut clusterNode = 0 as *mut clusterNode;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    if (*server.cluster).state != 0 as libc::c_int {
        return;
    }
    if mymaster.is_null() {
        return;
    }
    j = 0 as libc::c_int;
    while j < (*mymaster).numslaves {
        if (**((*mymaster).slaves).offset(j as isize)).flags & 8 as libc::c_int == 0
            && (**((*mymaster).slaves).offset(j as isize)).flags & 4 as libc::c_int == 0
        {
            okslaves += 1;
        }
        j += 1;
    }
    if okslaves <= server.cluster_migration_barrier {
        return;
    }
    candidate = myself;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        let mut okslaves_0: libc::c_int = 0 as libc::c_int;
        let mut is_orphaned: libc::c_int = 1 as libc::c_int;
        if (*node).flags & 2 as libc::c_int != 0 || (*node).flags & 8 as libc::c_int != 0
        {
            is_orphaned = 0 as libc::c_int;
        }
        if (*node).flags & 256 as libc::c_int == 0 {
            is_orphaned = 0 as libc::c_int;
        }
        if (*node).flags & 1 as libc::c_int != 0 {
            okslaves_0 = clusterCountNonFailingSlaves(node);
        }
        if okslaves_0 > 0 as libc::c_int {
            is_orphaned = 0 as libc::c_int;
        }
        if is_orphaned != 0 {
            if target.is_null() && (*node).numslots > 0 as libc::c_int {
                target = node;
            }
            if (*node).orphaned_time == 0 {
                (*node).orphaned_time = mstime();
            }
        } else {
            (*node).orphaned_time = 0 as libc::c_int as mstime_t;
        }
        if okslaves_0 == max_slaves {
            j = 0 as libc::c_int;
            while j < (*node).numslaves {
                if memcmp(
                    ((**((*node).slaves).offset(j as isize)).name).as_mut_ptr()
                        as *const libc::c_void,
                    ((*candidate).name).as_mut_ptr() as *const libc::c_void,
                    40 as libc::c_int as libc::c_ulong,
                ) < 0 as libc::c_int
                {
                    candidate = *((*node).slaves).offset(j as isize);
                }
                j += 1;
            }
        }
    }
    dictReleaseIterator(di);
    if !target.is_null() && candidate == myself
        && mstime() - (*target).orphaned_time > 5000 as libc::c_int as libc::c_longlong
        && server.cluster_module_flags & (1 as libc::c_int) << 1 as libc::c_int == 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Migrating to orphaned master %.40s\0" as *const u8
                    as *const libc::c_char,
                ((*target).name).as_mut_ptr(),
            );
        }
        clusterSetMaster(target);
    }
}
#[no_mangle]
pub unsafe extern "C" fn resetManualFailover() {
    if !((*server.cluster).mf_slave).is_null() {
        unpauseClients(PAUSE_DURING_FAILOVER);
    }
    (*server.cluster).mf_end = 0 as libc::c_int as mstime_t;
    (*server.cluster).mf_can_start = 0 as libc::c_int;
    (*server.cluster).mf_slave = 0 as *mut clusterNode;
    (*server.cluster).mf_master_offset = -(1 as libc::c_int) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn manualFailoverCheckTimeout() {
    if (*server.cluster).mf_end != 0 && (*server.cluster).mf_end < mstime() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Manual failover timed out.\0" as *const u8 as *const libc::c_char,
            );
        }
        resetManualFailover();
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterHandleManualFailover() {
    if (*server.cluster).mf_end == 0 as libc::c_int as libc::c_longlong {
        return;
    }
    if (*server.cluster).mf_can_start != 0 {
        return;
    }
    if (*server.cluster).mf_master_offset == -(1 as libc::c_int) as libc::c_longlong {
        return;
    }
    if (*server.cluster).mf_master_offset == replicationGetSlaveOffset() {
        (*server.cluster).mf_can_start = 1 as libc::c_int;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"All master replication stream processed, manual failover can start.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        clusterDoBeforeSleep((1 as libc::c_int) << 0 as libc::c_int);
        return;
    }
    clusterDoBeforeSleep((1 as libc::c_int) << 4 as libc::c_int);
}
unsafe extern "C" fn clusterNodeCronHandleReconnect(
    mut node: *mut clusterNode,
    mut handshake_timeout: mstime_t,
    mut now: mstime_t,
) -> libc::c_int {
    if (*node).flags & (16 as libc::c_int | 64 as libc::c_int) != 0 {
        return 1 as libc::c_int;
    }
    if (*node).flags & 4 as libc::c_int != 0 {
        (*server.cluster).stats_pfail_nodes += 1;
    }
    if (*node).flags & 32 as libc::c_int != 0 && now - (*node).ctime > handshake_timeout
    {
        clusterDelNode(node);
        return 1 as libc::c_int;
    }
    if ((*node).link).is_null() {
        let mut link: *mut clusterLink = createClusterLink(node);
        (*link)
            .conn = if server.tls_cluster != 0 {
            connCreateTLS()
        } else {
            connCreateSocket()
        };
        connSetPrivateData((*link).conn, link as *mut libc::c_void);
        if connConnect(
            (*link).conn,
            ((*node).ip).as_mut_ptr(),
            (*node).cport,
            server.bind_source_addr,
            Some(
                clusterLinkConnectHandler as unsafe extern "C" fn(*mut connection) -> (),
            ),
        ) == -(1 as libc::c_int)
        {
            if (*node).ping_sent == 0 as libc::c_int as libc::c_longlong {
                (*node).ping_sent = mstime();
            }
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"Unable to connect to Cluster Node [%s]:%d -> %s\0" as *const u8
                        as *const libc::c_char,
                    ((*node).ip).as_mut_ptr(),
                    (*node).cport,
                    (server.neterr).as_mut_ptr(),
                );
            }
            freeClusterLink(link);
            return 0 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn resizeClusterLinkBuffer(mut link: *mut clusterLink) {
    if !link.is_null()
        && (sdsavail((*link).sndbuf)).wrapping_div(4 as libc::c_int as libc::c_ulong)
            > sdslen((*link).sndbuf)
    {
        (*link).sndbuf = sdsRemoveFreeSpace((*link).sndbuf);
    }
}
unsafe extern "C" fn clusterNodeCronResizeBuffers(mut node: *mut clusterNode) {
    resizeClusterLinkBuffer((*node).link);
    resizeClusterLinkBuffer((*node).inbound_link);
}
unsafe extern "C" fn freeClusterLinkOnBufferLimitReached(mut link: *mut clusterLink) {
    if link.is_null()
        || server.cluster_link_sendbuf_limit_bytes
            == 0 as libc::c_int as libc::c_ulonglong
    {
        return;
    }
    let mut mem_link: libc::c_ulonglong = sdsalloc((*link).sndbuf) as libc::c_ulonglong;
    if mem_link > server.cluster_link_sendbuf_limit_bytes {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Freeing cluster link(%s node %.40s, used memory: %llu) due to exceeding send buffer memory limit.\0"
                    as *const u8 as *const libc::c_char,
                if (*link).inbound != 0 {
                    b"from\0" as *const u8 as *const libc::c_char
                } else {
                    b"to\0" as *const u8 as *const libc::c_char
                },
                if !((*link).node).is_null() {
                    ((*(*link).node).name).as_mut_ptr() as *const libc::c_char
                } else {
                    b"\0" as *const u8 as *const libc::c_char
                },
                mem_link,
            );
        }
        freeClusterLink(link);
        (*server.cluster)
            .stat_cluster_links_buffer_limit_exceeded = ((*server.cluster)
            .stat_cluster_links_buffer_limit_exceeded)
            .wrapping_add(1);
    }
}
unsafe extern "C" fn clusterNodeCronFreeLinkOnBufferLimitReached(
    mut node: *mut clusterNode,
) {
    freeClusterLinkOnBufferLimitReached((*node).link);
    freeClusterLinkOnBufferLimitReached((*node).inbound_link);
}
unsafe extern "C" fn getClusterLinkMemUsage(mut link: *mut clusterLink) -> size_t {
    if !link.is_null() {
        return (core::mem::size_of::<clusterLink>() as libc::c_ulong)
            .wrapping_add(sdsalloc((*link).sndbuf))
            .wrapping_add((*link).rcvbuf_alloc)
    } else {
        return 0 as libc::c_int as size_t
    };
}
unsafe extern "C" fn clusterNodeCronUpdateClusterLinksMemUsage(
    mut node: *mut clusterNode,
) {
    server
        .stat_cluster_links_memory = (server.stat_cluster_links_memory as libc::c_ulong)
        .wrapping_add(getClusterLinkMemUsage((*node).link)) as size_t as size_t;
    server
        .stat_cluster_links_memory = (server.stat_cluster_links_memory as libc::c_ulong)
        .wrapping_add(getClusterLinkMemUsage((*node).inbound_link)) as size_t as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn clusterCron() {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut update_state: libc::c_int = 0 as libc::c_int;
    let mut orphaned_masters: libc::c_int = 0;
    let mut max_slaves: libc::c_int = 0;
    let mut this_slaves: libc::c_int = 0;
    let mut min_pong: mstime_t = 0 as libc::c_int as mstime_t;
    let mut now: mstime_t = mstime();
    let mut min_pong_node: *mut clusterNode = 0 as *mut clusterNode;
    static mut iteration: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut handshake_timeout: mstime_t = 0;
    iteration = iteration.wrapping_add(1);
    clusterUpdateMyselfHostname();
    handshake_timeout = server.cluster_node_timeout;
    if handshake_timeout < 1000 as libc::c_int as libc::c_longlong {
        handshake_timeout = 1000 as libc::c_int as mstime_t;
    }
    (*server.cluster).stats_pfail_nodes = 0 as libc::c_int as libc::c_longlong;
    server.stat_cluster_links_memory = 0 as libc::c_int as size_t;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        clusterNodeCronResizeBuffers(node);
        clusterNodeCronFreeLinkOnBufferLimitReached(node);
        clusterNodeCronUpdateClusterLinksMemUsage(node);
        clusterNodeCronHandleReconnect(node, handshake_timeout, now) != 0;
    }
    dictReleaseIterator(di);
    if iteration.wrapping_rem(10 as libc::c_int as libc::c_ulonglong) == 0 {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < 5 as libc::c_int {
            de = dictGetRandomKey((*server.cluster).nodes);
            let mut this: *mut clusterNode = (*de).v.val as *mut clusterNode;
            if !(((*this).link).is_null()
                || (*this).ping_sent != 0 as libc::c_int as libc::c_longlong)
            {
                if !((*this).flags & (16 as libc::c_int | 32 as libc::c_int) != 0) {
                    if min_pong_node.is_null() || min_pong > (*this).pong_received {
                        min_pong_node = this;
                        min_pong = (*this).pong_received;
                    }
                }
            }
            j += 1;
        }
        if !min_pong_node.is_null() {
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"Pinging node %.40s\0" as *const u8 as *const libc::c_char,
                    ((*min_pong_node).name).as_mut_ptr(),
                );
            }
            clusterSendPing((*min_pong_node).link, 0 as libc::c_int);
        }
    }
    orphaned_masters = 0 as libc::c_int;
    max_slaves = 0 as libc::c_int;
    this_slaves = 0 as libc::c_int;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node_0: *mut clusterNode = (*de).v.val as *mut clusterNode;
        now = mstime();
        if (*node_0).flags & (16 as libc::c_int | 64 as libc::c_int | 32 as libc::c_int)
            != 0
        {
            continue;
        }
        if (*myself).flags & 2 as libc::c_int != 0
            && (*node_0).flags & 1 as libc::c_int != 0
            && (*node_0).flags & 8 as libc::c_int == 0
        {
            let mut okslaves: libc::c_int = clusterCountNonFailingSlaves(node_0);
            if okslaves == 0 as libc::c_int && (*node_0).numslots > 0 as libc::c_int
                && (*node_0).flags & 256 as libc::c_int != 0
            {
                orphaned_masters += 1;
            }
            if okslaves > max_slaves {
                max_slaves = okslaves;
            }
            if (*myself).slaveof == node_0 {
                this_slaves = okslaves;
            }
        }
        let mut ping_delay: mstime_t = now - (*node_0).ping_sent;
        let mut data_delay: mstime_t = now - (*node_0).data_received;
        if !((*node_0).link).is_null()
            && now - (*(*node_0).link).ctime > server.cluster_node_timeout
            && (*node_0).ping_sent != 0
            && ping_delay
                > server.cluster_node_timeout / 2 as libc::c_int as libc::c_longlong
            && data_delay
                > server.cluster_node_timeout / 2 as libc::c_int as libc::c_longlong
        {
            freeClusterLink((*node_0).link);
        }
        if !((*node_0).link).is_null()
            && (*node_0).ping_sent == 0 as libc::c_int as libc::c_longlong
            && now - (*node_0).pong_received
                > server.cluster_node_timeout / 2 as libc::c_int as libc::c_longlong
        {
            clusterSendPing((*node_0).link, 0 as libc::c_int);
        } else if (*server.cluster).mf_end != 0
            && (*myself).flags & 1 as libc::c_int != 0
            && (*server.cluster).mf_slave == node_0 && !((*node_0).link).is_null()
        {
            clusterSendPing((*node_0).link, 0 as libc::c_int);
        } else {
            if (*node_0).ping_sent == 0 as libc::c_int as libc::c_longlong {
                continue;
            }
            let mut node_delay: mstime_t = if ping_delay < data_delay {
                ping_delay
            } else {
                data_delay
            };
            if node_delay > server.cluster_node_timeout {
                if (*node_0).flags & (4 as libc::c_int | 8 as libc::c_int) == 0 {
                    if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            0 as libc::c_int,
                            b"*** NODE %.40s possibly failing\0" as *const u8
                                as *const libc::c_char,
                            ((*node_0).name).as_mut_ptr(),
                        );
                    }
                    (*node_0).flags |= 4 as libc::c_int;
                    update_state = 1 as libc::c_int;
                }
            }
        }
    }
    dictReleaseIterator(di);
    if (*myself).flags & 2 as libc::c_int != 0 && (server.masterhost).is_null()
        && !((*myself).slaveof).is_null()
        && (*(*myself).slaveof).flags & 64 as libc::c_int == 0
    {
        replicationSetMaster(
            ((*(*myself).slaveof).ip).as_mut_ptr(),
            (*(*myself).slaveof).port,
        );
    }
    manualFailoverCheckTimeout();
    if (*myself).flags & 2 as libc::c_int != 0 {
        clusterHandleManualFailover();
        if server.cluster_module_flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
            clusterHandleSlaveFailover();
        }
        if orphaned_masters != 0 && max_slaves >= 2 as libc::c_int
            && this_slaves == max_slaves && server.cluster_allow_replica_migration != 0
        {
            clusterHandleSlaveMigration(max_slaves);
        }
    }
    if update_state != 0 || (*server.cluster).state == 1 as libc::c_int {
        clusterUpdateState();
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterBeforeSleep() {
    let mut flags: libc::c_int = (*server.cluster).todo_before_sleep;
    (*server.cluster).todo_before_sleep = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        if (*myself).flags & 2 as libc::c_int != 0 {
            clusterHandleManualFailover();
            if server.cluster_module_flags & (1 as libc::c_int) << 1 as libc::c_int == 0
            {
                clusterHandleSlaveFailover();
            }
        }
    } else if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        clusterHandleSlaveFailover();
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        clusterUpdateState();
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        let mut fsync_0: libc::c_int = flags & (1 as libc::c_int) << 3 as libc::c_int;
        clusterSaveConfigOrDie(fsync_0);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterDoBeforeSleep(mut flags: libc::c_int) {
    (*server.cluster).todo_before_sleep |= flags;
}
#[no_mangle]
pub unsafe extern "C" fn bitmapTestBit(
    mut bitmap: *mut libc::c_uchar,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut byte: off_t = (pos / 8 as libc::c_int) as off_t;
    let mut bit: libc::c_int = pos & 7 as libc::c_int;
    return (*bitmap.offset(byte as isize) as libc::c_int & (1 as libc::c_int) << bit
        != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bitmapSetBit(
    mut bitmap: *mut libc::c_uchar,
    mut pos: libc::c_int,
) {
    let mut byte: off_t = (pos / 8 as libc::c_int) as off_t;
    let mut bit: libc::c_int = pos & 7 as libc::c_int;
    let ref mut fresh8 = *bitmap.offset(byte as isize);
    *fresh8 = (*fresh8 as libc::c_int | (1 as libc::c_int) << bit) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn bitmapClearBit(
    mut bitmap: *mut libc::c_uchar,
    mut pos: libc::c_int,
) {
    let mut byte: off_t = (pos / 8 as libc::c_int) as off_t;
    let mut bit: libc::c_int = pos & 7 as libc::c_int;
    let ref mut fresh9 = *bitmap.offset(byte as isize);
    *fresh9 = (*fresh9 as libc::c_int & !((1 as libc::c_int) << bit)) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn clusterMastersHaveSlaves() -> libc::c_int {
    let mut di: *mut dictIterator = dictGetSafeIterator((*server.cluster).nodes);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut slaves: libc::c_int = 0 as libc::c_int;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).flags & 2 as libc::c_int != 0 {
            continue;
        }
        slaves += (*node).numslaves;
    }
    dictReleaseIterator(di);
    return (slaves != 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeSetSlotBit(
    mut n: *mut clusterNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut old: libc::c_int = bitmapTestBit(((*n).slots).as_mut_ptr(), slot);
    bitmapSetBit(((*n).slots).as_mut_ptr(), slot);
    if old == 0 {
        (*n).numslots += 1;
        if (*n).numslots == 1 as libc::c_int && clusterMastersHaveSlaves() != 0 {
            (*n).flags |= 256 as libc::c_int;
        }
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeClearSlotBit(
    mut n: *mut clusterNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    let mut old: libc::c_int = bitmapTestBit(((*n).slots).as_mut_ptr(), slot);
    bitmapClearBit(((*n).slots).as_mut_ptr(), slot);
    if old != 0 {
        (*n).numslots -= 1;
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn clusterNodeGetSlotBit(
    mut n: *mut clusterNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    return bitmapTestBit(((*n).slots).as_mut_ptr(), slot);
}
#[no_mangle]
pub unsafe extern "C" fn clusterAddSlot(
    mut n: *mut clusterNode,
    mut slot: libc::c_int,
) -> libc::c_int {
    if !((*server.cluster).slots[slot as usize]).is_null() {
        return -(1 as libc::c_int);
    }
    clusterNodeSetSlotBit(n, slot);
    (*server.cluster).slots[slot as usize] = n;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterDelSlot(mut slot: libc::c_int) -> libc::c_int {
    let mut n: *mut clusterNode = (*server.cluster).slots[slot as usize];
    if n.is_null() {
        return -(1 as libc::c_int);
    }
    let mut nodes_for_slot: *mut list = clusterGetNodesServingMySlots(n);
    let mut ln: *mut listNode = listSearchKey(
        nodes_for_slot,
        myself as *mut libc::c_void,
    );
    if !ln.is_null() {
        removeChannelsInSlot(slot as libc::c_uint);
    }
    listRelease(nodes_for_slot);
    if clusterNodeClearSlotBit(n, slot) == 1 as libc::c_int {} else {
        _serverAssert(
            b"clusterNodeClearSlotBit(n,slot) == 1\0" as *const u8
                as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            4358 as libc::c_int,
        );
        unreachable!();
    };
    (*server.cluster).slots[slot as usize] = 0 as *mut clusterNode;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterDelNodeSlots(mut node: *mut clusterNode) -> libc::c_int {
    let mut deleted: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if clusterNodeGetSlotBit(node, j) != 0 {
            clusterDelSlot(j);
            deleted += 1;
        }
        j += 1;
    }
    return deleted;
}
#[no_mangle]
pub unsafe extern "C" fn clusterCloseAllSlots() {
    memset(
        ((*server.cluster).migrating_slots_to).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[*mut clusterNode; 16384]>() as libc::c_ulong,
    );
    memset(
        ((*server.cluster).importing_slots_from).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[*mut clusterNode; 16384]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateState() {
    let mut j: libc::c_int = 0;
    let mut new_state: libc::c_int = 0;
    let mut reachable_masters: libc::c_int = 0 as libc::c_int;
    static mut among_minority_time: mstime_t = 0;
    static mut first_call_time: mstime_t = 0 as libc::c_int as mstime_t;
    (*server.cluster).todo_before_sleep &= !((1 as libc::c_int) << 1 as libc::c_int);
    if first_call_time == 0 as libc::c_int as libc::c_longlong {
        first_call_time = mstime();
    }
    if (*myself).flags & 1 as libc::c_int != 0
        && (*server.cluster).state == 1 as libc::c_int
        && mstime() - first_call_time < 2000 as libc::c_int as libc::c_longlong
    {
        return;
    }
    new_state = 0 as libc::c_int;
    if server.cluster_require_full_coverage != 0 {
        j = 0 as libc::c_int;
        while j < 16384 as libc::c_int {
            if ((*server.cluster).slots[j as usize]).is_null()
                || (*(*server.cluster).slots[j as usize]).flags & 8 as libc::c_int != 0
            {
                new_state = 1 as libc::c_int;
                break;
            } else {
                j += 1;
            }
        }
    }
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    (*server.cluster).size = 0 as libc::c_int;
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).flags & 1 as libc::c_int != 0 && (*node).numslots != 0 {
            (*server.cluster).size += 1;
            if (*node).flags & (8 as libc::c_int | 4 as libc::c_int) == 0 as libc::c_int
            {
                reachable_masters += 1;
            }
        }
    }
    dictReleaseIterator(di);
    let mut needed_quorum: libc::c_int = (*server.cluster).size / 2 as libc::c_int
        + 1 as libc::c_int;
    if reachable_masters < needed_quorum {
        new_state = 1 as libc::c_int;
        among_minority_time = mstime();
    }
    if new_state != (*server.cluster).state {
        let mut rejoin_delay: mstime_t = server.cluster_node_timeout;
        if rejoin_delay > 5000 as libc::c_int as libc::c_longlong {
            rejoin_delay = 5000 as libc::c_int as mstime_t;
        }
        if rejoin_delay < 500 as libc::c_int as libc::c_longlong {
            rejoin_delay = 500 as libc::c_int as mstime_t;
        }
        if new_state == 0 as libc::c_int && (*myself).flags & 1 as libc::c_int != 0
            && mstime() - among_minority_time < rejoin_delay
        {
            return;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Cluster state changed: %s\0" as *const u8 as *const libc::c_char,
                if new_state == 0 as libc::c_int {
                    b"ok\0" as *const u8 as *const libc::c_char
                } else {
                    b"fail\0" as *const u8 as *const libc::c_char
                },
            );
        }
        (*server.cluster).state = new_state;
    }
}
#[no_mangle]
pub unsafe extern "C" fn verifyClusterConfigWithData() -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut update_config: libc::c_int = 0 as libc::c_int;
    if server.cluster_module_flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    if (*myself).flags & 2 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    j = 1 as libc::c_int;
    while j < server.dbnum {
        if ((*(*(server.db).offset(j as isize)).dict).ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*(*(server.db).offset(j as isize)).dict)
                    .ht_used[1 as libc::c_int as usize],
            ) != 0
        {
            return -(1 as libc::c_int);
        }
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if !(countKeysInSlot(j as libc::c_uint) == 0) {
            if !((*server.cluster).slots[j as usize] == myself
                || !((*server.cluster).importing_slots_from[j as usize]).is_null())
            {
                update_config += 1;
                if ((*server.cluster).slots[j as usize]).is_null() {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"I have keys for unassigned slot %d. Taking responsibility for it.\0"
                                as *const u8 as *const libc::c_char,
                            j,
                        );
                    }
                    clusterAddSlot(myself, j);
                } else {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"I have keys for slot %d, but the slot is assigned to another node. Setting it to importing state.\0"
                                as *const u8 as *const libc::c_char,
                            j,
                        );
                    }
                    (*server.cluster)
                        .importing_slots_from[j
                        as usize] = (*server.cluster).slots[j as usize];
                }
            }
        }
        j += 1;
    }
    if update_config != 0 {
        clusterSaveConfigOrDie(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterSetMaster(mut n: *mut clusterNode) {
    if n != myself {} else {
        _serverAssert(
            b"n != myself\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            4572 as libc::c_int,
        );
        unreachable!();
    };
    if (*myself).numslots == 0 as libc::c_int {} else {
        _serverAssert(
            b"myself->numslots == 0\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            4573 as libc::c_int,
        );
        unreachable!();
    };
    if (*myself).flags & 1 as libc::c_int != 0 {
        (*myself).flags &= !(1 as libc::c_int | 256 as libc::c_int);
        (*myself).flags |= 2 as libc::c_int;
        clusterCloseAllSlots();
    } else if !((*myself).slaveof).is_null() {
        clusterNodeRemoveSlave((*myself).slaveof, myself);
    }
    (*myself).slaveof = n;
    clusterNodeAddSlave(n, myself);
    replicationSetMaster(((*n).ip).as_mut_ptr(), (*n).port);
    resetManualFailover();
}
static mut redisNodeFlagsTable: [redisNodeFlags; 8] = [
    {
        let mut init = redisNodeFlags {
            flag: 16 as libc::c_int as uint16_t,
            name: b"myself,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 1 as libc::c_int as uint16_t,
            name: b"master,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 2 as libc::c_int as uint16_t,
            name: b"slave,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 4 as libc::c_int as uint16_t,
            name: b"fail?,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 8 as libc::c_int as uint16_t,
            name: b"fail,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 32 as libc::c_int as uint16_t,
            name: b"handshake,\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 64 as libc::c_int as uint16_t,
            name: b"noaddr,\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        };
        init
    },
    {
        let mut init = redisNodeFlags {
            flag: 512 as libc::c_int as uint16_t,
            name: b"nofailover,\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn representClusterNodeFlags(
    mut ci: sds,
    mut flags: uint16_t,
) -> sds {
    let mut orig_len: size_t = sdslen(ci);
    let mut i: libc::c_int = 0;
    let mut size: libc::c_int = (core::mem::size_of::<[redisNodeFlags; 8]>()
        as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<redisNodeFlags>() as libc::c_ulong)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < size {
        let mut nodeflag: *mut redisNodeFlags = redisNodeFlagsTable
            .as_mut_ptr()
            .offset(i as isize);
        if flags as libc::c_int & (*nodeflag).flag as libc::c_int != 0 {
            ci = sdscat(ci, (*nodeflag).name);
        }
        i += 1;
    }
    if sdslen(ci) == orig_len {
        ci = sdscat(ci, b"noflags,\0" as *const u8 as *const libc::c_char);
    }
    sdsIncrLen(ci, -(1 as libc::c_int) as ssize_t);
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn representSlotInfo(
    mut ci: sds,
    mut slot_info_pairs: *mut uint16_t,
    mut slot_info_pairs_count: libc::c_int,
) -> sds {
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < slot_info_pairs_count {
        let mut start: libc::c_ulong = *slot_info_pairs.offset(i as isize)
            as libc::c_ulong;
        let mut end: libc::c_ulong = *slot_info_pairs
            .offset((i + 1 as libc::c_int) as isize) as libc::c_ulong;
        if start == end {
            ci = sdscatfmt(ci, b" %i\0" as *const u8 as *const libc::c_char, start);
        } else {
            ci = sdscatfmt(
                ci,
                b" %i-%i\0" as *const u8 as *const libc::c_char,
                start,
                end,
            );
        }
        i += 2 as libc::c_int;
    }
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn clusterGenNodeDescription(
    mut node: *mut clusterNode,
    mut use_pport: libc::c_int,
) -> sds {
    let mut j: libc::c_int = 0;
    let mut start: libc::c_int = 0;
    let mut ci: sds = 0 as *mut libc::c_char;
    let mut port: libc::c_int = if use_pport != 0 && (*node).pport != 0 {
        (*node).pport
    } else {
        (*node).port
    };
    ci = sdscatlen(
        sdsempty(),
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    if sdslen((*node).hostname) != 0 as libc::c_int as libc::c_ulong {
        ci = sdscatfmt(
            ci,
            b" %s:%i@%i,%s \0" as *const u8 as *const libc::c_char,
            ((*node).ip).as_mut_ptr(),
            port,
            (*node).cport,
            (*node).hostname,
        );
    } else {
        ci = sdscatfmt(
            ci,
            b" %s:%i@%i \0" as *const u8 as *const libc::c_char,
            ((*node).ip).as_mut_ptr(),
            port,
            (*node).cport,
        );
    }
    ci = representClusterNodeFlags(ci, (*node).flags as uint16_t);
    ci = sdscatlen(
        ci,
        b" \0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    if !((*node).slaveof).is_null() {
        ci = sdscatlen(
            ci,
            ((*(*node).slaveof).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
        );
    } else {
        ci = sdscatlen(
            ci,
            b"-\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    let mut nodeEpoch: libc::c_ulonglong = (*node).configEpoch as libc::c_ulonglong;
    if (*node).flags & 2 as libc::c_int != 0 && !((*node).slaveof).is_null() {
        nodeEpoch = (*(*node).slaveof).configEpoch as libc::c_ulonglong;
    }
    ci = sdscatfmt(
        ci,
        b" %I %I %U %s\0" as *const u8 as *const libc::c_char,
        (*node).ping_sent,
        (*node).pong_received,
        nodeEpoch,
        if !((*node).link).is_null() || (*node).flags & 16 as libc::c_int != 0 {
            b"connected\0" as *const u8 as *const libc::c_char
        } else {
            b"disconnected\0" as *const u8 as *const libc::c_char
        },
    );
    if !((*node).slot_info_pairs).is_null() {
        ci = representSlotInfo(
            ci,
            (*node).slot_info_pairs,
            (*node).slot_info_pairs_count,
        );
    } else if (*node).numslots > 0 as libc::c_int {
        start = -(1 as libc::c_int);
        j = 0 as libc::c_int;
        while j < 16384 as libc::c_int {
            let mut bit: libc::c_int = 0;
            bit = clusterNodeGetSlotBit(node, j);
            if bit != 0 as libc::c_int {
                if start == -(1 as libc::c_int) {
                    start = j;
                }
            }
            if start != -(1 as libc::c_int)
                && (bit == 0 || j == 16384 as libc::c_int - 1 as libc::c_int)
            {
                if bit != 0 && j == 16384 as libc::c_int - 1 as libc::c_int {
                    j += 1;
                }
                if start == j - 1 as libc::c_int {
                    ci = sdscatfmt(
                        ci,
                        b" %i\0" as *const u8 as *const libc::c_char,
                        start,
                    );
                } else {
                    ci = sdscatfmt(
                        ci,
                        b" %i-%i\0" as *const u8 as *const libc::c_char,
                        start,
                        j - 1 as libc::c_int,
                    );
                }
                start = -(1 as libc::c_int);
            }
            j += 1;
        }
    }
    if (*node).flags & 16 as libc::c_int != 0 {
        j = 0 as libc::c_int;
        while j < 16384 as libc::c_int {
            if !((*server.cluster).migrating_slots_to[j as usize]).is_null() {
                ci = sdscatprintf(
                    ci,
                    b" [%d->-%.40s]\0" as *const u8 as *const libc::c_char,
                    j,
                    ((*(*server.cluster).migrating_slots_to[j as usize]).name)
                        .as_mut_ptr(),
                );
            } else if !((*server.cluster).importing_slots_from[j as usize]).is_null() {
                ci = sdscatprintf(
                    ci,
                    b" [%d-<-%.40s]\0" as *const u8 as *const libc::c_char,
                    j,
                    ((*(*server.cluster).importing_slots_from[j as usize]).name)
                        .as_mut_ptr(),
                );
            }
            j += 1;
        }
    }
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn clusterGenNodesSlotsInfo(mut filter: libc::c_int) {
    let mut n: *mut clusterNode = 0 as *mut clusterNode;
    let mut start: libc::c_int = -(1 as libc::c_int);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 16384 as libc::c_int {
        if n.is_null() {
            if i == 16384 as libc::c_int {
                break;
            }
            n = (*server.cluster).slots[i as usize];
            start = i;
        } else if i == 16384 as libc::c_int || n != (*server.cluster).slots[i as usize] {
            if (*n).flags & filter == 0 {
                if ((*n).slot_info_pairs).is_null() {
                    (*n)
                        .slot_info_pairs = zmalloc(
                        ((2 as libc::c_int * (*n).numslots) as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<uint16_t>() as libc::c_ulong,
                            ),
                    ) as *mut uint16_t;
                }
                if ((*n).slot_info_pairs_count + 1 as libc::c_int)
                    < 2 as libc::c_int * (*n).numslots
                {} else {
                    _serverAssert(
                        b"(n->slot_info_pairs_count + 1) < (2 * n->numslots)\0"
                            as *const u8 as *const libc::c_char,
                        b"cluster.c\0" as *const u8 as *const libc::c_char,
                        4752 as libc::c_int,
                    );
                    unreachable!();
                };
                let fresh10 = (*n).slot_info_pairs_count;
                (*n).slot_info_pairs_count = (*n).slot_info_pairs_count + 1;
                *((*n).slot_info_pairs).offset(fresh10 as isize) = start as uint16_t;
                let fresh11 = (*n).slot_info_pairs_count;
                (*n).slot_info_pairs_count = (*n).slot_info_pairs_count + 1;
                *((*n).slot_info_pairs)
                    .offset(fresh11 as isize) = (i - 1 as libc::c_int) as uint16_t;
            }
            if i == 16384 as libc::c_int {
                break;
            }
            n = (*server.cluster).slots[i as usize];
            start = i;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterFreeNodesSlotsInfo(mut n: *mut clusterNode) {
    zfree((*n).slot_info_pairs as *mut libc::c_void);
    (*n).slot_info_pairs = 0 as *mut uint16_t;
    (*n).slot_info_pairs_count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterGenNodesDescription(
    mut filter: libc::c_int,
    mut use_pport: libc::c_int,
) -> sds {
    let mut ci: sds = sdsempty();
    let mut ni: sds = 0 as *mut libc::c_char;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    clusterGenNodesSlotsInfo(filter);
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).flags & filter != 0 {
            continue;
        }
        ni = clusterGenNodeDescription(node, use_pport);
        ci = sdscatsds(ci, ni);
        sdsfree(ni);
        ci = sdscatlen(
            ci,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        clusterFreeNodesSlotsInfo(node);
    }
    dictReleaseIterator(di);
    return ci;
}
#[no_mangle]
pub unsafe extern "C" fn addReplyClusterLinkDescription(
    mut c: *mut client,
    mut link: *mut clusterLink,
) {
    addReplyMapLen(c, 6 as libc::c_int as libc::c_long);
    addReplyBulkCString(c, b"direction\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(
        c,
        if (*link).inbound != 0 {
            b"from\0" as *const u8 as *const libc::c_char
        } else {
            b"to\0" as *const u8 as *const libc::c_char
        },
    );
    if !((*link).node).is_null() {} else {
        _serverAssert(
            b"link->node\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            4820 as libc::c_int,
        );
        unreachable!();
    };
    let mut node_name: sds = sdsnewlen(
        ((*(*link).node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    addReplyBulkCString(c, b"node\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, node_name as *const libc::c_char);
    sdsfree(node_name);
    addReplyBulkCString(c, b"create-time\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*link).ctime);
    let mut events: [libc::c_char; 3] = [0; 3];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    p = events.as_mut_ptr();
    if !((*link).conn).is_null() {
        if connHasReadHandler((*link).conn) != 0 {
            let fresh12 = p;
            p = p.offset(1);
            *fresh12 = 'r' as i32 as libc::c_char;
        }
        if connHasWriteHandler((*link).conn) != 0 {
            let fresh13 = p;
            p = p.offset(1);
            *fresh13 = 'w' as i32 as libc::c_char;
        }
    }
    *p = '\0' as i32 as libc::c_char;
    addReplyBulkCString(c, b"events\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, events.as_mut_ptr());
    addReplyBulkCString(
        c,
        b"send-buffer-allocated\0" as *const u8 as *const libc::c_char,
    );
    addReplyLongLong(c, sdsalloc((*link).sndbuf) as libc::c_longlong);
    addReplyBulkCString(c, b"send-buffer-used\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, sdslen((*link).sndbuf) as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyClusterLinksDescription(mut c: *mut client) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut arraylen_ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut num_links: libc::c_int = 0 as libc::c_int;
    arraylen_ptr = addReplyDeferredLen(c);
    di = dictGetSafeIterator((*server.cluster).nodes);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if !((*node).link).is_null() {
            num_links += 1;
            addReplyClusterLinkDescription(c, (*node).link);
        }
        if !((*node).inbound_link).is_null() {
            num_links += 1;
            addReplyClusterLinkDescription(c, (*node).inbound_link);
        }
    }
    dictReleaseIterator(di);
    setDeferredArrayLen(c, arraylen_ptr, num_links as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn getPreferredEndpoint(
    mut n: *mut clusterNode,
) -> *const libc::c_char {
    match server.cluster_preferred_endpoint_type {
        0 => return ((*n).ip).as_mut_ptr(),
        1 => {
            return if sdslen((*n).hostname) != 0 as libc::c_int as libc::c_ulong {
                (*n).hostname as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            };
        }
        2 => return b"\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"unknown\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn clusterGetMessageTypeString(
    mut type_0: libc::c_int,
) -> *const libc::c_char {
    match type_0 {
        0 => return b"ping\0" as *const u8 as *const libc::c_char,
        1 => return b"pong\0" as *const u8 as *const libc::c_char,
        2 => return b"meet\0" as *const u8 as *const libc::c_char,
        3 => return b"fail\0" as *const u8 as *const libc::c_char,
        4 => return b"publish\0" as *const u8 as *const libc::c_char,
        10 => return b"publishshard\0" as *const u8 as *const libc::c_char,
        5 => return b"auth-req\0" as *const u8 as *const libc::c_char,
        6 => return b"auth-ack\0" as *const u8 as *const libc::c_char,
        7 => return b"update\0" as *const u8 as *const libc::c_char,
        8 => return b"mfstart\0" as *const u8 as *const libc::c_char,
        9 => return b"module\0" as *const u8 as *const libc::c_char,
        _ => {}
    }
    return b"unknown\0" as *const u8 as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn getSlotOrReply(
    mut c: *mut client,
    mut o: *mut robj,
) -> libc::c_int {
    let mut slot: libc::c_longlong = 0;
    if getLongLongFromObject(o, &mut slot) != 0 as libc::c_int
        || slot < 0 as libc::c_int as libc::c_longlong
        || slot >= 16384 as libc::c_int as libc::c_longlong
    {
        addReplyError(
            c,
            b"Invalid or out of range slot\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    return slot as libc::c_int;
}
unsafe extern "C" fn isReplicaAvailable(mut node: *mut clusterNode) -> libc::c_int {
    if (*node).flags & 8 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    let mut repl_offset: libc::c_longlong = (*node).repl_offset;
    if (*node).flags & 16 as libc::c_int != 0 {
        repl_offset = replicationGetSlaveOffset();
    }
    return (repl_offset != 0 as libc::c_int as libc::c_longlong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn checkSlotAssignmentsOrReply(
    mut c: *mut client,
    mut slots: *mut libc::c_uchar,
    mut del: libc::c_int,
    mut start_slot: libc::c_int,
    mut end_slot: libc::c_int,
) -> libc::c_int {
    let mut slot: libc::c_int = 0;
    slot = start_slot;
    while slot <= end_slot {
        if del != 0 && ((*server.cluster).slots[slot as usize]).is_null() {
            addReplyErrorFormat(
                c,
                b"Slot %d is already unassigned\0" as *const u8 as *const libc::c_char,
                slot,
            );
            return -(1 as libc::c_int);
        } else {
            if del == 0 && !((*server.cluster).slots[slot as usize]).is_null() {
                addReplyErrorFormat(
                    c,
                    b"Slot %d is already busy\0" as *const u8 as *const libc::c_char,
                    slot,
                );
                return -(1 as libc::c_int);
            }
        }
        let ref mut fresh14 = *slots.offset(slot as isize);
        let fresh15 = *fresh14;
        *fresh14 = (*fresh14).wrapping_add(1);
        if fresh15 as libc::c_int == 1 as libc::c_int {
            addReplyErrorFormat(
                c,
                b"Slot %d specified multiple times\0" as *const u8
                    as *const libc::c_char,
                slot,
            );
            return -(1 as libc::c_int);
        }
        slot += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clusterUpdateSlots(
    mut c: *mut client,
    mut slots: *mut libc::c_uchar,
    mut del: libc::c_int,
) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 16384 as libc::c_int {
        if *slots.offset(j as isize) != 0 {
            let mut retval: libc::c_int = 0;
            if !((*server.cluster).importing_slots_from[j as usize]).is_null() {
                (*server.cluster)
                    .importing_slots_from[j as usize] = 0 as *mut clusterNode;
            }
            retval = if del != 0 {
                clusterDelSlot(j)
            } else {
                clusterAddSlot(myself, j)
            };
            if retval == 0 as libc::c_int {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"retval == C_OK\0" as *const u8 as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    4964 as libc::c_int,
                );
                unreachable!();
            };
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addNodeToNodeReply(
    mut c: *mut client,
    mut node: *mut clusterNode,
) {
    addReplyArrayLen(c, 4 as libc::c_int as libc::c_long);
    if server.cluster_preferred_endpoint_type == CLUSTER_ENDPOINT_TYPE_IP as libc::c_int
    {
        addReplyBulkCString(c, ((*node).ip).as_mut_ptr());
    } else if server.cluster_preferred_endpoint_type
        == CLUSTER_ENDPOINT_TYPE_HOSTNAME as libc::c_int
    {
        if sdslen((*node).hostname) != 0 as libc::c_int as libc::c_ulong {
            addReplyBulkCBuffer(
                c,
                (*node).hostname as *const libc::c_void,
                sdslen((*node).hostname),
            );
        } else {
            addReplyBulkCString(c, b"?\0" as *const u8 as *const libc::c_char);
        }
    } else if server.cluster_preferred_endpoint_type
        == CLUSTER_ENDPOINT_TYPE_UNKNOWN_ENDPOINT as libc::c_int
    {
        addReplyNull(c);
    } else {
        _serverPanic(
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            4982 as libc::c_int,
            b"Unrecognized preferred endpoint type\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    let mut use_pport: libc::c_int = (server.tls_cluster != 0 && !((*c).conn).is_null()
        && connGetType((*c).conn) != 2 as libc::c_int) as libc::c_int;
    addReplyLongLong(
        c,
        (if use_pport != 0 && (*node).pport != 0 { (*node).pport } else { (*node).port })
            as libc::c_longlong,
    );
    addReplyBulkCBuffer(
        c,
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    let mut deflen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut length: libc::c_int = 0 as libc::c_int;
    if server.cluster_preferred_endpoint_type != CLUSTER_ENDPOINT_TYPE_IP as libc::c_int
    {
        addReplyBulkCString(c, b"ip\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, ((*node).ip).as_mut_ptr());
        length += 1;
    }
    if server.cluster_preferred_endpoint_type
        != CLUSTER_ENDPOINT_TYPE_HOSTNAME as libc::c_int
        && sdslen((*node).hostname) != 0 as libc::c_int as libc::c_ulong
    {
        addReplyBulkCString(c, b"hostname\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(
            c,
            (*node).hostname as *const libc::c_void,
            sdslen((*node).hostname),
        );
        length += 1;
    }
    setDeferredMapLen(c, deflen, length as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn addNodeReplyForClusterSlot(
    mut c: *mut client,
    mut node: *mut clusterNode,
    mut start_slot: libc::c_int,
    mut end_slot: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut nested_elements: libc::c_int = 3 as libc::c_int;
    let mut nested_replylen: *mut libc::c_void = addReplyDeferredLen(c);
    addReplyLongLong(c, start_slot as libc::c_longlong);
    addReplyLongLong(c, end_slot as libc::c_longlong);
    addNodeToNodeReply(c, node);
    i = 0 as libc::c_int;
    while i < (*node).numslaves {
        if !(isReplicaAvailable(*((*node).slaves).offset(i as isize)) == 0) {
            addNodeToNodeReply(c, *((*node).slaves).offset(i as isize));
            nested_elements += 1;
        }
        i += 1;
    }
    setDeferredArrayLen(c, nested_replylen, nested_elements as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn addNodeDetailsToShardReply(
    mut c: *mut client,
    mut node: *mut clusterNode,
) {
    let mut reply_count: libc::c_int = 0 as libc::c_int;
    let mut node_replylen: *mut libc::c_void = addReplyDeferredLen(c);
    addReplyBulkCString(c, b"id\0" as *const u8 as *const libc::c_char);
    addReplyBulkCBuffer(
        c,
        ((*node).name).as_mut_ptr() as *const libc::c_void,
        40 as libc::c_int as size_t,
    );
    reply_count += 1;
    let mut plaintext_port: libc::c_int = if server.tls_cluster != 0 {
        (*node).pport
    } else {
        (*node).port
    };
    let mut tls_port: libc::c_int = if server.tls_cluster != 0 {
        (*node).port
    } else {
        0 as libc::c_int
    };
    if plaintext_port != 0 {
        addReplyBulkCString(c, b"port\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, plaintext_port as libc::c_longlong);
        reply_count += 1;
    }
    if tls_port != 0 {
        addReplyBulkCString(c, b"tls-port\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, tls_port as libc::c_longlong);
        reply_count += 1;
    }
    addReplyBulkCString(c, b"ip\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, ((*node).ip).as_mut_ptr());
    reply_count += 1;
    addReplyBulkCString(c, b"endpoint\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, getPreferredEndpoint(node));
    reply_count += 1;
    if sdslen((*node).hostname) != 0 as libc::c_int as libc::c_ulong {
        addReplyBulkCString(c, b"hostname\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(
            c,
            (*node).hostname as *const libc::c_void,
            sdslen((*node).hostname),
        );
        reply_count += 1;
    }
    let mut node_offset: libc::c_longlong = 0;
    if (*node).flags & 16 as libc::c_int != 0 {
        node_offset = if (*node).flags & 2 as libc::c_int != 0 {
            replicationGetSlaveOffset()
        } else {
            server.master_repl_offset
        };
    } else {
        node_offset = (*node).repl_offset;
    }
    addReplyBulkCString(c, b"role\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(
        c,
        if (*node).flags & 2 as libc::c_int != 0 {
            b"replica\0" as *const u8 as *const libc::c_char
        } else {
            b"master\0" as *const u8 as *const libc::c_char
        },
    );
    reply_count += 1;
    addReplyBulkCString(c, b"replication-offset\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, node_offset);
    reply_count += 1;
    addReplyBulkCString(c, b"health\0" as *const u8 as *const libc::c_char);
    let mut health_msg: *const libc::c_char = 0 as *const libc::c_char;
    if (*node).flags & 8 as libc::c_int != 0 {
        health_msg = b"fail\0" as *const u8 as *const libc::c_char;
    } else if (*node).flags & 2 as libc::c_int != 0
        && node_offset == 0 as libc::c_int as libc::c_longlong
    {
        health_msg = b"loading\0" as *const u8 as *const libc::c_char;
    } else {
        health_msg = b"online\0" as *const u8 as *const libc::c_char;
    }
    addReplyBulkCString(c, health_msg);
    reply_count += 1;
    setDeferredMapLen(c, node_replylen, reply_count as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn addShardReplyForClusterShards(
    mut c: *mut client,
    mut node: *mut clusterNode,
    mut slot_info_pairs: *mut uint16_t,
    mut slot_pairs_count: libc::c_int,
) {
    addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
    addReplyBulkCString(c, b"slots\0" as *const u8 as *const libc::c_char);
    if !slot_info_pairs.is_null() {
        if slot_pairs_count % 2 as libc::c_int == 0 as libc::c_int {} else {
            _serverAssert(
                b"(slot_pairs_count % 2) == 0\0" as *const u8 as *const libc::c_char,
                b"cluster.c\0" as *const u8 as *const libc::c_char,
                5101 as libc::c_int,
            );
            unreachable!();
        };
        addReplyArrayLen(c, slot_pairs_count as libc::c_long);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < slot_pairs_count {
            addReplyLongLong(
                c,
                *slot_info_pairs.offset(i as isize) as libc::c_ulong as libc::c_longlong,
            );
            i += 1;
        }
    } else {
        addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
    }
    addReplyBulkCString(c, b"nodes\0" as *const u8 as *const libc::c_char);
    let mut nodes_for_slot: *mut list = clusterGetNodesServingMySlots(node);
    if !nodes_for_slot.is_null() {} else {
        _serverAssert(
            b"nodes_for_slot\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            5113 as libc::c_int,
        );
        unreachable!();
    };
    addReplyArrayLen(c, (*nodes_for_slot).len as libc::c_long);
    if (*nodes_for_slot).len != 0 as libc::c_int as libc::c_ulong {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind(nodes_for_slot, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut node_0: *mut clusterNode = (*ln).value as *mut clusterNode;
            addNodeDetailsToShardReply(c, node_0);
        }
        listRelease(nodes_for_slot);
    }
}
#[no_mangle]
pub unsafe extern "C" fn clusterReplyShards(mut c: *mut client) {
    let mut shard_replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut shard_count: libc::c_int = 0 as libc::c_int;
    clusterGenNodesSlotsInfo(0 as libc::c_int);
    let mut di: *mut dictIterator = dictGetSafeIterator((*server.cluster).nodes);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut n: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*n).flags & 1 as libc::c_int == 0 {
            clusterFreeNodesSlotsInfo(n);
        } else {
            shard_count += 1;
            addShardReplyForClusterShards(
                c,
                n,
                (*n).slot_info_pairs,
                (*n).slot_info_pairs_count,
            );
            clusterFreeNodesSlotsInfo(n);
        }
    }
    dictReleaseIterator(di);
    setDeferredArrayLen(c, shard_replylen, shard_count as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn clusterReplyMultiBulkSlots(mut c: *mut client) {
    let mut n: *mut clusterNode = 0 as *mut clusterNode;
    let mut num_masters: libc::c_int = 0 as libc::c_int;
    let mut start: libc::c_int = -(1 as libc::c_int);
    let mut slot_replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 16384 as libc::c_int {
        if n.is_null() {
            if i == 16384 as libc::c_int {
                break;
            }
            n = (*server.cluster).slots[i as usize];
            start = i;
        } else if i == 16384 as libc::c_int || n != (*server.cluster).slots[i as usize] {
            addNodeReplyForClusterSlot(c, n, start, i - 1 as libc::c_int);
            num_masters += 1;
            if i == 16384 as libc::c_int {
                break;
            }
            n = (*server.cluster).slots[i as usize];
            start = i;
        }
        i += 1;
    }
    setDeferredArrayLen(c, slot_replylen, num_masters as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn clusterCommand(mut c: *mut client) {
    if server.cluster_enabled == 0 as libc::c_int {
        addReplyError(
            c,
            b"This instance has cluster support disabled\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        let mut help: [*const libc::c_char; 54] = [
            b"ADDSLOTS <slot> [<slot> ...]\0" as *const u8 as *const libc::c_char,
            b"    Assign slots to current node.\0" as *const u8 as *const libc::c_char,
            b"ADDSLOTSRANGE <start slot> <end slot> [<start slot> <end slot> ...]\0"
                as *const u8 as *const libc::c_char,
            b"    Assign slots which are between <start-slot> and <end-slot> to current node.\0"
                as *const u8 as *const libc::c_char,
            b"BUMPEPOCH\0" as *const u8 as *const libc::c_char,
            b"    Advance the cluster config epoch.\0" as *const u8
                as *const libc::c_char,
            b"COUNT-FAILURE-REPORTS <node-id>\0" as *const u8 as *const libc::c_char,
            b"    Return number of failure reports for <node-id>.\0" as *const u8
                as *const libc::c_char,
            b"COUNTKEYSINSLOT <slot>\0" as *const u8 as *const libc::c_char,
            b"    Return the number of keys in <slot>.\0" as *const u8
                as *const libc::c_char,
            b"DELSLOTS <slot> [<slot> ...]\0" as *const u8 as *const libc::c_char,
            b"    Delete slots information from current node.\0" as *const u8
                as *const libc::c_char,
            b"DELSLOTSRANGE <start slot> <end slot> [<start slot> <end slot> ...]\0"
                as *const u8 as *const libc::c_char,
            b"    Delete slots information which are between <start-slot> and <end-slot> from current node.\0"
                as *const u8 as *const libc::c_char,
            b"FAILOVER [FORCE|TAKEOVER]\0" as *const u8 as *const libc::c_char,
            b"    Promote current replica node to being a master.\0" as *const u8
                as *const libc::c_char,
            b"FORGET <node-id>\0" as *const u8 as *const libc::c_char,
            b"    Remove a node from the cluster.\0" as *const u8 as *const libc::c_char,
            b"GETKEYSINSLOT <slot> <count>\0" as *const u8 as *const libc::c_char,
            b"    Return key names stored by current node in a slot.\0" as *const u8
                as *const libc::c_char,
            b"FLUSHSLOTS\0" as *const u8 as *const libc::c_char,
            b"    Delete current node own slots information.\0" as *const u8
                as *const libc::c_char,
            b"INFO\0" as *const u8 as *const libc::c_char,
            b"    Return information about the cluster.\0" as *const u8
                as *const libc::c_char,
            b"KEYSLOT <key>\0" as *const u8 as *const libc::c_char,
            b"    Return the hash slot for <key>.\0" as *const u8 as *const libc::c_char,
            b"MEET <ip> <port> [<bus-port>]\0" as *const u8 as *const libc::c_char,
            b"    Connect nodes into a working cluster.\0" as *const u8
                as *const libc::c_char,
            b"MYID\0" as *const u8 as *const libc::c_char,
            b"    Return the node id.\0" as *const u8 as *const libc::c_char,
            b"NODES\0" as *const u8 as *const libc::c_char,
            b"    Return cluster configuration seen by node. Output format:\0"
                as *const u8 as *const libc::c_char,
            b"    <id> <ip:port> <flags> <master> <pings> <pongs> <epoch> <link> <slot> ...\0"
                as *const u8 as *const libc::c_char,
            b"REPLICATE <node-id>\0" as *const u8 as *const libc::c_char,
            b"    Configure current node as replica to <node-id>.\0" as *const u8
                as *const libc::c_char,
            b"RESET [HARD|SOFT]\0" as *const u8 as *const libc::c_char,
            b"    Reset current node (default: soft).\0" as *const u8
                as *const libc::c_char,
            b"SET-CONFIG-EPOCH <epoch>\0" as *const u8 as *const libc::c_char,
            b"    Set config epoch of current node.\0" as *const u8
                as *const libc::c_char,
            b"SETSLOT <slot> (IMPORTING <node-id>|MIGRATING <node-id>|STABLE|NODE <node-id>)\0"
                as *const u8 as *const libc::c_char,
            b"    Set slot state.\0" as *const u8 as *const libc::c_char,
            b"REPLICAS <node-id>\0" as *const u8 as *const libc::c_char,
            b"    Return <node-id> replicas.\0" as *const u8 as *const libc::c_char,
            b"SAVECONFIG\0" as *const u8 as *const libc::c_char,
            b"    Force saving cluster configuration on disk.\0" as *const u8
                as *const libc::c_char,
            b"SLOTS\0" as *const u8 as *const libc::c_char,
            b"    Return information about slots range mappings. Each range is made of:\0"
                as *const u8 as *const libc::c_char,
            b"    start, end, master and replicas IP addresses, ports and ids\0"
                as *const u8 as *const libc::c_char,
            b"SHARDS\0" as *const u8 as *const libc::c_char,
            b"    Return information about slot range mappings and the nodes associated with them.\0"
                as *const u8 as *const libc::c_char,
            b"LINKS\0" as *const u8 as *const libc::c_char,
            b"    Return information about all network links between this node and its peers.\0"
                as *const u8 as *const libc::c_char,
            b"    Output format is an array where each array element is a map containing attributes of a link\0"
                as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"meet\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 4 as libc::c_int || (*c).argc == 5 as libc::c_int)
    {
        let mut port: libc::c_longlong = 0;
        let mut cport: libc::c_longlong = 0;
        if getLongLongFromObject(
            *((*c).argv).offset(3 as libc::c_int as isize),
            &mut port,
        ) != 0 as libc::c_int
        {
            addReplyErrorFormat(
                c,
                b"Invalid TCP base port specified: %s\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        }
        if (*c).argc == 5 as libc::c_int {
            if getLongLongFromObject(
                *((*c).argv).offset(4 as libc::c_int as isize),
                &mut cport,
            ) != 0 as libc::c_int
            {
                addReplyErrorFormat(
                    c,
                    b"Invalid TCP bus port specified: %s\0" as *const u8
                        as *const libc::c_char,
                    (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
        } else {
            cport = port + 10000 as libc::c_int as libc::c_longlong;
        }
        if clusterStartHandshake(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *mut libc::c_char,
            port as libc::c_int,
            cport as libc::c_int,
        ) == 0 as libc::c_int && *__errno_location() == 22 as libc::c_int
        {
            addReplyErrorFormat(
                c,
                b"Invalid node address specified: %s:%s\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
                (**((*c).argv).offset(3 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
        } else {
            addReply(c, shared.ok);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"nodes\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut use_pport: libc::c_int = (server.tls_cluster != 0
            && !((*c).conn).is_null() && connGetType((*c).conn) != 2 as libc::c_int)
            as libc::c_int;
        let mut nodes: sds = clusterGenNodesDescription(0 as libc::c_int, use_pport);
        addReplyVerbatim(
            c,
            nodes as *const libc::c_char,
            sdslen(nodes),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(nodes);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"myid\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        addReplyBulkCBuffer(
            c,
            ((*myself).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as size_t,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"slots\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        clusterReplyMultiBulkSlots(c);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"shards\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        clusterReplyShards(c);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"flushslots\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        if ((*(*(server.db).offset(0 as libc::c_int as isize)).dict)
            .ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*(*(server.db).offset(0 as libc::c_int as isize)).dict)
                    .ht_used[1 as libc::c_int as usize],
            ) != 0 as libc::c_int as libc::c_ulong
        {
            addReplyError(
                c,
                b"DB must be empty to perform CLUSTER FLUSHSLOTS.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        clusterDelNodeSlots(myself);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if (strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"addslots\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"delslots\0" as *const u8 as *const libc::c_char,
        ) == 0) && (*c).argc >= 3 as libc::c_int
    {
        let mut j: libc::c_int = 0;
        let mut slot: libc::c_int = 0;
        let mut slots: *mut libc::c_uchar = zmalloc(16384 as libc::c_int as size_t)
            as *mut libc::c_uchar;
        let mut del: libc::c_int = (strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"delslots\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
        memset(
            slots as *mut libc::c_void,
            0 as libc::c_int,
            16384 as libc::c_int as libc::c_ulong,
        );
        j = 2 as libc::c_int;
        while j < (*c).argc {
            slot = getSlotOrReply(c, *((*c).argv).offset(j as isize));
            if slot == -(1 as libc::c_int) {
                zfree(slots as *mut libc::c_void);
                return;
            }
            j += 1;
        }
        j = 2 as libc::c_int;
        while j < (*c).argc {
            slot = getSlotOrReply(c, *((*c).argv).offset(j as isize));
            if checkSlotAssignmentsOrReply(c, slots, del, slot, slot)
                == -(1 as libc::c_int)
            {
                zfree(slots as *mut libc::c_void);
                return;
            }
            j += 1;
        }
        clusterUpdateSlots(c, slots, del);
        zfree(slots as *mut libc::c_void);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if (strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"addslotsrange\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"delslotsrange\0" as *const u8 as *const libc::c_char,
        ) == 0) && (*c).argc >= 4 as libc::c_int
    {
        if (*c).argc % 2 as libc::c_int == 1 as libc::c_int {
            addReplyErrorArity(c);
            return;
        }
        let mut j_0: libc::c_int = 0;
        let mut startslot: libc::c_int = 0;
        let mut endslot: libc::c_int = 0;
        let mut slots_0: *mut libc::c_uchar = zmalloc(16384 as libc::c_int as size_t)
            as *mut libc::c_uchar;
        let mut del_0: libc::c_int = (strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"delslotsrange\0" as *const u8 as *const libc::c_char,
        ) == 0) as libc::c_int;
        memset(
            slots_0 as *mut libc::c_void,
            0 as libc::c_int,
            16384 as libc::c_int as libc::c_ulong,
        );
        j_0 = 2 as libc::c_int;
        while j_0 < (*c).argc {
            startslot = getSlotOrReply(c, *((*c).argv).offset(j_0 as isize));
            if startslot == -(1 as libc::c_int) {
                zfree(slots_0 as *mut libc::c_void);
                return;
            }
            endslot = getSlotOrReply(
                c,
                *((*c).argv).offset((j_0 + 1 as libc::c_int) as isize),
            );
            if endslot == -(1 as libc::c_int) {
                zfree(slots_0 as *mut libc::c_void);
                return;
            }
            if startslot > endslot {
                addReplyErrorFormat(
                    c,
                    b"start slot number %d is greater than end slot number %d\0"
                        as *const u8 as *const libc::c_char,
                    startslot,
                    endslot,
                );
                zfree(slots_0 as *mut libc::c_void);
                return;
            }
            if checkSlotAssignmentsOrReply(c, slots_0, del_0, startslot, endslot)
                == -(1 as libc::c_int)
            {
                zfree(slots_0 as *mut libc::c_void);
                return;
            }
            j_0 += 2 as libc::c_int;
        }
        clusterUpdateSlots(c, slots_0, del_0);
        zfree(slots_0 as *mut libc::c_void);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"setslot\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 4 as libc::c_int
    {
        let mut slot_0: libc::c_int = 0;
        let mut n: *mut clusterNode = 0 as *mut clusterNode;
        if (*myself).flags & 2 as libc::c_int != 0 {
            addReplyError(
                c,
                b"Please use SETSLOT only with masters.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        slot_0 = getSlotOrReply(c, *((*c).argv).offset(2 as libc::c_int as isize));
        if slot_0 == -(1 as libc::c_int) {
            return;
        }
        if strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"migrating\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc == 5 as libc::c_int
        {
            if (*server.cluster).slots[slot_0 as usize] != myself {
                addReplyErrorFormat(
                    c,
                    b"I'm not the owner of hash slot %u\0" as *const u8
                        as *const libc::c_char,
                    slot_0,
                );
                return;
            }
            n = clusterLookupNode(
                (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                sdslen((**((*c).argv).offset(4 as libc::c_int as isize)).ptr as sds)
                    as libc::c_int,
            );
            if n.is_null() {
                addReplyErrorFormat(
                    c,
                    b"I don't know about node %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
            if (*n).flags & 2 as libc::c_int != 0 {
                addReplyError(
                    c,
                    b"Target node is not a master\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            (*server.cluster).migrating_slots_to[slot_0 as usize] = n;
        } else if strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"importing\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc == 5 as libc::c_int
        {
            if (*server.cluster).slots[slot_0 as usize] == myself {
                addReplyErrorFormat(
                    c,
                    b"I'm already the owner of hash slot %u\0" as *const u8
                        as *const libc::c_char,
                    slot_0,
                );
                return;
            }
            n = clusterLookupNode(
                (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                sdslen((**((*c).argv).offset(4 as libc::c_int as isize)).ptr as sds)
                    as libc::c_int,
            );
            if n.is_null() {
                addReplyErrorFormat(
                    c,
                    b"I don't know about node %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
            if (*n).flags & 2 as libc::c_int != 0 {
                addReplyError(
                    c,
                    b"Target node is not a master\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            (*server.cluster).importing_slots_from[slot_0 as usize] = n;
        } else if strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"stable\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc == 4 as libc::c_int
        {
            (*server.cluster)
                .importing_slots_from[slot_0 as usize] = 0 as *mut clusterNode;
            (*server.cluster)
                .migrating_slots_to[slot_0 as usize] = 0 as *mut clusterNode;
        } else if strcasecmp(
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"node\0" as *const u8 as *const libc::c_char,
        ) == 0 && (*c).argc == 5 as libc::c_int
        {
            n = clusterLookupNode(
                (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                sdslen((**((*c).argv).offset(4 as libc::c_int as isize)).ptr as sds)
                    as libc::c_int,
            );
            if n.is_null() {
                addReplyErrorFormat(
                    c,
                    b"Unknown node %s\0" as *const u8 as *const libc::c_char,
                    (**((*c).argv).offset(4 as libc::c_int as isize)).ptr
                        as *mut libc::c_char,
                );
                return;
            }
            if (*n).flags & 2 as libc::c_int != 0 {
                addReplyError(
                    c,
                    b"Target node is not a master\0" as *const u8 as *const libc::c_char,
                );
                return;
            }
            if (*server.cluster).slots[slot_0 as usize] == myself && n != myself {
                if countKeysInSlot(slot_0 as libc::c_uint)
                    != 0 as libc::c_int as libc::c_uint
                {
                    addReplyErrorFormat(
                        c,
                        b"Can't assign hashslot %d to a different node while I still hold keys for this hash slot.\0"
                            as *const u8 as *const libc::c_char,
                        slot_0,
                    );
                    return;
                }
            }
            if countKeysInSlot(slot_0 as libc::c_uint)
                == 0 as libc::c_int as libc::c_uint
                && !((*server.cluster).migrating_slots_to[slot_0 as usize]).is_null()
            {
                (*server.cluster)
                    .migrating_slots_to[slot_0 as usize] = 0 as *mut clusterNode;
            }
            let mut slot_was_mine: libc::c_int = ((*server.cluster)
                .slots[slot_0 as usize] == myself) as libc::c_int;
            clusterDelSlot(slot_0);
            clusterAddSlot(n, slot_0);
            if slot_was_mine != 0 && n != myself
                && (*myself).numslots == 0 as libc::c_int
                && server.cluster_allow_replica_migration != 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Configuration change detected. Reconfiguring myself as a replica of %.40s\0"
                            as *const u8 as *const libc::c_char,
                        ((*n).name).as_mut_ptr(),
                    );
                }
                clusterSetMaster(n);
                clusterDoBeforeSleep(
                    (1 as libc::c_int) << 2 as libc::c_int
                        | (1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 3 as libc::c_int,
                );
            }
            if n == myself
                && !((*server.cluster).importing_slots_from[slot_0 as usize]).is_null()
            {
                if clusterBumpConfigEpochWithoutConsensus() == 0 as libc::c_int {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"configEpoch updated after importing slot %d\0" as *const u8
                                as *const libc::c_char,
                            slot_0,
                        );
                    }
                }
                (*server.cluster)
                    .importing_slots_from[slot_0 as usize] = 0 as *mut clusterNode;
                clusterBroadcastPong(0 as libc::c_int);
            }
        } else {
            addReplyError(
                c,
                b"Invalid CLUSTER SETSLOT action or number of arguments. Try CLUSTER HELP\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"bumpepoch\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut retval: libc::c_int = clusterBumpConfigEpochWithoutConsensus();
        let mut reply: sds = sdscatprintf(
            sdsempty(),
            b"+%s %llu\r\n\0" as *const u8 as *const libc::c_char,
            if retval == 0 as libc::c_int {
                b"BUMPED\0" as *const u8 as *const libc::c_char
            } else {
                b"STILL\0" as *const u8 as *const libc::c_char
            },
            (*myself).configEpoch as libc::c_ulonglong,
        );
        addReplySds(c, reply);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"info\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut statestr: [*mut libc::c_char; 2] = [
            b"ok\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            b"fail\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ];
        let mut slots_assigned: libc::c_int = 0 as libc::c_int;
        let mut slots_ok: libc::c_int = 0 as libc::c_int;
        let mut slots_pfail: libc::c_int = 0 as libc::c_int;
        let mut slots_fail: libc::c_int = 0 as libc::c_int;
        let mut myepoch: uint64_t = 0;
        let mut j_1: libc::c_int = 0;
        j_1 = 0 as libc::c_int;
        while j_1 < 16384 as libc::c_int {
            let mut n_0: *mut clusterNode = (*server.cluster).slots[j_1 as usize];
            if !n_0.is_null() {
                slots_assigned += 1;
                if (*n_0).flags & 8 as libc::c_int != 0 {
                    slots_fail += 1;
                } else if (*n_0).flags & 4 as libc::c_int != 0 {
                    slots_pfail += 1;
                } else {
                    slots_ok += 1;
                }
            }
            j_1 += 1;
        }
        myepoch = if (*myself).flags & 2 as libc::c_int != 0
            && !((*myself).slaveof).is_null()
        {
            (*(*myself).slaveof).configEpoch
        } else {
            (*myself).configEpoch
        };
        let mut info: sds = sdscatprintf(
            sdsempty(),
            b"cluster_state:%s\r\ncluster_slots_assigned:%d\r\ncluster_slots_ok:%d\r\ncluster_slots_pfail:%d\r\ncluster_slots_fail:%d\r\ncluster_known_nodes:%lu\r\ncluster_size:%d\r\ncluster_current_epoch:%llu\r\ncluster_my_epoch:%llu\r\n\0"
                as *const u8 as *const libc::c_char,
            statestr[(*server.cluster).state as usize],
            slots_assigned,
            slots_ok,
            slots_pfail,
            slots_fail,
            ((*(*server.cluster).nodes).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize],
                ),
            (*server.cluster).size,
            (*server.cluster).currentEpoch as libc::c_ulonglong,
            myepoch as libc::c_ulonglong,
        );
        let mut tot_msg_sent: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut tot_msg_received: libc::c_longlong = 0 as libc::c_int
            as libc::c_longlong;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 11 as libc::c_int {
            if !((*server.cluster).stats_bus_messages_sent[i as usize]
                == 0 as libc::c_int as libc::c_longlong)
            {
                tot_msg_sent += (*server.cluster).stats_bus_messages_sent[i as usize];
                info = sdscatprintf(
                    info,
                    b"cluster_stats_messages_%s_sent:%lld\r\n\0" as *const u8
                        as *const libc::c_char,
                    clusterGetMessageTypeString(i),
                    (*server.cluster).stats_bus_messages_sent[i as usize],
                );
            }
            i += 1;
        }
        info = sdscatprintf(
            info,
            b"cluster_stats_messages_sent:%lld\r\n\0" as *const u8
                as *const libc::c_char,
            tot_msg_sent,
        );
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < 11 as libc::c_int {
            if !((*server.cluster).stats_bus_messages_received[i_0 as usize]
                == 0 as libc::c_int as libc::c_longlong)
            {
                tot_msg_received
                    += (*server.cluster).stats_bus_messages_received[i_0 as usize];
                info = sdscatprintf(
                    info,
                    b"cluster_stats_messages_%s_received:%lld\r\n\0" as *const u8
                        as *const libc::c_char,
                    clusterGetMessageTypeString(i_0),
                    (*server.cluster).stats_bus_messages_received[i_0 as usize],
                );
            }
            i_0 += 1;
        }
        info = sdscatprintf(
            info,
            b"cluster_stats_messages_received:%lld\r\n\0" as *const u8
                as *const libc::c_char,
            tot_msg_received,
        );
        info = sdscatprintf(
            info,
            b"total_cluster_links_buffer_limit_exceeded:%llu\r\n\0" as *const u8
                as *const libc::c_char,
            (*server.cluster).stat_cluster_links_buffer_limit_exceeded,
        );
        addReplyVerbatim(
            c,
            info as *const libc::c_char,
            sdslen(info),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(info);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"saveconfig\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut retval_0: libc::c_int = clusterSaveConfig(1 as libc::c_int);
        if retval_0 == 0 as libc::c_int {
            addReply(c, shared.ok);
        } else {
            addReplyErrorFormat(
                c,
                b"error saving the cluster node config: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"keyslot\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut key: sds = (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds;
        addReplyLongLong(
            c,
            keyHashSlot(key, sdslen(key) as libc::c_int) as libc::c_longlong,
        );
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"countkeysinslot\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut slot_1: libc::c_longlong = 0;
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut slot_1,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if slot_1 < 0 as libc::c_int as libc::c_longlong
            || slot_1 >= 16384 as libc::c_int as libc::c_longlong
        {
            addReplyError(c, b"Invalid slot\0" as *const u8 as *const libc::c_char);
            return;
        }
        addReplyLongLong(c, countKeysInSlot(slot_1 as libc::c_uint) as libc::c_longlong);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"getkeysinslot\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 4 as libc::c_int
    {
        let mut maxkeys: libc::c_longlong = 0;
        let mut slot_2: libc::c_longlong = 0;
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut slot_2,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(3 as libc::c_int as isize),
            &mut maxkeys,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if slot_2 < 0 as libc::c_int as libc::c_longlong
            || slot_2 >= 16384 as libc::c_int as libc::c_longlong
            || maxkeys < 0 as libc::c_int as libc::c_longlong
        {
            addReplyError(
                c,
                b"Invalid slot or number of keys\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        let mut keys_in_slot: libc::c_uint = countKeysInSlot(slot_2 as libc::c_uint);
        let mut numkeys: libc::c_uint = (if maxkeys > keys_in_slot as libc::c_longlong {
            keys_in_slot as libc::c_longlong
        } else {
            maxkeys
        }) as libc::c_uint;
        addReplyArrayLen(c, numkeys as libc::c_long);
        let mut de: *mut dictEntry = (*(*server.db).slots_to_keys)
            .by_slot[slot_2 as usize]
            .head;
        let mut j_2: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        while j_2 < numkeys {
            if !de.is_null() {} else {
                _serverAssert(
                    b"de != NULL\0" as *const u8 as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    5641 as libc::c_int,
                );
                unreachable!();
            };
            let mut sdskey: sds = (*de).key as sds;
            addReplyBulkCBuffer(c, sdskey as *const libc::c_void, sdslen(sdskey));
            de = (*(&mut (*de).metadata as *mut [*mut libc::c_void; 0]
                as *mut clusterDictEntryMetadata))
                .next;
            j_2 = j_2.wrapping_add(1);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"forget\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut n_1: *mut clusterNode = clusterLookupNode(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
                as libc::c_int,
        );
        if n_1.is_null() {
            addReplyErrorFormat(
                c,
                b"Unknown node %s\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        } else {
            if n_1 == myself {
                addReplyError(
                    c,
                    b"I tried hard but I can't forget myself...\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            } else {
                if (*myself).flags & 2 as libc::c_int != 0 && (*myself).slaveof == n_1 {
                    addReplyError(
                        c,
                        b"Can't forget my master!\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
        }
        clusterBlacklistAddNode(n_1);
        clusterDelNode(n_1);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"replicate\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut n_2: *mut clusterNode = clusterLookupNode(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
                as libc::c_int,
        );
        if n_2.is_null() {
            addReplyErrorFormat(
                c,
                b"Unknown node %s\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        }
        if n_2 == myself {
            addReplyError(
                c,
                b"Can't replicate myself\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
        if (*n_2).flags & 2 as libc::c_int != 0 {
            addReplyError(
                c,
                b"I can only replicate a master, not a replica.\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if (*myself).flags & 1 as libc::c_int != 0
            && ((*myself).numslots != 0 as libc::c_int
                || ((*(*(server.db).offset(0 as libc::c_int as isize)).dict)
                    .ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*(*(server.db).offset(0 as libc::c_int as isize)).dict)
                            .ht_used[1 as libc::c_int as usize],
                    ) != 0 as libc::c_int as libc::c_ulong)
        {
            addReplyError(
                c,
                b"To set a master the node must be empty and without assigned slots.\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        clusterSetMaster(n_2);
        clusterDoBeforeSleep(
            (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int,
        );
        addReply(c, shared.ok);
    } else if (strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"slaves\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"replicas\0" as *const u8 as *const libc::c_char,
        ) == 0) && (*c).argc == 3 as libc::c_int
    {
        let mut n_3: *mut clusterNode = clusterLookupNode(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
                as libc::c_int,
        );
        let mut j_3: libc::c_int = 0;
        if n_3.is_null() {
            addReplyErrorFormat(
                c,
                b"Unknown node %s\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        }
        if (*n_3).flags & 2 as libc::c_int != 0 {
            addReplyError(
                c,
                b"The specified node is not a master\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        let mut use_pport_0: libc::c_int = (server.tls_cluster != 0
            && !((*c).conn).is_null() && connGetType((*c).conn) != 2 as libc::c_int)
            as libc::c_int;
        addReplyArrayLen(c, (*n_3).numslaves as libc::c_long);
        j_3 = 0 as libc::c_int;
        while j_3 < (*n_3).numslaves {
            let mut ni: sds = clusterGenNodeDescription(
                *((*n_3).slaves).offset(j_3 as isize),
                use_pport_0,
            );
            addReplyBulkCString(c, ni as *const libc::c_char);
            sdsfree(ni);
            j_3 += 1;
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"count-failure-reports\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut n_4: *mut clusterNode = clusterLookupNode(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            sdslen((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
                as libc::c_int,
        );
        if n_4.is_null() {
            addReplyErrorFormat(
                c,
                b"Unknown node %s\0" as *const u8 as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        } else {
            addReplyLongLong(c, clusterNodeFailureReportsCount(n_4) as libc::c_longlong);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"failover\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
    {
        let mut force: libc::c_int = 0 as libc::c_int;
        let mut takeover: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 3 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"force\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                force = 1 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"takeover\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                takeover = 1 as libc::c_int;
                force = 1 as libc::c_int;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
        }
        if (*myself).flags & 1 as libc::c_int != 0 {
            addReplyError(
                c,
                b"You should send CLUSTER FAILOVER to a replica\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        } else {
            if ((*myself).slaveof).is_null() {
                addReplyError(
                    c,
                    b"I'm a replica but my master is unknown to me\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            } else {
                if force == 0
                    && ((*(*myself).slaveof).flags & 8 as libc::c_int != 0
                        || ((*(*myself).slaveof).link).is_null())
                {
                    addReplyError(
                        c,
                        b"Master is down or failed, please use CLUSTER FAILOVER FORCE\0"
                            as *const u8 as *const libc::c_char,
                    );
                    return;
                }
            }
        }
        resetManualFailover();
        (*server.cluster).mf_end = mstime() + 5000 as libc::c_int as libc::c_longlong;
        if takeover != 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Taking over the master (user request).\0" as *const u8
                        as *const libc::c_char,
                );
            }
            clusterBumpConfigEpochWithoutConsensus();
            clusterFailoverReplaceYourMaster();
        } else if force != 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Forced failover user request accepted.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            (*server.cluster).mf_can_start = 1 as libc::c_int;
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Manual failover user request accepted.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            clusterSendMFStart((*myself).slaveof);
        }
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"set-config-epoch\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut epoch: libc::c_longlong = 0;
        if getLongLongFromObjectOrReply(
            c,
            *((*c).argv).offset(2 as libc::c_int as isize),
            &mut epoch,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return;
        }
        if epoch < 0 as libc::c_int as libc::c_longlong {
            addReplyErrorFormat(
                c,
                b"Invalid config epoch specified: %lld\0" as *const u8
                    as *const libc::c_char,
                epoch,
            );
        } else if ((*(*server.cluster).nodes).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize])
            > 1 as libc::c_int as libc::c_ulong
        {
            addReplyError(
                c,
                b"The user can assign a config epoch only when the node does not know any other node.\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if (*myself).configEpoch != 0 as libc::c_int as libc::c_ulong {
            addReplyError(
                c,
                b"Node config epoch is already non-zero\0" as *const u8
                    as *const libc::c_char,
            );
        } else {
            (*myself).configEpoch = epoch as uint64_t;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"configEpoch set to %llu via CLUSTER SET-CONFIG-EPOCH\0"
                        as *const u8 as *const libc::c_char,
                    (*myself).configEpoch as libc::c_ulonglong,
                );
            }
            if (*server.cluster).currentEpoch < epoch as uint64_t {
                (*server.cluster).currentEpoch = epoch as uint64_t;
            }
            clusterDoBeforeSleep(
                (1 as libc::c_int) << 1 as libc::c_int
                    | (1 as libc::c_int) << 2 as libc::c_int,
            );
            addReply(c, shared.ok);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"reset\0" as *const u8 as *const libc::c_char,
    ) == 0 && ((*c).argc == 2 as libc::c_int || (*c).argc == 3 as libc::c_int)
    {
        let mut hard: libc::c_int = 0 as libc::c_int;
        if (*c).argc == 3 as libc::c_int {
            if strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"hard\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                hard = 1 as libc::c_int;
            } else if strcasecmp(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                b"soft\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                hard = 0 as libc::c_int;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
        }
        if (*myself).flags & 1 as libc::c_int != 0
            && ((*(*(*c).db).dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*(*(*c).db).dict).ht_used[1 as libc::c_int as usize])
                != 0 as libc::c_int as libc::c_ulong
        {
            addReplyError(
                c,
                b"CLUSTER RESET can't be called with master nodes containing keys\0"
                    as *const u8 as *const libc::c_char,
            );
            return;
        }
        clusterReset(hard);
        addReply(c, shared.ok);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"links\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        addReplyClusterLinksDescription(c);
    } else {
        addReplySubcommandSyntaxError(c);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn removeChannelsInSlot(mut slot: libc::c_uint) {
    let mut channelcount: libc::c_uint = countChannelsInSlot(slot);
    if channelcount == 0 as libc::c_int as libc::c_uint {
        return;
    }
    let mut channels: *mut *mut robj = zmalloc(
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(channelcount as libc::c_ulong),
    ) as *mut *mut robj;
    let mut iter: raxIterator = raxIterator {
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
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut indexed: [libc::c_uchar; 2] = [0; 2];
    indexed[0 as libc::c_int
        as usize] = (slot >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    indexed[1 as libc::c_int
        as usize] = (slot & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    raxStart(&mut iter, (*server.cluster).slots_to_channels);
    raxSeek(
        &mut iter,
        b">=\0" as *const u8 as *const libc::c_char,
        indexed.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    while raxNext(&mut iter) != 0 {
        if *(iter.key).offset(0 as libc::c_int as isize) as libc::c_int
            != indexed[0 as libc::c_int as usize] as libc::c_int
            || *(iter.key).offset(1 as libc::c_int as isize) as libc::c_int
                != indexed[1 as libc::c_int as usize] as libc::c_int
        {
            break;
        }
        let fresh16 = j;
        j = j + 1;
        let ref mut fresh17 = *channels.offset(fresh16 as isize);
        *fresh17 = createStringObject(
            (iter.key as *mut libc::c_char).offset(2 as libc::c_int as isize),
            (iter.key_len).wrapping_sub(2 as libc::c_int as libc::c_ulong),
        );
    }
    raxStop(&mut iter);
    pubsubUnsubscribeShardChannels(channels, channelcount);
    zfree(channels as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn createDumpPayload(
    mut payload: *mut rio,
    mut o: *mut robj,
    mut key: *mut robj,
    mut dbid: libc::c_int,
) {
    let mut buf: [libc::c_uchar; 2] = [0; 2];
    let mut crc: uint64_t = 0;
    rioInitWithBuffer(payload, sdsempty());
    if rdbSaveObjectType(payload, o) != 0 {} else {
        _serverAssert(
            b"rdbSaveObjectType(payload,o)\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            5902 as libc::c_int,
        );
        unreachable!();
    };
    if rdbSaveObject(payload, o, key, dbid) != 0 {} else {
        _serverAssert(
            b"rdbSaveObject(payload,o,key,dbid)\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            5903 as libc::c_int,
        );
        unreachable!();
    };
    buf[0 as libc::c_int
        as usize] = (10 as libc::c_int & 0xff as libc::c_int) as libc::c_uchar;
    buf[1 as libc::c_int
        as usize] = (10 as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int)
        as libc::c_uchar;
    (*payload)
        .io
        .buffer
        .ptr = sdscatlen(
        (*payload).io.buffer.ptr,
        buf.as_mut_ptr() as *const libc::c_void,
        2 as libc::c_int as size_t,
    );
    crc = crc64(
        0 as libc::c_int as uint64_t,
        (*payload).io.buffer.ptr as *mut libc::c_uchar,
        sdslen((*payload).io.buffer.ptr),
    );
    (*payload)
        .io
        .buffer
        .ptr = sdscatlen(
        (*payload).io.buffer.ptr,
        &mut crc as *mut uint64_t as *const libc::c_void,
        8 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn verifyDumpPayload(
    mut p: *mut libc::c_uchar,
    mut len: size_t,
    mut rdbver_ptr: *mut uint16_t,
) -> libc::c_int {
    let mut footer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut rdbver: uint16_t = 0;
    let mut crc: uint64_t = 0;
    if len < 10 as libc::c_int as libc::c_ulong {
        return -(1 as libc::c_int);
    }
    footer = p.offset(len.wrapping_sub(10 as libc::c_int as libc::c_ulong) as isize);
    rdbver = ((*footer.offset(1 as libc::c_int as isize) as libc::c_int)
        << 8 as libc::c_int | *footer.offset(0 as libc::c_int as isize) as libc::c_int)
        as uint16_t;
    if !rdbver_ptr.is_null() {
        *rdbver_ptr = rdbver;
    }
    if rdbver as libc::c_int > 10 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if server.skip_checksum_validation != 0 {
        return 0 as libc::c_int;
    }
    crc = crc64(
        0 as libc::c_int as uint64_t,
        p,
        len.wrapping_sub(8 as libc::c_int as libc::c_ulong),
    );
    return if memcmp(
        &mut crc as *mut uint64_t as *const libc::c_void,
        footer.offset(2 as libc::c_int as isize) as *const libc::c_void,
        8 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn dumpCommand(mut c: *mut client) {
    let mut o: *mut robj = 0 as *mut robj;
    let mut payload: rio = rio {
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
    o = lookupKeyRead((*c).db, *((*c).argv).offset(1 as libc::c_int as isize));
    if o.is_null() {
        addReplyNull(c);
        return;
    }
    createDumpPayload(
        &mut payload,
        o,
        *((*c).argv).offset(1 as libc::c_int as isize),
        (*(*c).db).id,
    );
    addReplyBulkSds(c, payload.io.buffer.ptr);
}
#[no_mangle]
pub unsafe extern "C" fn restoreCommand(mut c: *mut client) {
    let mut ttl: libc::c_longlong = 0;
    let mut lfu_freq: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut lru_idle: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut lru_clock: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
    let mut payload: rio = rio {
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
    let mut j: libc::c_int = 0;
    let mut type_0: libc::c_int = 0;
    let mut replace: libc::c_int = 0 as libc::c_int;
    let mut absttl: libc::c_int = 0 as libc::c_int;
    let mut obj: *mut robj = 0 as *mut robj;
    j = 4 as libc::c_int;
    while j < (*c).argc {
        let mut additional: libc::c_int = (*c).argc - j - 1 as libc::c_int;
        if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            replace = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"absttl\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            absttl = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"idletime\0" as *const u8 as *const libc::c_char,
        ) == 0 && additional >= 1 as libc::c_int
            && lfu_freq == -(1 as libc::c_int) as libc::c_longlong
        {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut lru_idle,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if lru_idle < 0 as libc::c_int as libc::c_longlong {
                addReplyError(
                    c,
                    b"Invalid IDLETIME value, must be >= 0\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            lru_clock = LRU_CLOCK() as libc::c_longlong;
            j += 1;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"freq\0" as *const u8 as *const libc::c_char,
        ) == 0 && additional >= 1 as libc::c_int
            && lru_idle == -(1 as libc::c_int) as libc::c_longlong
        {
            if getLongLongFromObjectOrReply(
                c,
                *((*c).argv).offset((j + 1 as libc::c_int) as isize),
                &mut lfu_freq,
                0 as *const libc::c_char,
            ) != 0 as libc::c_int
            {
                return;
            }
            if lfu_freq < 0 as libc::c_int as libc::c_longlong
                || lfu_freq > 255 as libc::c_int as libc::c_longlong
            {
                addReplyError(
                    c,
                    b"Invalid FREQ value, must be >= 0 and <= 255\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
            j += 1;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    let mut key: *mut robj = *((*c).argv).offset(1 as libc::c_int as isize);
    if replace == 0 && !(lookupKeyWrite((*c).db, key)).is_null() {
        addReplyErrorObject(c, shared.busykeyerr);
        return;
    }
    if getLongLongFromObjectOrReply(
        c,
        *((*c).argv).offset(2 as libc::c_int as isize),
        &mut ttl,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return
    } else {
        if ttl < 0 as libc::c_int as libc::c_longlong {
            addReplyError(
                c,
                b"Invalid TTL value, must be >= 0\0" as *const u8 as *const libc::c_char,
            );
            return;
        }
    }
    if verifyDumpPayload(
        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as *mut libc::c_uchar,
        sdslen((**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds),
        0 as *mut uint16_t,
    ) == -(1 as libc::c_int)
    {
        addReplyError(
            c,
            b"DUMP payload version or checksum are wrong\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    rioInitWithBuffer(
        &mut payload,
        (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds,
    );
    type_0 = rdbLoadObjectType(&mut payload);
    if type_0 == -(1 as libc::c_int)
        || {
            obj = rdbLoadObject(
                type_0,
                &mut payload,
                (*key).ptr as sds,
                (*(*c).db).id,
                0 as *mut libc::c_int,
            );
            obj.is_null()
        }
    {
        addReplyError(c, b"Bad data format\0" as *const u8 as *const libc::c_char);
        return;
    }
    let mut deleted: libc::c_int = 0 as libc::c_int;
    if replace != 0 {
        deleted = dbDelete((*c).db, key);
    }
    if ttl != 0 && absttl == 0 {
        ttl += mstime();
    }
    if ttl != 0 && checkAlreadyExpired(ttl) != 0 {
        if deleted != 0 {
            rewriteClientCommandVector(c, 2 as libc::c_int, shared.del, key);
            signalModifiedKey(c, (*c).db, key);
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                key,
                (*(*c).db).id,
            );
            server.dirty += 1;
        }
        decrRefCount(obj);
        addReply(c, shared.ok);
        return;
    }
    dbAdd((*c).db, key, obj);
    if ttl != 0 {
        setExpire(c, (*c).db, key, ttl);
        if absttl == 0 {
            let mut ttl_obj: *mut robj = createStringObjectFromLongLong(ttl);
            rewriteClientCommandArgument(c, 2 as libc::c_int, ttl_obj);
            decrRefCount(ttl_obj);
            rewriteClientCommandArgument(c, (*c).argc, shared.absttl);
        }
    }
    objectSetLRUOrLFU(obj, lfu_freq, lru_idle, lru_clock, 1000 as libc::c_int);
    signalModifiedKey(c, (*c).db, key);
    notifyKeyspaceEvent(
        (1 as libc::c_int) << 2 as libc::c_int,
        b"restore\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        key,
        (*(*c).db).id,
    );
    addReply(c, shared.ok);
    server.dirty += 1;
}
#[no_mangle]
pub unsafe extern "C" fn migrateGetSocket(
    mut c: *mut client,
    mut host: *mut robj,
    mut port: *mut robj,
    mut timeout: libc::c_long,
) -> *mut migrateCachedSocket {
    let mut conn: *mut connection = 0 as *mut connection;
    let mut name: sds = sdsempty();
    let mut cs: *mut migrateCachedSocket = 0 as *mut migrateCachedSocket;
    name = sdscatlen(name, (*host).ptr, sdslen((*host).ptr as sds));
    name = sdscatlen(
        name,
        b":\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    name = sdscatlen(name, (*port).ptr, sdslen((*port).ptr as sds));
    cs = dictFetchValue(server.migrate_cached_sockets, name as *const libc::c_void)
        as *mut migrateCachedSocket;
    if !cs.is_null() {
        sdsfree(name);
        (*cs).last_use_time = server.unixtime as time_t;
        return cs;
    }
    if ((*server.migrate_cached_sockets).ht_used[0 as libc::c_int as usize])
        .wrapping_add(
            (*server.migrate_cached_sockets).ht_used[1 as libc::c_int as usize],
        ) == 64 as libc::c_int as libc::c_ulong
    {
        let mut de: *mut dictEntry = dictGetRandomKey(server.migrate_cached_sockets);
        cs = (*de).v.val as *mut migrateCachedSocket;
        connClose((*cs).conn);
        zfree(cs as *mut libc::c_void);
        dictDelete(server.migrate_cached_sockets, (*de).key);
    }
    conn = if server.tls_cluster != 0 { connCreateTLS() } else { connCreateSocket() };
    if connBlockingConnect(
        conn,
        (*host).ptr as *const libc::c_char,
        atoi((*port).ptr as *const libc::c_char),
        timeout as libc::c_longlong,
    ) != 0 as libc::c_int
    {
        addReplyError(
            c,
            b"-IOERR error or timeout connecting to the client\0" as *const u8
                as *const libc::c_char,
        );
        connClose(conn);
        sdsfree(name);
        return 0 as *mut migrateCachedSocket;
    }
    connEnableTcpNoDelay(conn);
    cs = zmalloc(core::mem::size_of::<migrateCachedSocket>() as libc::c_ulong)
        as *mut migrateCachedSocket;
    (*cs).conn = conn;
    (*cs).last_dbid = -(1 as libc::c_int) as libc::c_long;
    (*cs).last_use_time = server.unixtime as time_t;
    dictAdd(
        server.migrate_cached_sockets,
        name as *mut libc::c_void,
        cs as *mut libc::c_void,
    );
    return cs;
}
#[no_mangle]
pub unsafe extern "C" fn migrateCloseSocket(mut host: *mut robj, mut port: *mut robj) {
    let mut name: sds = sdsempty();
    let mut cs: *mut migrateCachedSocket = 0 as *mut migrateCachedSocket;
    name = sdscatlen(name, (*host).ptr, sdslen((*host).ptr as sds));
    name = sdscatlen(
        name,
        b":\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    name = sdscatlen(name, (*port).ptr, sdslen((*port).ptr as sds));
    cs = dictFetchValue(server.migrate_cached_sockets, name as *const libc::c_void)
        as *mut migrateCachedSocket;
    if cs.is_null() {
        sdsfree(name);
        return;
    }
    connClose((*cs).conn);
    zfree(cs as *mut libc::c_void);
    dictDelete(server.migrate_cached_sockets, name as *const libc::c_void);
    sdsfree(name);
}
#[no_mangle]
pub unsafe extern "C" fn migrateCloseTimedoutSockets() {
    let mut di: *mut dictIterator = dictGetSafeIterator(server.migrate_cached_sockets);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cs: *mut migrateCachedSocket = (*de).v.val as *mut migrateCachedSocket;
        if server.unixtime as libc::c_long - (*cs).last_use_time
            > 10 as libc::c_int as libc::c_long
        {
            connClose((*cs).conn);
            zfree(cs as *mut libc::c_void);
            dictDelete(server.migrate_cached_sockets, (*de).key);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn migrateCommand(mut c: *mut client) {
    let mut buf0: [libc::c_char; 1024] = [0; 1024];
    let mut buf1: [libc::c_char; 1024] = [0; 1024];
    let mut buf2: [libc::c_char; 1024] = [0; 1024];
    let mut error_from_target: libc::c_int = 0;
    let mut socket_error: libc::c_int = 0;
    let mut del_idx: libc::c_int = 0;
    let mut current_block: u64;
    let mut cs: *mut migrateCachedSocket = 0 as *mut migrateCachedSocket;
    let mut copy: libc::c_int = 0 as libc::c_int;
    let mut replace: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut username: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut password: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut timeout: libc::c_long = 0;
    let mut dbid: libc::c_long = 0;
    let mut ov: *mut *mut robj = 0 as *mut *mut robj;
    let mut kv: *mut *mut robj = 0 as *mut *mut robj;
    let mut newargv: *mut *mut robj = 0 as *mut *mut robj;
    let mut cmd: rio = rio {
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
    let mut payload: rio = rio {
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
    let mut may_retry: libc::c_int = 1 as libc::c_int;
    let mut write_error: libc::c_int = 0 as libc::c_int;
    let mut argv_rewritten: libc::c_int = 0 as libc::c_int;
    let mut first_key: libc::c_int = 3 as libc::c_int;
    let mut num_keys: libc::c_int = 1 as libc::c_int;
    j = 6 as libc::c_int;
    while j < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - j;
        if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"copy\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            copy = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"replace\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            replace = 1 as libc::c_int;
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"auth\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if moreargs == 0 {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            j += 1;
            password = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
            redactClientCommandArgument(c, j);
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"auth2\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if moreargs < 2 as libc::c_int {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            j += 1;
            username = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
            redactClientCommandArgument(c, j);
            j += 1;
            password = (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char;
            redactClientCommandArgument(c, j);
        } else if strcasecmp(
            (**((*c).argv).offset(j as isize)).ptr as *const libc::c_char,
            b"keys\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if sdslen((**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds)
                != 0 as libc::c_int as libc::c_ulong
            {
                addReplyError(
                    c,
                    b"When using MIGRATE KEYS option, the key argument must be set to the empty string\0"
                        as *const u8 as *const libc::c_char,
                );
                return;
            }
            first_key = j + 1 as libc::c_int;
            num_keys = (*c).argc - j - 1 as libc::c_int;
            break;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        j += 1;
    }
    if getLongFromObjectOrReply(
        c,
        *((*c).argv).offset(5 as libc::c_int as isize),
        &mut timeout,
        0 as *const libc::c_char,
    ) != 0 as libc::c_int
        || getLongFromObjectOrReply(
            c,
            *((*c).argv).offset(4 as libc::c_int as isize),
            &mut dbid,
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        return;
    }
    if timeout <= 0 as libc::c_int as libc::c_long {
        timeout = 1000 as libc::c_int as libc::c_long;
    }
    ov = zrealloc(
        ov as *mut libc::c_void,
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(num_keys as libc::c_ulong),
    ) as *mut *mut robj;
    kv = zrealloc(
        kv as *mut libc::c_void,
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(num_keys as libc::c_ulong),
    ) as *mut *mut robj;
    let mut oi: libc::c_int = 0 as libc::c_int;
    j = 0 as libc::c_int;
    while j < num_keys {
        let ref mut fresh18 = *ov.offset(oi as isize);
        *fresh18 = lookupKeyRead((*c).db, *((*c).argv).offset((first_key + j) as isize));
        if !(*fresh18).is_null() {
            let ref mut fresh19 = *kv.offset(oi as isize);
            *fresh19 = *((*c).argv).offset((first_key + j) as isize);
            oi += 1;
        }
        j += 1;
    }
    num_keys = oi;
    if num_keys == 0 as libc::c_int {
        zfree(ov as *mut libc::c_void);
        zfree(kv as *mut libc::c_void);
        addReplySds(c, sdsnew(b"+NOKEY\r\n\0" as *const u8 as *const libc::c_char));
        return;
    }
    loop {
        write_error = 0 as libc::c_int;
        cs = migrateGetSocket(
            c,
            *((*c).argv).offset(1 as libc::c_int as isize),
            *((*c).argv).offset(2 as libc::c_int as isize),
            timeout,
        );
        if cs.is_null() {
            zfree(ov as *mut libc::c_void);
            zfree(kv as *mut libc::c_void);
            return;
        }
        rioInitWithBuffer(&mut cmd, sdsempty());
        if !password.is_null() {
            let mut arity: libc::c_int = if !username.is_null() {
                3 as libc::c_int
            } else {
                2 as libc::c_int
            };
            if rioWriteBulkCount(
                &mut cmd,
                '*' as i32 as libc::c_char,
                arity as libc::c_long,
            ) != 0
            {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkCount(&cmd,'*',arity)\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6303 as libc::c_int,
                );
                unreachable!();
            };
            if rioWriteBulkString(
                &mut cmd,
                b"AUTH\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as size_t,
            ) != 0
            {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkString(&cmd,\"AUTH\",4)\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6304 as libc::c_int,
                );
                unreachable!();
            };
            if !username.is_null() {
                if rioWriteBulkString(&mut cmd, username, sdslen(username)) != 0
                {} else {
                    _serverAssertWithInfo(
                        c,
                        0 as *const robj,
                        b"rioWriteBulkString(&cmd,username, sdslen(username))\0"
                            as *const u8 as *const libc::c_char,
                        b"cluster.c\0" as *const u8 as *const libc::c_char,
                        6307 as libc::c_int,
                    );
                    unreachable!();
                };
            }
            if rioWriteBulkString(&mut cmd, password, sdslen(password)) != 0 {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkString(&cmd,password, sdslen(password))\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6310 as libc::c_int,
                );
                unreachable!();
            };
        }
        let mut select: libc::c_int = ((*cs).last_dbid != dbid) as libc::c_int;
        if select != 0 {
            if rioWriteBulkCount(
                &mut cmd,
                '*' as i32 as libc::c_char,
                2 as libc::c_int as libc::c_long,
            ) != 0
            {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkCount(&cmd,'*',2)\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6316 as libc::c_int,
                );
                unreachable!();
            };
            if rioWriteBulkString(
                &mut cmd,
                b"SELECT\0" as *const u8 as *const libc::c_char,
                6 as libc::c_int as size_t,
            ) != 0
            {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkString(&cmd,\"SELECT\",6)\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6317 as libc::c_int,
                );
                unreachable!();
            };
            if rioWriteBulkLongLong(&mut cmd, dbid as libc::c_longlong) != 0 {} else {
                _serverAssertWithInfo(
                    c,
                    0 as *const robj,
                    b"rioWriteBulkLongLong(&cmd,dbid)\0" as *const u8
                        as *const libc::c_char,
                    b"cluster.c\0" as *const u8 as *const libc::c_char,
                    6318 as libc::c_int,
                );
                unreachable!();
            };
        }
        let mut non_expired: libc::c_int = 0 as libc::c_int;
        let mut current_block_91: u64;
        j = 0 as libc::c_int;
        while j < num_keys {
            let mut ttl: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
            let mut expireat: libc::c_longlong = getExpire(
                (*c).db,
                *kv.offset(j as isize),
            );
            if expireat != -(1 as libc::c_int) as libc::c_longlong {
                ttl = expireat - mstime();
                if ttl < 0 as libc::c_int as libc::c_longlong {
                    current_block_91 = 8869332144787829186;
                } else {
                    if ttl < 1 as libc::c_int as libc::c_longlong {
                        ttl = 1 as libc::c_int as libc::c_longlong;
                    }
                    current_block_91 = 6367734732029634840;
                }
            } else {
                current_block_91 = 6367734732029634840;
            }
            match current_block_91 {
                6367734732029634840 => {
                    let ref mut fresh20 = *ov.offset(non_expired as isize);
                    *fresh20 = *ov.offset(j as isize);
                    let fresh21 = non_expired;
                    non_expired = non_expired + 1;
                    let ref mut fresh22 = *kv.offset(fresh21 as isize);
                    *fresh22 = *kv.offset(j as isize);
                    if rioWriteBulkCount(
                        &mut cmd,
                        '*' as i32 as libc::c_char,
                        (if replace != 0 { 5 as libc::c_int } else { 4 as libc::c_int })
                            as libc::c_long,
                    ) != 0
                    {} else {
                        _serverAssertWithInfo(
                            c,
                            0 as *const robj,
                            b"rioWriteBulkCount(&cmd,'*',replace ? 5 : 4)\0" as *const u8
                                as *const libc::c_char,
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            6346 as libc::c_int,
                        );
                        unreachable!();
                    };
                    if server.cluster_enabled != 0 {
                        if rioWriteBulkString(
                            &mut cmd,
                            b"RESTORE-ASKING\0" as *const u8 as *const libc::c_char,
                            14 as libc::c_int as size_t,
                        ) != 0
                        {} else {
                            _serverAssertWithInfo(
                                c,
                                0 as *const robj,
                                b"rioWriteBulkString(&cmd,\"RESTORE-ASKING\",14)\0"
                                    as *const u8 as *const libc::c_char,
                                b"cluster.c\0" as *const u8 as *const libc::c_char,
                                6350 as libc::c_int,
                            );
                            unreachable!();
                        };
                    } else {
                        if rioWriteBulkString(
                            &mut cmd,
                            b"RESTORE\0" as *const u8 as *const libc::c_char,
                            7 as libc::c_int as size_t,
                        ) != 0
                        {} else {
                            _serverAssertWithInfo(
                                c,
                                0 as *const robj,
                                b"rioWriteBulkString(&cmd,\"RESTORE\",7)\0" as *const u8
                                    as *const libc::c_char,
                                b"cluster.c\0" as *const u8 as *const libc::c_char,
                                6352 as libc::c_int,
                            );
                            unreachable!();
                        };
                    }
                    if (**kv.offset(j as isize)).encoding() as libc::c_int
                        == 0 as libc::c_int
                        || (**kv.offset(j as isize)).encoding() as libc::c_int
                            == 8 as libc::c_int
                    {} else {
                        _serverAssertWithInfo(
                            c,
                            0 as *const robj,
                            b"sdsEncodedObject(kv[j])\0" as *const u8
                                as *const libc::c_char,
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            6353 as libc::c_int,
                        );
                        unreachable!();
                    };
                    if rioWriteBulkString(
                        &mut cmd,
                        (**kv.offset(j as isize)).ptr as *const libc::c_char,
                        sdslen((**kv.offset(j as isize)).ptr as sds),
                    ) != 0
                    {} else {
                        _serverAssertWithInfo(
                            c,
                            0 as *const robj,
                            b"rioWriteBulkString(&cmd,kv[j]->ptr, sdslen(kv[j]->ptr))\0"
                                as *const u8 as *const libc::c_char,
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            6355 as libc::c_int,
                        );
                        unreachable!();
                    };
                    if rioWriteBulkLongLong(&mut cmd, ttl) != 0 {} else {
                        _serverAssertWithInfo(
                            c,
                            0 as *const robj,
                            b"rioWriteBulkLongLong(&cmd,ttl)\0" as *const u8
                                as *const libc::c_char,
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            6356 as libc::c_int,
                        );
                        unreachable!();
                    };
                    createDumpPayload(
                        &mut payload,
                        *ov.offset(j as isize),
                        *kv.offset(j as isize),
                        dbid as libc::c_int,
                    );
                    if rioWriteBulkString(
                        &mut cmd,
                        payload.io.buffer.ptr as *const libc::c_char,
                        sdslen(payload.io.buffer.ptr),
                    ) != 0
                    {} else {
                        _serverAssertWithInfo(
                            c,
                            0 as *const robj,
                            b"rioWriteBulkString(&cmd,payload.io.buffer.ptr, sdslen(payload.io.buffer.ptr))\0"
                                as *const u8 as *const libc::c_char,
                            b"cluster.c\0" as *const u8 as *const libc::c_char,
                            6363 as libc::c_int,
                        );
                        unreachable!();
                    };
                    sdsfree(payload.io.buffer.ptr);
                    if replace != 0 {
                        if rioWriteBulkString(
                            &mut cmd,
                            b"REPLACE\0" as *const u8 as *const libc::c_char,
                            7 as libc::c_int as size_t,
                        ) != 0
                        {} else {
                            _serverAssertWithInfo(
                                c,
                                0 as *const robj,
                                b"rioWriteBulkString(&cmd,\"REPLACE\",7)\0" as *const u8
                                    as *const libc::c_char,
                                b"cluster.c\0" as *const u8 as *const libc::c_char,
                                6369 as libc::c_int,
                            );
                            unreachable!();
                        };
                    }
                }
                _ => {}
            }
            j += 1;
        }
        num_keys = non_expired;
        *__errno_location() = 0 as libc::c_int;
        let mut buf: sds = cmd.io.buffer.ptr;
        let mut pos: size_t = 0 as libc::c_int as size_t;
        let mut towrite: size_t = 0;
        let mut nwritten: libc::c_int = 0 as libc::c_int;
        loop {
            towrite = (sdslen(buf)).wrapping_sub(pos);
            if !(towrite > 0 as libc::c_int as libc::c_ulong) {
                current_block = 10369920510435091891;
                break;
            }
            towrite = if towrite
                > (64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
            {
                (64 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
            } else {
                towrite
            };
            nwritten = connSyncWrite(
                (*cs).conn,
                buf.offset(pos as isize),
                towrite as ssize_t,
                timeout as libc::c_longlong,
            ) as libc::c_int;
            if nwritten != towrite as libc::c_int {
                write_error = 1 as libc::c_int;
                current_block = 14587052215474842647;
                break;
            } else {
                pos = (pos as libc::c_ulong).wrapping_add(nwritten as libc::c_ulong)
                    as size_t as size_t;
            }
        }
        match current_block {
            10369920510435091891 => {
                buf0 = [0; 1024];
                buf1 = [0; 1024];
                buf2 = [0; 1024];
                if !(!password.is_null()
                    && connSyncReadLine(
                        (*cs).conn,
                        buf0.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong
                            as ssize_t,
                        timeout as libc::c_longlong,
                    ) <= 0 as libc::c_int as libc::c_long)
                {
                    if !(select != 0
                        && connSyncReadLine(
                            (*cs).conn,
                            buf1.as_mut_ptr(),
                            core::mem::size_of::<[libc::c_char; 1024]>()
                                as libc::c_ulong as ssize_t,
                            timeout as libc::c_longlong,
                        ) <= 0 as libc::c_int as libc::c_long)
                    {
                        error_from_target = 0 as libc::c_int;
                        socket_error = 0 as libc::c_int;
                        del_idx = 1 as libc::c_int;
                        if copy == 0 {
                            newargv = zmalloc(
                                (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                                    .wrapping_mul(
                                        (num_keys + 1 as libc::c_int) as libc::c_ulong,
                                    ),
                            ) as *mut *mut robj;
                        }
                        j = 0 as libc::c_int;
                        while j < num_keys {
                            if connSyncReadLine(
                                (*cs).conn,
                                buf2.as_mut_ptr(),
                                core::mem::size_of::<[libc::c_char; 1024]>()
                                    as libc::c_ulong as ssize_t,
                                timeout as libc::c_longlong,
                            ) <= 0 as libc::c_int as libc::c_long
                            {
                                socket_error = 1 as libc::c_int;
                                break;
                            } else {
                                if !password.is_null()
                                    && buf0[0 as libc::c_int as usize] as libc::c_int
                                        == '-' as i32
                                    || select != 0
                                        && buf1[0 as libc::c_int as usize] as libc::c_int
                                            == '-' as i32
                                    || buf2[0 as libc::c_int as usize] as libc::c_int
                                        == '-' as i32
                                {
                                    if error_from_target == 0 {
                                        (*cs).last_dbid = -(1 as libc::c_int) as libc::c_long;
                                        let mut errbuf: *mut libc::c_char = 0 as *mut libc::c_char;
                                        if !password.is_null()
                                            && buf0[0 as libc::c_int as usize] as libc::c_int
                                                == '-' as i32
                                        {
                                            errbuf = buf0.as_mut_ptr();
                                        } else if select != 0
                                            && buf1[0 as libc::c_int as usize] as libc::c_int
                                                == '-' as i32
                                        {
                                            errbuf = buf1.as_mut_ptr();
                                        } else {
                                            errbuf = buf2.as_mut_ptr();
                                        }
                                        error_from_target = 1 as libc::c_int;
                                        addReplyErrorFormat(
                                            c,
                                            b"Target instance replied with error: %s\0" as *const u8
                                                as *const libc::c_char,
                                            errbuf.offset(1 as libc::c_int as isize),
                                        );
                                    }
                                } else if copy == 0 {
                                    dbDelete((*c).db, *kv.offset(j as isize));
                                    signalModifiedKey(c, (*c).db, *kv.offset(j as isize));
                                    notifyKeyspaceEvent(
                                        (1 as libc::c_int) << 2 as libc::c_int,
                                        b"del\0" as *const u8 as *const libc::c_char
                                            as *mut libc::c_char,
                                        *kv.offset(j as isize),
                                        (*(*c).db).id,
                                    );
                                    server.dirty += 1;
                                    let fresh23 = del_idx;
                                    del_idx = del_idx + 1;
                                    let ref mut fresh24 = *newargv.offset(fresh23 as isize);
                                    *fresh24 = *kv.offset(j as isize);
                                    incrRefCount(*kv.offset(j as isize));
                                }
                                j += 1;
                            }
                        }
                        if !(error_from_target == 0 && socket_error != 0
                            && j == 0 as libc::c_int && may_retry != 0
                            && *__errno_location() != 110 as libc::c_int)
                        {
                            if socket_error != 0 {
                                migrateCloseSocket(
                                    *((*c).argv).offset(1 as libc::c_int as isize),
                                    *((*c).argv).offset(2 as libc::c_int as isize),
                                );
                            }
                            if copy == 0 {
                                if del_idx > 1 as libc::c_int {
                                    let ref mut fresh25 = *newargv
                                        .offset(0 as libc::c_int as isize);
                                    *fresh25 = createStringObject(
                                        b"DEL\0" as *const u8 as *const libc::c_char,
                                        3 as libc::c_int as size_t,
                                    );
                                    replaceClientCommandVector(c, del_idx, newargv);
                                    argv_rewritten = 1 as libc::c_int;
                                } else {
                                    zfree(newargv as *mut libc::c_void);
                                }
                                newargv = 0 as *mut *mut robj;
                            }
                            if error_from_target == 0 && socket_error != 0 {
                                may_retry = 0 as libc::c_int;
                            } else {
                                if error_from_target == 0 {
                                    (*cs).last_dbid = dbid;
                                    addReply(c, shared.ok);
                                }
                                sdsfree(cmd.io.buffer.ptr);
                                zfree(ov as *mut libc::c_void);
                                zfree(kv as *mut libc::c_void);
                                zfree(newargv as *mut libc::c_void);
                                return;
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        sdsfree(cmd.io.buffer.ptr);
        if argv_rewritten == 0 {
            migrateCloseSocket(
                *((*c).argv).offset(1 as libc::c_int as isize),
                *((*c).argv).offset(2 as libc::c_int as isize),
            );
        }
        zfree(newargv as *mut libc::c_void);
        newargv = 0 as *mut *mut robj;
        if *__errno_location() != 110 as libc::c_int && may_retry != 0 {
            may_retry = 0 as libc::c_int;
        } else {
            zfree(ov as *mut libc::c_void);
            zfree(kv as *mut libc::c_void);
            addReplyErrorSds(
                c,
                sdscatprintf(
                    sdsempty(),
                    b"-IOERR error or timeout %s to target instance\0" as *const u8
                        as *const libc::c_char,
                    if write_error != 0 {
                        b"writing\0" as *const u8 as *const libc::c_char
                    } else {
                        b"reading\0" as *const u8 as *const libc::c_char
                    },
                ),
            );
            return;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn askingCommand(mut c: *mut client) {
    if server.cluster_enabled == 0 as libc::c_int {
        addReplyError(
            c,
            b"This instance has cluster support disabled\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    (*c).flags |= ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn readonlyCommand(mut c: *mut client) {
    if server.cluster_enabled == 0 as libc::c_int {
        addReplyError(
            c,
            b"This instance has cluster support disabled\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    (*c).flags |= ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn readwriteCommand(mut c: *mut client) {
    if server.cluster_enabled == 0 as libc::c_int {
        addReplyError(
            c,
            b"This instance has cluster support disabled\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    (*c).flags &= !((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong;
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn getNodeByQuery(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut hashslot: *mut libc::c_int,
    mut error_code: *mut libc::c_int,
) -> *mut clusterNode {
    let mut n: *mut clusterNode = 0 as *mut clusterNode;
    let mut firstkey: *mut robj = 0 as *mut robj;
    let mut multiple_keys: libc::c_int = 0 as libc::c_int;
    let mut ms: *mut multiState = 0 as *mut multiState;
    let mut _ms: multiState = multiState {
        commands: 0 as *mut multiCmd,
        count: 0,
        cmd_flags: 0,
        cmd_inv_flags: 0,
        argv_len_sums: 0,
        alloc_count: 0,
    };
    let mut mc: multiCmd = multiCmd {
        argv: 0 as *mut *mut robj,
        argv_len: 0,
        argc: 0,
        cmd: 0 as *mut redisCommand,
    };
    let mut i: libc::c_int = 0;
    let mut slot: libc::c_int = 0 as libc::c_int;
    let mut migrating_slot: libc::c_int = 0 as libc::c_int;
    let mut importing_slot: libc::c_int = 0 as libc::c_int;
    let mut missing_keys: libc::c_int = 0 as libc::c_int;
    let mut existing_keys: libc::c_int = 0 as libc::c_int;
    if server.cluster_module_flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        return myself;
    }
    if !error_code.is_null() {
        *error_code = 0 as libc::c_int;
    }
    if (*cmd).proc_0 == Some(execCommand as unsafe extern "C" fn(*mut client) -> ()) {
        if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong == 0 {
            return myself;
        }
        ms = &mut (*c).mstate;
    } else {
        ms = &mut _ms;
        _ms.commands = &mut mc;
        _ms.count = 1 as libc::c_int;
        mc.argv = argv;
        mc.argc = argc;
        mc.cmd = cmd;
    }
    let mut is_pubsubshard: libc::c_int = ((*cmd).proc_0
        == Some(ssubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*cmd).proc_0
            == Some(sunsubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*cmd).proc_0
            == Some(spublishCommand as unsafe extern "C" fn(*mut client) -> ()))
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*ms).count {
        let mut mcmd: *mut redisCommand = 0 as *mut redisCommand;
        let mut margv: *mut *mut robj = 0 as *mut *mut robj;
        let mut margc: libc::c_int = 0;
        let mut numkeys: libc::c_int = 0;
        let mut j: libc::c_int = 0;
        let mut keyindex: *mut keyReference = 0 as *mut keyReference;
        mcmd = (*((*ms).commands).offset(i as isize)).cmd;
        margc = (*((*ms).commands).offset(i as isize)).argc;
        margv = (*((*ms).commands).offset(i as isize)).argv;
        let mut result: getKeysResult = {
            let mut init = getKeysResult {
                keysbuf: [
                    {
                        let mut init = keyReference {
                            pos: 0 as libc::c_int,
                            flags: 0,
                        };
                        init
                    },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                    keyReference { pos: 0, flags: 0 },
                ],
                keys: 0 as *mut keyReference,
                numkeys: 0 as libc::c_int,
                size: 256 as libc::c_int,
            };
            init
        };
        numkeys = getKeysFromCommand(mcmd, margv, margc, &mut result);
        keyindex = result.keys;
        j = 0 as libc::c_int;
        while j < numkeys {
            let mut thiskey: *mut robj = *margv
                .offset((*keyindex.offset(j as isize)).pos as isize);
            let mut thisslot: libc::c_int = keyHashSlot(
                (*thiskey).ptr as *mut libc::c_char,
                sdslen((*thiskey).ptr as sds) as libc::c_int,
            ) as libc::c_int;
            if firstkey.is_null() {
                firstkey = thiskey;
                slot = thisslot;
                n = (*server.cluster).slots[slot as usize];
                if n.is_null() {
                    getKeysFreeResult(&mut result);
                    if !error_code.is_null() {
                        *error_code = 6 as libc::c_int;
                    }
                    return 0 as *mut clusterNode;
                }
                if n == myself
                    && !((*server.cluster).migrating_slots_to[slot as usize]).is_null()
                {
                    migrating_slot = 1 as libc::c_int;
                } else if !((*server.cluster).importing_slots_from[slot as usize])
                    .is_null()
                {
                    importing_slot = 1 as libc::c_int;
                }
            } else if equalStringObjects(firstkey, thiskey) == 0 {
                if slot != thisslot {
                    getKeysFreeResult(&mut result);
                    if !error_code.is_null() {
                        *error_code = 1 as libc::c_int;
                    }
                    return 0 as *mut clusterNode;
                } else {
                    multiple_keys = 1 as libc::c_int;
                }
            }
            let mut flags: libc::c_int = (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int;
            if (migrating_slot != 0 || importing_slot != 0) && is_pubsubshard == 0 {
                if (lookupKeyReadWithFlags(
                    &mut *(server.db).offset(0 as libc::c_int as isize),
                    thiskey,
                    flags,
                ))
                    .is_null()
                {
                    missing_keys += 1;
                } else {
                    existing_keys += 1;
                }
            }
            j += 1;
        }
        getKeysFreeResult(&mut result);
        i += 1;
    }
    if n.is_null() {
        return myself;
    }
    let mut cmd_flags: uint64_t = getCommandFlags(c);
    if (*server.cluster).state != 0 as libc::c_int {
        if is_pubsubshard != 0 {
            if server.cluster_allow_pubsubshard_when_down == 0 {
                if !error_code.is_null() {
                    *error_code = 5 as libc::c_int;
                }
                return 0 as *mut clusterNode;
            }
        } else if server.cluster_allow_reads_when_down == 0 {
            if !error_code.is_null() {
                *error_code = 5 as libc::c_int;
            }
            return 0 as *mut clusterNode;
        } else {
            if cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
            {
                if !error_code.is_null() {
                    *error_code = 7 as libc::c_int;
                }
                return 0 as *mut clusterNode;
            }
        }
    }
    if !hashslot.is_null() {
        *hashslot = slot;
    }
    if (migrating_slot != 0 || importing_slot != 0)
        && (*cmd).proc_0
            == Some(migrateCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        return myself;
    }
    if migrating_slot != 0 && missing_keys != 0 {
        if existing_keys != 0 {
            if !error_code.is_null() {
                *error_code = 2 as libc::c_int;
            }
            return 0 as *mut clusterNode;
        } else {
            if !error_code.is_null() {
                *error_code = 3 as libc::c_int;
            }
            return (*server.cluster).migrating_slots_to[slot as usize];
        }
    }
    if importing_slot != 0
        && ((*c).flags & ((1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong != 0
            || cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 13 as libc::c_int != 0)
    {
        if multiple_keys != 0 && missing_keys != 0 {
            if !error_code.is_null() {
                *error_code = 2 as libc::c_int;
            }
            return 0 as *mut clusterNode;
        } else {
            return myself
        }
    }
    let mut is_write_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0) as libc::c_int;
    if ((*c).flags & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong != 0
        || is_pubsubshard != 0) && is_write_command == 0
        && (*myself).flags & 2 as libc::c_int != 0 && (*myself).slaveof == n
    {
        return myself;
    }
    if n != myself && !error_code.is_null() {
        *error_code = 4 as libc::c_int;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn clusterRedirectClient(
    mut c: *mut client,
    mut n: *mut clusterNode,
    mut hashslot: libc::c_int,
    mut error_code: libc::c_int,
) {
    if error_code == 1 as libc::c_int {
        addReplyError(
            c,
            b"-CROSSSLOT Keys in request don't hash to the same slot\0" as *const u8
                as *const libc::c_char,
        );
    } else if error_code == 2 as libc::c_int {
        addReplyError(
            c,
            b"-TRYAGAIN Multiple keys request during rehashing of slot\0" as *const u8
                as *const libc::c_char,
        );
    } else if error_code == 5 as libc::c_int {
        addReplyError(
            c,
            b"-CLUSTERDOWN The cluster is down\0" as *const u8 as *const libc::c_char,
        );
    } else if error_code == 7 as libc::c_int {
        addReplyError(
            c,
            b"-CLUSTERDOWN The cluster is down and only accepts read commands\0"
                as *const u8 as *const libc::c_char,
        );
    } else if error_code == 6 as libc::c_int {
        addReplyError(
            c,
            b"-CLUSTERDOWN Hash slot not served\0" as *const u8 as *const libc::c_char,
        );
    } else if error_code == 4 as libc::c_int || error_code == 3 as libc::c_int {
        let mut use_pport: libc::c_int = (server.tls_cluster != 0
            && !((*c).conn).is_null() && connGetType((*c).conn) != 2 as libc::c_int)
            as libc::c_int;
        let mut port: libc::c_int = if use_pport != 0 && (*n).pport != 0 {
            (*n).pport
        } else {
            (*n).port
        };
        addReplyErrorSds(
            c,
            sdscatprintf(
                sdsempty(),
                b"-%s %d %s:%d\0" as *const u8 as *const libc::c_char,
                if error_code == 3 as libc::c_int {
                    b"ASK\0" as *const u8 as *const libc::c_char
                } else {
                    b"MOVED\0" as *const u8 as *const libc::c_char
                },
                hashslot,
                getPreferredEndpoint(n),
                port,
            ),
        );
    } else {
        _serverPanic(
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            6857 as libc::c_int,
            b"getNodeByQuery() unknown error.\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn clusterRedirectBlockedClientIfNeeded(
    mut c: *mut client,
) -> libc::c_int {
    if (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0
        && ((*c).btype == 1 as libc::c_int || (*c).btype == 5 as libc::c_int
            || (*c).btype == 4 as libc::c_int || (*c).btype == 3 as libc::c_int)
    {
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        if (*server.cluster).state == 1 as libc::c_int {
            clusterRedirectClient(
                c,
                0 as *mut clusterNode,
                0 as libc::c_int,
                5 as libc::c_int,
            );
            return 1 as libc::c_int;
        }
        if (*c).btype == 3 as libc::c_int && moduleClientIsBlockedOnKeys(c) == 0 {
            return 0 as libc::c_int;
        }
        di = dictGetIterator((*c).bpop.keys);
        de = dictNext(di);
        if !de.is_null() {
            let mut key: *mut robj = (*de).key as *mut robj;
            let mut slot: libc::c_int = keyHashSlot(
                (*key).ptr as *mut libc::c_char,
                sdslen((*key).ptr as sds) as libc::c_int,
            ) as libc::c_int;
            let mut node: *mut clusterNode = (*server.cluster).slots[slot as usize];
            if (*c).flags & ((1 as libc::c_int) << 17 as libc::c_int) as libc::c_ulong
                != 0
                && (*(*c).lastcmd).flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 0 as libc::c_int == 0
                && (*myself).flags & 2 as libc::c_int != 0 && (*myself).slaveof == node
            {
                node = myself;
            }
            if node != myself
                && ((*server.cluster).importing_slots_from[slot as usize]).is_null()
            {
                if node.is_null() {
                    clusterRedirectClient(
                        c,
                        0 as *mut clusterNode,
                        0 as libc::c_int,
                        6 as libc::c_int,
                    );
                } else {
                    clusterRedirectClient(c, node, slot, 4 as libc::c_int);
                }
                dictReleaseIterator(di);
                return 1 as libc::c_int;
            }
        }
        dictReleaseIterator(di);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyAddEntry(
    mut entry: *mut dictEntry,
    mut db: *mut redisDb,
) {
    let mut key: sds = (*entry).key as sds;
    let mut hashslot: libc::c_uint = keyHashSlot(key, sdslen(key) as libc::c_int);
    let mut slot_to_keys: *mut slotToKeys = &mut *((*(*db).slots_to_keys).by_slot)
        .as_mut_ptr()
        .offset(hashslot as isize) as *mut slotToKeys;
    (*slot_to_keys).count = ((*slot_to_keys).count).wrapping_add(1);
    let mut first: *mut dictEntry = (*slot_to_keys).head;
    let ref mut fresh26 = (*(&mut (*entry).metadata as *mut [*mut libc::c_void; 0]
        as *mut clusterDictEntryMetadata))
        .next;
    *fresh26 = first;
    if !first.is_null() {
        if ((*(&mut (*first).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .prev)
            .is_null()
        {} else {
            _serverAssert(
                b"dictEntryPrevInSlot(first) == NULL\0" as *const u8
                    as *const libc::c_char,
                b"cluster.c\0" as *const u8 as *const libc::c_char,
                6949 as libc::c_int,
            );
            unreachable!();
        };
        let ref mut fresh27 = (*(&mut (*first).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .prev;
        *fresh27 = entry;
    }
    if ((*(&mut (*entry).metadata as *mut [*mut libc::c_void; 0]
        as *mut clusterDictEntryMetadata))
        .prev)
        .is_null()
    {} else {
        _serverAssert(
            b"dictEntryPrevInSlot(entry) == NULL\0" as *const u8 as *const libc::c_char,
            b"cluster.c\0" as *const u8 as *const libc::c_char,
            6952 as libc::c_int,
        );
        unreachable!();
    };
    (*slot_to_keys).head = entry;
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyDelEntry(
    mut entry: *mut dictEntry,
    mut db: *mut redisDb,
) {
    let mut key: sds = (*entry).key as sds;
    let mut hashslot: libc::c_uint = keyHashSlot(key, sdslen(key) as libc::c_int);
    let mut slot_to_keys: *mut slotToKeys = &mut *((*(*db).slots_to_keys).by_slot)
        .as_mut_ptr()
        .offset(hashslot as isize) as *mut slotToKeys;
    (*slot_to_keys).count = ((*slot_to_keys).count).wrapping_sub(1);
    let mut next: *mut dictEntry = (*(&mut (*entry).metadata
        as *mut [*mut libc::c_void; 0] as *mut clusterDictEntryMetadata))
        .next;
    let mut prev: *mut dictEntry = (*(&mut (*entry).metadata
        as *mut [*mut libc::c_void; 0] as *mut clusterDictEntryMetadata))
        .prev;
    if !next.is_null() {
        let ref mut fresh28 = (*(&mut (*next).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .prev;
        *fresh28 = prev;
    }
    if !prev.is_null() {
        let ref mut fresh29 = (*(&mut (*prev).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .next;
        *fresh29 = next;
    } else {
        if (*slot_to_keys).head == entry {} else {
            _serverAssert(
                b"slot_to_keys->head == entry\0" as *const u8 as *const libc::c_char,
                b"cluster.c\0" as *const u8 as *const libc::c_char,
                6972 as libc::c_int,
            );
            unreachable!();
        };
        (*slot_to_keys).head = next;
    };
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyReplaceEntry(
    mut entry: *mut dictEntry,
    mut db: *mut redisDb,
) {
    let mut next: *mut dictEntry = (*(&mut (*entry).metadata
        as *mut [*mut libc::c_void; 0] as *mut clusterDictEntryMetadata))
        .next;
    let mut prev: *mut dictEntry = (*(&mut (*entry).metadata
        as *mut [*mut libc::c_void; 0] as *mut clusterDictEntryMetadata))
        .prev;
    if !next.is_null() {
        let ref mut fresh30 = (*(&mut (*next).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .prev;
        *fresh30 = entry;
    }
    if !prev.is_null() {
        let ref mut fresh31 = (*(&mut (*prev).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .next;
        *fresh31 = entry;
    } else {
        let mut key: sds = (*entry).key as sds;
        let mut hashslot: libc::c_uint = keyHashSlot(key, sdslen(key) as libc::c_int);
        let mut slot_to_keys: *mut slotToKeys = &mut *((*(*db).slots_to_keys).by_slot)
            .as_mut_ptr()
            .offset(hashslot as isize) as *mut slotToKeys;
        (*slot_to_keys).head = entry;
    };
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyInit(mut db: *mut redisDb) {
    (*db)
        .slots_to_keys = zcalloc(
        core::mem::size_of::<clusterSlotToKeyMapping>() as libc::c_ulong,
    ) as *mut clusterSlotToKeyMapping;
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyFlush(mut db: *mut redisDb) {
    memset(
        (*db).slots_to_keys as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<clusterSlotToKeyMapping>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn slotToKeyDestroy(mut db: *mut redisDb) {
    zfree((*db).slots_to_keys as *mut libc::c_void);
    (*db).slots_to_keys = 0 as *mut clusterSlotToKeyMapping;
}
#[no_mangle]
pub unsafe extern "C" fn delKeysInSlot(mut hashslot: libc::c_uint) -> libc::c_uint {
    let mut j: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut de: *mut dictEntry = (*(*server.db).slots_to_keys)
        .by_slot[hashslot as usize]
        .head;
    while !de.is_null() {
        let mut sdskey: sds = (*de).key as sds;
        de = (*(&mut (*de).metadata as *mut [*mut libc::c_void; 0]
            as *mut clusterDictEntryMetadata))
            .next;
        let mut key: *mut robj = createStringObject(
            sdskey as *const libc::c_char,
            sdslen(sdskey),
        );
        dbDelete(&mut *(server.db).offset(0 as libc::c_int as isize), key);
        decrRefCount(key);
        j = j.wrapping_add(1);
    }
    return j;
}
#[no_mangle]
pub unsafe extern "C" fn countKeysInSlot(mut hashslot: libc::c_uint) -> libc::c_uint {
    return (*(*server.db).slots_to_keys).by_slot[hashslot as usize].count
        as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn slotToChannelUpdate(mut channel: sds, mut add: libc::c_int) {
    let mut keylen: size_t = sdslen(channel);
    let mut hashslot: libc::c_uint = keyHashSlot(channel, keylen as libc::c_int);
    let mut buf: [libc::c_uchar; 64] = [0; 64];
    let mut indexed: *mut libc::c_uchar = buf.as_mut_ptr();
    if keylen.wrapping_add(2 as libc::c_int as libc::c_ulong)
        > 64 as libc::c_int as libc::c_ulong
    {
        indexed = zmalloc(keylen.wrapping_add(2 as libc::c_int as libc::c_ulong))
            as *mut libc::c_uchar;
    }
    *indexed
        .offset(
            0 as libc::c_int as isize,
        ) = (hashslot >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *indexed
        .offset(
            1 as libc::c_int as isize,
        ) = (hashslot & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    memcpy(
        indexed.offset(2 as libc::c_int as isize) as *mut libc::c_void,
        channel as *const libc::c_void,
        keylen,
    );
    if add != 0 {
        raxInsert(
            (*server.cluster).slots_to_channels,
            indexed,
            keylen.wrapping_add(2 as libc::c_int as libc::c_ulong),
            0 as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    } else {
        raxRemove(
            (*server.cluster).slots_to_channels,
            indexed,
            keylen.wrapping_add(2 as libc::c_int as libc::c_ulong),
            0 as *mut *mut libc::c_void,
        );
    }
    if indexed != buf.as_mut_ptr() {
        zfree(indexed as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn slotToChannelAdd(mut channel: sds) {
    slotToChannelUpdate(channel, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn slotToChannelDel(mut channel: sds) {
    slotToChannelUpdate(channel, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn countChannelsInSlot(
    mut hashslot: libc::c_uint,
) -> libc::c_uint {
    let mut iter: raxIterator = raxIterator {
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
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut indexed: [libc::c_uchar; 2] = [0; 2];
    indexed[0 as libc::c_int
        as usize] = (hashslot >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    indexed[1 as libc::c_int
        as usize] = (hashslot & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
    raxStart(&mut iter, (*server.cluster).slots_to_channels);
    raxSeek(
        &mut iter,
        b">=\0" as *const u8 as *const libc::c_char,
        indexed.as_mut_ptr(),
        2 as libc::c_int as size_t,
    );
    while raxNext(&mut iter) != 0 {
        if *(iter.key).offset(0 as libc::c_int as isize) as libc::c_int
            != indexed[0 as libc::c_int as usize] as libc::c_int
            || *(iter.key).offset(1 as libc::c_int as isize) as libc::c_int
                != indexed[1 as libc::c_int as usize] as libc::c_int
        {
            break;
        }
        j += 1;
    }
    raxStop(&mut iter);
    return j as libc::c_uint;
}

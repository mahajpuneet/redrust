extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type functionsLibCtx;
    pub type clusterState;
    fn init_genrand64(seed: libc::c_ulonglong);
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn rioInitWithFile(r: *mut rio, fp: *mut FILE);
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
    fn getpid() -> __pid_t;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn moduleTypeNameByID(name: *mut libc::c_char, moduleid: uint64_t);
    fn mstime() -> libc::c_longlong;
    fn decrRefCount(o: *mut robj);
    fn startLoadingFile(
        size: size_t,
        filename: *mut libc::c_char,
        rdbflags: libc::c_int,
    );
    fn stopLoading(success: libc::c_int);
    fn rdbLoadType(rdb: *mut rio) -> libc::c_int;
    fn rdbLoadTime(rdb: *mut rio) -> time_t;
    fn rdbLoadMillisecondTime(rdb: *mut rio, rdbver: libc::c_int) -> libc::c_longlong;
    fn rdbLoadLen(rdb: *mut rio, isencoded: *mut libc::c_int) -> uint64_t;
    fn rdbLoadObject(
        type_0: libc::c_int,
        rdb: *mut rio,
        key: sds,
        dbid: libc::c_int,
        error: *mut libc::c_int,
    ) -> *mut robj;
    fn rdbLoadCheckModuleValue(
        rdb: *mut rio,
        modulename: *mut libc::c_char,
    ) -> *mut robj;
    fn rdbLoadStringObject(rdb: *mut rio) -> *mut robj;
    fn rdbFunctionLoad(
        rdb: *mut rio,
        ver: libc::c_int,
        lib_ctx: *mut functionsLibCtx,
        type_0: libc::c_int,
        rdbflags: libc::c_int,
        err: *mut sds,
    ) -> libc::c_int;
    fn redisGitDirty() -> *mut libc::c_char;
    fn redisGitSHA1() -> *mut libc::c_char;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn createSharedObjects();
    fn rdbLoadProgressCallback(r: *mut rio, buf: *const libc::c_void, len: size_t);
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
pub type __clock_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type sds = *mut libc::c_char;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_13,
    pub _timer: C2RustUnnamed_12,
    pub _rt: C2RustUnnamed_11,
    pub _sigchld: C2RustUnnamed_10,
    pub _sigfault: C2RustUnnamed_7,
    pub _sigpoll: C2RustUnnamed_6,
    pub _sigsys: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub _addr_bnd: C2RustUnnamed_9,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_14,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_14 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
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
    pub v: C2RustUnnamed_15,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
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
    pub bs: C2RustUnnamed_19,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub range: C2RustUnnamed_18,
    pub keynum: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
pub union C2RustUnnamed_19 {
    pub index: C2RustUnnamed_21,
    pub keyword: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
    pub inst_metric: [C2RustUnnamed_22; 5],
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
pub struct C2RustUnnamed_22 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub rio: *mut rio,
    pub key: *mut robj,
    pub key_type: libc::c_int,
    pub keys: libc::c_ulong,
    pub expires: libc::c_ulong,
    pub already_expired: libc::c_ulong,
    pub doing: libc::c_int,
    pub error_set: libc::c_int,
    pub error: [libc::c_char; 1024],
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
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(0 as libc::c_int, __fd, __statbuf);
}
#[no_mangle]
pub static mut rdbCheckMode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut rdbstate: C2RustUnnamed_23 = C2RustUnnamed_23 {
    rio: 0 as *const rio as *mut rio,
    key: 0 as *const robj as *mut robj,
    key_type: 0,
    keys: 0,
    expires: 0,
    already_expired: 0,
    doing: 0,
    error_set: 0,
    error: [0; 1024],
};
#[no_mangle]
pub static mut rdb_check_doing_string: [*mut libc::c_char; 10] = [
    b"start\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-type\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-expire\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-key\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-object-value\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"check-sum\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-len\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-aux\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-module-aux\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"read-functions\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut rdb_type_string: [*mut libc::c_char; 19] = [
    b"string\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"list-linked\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"set-hashtable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zset-v1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hash-hashtable\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zset-v2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"module-value\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hash-zipmap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"list-ziplist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"set-intset\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zset-ziplist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hash-ziplist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"quicklist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"stream\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"hash-listpack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"zset-listpack\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"quicklist-v2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn rdbShowGenericInfo() {
    printf(
        b"[info] %lu keys read\n\0" as *const u8 as *const libc::c_char,
        rdbstate.keys,
    );
    printf(
        b"[info] %lu expires\n\0" as *const u8 as *const libc::c_char,
        rdbstate.expires,
    );
    printf(
        b"[info] %lu already expired\n\0" as *const u8 as *const libc::c_char,
        rdbstate.already_expired,
    );
}
#[no_mangle]
pub unsafe extern "C" fn rdbCheckError(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    printf(b"--- RDB ERROR DETECTED ---\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"[offset %llu] %s\n\0" as *const u8 as *const libc::c_char,
        (if !(rdbstate.rio).is_null() {
            (*rdbstate.rio).processed_bytes
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_ulonglong,
        msg.as_mut_ptr(),
    );
    printf(
        b"[additional info] While doing: %s\n\0" as *const u8 as *const libc::c_char,
        rdb_check_doing_string[rdbstate.doing as usize],
    );
    if !(rdbstate.key).is_null() {
        printf(
            b"[additional info] Reading key '%s'\n\0" as *const u8
                as *const libc::c_char,
            (*rdbstate.key).ptr as *mut libc::c_char,
        );
    }
    if rdbstate.key_type != -(1 as libc::c_int) {
        printf(
            b"[additional info] Reading type %d (%s)\n\0" as *const u8
                as *const libc::c_char,
            rdbstate.key_type,
            if (rdbstate.key_type as libc::c_uint as libc::c_ulong)
                < (core::mem::size_of::<[*mut libc::c_char; 19]>() as libc::c_ulong)
                    .wrapping_div(
                        core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
            {
                rdb_type_string[rdbstate.key_type as usize] as *const libc::c_char
            } else {
                b"unknown\0" as *const u8 as *const libc::c_char
            },
        );
    }
    rdbShowGenericInfo();
}
#[no_mangle]
pub unsafe extern "C" fn rdbCheckInfo(mut fmt: *const libc::c_char, mut args: ...) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    printf(
        b"[offset %llu] %s\n\0" as *const u8 as *const libc::c_char,
        (if !(rdbstate.rio).is_null() {
            (*rdbstate.rio).processed_bytes
        } else {
            0 as libc::c_int as libc::c_ulong
        }) as libc::c_ulonglong,
        msg.as_mut_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn rdbCheckSetError(mut fmt: *const libc::c_char, mut args: ...) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    vsnprintf(
        (rdbstate.error).as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    rdbstate.error_set = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn rdbCheckHandleCrash(
    mut sig: libc::c_int,
    mut info: *mut siginfo_t,
    mut secret: *mut libc::c_void,
) {
    rdbCheckError(
        b"Server crash checking the specified RDB file!\0" as *const u8
            as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn rdbCheckSetupSignals() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_14 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut act.sa_mask);
    act
        .sa_flags = (0x40000000 as libc::c_int as libc::c_uint
        | 0x80000000 as libc::c_uint | 4 as libc::c_int as libc::c_uint) as libc::c_int;
    act
        .__sigaction_handler
        .sa_sigaction = Some(
        rdbCheckHandleCrash
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(11 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(7 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(8 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(4 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(6 as libc::c_int, &mut act, 0 as *mut sigaction);
}
#[no_mangle]
pub unsafe extern "C" fn redis_check_rdb(
    mut rdbfilename: *mut libc::c_char,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dbid: uint64_t = 0;
    let mut selected_dbid: libc::c_int = -(1 as libc::c_int);
    let mut type_0: libc::c_int = 0;
    let mut rdbver: libc::c_int = 0;
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut expiretime: libc::c_longlong = 0;
    let mut now: libc::c_longlong = mstime();
    static mut rdb: rio = rio {
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
                ptr: 0 as *const libc::c_char as *mut libc::c_char,
                pos: 0,
            },
        },
    };
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
    let mut closefile: libc::c_int = (fp == 0 as *mut libc::c_void as *mut FILE)
        as libc::c_int;
    if fp.is_null()
        && {
            fp = fopen(rdbfilename, b"r\0" as *const u8 as *const libc::c_char);
            fp.is_null()
        }
    {
        return 1 as libc::c_int;
    }
    if fstat(fileno(fp), &mut sb) == -(1 as libc::c_int) {
        sb.st_size = 0 as libc::c_int as __off64_t;
    }
    startLoadingFile(sb.st_size as size_t, rdbfilename, 0 as libc::c_int);
    rioInitWithFile(&mut rdb, fp);
    rdbstate.rio = &mut rdb;
    rdb
        .update_cksum = Some(
        rdbLoadProgressCallback
            as unsafe extern "C" fn(*mut rio, *const libc::c_void, size_t) -> (),
    );
    if rioRead(
        &mut rdb,
        buf.as_mut_ptr() as *mut libc::c_void,
        9 as libc::c_int as size_t,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        current_block = 6512628852423894791;
    } else {
        buf[9 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
        if memcmp(
            buf.as_mut_ptr() as *const libc::c_void,
            b"REDIS\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            5 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
        {
            rdbCheckError(
                b"Wrong signature trying to load DB from file\0" as *const u8
                    as *const libc::c_char,
            );
            current_block = 12507377084108038387;
        } else {
            rdbver = atoi(buf.as_mut_ptr().offset(5 as libc::c_int as isize));
            if rdbver < 1 as libc::c_int || rdbver > 10 as libc::c_int {
                rdbCheckError(
                    b"Can't handle RDB format version %d\0" as *const u8
                        as *const libc::c_char,
                    rdbver,
                );
                current_block = 12507377084108038387;
            } else {
                expiretime = -(1 as libc::c_int) as libc::c_longlong;
                loop {
                    let mut key: *mut robj = 0 as *mut robj;
                    let mut val: *mut robj = 0 as *mut robj;
                    rdbstate.doing = 1 as libc::c_int;
                    type_0 = rdbLoadType(&mut rdb);
                    if type_0 == -(1 as libc::c_int) {
                        current_block = 6512628852423894791;
                        break;
                    }
                    if type_0 == 253 as libc::c_int {
                        rdbstate.doing = 2 as libc::c_int;
                        expiretime = rdbLoadTime(&mut rdb) as libc::c_longlong;
                        expiretime *= 1000 as libc::c_int as libc::c_longlong;
                        if rioGetReadError(&mut rdb) != 0 {
                            current_block = 6512628852423894791;
                            break;
                        }
                    } else if type_0 == 252 as libc::c_int {
                        rdbstate.doing = 2 as libc::c_int;
                        expiretime = rdbLoadMillisecondTime(&mut rdb, rdbver);
                        if rioGetReadError(&mut rdb) != 0 {
                            current_block = 6512628852423894791;
                            break;
                        }
                    } else if type_0 == 249 as libc::c_int {
                        let mut byte: uint8_t = 0;
                        if rioRead(
                            &mut rdb,
                            &mut byte as *mut uint8_t as *mut libc::c_void,
                            1 as libc::c_int as size_t,
                        ) == 0 as libc::c_int as libc::c_ulong
                        {
                            current_block = 6512628852423894791;
                            break;
                        }
                    } else if type_0 == 248 as libc::c_int {
                        if rdbLoadLen(&mut rdb, 0 as *mut libc::c_int)
                            == 18446744073709551615 as libc::c_ulong
                        {
                            current_block = 6512628852423894791;
                            break;
                        }
                    } else if type_0 == 255 as libc::c_int {
                        current_block = 10007731352114176167;
                        break;
                    } else if type_0 == 254 as libc::c_int {
                        rdbstate.doing = 6 as libc::c_int;
                        dbid = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if dbid == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                        rdbCheckInfo(
                            b"Selecting DB ID %llu\0" as *const u8
                                as *const libc::c_char,
                            dbid as libc::c_ulonglong,
                        );
                        selected_dbid = dbid as libc::c_int;
                    } else if type_0 == 251 as libc::c_int {
                        let mut db_size: uint64_t = 0;
                        let mut expires_size: uint64_t = 0;
                        rdbstate.doing = 6 as libc::c_int;
                        db_size = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if db_size == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                        expires_size = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if expires_size == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                    } else if type_0 == 250 as libc::c_int {
                        let mut auxkey: *mut robj = 0 as *mut robj;
                        let mut auxval: *mut robj = 0 as *mut robj;
                        rdbstate.doing = 7 as libc::c_int;
                        auxkey = rdbLoadStringObject(&mut rdb);
                        if auxkey.is_null() {
                            current_block = 6512628852423894791;
                            break;
                        }
                        auxval = rdbLoadStringObject(&mut rdb);
                        if auxval.is_null() {
                            decrRefCount(auxkey);
                            current_block = 6512628852423894791;
                            break;
                        } else {
                            rdbCheckInfo(
                                b"AUX FIELD %s = '%s'\0" as *const u8
                                    as *const libc::c_char,
                                (*auxkey).ptr as *mut libc::c_char,
                                (*auxval).ptr as *mut libc::c_char,
                            );
                            decrRefCount(auxkey);
                            decrRefCount(auxval);
                        }
                    } else if type_0 == 247 as libc::c_int {
                        let mut moduleid: uint64_t = 0;
                        let mut when_opcode: uint64_t = 0;
                        let mut when: uint64_t = 0;
                        rdbstate.doing = 8 as libc::c_int;
                        moduleid = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if moduleid == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                        when_opcode = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if when_opcode == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                        when = rdbLoadLen(&mut rdb, 0 as *mut libc::c_int);
                        if when == 18446744073709551615 as libc::c_ulong {
                            current_block = 6512628852423894791;
                            break;
                        }
                        if when_opcode != 2 as libc::c_int as libc::c_ulong {
                            rdbCheckError(
                                b"bad when_opcode\0" as *const u8 as *const libc::c_char,
                            );
                            current_block = 12507377084108038387;
                            break;
                        } else {
                            let mut name: [libc::c_char; 10] = [0; 10];
                            moduleTypeNameByID(name.as_mut_ptr(), moduleid);
                            rdbCheckInfo(
                                b"MODULE AUX for: %s\0" as *const u8 as *const libc::c_char,
                                name.as_mut_ptr(),
                            );
                            let mut o: *mut robj = rdbLoadCheckModuleValue(
                                &mut rdb,
                                name.as_mut_ptr(),
                            );
                            decrRefCount(o);
                        }
                    } else if type_0 == 246 as libc::c_int
                        || type_0 == 245 as libc::c_int
                    {
                        let mut err: sds = 0 as sds;
                        rdbstate.doing = 9 as libc::c_int;
                        if !(rdbFunctionLoad(
                            &mut rdb,
                            rdbver,
                            0 as *mut functionsLibCtx,
                            type_0,
                            0 as libc::c_int,
                            &mut err,
                        ) != 0 as libc::c_int)
                        {
                            continue;
                        }
                        rdbCheckError(
                            b"Failed loading library, %s\0" as *const u8
                                as *const libc::c_char,
                            err,
                        );
                        sdsfree(err);
                        current_block = 12507377084108038387;
                        break;
                    } else if !(type_0 >= 0 as libc::c_int && type_0 <= 7 as libc::c_int
                        || type_0 >= 9 as libc::c_int && type_0 <= 19 as libc::c_int)
                    {
                        rdbCheckError(
                            b"Invalid object type: %d\0" as *const u8
                                as *const libc::c_char,
                            type_0,
                        );
                        current_block = 12507377084108038387;
                        break;
                    } else {
                        rdbstate.key_type = type_0;
                        rdbstate.doing = 3 as libc::c_int;
                        key = rdbLoadStringObject(&mut rdb);
                        if key.is_null() {
                            current_block = 6512628852423894791;
                            break;
                        }
                        rdbstate.key = key;
                        rdbstate.keys = (rdbstate.keys).wrapping_add(1);
                        rdbstate.doing = 4 as libc::c_int;
                        val = rdbLoadObject(
                            type_0,
                            &mut rdb,
                            (*key).ptr as sds,
                            selected_dbid,
                            0 as *mut libc::c_int,
                        );
                        if val.is_null() {
                            current_block = 6512628852423894791;
                            break;
                        }
                        if expiretime != -(1 as libc::c_int) as libc::c_longlong
                            && expiretime < now
                        {
                            rdbstate
                                .already_expired = (rdbstate.already_expired)
                                .wrapping_add(1);
                        }
                        if expiretime != -(1 as libc::c_int) as libc::c_longlong {
                            rdbstate.expires = (rdbstate.expires).wrapping_add(1);
                        }
                        rdbstate.key = 0 as *mut robj;
                        decrRefCount(key);
                        decrRefCount(val);
                        rdbstate.key_type = -(1 as libc::c_int);
                        expiretime = -(1 as libc::c_int) as libc::c_longlong;
                    }
                }
                match current_block {
                    6512628852423894791 => {}
                    12507377084108038387 => {}
                    _ => {
                        if rdbver >= 5 as libc::c_int && server.rdb_checksum != 0 {
                            let mut cksum: uint64_t = 0;
                            let mut expected: uint64_t = rdb.cksum;
                            rdbstate.doing = 5 as libc::c_int;
                            if rioRead(
                                &mut rdb,
                                &mut cksum as *mut uint64_t as *mut libc::c_void,
                                8 as libc::c_int as size_t,
                            ) == 0 as libc::c_int as libc::c_ulong
                            {
                                current_block = 6512628852423894791;
                            } else if cksum == 0 as libc::c_int as libc::c_ulong {
                                rdbCheckInfo(
                                    b"RDB file was saved with checksum disabled: no check performed.\0"
                                        as *const u8 as *const libc::c_char,
                                );
                                current_block = 11869735117417356968;
                            } else if cksum != expected {
                                rdbCheckError(
                                    b"RDB CRC error\0" as *const u8 as *const libc::c_char,
                                );
                                current_block = 12507377084108038387;
                            } else {
                                rdbCheckInfo(
                                    b"Checksum OK\0" as *const u8 as *const libc::c_char,
                                );
                                current_block = 11869735117417356968;
                            }
                        } else {
                            current_block = 11869735117417356968;
                        }
                        match current_block {
                            12507377084108038387 => {}
                            6512628852423894791 => {}
                            _ => {
                                if closefile != 0 {
                                    fclose(fp);
                                }
                                stopLoading(1 as libc::c_int);
                                return 0 as libc::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    match current_block {
        6512628852423894791 => {
            if rdbstate.error_set != 0 {
                rdbCheckError((rdbstate.error).as_mut_ptr());
            } else {
                rdbCheckError(
                    b"Unexpected EOF reading RDB file\0" as *const u8
                        as *const libc::c_char,
                );
            }
        }
        _ => {}
    }
    if closefile != 0 {
        fclose(fp);
    }
    stopLoading(0 as libc::c_int);
    return 1 as libc::c_int;
}
unsafe extern "C" fn checkRdbVersion() -> sds {
    let mut version: sds = 0 as *mut libc::c_char;
    version = sdscatprintf(
        sdsempty(),
        b"%s\0" as *const u8 as *const libc::c_char,
        b"7.0.8\0" as *const u8 as *const libc::c_char,
    );
    if strtoll(redisGitSHA1(), 0 as *mut *mut libc::c_char, 16 as libc::c_int) != 0 {
        version = sdscatprintf(
            version,
            b" (git:%s\0" as *const u8 as *const libc::c_char,
            redisGitSHA1(),
        );
        if strtoll(redisGitDirty(), 0 as *mut *mut libc::c_char, 10 as libc::c_int) != 0
        {
            version = sdscatprintf(
                version,
                b"-dirty\0" as *const u8 as *const libc::c_char,
            );
        }
        version = sdscat(version, b")\0" as *const u8 as *const libc::c_char);
    }
    return version;
}
#[no_mangle]
pub unsafe extern "C" fn redis_check_rdb_main(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut fp: *mut FILE,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    if argc != 2 as libc::c_int && fp.is_null() {
        fprintf(
            stderr,
            b"Usage: %s <rdb-file-name>\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    } else {
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            let mut version: sds = checkRdbVersion();
            printf(
                b"redis-check-rdb %s\n\0" as *const u8 as *const libc::c_char,
                version,
            );
            sdsfree(version);
            exit(0 as libc::c_int);
        }
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    init_genrand64(
        (tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong
            + tv.tv_usec as libc::c_longlong ^ getpid() as libc::c_longlong)
            as libc::c_ulonglong,
    );
    if (shared.integers[0 as libc::c_int as usize]).is_null() {
        createSharedObjects();
    }
    server.loading_process_events_interval_bytes = 0 as libc::c_int as off_t;
    server.sanitize_dump_payload = 1 as libc::c_int;
    rdbCheckMode = 1 as libc::c_int;
    rdbCheckInfo(
        b"Checking RDB file %s\0" as *const u8 as *const libc::c_char,
        *argv.offset(1 as libc::c_int as isize),
    );
    rdbCheckSetupSignals();
    let mut retval: libc::c_int = redis_check_rdb(
        *argv.offset(1 as libc::c_int as isize),
        fp,
    );
    if retval == 0 as libc::c_int {
        rdbCheckInfo(b"\\o/ RDB looks OK! \\o/\0" as *const u8 as *const libc::c_char);
        rdbShowGenericInfo();
    }
    if !fp.is_null() {
        return if retval == 0 as libc::c_int {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    exit(retval);
}

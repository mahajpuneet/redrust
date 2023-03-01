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
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn fixedpoint_d2string(
        dst: *mut libc::c_char,
        dstlen: size_t,
        dvalue: libc::c_double,
        fractional_digits: libc::c_int,
    ) -> libc::c_int;
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    fn addReplyNull(c: *mut client);
    fn addReplyNullArray(c: *mut client);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulkSds(c: *mut client, s: sds);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyHumanLongDouble(c: *mut client, d: f64);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn replaceClientCommandVector(
        c: *mut client,
        argc: libc::c_int,
        argv: *mut *mut robj,
    );
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn createZsetObject() -> *mut robj;
    fn checkType(c: *mut client, o: *mut robj, type_0: libc::c_int) -> libc::c_int;
    fn getLongLongFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_longlong,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn getDoubleFromObjectOrReply(
        c: *mut client,
        o: *mut robj,
        target: *mut libc::c_double,
        msg: *const libc::c_char,
    ) -> libc::c_int;
    fn zslInsert(
        zsl: *mut zskiplist,
        score: libc::c_double,
        ele: sds,
    ) -> *mut zskiplistNode;
    fn zslFirstInRange(
        zsl: *mut zskiplist,
        range: *mut zrangespec,
    ) -> *mut zskiplistNode;
    fn zzlGetScore(sptr: *mut libc::c_uchar) -> libc::c_double;
    fn zzlNext(
        zl: *mut libc::c_uchar,
        eptr: *mut *mut libc::c_uchar,
        sptr: *mut *mut libc::c_uchar,
    );
    fn zzlFirstInRange(
        zl: *mut libc::c_uchar,
        range: *mut zrangespec,
    ) -> *mut libc::c_uchar;
    fn zsetConvertToListpackIfNeeded(
        zobj: *mut robj,
        maxelelen: size_t,
        totelelen: size_t,
    );
    fn zsetScore(
        zobj: *mut robj,
        member: sds,
        score: *mut libc::c_double,
    ) -> libc::c_int;
    fn zslValueLteMax(value: libc::c_double, spec: *mut zrangespec) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn lookupKeyRead(db: *mut redisDb, key: *mut robj) -> *mut robj;
    fn lookupKeyReadOrReply(
        c: *mut client,
        key: *mut robj,
        reply: *mut robj,
    ) -> *mut robj;
    fn setKey(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        val: *mut robj,
        flags: libc::c_int,
    );
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn zaddCommand(c: *mut client);
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn geohashGetCoordRange(long_range: *mut GeoHashRange, lat_range: *mut GeoHashRange);
    fn geohashEncode(
        long_range: *const GeoHashRange,
        lat_range: *const GeoHashRange,
        longitude: libc::c_double,
        latitude: libc::c_double,
        step: uint8_t,
        hash: *mut GeoHashBits,
    ) -> libc::c_int;
    fn geohashEncodeWGS84(
        longitude: libc::c_double,
        latitude: libc::c_double,
        step: uint8_t,
        hash: *mut GeoHashBits,
    ) -> libc::c_int;
    fn geohashDecode(
        long_range: GeoHashRange,
        lat_range: GeoHashRange,
        hash: GeoHashBits,
        area: *mut GeoHashArea,
    ) -> libc::c_int;
    fn geohashCalculateAreasByShapeWGS84(shape: *mut GeoShape) -> GeoHashRadius;
    fn geohashGetDistance(
        lon1d: libc::c_double,
        lat1d: libc::c_double,
        lon2d: libc::c_double,
        lat2d: libc::c_double,
    ) -> libc::c_double;
    fn geohashGetDistanceIfInRectangle(
        width_m: libc::c_double,
        height_m: libc::c_double,
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
        distance: *mut libc::c_double,
    ) -> libc::c_int;
    fn geohashDecodeToLongLatWGS84(
        hash: GeoHashBits,
        xy: *mut libc::c_double,
    ) -> libc::c_int;
    fn geohashAlign52Bits(hash: GeoHashBits) -> GeoHashFix52Bits;
    fn geohashGetDistanceIfInRadiusWGS84(
        x1: libc::c_double,
        y1: libc::c_double,
        x2: libc::c_double,
        y2: libc::c_double,
        radius: libc::c_double,
        distance: *mut libc::c_double,
    ) -> libc::c_int;
    fn pqsort(
        a: *mut libc::c_void,
        n: size_t,
        es: size_t,
        cmp: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        >,
        lrange: size_t,
        rrange: size_t,
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
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct zrangespec {
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub minex: libc::c_int,
    pub maxex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct geoArray {
    pub array: *mut geoPoint,
    pub buckets: size_t,
    pub used: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct geoPoint {
    pub longitude: libc::c_double,
    pub latitude: libc::c_double,
    pub dist: libc::c_double,
    pub score: libc::c_double,
    pub member: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoShape {
    pub type_0: libc::c_int,
    pub xy: [libc::c_double; 2],
    pub conversion: libc::c_double,
    pub bounds: [libc::c_double; 4],
    pub t: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub radius: libc::c_double,
    pub r: C2RustUnnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub height: libc::c_double,
    pub width: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashRadius {
    pub hash: GeoHashBits,
    pub area: GeoHashArea,
    pub neighbors: GeoHashNeighbors,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashNeighbors {
    pub north: GeoHashBits,
    pub east: GeoHashBits,
    pub west: GeoHashBits,
    pub south: GeoHashBits,
    pub north_east: GeoHashBits,
    pub south_east: GeoHashBits,
    pub north_west: GeoHashBits,
    pub south_west: GeoHashBits,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashBits {
    pub bits: uint64_t,
    pub step: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashArea {
    pub hash: GeoHashBits,
    pub longitude: GeoHashRange,
    pub latitude: GeoHashRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GeoHashRange {
    pub min: libc::c_double,
    pub max: libc::c_double,
}
pub type GeoHashFix52Bits = uint64_t;
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
#[no_mangle]
pub unsafe extern "C" fn geoArrayCreate() -> *mut geoArray {
    let mut ga: *mut geoArray = zmalloc(
        core::mem::size_of::<geoArray>() as libc::c_ulong,
    ) as *mut geoArray;
    (*ga).array = 0 as *mut geoPoint;
    (*ga).buckets = 0 as libc::c_int as size_t;
    (*ga).used = 0 as libc::c_int as size_t;
    return ga;
}
#[no_mangle]
pub unsafe extern "C" fn geoArrayAppend(
    mut ga: *mut geoArray,
    mut xy: *mut libc::c_double,
    mut dist: libc::c_double,
    mut score: libc::c_double,
    mut member: *mut libc::c_char,
) -> *mut geoPoint {
    if (*ga).used == (*ga).buckets {
        (*ga)
            .buckets = if (*ga).buckets == 0 as libc::c_int as libc::c_ulong {
            8 as libc::c_int as libc::c_ulong
        } else {
            ((*ga).buckets).wrapping_mul(2 as libc::c_int as libc::c_ulong)
        };
        (*ga)
            .array = zrealloc(
            (*ga).array as *mut libc::c_void,
            (core::mem::size_of::<geoPoint>() as libc::c_ulong)
                .wrapping_mul((*ga).buckets),
        ) as *mut geoPoint;
    }
    let mut gp: *mut geoPoint = ((*ga).array).offset((*ga).used as isize);
    (*gp).longitude = *xy.offset(0 as libc::c_int as isize);
    (*gp).latitude = *xy.offset(1 as libc::c_int as isize);
    (*gp).dist = dist;
    (*gp).member = member;
    (*gp).score = score;
    (*ga).used = ((*ga).used).wrapping_add(1);
    return gp;
}
#[no_mangle]
pub unsafe extern "C" fn geoArrayFree(mut ga: *mut geoArray) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < (*ga).used {
        sdsfree((*((*ga).array).offset(i as isize)).member);
        i = i.wrapping_add(1);
    }
    zfree((*ga).array as *mut libc::c_void);
    zfree(ga as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn decodeGeohash(
    mut bits: libc::c_double,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    let mut hash: GeoHashBits = {
        let mut init = GeoHashBits {
            bits: bits as uint64_t,
            step: 26 as libc::c_int as uint8_t,
        };
        init
    };
    return geohashDecodeToLongLatWGS84(hash, xy);
}
#[no_mangle]
pub unsafe extern "C" fn extractLongLatOrReply(
    mut c: *mut client,
    mut argv: *mut *mut robj,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if getDoubleFromObjectOrReply(
            c,
            *argv.offset(i as isize),
            xy.offset(i as isize),
            0 as *const libc::c_char,
        ) != 0 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        i += 1;
    }
    if *xy.offset(0 as libc::c_int as isize) < -(180 as libc::c_int) as libc::c_double
        || *xy.offset(0 as libc::c_int as isize) > 180 as libc::c_int as libc::c_double
        || *xy.offset(1 as libc::c_int as isize) < -85.05112878f64
        || *xy.offset(1 as libc::c_int as isize) > 85.05112878f64
    {
        addReplyErrorFormat(
            c,
            b"-ERR invalid longitude,latitude pair %f,%f\r\n\0" as *const u8
                as *const libc::c_char,
            *xy.offset(0 as libc::c_int as isize),
            *xy.offset(1 as libc::c_int as isize),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn longLatFromMember(
    mut zobj: *mut robj,
    mut member: *mut robj,
    mut xy: *mut libc::c_double,
) -> libc::c_int {
    let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
    if zsetScore(zobj, (*member).ptr as sds, &mut score) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if decodeGeohash(score, xy) == 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn extractUnitOrReply(
    mut c: *mut client,
    mut unit: *mut robj,
) -> libc::c_double {
    let mut u: *mut libc::c_char = (*unit).ptr as *mut libc::c_char;
    if strcasecmp(u, b"m\0" as *const u8 as *const libc::c_char) == 0 {
        return 1 as libc::c_int as libc::c_double
    } else if strcasecmp(u, b"km\0" as *const u8 as *const libc::c_char) == 0 {
        return 1000 as libc::c_int as libc::c_double
    } else if strcasecmp(u, b"ft\0" as *const u8 as *const libc::c_char) == 0 {
        return 0.3048f64
    } else if strcasecmp(u, b"mi\0" as *const u8 as *const libc::c_char) == 0 {
        return 1609.34f64
    } else {
        addReplyError(
            c,
            b"unsupported unit provided. please use M, KM, FT, MI\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int) as libc::c_double;
    };
}
#[no_mangle]
pub unsafe extern "C" fn extractDistanceOrReply(
    mut c: *mut client,
    mut argv: *mut *mut robj,
    mut conversion: *mut libc::c_double,
    mut radius: *mut libc::c_double,
) -> libc::c_int {
    let mut distance: libc::c_double = 0.;
    if getDoubleFromObjectOrReply(
        c,
        *argv.offset(0 as libc::c_int as isize),
        &mut distance,
        b"need numeric radius\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if distance < 0 as libc::c_int as libc::c_double {
        addReplyError(
            c,
            b"radius cannot be negative\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !radius.is_null() {
        *radius = distance;
    }
    let mut to_meters: libc::c_double = extractUnitOrReply(
        c,
        *argv.offset(1 as libc::c_int as isize),
    );
    if to_meters < 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int);
    }
    if !conversion.is_null() {
        *conversion = to_meters;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn extractBoxOrReply(
    mut c: *mut client,
    mut argv: *mut *mut robj,
    mut conversion: *mut libc::c_double,
    mut width: *mut libc::c_double,
    mut height: *mut libc::c_double,
) -> libc::c_int {
    let mut h: libc::c_double = 0.;
    let mut w: libc::c_double = 0.;
    if getDoubleFromObjectOrReply(
        c,
        *argv.offset(0 as libc::c_int as isize),
        &mut w,
        b"need numeric width\0" as *const u8 as *const libc::c_char,
    ) != 0 as libc::c_int
        || getDoubleFromObjectOrReply(
            c,
            *argv.offset(1 as libc::c_int as isize),
            &mut h,
            b"need numeric height\0" as *const u8 as *const libc::c_char,
        ) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if h < 0 as libc::c_int as libc::c_double || w < 0 as libc::c_int as libc::c_double {
        addReplyError(
            c,
            b"height or width cannot be negative\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if !height.is_null() {
        *height = h;
    }
    if !width.is_null() {
        *width = w;
    }
    let mut to_meters: libc::c_double = extractUnitOrReply(
        c,
        *argv.offset(2 as libc::c_int as isize),
    );
    if to_meters < 0 as libc::c_int as libc::c_double {
        return -(1 as libc::c_int);
    }
    if !conversion.is_null() {
        *conversion = to_meters;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addReplyDoubleDistance(
    mut c: *mut client,
    mut d: libc::c_double,
) {
    let mut dbuf: [libc::c_char; 128] = [0; 128];
    let dlen: libc::c_int = fixedpoint_d2string(
        dbuf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        d,
        4 as libc::c_int,
    );
    addReplyBulkCBuffer(c, dbuf.as_mut_ptr() as *const libc::c_void, dlen as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn geoWithinShape(
    mut shape: *mut GeoShape,
    mut score: libc::c_double,
    mut xy: *mut libc::c_double,
    mut distance: *mut libc::c_double,
) -> libc::c_int {
    if decodeGeohash(score, xy) == 0 {
        return -(1 as libc::c_int);
    }
    if (*shape).type_0 == 1 as libc::c_int {
        if geohashGetDistanceIfInRadiusWGS84(
            (*shape).xy[0 as libc::c_int as usize],
            (*shape).xy[1 as libc::c_int as usize],
            *xy.offset(0 as libc::c_int as isize),
            *xy.offset(1 as libc::c_int as isize),
            (*shape).t.radius * (*shape).conversion,
            distance,
        ) == 0
        {
            return -(1 as libc::c_int);
        }
    } else if (*shape).type_0 == 2 as libc::c_int {
        if geohashGetDistanceIfInRectangle(
            (*shape).t.r.width * (*shape).conversion,
            (*shape).t.r.height * (*shape).conversion,
            (*shape).xy[0 as libc::c_int as usize],
            (*shape).xy[1 as libc::c_int as usize],
            *xy.offset(0 as libc::c_int as isize),
            *xy.offset(1 as libc::c_int as isize),
            distance,
        ) == 0
        {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn geoGetPointsInRange(
    mut zobj: *mut robj,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut shape: *mut GeoShape,
    mut ga: *mut geoArray,
    mut limit: libc::c_ulong,
) -> libc::c_int {
    let mut range: zrangespec = {
        let mut init = zrangespec {
            min: min,
            max: max,
            minex: 0 as libc::c_int,
            maxex: 1 as libc::c_int,
        };
        init
    };
    let mut origincount: size_t = (*ga).used;
    if (*zobj).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*zobj).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut vlong: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
        let mut score: libc::c_double = 0 as libc::c_int as libc::c_double;
        eptr = zzlFirstInRange(zl, &mut range);
        if eptr.is_null() {
            return 0 as libc::c_int;
        }
        sptr = lpNext(zl, eptr);
        while !eptr.is_null() {
            let mut xy: [libc::c_double; 2] = [0.; 2];
            let mut distance: libc::c_double = 0 as libc::c_int as libc::c_double;
            score = zzlGetScore(sptr);
            if zslValueLteMax(score, &mut range) == 0 {
                break;
            }
            vstr = lpGetValue(eptr, &mut vlen, &mut vlong);
            if geoWithinShape(shape, score, xy.as_mut_ptr(), &mut distance)
                == 0 as libc::c_int
            {
                let mut member: *mut libc::c_char = if vstr.is_null() {
                    sdsfromlonglong(vlong)
                } else {
                    sdsnewlen(vstr as *const libc::c_void, vlen as size_t)
                };
                geoArrayAppend(ga, xy.as_mut_ptr(), distance, score, member);
            }
            if (*ga).used != 0 && limit != 0 && (*ga).used >= limit {
                break;
            }
            zzlNext(zl, &mut eptr, &mut sptr);
        }
    } else if (*zobj).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*zobj).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        let mut ln: *mut zskiplistNode = 0 as *mut zskiplistNode;
        ln = zslFirstInRange(zsl, &mut range);
        if ln.is_null() {
            return 0 as libc::c_int;
        }
        while !ln.is_null() {
            let mut xy_0: [libc::c_double; 2] = [0.; 2];
            let mut distance_0: libc::c_double = 0 as libc::c_int as libc::c_double;
            if zslValueLteMax((*ln).score, &mut range) == 0 {
                break;
            }
            if geoWithinShape(shape, (*ln).score, xy_0.as_mut_ptr(), &mut distance_0)
                == 0 as libc::c_int
            {
                geoArrayAppend(
                    ga,
                    xy_0.as_mut_ptr(),
                    distance_0,
                    (*ln).score,
                    sdsdup((*ln).ele),
                );
            }
            if (*ga).used != 0 && limit != 0 && (*ga).used >= limit {
                break;
            }
            ln = (*((*ln).level).as_mut_ptr().offset(0 as libc::c_int as isize)).forward;
        }
    }
    return ((*ga).used).wrapping_sub(origincount) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn scoresOfGeoHashBox(
    mut hash: GeoHashBits,
    mut min: *mut GeoHashFix52Bits,
    mut max: *mut GeoHashFix52Bits,
) {
    *min = geohashAlign52Bits(hash);
    hash.bits = (hash.bits).wrapping_add(1);
    *max = geohashAlign52Bits(hash);
}
#[no_mangle]
pub unsafe extern "C" fn membersOfGeoHashBox(
    mut zobj: *mut robj,
    mut hash: GeoHashBits,
    mut ga: *mut geoArray,
    mut shape: *mut GeoShape,
    mut limit: libc::c_ulong,
) -> libc::c_int {
    let mut min: GeoHashFix52Bits = 0;
    let mut max: GeoHashFix52Bits = 0;
    scoresOfGeoHashBox(hash, &mut min, &mut max);
    return geoGetPointsInRange(
        zobj,
        min as libc::c_double,
        max as libc::c_double,
        shape,
        ga,
        limit,
    );
}
#[no_mangle]
pub unsafe extern "C" fn membersOfAllNeighbors(
    mut zobj: *mut robj,
    mut n: *const GeoHashRadius,
    mut shape: *mut GeoShape,
    mut ga: *mut geoArray,
    mut limit: libc::c_ulong,
) -> libc::c_int {
    let mut neighbors: [GeoHashBits; 9] = [GeoHashBits { bits: 0, step: 0 }; 9];
    let mut i: libc::c_uint = 0;
    let mut count: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut last_processed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut debugmsg: libc::c_int = 0 as libc::c_int;
    neighbors[0 as libc::c_int as usize] = (*n).hash;
    neighbors[1 as libc::c_int as usize] = (*n).neighbors.north;
    neighbors[2 as libc::c_int as usize] = (*n).neighbors.south;
    neighbors[3 as libc::c_int as usize] = (*n).neighbors.east;
    neighbors[4 as libc::c_int as usize] = (*n).neighbors.west;
    neighbors[5 as libc::c_int as usize] = (*n).neighbors.north_east;
    neighbors[6 as libc::c_int as usize] = (*n).neighbors.north_west;
    neighbors[7 as libc::c_int as usize] = (*n).neighbors.south_east;
    neighbors[8 as libc::c_int as usize] = (*n).neighbors.south_west;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (core::mem::size_of::<[GeoHashBits; 9]>() as libc::c_ulong)
            .wrapping_div(core::mem::size_of::<GeoHashBits>() as libc::c_ulong)
    {
        if neighbors[i as usize].bits == 0 && neighbors[i as usize].step == 0 {
            if debugmsg != 0 {
                let mut fp: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    384 as libc::c_int,
                );
                fprintf(
                    fp,
                    b"neighbors[%d] is zero\0" as *const u8 as *const libc::c_char,
                    i,
                );
                fprintf(fp, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp);
            }
        } else {
            if debugmsg != 0 {
                let mut long_range: GeoHashRange = GeoHashRange { min: 0., max: 0. };
                let mut lat_range: GeoHashRange = GeoHashRange { min: 0., max: 0. };
                geohashGetCoordRange(&mut long_range, &mut lat_range);
                let mut myarea: GeoHashArea = {
                    let mut init = GeoHashArea {
                        hash: {
                            let mut init = GeoHashBits {
                                bits: 0 as libc::c_int as uint64_t,
                                step: 0,
                            };
                            init
                        },
                        longitude: GeoHashRange { min: 0., max: 0. },
                        latitude: GeoHashRange { min: 0., max: 0. },
                    };
                    init
                };
                geohashDecode(long_range, lat_range, neighbors[i as usize], &mut myarea);
                let mut fp_0: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_0,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    396 as libc::c_int,
                );
                fprintf(
                    fp_0,
                    b"neighbors[%d]:\n\0" as *const u8 as *const libc::c_char,
                    i,
                );
                fprintf(fp_0, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_0);
                let mut fp_1: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_1,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    397 as libc::c_int,
                );
                fprintf(
                    fp_1,
                    b"area.longitude.min: %f\n\0" as *const u8 as *const libc::c_char,
                    myarea.longitude.min,
                );
                fprintf(fp_1, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_1);
                let mut fp_2: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_2,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    398 as libc::c_int,
                );
                fprintf(
                    fp_2,
                    b"area.longitude.max: %f\n\0" as *const u8 as *const libc::c_char,
                    myarea.longitude.max,
                );
                fprintf(fp_2, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_2);
                let mut fp_3: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_3,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    399 as libc::c_int,
                );
                fprintf(
                    fp_3,
                    b"area.latitude.min: %f\n\0" as *const u8 as *const libc::c_char,
                    myarea.latitude.min,
                );
                fprintf(fp_3, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_3);
                let mut fp_4: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_4,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    400 as libc::c_int,
                );
                fprintf(
                    fp_4,
                    b"area.latitude.max: %f\n\0" as *const u8 as *const libc::c_char,
                    myarea.latitude.max,
                );
                fprintf(fp_4, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_4);
                let mut fp_5: *mut FILE = fopen(
                    b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                    b"a\0" as *const u8 as *const libc::c_char,
                );
                fprintf(
                    fp_5,
                    b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    (*core::mem::transmute::<
                        &[u8; 22],
                        &[libc::c_char; 22],
                    >(b"membersOfAllNeighbors\0"))
                        .as_ptr(),
                    401 as libc::c_int,
                );
                fprintf(fp_5, b"\n\0" as *const u8 as *const libc::c_char);
                fprintf(fp_5, b"\n\0" as *const u8 as *const libc::c_char);
                fclose(fp_5);
            }
            if last_processed != 0
                && neighbors[i as usize].bits == neighbors[last_processed as usize].bits
                && neighbors[i as usize].step as libc::c_int
                    == neighbors[last_processed as usize].step as libc::c_int
            {
                if debugmsg != 0 {
                    let mut fp_6: *mut FILE = fopen(
                        b"/tmp/log.txt\0" as *const u8 as *const libc::c_char,
                        b"a\0" as *const u8 as *const libc::c_char,
                    );
                    fprintf(
                        fp_6,
                        b"%s:%s:%d:\t\0" as *const u8 as *const libc::c_char,
                        b"geo.c\0" as *const u8 as *const libc::c_char,
                        (*core::mem::transmute::<
                            &[u8; 22],
                            &[libc::c_char; 22],
                        >(b"membersOfAllNeighbors\0"))
                            .as_ptr(),
                        413 as libc::c_int,
                    );
                    fprintf(
                        fp_6,
                        b"Skipping processing of %d, same as previous\n\0" as *const u8
                            as *const libc::c_char,
                        i,
                    );
                    fprintf(fp_6, b"\n\0" as *const u8 as *const libc::c_char);
                    fclose(fp_6);
                }
            } else {
                if (*ga).used != 0 && limit != 0 && (*ga).used >= limit {
                    break;
                }
                count = count
                    .wrapping_add(
                        membersOfGeoHashBox(
                            zobj,
                            neighbors[i as usize],
                            ga,
                            shape,
                            limit,
                        ) as libc::c_uint,
                    );
                last_processed = i;
            }
        }
        i = i.wrapping_add(1);
    }
    return count as libc::c_int;
}
unsafe extern "C" fn sort_gp_asc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut gpa: *const geoPoint = a as *const geoPoint;
    let mut gpb: *const geoPoint = b as *const geoPoint;
    if (*gpa).dist > (*gpb).dist {
        return 1 as libc::c_int
    } else if (*gpa).dist == (*gpb).dist {
        return 0 as libc::c_int
    } else {
        return -(1 as libc::c_int)
    };
}
unsafe extern "C" fn sort_gp_desc(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    return -sort_gp_asc(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn geoaddCommand(mut c: *mut client) {
    let mut xx: libc::c_int = 0 as libc::c_int;
    let mut nx: libc::c_int = 0 as libc::c_int;
    let mut longidx: libc::c_int = 2 as libc::c_int;
    let mut i: libc::c_int = 0;
    while longidx < (*c).argc {
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(longidx as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"nx\0" as *const u8 as *const libc::c_char) == 0 {
            nx = 1 as libc::c_int;
        } else if strcasecmp(opt, b"xx\0" as *const u8 as *const libc::c_char) == 0 {
            xx = 1 as libc::c_int;
        } else if !(strcasecmp(opt, b"ch\0" as *const u8 as *const libc::c_char) == 0) {
            break;
        }
        longidx += 1;
    }
    if ((*c).argc - longidx) % 3 as libc::c_int != 0 || xx != 0 && nx != 0 {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    let mut elements: libc::c_int = ((*c).argc - longidx) / 3 as libc::c_int;
    let mut argc: libc::c_int = longidx + elements * 2 as libc::c_int;
    let mut argv: *mut *mut robj = zcalloc(
        (argc as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<*mut robj>() as libc::c_ulong),
    ) as *mut *mut robj;
    let ref mut fresh0 = *argv.offset(0 as libc::c_int as isize);
    *fresh0 = createRawStringObject(
        b"zadd\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    i = 1 as libc::c_int;
    while i < longidx {
        let ref mut fresh1 = *argv.offset(i as isize);
        *fresh1 = *((*c).argv).offset(i as isize);
        incrRefCount(*argv.offset(i as isize));
        i += 1;
    }
    i = 0 as libc::c_int;
    while i < elements {
        let mut xy: [libc::c_double; 2] = [0.; 2];
        if extractLongLatOrReply(
            c,
            ((*c).argv).offset(longidx as isize).offset((i * 3 as libc::c_int) as isize),
            xy.as_mut_ptr(),
        ) == -(1 as libc::c_int)
        {
            i = 0 as libc::c_int;
            while i < argc {
                if !(*argv.offset(i as isize)).is_null() {
                    decrRefCount(*argv.offset(i as isize));
                }
                i += 1;
            }
            zfree(argv as *mut libc::c_void);
            return;
        }
        let mut hash: GeoHashBits = GeoHashBits { bits: 0, step: 0 };
        geohashEncodeWGS84(
            xy[0 as libc::c_int as usize],
            xy[1 as libc::c_int as usize],
            26 as libc::c_int as uint8_t,
            &mut hash,
        );
        let mut bits: GeoHashFix52Bits = geohashAlign52Bits(hash);
        let mut score: *mut robj = createObject(
            0 as libc::c_int,
            sdsfromlonglong(bits as libc::c_longlong) as *mut libc::c_void,
        );
        let mut val: *mut robj = *((*c).argv)
            .offset((longidx + i * 3 as libc::c_int + 2 as libc::c_int) as isize);
        let ref mut fresh2 = *argv.offset((longidx + i * 2 as libc::c_int) as isize);
        *fresh2 = score;
        let ref mut fresh3 = *argv
            .offset((longidx + 1 as libc::c_int + i * 2 as libc::c_int) as isize);
        *fresh3 = val;
        incrRefCount(val);
        i += 1;
    }
    replaceClientCommandVector(c, argc, argv);
    zaddCommand(c);
}
#[no_mangle]
pub unsafe extern "C" fn georadiusGeneric(
    mut c: *mut client,
    mut srcKeyIndex: libc::c_int,
    mut flags: libc::c_int,
) {
    let mut storekey: *mut robj = 0 as *mut robj;
    let mut storedist: libc::c_int = 0 as libc::c_int;
    let mut zobj: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(srcKeyIndex as isize),
    );
    if checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    let mut base_args: libc::c_int = 0;
    let mut shape: GeoShape = {
        let mut init = GeoShape {
            type_0: 0 as libc::c_int,
            xy: [0.; 2],
            conversion: 0.,
            bounds: [0.; 4],
            t: C2RustUnnamed_7 { radius: 0. },
        };
        init
    };
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        base_args = 6 as libc::c_int;
        shape.type_0 = 1 as libc::c_int;
        if extractLongLatOrReply(
            c,
            ((*c).argv).offset(2 as libc::c_int as isize),
            (shape.xy).as_mut_ptr(),
        ) == -(1 as libc::c_int)
        {
            return;
        }
        if extractDistanceOrReply(
            c,
            ((*c).argv).offset(base_args as isize).offset(-(2 as libc::c_int as isize)),
            &mut shape.conversion,
            &mut shape.t.radius,
        ) != 0 as libc::c_int
        {
            return;
        }
    } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 && zobj.is_null() {
        base_args = 5 as libc::c_int;
    } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        base_args = 5 as libc::c_int;
        shape.type_0 = 1 as libc::c_int;
        let mut member: *mut robj = *((*c).argv).offset(2 as libc::c_int as isize);
        if longLatFromMember(zobj, member, (shape.xy).as_mut_ptr())
            == -(1 as libc::c_int)
        {
            addReplyError(
                c,
                b"could not decode requested zset member\0" as *const u8
                    as *const libc::c_char,
            );
            return;
        }
        if extractDistanceOrReply(
            c,
            ((*c).argv).offset(base_args as isize).offset(-(2 as libc::c_int as isize)),
            &mut shape.conversion,
            &mut shape.t.radius,
        ) != 0 as libc::c_int
        {
            return;
        }
    } else if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        base_args = 2 as libc::c_int;
        if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
            base_args = 3 as libc::c_int;
            storekey = *((*c).argv).offset(1 as libc::c_int as isize);
        }
    } else {
        addReplyError(
            c,
            b"Unknown georadius search type\0" as *const u8 as *const libc::c_char,
        );
        return;
    }
    let mut withdist: libc::c_int = 0 as libc::c_int;
    let mut withhash: libc::c_int = 0 as libc::c_int;
    let mut withcoords: libc::c_int = 0 as libc::c_int;
    let mut frommember: libc::c_int = 0 as libc::c_int;
    let mut fromloc: libc::c_int = 0 as libc::c_int;
    let mut byradius: libc::c_int = 0 as libc::c_int;
    let mut bybox: libc::c_int = 0 as libc::c_int;
    let mut sort: libc::c_int = 0 as libc::c_int;
    let mut any: libc::c_int = 0 as libc::c_int;
    let mut count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if (*c).argc > base_args {
        let mut remaining: libc::c_int = (*c).argc - base_args;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < remaining {
            let mut arg: *mut libc::c_char = (**((*c).argv)
                .offset((base_args + i) as isize))
                .ptr as *mut libc::c_char;
            if strcasecmp(arg, b"withdist\0" as *const u8 as *const libc::c_char) == 0 {
                withdist = 1 as libc::c_int;
            } else if strcasecmp(arg, b"withhash\0" as *const u8 as *const libc::c_char)
                == 0
            {
                withhash = 1 as libc::c_int;
            } else if strcasecmp(arg, b"withcoord\0" as *const u8 as *const libc::c_char)
                == 0
            {
                withcoords = 1 as libc::c_int;
            } else if strcasecmp(arg, b"any\0" as *const u8 as *const libc::c_char) == 0
            {
                any = 1 as libc::c_int;
            } else if strcasecmp(arg, b"asc\0" as *const u8 as *const libc::c_char) == 0
            {
                sort = 1 as libc::c_int;
            } else if strcasecmp(arg, b"desc\0" as *const u8 as *const libc::c_char) == 0
            {
                sort = 2 as libc::c_int;
            } else if strcasecmp(arg, b"count\0" as *const u8 as *const libc::c_char)
                == 0 && (i + 1 as libc::c_int) < remaining
            {
                if getLongLongFromObjectOrReply(
                    c,
                    *((*c).argv).offset((base_args + i + 1 as libc::c_int) as isize),
                    &mut count,
                    0 as *const libc::c_char,
                ) != 0 as libc::c_int
                {
                    return;
                }
                if count <= 0 as libc::c_int as libc::c_longlong {
                    addReplyError(
                        c,
                        b"COUNT must be > 0\0" as *const u8 as *const libc::c_char,
                    );
                    return;
                }
                i += 1;
            } else if strcasecmp(arg, b"store\0" as *const u8 as *const libc::c_char)
                == 0 && (i + 1 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 2 as libc::c_int == 0
                && flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            {
                storekey = *((*c).argv)
                    .offset((base_args + i + 1 as libc::c_int) as isize);
                storedist = 0 as libc::c_int;
                i += 1;
            } else if strcasecmp(arg, b"storedist\0" as *const u8 as *const libc::c_char)
                == 0 && (i + 1 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 2 as libc::c_int == 0
                && flags & (1 as libc::c_int) << 3 as libc::c_int == 0
            {
                storekey = *((*c).argv)
                    .offset((base_args + i + 1 as libc::c_int) as isize);
                storedist = 1 as libc::c_int;
                i += 1;
            } else if strcasecmp(arg, b"storedist\0" as *const u8 as *const libc::c_char)
                == 0 && flags & (1 as libc::c_int) << 3 as libc::c_int != 0
                && flags & (1 as libc::c_int) << 4 as libc::c_int != 0
            {
                storedist = 1 as libc::c_int;
            } else if strcasecmp(
                arg,
                b"frommember\0" as *const u8 as *const libc::c_char,
            ) == 0 && (i + 1 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && fromloc == 0
            {
                if zobj.is_null() {
                    frommember = 1 as libc::c_int;
                    i += 1;
                } else {
                    if longLatFromMember(
                        zobj,
                        *((*c).argv).offset((base_args + i + 1 as libc::c_int) as isize),
                        (shape.xy).as_mut_ptr(),
                    ) == -(1 as libc::c_int)
                    {
                        addReplyError(
                            c,
                            b"could not decode requested zset member\0" as *const u8
                                as *const libc::c_char,
                        );
                        return;
                    }
                    frommember = 1 as libc::c_int;
                    i += 1;
                }
            } else if strcasecmp(
                arg,
                b"fromlonlat\0" as *const u8 as *const libc::c_char,
            ) == 0 && (i + 2 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && frommember == 0
            {
                if extractLongLatOrReply(
                    c,
                    ((*c).argv)
                        .offset(base_args as isize)
                        .offset(i as isize)
                        .offset(1 as libc::c_int as isize),
                    (shape.xy).as_mut_ptr(),
                ) == -(1 as libc::c_int)
                {
                    return;
                }
                fromloc = 1 as libc::c_int;
                i += 2 as libc::c_int;
            } else if strcasecmp(arg, b"byradius\0" as *const u8 as *const libc::c_char)
                == 0 && (i + 2 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && bybox == 0
            {
                if extractDistanceOrReply(
                    c,
                    ((*c).argv)
                        .offset(base_args as isize)
                        .offset(i as isize)
                        .offset(1 as libc::c_int as isize),
                    &mut shape.conversion,
                    &mut shape.t.radius,
                ) != 0 as libc::c_int
                {
                    return;
                }
                shape.type_0 = 1 as libc::c_int;
                byradius = 1 as libc::c_int;
                i += 2 as libc::c_int;
            } else if strcasecmp(arg, b"bybox\0" as *const u8 as *const libc::c_char)
                == 0 && (i + 3 as libc::c_int) < remaining
                && flags & (1 as libc::c_int) << 3 as libc::c_int != 0 && byradius == 0
            {
                if extractBoxOrReply(
                    c,
                    ((*c).argv)
                        .offset(base_args as isize)
                        .offset(i as isize)
                        .offset(1 as libc::c_int as isize),
                    &mut shape.conversion,
                    &mut shape.t.r.width,
                    &mut shape.t.r.height,
                ) != 0 as libc::c_int
                {
                    return;
                }
                shape.type_0 = 2 as libc::c_int;
                bybox = 1 as libc::c_int;
                i += 3 as libc::c_int;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            i += 1;
        }
    }
    if !storekey.is_null() && (withdist != 0 || withhash != 0 || withcoords != 0) {
        addReplyErrorFormat(
            c,
            b"%s is not compatible with WITHDIST, WITHHASH and WITHCOORD options\0"
                as *const u8 as *const libc::c_char,
            if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                b"GEOSEARCHSTORE\0" as *const u8 as *const libc::c_char
            } else {
                b"STORE option in GEORADIUS\0" as *const u8 as *const libc::c_char
            },
        );
        return;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0
        && !(frommember != 0 || fromloc != 0)
    {
        addReplyErrorFormat(
            c,
            b"exactly one of FROMMEMBER or FROMLONLAT can be specified for %s\0"
                as *const u8 as *const libc::c_char,
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
        );
        return;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0
        && !(byradius != 0 || bybox != 0)
    {
        addReplyErrorFormat(
            c,
            b"exactly one of BYRADIUS and BYBOX can be specified for %s\0" as *const u8
                as *const libc::c_char,
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
        );
        return;
    }
    if any != 0 && count == 0 {
        addReplyErrorFormat(
            c,
            b"the ANY argument requires COUNT argument\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if zobj.is_null() {
        if !storekey.is_null() {
            if dbDelete((*c).db, storekey) != 0 {
                signalModifiedKey(c, (*c).db, storekey);
                notifyKeyspaceEvent(
                    (1 as libc::c_int) << 2 as libc::c_int,
                    b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    storekey,
                    (*(*c).db).id,
                );
                server.dirty += 1;
            }
            addReply(c, shared.czero);
        } else {
            addReply(c, shared.emptyarray);
        }
        return;
    }
    if count != 0 as libc::c_int as libc::c_longlong && sort == 0 as libc::c_int
        && any == 0
    {
        sort = 1 as libc::c_int;
    }
    let mut georadius: GeoHashRadius = geohashCalculateAreasByShapeWGS84(&mut shape);
    let mut ga: *mut geoArray = geoArrayCreate();
    membersOfAllNeighbors(
        zobj,
        &mut georadius,
        &mut shape,
        ga,
        (if any != 0 { count } else { 0 as libc::c_int as libc::c_longlong })
            as libc::c_ulong,
    );
    if (*ga).used == 0 as libc::c_int as libc::c_ulong && storekey.is_null() {
        addReply(c, shared.emptyarray);
        geoArrayFree(ga);
        return;
    }
    let mut result_length: libc::c_long = (*ga).used as libc::c_long;
    let mut returned_items: libc::c_long = (if count
        == 0 as libc::c_int as libc::c_longlong
        || (result_length as libc::c_longlong) < count
    {
        result_length as libc::c_longlong
    } else {
        count
    }) as libc::c_long;
    let mut option_length: libc::c_long = 0 as libc::c_int as libc::c_long;
    if sort != 0 as libc::c_int {
        let mut sort_gp_callback: Option::<
            unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
        > = None;
        if sort == 1 as libc::c_int {
            sort_gp_callback = Some(
                sort_gp_asc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
        } else if sort == 2 as libc::c_int {
            sort_gp_callback = Some(
                sort_gp_desc
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            );
        }
        if returned_items == result_length {
            qsort(
                (*ga).array as *mut libc::c_void,
                result_length as size_t,
                core::mem::size_of::<geoPoint>() as libc::c_ulong,
                sort_gp_callback,
            );
        } else {
            pqsort(
                (*ga).array as *mut libc::c_void,
                result_length as size_t,
                core::mem::size_of::<geoPoint>() as libc::c_ulong,
                sort_gp_callback,
                0 as libc::c_int as size_t,
                (returned_items - 1 as libc::c_int as libc::c_long) as size_t,
            );
        }
    }
    if storekey.is_null() {
        if withdist != 0 {
            option_length += 1;
        }
        if withcoords != 0 {
            option_length += 1;
        }
        if withhash != 0 {
            option_length += 1;
        }
        addReplyArrayLen(c, returned_items);
        let mut i_0: libc::c_int = 0;
        i_0 = 0 as libc::c_int;
        while (i_0 as libc::c_long) < returned_items {
            let mut gp: *mut geoPoint = ((*ga).array).offset(i_0 as isize);
            (*gp).dist /= shape.conversion;
            if option_length != 0 {
                addReplyArrayLen(c, option_length + 1 as libc::c_int as libc::c_long);
            }
            addReplyBulkSds(c, (*gp).member);
            (*gp).member = 0 as *mut libc::c_char;
            if withdist != 0 {
                addReplyDoubleDistance(c, (*gp).dist);
            }
            if withhash != 0 {
                addReplyLongLong(c, (*gp).score as libc::c_longlong);
            }
            if withcoords != 0 {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                addReplyHumanLongDouble(c, ((*gp).longitude) as f64);
                addReplyHumanLongDouble(c, ((*gp).latitude) as f64);
            }
            i_0 += 1;
        }
    } else {
        let mut zobj_0: *mut robj = 0 as *mut robj;
        let mut zs: *mut zset = 0 as *mut zset;
        let mut i_1: libc::c_int = 0;
        let mut maxelelen: size_t = 0 as libc::c_int as size_t;
        let mut totelelen: size_t = 0 as libc::c_int as size_t;
        if returned_items != 0 {
            zobj_0 = createZsetObject();
            zs = (*zobj_0).ptr as *mut zset;
        }
        i_1 = 0 as libc::c_int;
        while (i_1 as libc::c_long) < returned_items {
            let mut znode: *mut zskiplistNode = 0 as *mut zskiplistNode;
            let mut gp_0: *mut geoPoint = ((*ga).array).offset(i_1 as isize);
            (*gp_0).dist /= shape.conversion;
            let mut score: libc::c_double = if storedist != 0 {
                (*gp_0).dist
            } else {
                (*gp_0).score
            };
            let mut elelen: size_t = sdslen((*gp_0).member);
            if maxelelen < elelen {
                maxelelen = elelen;
            }
            totelelen = (totelelen as libc::c_ulong).wrapping_add(elelen) as size_t
                as size_t;
            znode = zslInsert((*zs).zsl, score, (*gp_0).member);
            if dictAdd(
                (*zs).dict,
                (*gp_0).member as *mut libc::c_void,
                &mut (*znode).score as *mut libc::c_double as *mut libc::c_void,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"dictAdd(zs->dict,gp->member,&znode->score) == DICT_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"geo.c\0" as *const u8 as *const libc::c_char,
                    825 as libc::c_int,
                );
                unreachable!();
            };
            (*gp_0).member = 0 as *mut libc::c_char;
            i_1 += 1;
        }
        if returned_items != 0 {
            zsetConvertToListpackIfNeeded(zobj_0, maxelelen, totelelen);
            setKey(c, (*c).db, storekey, zobj_0, 0 as libc::c_int);
            decrRefCount(zobj_0);
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 7 as libc::c_int,
                (if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
                    b"geosearchstore\0" as *const u8 as *const libc::c_char
                } else {
                    b"georadiusstore\0" as *const u8 as *const libc::c_char
                }) as *mut libc::c_char,
                storekey,
                (*(*c).db).id,
            );
            server.dirty += returned_items as libc::c_longlong;
        } else if dbDelete((*c).db, storekey) != 0 {
            signalModifiedKey(c, (*c).db, storekey);
            notifyKeyspaceEvent(
                (1 as libc::c_int) << 2 as libc::c_int,
                b"del\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                storekey,
                (*(*c).db).id,
            );
            server.dirty += 1;
        }
        addReplyLongLong(c, returned_items as libc::c_longlong);
    }
    geoArrayFree(ga);
}
#[no_mangle]
pub unsafe extern "C" fn georadiusCommand(mut c: *mut client) {
    georadiusGeneric(c, 1 as libc::c_int, (1 as libc::c_int) << 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn georadiusbymemberCommand(mut c: *mut client) {
    georadiusGeneric(c, 1 as libc::c_int, (1 as libc::c_int) << 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn georadiusroCommand(mut c: *mut client) {
    georadiusGeneric(
        c,
        1 as libc::c_int,
        (1 as libc::c_int) << 0 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn georadiusbymemberroCommand(mut c: *mut client) {
    georadiusGeneric(
        c,
        1 as libc::c_int,
        (1 as libc::c_int) << 1 as libc::c_int | (1 as libc::c_int) << 2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geosearchCommand(mut c: *mut client) {
    georadiusGeneric(c, 1 as libc::c_int, (1 as libc::c_int) << 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn geosearchstoreCommand(mut c: *mut client) {
    georadiusGeneric(
        c,
        2 as libc::c_int,
        (1 as libc::c_int) << 3 as libc::c_int | (1 as libc::c_int) << 4 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn geohashCommand(mut c: *mut client) {
    let mut geoalphabet: *mut libc::c_char = b"0123456789bcdefghjkmnpqrstuvwxyz\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    let mut j: libc::c_int = 0;
    let mut zobj: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    if checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
    j = 2 as libc::c_int;
    while j < (*c).argc {
        let mut score: libc::c_double = 0.;
        if zobj.is_null()
            || zsetScore(zobj, (**((*c).argv).offset(j as isize)).ptr as sds, &mut score)
                == -(1 as libc::c_int)
        {
            addReplyNull(c);
        } else {
            let mut xy: [libc::c_double; 2] = [0.; 2];
            if decodeGeohash(score, xy.as_mut_ptr()) == 0 {
                addReplyNull(c);
            } else {
                let mut r: [GeoHashRange; 2] = [GeoHashRange { min: 0., max: 0. }; 2];
                let mut hash: GeoHashBits = GeoHashBits { bits: 0, step: 0 };
                r[0 as libc::c_int as usize]
                    .min = -(180 as libc::c_int) as libc::c_double;
                r[0 as libc::c_int as usize].max = 180 as libc::c_int as libc::c_double;
                r[1 as libc::c_int as usize]
                    .min = -(90 as libc::c_int) as libc::c_double;
                r[1 as libc::c_int as usize].max = 90 as libc::c_int as libc::c_double;
                geohashEncode(
                    &mut *r.as_mut_ptr().offset(0 as libc::c_int as isize),
                    &mut *r.as_mut_ptr().offset(1 as libc::c_int as isize),
                    xy[0 as libc::c_int as usize],
                    xy[1 as libc::c_int as usize],
                    26 as libc::c_int as uint8_t,
                    &mut hash,
                );
                let mut buf: [libc::c_char; 12] = [0; 12];
                let mut i: libc::c_int = 0;
                i = 0 as libc::c_int;
                while i < 11 as libc::c_int {
                    let mut idx: libc::c_int = 0;
                    if i == 10 as libc::c_int {
                        idx = 0 as libc::c_int;
                    } else {
                        idx = (hash.bits
                            >> 52 as libc::c_int
                                - (i + 1 as libc::c_int) * 5 as libc::c_int
                            & 0x1f as libc::c_int as libc::c_ulong) as libc::c_int;
                    }
                    buf[i as usize] = *geoalphabet.offset(idx as isize);
                    i += 1;
                }
                buf[11 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                addReplyBulkCBuffer(
                    c,
                    buf.as_mut_ptr() as *const libc::c_void,
                    11 as libc::c_int as size_t,
                );
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn geoposCommand(mut c: *mut client) {
    let mut j: libc::c_int = 0;
    let mut zobj: *mut robj = lookupKeyRead(
        (*c).db,
        *((*c).argv).offset(1 as libc::c_int as isize),
    );
    if checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
    j = 2 as libc::c_int;
    while j < (*c).argc {
        let mut score: libc::c_double = 0.;
        if zobj.is_null()
            || zsetScore(zobj, (**((*c).argv).offset(j as isize)).ptr as sds, &mut score)
                == -(1 as libc::c_int)
        {
            addReplyNullArray(c);
        } else {
            let mut xy: [libc::c_double; 2] = [0.; 2];
            if decodeGeohash(score, xy.as_mut_ptr()) == 0 {
                addReplyNullArray(c);
            } else {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                addReplyHumanLongDouble(
                    c,
                    (xy[0 as libc::c_int as usize]) as f64,
                );
                addReplyHumanLongDouble(
                    c,
                    (xy[1 as libc::c_int as usize]) as f64,
                );
            }
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn geodistCommand(mut c: *mut client) {
    let mut to_meter: libc::c_double = 1 as libc::c_int as libc::c_double;
    if (*c).argc == 5 as libc::c_int {
        to_meter = extractUnitOrReply(c, *((*c).argv).offset(4 as libc::c_int as isize));
        if to_meter < 0 as libc::c_int as libc::c_double {
            return;
        }
    } else if (*c).argc > 5 as libc::c_int {
        addReplyErrorObject(c, shared.syntaxerr);
        return;
    }
    let mut zobj: *mut robj = 0 as *mut robj;
    zobj = lookupKeyReadOrReply(
        c,
        *((*c).argv).offset(1 as libc::c_int as isize),
        shared.null[(*c).resp as usize],
    );
    if zobj.is_null() || checkType(c, zobj, 3 as libc::c_int) != 0 {
        return;
    }
    let mut score1: libc::c_double = 0.;
    let mut score2: libc::c_double = 0.;
    let mut xyxy: [libc::c_double; 4] = [0.; 4];
    if zsetScore(
        zobj,
        (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds,
        &mut score1,
    ) == -(1 as libc::c_int)
        || zsetScore(
            zobj,
            (**((*c).argv).offset(3 as libc::c_int as isize)).ptr as sds,
            &mut score2,
        ) == -(1 as libc::c_int)
    {
        addReplyNull(c);
        return;
    }
    if decodeGeohash(score1, xyxy.as_mut_ptr()) == 0
        || decodeGeohash(score2, xyxy.as_mut_ptr().offset(2 as libc::c_int as isize))
            == 0
    {
        addReplyNull(c);
    } else {
        addReplyDoubleDistance(
            c,
            geohashGetDistance(
                xyxy[0 as libc::c_int as usize],
                xyxy[1 as libc::c_int as usize],
                xyxy[2 as libc::c_int as usize],
                xyxy[3 as libc::c_int as usize],
            ) / to_meter,
        );
    };
}

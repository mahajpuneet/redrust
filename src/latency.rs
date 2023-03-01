extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    pub type clusterState;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn time(__timer: *mut time_t) -> time_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn hdr_iter_log_init(
        iter: *mut hdr_iter,
        h: *const hdr_histogram,
        value_units_first_bucket: int64_t,
        log_base: libc::c_double,
    );
    fn hdr_iter_next(iter: *mut hdr_iter) -> bool;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGenHashFunction(key: *const libc::c_void, len: size_t) -> uint64_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn zmalloc_get_smap_bytes_by_field(
        field: *mut libc::c_char,
        pid: libc::c_long,
    ) -> size_t;
    static mut server: redisServer;
    fn createSparklineSequence() -> *mut sequence;
    fn sparklineSequenceAddSample(
        seq: *mut sequence,
        value: libc::c_double,
        label: *mut libc::c_char,
    );
    fn freeSparklineSequence(seq: *mut sequence);
    fn sparklineRender(
        output: sds,
        seq: *mut sequence,
        columns: libc::c_int,
        rows: libc::c_int,
        flags: libc::c_int,
    ) -> sds;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn lookupCommandBySds(s: sds) -> *mut redisCommand;
    fn dictVanillaFree(d: *mut dict, val: *mut libc::c_void);
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
pub type pthread_t = libc::c_ulong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_iter_percentiles {
    pub seen_last_value: bool,
    pub ticks_per_half_distance: int32_t,
    pub percentile_to_iterate_to: libc::c_double,
    pub percentile: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_iter_recorded {
    pub count_added_in_this_iteration_step: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_iter_linear {
    pub value_units_per_bucket: int64_t,
    pub count_added_in_this_iteration_step: int64_t,
    pub next_value_reporting_level: int64_t,
    pub next_value_reporting_level_lowest_equivalent: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_iter_log {
    pub log_base: libc::c_double,
    pub count_added_in_this_iteration_step: int64_t,
    pub next_value_reporting_level: int64_t,
    pub next_value_reporting_level_lowest_equivalent: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_iter {
    pub h: *const hdr_histogram,
    pub counts_index: int32_t,
    pub total_count: int64_t,
    pub count: int64_t,
    pub cumulative_count: int64_t,
    pub value: int64_t,
    pub highest_equivalent_value: int64_t,
    pub lowest_equivalent_value: int64_t,
    pub median_equivalent_value: int64_t,
    pub value_iterated_from: int64_t,
    pub value_iterated_to: int64_t,
    pub specifics: C2RustUnnamed,
    pub _next_fp: Option::<unsafe extern "C" fn(*mut hdr_iter) -> bool>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub percentiles: hdr_iter_percentiles,
    pub recorded: hdr_iter_recorded,
    pub linear: hdr_iter_linear,
    pub log: hdr_iter_log,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latencySample {
    pub time: int32_t,
    pub latency: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latencyTimeSeries {
    pub idx: libc::c_int,
    pub max: uint32_t,
    pub samples: [latencySample; 160],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct latencyStats {
    pub all_time_high: uint32_t,
    pub avg: uint32_t,
    pub min: uint32_t,
    pub max: uint32_t,
    pub mad: uint32_t,
    pub samples: uint32_t,
    pub period: time_t,
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
    pub inst_metric: [C2RustUnnamed_7; 5],
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
pub struct sentinelConfig {
    pub pre_monitor_cfg: *mut list,
    pub monitor_cfg: *mut list,
    pub post_monitor_cfg: *mut list,
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
pub struct rax {
    pub head: *mut raxNode,
    pub numele: uint64_t,
    pub numnodes: uint64_t,
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
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
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
pub type robj = redisObject;
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
    pub bs: C2RustUnnamed_4,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub range: C2RustUnnamed_3,
    pub keynum: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
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
pub union C2RustUnnamed_4 {
    pub index: C2RustUnnamed_6,
    pub keyword: C2RustUnnamed_5,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
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
pub struct user {
    pub name: sds,
    pub flags: uint32_t,
    pub passwords: *mut list,
    pub selectors: *mut list,
    pub acl_string: *mut robj,
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
pub struct replBacklog {
    pub ref_repl_buf_node: *mut listNode,
    pub unindexed_count: size_t,
    pub blocks_index: *mut rax,
    pub histlen: libc::c_longlong,
    pub offset: libc::c_longlong,
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
pub struct redisOp {
    pub argv: *mut *mut robj,
    pub argc: libc::c_int,
    pub dbid: libc::c_int,
    pub target: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveparam {
    pub seconds: time_t,
    pub changes: libc::c_int,
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
pub struct aofInfo {
    pub file_name: sds,
    pub file_seq: libc::c_longlong,
    pub file_type: aof_file_type,
}
pub type aof_file_type = libc::c_uint;
pub const AOF_FILE_TYPE_INCR: aof_file_type = 105;
pub const AOF_FILE_TYPE_HIST: aof_file_type = 104;
pub const AOF_FILE_TYPE_BASE: aof_file_type = 98;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientBufferLimitsConfig {
    pub hard_limit_bytes: libc::c_ulonglong,
    pub soft_limit_bytes: libc::c_ulonglong,
    pub soft_limit_seconds: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
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
pub struct pause_event {
    pub type_0: pause_type,
    pub end: mstime_t,
}
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socketFds {
    pub fd: [libc::c_int; 16],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sample {
    pub value: libc::c_double,
    pub label: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sequence {
    pub length: libc::c_int,
    pub labels: libc::c_int,
    pub samples: *mut sample,
    pub min: libc::c_double,
    pub max: libc::c_double,
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
#[no_mangle]
pub unsafe extern "C" fn dictStringKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return (strcmp(key1 as *const libc::c_char, key2 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictStringHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenHashFunction(key, strlen(key as *const libc::c_char));
}
#[no_mangle]
pub static mut latencyTimeSeriesDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictStringHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictStringKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictVanillaFree
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictVanillaFree
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn THPGetAnonHugePagesSize() -> libc::c_int {
    return zmalloc_get_smap_bytes_by_field(
        b"AnonHugePages:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        -(1 as libc::c_int) as libc::c_long,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn latencyMonitorInit() {
    server.latency_events = dictCreate(&mut latencyTimeSeriesDictType);
}
#[no_mangle]
pub unsafe extern "C" fn latencyAddSample(
    mut event: *const libc::c_char,
    mut latency: mstime_t,
) {
    let mut ts: *mut latencyTimeSeries = dictFetchValue(
        server.latency_events,
        event as *const libc::c_void,
    ) as *mut latencyTimeSeries;
    let mut now: time_t = time(0 as *mut time_t);
    let mut prev: libc::c_int = 0;
    if ts.is_null() {
        ts = zmalloc(core::mem::size_of::<latencyTimeSeries>() as libc::c_ulong)
            as *mut latencyTimeSeries;
        (*ts).idx = 0 as libc::c_int;
        (*ts).max = 0 as libc::c_int as uint32_t;
        memset(
            ((*ts).samples).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[latencySample; 160]>() as libc::c_ulong,
        );
        dictAdd(
            server.latency_events,
            zstrdup(event) as *mut libc::c_void,
            ts as *mut libc::c_void,
        );
    }
    if latency > (*ts).max as libc::c_longlong {
        (*ts).max = latency as uint32_t;
    }
    prev = ((*ts).idx + 160 as libc::c_int - 1 as libc::c_int) % 160 as libc::c_int;
    if (*ts).samples[prev as usize].time as libc::c_long == now {
        if latency > (*ts).samples[prev as usize].latency as libc::c_longlong {
            (*ts).samples[prev as usize].latency = latency as uint32_t;
        }
        return;
    }
    (*ts).samples[(*ts).idx as usize].time = now as int32_t;
    (*ts).samples[(*ts).idx as usize].latency = latency as uint32_t;
    (*ts).idx += 1;
    if (*ts).idx == 160 as libc::c_int {
        (*ts).idx = 0 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn latencyResetEvent(
    mut event_to_reset: *mut libc::c_char,
) -> libc::c_int {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut resets: libc::c_int = 0 as libc::c_int;
    di = dictGetSafeIterator(server.latency_events);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut event: *mut libc::c_char = (*de).key as *mut libc::c_char;
        if event_to_reset.is_null()
            || strcasecmp(event, event_to_reset) == 0 as libc::c_int
        {
            dictDelete(server.latency_events, event as *const libc::c_void);
            resets += 1;
        }
    }
    dictReleaseIterator(di);
    return resets;
}
#[no_mangle]
pub unsafe extern "C" fn analyzeLatencyForEvent(
    mut event: *mut libc::c_char,
    mut ls: *mut latencyStats,
) {
    let mut ts: *mut latencyTimeSeries = dictFetchValue(
        server.latency_events,
        event as *const libc::c_void,
    ) as *mut latencyTimeSeries;
    let mut j: libc::c_int = 0;
    let mut sum: uint64_t = 0;
    (*ls)
        .all_time_high = if !ts.is_null() {
        (*ts).max
    } else {
        0 as libc::c_int as libc::c_uint
    };
    (*ls).avg = 0 as libc::c_int as uint32_t;
    (*ls).min = 0 as libc::c_int as uint32_t;
    (*ls).max = 0 as libc::c_int as uint32_t;
    (*ls).mad = 0 as libc::c_int as uint32_t;
    (*ls).samples = 0 as libc::c_int as uint32_t;
    (*ls).period = 0 as libc::c_int as time_t;
    if ts.is_null() {
        return;
    }
    sum = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int;
    while j < 160 as libc::c_int {
        if !((*ts).samples[j as usize].time == 0 as libc::c_int) {
            (*ls).samples = ((*ls).samples).wrapping_add(1);
            if (*ls).samples == 1 as libc::c_int as libc::c_uint {
                (*ls).max = (*ts).samples[j as usize].latency;
                (*ls).min = (*ls).max;
            } else {
                if (*ls).min > (*ts).samples[j as usize].latency {
                    (*ls).min = (*ts).samples[j as usize].latency;
                }
                if (*ls).max < (*ts).samples[j as usize].latency {
                    (*ls).max = (*ts).samples[j as usize].latency;
                }
            }
            sum = (sum as libc::c_ulong)
                .wrapping_add((*ts).samples[j as usize].latency as libc::c_ulong)
                as uint64_t as uint64_t;
            if (*ls).period == 0 as libc::c_int as libc::c_long
                || ((*ts).samples[j as usize].time as libc::c_long) < (*ls).period
            {
                (*ls).period = (*ts).samples[j as usize].time as time_t;
            }
        }
        j += 1;
    }
    if (*ls).samples != 0 {
        (*ls).avg = sum.wrapping_div((*ls).samples as libc::c_ulong) as uint32_t;
        (*ls).period = time(0 as *mut time_t) - (*ls).period;
        if (*ls).period == 0 as libc::c_int as libc::c_long {
            (*ls).period = 1 as libc::c_int as time_t;
        }
    }
    sum = 0 as libc::c_int as uint64_t;
    j = 0 as libc::c_int;
    while j < 160 as libc::c_int {
        let mut delta: int64_t = 0;
        if !((*ts).samples[j as usize].time == 0 as libc::c_int) {
            delta = (*ls).avg as int64_t
                - (*ts).samples[j as usize].latency as libc::c_long;
            if delta < 0 as libc::c_int as libc::c_long {
                delta = -delta;
            }
            sum = (sum as libc::c_ulong).wrapping_add(delta as libc::c_ulong) as uint64_t
                as uint64_t;
        }
        j += 1;
    }
    if (*ls).samples != 0 {
        (*ls).mad = sum.wrapping_div((*ls).samples as libc::c_ulong) as uint32_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn createLatencyReport() -> sds {
    let mut report: sds = sdsempty();
    let mut advise_better_vm: libc::c_int = 0 as libc::c_int;
    let mut advise_slowlog_enabled: libc::c_int = 0 as libc::c_int;
    let mut advise_slowlog_tuning: libc::c_int = 0 as libc::c_int;
    let mut advise_slowlog_inspect: libc::c_int = 0 as libc::c_int;
    let mut advise_disk_contention: libc::c_int = 0 as libc::c_int;
    let mut advise_scheduler: libc::c_int = 0 as libc::c_int;
    let mut advise_data_writeback: libc::c_int = 0 as libc::c_int;
    let mut advise_no_appendfsync: libc::c_int = 0 as libc::c_int;
    let mut advise_local_disk: libc::c_int = 0 as libc::c_int;
    let mut advise_ssd: libc::c_int = 0 as libc::c_int;
    let mut advise_write_load_info: libc::c_int = 0 as libc::c_int;
    let mut advise_hz: libc::c_int = 0 as libc::c_int;
    let mut advise_large_objects: libc::c_int = 0 as libc::c_int;
    let mut advise_mass_eviction: libc::c_int = 0 as libc::c_int;
    let mut advise_relax_fsync_policy: libc::c_int = 0 as libc::c_int;
    let mut advise_disable_thp: libc::c_int = 0 as libc::c_int;
    let mut advices: libc::c_int = 0 as libc::c_int;
    if ((*server.latency_events).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*server.latency_events).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
        && server.latency_monitor_threshold == 0 as libc::c_int as libc::c_longlong
    {
        report = sdscat(
            report,
            b"I'm sorry, Dave, I can't do that. Latency monitoring is disabled in this Redis instance. You may use \"CONFIG SET latency-monitor-threshold <milliseconds>.\" in order to enable it. If we weren't in a deep space mission I'd suggest to take a look at https://redis.io/topics/latency-monitor.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return report;
    }
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut eventnum: libc::c_int = 0 as libc::c_int;
    di = dictGetSafeIterator(server.latency_events);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut event: *mut libc::c_char = (*de).key as *mut libc::c_char;
        let mut ts: *mut latencyTimeSeries = (*de).v.val as *mut latencyTimeSeries;
        let mut ls: latencyStats = latencyStats {
            all_time_high: 0,
            avg: 0,
            min: 0,
            max: 0,
            mad: 0,
            samples: 0,
            period: 0,
        };
        if ts.is_null() {
            continue;
        }
        eventnum += 1;
        if eventnum == 1 as libc::c_int {
            report = sdscat(
                report,
                b"Dave, I have observed latency spikes in this Redis instance. You don't mind talking about it, do you Dave?\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        analyzeLatencyForEvent(event, &mut ls);
        report = sdscatprintf(
            report,
            b"%d. %s: %d latency spikes (average %lums, mean deviation %lums, period %.2f sec). Worst all time event %lums.\0"
                as *const u8 as *const libc::c_char,
            eventnum,
            event,
            ls.samples,
            ls.avg as libc::c_ulong,
            ls.mad as libc::c_ulong,
            ls.period as libc::c_double / ls.samples as libc::c_double,
            (*ts).max as libc::c_ulong,
        );
        if strcasecmp(event, b"fork\0" as *const u8 as *const libc::c_char) == 0 {
            let mut fork_quality: *mut libc::c_char = 0 as *mut libc::c_char;
            if server.stat_fork_rate < 10 as libc::c_int as libc::c_double {
                fork_quality = b"terrible\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                advise_better_vm = 1 as libc::c_int;
                advices += 1;
            } else if server.stat_fork_rate < 25 as libc::c_int as libc::c_double {
                fork_quality = b"poor\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
                advise_better_vm = 1 as libc::c_int;
                advices += 1;
            } else if server.stat_fork_rate < 100 as libc::c_int as libc::c_double {
                fork_quality = b"good\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else {
                fork_quality = b"excellent\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
            report = sdscatprintf(
                report,
                b" Fork rate is %.2f GB/sec (%s).\0" as *const u8 as *const libc::c_char,
                server.stat_fork_rate,
                fork_quality,
            );
        }
        if strcasecmp(event, b"command\0" as *const u8 as *const libc::c_char) == 0 {
            if server.slowlog_log_slower_than < 0 as libc::c_int as libc::c_longlong {
                advise_slowlog_enabled = 1 as libc::c_int;
                advices += 1;
            } else if server.slowlog_log_slower_than
                / 1000 as libc::c_int as libc::c_longlong
                > server.latency_monitor_threshold
            {
                advise_slowlog_tuning = 1 as libc::c_int;
                advices += 1;
            }
            advise_slowlog_inspect = 1 as libc::c_int;
            advise_large_objects = 1 as libc::c_int;
            advices += 2 as libc::c_int;
        }
        if strcasecmp(event, b"fast-command\0" as *const u8 as *const libc::c_char) == 0
        {
            advise_scheduler = 1 as libc::c_int;
            advices += 1;
        }
        if strcasecmp(
            event,
            b"aof-write-pending-fsync\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            advise_local_disk = 1 as libc::c_int;
            advise_disk_contention = 1 as libc::c_int;
            advise_ssd = 1 as libc::c_int;
            advise_data_writeback = 1 as libc::c_int;
            advices += 4 as libc::c_int;
        }
        if strcasecmp(
            event,
            b"aof-write-active-child\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            advise_no_appendfsync = 1 as libc::c_int;
            advise_data_writeback = 1 as libc::c_int;
            advise_ssd = 1 as libc::c_int;
            advices += 3 as libc::c_int;
        }
        if strcasecmp(event, b"aof-write-alone\0" as *const u8 as *const libc::c_char)
            == 0
        {
            advise_local_disk = 1 as libc::c_int;
            advise_data_writeback = 1 as libc::c_int;
            advise_ssd = 1 as libc::c_int;
            advices += 3 as libc::c_int;
        }
        if strcasecmp(event, b"aof-fsync-always\0" as *const u8 as *const libc::c_char)
            == 0
        {
            advise_relax_fsync_policy = 1 as libc::c_int;
            advices += 1;
        }
        if strcasecmp(event, b"aof-fstat\0" as *const u8 as *const libc::c_char) == 0
            || strcasecmp(
                event,
                b"rdb-unlink-temp-file\0" as *const u8 as *const libc::c_char,
            ) == 0
        {
            advise_disk_contention = 1 as libc::c_int;
            advise_local_disk = 1 as libc::c_int;
            advices += 2 as libc::c_int;
        }
        if strcasecmp(
            event,
            b"aof-rewrite-diff-write\0" as *const u8 as *const libc::c_char,
        ) == 0
            || strcasecmp(event, b"aof-rename\0" as *const u8 as *const libc::c_char)
                == 0
        {
            advise_write_load_info = 1 as libc::c_int;
            advise_data_writeback = 1 as libc::c_int;
            advise_ssd = 1 as libc::c_int;
            advise_local_disk = 1 as libc::c_int;
            advices += 4 as libc::c_int;
        }
        if strcasecmp(event, b"expire-cycle\0" as *const u8 as *const libc::c_char) == 0
        {
            advise_hz = 1 as libc::c_int;
            advise_large_objects = 1 as libc::c_int;
            advices += 2 as libc::c_int;
        }
        if strcasecmp(event, b"eviction-del\0" as *const u8 as *const libc::c_char) == 0
        {
            advise_large_objects = 1 as libc::c_int;
            advices += 1;
        }
        if strcasecmp(event, b"eviction-cycle\0" as *const u8 as *const libc::c_char)
            == 0
        {
            advise_mass_eviction = 1 as libc::c_int;
            advices += 1;
        }
        report = sdscatlen(
            report,
            b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
    }
    dictReleaseIterator(di);
    if THPGetAnonHugePagesSize() > 0 as libc::c_int {
        advise_disable_thp = 1 as libc::c_int;
        advices += 1;
    }
    if eventnum == 0 as libc::c_int && advices == 0 as libc::c_int {
        report = sdscat(
            report,
            b"Dave, no latency spike was observed during the lifetime of this Redis instance, not in the slightest bit. I honestly think you ought to sit down calmly, take a stress pill, and think things over.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if eventnum > 0 as libc::c_int && advices == 0 as libc::c_int {
        report = sdscat(
            report,
            b"\nWhile there are latency events logged, I'm not able to suggest any easy fix. Please use the Redis community to get some help, providing this report in your help request.\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else {
        report = sdscat(
            report,
            b"\nI have a few advices for you:\n\n\0" as *const u8 as *const libc::c_char,
        );
        if advise_better_vm != 0 {
            report = sdscat(
                report,
                b"- If you are using a virtual machine, consider upgrading it with a faster one using a hypervisior that provides less latency during fork() calls. Xen is known to have poor fork() performance. Even in the context of the same VM provider, certain kinds of instances can execute fork faster than others.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_slowlog_enabled != 0 {
            report = sdscatprintf(
                report,
                b"- There are latency issues with potentially slow commands you are using. Try to enable the Slow Log Redis feature using the command 'CONFIG SET slowlog-log-slower-than %llu'. If the Slow log is disabled Redis is not able to log slow commands execution for you.\n\0"
                    as *const u8 as *const libc::c_char,
                (server.latency_monitor_threshold as libc::c_ulonglong)
                    .wrapping_mul(1000 as libc::c_int as libc::c_ulonglong),
            );
        }
        if advise_slowlog_tuning != 0 {
            report = sdscatprintf(
                report,
                b"- Your current Slow Log configuration only logs events that are slower than your configured latency monitor threshold. Please use 'CONFIG SET slowlog-log-slower-than %llu'.\n\0"
                    as *const u8 as *const libc::c_char,
                (server.latency_monitor_threshold as libc::c_ulonglong)
                    .wrapping_mul(1000 as libc::c_int as libc::c_ulonglong),
            );
        }
        if advise_slowlog_inspect != 0 {
            report = sdscat(
                report,
                b"- Check your Slow Log to understand what are the commands you are running which are too slow to execute. Please check https://redis.io/commands/slowlog for more information.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_scheduler != 0 {
            report = sdscat(
                report,
                b"- The system is slow to execute Redis code paths not containing system calls. This usually means the system does not provide Redis CPU time to run for long periods. You should try to:\n  1) Lower the system load.\n  2) Use a computer / VM just for Redis if you are running other software in the same system.\n  3) Check if you have a \"noisy neighbour\" problem.\n  4) Check with 'redis-cli --intrinsic-latency 100' what is the intrinsic latency in your system.\n  5) Check if the problem is allocator-related by recompiling Redis with MALLOC=libc, if you are using Jemalloc. However this may create fragmentation problems.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_local_disk != 0 {
            report = sdscat(
                report,
                b"- It is strongly advised to use local disks for persistence, especially if you are using AOF. Remote disks provided by platform-as-a-service providers are known to be slow.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_ssd != 0 {
            report = sdscat(
                report,
                b"- SSD disks are able to reduce fsync latency, and total time needed for snapshotting and AOF log rewriting (resulting in smaller memory usage). With extremely high write load SSD disks can be a good option. However Redis should perform reasonably with high load using normal disks. Use this advice as a last resort.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_data_writeback != 0 {
            report = sdscat(
                report,
                b"- Mounting ext3/4 filesystems with data=writeback can provide a performance boost compared to data=ordered, however this mode of operation provides less guarantees, and sometimes it can happen that after a hard crash the AOF file will have a half-written command at the end and will require to be repaired before Redis restarts.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_disk_contention != 0 {
            report = sdscat(
                report,
                b"- Try to lower the disk contention. This is often caused by other disk intensive processes running in the same computer (including other Redis instances).\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_no_appendfsync != 0 {
            report = sdscat(
                report,
                b"- Assuming from the point of view of data safety this is viable in your environment, you could try to enable the 'no-appendfsync-on-rewrite' option, so that fsync will not be performed while there is a child rewriting the AOF file or producing an RDB file (the moment where there is high disk contention).\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_relax_fsync_policy != 0 && server.aof_fsync == 1 as libc::c_int {
            report = sdscat(
                report,
                b"- Your fsync policy is set to 'always'. It is very hard to get good performances with such a setup, if possible try to relax the fsync policy to 'onesec'.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_write_load_info != 0 {
            report = sdscat(
                report,
                b"- Latency during the AOF atomic rename operation or when the final difference is flushed to the AOF file at the end of the rewrite, sometimes is caused by very high write load, causing the AOF buffer to get very large. If possible try to send less commands to accomplish the same work, or use Lua scripts to group multiple operations into a single EVALSHA call.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_hz != 0 && server.hz < 100 as libc::c_int {
            report = sdscat(
                report,
                b"- In order to make the Redis keys expiring process more incremental, try to set the 'hz' configuration parameter to 100 using 'CONFIG SET hz 100'.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_large_objects != 0 {
            report = sdscat(
                report,
                b"- Deleting, expiring or evicting (because of maxmemory policy) large objects is a blocking operation. If you have very large objects that are often deleted, expired, or evicted, try to fragment those objects into multiple smaller objects.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_mass_eviction != 0 {
            report = sdscat(
                report,
                b"- Sudden changes to the 'maxmemory' setting via 'CONFIG SET', or allocation of large objects via sets or sorted sets intersections, STORE option of SORT, Redis Cluster large keys migrations (RESTORE command), may create sudden memory pressure forcing the server to block trying to evict keys. \n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        if advise_disable_thp != 0 {
            report = sdscat(
                report,
                b"- I detected a non zero amount of anonymous huge pages used by your process. This creates very serious latency events in different conditions, especially when Redis is persisting on disk. To disable THP support use the command 'echo never > /sys/kernel/mm/transparent_hugepage/enabled', make sure to also add it into /etc/rc.local so that the command will be executed again after a reboot. Note that even if you have already disabled THP, you still need to restart the Redis process to get rid of the huge pages already created.\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
    return report;
}
#[no_mangle]
pub unsafe extern "C" fn fillCommandCDF(
    mut c: *mut client,
    mut histogram: *mut hdr_histogram,
) {
    addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
    addReplyBulkCString(c, b"calls\0" as *const u8 as *const libc::c_char);
    addReplyLongLong(c, (*histogram).total_count as libc::c_longlong);
    addReplyBulkCString(c, b"histogram_usec\0" as *const u8 as *const libc::c_char);
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut samples: libc::c_int = 0 as libc::c_int;
    let mut iter: hdr_iter = hdr_iter {
        h: 0 as *const hdr_histogram,
        counts_index: 0,
        total_count: 0,
        count: 0,
        cumulative_count: 0,
        value: 0,
        highest_equivalent_value: 0,
        lowest_equivalent_value: 0,
        median_equivalent_value: 0,
        value_iterated_from: 0,
        value_iterated_to: 0,
        specifics: C2RustUnnamed {
            percentiles: hdr_iter_percentiles {
                seen_last_value: false,
                ticks_per_half_distance: 0,
                percentile_to_iterate_to: 0.,
                percentile: 0.,
            },
        },
        _next_fp: None,
    };
    hdr_iter_log_init(
        &mut iter,
        histogram,
        1024 as libc::c_int as int64_t,
        2 as libc::c_int as libc::c_double,
    );
    let mut previous_count: int64_t = 0 as libc::c_int as int64_t;
    while hdr_iter_next(&mut iter) {
        let micros: int64_t = iter.highest_equivalent_value
            / 1000 as libc::c_int as libc::c_long;
        let cumulative_count: int64_t = iter.cumulative_count;
        if cumulative_count > previous_count {
            addReplyLongLong(c, micros as libc::c_longlong);
            addReplyLongLong(c, cumulative_count as libc::c_longlong);
            samples += 1;
        }
        previous_count = cumulative_count;
    }
    setDeferredMapLen(c, replylen, samples as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn latencyAllCommandsFillCDF(
    mut c: *mut client,
    mut commands: *mut dict,
    mut command_with_data: *mut libc::c_int,
) {
    let mut di: *mut dictIterator = dictGetSafeIterator(commands);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        cmd = (*de).v.val as *mut redisCommand;
        if !((*cmd).latency_histogram).is_null() {
            addReplyBulkCBuffer(
                c,
                (*cmd).fullname as *const libc::c_void,
                sdslen((*cmd).fullname),
            );
            fillCommandCDF(c, (*cmd).latency_histogram);
            *command_with_data += 1;
        }
        if !((*cmd).subcommands).is_null() {
            latencyAllCommandsFillCDF(c, (*cmd).subcommands_dict, command_with_data);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn latencySpecificCommandsFillCDF(mut c: *mut client) {
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut command_with_data: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 2 as libc::c_int;
    while j < (*c).argc {
        let mut cmd: *mut redisCommand = lookupCommandBySds(
            (**((*c).argv).offset(j as isize)).ptr as sds,
        );
        if !cmd.is_null() {
            if !((*cmd).latency_histogram).is_null() {
                addReplyBulkCBuffer(
                    c,
                    (*cmd).fullname as *const libc::c_void,
                    sdslen((*cmd).fullname),
                );
                fillCommandCDF(c, (*cmd).latency_histogram);
                command_with_data += 1;
            }
            if !((*cmd).subcommands_dict).is_null() {
                let mut de: *mut dictEntry = 0 as *mut dictEntry;
                let mut di: *mut dictIterator = dictGetSafeIterator(
                    (*cmd).subcommands_dict,
                );
                loop {
                    de = dictNext(di);
                    if de.is_null() {
                        break;
                    }
                    let mut sub: *mut redisCommand = (*de).v.val as *mut redisCommand;
                    if !((*sub).latency_histogram).is_null() {
                        addReplyBulkCBuffer(
                            c,
                            (*sub).fullname as *const libc::c_void,
                            sdslen((*sub).fullname),
                        );
                        fillCommandCDF(c, (*sub).latency_histogram);
                        command_with_data += 1;
                    }
                }
                dictReleaseIterator(di);
            }
        }
        j += 1;
    }
    setDeferredMapLen(c, replylen, command_with_data as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn latencyCommandReplyWithSamples(
    mut c: *mut client,
    mut ts: *mut latencyTimeSeries,
) {
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    let mut samples: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 160 as libc::c_int {
        let mut i: libc::c_int = ((*ts).idx + j) % 160 as libc::c_int;
        if !((*ts).samples[i as usize].time == 0 as libc::c_int) {
            addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
            addReplyLongLong(c, (*ts).samples[i as usize].time as libc::c_longlong);
            addReplyLongLong(c, (*ts).samples[i as usize].latency as libc::c_longlong);
            samples += 1;
        }
        j += 1;
    }
    setDeferredArrayLen(c, replylen, samples as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn latencyCommandReplyWithLatestEvents(mut c: *mut client) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    addReplyArrayLen(
        c,
        ((*server.latency_events).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*server.latency_events).ht_used[1 as libc::c_int as usize])
            as libc::c_long,
    );
    di = dictGetIterator(server.latency_events);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut event: *mut libc::c_char = (*de).key as *mut libc::c_char;
        let mut ts: *mut latencyTimeSeries = (*de).v.val as *mut latencyTimeSeries;
        let mut last: libc::c_int = ((*ts).idx + 160 as libc::c_int - 1 as libc::c_int)
            % 160 as libc::c_int;
        addReplyArrayLen(c, 4 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, event);
        addReplyLongLong(c, (*ts).samples[last as usize].time as libc::c_longlong);
        addReplyLongLong(c, (*ts).samples[last as usize].latency as libc::c_longlong);
        addReplyLongLong(c, (*ts).max as libc::c_longlong);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn latencyCommandGenSparkeline(
    mut event: *mut libc::c_char,
    mut ts: *mut latencyTimeSeries,
) -> sds {
    let mut j: libc::c_int = 0;
    let mut seq: *mut sequence = createSparklineSequence();
    let mut graph: sds = sdsempty();
    let mut min: uint32_t = 0 as libc::c_int as uint32_t;
    let mut max: uint32_t = 0 as libc::c_int as uint32_t;
    j = 0 as libc::c_int;
    while j < 160 as libc::c_int {
        let mut i: libc::c_int = ((*ts).idx + j) % 160 as libc::c_int;
        let mut elapsed: libc::c_int = 0;
        let mut buf: [libc::c_char; 64] = [0; 64];
        if !((*ts).samples[i as usize].time == 0 as libc::c_int) {
            if (*seq).length == 0 as libc::c_int {
                max = (*ts).samples[i as usize].latency;
                min = max;
            } else {
                if (*ts).samples[i as usize].latency > max {
                    max = (*ts).samples[i as usize].latency;
                }
                if (*ts).samples[i as usize].latency < min {
                    min = (*ts).samples[i as usize].latency;
                }
            }
            elapsed = (time(0 as *mut time_t)
                - (*ts).samples[i as usize].time as libc::c_long) as libc::c_int;
            if elapsed < 60 as libc::c_int {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%ds\0" as *const u8 as *const libc::c_char,
                    elapsed,
                );
            } else if elapsed < 3600 as libc::c_int {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%dm\0" as *const u8 as *const libc::c_char,
                    elapsed / 60 as libc::c_int,
                );
            } else if elapsed < 3600 as libc::c_int * 24 as libc::c_int {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%dh\0" as *const u8 as *const libc::c_char,
                    elapsed / 3600 as libc::c_int,
                );
            } else {
                snprintf(
                    buf.as_mut_ptr(),
                    core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%dd\0" as *const u8 as *const libc::c_char,
                    elapsed / (3600 as libc::c_int * 24 as libc::c_int),
                );
            }
            sparklineSequenceAddSample(
                seq,
                (*ts).samples[i as usize].latency as libc::c_double,
                buf.as_mut_ptr(),
            );
        }
        j += 1;
    }
    graph = sdscatprintf(
        graph,
        b"%s - high %lu ms, low %lu ms (all time high %lu ms)\n\0" as *const u8
            as *const libc::c_char,
        event,
        max as libc::c_ulong,
        min as libc::c_ulong,
        (*ts).max as libc::c_ulong,
    );
    j = 0 as libc::c_int;
    while j < 80 as libc::c_int {
        graph = sdscatlen(
            graph,
            b"-\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        );
        j += 1;
    }
    graph = sdscatlen(
        graph,
        b"\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    );
    graph = sparklineRender(
        graph,
        seq,
        80 as libc::c_int,
        4 as libc::c_int,
        1 as libc::c_int,
    );
    freeSparklineSequence(seq);
    return graph;
}
#[no_mangle]
pub unsafe extern "C" fn latencyCommand(mut c: *mut client) {
    let mut ts: *mut latencyTimeSeries = 0 as *mut latencyTimeSeries;
    if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"history\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        ts = dictFetchValue(
            server.latency_events,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
        ) as *mut latencyTimeSeries;
        if ts.is_null() {
            addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
        } else {
            latencyCommandReplyWithSamples(c, ts);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"graph\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 3 as libc::c_int
    {
        let mut graph: sds = 0 as *mut libc::c_char;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut event: *mut libc::c_char = 0 as *mut libc::c_char;
        de = dictFind(
            server.latency_events,
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr,
        );
        if de.is_null() {
            addReplyErrorFormat(
                c,
                b"No samples available for event '%s'\0" as *const u8
                    as *const libc::c_char,
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *mut libc::c_char,
            );
            return;
        } else {
            ts = (*de).v.val as *mut latencyTimeSeries;
            event = (*de).key as *mut libc::c_char;
            graph = latencyCommandGenSparkeline(event, ts);
            addReplyVerbatim(
                c,
                graph as *const libc::c_char,
                sdslen(graph),
                b"txt\0" as *const u8 as *const libc::c_char,
            );
            sdsfree(graph);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"latest\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        latencyCommandReplyWithLatestEvents(c);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"doctor\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut report: sds = createLatencyReport();
        addReplyVerbatim(
            c,
            report as *const libc::c_char,
            sdslen(report),
            b"txt\0" as *const u8 as *const libc::c_char,
        );
        sdsfree(report);
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"reset\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 2 as libc::c_int
    {
        if (*c).argc == 2 as libc::c_int {
            addReplyLongLong(
                c,
                latencyResetEvent(0 as *mut libc::c_char) as libc::c_longlong,
            );
        } else {
            let mut j: libc::c_int = 0;
            let mut resets: libc::c_int = 0 as libc::c_int;
            j = 2 as libc::c_int;
            while j < (*c).argc {
                resets
                    += latencyResetEvent(
                        (**((*c).argv).offset(j as isize)).ptr as *mut libc::c_char,
                    );
                j += 1;
            }
            addReplyLongLong(c, resets as libc::c_longlong);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"histogram\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc >= 2 as libc::c_int
    {
        if (*c).argc == 2 as libc::c_int {
            let mut command_with_data: libc::c_int = 0 as libc::c_int;
            let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
            latencyAllCommandsFillCDF(c, server.commands, &mut command_with_data);
            setDeferredMapLen(c, replylen, command_with_data as libc::c_long);
        } else {
            latencySpecificCommandsFillCDF(c);
        }
    } else if strcasecmp(
        (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"help\0" as *const u8 as *const libc::c_char,
    ) == 0 && (*c).argc == 2 as libc::c_int
    {
        let mut help: [*const libc::c_char; 15] = [
            b"DOCTOR\0" as *const u8 as *const libc::c_char,
            b"    Return a human readable latency analysis report.\0" as *const u8
                as *const libc::c_char,
            b"GRAPH <event>\0" as *const u8 as *const libc::c_char,
            b"    Return an ASCII latency graph for the <event> class.\0" as *const u8
                as *const libc::c_char,
            b"HISTORY <event>\0" as *const u8 as *const libc::c_char,
            b"    Return time-latency samples for the <event> class.\0" as *const u8
                as *const libc::c_char,
            b"LATEST\0" as *const u8 as *const libc::c_char,
            b"    Return the latest latency samples for all events.\0" as *const u8
                as *const libc::c_char,
            b"RESET [<event> ...]\0" as *const u8 as *const libc::c_char,
            b"    Reset latency data of one or more <event> classes.\0" as *const u8
                as *const libc::c_char,
            b"    (default: reset all data for all event classes)\0" as *const u8
                as *const libc::c_char,
            b"HISTOGRAM [COMMAND ...]\0" as *const u8 as *const libc::c_char,
            b"    Return a cumulative distribution of latencies in the format of a histogram for the specified command names.\0"
                as *const u8 as *const libc::c_char,
            b"    If no commands are specified then all histograms are replied.\0"
                as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else {
        addReplySubcommandSyntaxError(c);
    };
}

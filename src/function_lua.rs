extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type lua_State;
    pub type RedisModuleCommand;
    pub type clusterSlotToKeyMapping;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn lua_gettop(L: *mut lua_State) -> libc::c_int;
    fn lua_settop(L: *mut lua_State, idx: libc::c_int);
    fn lua_pushvalue(L: *mut lua_State, idx: libc::c_int);
    fn lua_replace(L: *mut lua_State, idx: libc::c_int);
    fn lua_isstring(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_type(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_tolstring(
        L: *mut lua_State,
        idx: libc::c_int,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn lua_pushnil(L: *mut lua_State);
    fn lua_pushnumber(L: *mut lua_State, n: lua_Number);
    fn lua_pushstring(L: *mut lua_State, s: *const libc::c_char);
    fn lua_pushcclosure(L: *mut lua_State, fn_0: lua_CFunction, n: libc::c_int);
    fn lua_gettable(L: *mut lua_State, idx: libc::c_int);
    fn lua_getfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_rawgeti(L: *mut lua_State, idx: libc::c_int, n: libc::c_int);
    fn lua_createtable(L: *mut lua_State, narr: libc::c_int, nrec: libc::c_int);
    fn lua_getmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_settable(L: *mut lua_State, idx: libc::c_int);
    fn lua_setfield(L: *mut lua_State, idx: libc::c_int, k: *const libc::c_char);
    fn lua_setmetatable(L: *mut lua_State, objindex: libc::c_int) -> libc::c_int;
    fn lua_pcall(
        L: *mut lua_State,
        nargs: libc::c_int,
        nresults: libc::c_int,
        errfunc: libc::c_int,
    ) -> libc::c_int;
    fn lua_next(L: *mut lua_State, idx: libc::c_int) -> libc::c_int;
    fn lua_sethook(
        L: *mut lua_State,
        func: lua_Hook,
        mask: libc::c_int,
        count: libc::c_int,
    ) -> libc::c_int;
    fn lua_enablereadonlytable(
        L: *mut lua_State,
        index: libc::c_int,
        enabled: libc::c_int,
    );
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    static mut scripts_flags_def: [scriptFlag; 0];
    fn functionsRegisterEngine(
        engine_name: *const libc::c_char,
        engine_ctx: *mut engine,
    ) -> libc::c_int;
    fn functionLibCreateFunction(
        name: sds,
        function: *mut libc::c_void,
        li: *mut functionLibInfo,
        desc: sds,
        f_flags: uint64_t,
        err: *mut sds,
    ) -> libc::c_int;
    fn luaL_ref(L: *mut lua_State, t: libc::c_int) -> libc::c_int;
    fn luaL_unref(L: *mut lua_State, t: libc::c_int, ref_0: libc::c_int);
    fn luaL_loadbuffer(
        L: *mut lua_State,
        buff: *const libc::c_char,
        sz: size_t,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn luaL_newstate() -> *mut lua_State;
    fn luaSetErrorMetatable(lua: *mut lua_State);
    fn luaRegisterRedisAPI(lua: *mut lua_State);
    fn luaGetStringSds(lua: *mut lua_State, index: libc::c_int) -> sds;
    fn luaSetTableProtectionRecursively(lua: *mut lua_State);
    fn luaRegisterLogFunction(lua: *mut lua_State);
    fn luaRegisterVersion(lua: *mut lua_State);
    fn luaPushError(lua: *mut lua_State, error: *const libc::c_char);
    fn luaError(lua: *mut lua_State) -> libc::c_int;
    fn luaSaveOnRegistry(
        lua: *mut lua_State,
        name: *const libc::c_char,
        ptr: *mut libc::c_void,
    );
    fn luaGetFromRegistry(
        lua: *mut lua_State,
        name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn luaCallFunction(
        r_ctx: *mut scriptRunCtx,
        lua: *mut lua_State,
        keys: *mut *mut robj,
        nkeys: size_t,
        args: *mut *mut robj,
        nargs: size_t,
        debug_enabled: libc::c_int,
    );
    fn luaExtractErrorInformation(lua: *mut lua_State, err_info: *mut errorInfo);
    fn luaErrorInformationDiscard(err_info: *mut errorInfo);
    fn luaMemory(lua: *mut lua_State) -> libc::c_ulong;
}
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type off_t = __off64_t;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type lua_CFunction = Option::<unsafe extern "C" fn(*mut lua_State) -> libc::c_int>;
pub type lua_Number = libc::c_double;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lua_Debug {
    pub event: libc::c_int,
    pub name: *const libc::c_char,
    pub namewhat: *const libc::c_char,
    pub what: *const libc::c_char,
    pub source: *const libc::c_char,
    pub currentline: libc::c_int,
    pub nups: libc::c_int,
    pub linedefined: libc::c_int,
    pub lastlinedefined: libc::c_int,
    pub short_src: [libc::c_char; 60],
    pub i_ci: libc::c_int,
}
pub type lua_Hook = Option::<unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> ()>;
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
pub struct scriptRunCtx {
    pub funcname: *const libc::c_char,
    pub c: *mut client,
    pub original_client: *mut client,
    pub flags: libc::c_int,
    pub repl_flags: libc::c_int,
    pub start_time: monotime,
    pub snapshot_time: mstime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scriptFlag {
    pub flag: uint64_t,
    pub str_0: *const libc::c_char,
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
pub struct luaFunctionCtx {
    pub lua_function_ref: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct luaEngineCtx {
    pub lua: *mut lua_State,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errorInfo {
    pub msg: sds,
    pub source: sds,
    pub line: sds,
    pub ignore_err_stats_update: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct loadCtx {
    pub li: *mut functionLibInfo,
    pub start_time: monotime,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct registerFunctionArgs {
    pub name: sds,
    pub desc: sds,
    pub lua_f_ctx: *mut luaFunctionCtx,
    pub f_flags: uint64_t,
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
unsafe extern "C" fn elapsedUs(mut start_time: monotime) -> uint64_t {
    return (getMonotonicUs.expect("non-null function pointer")())
        .wrapping_sub(start_time);
}
#[inline]
unsafe extern "C" fn elapsedMs(mut start_time: monotime) -> uint64_t {
    return (elapsedUs(start_time)).wrapping_div(1000 as libc::c_int as libc::c_ulong);
}
unsafe extern "C" fn luaEngineLoadHook(mut lua: *mut lua_State, mut ar: *mut lua_Debug) {
    let mut load_ctx: *mut loadCtx = luaGetFromRegistry(
        lua,
        b"__LIBRARY_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut loadCtx;
    let mut duration: uint64_t = elapsedMs((*load_ctx).start_time);
    if duration > 500 as libc::c_int as libc::c_ulong {
        lua_sethook(
            lua,
            Some(
                luaEngineLoadHook
                    as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> (),
            ),
            (1 as libc::c_int) << 2 as libc::c_int,
            0 as libc::c_int,
        );
        luaPushError(
            lua,
            b"FUNCTION LOAD timeout\0" as *const u8 as *const libc::c_char,
        );
        luaError(lua);
    }
}
unsafe extern "C" fn luaEngineCreate(
    mut engine_ctx: *mut libc::c_void,
    mut li: *mut functionLibInfo,
    mut blob: sds,
    mut err: *mut sds,
) -> libc::c_int {
    let mut load_ctx: loadCtx = loadCtx {
        li: 0 as *mut functionLibInfo,
        start_time: 0,
    };
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut lua_engine_ctx: *mut luaEngineCtx = engine_ctx as *mut luaEngineCtx;
    let mut lua: *mut lua_State = (*lua_engine_ctx).lua;
    lua_getmetatable(lua, -(10002 as libc::c_int));
    lua_enablereadonlytable(lua, -(1 as libc::c_int), 0 as libc::c_int);
    lua_getfield(
        lua,
        -(10000 as libc::c_int),
        b"__LIBRARY_API__\0" as *const u8 as *const libc::c_char,
    );
    lua_setfield(
        lua,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_enablereadonlytable(lua, -(10002 as libc::c_int), 1 as libc::c_int);
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    if luaL_loadbuffer(
        lua,
        blob as *const libc::c_char,
        sdslen(blob),
        b"@user_function\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        *err = sdscatprintf(
            sdsempty(),
            b"Error compiling function: %s\0" as *const u8 as *const libc::c_char,
            lua_tolstring(lua, -(1 as libc::c_int), 0 as *mut size_t),
        );
        lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    } else {
        if lua_type(lua, -(1 as libc::c_int)) == 6 as libc::c_int {} else {
            _serverAssert(
                b"lua_isfunction(lua, -1)\0" as *const u8 as *const libc::c_char,
                b"function_lua.c\0" as *const u8 as *const libc::c_char,
                121 as libc::c_int,
            );
            unreachable!();
        };
        load_ctx = {
            let mut init = loadCtx {
                li: li,
                start_time: getMonotonicUs.expect("non-null function pointer")(),
            };
            init
        };
        luaSaveOnRegistry(
            lua,
            b"__LIBRARY_CTX__\0" as *const u8 as *const libc::c_char,
            &mut load_ctx as *mut loadCtx as *mut libc::c_void,
        );
        lua_sethook(
            lua,
            Some(
                luaEngineLoadHook
                    as unsafe extern "C" fn(*mut lua_State, *mut lua_Debug) -> (),
            ),
            (1 as libc::c_int) << 3 as libc::c_int,
            100000 as libc::c_int,
        );
        if lua_pcall(lua, 0 as libc::c_int, 0 as libc::c_int, 0 as libc::c_int) != 0 {
            let mut err_info: errorInfo = {
                let mut init = errorInfo {
                    msg: 0 as sds,
                    source: 0 as *mut libc::c_char,
                    line: 0 as *mut libc::c_char,
                    ignore_err_stats_update: 0,
                };
                init
            };
            luaExtractErrorInformation(lua, &mut err_info);
            *err = sdscatprintf(
                sdsempty(),
                b"Error registering functions: %s\0" as *const u8 as *const libc::c_char,
                err_info.msg,
            );
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            luaErrorInformationDiscard(&mut err_info);
        } else {
            ret = 0 as libc::c_int;
        }
    }
    lua_getmetatable(lua, -(10002 as libc::c_int));
    lua_enablereadonlytable(lua, -(1 as libc::c_int), 0 as libc::c_int);
    lua_getfield(
        lua,
        -(10000 as libc::c_int),
        b"__GLOBALS_API__\0" as *const u8 as *const libc::c_char,
    );
    lua_setfield(
        lua,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_enablereadonlytable(lua, -(10002 as libc::c_int), 1 as libc::c_int);
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_sethook(lua, None, 0 as libc::c_int, 0 as libc::c_int);
    luaSaveOnRegistry(
        lua,
        b"__LIBRARY_CTX__\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
    );
    return ret;
}
unsafe extern "C" fn luaEngineCall(
    mut run_ctx: *mut scriptRunCtx,
    mut engine_ctx: *mut libc::c_void,
    mut compiled_function: *mut libc::c_void,
    mut keys: *mut *mut robj,
    mut nkeys: size_t,
    mut args: *mut *mut robj,
    mut nargs: size_t,
) {
    let mut lua_engine_ctx: *mut luaEngineCtx = engine_ctx as *mut luaEngineCtx;
    let mut lua: *mut lua_State = (*lua_engine_ctx).lua;
    let mut f_ctx: *mut luaFunctionCtx = compiled_function as *mut luaFunctionCtx;
    lua_pushstring(lua, b"__ERROR_HANDLER__\0" as *const u8 as *const libc::c_char);
    lua_gettable(lua, -(10000 as libc::c_int));
    lua_rawgeti(lua, -(10000 as libc::c_int), (*f_ctx).lua_function_ref);
    if lua_type(lua, -(1 as libc::c_int)) == 6 as libc::c_int {} else {
        _serverAssert(
            b"lua_isfunction(lua, -1)\0" as *const u8 as *const libc::c_char,
            b"function_lua.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
        );
        unreachable!();
    };
    luaCallFunction(run_ctx, lua, keys, nkeys, args, nargs, 0 as libc::c_int);
    lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
}
unsafe extern "C" fn luaEngineGetUsedMemoy(mut engine_ctx: *mut libc::c_void) -> size_t {
    let mut lua_engine_ctx: *mut luaEngineCtx = engine_ctx as *mut luaEngineCtx;
    return luaMemory((*lua_engine_ctx).lua);
}
unsafe extern "C" fn luaEngineFunctionMemoryOverhead(
    mut compiled_function: *mut libc::c_void,
) -> size_t {
    return je_malloc_usable_size(compiled_function);
}
unsafe extern "C" fn luaEngineMemoryOverhead(
    mut engine_ctx: *mut libc::c_void,
) -> size_t {
    let mut lua_engine_ctx: *mut luaEngineCtx = engine_ctx as *mut luaEngineCtx;
    return je_malloc_usable_size(lua_engine_ctx as *mut libc::c_void);
}
unsafe extern "C" fn luaEngineFreeFunction(
    mut engine_ctx: *mut libc::c_void,
    mut compiled_function: *mut libc::c_void,
) {
    let mut lua_engine_ctx: *mut luaEngineCtx = engine_ctx as *mut luaEngineCtx;
    let mut lua: *mut lua_State = (*lua_engine_ctx).lua;
    let mut f_ctx: *mut luaFunctionCtx = compiled_function as *mut luaFunctionCtx;
    luaL_unref(lua, -(10000 as libc::c_int), (*f_ctx).lua_function_ref);
    zfree(f_ctx as *mut libc::c_void);
}
unsafe extern "C" fn luaRegisterFunctionArgsInitialize(
    mut register_f_args: *mut registerFunctionArgs,
    mut name: sds,
    mut desc: sds,
    mut lua_f_ctx: *mut luaFunctionCtx,
    mut flags: uint64_t,
) {
    *register_f_args = {
        let mut init = registerFunctionArgs {
            name: name,
            desc: desc,
            lua_f_ctx: lua_f_ctx,
            f_flags: flags,
        };
        init
    };
}
unsafe extern "C" fn luaRegisterFunctionArgsDispose(
    mut lua: *mut lua_State,
    mut register_f_args: *mut registerFunctionArgs,
) {
    sdsfree((*register_f_args).name);
    if !((*register_f_args).desc).is_null() {
        sdsfree((*register_f_args).desc);
    }
    luaL_unref(
        lua,
        -(10000 as libc::c_int),
        (*(*register_f_args).lua_f_ctx).lua_function_ref,
    );
    zfree((*register_f_args).lua_f_ctx as *mut libc::c_void);
}
unsafe extern "C" fn luaRegisterFunctionReadFlags(
    mut lua: *mut lua_State,
    mut flags: *mut uint64_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut j: libc::c_int = 1 as libc::c_int;
    let mut ret: libc::c_int = -(1 as libc::c_int);
    let mut f_flags: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh0 = j;
        j = j + 1;
        lua_pushnumber(lua, fresh0 as lua_Number);
        lua_gettable(lua, -(2 as libc::c_int));
        let mut t: libc::c_int = lua_type(lua, -(1 as libc::c_int));
        if t == 0 as libc::c_int {
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            current_block = 8831408221741692167;
            break;
        } else if lua_isstring(lua, -(1 as libc::c_int)) == 0 {
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            current_block = 10147241352347644208;
            break;
        } else {
            let mut flag_str: *const libc::c_char = lua_tolstring(
                lua,
                -(1 as libc::c_int),
                0 as *mut size_t,
            );
            let mut found: libc::c_int = 0 as libc::c_int;
            let mut flag: *mut scriptFlag = scripts_flags_def.as_mut_ptr();
            while !((*flag).str_0).is_null() {
                if strcasecmp((*flag).str_0, flag_str) == 0 {
                    f_flags = (f_flags as libc::c_ulong | (*flag).flag) as libc::c_int;
                    found = 1 as libc::c_int;
                    break;
                } else {
                    flag = flag.offset(1);
                }
            }
            lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            if found == 0 {
                current_block = 10147241352347644208;
                break;
            }
        }
    }
    match current_block {
        8831408221741692167 => {
            *flags = f_flags as uint64_t;
            ret = 0 as libc::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn luaRegisterFunctionReadNamedArgs(
    mut lua: *mut lua_State,
    mut register_f_args: *mut registerFunctionArgs,
) -> libc::c_int {
    let mut current_block: u64;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: sds = 0 as sds;
    let mut desc: sds = 0 as sds;
    let mut lua_f_ctx: *mut luaFunctionCtx = 0 as *mut luaFunctionCtx;
    let mut flags: uint64_t = 0 as libc::c_int as uint64_t;
    if !(lua_type(lua, 1 as libc::c_int) == 5 as libc::c_int) {
        err = b"calling redis.register_function with a single argument is only applicable to Lua table (representing named arguments).\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        lua_pushnil(lua);
        loop {
            if !(lua_next(lua, -(2 as libc::c_int)) != 0) {
                current_block = 14648156034262866959;
                break;
            }
            if lua_isstring(lua, -(2 as libc::c_int)) == 0 {
                err = b"named argument key given to redis.register_function is not a string\0"
                    as *const u8 as *const libc::c_char as *mut libc::c_char;
                current_block = 12190751697357944742;
                break;
            } else {
                let mut key: *const libc::c_char = lua_tolstring(
                    lua,
                    -(2 as libc::c_int),
                    0 as *mut size_t,
                );
                if strcasecmp(
                    key,
                    b"function_name\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    name = luaGetStringSds(lua, -(1 as libc::c_int));
                    if name.is_null() {
                        err = b"function_name argument given to redis.register_function must be a string\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char;
                        current_block = 12190751697357944742;
                        break;
                    }
                } else if strcasecmp(
                    key,
                    b"description\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    desc = luaGetStringSds(lua, -(1 as libc::c_int));
                    if desc.is_null() {
                        err = b"description argument given to redis.register_function must be a string\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char;
                        current_block = 12190751697357944742;
                        break;
                    }
                } else if strcasecmp(
                    key,
                    b"callback\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if !(lua_type(lua, -(1 as libc::c_int)) == 6 as libc::c_int) {
                        err = b"callback argument given to redis.register_function must be a function\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char;
                        current_block = 12190751697357944742;
                        break;
                    } else {
                        let mut lua_function_ref: libc::c_int = luaL_ref(
                            lua,
                            -(10000 as libc::c_int),
                        );
                        lua_f_ctx = zmalloc(
                            core::mem::size_of::<luaFunctionCtx>() as libc::c_ulong,
                        ) as *mut luaFunctionCtx;
                        (*lua_f_ctx).lua_function_ref = lua_function_ref;
                        continue;
                    }
                } else if strcasecmp(key, b"flags\0" as *const u8 as *const libc::c_char)
                    == 0
                {
                    if !(lua_type(lua, -(1 as libc::c_int)) == 5 as libc::c_int) {
                        err = b"flags argument to redis.register_function must be a table representing function flags\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char;
                        current_block = 12190751697357944742;
                        break;
                    } else if luaRegisterFunctionReadFlags(lua, &mut flags)
                        != 0 as libc::c_int
                    {
                        err = b"unknown flag given\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char;
                        current_block = 12190751697357944742;
                        break;
                    }
                } else {
                    err = b"unknown argument given to redis.register_function\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                    current_block = 12190751697357944742;
                    break;
                }
                lua_settop(lua, -(1 as libc::c_int) - 1 as libc::c_int);
            }
        }
        match current_block {
            12190751697357944742 => {}
            _ => {
                if name.is_null() {
                    err = b"redis.register_function must get a function name argument\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                } else if lua_f_ctx.is_null() {
                    err = b"redis.register_function must get a callback argument\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                } else {
                    luaRegisterFunctionArgsInitialize(
                        register_f_args,
                        name,
                        desc,
                        lua_f_ctx,
                        flags,
                    );
                    return 0 as libc::c_int;
                }
            }
        }
    }
    if !name.is_null() {
        sdsfree(name);
    }
    if !desc.is_null() {
        sdsfree(desc);
    }
    if !lua_f_ctx.is_null() {
        luaL_unref(lua, -(10000 as libc::c_int), (*lua_f_ctx).lua_function_ref);
        zfree(lua_f_ctx as *mut libc::c_void);
    }
    luaPushError(lua, err);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn luaRegisterFunctionReadPositionalArgs(
    mut lua: *mut lua_State,
    mut register_f_args: *mut registerFunctionArgs,
) -> libc::c_int {
    let mut lua_function_ref: libc::c_int = 0;
    let mut err: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut name: sds = 0 as sds;
    let mut desc: sds = 0 as sds;
    let mut lua_f_ctx: *mut luaFunctionCtx = 0 as *mut luaFunctionCtx;
    name = luaGetStringSds(lua, 1 as libc::c_int);
    if name.is_null() {
        err = b"first argument to redis.register_function must be a string\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if !(lua_type(lua, 2 as libc::c_int) == 6 as libc::c_int) {
        err = b"second argument to redis.register_function must be a function\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        lua_function_ref = luaL_ref(lua, -(10000 as libc::c_int));
        lua_f_ctx = zmalloc(core::mem::size_of::<luaFunctionCtx>() as libc::c_ulong)
            as *mut luaFunctionCtx;
        (*lua_f_ctx).lua_function_ref = lua_function_ref;
        luaRegisterFunctionArgsInitialize(
            register_f_args,
            name,
            0 as sds,
            lua_f_ctx,
            0 as libc::c_int as uint64_t,
        );
        return 0 as libc::c_int;
    }
    if !name.is_null() {
        sdsfree(name);
    }
    if !desc.is_null() {
        sdsfree(desc);
    }
    luaPushError(lua, err);
    return -(1 as libc::c_int);
}
unsafe extern "C" fn luaRegisterFunctionReadArgs(
    mut lua: *mut lua_State,
    mut register_f_args: *mut registerFunctionArgs,
) -> libc::c_int {
    let mut argc: libc::c_int = lua_gettop(lua);
    if argc < 1 as libc::c_int || argc > 2 as libc::c_int {
        luaPushError(
            lua,
            b"wrong number of arguments to redis.register_function\0" as *const u8
                as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if argc == 1 as libc::c_int {
        return luaRegisterFunctionReadNamedArgs(lua, register_f_args)
    } else {
        return luaRegisterFunctionReadPositionalArgs(lua, register_f_args)
    };
}
unsafe extern "C" fn luaRegisterFunction(mut lua: *mut lua_State) -> libc::c_int {
    let mut register_f_args: registerFunctionArgs = {
        let mut init = registerFunctionArgs {
            name: 0 as sds,
            desc: 0 as *mut libc::c_char,
            lua_f_ctx: 0 as *mut luaFunctionCtx,
            f_flags: 0,
        };
        init
    };
    let mut load_ctx: *mut loadCtx = luaGetFromRegistry(
        lua,
        b"__LIBRARY_CTX__\0" as *const u8 as *const libc::c_char,
    ) as *mut loadCtx;
    if load_ctx.is_null() {
        luaPushError(
            lua,
            b"redis.register_function can only be called on FUNCTION LOAD command\0"
                as *const u8 as *const libc::c_char,
        );
        return luaError(lua);
    }
    if luaRegisterFunctionReadArgs(lua, &mut register_f_args) != 0 as libc::c_int {
        return luaError(lua);
    }
    let mut err: sds = 0 as sds;
    if functionLibCreateFunction(
        register_f_args.name,
        register_f_args.lua_f_ctx as *mut libc::c_void,
        (*load_ctx).li,
        register_f_args.desc,
        register_f_args.f_flags,
        &mut err,
    ) != 0 as libc::c_int
    {
        luaRegisterFunctionArgsDispose(lua, &mut register_f_args);
        luaPushError(lua, err as *const libc::c_char);
        sdsfree(err);
        return luaError(lua);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn luaEngineInitEngine() -> libc::c_int {
    let mut lua_engine_ctx: *mut luaEngineCtx = zmalloc(
        core::mem::size_of::<luaEngineCtx>() as libc::c_ulong,
    ) as *mut luaEngineCtx;
    (*lua_engine_ctx).lua = luaL_newstate();
    luaRegisterRedisAPI((*lua_engine_ctx).lua);
    lua_createtable((*lua_engine_ctx).lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_createtable((*lua_engine_ctx).lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushstring(
        (*lua_engine_ctx).lua,
        b"register_function\0" as *const u8 as *const libc::c_char,
    );
    lua_pushcclosure(
        (*lua_engine_ctx).lua,
        Some(luaRegisterFunction as unsafe extern "C" fn(*mut lua_State) -> libc::c_int),
        0 as libc::c_int,
    );
    lua_settable((*lua_engine_ctx).lua, -(3 as libc::c_int));
    luaRegisterLogFunction((*lua_engine_ctx).lua);
    luaRegisterVersion((*lua_engine_ctx).lua);
    luaSetErrorMetatable((*lua_engine_ctx).lua);
    lua_setfield(
        (*lua_engine_ctx).lua,
        -(2 as libc::c_int),
        b"redis\0" as *const u8 as *const libc::c_char,
    );
    luaSetErrorMetatable((*lua_engine_ctx).lua);
    luaSetTableProtectionRecursively((*lua_engine_ctx).lua);
    lua_setfield(
        (*lua_engine_ctx).lua,
        -(10000 as libc::c_int),
        b"__LIBRARY_API__\0" as *const u8 as *const libc::c_char,
    );
    lua_pushstring(
        (*lua_engine_ctx).lua,
        b"__ERROR_HANDLER__\0" as *const u8 as *const libc::c_char,
    );
    let mut errh_func: *mut libc::c_char = b"local dbg = debug\ndebug = nil\nlocal error_handler = function (err)\n  local i = dbg.getinfo(2,'nSl')\n  if i and i.what == 'C' then\n    i = dbg.getinfo(3,'nSl')\n  end\n  if type(err) ~= 'table' then\n    err = {err='ERR ' .. tostring(err)}  end  if i then\n    err['source'] = i.source\n    err['line'] = i.currentline\n  end  return err\nend\nreturn error_handler\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char;
    luaL_loadbuffer(
        (*lua_engine_ctx).lua,
        errh_func,
        strlen(errh_func),
        b"@err_handler_def\0" as *const u8 as *const libc::c_char,
    );
    lua_pcall(
        (*lua_engine_ctx).lua,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
    lua_settable((*lua_engine_ctx).lua, -(10000 as libc::c_int));
    lua_pushvalue((*lua_engine_ctx).lua, -(10002 as libc::c_int));
    luaSetErrorMetatable((*lua_engine_ctx).lua);
    luaSetTableProtectionRecursively((*lua_engine_ctx).lua);
    lua_settop((*lua_engine_ctx).lua, -(1 as libc::c_int) - 1 as libc::c_int);
    lua_pushvalue((*lua_engine_ctx).lua, -(10002 as libc::c_int));
    lua_setfield(
        (*lua_engine_ctx).lua,
        -(10000 as libc::c_int),
        b"__GLOBALS_API__\0" as *const u8 as *const libc::c_char,
    );
    luaSaveOnRegistry(
        (*lua_engine_ctx).lua,
        b"__ENGINE_CTX__\0" as *const u8 as *const libc::c_char,
        lua_engine_ctx as *mut libc::c_void,
    );
    lua_createtable((*lua_engine_ctx).lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_createtable((*lua_engine_ctx).lua, 0 as libc::c_int, 0 as libc::c_int);
    lua_pushvalue((*lua_engine_ctx).lua, -(10002 as libc::c_int));
    lua_setfield(
        (*lua_engine_ctx).lua,
        -(2 as libc::c_int),
        b"__index\0" as *const u8 as *const libc::c_char,
    );
    lua_enablereadonlytable(
        (*lua_engine_ctx).lua,
        -(1 as libc::c_int),
        1 as libc::c_int,
    );
    lua_setmetatable((*lua_engine_ctx).lua, -(2 as libc::c_int));
    lua_enablereadonlytable(
        (*lua_engine_ctx).lua,
        -(1 as libc::c_int),
        1 as libc::c_int,
    );
    lua_replace((*lua_engine_ctx).lua, -(10002 as libc::c_int));
    let mut lua_engine: *mut engine = zmalloc(
        core::mem::size_of::<engine>() as libc::c_ulong,
    ) as *mut engine;
    *lua_engine = {
        let mut init = engine {
            engine_ctx: lua_engine_ctx as *mut libc::c_void,
            create: Some(
                luaEngineCreate
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut functionLibInfo,
                        sds,
                        *mut sds,
                    ) -> libc::c_int,
            ),
            call: Some(
                luaEngineCall
                    as unsafe extern "C" fn(
                        *mut scriptRunCtx,
                        *mut libc::c_void,
                        *mut libc::c_void,
                        *mut *mut robj,
                        size_t,
                        *mut *mut robj,
                        size_t,
                    ) -> (),
            ),
            get_used_memory: Some(
                luaEngineGetUsedMemoy
                    as unsafe extern "C" fn(*mut libc::c_void) -> size_t,
            ),
            get_function_memory_overhead: Some(
                luaEngineFunctionMemoryOverhead
                    as unsafe extern "C" fn(*mut libc::c_void) -> size_t,
            ),
            get_engine_memory_overhead: Some(
                luaEngineMemoryOverhead
                    as unsafe extern "C" fn(*mut libc::c_void) -> size_t,
            ),
            free_function: Some(
                luaEngineFreeFunction
                    as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
            ),
        };
        init
    };
    return functionsRegisterEngine(
        b"LUA\0" as *const u8 as *const libc::c_char,
        lua_engine,
    );
}

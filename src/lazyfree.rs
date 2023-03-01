extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type clusterSlotToKeyMapping;
    pub type functionsLibCtx;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn listRelease(list: *mut list);
    fn dictRelease(d: *mut dict);
    fn raxFree(rax: *mut rax);
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
    static mut dbDictType: dictType;
    static mut dbExpiresDictType: dictType;
    fn moduleGetFreeEffort(key: *mut robj, val: *mut robj, dbid: libc::c_int) -> size_t;
    fn freeTrackingRadixTree(rt: *mut rax);
    fn decrRefCount(o: *mut robj);
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn bioCreateLazyFreeJob(
        free_fn: Option::<lazy_free_fn>,
        arg_count: libc::c_int,
        _: ...
    );
    fn functionsLibCtxfunctionsLen(functions_ctx: *mut functionsLibCtx) -> size_t;
    fn functionsLibCtxFree(lib_ctx: *mut functionsLibCtx);
}
pub type __int16_t = libc::c_short;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sds = *mut libc::c_char;
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
pub type robj = redisObject;
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
pub type lazy_free_fn = unsafe extern "C" fn(*mut *mut libc::c_void) -> ();
static mut lazyfree_objects: size_t = 0 as libc::c_int as libc::c_ulong;
static mut lazyfreed_objects: size_t = 0 as libc::c_int as libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn lazyfreeFreeObject(mut args: *mut *mut libc::c_void) {
    let mut o: *mut robj = *args.offset(0 as libc::c_int as isize) as *mut robj;
    decrRefCount(o);
    let fresh0 = &mut lazyfree_objects;
    let fresh1 = 1 as libc::c_int as size_t;
    core::intrinsics::atomic_xsub_relaxed(fresh0, fresh1) - fresh1;
    let fresh2 = &mut lazyfreed_objects;
    let fresh3 = 1 as libc::c_int as size_t;
    core::intrinsics::atomic_xadd_relaxed(fresh2, fresh3) + fresh3;
}
#[no_mangle]
pub unsafe extern "C" fn lazyfreeFreeDatabase(mut args: *mut *mut libc::c_void) {
    let mut ht1: *mut dict = *args.offset(0 as libc::c_int as isize) as *mut dict;
    let mut ht2: *mut dict = *args.offset(1 as libc::c_int as isize) as *mut dict;
    let mut numkeys: size_t = ((*ht1).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*ht1).ht_used[1 as libc::c_int as usize]);
    dictRelease(ht1);
    dictRelease(ht2);
    let fresh4 = &mut lazyfree_objects;
    let fresh5 = numkeys;
    core::intrinsics::atomic_xsub_relaxed(fresh4, fresh5) - fresh5;
    let fresh6 = &mut lazyfreed_objects;
    let fresh7 = numkeys;
    core::intrinsics::atomic_xadd_relaxed(fresh6, fresh7) + fresh7;
}
#[no_mangle]
pub unsafe extern "C" fn lazyFreeTrackingTable(mut args: *mut *mut libc::c_void) {
    let mut rt: *mut rax = *args.offset(0 as libc::c_int as isize) as *mut rax;
    let mut len: size_t = (*rt).numele;
    freeTrackingRadixTree(rt);
    let fresh8 = &mut lazyfree_objects;
    let fresh9 = len;
    core::intrinsics::atomic_xsub_relaxed(fresh8, fresh9) - fresh9;
    let fresh10 = &mut lazyfreed_objects;
    let fresh11 = len;
    core::intrinsics::atomic_xadd_relaxed(fresh10, fresh11) + fresh11;
}
#[no_mangle]
pub unsafe extern "C" fn lazyFreeLuaScripts(mut args: *mut *mut libc::c_void) {
    let mut lua_scripts: *mut dict = *args.offset(0 as libc::c_int as isize)
        as *mut dict;
    let mut len: libc::c_longlong = ((*lua_scripts).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*lua_scripts).ht_used[1 as libc::c_int as usize])
        as libc::c_longlong;
    dictRelease(lua_scripts);
    let fresh12 = &mut lazyfree_objects;
    let fresh13 = len as size_t;
    core::intrinsics::atomic_xsub_relaxed(fresh12, fresh13) - fresh13;
    let fresh14 = &mut lazyfreed_objects;
    let fresh15 = len as size_t;
    core::intrinsics::atomic_xadd_relaxed(fresh14, fresh15) + fresh15;
}
#[no_mangle]
pub unsafe extern "C" fn lazyFreeFunctionsCtx(mut args: *mut *mut libc::c_void) {
    let mut functions_lib_ctx: *mut functionsLibCtx = *args
        .offset(0 as libc::c_int as isize) as *mut functionsLibCtx;
    let mut len: size_t = functionsLibCtxfunctionsLen(functions_lib_ctx);
    functionsLibCtxFree(functions_lib_ctx);
    let fresh16 = &mut lazyfree_objects;
    let fresh17 = len;
    core::intrinsics::atomic_xsub_relaxed(fresh16, fresh17) - fresh17;
    let fresh18 = &mut lazyfreed_objects;
    let fresh19 = len;
    core::intrinsics::atomic_xadd_relaxed(fresh18, fresh19) + fresh19;
}
#[no_mangle]
pub unsafe extern "C" fn lazyFreeReplicationBacklogRefMem(
    mut args: *mut *mut libc::c_void,
) {
    let mut blocks: *mut list = *args.offset(0 as libc::c_int as isize) as *mut list;
    let mut index: *mut rax = *args.offset(1 as libc::c_int as isize) as *mut rax;
    let mut len: libc::c_longlong = (*blocks).len as libc::c_longlong;
    len = (len as libc::c_ulonglong).wrapping_add(raxSize(index) as libc::c_ulonglong)
        as libc::c_longlong as libc::c_longlong;
    listRelease(blocks);
    raxFree(index);
    let fresh20 = &mut lazyfree_objects;
    let fresh21 = len as size_t;
    core::intrinsics::atomic_xsub_relaxed(fresh20, fresh21) - fresh21;
    let fresh22 = &mut lazyfreed_objects;
    let fresh23 = len as size_t;
    core::intrinsics::atomic_xadd_relaxed(fresh22, fresh23) + fresh23;
}
#[no_mangle]
pub unsafe extern "C" fn lazyfreeGetPendingObjectsCount() -> size_t {
    let mut aux: size_t = 0;
    aux = core::intrinsics::atomic_load_relaxed(&mut lazyfree_objects);
    return aux;
}
#[no_mangle]
pub unsafe extern "C" fn lazyfreeGetFreedObjectsCount() -> size_t {
    let mut aux: size_t = 0;
    aux = core::intrinsics::atomic_load_relaxed(&mut lazyfreed_objects);
    return aux;
}
#[no_mangle]
pub unsafe extern "C" fn lazyfreeResetStats() {
    core::intrinsics::atomic_store_relaxed(
        &mut lazyfreed_objects,
        0 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lazyfreeGetFreeEffort(
    mut key: *mut robj,
    mut obj: *mut robj,
    mut dbid: libc::c_int,
) -> size_t {
    if (*obj).type_0() as libc::c_int == 1 as libc::c_int {
        let mut ql: *mut quicklist = (*obj).ptr as *mut quicklist;
        return (*ql).len;
    } else if (*obj).type_0() as libc::c_int == 2 as libc::c_int
        && (*obj).encoding() as libc::c_int == 2 as libc::c_int
    {
        let mut ht: *mut dict = (*obj).ptr as *mut dict;
        return ((*ht).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*ht).ht_used[1 as libc::c_int as usize]);
    } else if (*obj).type_0() as libc::c_int == 3 as libc::c_int
        && (*obj).encoding() as libc::c_int == 7 as libc::c_int
    {
        let mut zs: *mut zset = (*obj).ptr as *mut zset;
        return (*(*zs).zsl).length;
    } else if (*obj).type_0() as libc::c_int == 4 as libc::c_int
        && (*obj).encoding() as libc::c_int == 2 as libc::c_int
    {
        let mut ht_0: *mut dict = (*obj).ptr as *mut dict;
        return ((*ht_0).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*ht_0).ht_used[1 as libc::c_int as usize]);
    } else if (*obj).type_0() as libc::c_int == 6 as libc::c_int {
        let mut effort: size_t = 0 as libc::c_int as size_t;
        let mut s: *mut stream = (*obj).ptr as *mut stream;
        effort = (effort as libc::c_ulong).wrapping_add((*(*s).rax).numnodes) as size_t
            as size_t;
        if !((*s).cgroups).is_null() && raxSize((*s).cgroups) != 0 {
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
            let mut cg: *mut streamCG = 0 as *mut streamCG;
            raxStart(&mut ri, (*s).cgroups);
            raxSeek(
                &mut ri,
                b"^\0" as *const u8 as *const libc::c_char,
                0 as *mut libc::c_uchar,
                0 as libc::c_int as size_t,
            );
            if raxNext(&mut ri) != 0 {} else {
                _serverAssert(
                    b"raxNext(&ri)\0" as *const u8 as *const libc::c_char,
                    b"lazyfree.c\0" as *const u8 as *const libc::c_char,
                    135 as libc::c_int,
                );
                unreachable!();
            };
            cg = ri.data as *mut streamCG;
            effort = (effort as libc::c_ulong)
                .wrapping_add(
                    (raxSize((*s).cgroups))
                        .wrapping_mul(
                            (1 as libc::c_int as libc::c_ulong)
                                .wrapping_add(raxSize((*cg).pel)),
                        ),
                ) as size_t as size_t;
            raxStop(&mut ri);
        }
        return effort;
    } else if (*obj).type_0() as libc::c_int == 5 as libc::c_int {
        let mut effort_0: size_t = moduleGetFreeEffort(key, obj, dbid);
        return if effort_0 == 0 as libc::c_int as libc::c_ulong {
            (9223372036854775807 as libc::c_long as libc::c_ulong)
                .wrapping_mul(2 as libc::c_ulong)
                .wrapping_add(1 as libc::c_ulong)
        } else {
            effort_0
        };
    } else {
        return 1 as libc::c_int as size_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeObjAsync(
    mut key: *mut robj,
    mut obj: *mut robj,
    mut dbid: libc::c_int,
) {
    let mut free_effort: size_t = lazyfreeGetFreeEffort(key, obj, dbid);
    if free_effort > 64 as libc::c_int as libc::c_ulong
        && (*obj).refcount == 1 as libc::c_int
    {
        let fresh24 = &mut lazyfree_objects;
        let fresh25 = 1 as libc::c_int as size_t;
        core::intrinsics::atomic_xadd_relaxed(fresh24, fresh25) + fresh25;
        bioCreateLazyFreeJob(
            Some(
                lazyfreeFreeObject as unsafe extern "C" fn(*mut *mut libc::c_void) -> (),
            ),
            1 as libc::c_int,
            obj,
        );
    } else {
        decrRefCount(obj);
    };
}
#[no_mangle]
pub unsafe extern "C" fn emptyDbAsync(mut db: *mut redisDb) {
    let mut oldht1: *mut dict = (*db).dict;
    let mut oldht2: *mut dict = (*db).expires;
    (*db).dict = dictCreate(&mut dbDictType);
    (*db).expires = dictCreate(&mut dbExpiresDictType);
    let fresh26 = &mut lazyfree_objects;
    let fresh27 = ((*oldht1).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*oldht1).ht_used[1 as libc::c_int as usize]);
    core::intrinsics::atomic_xadd_relaxed(fresh26, fresh27) + fresh27;
    bioCreateLazyFreeJob(
        Some(lazyfreeFreeDatabase as unsafe extern "C" fn(*mut *mut libc::c_void) -> ()),
        2 as libc::c_int,
        oldht1,
        oldht2,
    );
}
#[no_mangle]
pub unsafe extern "C" fn freeTrackingRadixTreeAsync(mut tracking: *mut rax) {
    if (*tracking).numnodes > 64 as libc::c_int as libc::c_ulong {
        let fresh28 = &mut lazyfree_objects;
        let fresh29 = (*tracking).numele;
        core::intrinsics::atomic_xadd_relaxed(fresh28, fresh29) + fresh29;
        bioCreateLazyFreeJob(
            Some(
                lazyFreeTrackingTable
                    as unsafe extern "C" fn(*mut *mut libc::c_void) -> (),
            ),
            1 as libc::c_int,
            tracking,
        );
    } else {
        freeTrackingRadixTree(tracking);
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeLuaScriptsAsync(mut lua_scripts: *mut dict) {
    if ((*lua_scripts).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*lua_scripts).ht_used[1 as libc::c_int as usize])
        > 64 as libc::c_int as libc::c_ulong
    {
        let fresh30 = &mut lazyfree_objects;
        let fresh31 = ((*lua_scripts).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*lua_scripts).ht_used[1 as libc::c_int as usize]);
        core::intrinsics::atomic_xadd_relaxed(fresh30, fresh31) + fresh31;
        bioCreateLazyFreeJob(
            Some(
                lazyFreeLuaScripts as unsafe extern "C" fn(*mut *mut libc::c_void) -> (),
            ),
            1 as libc::c_int,
            lua_scripts,
        );
    } else {
        dictRelease(lua_scripts);
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeFunctionsAsync(
    mut functions_lib_ctx: *mut functionsLibCtx,
) {
    if functionsLibCtxfunctionsLen(functions_lib_ctx)
        > 64 as libc::c_int as libc::c_ulong
    {
        let fresh32 = &mut lazyfree_objects;
        let fresh33 = functionsLibCtxfunctionsLen(functions_lib_ctx);
        core::intrinsics::atomic_xadd_relaxed(fresh32, fresh33) + fresh33;
        bioCreateLazyFreeJob(
            Some(
                lazyFreeFunctionsCtx
                    as unsafe extern "C" fn(*mut *mut libc::c_void) -> (),
            ),
            1 as libc::c_int,
            functions_lib_ctx,
        );
    } else {
        functionsLibCtxFree(functions_lib_ctx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn freeReplicationBacklogRefMemAsync(
    mut blocks: *mut list,
    mut index: *mut rax,
) {
    if (*blocks).len > 64 as libc::c_int as libc::c_ulong
        || raxSize(index) > 64 as libc::c_int as libc::c_ulong
    {
        let fresh34 = &mut lazyfree_objects;
        let fresh35 = ((*blocks).len).wrapping_add(raxSize(index));
        core::intrinsics::atomic_xadd_relaxed(fresh34, fresh35) + fresh35;
        bioCreateLazyFreeJob(
            Some(
                lazyFreeReplicationBacklogRefMem
                    as unsafe extern "C" fn(*mut *mut libc::c_void) -> (),
            ),
            2 as libc::c_int,
            blocks,
            index,
        );
    } else {
        listRelease(blocks);
        raxFree(index);
    };
}

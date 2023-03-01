extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn lpNew(capacity: size_t) -> *mut libc::c_uchar;
    fn lpInsertString(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
        p: *mut libc::c_uchar,
        where_0: libc::c_int,
        newp: *mut *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpPrepend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpAppend(
        lp: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpReplace(
        lp: *mut libc::c_uchar,
        p: *mut *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> *mut libc::c_uchar;
    fn lpDelete(
        lp: *mut libc::c_uchar,
        p: *mut libc::c_uchar,
        newp: *mut *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpDeleteRange(
        lp: *mut libc::c_uchar,
        index: libc::c_long,
        num: libc::c_ulong,
    ) -> *mut libc::c_uchar;
    fn lpMerge(
        first: *mut *mut libc::c_uchar,
        second: *mut *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpLength(lp: *mut libc::c_uchar) -> libc::c_ulong;
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpPrev(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpBytes(lp: *mut libc::c_uchar) -> size_t;
    fn lpSeek(lp: *mut libc::c_uchar, index: libc::c_long) -> *mut libc::c_uchar;
    fn lpCompare(
        p: *mut libc::c_uchar,
        s: *mut libc::c_uchar,
        slen: uint32_t,
    ) -> libc::c_uint;
    fn lpRepr(lp: *mut libc::c_uchar);
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
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
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone, c2rust_bitfields:: BitfieldStruct)]
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
pub struct quicklistLZF {
    pub sz: size_t,
    pub compressed: [libc::c_char; 0],
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
static mut optimization_level: [size_t; 5] = [
    4096 as libc::c_int as size_t,
    8192 as libc::c_int as size_t,
    16384 as libc::c_int as size_t,
    32768 as libc::c_int as size_t,
    65536 as libc::c_int as size_t,
];
static mut packed_threshold: size_t = ((1 as libc::c_int) << 30 as libc::c_int)
    as size_t;
#[no_mangle]
pub unsafe extern "C" fn quicklistisSetPackedThreshold(mut sz: size_t) -> libc::c_int {
    if sz as libc::c_ulonglong
        > ((1 as libc::c_ulonglong) << 32 as libc::c_int)
            .wrapping_sub(((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulonglong)
    {
        return 0 as libc::c_int
    } else {
        if sz == 0 as libc::c_int as libc::c_ulong {
            sz = ((1 as libc::c_int) << 30 as libc::c_int) as size_t;
        }
    }
    packed_threshold = sz;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistCreate() -> *mut quicklist {
    let mut quicklist: *mut quicklist = 0 as *mut quicklist;
    quicklist = zmalloc(core::mem::size_of::<quicklist>() as libc::c_ulong)
        as *mut quicklist;
    (*quicklist).tail = 0 as *mut quicklistNode;
    (*quicklist).head = (*quicklist).tail;
    (*quicklist).len = 0 as libc::c_int as libc::c_ulong;
    (*quicklist).count = 0 as libc::c_int as libc::c_ulong;
    (*quicklist).set_compress(0 as libc::c_int as libc::c_uint);
    (*quicklist).set_fill(-(2 as libc::c_int));
    (*quicklist).set_bookmark_count(0 as libc::c_int as libc::c_uint);
    return quicklist;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistSetCompressDepth(
    mut quicklist: *mut quicklist,
    mut compress: libc::c_int,
) {
    if compress > ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int {
        compress = ((1 as libc::c_int) << 16 as libc::c_int) - 1 as libc::c_int;
    } else if compress < 0 as libc::c_int {
        compress = 0 as libc::c_int;
    }
    (*quicklist).set_compress(compress as libc::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistSetFill(
    mut quicklist: *mut quicklist,
    mut fill: libc::c_int,
) {
    if fill
        > ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int
    {
        fill = ((1 as libc::c_int) << 16 as libc::c_int - 1 as libc::c_int)
            - 1 as libc::c_int;
    } else if fill < -(5 as libc::c_int) {
        fill = -(5 as libc::c_int);
    }
    (*quicklist).set_fill(fill);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistSetOptions(
    mut quicklist: *mut quicklist,
    mut fill: libc::c_int,
    mut depth: libc::c_int,
) {
    quicklistSetFill(quicklist, fill);
    quicklistSetCompressDepth(quicklist, depth);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistNew(
    mut fill: libc::c_int,
    mut compress: libc::c_int,
) -> *mut quicklist {
    let mut quicklist: *mut quicklist = quicklistCreate();
    quicklistSetOptions(quicklist, fill, compress);
    return quicklist;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistCreateNode() -> *mut quicklistNode {
    let mut node: *mut quicklistNode = 0 as *mut quicklistNode;
    node = zmalloc(core::mem::size_of::<quicklistNode>() as libc::c_ulong)
        as *mut quicklistNode;
    (*node).entry = 0 as *mut libc::c_uchar;
    (*node).set_count(0 as libc::c_int as libc::c_uint);
    (*node).sz = 0 as libc::c_int as size_t;
    (*node).prev = 0 as *mut quicklistNode;
    (*node).next = (*node).prev;
    (*node).set_encoding(1 as libc::c_int as libc::c_uint);
    (*node).set_container(2 as libc::c_int as libc::c_uint);
    (*node).set_recompress(0 as libc::c_int as libc::c_uint);
    (*node).set_dont_compress(0 as libc::c_int as libc::c_uint);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistCount(mut ql: *const quicklist) -> libc::c_ulong {
    return (*ql).count;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistRelease(mut quicklist: *mut quicklist) {
    let mut len: libc::c_ulong = 0;
    let mut current: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut next: *mut quicklistNode = 0 as *mut quicklistNode;
    current = (*quicklist).head;
    len = (*quicklist).len;
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        next = (*current).next;
        zfree((*current).entry as *mut libc::c_void);
        (*quicklist)
            .count = ((*quicklist).count)
            .wrapping_sub((*current).count() as libc::c_ulong);
        zfree(current as *mut libc::c_void);
        (*quicklist).len = ((*quicklist).len).wrapping_sub(1);
        current = next;
    }
    quicklistBookmarksClear(quicklist);
    zfree(quicklist as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn __quicklistCompressNode(
    mut node: *mut quicklistNode,
) -> libc::c_int {
    if (*node).dont_compress() != 0 {
        return 0 as libc::c_int;
    }
    if (!((*node).prev).is_null() && !((*node).next).is_null()) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"node->prev && node->next\0" as *const u8 as *const libc::c_char,
            b"quicklist.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
        );
        unreachable!();
    };
    (*node).set_recompress(0 as libc::c_int as libc::c_uint);
    if (*node).sz < 48 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut lzf: *mut quicklistLZF = zmalloc(
        (core::mem::size_of::<quicklistLZF>() as libc::c_ulong)
            .wrapping_add((*node).sz),
    ) as *mut quicklistLZF;
    (*lzf)
        .sz = lzf_compress(
        (*node).entry as *const libc::c_void,
        (*node).sz,
        ((*lzf).compressed).as_mut_ptr() as *mut libc::c_void,
        (*node).sz,
    );
    if (*lzf).sz == 0 as libc::c_int as libc::c_ulong
        || ((*lzf).sz).wrapping_add(8 as libc::c_int as libc::c_ulong) >= (*node).sz
    {
        zfree(lzf as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    lzf = zrealloc(
        lzf as *mut libc::c_void,
        (core::mem::size_of::<quicklistLZF>() as libc::c_ulong).wrapping_add((*lzf).sz),
    ) as *mut quicklistLZF;
    zfree((*node).entry as *mut libc::c_void);
    (*node).entry = lzf as *mut libc::c_uchar;
    (*node).set_encoding(2 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __quicklistDecompressNode(
    mut node: *mut quicklistNode,
) -> libc::c_int {
    (*node).set_recompress(0 as libc::c_int as libc::c_uint);
    let mut decompressed: *mut libc::c_void = zmalloc((*node).sz);
    let mut lzf: *mut quicklistLZF = (*node).entry as *mut quicklistLZF;
    if lzf_decompress(
        ((*lzf).compressed).as_mut_ptr() as *const libc::c_void,
        (*lzf).sz,
        decompressed,
        (*node).sz,
    ) == 0 as libc::c_int as libc::c_ulong
    {
        zfree(decompressed);
        return 0 as libc::c_int;
    }
    zfree(lzf as *mut libc::c_void);
    (*node).entry = decompressed as *mut libc::c_uchar;
    (*node).set_encoding(1 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistGetLzf(
    mut node: *const quicklistNode,
    mut data: *mut *mut libc::c_void,
) -> size_t {
    let mut lzf: *mut quicklistLZF = (*node).entry as *mut quicklistLZF;
    *data = ((*lzf).compressed).as_mut_ptr() as *mut libc::c_void;
    return (*lzf).sz;
}
#[no_mangle]
pub unsafe extern "C" fn __quicklistCompress(
    mut quicklist: *const quicklist,
    mut node: *mut quicklistNode,
) {
    if (*quicklist).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if ((*(*quicklist).head).recompress() as libc::c_int == 0 as libc::c_int
        && (*(*quicklist).tail).recompress() as libc::c_int == 0 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {} else {
        _serverAssert(
            b"quicklist->head->recompress == 0 && quicklist->tail->recompress == 0\0"
                as *const u8 as *const libc::c_char,
            b"quicklist.c\0" as *const u8 as *const libc::c_char,
            312 as libc::c_int,
        );
        unreachable!();
    };
    if !((*quicklist).compress() as libc::c_int != 0 as libc::c_int)
        || (*quicklist).len
            < ((*quicklist).compress() as libc::c_int * 2 as libc::c_int) as libc::c_uint
                as libc::c_ulong
    {
        return;
    }
    let mut forward: *mut quicklistNode = (*quicklist).head;
    let mut reverse: *mut quicklistNode = (*quicklist).tail;
    let mut depth: libc::c_int = 0 as libc::c_int;
    let mut in_depth: libc::c_int = 0 as libc::c_int;
    loop {
        let fresh1 = depth;
        depth = depth + 1;
        if !(fresh1 < (*quicklist).compress() as libc::c_int) {
            break;
        }
        if !forward.is_null() && (*forward).encoding() as libc::c_int == 2 as libc::c_int
        {
            __quicklistDecompressNode(forward);
        }
        if !reverse.is_null() && (*reverse).encoding() as libc::c_int == 2 as libc::c_int
        {
            __quicklistDecompressNode(reverse);
        }
        if forward == node || reverse == node {
            in_depth = 1 as libc::c_int;
        }
        if forward == reverse || (*forward).next == reverse {
            return;
        }
        forward = (*forward).next;
        reverse = (*reverse).prev;
    }
    if in_depth == 0 {
        if !node.is_null() && (*node).encoding() as libc::c_int == 1 as libc::c_int {
            __quicklistCompressNode(node);
        }
    }
    if !forward.is_null() && (*forward).encoding() as libc::c_int == 1 as libc::c_int {
        __quicklistCompressNode(forward);
    }
    if !reverse.is_null() && (*reverse).encoding() as libc::c_int == 1 as libc::c_int {
        __quicklistCompressNode(reverse);
    }
}
#[no_mangle]
pub unsafe extern "C" fn __quicklistInsertNode(
    mut quicklist: *mut quicklist,
    mut old_node: *mut quicklistNode,
    mut new_node: *mut quicklistNode,
    mut after: libc::c_int,
) {
    if after != 0 {
        (*new_node).prev = old_node;
        if !old_node.is_null() {
            (*new_node).next = (*old_node).next;
            if !((*old_node).next).is_null() {
                (*(*old_node).next).prev = new_node;
            }
            (*old_node).next = new_node;
        }
        if (*quicklist).tail == old_node {
            (*quicklist).tail = new_node;
        }
    } else {
        (*new_node).next = old_node;
        if !old_node.is_null() {
            (*new_node).prev = (*old_node).prev;
            if !((*old_node).prev).is_null() {
                (*(*old_node).prev).next = new_node;
            }
            (*old_node).prev = new_node;
        }
        if (*quicklist).head == old_node {
            (*quicklist).head = new_node;
        }
    }
    if (*quicklist).len == 0 as libc::c_int as libc::c_ulong {
        (*quicklist).tail = new_node;
        (*quicklist).head = (*quicklist).tail;
    }
    (*quicklist).len = ((*quicklist).len).wrapping_add(1);
    if !old_node.is_null() {
        if (*old_node).recompress() != 0 {
            if !old_node.is_null()
                && (*old_node).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode(old_node);
            }
        } else {
            __quicklistCompress(quicklist, old_node);
        }
    }
    if (*new_node).recompress() != 0 {
        if !new_node.is_null()
            && (*new_node).encoding() as libc::c_int == 1 as libc::c_int
        {
            __quicklistCompressNode(new_node);
        }
    } else {
        __quicklistCompress(quicklist, new_node);
    };
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistInsertNodeBefore(
    mut quicklist: *mut quicklist,
    mut old_node: *mut quicklistNode,
    mut new_node: *mut quicklistNode,
) {
    __quicklistInsertNode(quicklist, old_node, new_node, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistInsertNodeAfter(
    mut quicklist: *mut quicklist,
    mut old_node: *mut quicklistNode,
    mut new_node: *mut quicklistNode,
) {
    __quicklistInsertNode(quicklist, old_node, new_node, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistNodeSizeMeetsOptimizationRequirement(
    sz: size_t,
    fill: libc::c_int,
) -> libc::c_int {
    if fill >= 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut offset: size_t = (-fill - 1 as libc::c_int) as size_t;
    if offset
        < (core::mem::size_of::<[size_t; 5]>() as libc::c_ulong)
            .wrapping_div(core::mem::size_of::<size_t>() as libc::c_ulong)
    {
        if sz <= optimization_level[offset as usize] {
            return 1 as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistNodeAllowInsert(
    mut node: *const quicklistNode,
    fill: libc::c_int,
    sz: size_t,
) -> libc::c_int {
    if node.is_null() as libc::c_int as libc::c_long != 0 {
        return 0 as libc::c_int;
    }
    if ((*node).container() as libc::c_int == 1 as libc::c_int || sz >= packed_threshold)
        as libc::c_int as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    let mut new_sz: size_t = ((*node).sz)
        .wrapping_add(sz)
        .wrapping_add(8 as libc::c_int as libc::c_ulong);
    if (_quicklistNodeSizeMeetsOptimizationRequirement(new_sz, fill) != 0) as libc::c_int
        as libc::c_long != 0
    {
        return 1 as libc::c_int
    } else if !(new_sz <= 8192 as libc::c_int as libc::c_ulong) {
        return 0 as libc::c_int
    } else if ((*node).count() as libc::c_int) < fill {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistNodeAllowMerge(
    mut a: *const quicklistNode,
    mut b: *const quicklistNode,
    fill: libc::c_int,
) -> libc::c_int {
    if a.is_null() || b.is_null() {
        return 0 as libc::c_int;
    }
    if ((*a).container() as libc::c_int == 1 as libc::c_int
        || (*b).container() as libc::c_int == 1 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        return 0 as libc::c_int;
    }
    let mut merge_sz: libc::c_uint = ((*a).sz)
        .wrapping_add((*b).sz)
        .wrapping_sub(11 as libc::c_int as libc::c_ulong) as libc::c_uint;
    if (_quicklistNodeSizeMeetsOptimizationRequirement(merge_sz as size_t, fill) != 0)
        as libc::c_int as libc::c_long != 0
    {
        return 1 as libc::c_int
    } else if !(merge_sz <= 8192 as libc::c_int as libc::c_uint) {
        return 0 as libc::c_int
    } else if (*a).count() as libc::c_int + (*b).count() as libc::c_int <= fill {
        return 1 as libc::c_int
    } else {
        return 0 as libc::c_int
    };
}
unsafe extern "C" fn __quicklistCreatePlainNode(
    mut value: *mut libc::c_void,
    mut sz: size_t,
) -> *mut quicklistNode {
    let mut new_node: *mut quicklistNode = quicklistCreateNode();
    (*new_node).entry = zmalloc(sz) as *mut libc::c_uchar;
    (*new_node).set_container(1 as libc::c_int as libc::c_uint);
    memcpy((*new_node).entry as *mut libc::c_void, value, sz);
    (*new_node).sz = sz;
    (*new_node).set_count((*new_node).count() + 1);
    return new_node;
}
unsafe extern "C" fn __quicklistInsertPlainNode(
    mut quicklist: *mut quicklist,
    mut old_node: *mut quicklistNode,
    mut value: *mut libc::c_void,
    mut sz: size_t,
    mut after: libc::c_int,
) {
    __quicklistInsertNode(
        quicklist,
        old_node,
        __quicklistCreatePlainNode(value, sz),
        after,
    );
    (*quicklist).count = ((*quicklist).count).wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistPushHead(
    mut quicklist: *mut quicklist,
    mut value: *mut libc::c_void,
    mut sz: size_t,
) -> libc::c_int {
    let mut orig_head: *mut quicklistNode = (*quicklist).head;
    if (sz >= packed_threshold) as libc::c_int as libc::c_long != 0 {
        __quicklistInsertPlainNode(
            quicklist,
            (*quicklist).head,
            value,
            sz,
            0 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    if (_quicklistNodeAllowInsert((*quicklist).head, (*quicklist).fill(), sz) != 0)
        as libc::c_int as libc::c_long != 0
    {
        (*(*quicklist).head)
            .entry = lpPrepend(
            (*(*quicklist).head).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*(*quicklist).head).sz = lpBytes((*(*quicklist).head).entry);
    } else {
        let mut node: *mut quicklistNode = quicklistCreateNode();
        (*node)
            .entry = lpPrepend(
            lpNew(0 as libc::c_int as size_t),
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*node).sz = lpBytes((*node).entry);
        _quicklistInsertNodeBefore(quicklist, (*quicklist).head, node);
    }
    (*quicklist).count = ((*quicklist).count).wrapping_add(1);
    (*(*quicklist).head).set_count((*(*quicklist).head).count() + 1);
    return (orig_head != (*quicklist).head) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistPushTail(
    mut quicklist: *mut quicklist,
    mut value: *mut libc::c_void,
    mut sz: size_t,
) -> libc::c_int {
    let mut orig_tail: *mut quicklistNode = (*quicklist).tail;
    if (sz >= packed_threshold) as libc::c_int as libc::c_long != 0 {
        __quicklistInsertPlainNode(
            quicklist,
            (*quicklist).tail,
            value,
            sz,
            1 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    if (_quicklistNodeAllowInsert((*quicklist).tail, (*quicklist).fill(), sz) != 0)
        as libc::c_int as libc::c_long != 0
    {
        (*(*quicklist).tail)
            .entry = lpAppend(
            (*(*quicklist).tail).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*(*quicklist).tail).sz = lpBytes((*(*quicklist).tail).entry);
    } else {
        let mut node: *mut quicklistNode = quicklistCreateNode();
        (*node)
            .entry = lpAppend(
            lpNew(0 as libc::c_int as size_t),
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*node).sz = lpBytes((*node).entry);
        _quicklistInsertNodeAfter(quicklist, (*quicklist).tail, node);
    }
    (*quicklist).count = ((*quicklist).count).wrapping_add(1);
    (*(*quicklist).tail).set_count((*(*quicklist).tail).count() + 1);
    return (orig_tail != (*quicklist).tail) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistAppendListpack(
    mut quicklist: *mut quicklist,
    mut zl: *mut libc::c_uchar,
) {
    let mut node: *mut quicklistNode = quicklistCreateNode();
    (*node).entry = zl;
    (*node).set_count(lpLength((*node).entry) as libc::c_uint);
    (*node).sz = lpBytes(zl);
    _quicklistInsertNodeAfter(quicklist, (*quicklist).tail, node);
    (*quicklist)
        .count = ((*quicklist).count).wrapping_add((*node).count() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistAppendPlainNode(
    mut quicklist: *mut quicklist,
    mut data: *mut libc::c_uchar,
    mut sz: size_t,
) {
    let mut node: *mut quicklistNode = quicklistCreateNode();
    (*node).entry = data;
    (*node).set_count(1 as libc::c_int as libc::c_uint);
    (*node).sz = sz;
    (*node).set_container(1 as libc::c_int as libc::c_uint);
    _quicklistInsertNodeAfter(quicklist, (*quicklist).tail, node);
    (*quicklist)
        .count = ((*quicklist).count).wrapping_add((*node).count() as libc::c_ulong);
}
#[no_mangle]
pub unsafe extern "C" fn __quicklistDelNode(
    mut quicklist: *mut quicklist,
    mut node: *mut quicklistNode,
) {
    let mut bm: *mut quicklistBookmark = _quicklistBookmarkFindByNode(quicklist, node);
    if !bm.is_null() {
        (*bm).node = (*node).next;
        if ((*bm).node).is_null() {
            _quicklistBookmarkDelete(quicklist, bm);
        }
    }
    if !((*node).next).is_null() {
        (*(*node).next).prev = (*node).prev;
    }
    if !((*node).prev).is_null() {
        (*(*node).prev).next = (*node).next;
    }
    if node == (*quicklist).tail {
        (*quicklist).tail = (*node).prev;
    }
    if node == (*quicklist).head {
        (*quicklist).head = (*node).next;
    }
    (*quicklist).len = ((*quicklist).len).wrapping_sub(1);
    (*quicklist)
        .count = ((*quicklist).count).wrapping_sub((*node).count() as libc::c_ulong);
    __quicklistCompress(quicklist, 0 as *mut quicklistNode);
    zfree((*node).entry as *mut libc::c_void);
    zfree(node as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistDelIndex(
    mut quicklist: *mut quicklist,
    mut node: *mut quicklistNode,
    mut p: *mut *mut libc::c_uchar,
) -> libc::c_int {
    let mut gone: libc::c_int = 0 as libc::c_int;
    if ((*node).container() as libc::c_int == 1 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        __quicklistDelNode(quicklist, node);
        return 1 as libc::c_int;
    }
    (*node).entry = lpDelete((*node).entry, *p, p);
    (*node).set_count((*node).count() - 1);
    if (*node).count() as libc::c_int == 0 as libc::c_int {
        gone = 1 as libc::c_int;
        __quicklistDelNode(quicklist, node);
    } else {
        (*node).sz = lpBytes((*node).entry);
    }
    (*quicklist).count = ((*quicklist).count).wrapping_sub(1);
    return if gone != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn quicklistDelEntry(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
) {
    let mut prev: *mut quicklistNode = (*(*entry).node).prev;
    let mut next: *mut quicklistNode = (*(*entry).node).next;
    let mut deleted_node: libc::c_int = quicklistDelIndex(
        (*entry).quicklist as *mut quicklist,
        (*entry).node,
        &mut (*entry).zi,
    );
    (*iter).zi = 0 as *mut libc::c_uchar;
    if deleted_node != 0 {
        if (*iter).direction == 0 as libc::c_int {
            (*iter).current = next;
            (*iter).offset = 0 as libc::c_int as libc::c_long;
        } else if (*iter).direction == 1 as libc::c_int {
            (*iter).current = prev;
            (*iter).offset = -(1 as libc::c_int) as libc::c_long;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn quicklistReplaceEntry(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
    mut data: *mut libc::c_void,
    mut sz: size_t,
) {
    let mut quicklist: *mut quicklist = (*iter).quicklist;
    if (!((*(*entry).node).container() as libc::c_int == 1 as libc::c_int)
        && !(sz >= packed_threshold)) as libc::c_int as libc::c_long != 0
    {
        (*(*entry).node)
            .entry = lpReplace(
            (*(*entry).node).entry,
            &mut (*entry).zi,
            data as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*(*entry).node).sz = lpBytes((*(*entry).node).entry);
        if (*(*entry).node).recompress() != 0 {
            if !((*entry).node).is_null()
                && (*(*entry).node).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode((*entry).node);
            }
        } else {
            __quicklistCompress(quicklist, (*entry).node);
        }
    } else if (*(*entry).node).container() as libc::c_int == 1 as libc::c_int {
        if sz >= packed_threshold {
            zfree((*(*entry).node).entry as *mut libc::c_void);
            (*(*entry).node).entry = zmalloc(sz) as *mut libc::c_uchar;
            (*(*entry).node).sz = sz;
            memcpy((*(*entry).node).entry as *mut libc::c_void, data, sz);
            if (*(*entry).node).recompress() != 0 {
                if !((*entry).node).is_null()
                    && (*(*entry).node).encoding() as libc::c_int == 1 as libc::c_int
                {
                    __quicklistCompressNode((*entry).node);
                }
            } else {
                __quicklistCompress(quicklist, (*entry).node);
            }
        } else {
            quicklistInsertAfter(iter, entry, data, sz);
            __quicklistDelNode(quicklist, (*entry).node);
        }
    } else {
        (*(*entry).node).set_dont_compress(1 as libc::c_int as libc::c_uint);
        quicklistInsertAfter(iter, entry, data, sz);
        if (*(*entry).node).count() as libc::c_int == 1 as libc::c_int {
            __quicklistDelNode(quicklist, (*entry).node);
        } else {
            let mut p: *mut libc::c_uchar = lpSeek(
                (*(*entry).node).entry,
                -(1 as libc::c_int) as libc::c_long,
            );
            quicklistDelIndex(quicklist, (*entry).node, &mut p);
            (*(*entry).node).set_dont_compress(0 as libc::c_int as libc::c_uint);
            if (*(*entry).node).recompress() != 0 {
                if !((*entry).node).is_null()
                    && (*(*entry).node).encoding() as libc::c_int == 1 as libc::c_int
                {
                    __quicklistCompressNode((*entry).node);
                }
            } else {
                __quicklistCompress(quicklist, (*entry).node);
            }
            if (*(*(*entry).node).next).recompress() != 0 {
                if !((*(*entry).node).next).is_null()
                    && (*(*(*entry).node).next).encoding() as libc::c_int
                        == 1 as libc::c_int
                {
                    __quicklistCompressNode((*(*entry).node).next);
                }
            } else {
                __quicklistCompress(quicklist, (*(*entry).node).next);
            }
        }
    }
    (*iter).current = 0 as *mut quicklistNode;
    (*iter).zi = 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistReplaceAtIndex(
    mut quicklist: *mut quicklist,
    mut index: libc::c_long,
    mut data: *mut libc::c_void,
    mut sz: size_t,
) -> libc::c_int {
    let mut entry: quicklistEntry = quicklistEntry {
        quicklist: 0 as *const quicklist,
        node: 0 as *mut quicklistNode,
        zi: 0 as *mut libc::c_uchar,
        value: 0 as *mut libc::c_uchar,
        longval: 0,
        sz: 0,
        offset: 0,
    };
    let mut iter: *mut quicklistIter = quicklistGetIteratorEntryAtIdx(
        quicklist,
        index as libc::c_longlong,
        &mut entry,
    );
    if !iter.is_null() as libc::c_int as libc::c_long != 0 {
        quicklistReplaceEntry(iter, &mut entry, data, sz);
        quicklistReleaseIterator(iter);
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistListpackMerge(
    mut quicklist: *mut quicklist,
    mut a: *mut quicklistNode,
    mut b: *mut quicklistNode,
) -> *mut quicklistNode {
    if !a.is_null() && (*a).encoding() as libc::c_int == 2 as libc::c_int {
        __quicklistDecompressNode(a);
    }
    if !b.is_null() && (*b).encoding() as libc::c_int == 2 as libc::c_int {
        __quicklistDecompressNode(b);
    }
    if !(lpMerge(&mut (*a).entry, &mut (*b).entry)).is_null() {
        let mut keep: *mut quicklistNode = 0 as *mut quicklistNode;
        let mut nokeep: *mut quicklistNode = 0 as *mut quicklistNode;
        if ((*a).entry).is_null() {
            nokeep = a;
            keep = b;
        } else if ((*b).entry).is_null() {
            nokeep = b;
            keep = a;
        }
        (*keep).set_count(lpLength((*keep).entry) as libc::c_uint);
        (*keep).sz = lpBytes((*keep).entry);
        (*nokeep).set_count(0 as libc::c_int as libc::c_uint);
        __quicklistDelNode(quicklist, nokeep);
        if (*keep).recompress() != 0 {
            if !keep.is_null() && (*keep).encoding() as libc::c_int == 1 as libc::c_int {
                __quicklistCompressNode(keep);
            }
        } else {
            __quicklistCompress(quicklist, keep);
        }
        return keep;
    } else {
        return 0 as *mut quicklistNode
    };
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistMergeNodes(
    mut quicklist: *mut quicklist,
    mut center: *mut quicklistNode,
) {
    let mut fill: libc::c_int = (*quicklist).fill();
    let mut prev: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut prev_prev: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut next: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut next_next: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut target: *mut quicklistNode = 0 as *mut quicklistNode;
    target = 0 as *mut quicklistNode;
    next_next = target;
    next = next_next;
    prev_prev = next;
    prev = prev_prev;
    if !((*center).prev).is_null() {
        prev = (*center).prev;
        if !((*(*center).prev).prev).is_null() {
            prev_prev = (*(*center).prev).prev;
        }
    }
    if !((*center).next).is_null() {
        next = (*center).next;
        if !((*(*center).next).next).is_null() {
            next_next = (*(*center).next).next;
        }
    }
    if _quicklistNodeAllowMerge(prev, prev_prev, fill) != 0 {
        _quicklistListpackMerge(quicklist, prev_prev, prev);
        prev = 0 as *mut quicklistNode;
        prev_prev = prev;
    }
    if _quicklistNodeAllowMerge(next, next_next, fill) != 0 {
        _quicklistListpackMerge(quicklist, next, next_next);
        next_next = 0 as *mut quicklistNode;
        next = next_next;
    }
    if _quicklistNodeAllowMerge(center, (*center).prev, fill) != 0 {
        target = _quicklistListpackMerge(quicklist, (*center).prev, center);
        center = 0 as *mut quicklistNode;
    } else {
        target = center;
    }
    if _quicklistNodeAllowMerge(target, (*target).next, fill) != 0 {
        _quicklistListpackMerge(quicklist, target, (*target).next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistSplitNode(
    mut node: *mut quicklistNode,
    mut offset: libc::c_int,
    mut after: libc::c_int,
) -> *mut quicklistNode {
    let mut zl_sz: size_t = (*node).sz;
    let mut new_node: *mut quicklistNode = quicklistCreateNode();
    (*new_node).entry = zmalloc(zl_sz) as *mut libc::c_uchar;
    memcpy(
        (*new_node).entry as *mut libc::c_void,
        (*node).entry as *const libc::c_void,
        zl_sz,
    );
    if offset < 0 as libc::c_int {
        offset = (*node).count() as libc::c_int + offset;
    }
    let mut orig_start: libc::c_int = if after != 0 {
        offset + 1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut orig_extent: libc::c_int = if after != 0 {
        -(1 as libc::c_int)
    } else {
        offset
    };
    let mut new_start: libc::c_int = if after != 0 { 0 as libc::c_int } else { offset };
    let mut new_extent: libc::c_int = if after != 0 {
        offset + 1 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    (*node)
        .entry = lpDeleteRange(
        (*node).entry,
        orig_start as libc::c_long,
        orig_extent as libc::c_ulong,
    );
    (*node).set_count(lpLength((*node).entry) as libc::c_uint);
    (*node).sz = lpBytes((*node).entry);
    (*new_node)
        .entry = lpDeleteRange(
        (*new_node).entry,
        new_start as libc::c_long,
        new_extent as libc::c_ulong,
    );
    (*new_node).set_count(lpLength((*new_node).entry) as libc::c_uint);
    (*new_node).sz = lpBytes((*new_node).entry);
    return new_node;
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistInsert(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
    mut value: *mut libc::c_void,
    sz: size_t,
    mut after: libc::c_int,
) {
    let mut quicklist: *mut quicklist = (*iter).quicklist;
    let mut full: libc::c_int = 0 as libc::c_int;
    let mut at_tail: libc::c_int = 0 as libc::c_int;
    let mut at_head: libc::c_int = 0 as libc::c_int;
    let mut avail_next: libc::c_int = 0 as libc::c_int;
    let mut avail_prev: libc::c_int = 0 as libc::c_int;
    let mut fill: libc::c_int = (*quicklist).fill();
    let mut node: *mut quicklistNode = (*entry).node;
    let mut new_node: *mut quicklistNode = 0 as *mut quicklistNode;
    if node.is_null() {
        if (sz >= packed_threshold) as libc::c_int as libc::c_long != 0 {
            __quicklistInsertPlainNode(quicklist, (*quicklist).tail, value, sz, after);
            return;
        }
        new_node = quicklistCreateNode();
        (*new_node)
            .entry = lpPrepend(
            lpNew(0 as libc::c_int as size_t),
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        __quicklistInsertNode(quicklist, 0 as *mut quicklistNode, new_node, after);
        (*new_node).set_count((*new_node).count() + 1);
        (*quicklist).count = ((*quicklist).count).wrapping_add(1);
        return;
    }
    if _quicklistNodeAllowInsert(node, fill, sz) == 0 {
        full = 1 as libc::c_int;
    }
    if after != 0
        && ((*entry).offset == (*node).count() as libc::c_int - 1 as libc::c_int
            || (*entry).offset == -(1 as libc::c_int))
    {
        at_tail = 1 as libc::c_int;
        if _quicklistNodeAllowInsert((*node).next, fill, sz) != 0 {
            avail_next = 1 as libc::c_int;
        }
    }
    if after == 0
        && ((*entry).offset == 0 as libc::c_int
            || (*entry).offset == -((*node).count() as libc::c_int))
    {
        at_head = 1 as libc::c_int;
        if _quicklistNodeAllowInsert((*node).prev, fill, sz) != 0 {
            avail_prev = 1 as libc::c_int;
        }
    }
    if (sz >= packed_threshold) as libc::c_int as libc::c_long != 0 {
        if (*node).container() as libc::c_int == 1 as libc::c_int
            || at_tail != 0 && after != 0 || at_head != 0 && after == 0
        {
            __quicklistInsertPlainNode(quicklist, node, value, sz, after);
        } else {
            if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
                __quicklistDecompressNode(node);
                (*node).set_recompress(1 as libc::c_int as libc::c_uint);
            }
            new_node = _quicklistSplitNode(node, (*entry).offset, after);
            let mut entry_node: *mut quicklistNode = __quicklistCreatePlainNode(
                value,
                sz,
            );
            __quicklistInsertNode(quicklist, node, entry_node, after);
            __quicklistInsertNode(quicklist, entry_node, new_node, after);
            (*quicklist).count = ((*quicklist).count).wrapping_add(1);
        }
        return;
    }
    if full == 0 && after != 0 {
        if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
            __quicklistDecompressNode(node);
            (*node).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        (*node)
            .entry = lpInsertString(
            (*node).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
            (*entry).zi,
            1 as libc::c_int,
            0 as *mut *mut libc::c_uchar,
        );
        (*node).set_count((*node).count() + 1);
        (*node).sz = lpBytes((*node).entry);
        if (*node).recompress() != 0 {
            if !node.is_null() && (*node).encoding() as libc::c_int == 1 as libc::c_int {
                __quicklistCompressNode(node);
            }
        }
    } else if full == 0 && after == 0 {
        if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
            __quicklistDecompressNode(node);
            (*node).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        (*node)
            .entry = lpInsertString(
            (*node).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
            (*entry).zi,
            0 as libc::c_int,
            0 as *mut *mut libc::c_uchar,
        );
        (*node).set_count((*node).count() + 1);
        (*node).sz = lpBytes((*node).entry);
        if (*node).recompress() != 0 {
            if !node.is_null() && (*node).encoding() as libc::c_int == 1 as libc::c_int {
                __quicklistCompressNode(node);
            }
        }
    } else if full != 0 && at_tail != 0 && avail_next != 0 && after != 0 {
        new_node = (*node).next;
        if !new_node.is_null()
            && (*new_node).encoding() as libc::c_int == 2 as libc::c_int
        {
            __quicklistDecompressNode(new_node);
            (*new_node).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        (*new_node)
            .entry = lpPrepend(
            (*new_node).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*new_node).set_count((*new_node).count() + 1);
        (*new_node).sz = lpBytes((*new_node).entry);
        if (*new_node).recompress() != 0 {
            if !new_node.is_null()
                && (*new_node).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode(new_node);
            }
        }
        if (*node).recompress() != 0 {
            if !node.is_null() && (*node).encoding() as libc::c_int == 1 as libc::c_int {
                __quicklistCompressNode(node);
            }
        }
    } else if full != 0 && at_head != 0 && avail_prev != 0 && after == 0 {
        new_node = (*node).prev;
        if !new_node.is_null()
            && (*new_node).encoding() as libc::c_int == 2 as libc::c_int
        {
            __quicklistDecompressNode(new_node);
            (*new_node).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        (*new_node)
            .entry = lpAppend(
            (*new_node).entry,
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*new_node).set_count((*new_node).count() + 1);
        (*new_node).sz = lpBytes((*new_node).entry);
        if (*new_node).recompress() != 0 {
            if !new_node.is_null()
                && (*new_node).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode(new_node);
            }
        }
        if (*node).recompress() != 0 {
            if !node.is_null() && (*node).encoding() as libc::c_int == 1 as libc::c_int {
                __quicklistCompressNode(node);
            }
        }
    } else if full != 0
        && (at_tail != 0 && avail_next == 0 && after != 0
            || at_head != 0 && avail_prev == 0 && after == 0)
    {
        new_node = quicklistCreateNode();
        (*new_node)
            .entry = lpPrepend(
            lpNew(0 as libc::c_int as size_t),
            value as *mut libc::c_uchar,
            sz as uint32_t,
        );
        (*new_node).set_count((*new_node).count() + 1);
        (*new_node).sz = lpBytes((*new_node).entry);
        __quicklistInsertNode(quicklist, node, new_node, after);
    } else if full != 0 {
        if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
            __quicklistDecompressNode(node);
            (*node).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        new_node = _quicklistSplitNode(node, (*entry).offset, after);
        if after != 0 {
            (*new_node)
                .entry = lpPrepend(
                (*new_node).entry,
                value as *mut libc::c_uchar,
                sz as uint32_t,
            );
        } else {
            (*new_node)
                .entry = lpAppend(
                (*new_node).entry,
                value as *mut libc::c_uchar,
                sz as uint32_t,
            );
        }
        (*new_node).set_count((*new_node).count() + 1);
        (*new_node).sz = lpBytes((*new_node).entry);
        __quicklistInsertNode(quicklist, node, new_node, after);
        _quicklistMergeNodes(quicklist, node);
    }
    (*quicklist).count = ((*quicklist).count).wrapping_add(1);
    (*iter).current = 0 as *mut quicklistNode;
    (*iter).zi = 0 as *mut libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistInsertBefore(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
    mut value: *mut libc::c_void,
    sz: size_t,
) {
    _quicklistInsert(iter, entry, value, sz, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistInsertAfter(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
    mut value: *mut libc::c_void,
    sz: size_t,
) {
    _quicklistInsert(iter, entry, value, sz, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistDelRange(
    mut quicklist: *mut quicklist,
    start: libc::c_long,
    count: libc::c_long,
) -> libc::c_int {
    if count <= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int;
    }
    let mut extent: libc::c_ulong = count as libc::c_ulong;
    if start >= 0 as libc::c_int as libc::c_long
        && extent > ((*quicklist).count).wrapping_sub(start as libc::c_ulong)
    {
        extent = ((*quicklist).count).wrapping_sub(start as libc::c_ulong);
    } else if start < 0 as libc::c_int as libc::c_long
        && extent > -start as libc::c_ulong
    {
        extent = -start as libc::c_ulong;
    }
    let mut iter: *mut quicklistIter = quicklistGetIteratorAtIdx(
        quicklist,
        1 as libc::c_int,
        start as libc::c_longlong,
    );
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    let mut node: *mut quicklistNode = (*iter).current;
    let mut offset: libc::c_long = (*iter).offset;
    quicklistReleaseIterator(iter);
    while extent != 0 {
        let mut next: *mut quicklistNode = (*node).next;
        let mut del: libc::c_ulong = 0;
        let mut delete_entire_node: libc::c_int = 0 as libc::c_int;
        if offset == 0 as libc::c_int as libc::c_long
            && extent >= (*node).count() as libc::c_ulong
        {
            delete_entire_node = 1 as libc::c_int;
            del = (*node).count() as libc::c_ulong;
        } else if offset >= 0 as libc::c_int as libc::c_long
            && extent.wrapping_add(offset as libc::c_ulong)
                >= (*node).count() as libc::c_ulong
        {
            del = ((*node).count() as libc::c_long - offset) as libc::c_ulong;
        } else if offset < 0 as libc::c_int as libc::c_long {
            del = -offset as libc::c_ulong;
            if del > extent {
                del = extent;
            }
        } else {
            del = extent;
        }
        if delete_entire_node != 0
            || (*node).container() as libc::c_int == 1 as libc::c_int
        {
            __quicklistDelNode(quicklist, node);
        } else {
            if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
                __quicklistDecompressNode(node);
                (*node).set_recompress(1 as libc::c_int as libc::c_uint);
            }
            (*node).entry = lpDeleteRange((*node).entry, offset, del);
            (*node).sz = lpBytes((*node).entry);
            (*node).set_count((*node).count() - del as libc::c_uint);
            (*quicklist).count = ((*quicklist).count).wrapping_sub(del);
            if (*node).count() as libc::c_int == 0 as libc::c_int {
                __quicklistDelNode(quicklist, node);
                node = 0 as *mut quicklistNode;
            }
            if !node.is_null() {
                if (*node).recompress() != 0 {
                    if !node.is_null()
                        && (*node).encoding() as libc::c_int == 1 as libc::c_int
                    {
                        __quicklistCompressNode(node);
                    }
                }
            }
        }
        extent = extent.wrapping_sub(del);
        node = next;
        offset = 0 as libc::c_int as libc::c_long;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistCompare(
    mut entry: *mut quicklistEntry,
    mut p2: *mut libc::c_uchar,
    p2_len: size_t,
) -> libc::c_int {
    if ((*(*entry).node).container() as libc::c_int == 1 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        return ((*entry).sz == p2_len
            && memcmp(
                (*entry).value as *const libc::c_void,
                p2 as *const libc::c_void,
                p2_len,
            ) == 0 as libc::c_int) as libc::c_int;
    }
    return lpCompare((*entry).zi, p2, p2_len as uint32_t) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistGetIterator(
    mut quicklist: *mut quicklist,
    mut direction: libc::c_int,
) -> *mut quicklistIter {
    let mut iter: *mut quicklistIter = 0 as *mut quicklistIter;
    iter = zmalloc(core::mem::size_of::<quicklistIter>() as libc::c_ulong)
        as *mut quicklistIter;
    if direction == 0 as libc::c_int {
        (*iter).current = (*quicklist).head;
        (*iter).offset = 0 as libc::c_int as libc::c_long;
    } else if direction == 1 as libc::c_int {
        (*iter).current = (*quicklist).tail;
        (*iter).offset = -(1 as libc::c_int) as libc::c_long;
    }
    (*iter).direction = direction;
    (*iter).quicklist = quicklist;
    (*iter).zi = 0 as *mut libc::c_uchar;
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistGetIteratorAtIdx(
    mut quicklist: *mut quicklist,
    direction: libc::c_int,
    idx: libc::c_longlong,
) -> *mut quicklistIter {
    let mut n: *mut quicklistNode = 0 as *mut quicklistNode;
    let mut accum: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut index: libc::c_ulonglong = 0;
    let mut forward: libc::c_int = if idx < 0 as libc::c_int as libc::c_longlong {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    index = (if forward != 0 {
        idx
    } else {
        -idx - 1 as libc::c_int as libc::c_longlong
    }) as libc::c_ulonglong;
    if index >= (*quicklist).count as libc::c_ulonglong {
        return 0 as *mut quicklistIter;
    }
    let mut seek_forward: libc::c_int = forward;
    let mut seek_index: libc::c_ulonglong = index;
    if index
        > ((*quicklist).count)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_ulonglong
    {
        seek_forward = (forward == 0) as libc::c_int;
        seek_index = (((*quicklist).count)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_ulonglong)
            .wrapping_sub(index);
    }
    n = if seek_forward != 0 { (*quicklist).head } else { (*quicklist).tail };
    while !n.is_null() as libc::c_int as libc::c_long != 0 {
        if accum.wrapping_add((*n).count() as libc::c_ulonglong) > seek_index {
            break;
        }
        accum = accum.wrapping_add((*n).count() as libc::c_ulonglong);
        n = if seek_forward != 0 { (*n).next } else { (*n).prev };
    }
    if n.is_null() {
        return 0 as *mut quicklistIter;
    }
    if seek_forward != forward {
        accum = (((*quicklist).count).wrapping_sub((*n).count() as libc::c_ulong)
            as libc::c_ulonglong)
            .wrapping_sub(accum);
    }
    let mut iter: *mut quicklistIter = quicklistGetIterator(quicklist, direction);
    (*iter).current = n;
    if forward != 0 {
        (*iter).offset = index.wrapping_sub(accum) as libc::c_long;
    } else {
        (*iter)
            .offset = index
            .wrapping_neg()
            .wrapping_sub(1 as libc::c_int as libc::c_ulonglong)
            .wrapping_add(accum) as libc::c_long;
    }
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistReleaseIterator(mut iter: *mut quicklistIter) {
    if iter.is_null() {
        return;
    }
    if !((*iter).current).is_null() {
        if (*(*iter).current).recompress() != 0 {
            if !((*iter).current).is_null()
                && (*(*iter).current).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode((*iter).current);
            }
        } else {
            __quicklistCompress((*iter).quicklist, (*iter).current);
        }
    }
    zfree(iter as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn quicklistNext(
    mut iter: *mut quicklistIter,
    mut entry: *mut quicklistEntry,
) -> libc::c_int {
    (*entry).value = 0 as *mut libc::c_uchar;
    (*entry).zi = (*entry).value;
    (*entry).longval = -(123456789 as libc::c_int) as libc::c_longlong;
    (*entry).quicklist = 0 as *const quicklist;
    (*entry).node = 0 as *mut quicklistNode;
    (*entry).offset = 123456789 as libc::c_int;
    (*entry).sz = 0 as libc::c_int as size_t;
    if iter.is_null() {
        return 0 as libc::c_int;
    }
    (*entry).quicklist = (*iter).quicklist;
    (*entry).node = (*iter).current;
    if ((*iter).current).is_null() {
        return 0 as libc::c_int;
    }
    let mut nextFn: Option::<
        unsafe extern "C" fn(
            *mut libc::c_uchar,
            *mut libc::c_uchar,
        ) -> *mut libc::c_uchar,
    > = None;
    let mut offset_update: libc::c_int = 0 as libc::c_int;
    let mut plain: libc::c_int = ((*(*iter).current).container() as libc::c_int
        == 1 as libc::c_int) as libc::c_int;
    if ((*iter).zi).is_null() {
        if !((*iter).current).is_null()
            && (*(*iter).current).encoding() as libc::c_int == 2 as libc::c_int
        {
            __quicklistDecompressNode((*iter).current);
            (*(*iter).current).set_recompress(1 as libc::c_int as libc::c_uint);
        }
        if (plain != 0) as libc::c_int as libc::c_long != 0 {
            (*iter).zi = (*(*iter).current).entry;
        } else {
            (*iter).zi = lpSeek((*(*iter).current).entry, (*iter).offset);
        }
    } else if (plain != 0) as libc::c_int as libc::c_long != 0 {
        (*iter).zi = 0 as *mut libc::c_uchar;
    } else {
        if (*iter).direction == 0 as libc::c_int {
            nextFn = Some(
                lpNext
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        *mut libc::c_uchar,
                    ) -> *mut libc::c_uchar,
            );
            offset_update = 1 as libc::c_int;
        } else if (*iter).direction == 1 as libc::c_int {
            nextFn = Some(
                lpPrev
                    as unsafe extern "C" fn(
                        *mut libc::c_uchar,
                        *mut libc::c_uchar,
                    ) -> *mut libc::c_uchar,
            );
            offset_update = -(1 as libc::c_int);
        }
        (*iter)
            .zi = nextFn
            .expect("non-null function pointer")((*(*iter).current).entry, (*iter).zi);
        (*iter).offset += offset_update as libc::c_long;
    }
    (*entry).zi = (*iter).zi;
    (*entry).offset = (*iter).offset as libc::c_int;
    if !((*iter).zi).is_null() {
        if (plain != 0) as libc::c_int as libc::c_long != 0 {
            (*entry).value = (*(*entry).node).entry;
            (*entry).sz = (*(*entry).node).sz;
            return 1 as libc::c_int;
        }
        let mut sz: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        (*entry).value = lpGetValue((*entry).zi, &mut sz, &mut (*entry).longval);
        (*entry).sz = sz as size_t;
        return 1 as libc::c_int;
    } else {
        if (*(*iter).current).recompress() != 0 {
            if !((*iter).current).is_null()
                && (*(*iter).current).encoding() as libc::c_int == 1 as libc::c_int
            {
                __quicklistCompressNode((*iter).current);
            }
        } else {
            __quicklistCompress((*iter).quicklist, (*iter).current);
        }
        if (*iter).direction == 0 as libc::c_int {
            (*iter).current = (*(*iter).current).next;
            (*iter).offset = 0 as libc::c_int as libc::c_long;
        } else if (*iter).direction == 1 as libc::c_int {
            (*iter).current = (*(*iter).current).prev;
            (*iter).offset = -(1 as libc::c_int) as libc::c_long;
        }
        (*iter).zi = 0 as *mut libc::c_uchar;
        return quicklistNext(iter, entry);
    };
}
#[no_mangle]
pub unsafe extern "C" fn quicklistSetDirection(
    mut iter: *mut quicklistIter,
    mut direction: libc::c_int,
) {
    (*iter).direction = direction;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistDup(mut orig: *mut quicklist) -> *mut quicklist {
    let mut copy: *mut quicklist = 0 as *mut quicklist;
    copy = quicklistNew((*orig).fill(), (*orig).compress() as libc::c_int);
    let mut current: *mut quicklistNode = (*orig).head;
    while !current.is_null() {
        let mut node: *mut quicklistNode = quicklistCreateNode();
        if (*current).encoding() as libc::c_int == 2 as libc::c_int {
            let mut lzf: *mut quicklistLZF = (*current).entry as *mut quicklistLZF;
            let mut lzf_sz: size_t = (core::mem::size_of::<quicklistLZF>()
                as libc::c_ulong)
                .wrapping_add((*lzf).sz);
            (*node).entry = zmalloc(lzf_sz) as *mut libc::c_uchar;
            memcpy(
                (*node).entry as *mut libc::c_void,
                (*current).entry as *const libc::c_void,
                lzf_sz,
            );
        } else if (*current).encoding() as libc::c_int == 1 as libc::c_int {
            (*node).entry = zmalloc((*current).sz) as *mut libc::c_uchar;
            memcpy(
                (*node).entry as *mut libc::c_void,
                (*current).entry as *const libc::c_void,
                (*current).sz,
            );
        }
        (*node).set_count((*current).count());
        (*copy).count = ((*copy).count).wrapping_add((*node).count() as libc::c_ulong);
        (*node).sz = (*current).sz;
        (*node).set_encoding((*current).encoding());
        (*node).set_container((*current).container());
        _quicklistInsertNodeAfter(copy, (*copy).tail, node);
        current = (*current).next;
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistGetIteratorEntryAtIdx(
    mut quicklist: *mut quicklist,
    idx: libc::c_longlong,
    mut entry: *mut quicklistEntry,
) -> *mut quicklistIter {
    let mut iter: *mut quicklistIter = quicklistGetIteratorAtIdx(
        quicklist,
        1 as libc::c_int,
        idx,
    );
    if iter.is_null() {
        return 0 as *mut quicklistIter;
    }
    if (quicklistNext(iter, entry) != 0) as libc::c_int as libc::c_long != 0 {} else {
        _serverAssert(
            b"quicklistNext(iter, entry)\0" as *const u8 as *const libc::c_char,
            b"quicklist.c\0" as *const u8 as *const libc::c_char,
            1431 as libc::c_int,
        );
        unreachable!();
    };
    return iter;
}
unsafe extern "C" fn quicklistRotatePlain(mut quicklist: *mut quicklist) {
    let mut new_head: *mut quicklistNode = (*quicklist).tail;
    let mut new_tail: *mut quicklistNode = (*(*quicklist).tail).prev;
    (*(*quicklist).head).prev = new_head;
    (*new_tail).next = 0 as *mut quicklistNode;
    (*new_head).next = (*quicklist).head;
    (*new_head).prev = 0 as *mut quicklistNode;
    (*quicklist).head = new_head;
    (*quicklist).tail = new_tail;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistRotate(mut quicklist: *mut quicklist) {
    if (*quicklist).count <= 1 as libc::c_int as libc::c_ulong {
        return;
    }
    if ((*(*quicklist).tail).container() as libc::c_int == 1 as libc::c_int)
        as libc::c_int as libc::c_long != 0
    {
        quicklistRotatePlain(quicklist);
        return;
    }
    let mut p: *mut libc::c_uchar = lpSeek(
        (*(*quicklist).tail).entry,
        -(1 as libc::c_int) as libc::c_long,
    );
    let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut tmp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut longval: libc::c_longlong = 0;
    let mut sz: libc::c_uint = 0;
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
    tmp = lpGetValue(p, &mut sz, &mut longval);
    if tmp.is_null() {
        sz = ll2string(
            longstr.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 32]>() as libc::c_ulong,
            longval,
        ) as libc::c_uint;
        value = longstr.as_mut_ptr() as *mut libc::c_uchar;
    } else if (*quicklist).len == 1 as libc::c_int as libc::c_ulong {
        value = zmalloc(sz as size_t) as *mut libc::c_uchar;
        memcpy(
            value as *mut libc::c_void,
            tmp as *const libc::c_void,
            sz as libc::c_ulong,
        );
    } else {
        value = tmp;
    }
    quicklistPushHead(quicklist, value as *mut libc::c_void, sz as size_t);
    if (*quicklist).len == 1 as libc::c_int as libc::c_ulong {
        p = lpSeek((*(*quicklist).tail).entry, -(1 as libc::c_int) as libc::c_long);
    }
    quicklistDelIndex(quicklist, (*quicklist).tail, &mut p);
    if value != longstr.as_mut_ptr() as *mut libc::c_uchar && value != tmp {
        zfree(value as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn quicklistPopCustom(
    mut quicklist: *mut quicklist,
    mut where_0: libc::c_int,
    mut data: *mut *mut libc::c_uchar,
    mut sz: *mut size_t,
    mut sval: *mut libc::c_longlong,
    mut saver: Option::<
        unsafe extern "C" fn(*mut libc::c_uchar, size_t) -> *mut libc::c_void,
    >,
) -> libc::c_int {
    let mut p: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: libc::c_uint = 0;
    let mut vlong: libc::c_longlong = 0;
    let mut pos: libc::c_int = if where_0 == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
    if (*quicklist).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if !data.is_null() {
        *data = 0 as *mut libc::c_uchar;
    }
    if !sz.is_null() {
        *sz = 0 as libc::c_int as size_t;
    }
    if !sval.is_null() {
        *sval = -(123456789 as libc::c_int) as libc::c_longlong;
    }
    let mut node: *mut quicklistNode = 0 as *mut quicklistNode;
    if where_0 == 0 as libc::c_int && !((*quicklist).head).is_null() {
        node = (*quicklist).head;
    } else if where_0 == -(1 as libc::c_int) && !((*quicklist).tail).is_null() {
        node = (*quicklist).tail;
    } else {
        return 0 as libc::c_int
    }
    if ((*node).encoding() as libc::c_int != 2 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {} else {
        _serverAssert(
            b"node->encoding != QUICKLIST_NODE_ENCODING_LZF\0" as *const u8
                as *const libc::c_char,
            b"quicklist.c\0" as *const u8 as *const libc::c_char,
            1532 as libc::c_int,
        );
        unreachable!();
    };
    if ((*node).container() as libc::c_int == 1 as libc::c_int) as libc::c_int
        as libc::c_long != 0
    {
        if !data.is_null() {
            *data = saver.expect("non-null function pointer")((*node).entry, (*node).sz)
                as *mut libc::c_uchar;
        }
        if !sz.is_null() {
            *sz = (*node).sz;
        }
        quicklistDelIndex(quicklist, node, 0 as *mut *mut libc::c_uchar);
        return 1 as libc::c_int;
    }
    p = lpSeek((*node).entry, pos as libc::c_long);
    vstr = lpGetValue(p, &mut vlen, &mut vlong);
    if !vstr.is_null() {
        if !data.is_null() {
            *data = saver.expect("non-null function pointer")(vstr, vlen as size_t)
                as *mut libc::c_uchar;
        }
        if !sz.is_null() {
            *sz = vlen as size_t;
        }
    } else {
        if !data.is_null() {
            *data = 0 as *mut libc::c_uchar;
        }
        if !sval.is_null() {
            *sval = vlong;
        }
    }
    quicklistDelIndex(quicklist, node, &mut p);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistSaver(
    mut data: *mut libc::c_uchar,
    mut sz: size_t,
) -> *mut libc::c_void {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if !data.is_null() {
        vstr = zmalloc(sz) as *mut libc::c_uchar;
        memcpy(vstr as *mut libc::c_void, data as *const libc::c_void, sz);
        return vstr as *mut libc::c_void;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistPop(
    mut quicklist: *mut quicklist,
    mut where_0: libc::c_int,
    mut data: *mut *mut libc::c_uchar,
    mut sz: *mut size_t,
    mut slong: *mut libc::c_longlong,
) -> libc::c_int {
    let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut vlen: size_t = 0;
    let mut vlong: libc::c_longlong = 0;
    if (*quicklist).count == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = quicklistPopCustom(
        quicklist,
        where_0,
        &mut vstr,
        &mut vlen,
        &mut vlong,
        Some(
            _quicklistSaver
                as unsafe extern "C" fn(*mut libc::c_uchar, size_t) -> *mut libc::c_void,
        ),
    );
    if !data.is_null() {
        *data = vstr;
    }
    if !slong.is_null() {
        *slong = vlong;
    }
    if !sz.is_null() {
        *sz = vlen;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistPush(
    mut quicklist: *mut quicklist,
    mut value: *mut libc::c_void,
    sz: size_t,
    mut where_0: libc::c_int,
) {
    if !((*quicklist).head).is_null() {
        if ((*(*quicklist).head).encoding() as libc::c_int != 2 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"quicklist->head->encoding != QUICKLIST_NODE_ENCODING_LZF\0"
                    as *const u8 as *const libc::c_char,
                b"quicklist.c\0" as *const u8 as *const libc::c_char,
                1597 as libc::c_int,
            );
            unreachable!();
        };
    }
    if !((*quicklist).tail).is_null() {
        if ((*(*quicklist).tail).encoding() as libc::c_int != 2 as libc::c_int)
            as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"quicklist->tail->encoding != QUICKLIST_NODE_ENCODING_LZF\0"
                    as *const u8 as *const libc::c_char,
                b"quicklist.c\0" as *const u8 as *const libc::c_char,
                1599 as libc::c_int,
            );
            unreachable!();
        };
    }
    if where_0 == 0 as libc::c_int {
        quicklistPushHead(quicklist, value, sz);
    } else if where_0 == -(1 as libc::c_int) {
        quicklistPushTail(quicklist, value, sz);
    }
}
#[no_mangle]
pub unsafe extern "C" fn quicklistRepr(
    mut ql: *mut libc::c_uchar,
    mut full: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut quicklist: *mut quicklist = ql as *mut quicklist;
    printf(b"{count : %ld}\n\0" as *const u8 as *const libc::c_char, (*quicklist).count);
    printf(b"{len : %ld}\n\0" as *const u8 as *const libc::c_char, (*quicklist).len);
    printf(b"{fill : %d}\n\0" as *const u8 as *const libc::c_char, (*quicklist).fill());
    printf(
        b"{compress : %d}\n\0" as *const u8 as *const libc::c_char,
        (*quicklist).compress() as libc::c_int,
    );
    printf(
        b"{bookmark_count : %d}\n\0" as *const u8 as *const libc::c_char,
        (*quicklist).bookmark_count() as libc::c_int,
    );
    let mut node: *mut quicklistNode = (*quicklist).head;
    while !node.is_null() {
        let fresh2 = i;
        i = i + 1;
        printf(b"{quicklist node(%d)\n\0" as *const u8 as *const libc::c_char, fresh2);
        printf(
            b"{container : %s, encoding: %s, size: %zu, count: %d, recompress: %d, attempted_compress: %d}\n\0"
                as *const u8 as *const libc::c_char,
            if (*node).container() as libc::c_int == 1 as libc::c_int {
                b"PLAIN\0" as *const u8 as *const libc::c_char
            } else {
                b"PACKED\0" as *const u8 as *const libc::c_char
            },
            if (*node).encoding() as libc::c_int == 1 as libc::c_int {
                b"RAW\0" as *const u8 as *const libc::c_char
            } else {
                b"LZF\0" as *const u8 as *const libc::c_char
            },
            (*node).sz,
            (*node).count() as libc::c_int,
            (*node).recompress() as libc::c_int,
            (*node).attempted_compress() as libc::c_int,
        );
        if full != 0 {
            if !node.is_null() && (*node).encoding() as libc::c_int == 2 as libc::c_int {
                __quicklistDecompressNode(node);
            }
            if (*node).container() as libc::c_int == 2 as libc::c_int {
                printf(b"{ listpack:\n\0" as *const u8 as *const libc::c_char);
                lpRepr((*node).entry);
                printf(b"}\n\0" as *const u8 as *const libc::c_char);
            } else if (*node).container() as libc::c_int == 1 as libc::c_int {
                printf(
                    b"{ entry : %s }\n\0" as *const u8 as *const libc::c_char,
                    (*node).entry,
                );
            }
            printf(b"}\n\0" as *const u8 as *const libc::c_char);
            if (*node).recompress() != 0 {
                if !node.is_null()
                    && (*node).encoding() as libc::c_int == 1 as libc::c_int
                {
                    __quicklistCompressNode(node);
                }
            }
        }
        node = (*node).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn quicklistBookmarkCreate(
    mut ql_ref: *mut *mut quicklist,
    mut name: *const libc::c_char,
    mut node: *mut quicklistNode,
) -> libc::c_int {
    let mut ql: *mut quicklist = *ql_ref;
    if (*ql).bookmark_count() as libc::c_int
        >= ((1 as libc::c_int) << 4 as libc::c_int) - 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut bm: *mut quicklistBookmark = _quicklistBookmarkFindByName(ql, name);
    if !bm.is_null() {
        (*bm).node = node;
        return 1 as libc::c_int;
    }
    ql = zrealloc(
        ql as *mut libc::c_void,
        (core::mem::size_of::<quicklist>() as libc::c_ulong)
            .wrapping_add(
                (((*ql).bookmark_count() as libc::c_int + 1 as libc::c_int)
                    as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<quicklistBookmark>() as libc::c_ulong,
                    ),
            ),
    ) as *mut quicklist;
    *ql_ref = ql;
    let ref mut fresh3 = (*((*ql).bookmarks)
        .as_mut_ptr()
        .offset((*ql).bookmark_count() as isize))
        .node;
    *fresh3 = node;
    let ref mut fresh4 = (*((*ql).bookmarks)
        .as_mut_ptr()
        .offset((*ql).bookmark_count() as isize))
        .name;
    *fresh4 = zstrdup(name);
    (*ql).set_bookmark_count((*ql).bookmark_count() + 1);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistBookmarkFind(
    mut ql: *mut quicklist,
    mut name: *const libc::c_char,
) -> *mut quicklistNode {
    let mut bm: *mut quicklistBookmark = _quicklistBookmarkFindByName(ql, name);
    if bm.is_null() {
        return 0 as *mut quicklistNode;
    }
    return (*bm).node;
}
#[no_mangle]
pub unsafe extern "C" fn quicklistBookmarkDelete(
    mut ql: *mut quicklist,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut bm: *mut quicklistBookmark = _quicklistBookmarkFindByName(ql, name);
    if bm.is_null() {
        return 0 as libc::c_int;
    }
    _quicklistBookmarkDelete(ql, bm);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistBookmarkFindByName(
    mut ql: *mut quicklist,
    mut name: *const libc::c_char,
) -> *mut quicklistBookmark {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*ql).bookmark_count() {
        if strcmp((*((*ql).bookmarks).as_mut_ptr().offset(i as isize)).name, name) == 0 {
            return &mut *((*ql).bookmarks).as_mut_ptr().offset(i as isize)
                as *mut quicklistBookmark;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut quicklistBookmark;
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistBookmarkFindByNode(
    mut ql: *mut quicklist,
    mut node: *mut quicklistNode,
) -> *mut quicklistBookmark {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (*ql).bookmark_count() {
        if (*((*ql).bookmarks).as_mut_ptr().offset(i as isize)).node == node {
            return &mut *((*ql).bookmarks).as_mut_ptr().offset(i as isize)
                as *mut quicklistBookmark;
        }
        i = i.wrapping_add(1);
    }
    return 0 as *mut quicklistBookmark;
}
#[no_mangle]
pub unsafe extern "C" fn _quicklistBookmarkDelete(
    mut ql: *mut quicklist,
    mut bm: *mut quicklistBookmark,
) {
    let mut index: libc::c_int = bm.offset_from(((*ql).bookmarks).as_mut_ptr())
        as libc::c_long as libc::c_int;
    zfree((*bm).name as *mut libc::c_void);
    (*ql).set_bookmark_count((*ql).bookmark_count() - 1);
    memmove(
        bm as *mut libc::c_void,
        bm.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (((*ql).bookmark_count() as libc::c_int - index) as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<quicklistBookmark>() as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn quicklistBookmarksClear(mut ql: *mut quicklist) {
    while (*ql).bookmark_count() != 0 {
        (*ql).set_bookmark_count((*ql).bookmark_count() - 1);
        zfree(
            (*((*ql).bookmarks).as_mut_ptr().offset((*ql).bookmark_count() as isize))
                .name as *mut libc::c_void,
        );
    }
}

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rand() -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn log(_: libc::c_double) -> libc::c_double;
    fn floor(_: libc::c_double) -> libc::c_double;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
#[inline]
unsafe extern "C" fn putchar(mut __c: libc::c_int) -> libc::c_int {
    return putc(__c, stdout);
}
#[no_mangle]
pub static mut raxNotFound: *mut libc::c_void = b"rax-not-found-pointer\0" as *const u8
    as *const libc::c_char as *mut libc::c_void;
static mut raxDebugMsg: libc::c_int = 1 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn raxSetDebugMsg(mut onoff: libc::c_int) {
    raxDebugMsg = onoff;
}
#[inline]
unsafe extern "C" fn raxStackInit(mut ts: *mut raxStack) {
    (*ts).stack = ((*ts).static_items).as_mut_ptr();
    (*ts).items = 0 as libc::c_int as size_t;
    (*ts).maxitems = 32 as libc::c_int as size_t;
    (*ts).oom = 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn raxStackPush(
    mut ts: *mut raxStack,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if (*ts).items == (*ts).maxitems {
        if (*ts).stack == ((*ts).static_items).as_mut_ptr() {
            (*ts)
                .stack = zmalloc(
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_mul((*ts).maxitems)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            ) as *mut *mut libc::c_void;
            if ((*ts).stack).is_null() {
                (*ts).stack = ((*ts).static_items).as_mut_ptr();
                (*ts).oom = 1 as libc::c_int;
                *__errno_location() = 12 as libc::c_int;
                return 0 as libc::c_int;
            }
            memcpy(
                (*ts).stack as *mut libc::c_void,
                ((*ts).static_items).as_mut_ptr() as *const libc::c_void,
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_mul((*ts).maxitems),
            );
        } else {
            let mut newalloc: *mut *mut libc::c_void = zrealloc(
                (*ts).stack as *mut libc::c_void,
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_mul((*ts).maxitems)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong),
            ) as *mut *mut libc::c_void;
            if newalloc.is_null() {
                (*ts).oom = 1 as libc::c_int;
                *__errno_location() = 12 as libc::c_int;
                return 0 as libc::c_int;
            }
            (*ts).stack = newalloc;
        }
        (*ts)
            .maxitems = ((*ts).maxitems as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
    }
    let ref mut fresh0 = *((*ts).stack).offset((*ts).items as isize);
    *fresh0 = ptr;
    (*ts).items = ((*ts).items).wrapping_add(1);
    return 1 as libc::c_int;
}
#[inline]
unsafe extern "C" fn raxStackPop(mut ts: *mut raxStack) -> *mut libc::c_void {
    if (*ts).items == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    (*ts).items = ((*ts).items).wrapping_sub(1);
    return *((*ts).stack).offset((*ts).items as isize);
}
#[inline]
unsafe extern "C" fn raxStackPeek(mut ts: *mut raxStack) -> *mut libc::c_void {
    if (*ts).items == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    return *((*ts).stack)
        .offset(((*ts).items).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize);
}
#[inline]
unsafe extern "C" fn raxStackFree(mut ts: *mut raxStack) {
    if (*ts).stack != ((*ts).static_items).as_mut_ptr() {
        zfree((*ts).stack as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn raxNewNode(
    mut children: size_t,
    mut datafield: libc::c_int,
) -> *mut raxNode {
    let mut nodesize: size_t = (core::mem::size_of::<raxNode>() as libc::c_ulong)
        .wrapping_add(children)
        .wrapping_add(
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    children
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                .wrapping_mul(children),
        );
    if datafield != 0 {
        nodesize = (nodesize as libc::c_ulong)
            .wrapping_add(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
            as size_t as size_t;
    }
    let mut node: *mut raxNode = zmalloc(nodesize) as *mut raxNode;
    if node.is_null() {
        return 0 as *mut raxNode;
    }
    (*node).set_iskey(0 as libc::c_int as uint32_t);
    (*node).set_isnull(0 as libc::c_int as uint32_t);
    (*node).set_iscompr(0 as libc::c_int as uint32_t);
    (*node).set_size(children as uint32_t);
    return node;
}
#[no_mangle]
pub unsafe extern "C" fn raxNew() -> *mut rax {
    let mut rax: *mut rax = zmalloc(core::mem::size_of::<rax>() as libc::c_ulong)
        as *mut rax;
    if rax.is_null() {
        return 0 as *mut rax;
    }
    (*rax).numele = 0 as libc::c_int as uint64_t;
    (*rax).numnodes = 1 as libc::c_int as uint64_t;
    (*rax).head = raxNewNode(0 as libc::c_int as size_t, 0 as libc::c_int);
    if ((*rax).head).is_null() {
        zfree(rax as *mut libc::c_void);
        return 0 as *mut rax;
    } else {
        return rax
    };
}
#[no_mangle]
pub unsafe extern "C" fn raxReallocForData(
    mut n: *mut raxNode,
    mut data: *mut libc::c_void,
) -> *mut raxNode {
    if data.is_null() {
        return n;
    }
    let mut curlen: size_t = (core::mem::size_of::<raxNode>() as libc::c_ulong)
        .wrapping_add((*n).size() as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (if (*n).iscompr() as libc::c_int != 0 {
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
            } else {
                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                    .wrapping_mul((*n).size() as libc::c_ulong)
            }),
        )
        .wrapping_add(
            (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0) as libc::c_int
                as libc::c_ulong)
                .wrapping_mul(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    return zrealloc(
        n as *mut libc::c_void,
        curlen.wrapping_add(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong),
    ) as *mut raxNode;
}
#[no_mangle]
pub unsafe extern "C" fn raxSetData(mut n: *mut raxNode, mut data: *mut libc::c_void) {
    (*n).set_iskey(1 as libc::c_int as uint32_t);
    if !data.is_null() {
        (*n).set_isnull(0 as libc::c_int as uint32_t);
        let mut ndata: *mut *mut libc::c_void = (n as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*n).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*n).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*n).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*n).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0)
                            as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(
                -(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize),
            ) as *mut *mut libc::c_void;
        memcpy(
            ndata as *mut libc::c_void,
            &mut data as *mut *mut libc::c_void as *const libc::c_void,
            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
    } else {
        (*n).set_isnull(1 as libc::c_int as uint32_t);
    };
}
#[no_mangle]
pub unsafe extern "C" fn raxGetData(mut n: *mut raxNode) -> *mut libc::c_void {
    if (*n).isnull() != 0 {
        return 0 as *mut libc::c_void;
    }
    let mut ndata: *mut *mut libc::c_void = (n as *mut libc::c_char)
        .offset(
            (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add((*n).size() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*n).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(
                    (if (*n).iscompr() as libc::c_int != 0 {
                        core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                    } else {
                        (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                            .wrapping_mul((*n).size() as libc::c_ulong)
                    }),
                )
                .wrapping_add(
                    (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0)
                        as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                ) as isize,
        )
        .offset(-(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize))
        as *mut *mut libc::c_void;
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    memcpy(
        &mut data as *mut *mut libc::c_void as *mut libc::c_void,
        ndata as *const libc::c_void,
        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
    );
    return data;
}
#[no_mangle]
pub unsafe extern "C" fn raxAddChild(
    mut n: *mut raxNode,
    mut c: libc::c_uchar,
    mut childptr: *mut *mut raxNode,
    mut parentlink: *mut *mut *mut raxNode,
) -> *mut raxNode {
    if (*n).iscompr() as libc::c_int == 0 as libc::c_int {} else {
        __assert_fail(
            b"n->iscompr == 0\0" as *const u8 as *const libc::c_char,
            b"rax.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 72],
                &[libc::c_char; 72],
            >(
                b"raxNode *raxAddChild(raxNode *, unsigned char, raxNode **, raxNode ***)\0",
            ))
                .as_ptr(),
        );
    };
    let mut curlen: size_t = (core::mem::size_of::<raxNode>() as libc::c_ulong)
        .wrapping_add((*n).size() as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (if (*n).iscompr() as libc::c_int != 0 {
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
            } else {
                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                    .wrapping_mul((*n).size() as libc::c_ulong)
            }),
        )
        .wrapping_add(
            (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0) as libc::c_int
                as libc::c_ulong)
                .wrapping_mul(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    (*n).set_size((*n).size() + 1);
    let mut newlen: size_t = (core::mem::size_of::<raxNode>() as libc::c_ulong)
        .wrapping_add((*n).size() as libc::c_ulong)
        .wrapping_add(
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(
            (if (*n).iscompr() as libc::c_int != 0 {
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
            } else {
                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                    .wrapping_mul((*n).size() as libc::c_ulong)
            }),
        )
        .wrapping_add(
            (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0) as libc::c_int
                as libc::c_ulong)
                .wrapping_mul(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ),
        );
    (*n).set_size((*n).size() - 1);
    let mut child: *mut raxNode = raxNewNode(
        0 as libc::c_int as size_t,
        0 as libc::c_int,
    );
    if child.is_null() {
        return 0 as *mut raxNode;
    }
    let mut newn: *mut raxNode = zrealloc(n as *mut libc::c_void, newlen)
        as *mut raxNode;
    if newn.is_null() {
        zfree(child as *mut libc::c_void);
        return 0 as *mut raxNode;
    }
    n = newn;
    let mut pos: libc::c_int = 0;
    pos = 0 as libc::c_int;
    while pos < (*n).size() as libc::c_int {
        if *((*n).data).as_mut_ptr().offset(pos as isize) as libc::c_int
            > c as libc::c_int
        {
            break;
        }
        pos += 1;
    }
    let mut src: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut dst: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0 {
        src = (n as *mut libc::c_uchar)
            .offset(curlen as isize)
            .offset(
                -(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize),
            );
        dst = (n as *mut libc::c_uchar)
            .offset(newlen as isize)
            .offset(
                -(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong as isize),
            );
        memmove(
            dst as *mut libc::c_void,
            src as *const libc::c_void,
            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
        );
    }
    let mut shift: size_t = newlen
        .wrapping_sub(curlen)
        .wrapping_sub(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong);
    src = ((*n).data)
        .as_mut_ptr()
        .offset((*n).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        )
        .offset(
            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                .wrapping_mul(pos as libc::c_ulong) as isize,
        );
    memmove(
        src
            .offset(shift as isize)
            .offset(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize)
            as *mut libc::c_void,
        src as *const libc::c_void,
        (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
            .wrapping_mul(((*n).size() as libc::c_int - pos) as libc::c_ulong),
    );
    if shift != 0 {
        src = ((*n).data)
            .as_mut_ptr()
            .offset((*n).size() as libc::c_int as isize)
            .offset(
                ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(
                        (((*n).size() as libc::c_int + 4 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_rem(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    )
                    & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut *mut raxNode as *mut libc::c_uchar;
        memmove(
            src.offset(shift as isize) as *mut libc::c_void,
            src as *const libc::c_void,
            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                .wrapping_mul(pos as libc::c_ulong),
        );
    }
    src = ((*n).data).as_mut_ptr().offset(pos as isize);
    memmove(
        src.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        src as *const libc::c_void,
        ((*n).size() as libc::c_int - pos) as libc::c_ulong,
    );
    *((*n).data).as_mut_ptr().offset(pos as isize) = c;
    (*n).set_size((*n).size() + 1);
    src = ((*n).data)
        .as_mut_ptr()
        .offset((*n).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut *mut raxNode as *mut libc::c_uchar;
    let mut childfield: *mut *mut raxNode = src
        .offset(
            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                .wrapping_mul(pos as libc::c_ulong) as isize,
        ) as *mut *mut raxNode;
    memcpy(
        childfield as *mut libc::c_void,
        &mut child as *mut *mut raxNode as *const libc::c_void,
        core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
    );
    *childptr = child;
    *parentlink = childfield;
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn raxCompressNode(
    mut n: *mut raxNode,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut child: *mut *mut raxNode,
) -> *mut raxNode {
    if (*n).size() as libc::c_int == 0 as libc::c_int
        && (*n).iscompr() as libc::c_int == 0 as libc::c_int
    {} else {
        __assert_fail(
            b"n->size == 0 && n->iscompr == 0\0" as *const u8 as *const libc::c_char,
            b"rax.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 73],
                &[libc::c_char; 73],
            >(
                b"raxNode *raxCompressNode(raxNode *, unsigned char *, size_t, raxNode **)\0",
            ))
                .as_ptr(),
        );
    };
    let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut newsize: size_t = 0;
    *child = raxNewNode(0 as libc::c_int as size_t, 0 as libc::c_int);
    if (*child).is_null() {
        return 0 as *mut raxNode;
    }
    newsize = (core::mem::size_of::<raxNode>() as libc::c_ulong)
        .wrapping_add(len)
        .wrapping_add(
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    len
                        .wrapping_add(4 as libc::c_int as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )
        .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
    if (*n).iskey() != 0 {
        data = raxGetData(n);
        if (*n).isnull() == 0 {
            newsize = (newsize as libc::c_ulong)
                .wrapping_add(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as size_t as size_t;
        }
    }
    let mut newn: *mut raxNode = zrealloc(n as *mut libc::c_void, newsize)
        as *mut raxNode;
    if newn.is_null() {
        zfree(*child as *mut libc::c_void);
        return 0 as *mut raxNode;
    }
    n = newn;
    (*n).set_iscompr(1 as libc::c_int as uint32_t);
    (*n).set_size(len as uint32_t);
    memcpy(((*n).data).as_mut_ptr() as *mut libc::c_void, s as *const libc::c_void, len);
    if (*n).iskey() != 0 {
        raxSetData(n, data);
    }
    let mut childfield: *mut *mut raxNode = (n as *mut libc::c_char)
        .offset(
            (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add((*n).size() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*n).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(
                    (if (*n).iscompr() as libc::c_int != 0 {
                        core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                    } else {
                        (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                            .wrapping_mul((*n).size() as libc::c_ulong)
                    }),
                )
                .wrapping_add(
                    (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0)
                        as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                ) as isize,
        )
        .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
        .offset(
            -((if (*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0 {
                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as isize),
        ) as *mut *mut raxNode;
    memcpy(
        childfield as *mut libc::c_void,
        child as *const libc::c_void,
        core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
    );
    return n;
}
#[inline]
unsafe extern "C" fn raxLowWalk(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut stopnode: *mut *mut raxNode,
    mut plink: *mut *mut *mut raxNode,
    mut splitpos: *mut libc::c_int,
    mut ts: *mut raxStack,
) -> size_t {
    let mut h: *mut raxNode = (*rax).head;
    let mut parentlink: *mut *mut raxNode = &mut (*rax).head;
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut j: size_t = 0 as libc::c_int as size_t;
    while (*h).size() as libc::c_int != 0 && i < len {
        let mut v: *mut libc::c_uchar = ((*h).data).as_mut_ptr();
        if (*h).iscompr() != 0 {
            j = 0 as libc::c_int as size_t;
            while j < (*h).size() as libc::c_ulong && i < len {
                if *v.offset(j as isize) as libc::c_int
                    != *s.offset(i as isize) as libc::c_int
                {
                    break;
                }
                j = j.wrapping_add(1);
                i = i.wrapping_add(1);
            }
            if j != (*h).size() as libc::c_ulong {
                break;
            }
        } else {
            j = 0 as libc::c_int as size_t;
            while j < (*h).size() as libc::c_ulong {
                if *v.offset(j as isize) as libc::c_int
                    == *s.offset(i as isize) as libc::c_int
                {
                    break;
                }
                j = j.wrapping_add(1);
            }
            if j == (*h).size() as libc::c_ulong {
                break;
            }
            i = i.wrapping_add(1);
        }
        if !ts.is_null() {
            raxStackPush(ts, h as *mut libc::c_void);
        }
        let mut children: *mut *mut raxNode = ((*h).data)
            .as_mut_ptr()
            .offset((*h).size() as libc::c_int as isize)
            .offset(
                ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(
                        (((*h).size() as libc::c_int + 4 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_rem(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    )
                    & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
            ) as *mut *mut raxNode;
        if (*h).iscompr() != 0 {
            j = 0 as libc::c_int as size_t;
        }
        memcpy(
            &mut h as *mut *mut raxNode as *mut libc::c_void,
            children.offset(j as isize) as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        parentlink = children.offset(j as isize);
        j = 0 as libc::c_int as size_t;
    }
    if !stopnode.is_null() {
        *stopnode = h;
    }
    if !plink.is_null() {
        *plink = parentlink;
    }
    if !splitpos.is_null() && (*h).iscompr() as libc::c_int != 0 {
        *splitpos = j as libc::c_int;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn raxGenericInsert(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut data: *mut libc::c_void,
    mut old: *mut *mut libc::c_void,
    mut overwrite: libc::c_int,
) -> libc::c_int {
    let mut newh_1: *mut raxNode = 0 as *mut raxNode;
    let mut current_block: u64;
    let mut i: size_t = 0;
    let mut j: libc::c_int = 0 as libc::c_int;
    let mut h: *mut raxNode = 0 as *mut raxNode;
    let mut parentlink: *mut *mut raxNode = 0 as *mut *mut raxNode;
    i = raxLowWalk(rax, s, len, &mut h, &mut parentlink, &mut j, 0 as *mut raxStack);
    if i == len && ((*h).iscompr() == 0 || j == 0 as libc::c_int) {
        if (*h).iskey() == 0 || (*h).isnull() as libc::c_int != 0 && overwrite != 0 {
            h = raxReallocForData(h, data);
            if !h.is_null() {
                memcpy(
                    parentlink as *mut libc::c_void,
                    &mut h as *mut *mut raxNode as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
            }
        }
        if h.is_null() {
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int;
        }
        if (*h).iskey() != 0 {
            if !old.is_null() {
                *old = raxGetData(h);
            }
            if overwrite != 0 {
                raxSetData(h, data);
            }
            *__errno_location() = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        raxSetData(h, data);
        (*rax).numele = ((*rax).numele).wrapping_add(1);
        return 1 as libc::c_int;
    }
    if (*h).iscompr() as libc::c_int != 0 && i != len {
        let mut childfield: *mut *mut raxNode = (h as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*h).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*h).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*h).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*h).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0)
                            as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        let mut next: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut next as *mut *mut raxNode as *mut libc::c_void,
            childfield as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        (*h).iskey() != 0;
        let mut trimmedlen: size_t = j as size_t;
        let mut postfixlen: size_t = ((*h).size() as libc::c_int - j - 1 as libc::c_int)
            as size_t;
        let mut split_node_is_key: libc::c_int = (trimmedlen == 0
            && (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0) as libc::c_int;
        let mut nodesize: size_t = 0;
        let mut splitnode: *mut raxNode = raxNewNode(
            1 as libc::c_int as size_t,
            split_node_is_key,
        );
        let mut trimmed: *mut raxNode = 0 as *mut raxNode;
        let mut postfix: *mut raxNode = 0 as *mut raxNode;
        if trimmedlen != 0 {
            nodesize = (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add(trimmedlen)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            trimmedlen
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
            if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                nodesize = (nodesize as libc::c_ulong)
                    .wrapping_add(
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ) as size_t as size_t;
            }
            trimmed = zmalloc(nodesize) as *mut raxNode;
        }
        if postfixlen != 0 {
            nodesize = (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add(postfixlen)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            postfixlen
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
            postfix = zmalloc(nodesize) as *mut raxNode;
        }
        if splitnode.is_null() || trimmedlen != 0 && trimmed.is_null()
            || postfixlen != 0 && postfix.is_null()
        {
            zfree(splitnode as *mut libc::c_void);
            zfree(trimmed as *mut libc::c_void);
            zfree(postfix as *mut libc::c_void);
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int;
        }
        *((*splitnode).data)
            .as_mut_ptr()
            .offset(
                0 as libc::c_int as isize,
            ) = *((*h).data).as_mut_ptr().offset(j as isize);
        if j == 0 as libc::c_int {
            if (*h).iskey() != 0 {
                let mut ndata: *mut libc::c_void = raxGetData(h);
                raxSetData(splitnode, ndata);
            }
            memcpy(
                parentlink as *mut libc::c_void,
                &mut splitnode as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
        } else {
            (*trimmed).set_size(j as uint32_t);
            memcpy(
                ((*trimmed).data).as_mut_ptr() as *mut libc::c_void,
                ((*h).data).as_mut_ptr() as *const libc::c_void,
                j as libc::c_ulong,
            );
            (*trimmed)
                .set_iscompr(
                    (if j > 1 as libc::c_int {
                        1 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as uint32_t,
                );
            (*trimmed).set_iskey((*h).iskey());
            (*trimmed).set_isnull((*h).isnull());
            if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                let mut ndata_0: *mut libc::c_void = raxGetData(h);
                raxSetData(trimmed, ndata_0);
            }
            let mut cp: *mut *mut raxNode = (trimmed as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*trimmed).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*trimmed).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*trimmed).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*trimmed).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*trimmed).iskey() as libc::c_int != 0
                                && (*trimmed).isnull() == 0) as libc::c_int
                                as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*trimmed).iskey() as libc::c_int != 0
                        && (*trimmed).isnull() == 0
                    {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            memcpy(
                cp as *mut libc::c_void,
                &mut splitnode as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            memcpy(
                parentlink as *mut libc::c_void,
                &mut trimmed as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            parentlink = cp;
            (*rax).numnodes = ((*rax).numnodes).wrapping_add(1);
        }
        if postfixlen != 0 {
            (*postfix).set_iskey(0 as libc::c_int as uint32_t);
            (*postfix).set_isnull(0 as libc::c_int as uint32_t);
            (*postfix).set_size(postfixlen as uint32_t);
            (*postfix)
                .set_iscompr(
                    (postfixlen > 1 as libc::c_int as libc::c_ulong) as libc::c_int
                        as uint32_t,
                );
            memcpy(
                ((*postfix).data).as_mut_ptr() as *mut libc::c_void,
                ((*h).data)
                    .as_mut_ptr()
                    .offset(j as isize)
                    .offset(1 as libc::c_int as isize) as *const libc::c_void,
                postfixlen,
            );
            let mut cp_0: *mut *mut raxNode = (postfix as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*postfix).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*postfix).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*postfix).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*postfix).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*postfix).iskey() as libc::c_int != 0
                                && (*postfix).isnull() == 0) as libc::c_int
                                as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*postfix).iskey() as libc::c_int != 0
                        && (*postfix).isnull() == 0
                    {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            memcpy(
                cp_0 as *mut libc::c_void,
                &mut next as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            (*rax).numnodes = ((*rax).numnodes).wrapping_add(1);
        } else {
            postfix = next;
        }
        let mut splitchild: *mut *mut raxNode = (splitnode as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*splitnode).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*splitnode).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*splitnode).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*splitnode).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*splitnode).iskey() as libc::c_int != 0
                            && (*splitnode).isnull() == 0) as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*splitnode).iskey() as libc::c_int != 0
                    && (*splitnode).isnull() == 0
                {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        memcpy(
            splitchild as *mut libc::c_void,
            &mut postfix as *mut *mut raxNode as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        zfree(h as *mut libc::c_void);
        h = splitnode;
    } else if (*h).iscompr() as libc::c_int != 0 && i == len {
        let mut postfixlen_0: size_t = ((*h).size() as libc::c_int - j) as size_t;
        let mut nodesize_0: size_t = (core::mem::size_of::<raxNode>() as libc::c_ulong)
            .wrapping_add(postfixlen_0)
            .wrapping_add(
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(
                        postfixlen_0
                            .wrapping_add(4 as libc::c_int as libc::c_ulong)
                            .wrapping_rem(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    )
                    & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
        if !data.is_null() {
            nodesize_0 = (nodesize_0 as libc::c_ulong)
                .wrapping_add(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as size_t as size_t;
        }
        let mut postfix_0: *mut raxNode = zmalloc(nodesize_0) as *mut raxNode;
        nodesize_0 = (core::mem::size_of::<raxNode>() as libc::c_ulong)
            .wrapping_add(j as libc::c_ulong)
            .wrapping_add(
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(
                        ((j + 4 as libc::c_int) as libc::c_ulong)
                            .wrapping_rem(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    )
                    & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
        if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
            nodesize_0 = (nodesize_0 as libc::c_ulong)
                .wrapping_add(
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                ) as size_t as size_t;
        }
        let mut trimmed_0: *mut raxNode = zmalloc(nodesize_0) as *mut raxNode;
        if postfix_0.is_null() || trimmed_0.is_null() {
            zfree(postfix_0 as *mut libc::c_void);
            zfree(trimmed_0 as *mut libc::c_void);
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int;
        }
        let mut childfield_0: *mut *mut raxNode = (h as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*h).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*h).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*h).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*h).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0)
                            as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        let mut next_0: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut next_0 as *mut *mut raxNode as *mut libc::c_void,
            childfield_0 as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        (*postfix_0).set_size(postfixlen_0 as uint32_t);
        (*postfix_0)
            .set_iscompr(
                (postfixlen_0 > 1 as libc::c_int as libc::c_ulong) as libc::c_int
                    as uint32_t,
            );
        (*postfix_0).set_iskey(1 as libc::c_int as uint32_t);
        (*postfix_0).set_isnull(0 as libc::c_int as uint32_t);
        memcpy(
            ((*postfix_0).data).as_mut_ptr() as *mut libc::c_void,
            ((*h).data).as_mut_ptr().offset(j as isize) as *const libc::c_void,
            postfixlen_0,
        );
        raxSetData(postfix_0, data);
        let mut cp_1: *mut *mut raxNode = (postfix_0 as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*postfix_0).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*postfix_0).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*postfix_0).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*postfix_0).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*postfix_0).iskey() as libc::c_int != 0
                            && (*postfix_0).isnull() == 0) as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*postfix_0).iskey() as libc::c_int != 0
                    && (*postfix_0).isnull() == 0
                {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        memcpy(
            cp_1 as *mut libc::c_void,
            &mut next_0 as *mut *mut raxNode as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        (*rax).numnodes = ((*rax).numnodes).wrapping_add(1);
        (*trimmed_0).set_size(j as uint32_t);
        (*trimmed_0).set_iscompr((j > 1 as libc::c_int) as libc::c_int as uint32_t);
        (*trimmed_0).set_iskey(0 as libc::c_int as uint32_t);
        (*trimmed_0).set_isnull(0 as libc::c_int as uint32_t);
        memcpy(
            ((*trimmed_0).data).as_mut_ptr() as *mut libc::c_void,
            ((*h).data).as_mut_ptr() as *const libc::c_void,
            j as libc::c_ulong,
        );
        memcpy(
            parentlink as *mut libc::c_void,
            &mut trimmed_0 as *mut *mut raxNode as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        if (*h).iskey() != 0 {
            let mut aux: *mut libc::c_void = raxGetData(h);
            raxSetData(trimmed_0, aux);
        }
        cp_1 = (trimmed_0 as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*trimmed_0).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*trimmed_0).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*trimmed_0).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*trimmed_0).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*trimmed_0).iskey() as libc::c_int != 0
                            && (*trimmed_0).isnull() == 0) as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*trimmed_0).iskey() as libc::c_int != 0
                    && (*trimmed_0).isnull() == 0
                {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        memcpy(
            cp_1 as *mut libc::c_void,
            &mut postfix_0 as *mut *mut raxNode as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        (*rax).numele = ((*rax).numele).wrapping_add(1);
        zfree(h as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    loop {
        if !(i < len) {
            current_block = 10109057886293123569;
            break;
        }
        let mut child: *mut raxNode = 0 as *mut raxNode;
        if (*h).size() as libc::c_int == 0 as libc::c_int
            && len.wrapping_sub(i) > 1 as libc::c_int as libc::c_ulong
        {
            let mut comprsize: size_t = len.wrapping_sub(i);
            if comprsize
                > (((1 as libc::c_int) << 29 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong
            {
                comprsize = (((1 as libc::c_int) << 29 as libc::c_int)
                    - 1 as libc::c_int) as size_t;
            }
            let mut newh: *mut raxNode = raxCompressNode(
                h,
                s.offset(i as isize),
                comprsize,
                &mut child,
            );
            if newh.is_null() {
                current_block = 3579863436504287334;
                break;
            }
            h = newh;
            memcpy(
                parentlink as *mut libc::c_void,
                &mut h as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            parentlink = (h as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*h).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*h).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*h).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*h).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0)
                                as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            i = (i as libc::c_ulong).wrapping_add(comprsize) as size_t as size_t;
        } else {
            let mut new_parentlink: *mut *mut raxNode = 0 as *mut *mut raxNode;
            let mut newh_0: *mut raxNode = raxAddChild(
                h,
                *s.offset(i as isize),
                &mut child,
                &mut new_parentlink,
            );
            if newh_0.is_null() {
                current_block = 3579863436504287334;
                break;
            }
            h = newh_0;
            memcpy(
                parentlink as *mut libc::c_void,
                &mut h as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            parentlink = new_parentlink;
            i = i.wrapping_add(1);
        }
        (*rax).numnodes = ((*rax).numnodes).wrapping_add(1);
        h = child;
    }
    match current_block {
        10109057886293123569 => {
            newh_1 = raxReallocForData(h, data);
            if !newh_1.is_null() {
                h = newh_1;
                if (*h).iskey() == 0 {
                    (*rax).numele = ((*rax).numele).wrapping_add(1);
                }
                raxSetData(h, data);
                memcpy(
                    parentlink as *mut libc::c_void,
                    &mut h as *mut *mut raxNode as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    if (*h).size() as libc::c_int == 0 as libc::c_int {
        (*h).set_isnull(1 as libc::c_int as uint32_t);
        (*h).set_iskey(1 as libc::c_int as uint32_t);
        (*rax).numele = ((*rax).numele).wrapping_add(1);
        if raxRemove(rax, s, i, 0 as *mut *mut libc::c_void) != 0 as libc::c_int
        {} else {
            __assert_fail(
                b"raxRemove(rax,s,i,NULL) != 0\0" as *const u8 as *const libc::c_char,
                b"rax.c\0" as *const u8 as *const libc::c_char,
                896 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 75],
                    &[libc::c_char; 75],
                >(
                    b"int raxGenericInsert(rax *, unsigned char *, size_t, void *, void **, int)\0",
                ))
                    .as_ptr(),
            );
        };
    }
    *__errno_location() = 12 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxInsert(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut data: *mut libc::c_void,
    mut old: *mut *mut libc::c_void,
) -> libc::c_int {
    return raxGenericInsert(rax, s, len, data, old, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn raxTryInsert(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut data: *mut libc::c_void,
    mut old: *mut *mut libc::c_void,
) -> libc::c_int {
    return raxGenericInsert(rax, s, len, data, old, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn raxFind(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
) -> *mut libc::c_void {
    let mut h: *mut raxNode = 0 as *mut raxNode;
    let mut splitpos: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = raxLowWalk(
        rax,
        s,
        len,
        &mut h,
        0 as *mut *mut *mut raxNode,
        &mut splitpos,
        0 as *mut raxStack,
    );
    if i != len || (*h).iscompr() as libc::c_int != 0 && splitpos != 0 as libc::c_int
        || (*h).iskey() == 0
    {
        return raxNotFound;
    }
    return raxGetData(h);
}
#[no_mangle]
pub unsafe extern "C" fn raxFindParentLink(
    mut parent: *mut raxNode,
    mut child: *mut raxNode,
) -> *mut *mut raxNode {
    let mut cp: *mut *mut raxNode = ((*parent).data)
        .as_mut_ptr()
        .offset((*parent).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*parent).size() as libc::c_int + 4 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut *mut raxNode;
    let mut c: *mut raxNode = 0 as *mut raxNode;
    loop {
        memcpy(
            &mut c as *mut *mut raxNode as *mut libc::c_void,
            cp as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        if c == child {
            break;
        }
        cp = cp.offset(1);
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn raxRemoveChild(
    mut parent: *mut raxNode,
    mut child: *mut raxNode,
) -> *mut raxNode {
    if (*parent).iscompr() != 0 {
        let mut data: *mut libc::c_void = 0 as *mut libc::c_void;
        if (*parent).iskey() != 0 {
            data = raxGetData(parent);
        }
        (*parent).set_isnull(0 as libc::c_int as uint32_t);
        (*parent).set_iscompr(0 as libc::c_int as uint32_t);
        (*parent).set_size(0 as libc::c_int as uint32_t);
        if (*parent).iskey() != 0 {
            raxSetData(parent, data);
        }
        return parent;
    }
    let mut cp: *mut *mut raxNode = ((*parent).data)
        .as_mut_ptr()
        .offset((*parent).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*parent).size() as libc::c_int + 4 as libc::c_int)
                        as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut *mut raxNode;
    let mut c: *mut *mut raxNode = cp;
    let mut e: *mut libc::c_uchar = ((*parent).data).as_mut_ptr();
    loop {
        let mut aux: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut aux as *mut *mut raxNode as *mut libc::c_void,
            c as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        if aux == child {
            break;
        }
        c = c.offset(1);
        e = e.offset(1);
    }
    let mut taillen: libc::c_int = ((*parent).size() as libc::c_long
        - e.offset_from(((*parent).data).as_mut_ptr()) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as libc::c_int;
    memmove(
        e as *mut libc::c_void,
        e.offset(1 as libc::c_int as isize) as *const libc::c_void,
        taillen as libc::c_ulong,
    );
    let mut shift: size_t = if (((*parent).size() as libc::c_int + 4 as libc::c_int)
        as libc::c_ulong)
        .wrapping_rem(core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
        == 1 as libc::c_int as libc::c_ulong
    {
        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    if shift != 0 {
        memmove(
            (cp as *mut libc::c_char).offset(-(shift as isize)) as *mut libc::c_void,
            cp as *const libc::c_void,
            (((*parent).size() as libc::c_int - taillen - 1 as libc::c_int)
                as libc::c_ulong)
                .wrapping_mul(
                    core::mem::size_of::<*mut *mut raxNode>() as libc::c_ulong,
                ),
        );
    }
    let mut valuelen: size_t = if (*parent).iskey() as libc::c_int != 0
        && (*parent).isnull() == 0
    {
        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    memmove(
        (c as *mut libc::c_char).offset(-(shift as isize)) as *mut libc::c_void,
        c.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (taillen as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<*mut *mut raxNode>() as libc::c_ulong)
            .wrapping_add(valuelen),
    );
    (*parent).set_size((*parent).size() - 1);
    let mut newnode: *mut raxNode = zrealloc(
        parent as *mut libc::c_void,
        (core::mem::size_of::<raxNode>() as libc::c_ulong)
            .wrapping_add((*parent).size() as libc::c_ulong)
            .wrapping_add(
                (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(
                        (((*parent).size() as libc::c_int + 4 as libc::c_int)
                            as libc::c_ulong)
                            .wrapping_rem(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    )
                    & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            )
            .wrapping_add(
                (if (*parent).iscompr() as libc::c_int != 0 {
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                } else {
                    (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                        .wrapping_mul((*parent).size() as libc::c_ulong)
                }),
            )
            .wrapping_add(
                (((*parent).iskey() as libc::c_int != 0 && (*parent).isnull() == 0)
                    as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                    ),
            ),
    ) as *mut raxNode;
    !newnode.is_null();
    return if !newnode.is_null() { newnode } else { parent };
}
#[no_mangle]
pub unsafe extern "C" fn raxRemove(
    mut rax: *mut rax,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
    mut old: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut h: *mut raxNode = 0 as *mut raxNode;
    let mut ts: raxStack = raxStack {
        stack: 0 as *mut *mut libc::c_void,
        items: 0,
        maxitems: 0,
        static_items: [0 as *mut libc::c_void; 32],
        oom: 0,
    };
    raxStackInit(&mut ts);
    let mut splitpos: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = raxLowWalk(
        rax,
        s,
        len,
        &mut h,
        0 as *mut *mut *mut raxNode,
        &mut splitpos,
        &mut ts,
    );
    if i != len || (*h).iscompr() as libc::c_int != 0 && splitpos != 0 as libc::c_int
        || (*h).iskey() == 0
    {
        raxStackFree(&mut ts);
        return 0 as libc::c_int;
    }
    if !old.is_null() {
        *old = raxGetData(h);
    }
    (*h).set_iskey(0 as libc::c_int as uint32_t);
    (*rax).numele = ((*rax).numele).wrapping_sub(1);
    let mut trycompress: libc::c_int = 0 as libc::c_int;
    if (*h).size() as libc::c_int == 0 as libc::c_int {
        let mut child: *mut raxNode = 0 as *mut raxNode;
        while h != (*rax).head {
            child = h;
            zfree(child as *mut libc::c_void);
            (*rax).numnodes = ((*rax).numnodes).wrapping_sub(1);
            h = raxStackPop(&mut ts) as *mut raxNode;
            if (*h).iskey() as libc::c_int != 0
                || (*h).iscompr() == 0 && (*h).size() as libc::c_int != 1 as libc::c_int
            {
                break;
            }
        }
        if !child.is_null() {
            let mut new: *mut raxNode = raxRemoveChild(h, child);
            if new != h {
                let mut parent: *mut raxNode = raxStackPeek(&mut ts) as *mut raxNode;
                let mut parentlink: *mut *mut raxNode = 0 as *mut *mut raxNode;
                if parent.is_null() {
                    parentlink = &mut (*rax).head;
                } else {
                    parentlink = raxFindParentLink(parent, h);
                }
                memcpy(
                    parentlink as *mut libc::c_void,
                    &mut new as *mut *mut raxNode as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
            }
            if (*new).size() as libc::c_int == 1 as libc::c_int
                && (*new).iskey() as libc::c_int == 0 as libc::c_int
            {
                trycompress = 1 as libc::c_int;
                h = new;
            }
        }
    } else if (*h).size() as libc::c_int == 1 as libc::c_int {
        trycompress = 1 as libc::c_int;
    }
    if trycompress != 0 && ts.oom != 0 {
        trycompress = 0 as libc::c_int;
    }
    if trycompress != 0 {
        let mut parent_0: *mut raxNode = 0 as *mut raxNode;
        loop {
            parent_0 = raxStackPop(&mut ts) as *mut raxNode;
            if parent_0.is_null() || (*parent_0).iskey() as libc::c_int != 0
                || (*parent_0).iscompr() == 0
                    && (*parent_0).size() as libc::c_int != 1 as libc::c_int
            {
                break;
            }
            h = parent_0;
        }
        let mut start: *mut raxNode = h;
        let mut comprsize: size_t = (*h).size() as size_t;
        let mut nodes: libc::c_int = 1 as libc::c_int;
        while (*h).size() as libc::c_int != 0 as libc::c_int {
            let mut cp: *mut *mut raxNode = (h as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*h).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*h).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*h).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*h).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0)
                                as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            memcpy(
                &mut h as *mut *mut raxNode as *mut libc::c_void,
                cp as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            if (*h).iskey() as libc::c_int != 0
                || (*h).iscompr() == 0 && (*h).size() as libc::c_int != 1 as libc::c_int
            {
                break;
            }
            if comprsize.wrapping_add((*h).size() as libc::c_ulong)
                > (((1 as libc::c_int) << 29 as libc::c_int) - 1 as libc::c_int)
                    as libc::c_ulong
            {
                break;
            }
            nodes += 1;
            comprsize = (comprsize as libc::c_ulong)
                .wrapping_add((*h).size() as libc::c_ulong) as size_t as size_t;
        }
        if nodes > 1 as libc::c_int {
            let mut nodesize: size_t = (core::mem::size_of::<raxNode>()
                as libc::c_ulong)
                .wrapping_add(comprsize)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            comprsize
                                .wrapping_add(4 as libc::c_int as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(core::mem::size_of::<*mut raxNode>() as libc::c_ulong);
            let mut new_0: *mut raxNode = zmalloc(nodesize) as *mut raxNode;
            if new_0.is_null() {
                raxStackFree(&mut ts);
                return 1 as libc::c_int;
            }
            (*new_0).set_iskey(0 as libc::c_int as uint32_t);
            (*new_0).set_isnull(0 as libc::c_int as uint32_t);
            (*new_0).set_iscompr(1 as libc::c_int as uint32_t);
            (*new_0).set_size(comprsize as uint32_t);
            (*rax).numnodes = ((*rax).numnodes).wrapping_add(1);
            comprsize = 0 as libc::c_int as size_t;
            h = start;
            while (*h).size() as libc::c_int != 0 as libc::c_int {
                memcpy(
                    ((*new_0).data).as_mut_ptr().offset(comprsize as isize)
                        as *mut libc::c_void,
                    ((*h).data).as_mut_ptr() as *const libc::c_void,
                    (*h).size() as libc::c_ulong,
                );
                comprsize = (comprsize as libc::c_ulong)
                    .wrapping_add((*h).size() as libc::c_ulong) as size_t as size_t;
                let mut cp_0: *mut *mut raxNode = (h as *mut libc::c_char)
                    .offset(
                        (core::mem::size_of::<raxNode>() as libc::c_ulong)
                            .wrapping_add((*h).size() as libc::c_ulong)
                            .wrapping_add(
                                (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(
                                        (((*h).size() as libc::c_int + 4 as libc::c_int)
                                            as libc::c_ulong)
                                            .wrapping_rem(
                                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                            ),
                                    )
                                    & (core::mem::size_of::<*mut libc::c_void>()
                                        as libc::c_ulong)
                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            )
                            .wrapping_add(
                                (if (*h).iscompr() as libc::c_int != 0 {
                                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                                } else {
                                    (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                        .wrapping_mul((*h).size() as libc::c_ulong)
                                }),
                            )
                            .wrapping_add(
                                (((*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0)
                                    as libc::c_int as libc::c_ulong)
                                    .wrapping_mul(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            ) as isize,
                    )
                    .offset(
                        -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            as isize),
                    )
                    .offset(
                        -((if (*h).iskey() as libc::c_int != 0 && (*h).isnull() == 0 {
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                        } else {
                            0 as libc::c_int as libc::c_ulong
                        }) as isize),
                    ) as *mut *mut raxNode;
                let mut tofree: *mut raxNode = h;
                memcpy(
                    &mut h as *mut *mut raxNode as *mut libc::c_void,
                    cp_0 as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
                zfree(tofree as *mut libc::c_void);
                (*rax).numnodes = ((*rax).numnodes).wrapping_sub(1);
                if (*h).iskey() as libc::c_int != 0
                    || (*h).iscompr() == 0
                        && (*h).size() as libc::c_int != 1 as libc::c_int
                {
                    break;
                }
            }
            let mut cp_1: *mut *mut raxNode = (new_0 as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*new_0).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*new_0).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*new_0).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*new_0).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*new_0).iskey() as libc::c_int != 0
                                && (*new_0).isnull() == 0) as libc::c_int as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*new_0).iskey() as libc::c_int != 0 && (*new_0).isnull() == 0
                    {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            memcpy(
                cp_1 as *mut libc::c_void,
                &mut h as *mut *mut raxNode as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            if !parent_0.is_null() {
                let mut parentlink_0: *mut *mut raxNode = raxFindParentLink(
                    parent_0,
                    start,
                );
                memcpy(
                    parentlink_0 as *mut libc::c_void,
                    &mut new_0 as *mut *mut raxNode as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
            } else {
                (*rax).head = new_0;
            }
        }
    }
    raxStackFree(&mut ts);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxRecursiveFree(
    mut rax: *mut rax,
    mut n: *mut raxNode,
    mut free_callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    let mut numchildren: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        (*n).size() as libc::c_int
    };
    let mut cp: *mut *mut raxNode = (n as *mut libc::c_char)
        .offset(
            (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add((*n).size() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*n).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(
                    (if (*n).iscompr() as libc::c_int != 0 {
                        core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                    } else {
                        (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                            .wrapping_mul((*n).size() as libc::c_ulong)
                    }),
                )
                .wrapping_add(
                    (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0)
                        as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                ) as isize,
        )
        .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
        .offset(
            -((if (*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0 {
                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as isize),
        ) as *mut *mut raxNode;
    loop {
        let fresh1 = numchildren;
        numchildren = numchildren - 1;
        if !(fresh1 != 0) {
            break;
        }
        let mut child: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut child as *mut *mut raxNode as *mut libc::c_void,
            cp as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        raxRecursiveFree(rax, child, free_callback);
        cp = cp.offset(-1);
    }
    if free_callback.is_some() && (*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0
    {
        free_callback.expect("non-null function pointer")(raxGetData(n));
    }
    zfree(n as *mut libc::c_void);
    (*rax).numnodes = ((*rax).numnodes).wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn raxFreeWithCallback(
    mut rax: *mut rax,
    mut free_callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
) {
    raxRecursiveFree(rax, (*rax).head, free_callback);
    if (*rax).numnodes == 0 as libc::c_int as libc::c_ulong {} else {
        __assert_fail(
            b"rax->numnodes == 0\0" as *const u8 as *const libc::c_char,
            b"rax.c\0" as *const u8 as *const libc::c_char,
            1244 as libc::c_int as libc::c_uint,
            (*core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void raxFreeWithCallback(rax *, void (*)(void *))\0"))
                .as_ptr(),
        );
    };
    zfree(rax as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn raxFree(mut rax: *mut rax) {
    raxFreeWithCallback(rax, None);
}
#[no_mangle]
pub unsafe extern "C" fn raxStart(mut it: *mut raxIterator, mut rt: *mut rax) {
    (*it).flags = (1 as libc::c_int) << 1 as libc::c_int;
    (*it).rt = rt;
    (*it).key_len = 0 as libc::c_int as size_t;
    (*it).key = ((*it).key_static_string).as_mut_ptr();
    (*it).key_max = 128 as libc::c_int as size_t;
    (*it).data = 0 as *mut libc::c_void;
    (*it).node_cb = None;
    raxStackInit(&mut (*it).stack);
}
#[no_mangle]
pub unsafe extern "C" fn raxIteratorAddChars(
    mut it: *mut raxIterator,
    mut s: *mut libc::c_uchar,
    mut len: size_t,
) -> libc::c_int {
    if len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if (*it).key_max < ((*it).key_len).wrapping_add(len) {
        let mut old: *mut libc::c_uchar = if (*it).key
            == ((*it).key_static_string).as_mut_ptr()
        {
            0 as *mut libc::c_uchar
        } else {
            (*it).key
        };
        let mut new_max: size_t = ((*it).key_len)
            .wrapping_add(len)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong);
        (*it).key = zrealloc(old as *mut libc::c_void, new_max) as *mut libc::c_uchar;
        if ((*it).key).is_null() {
            (*it)
                .key = if old.is_null() {
                ((*it).key_static_string).as_mut_ptr()
            } else {
                old
            };
            *__errno_location() = 12 as libc::c_int;
            return 0 as libc::c_int;
        }
        if old.is_null() {
            memcpy(
                (*it).key as *mut libc::c_void,
                ((*it).key_static_string).as_mut_ptr() as *const libc::c_void,
                (*it).key_len,
            );
        }
        (*it).key_max = new_max;
    }
    memmove(
        ((*it).key).offset((*it).key_len as isize) as *mut libc::c_void,
        s as *const libc::c_void,
        len,
    );
    (*it)
        .key_len = ((*it).key_len as libc::c_ulong).wrapping_add(len) as size_t
        as size_t;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxIteratorDelChars(
    mut it: *mut raxIterator,
    mut count: size_t,
) {
    (*it)
        .key_len = ((*it).key_len as libc::c_ulong).wrapping_sub(count) as size_t
        as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn raxIteratorNextStep(
    mut it: *mut raxIterator,
    mut noup: libc::c_int,
) -> libc::c_int {
    if (*it).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return 1 as libc::c_int
    } else {
        if (*it).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            (*it).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            return 1 as libc::c_int;
        }
    }
    let mut orig_key_len: size_t = (*it).key_len;
    let mut orig_stack_items: size_t = (*it).stack.items;
    let mut orig_node: *mut raxNode = (*it).node;
    loop {
        let mut children: libc::c_int = if (*(*it).node).iscompr() as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            (*(*it).node).size() as libc::c_int
        };
        if noup == 0 && children != 0 {
            if raxStackPush(&mut (*it).stack, (*it).node as *mut libc::c_void) == 0 {
                return 0 as libc::c_int;
            }
            let mut cp: *mut *mut raxNode = ((*(*it).node).data)
                .as_mut_ptr()
                .offset((*(*it).node).size() as libc::c_int as isize)
                .offset(
                    ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*(*it).node).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
                ) as *mut *mut raxNode;
            if raxIteratorAddChars(
                it,
                ((*(*it).node).data).as_mut_ptr(),
                (if (*(*it).node).iscompr() as libc::c_int != 0 {
                    (*(*it).node).size() as libc::c_int
                } else {
                    1 as libc::c_int
                }) as size_t,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            memcpy(
                &mut (*it).node as *mut *mut raxNode as *mut libc::c_void,
                cp as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
            if ((*it).node_cb).is_some()
                && ((*it).node_cb).expect("non-null function pointer")(&mut (*it).node)
                    != 0
            {
                memcpy(
                    cp as *mut libc::c_void,
                    &mut (*it).node as *mut *mut raxNode as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
            }
            if (*(*it).node).iskey() != 0 {
                (*it).data = raxGetData((*it).node);
                return 1 as libc::c_int;
            }
        } else {
            loop {
                let mut old_noup: libc::c_int = noup;
                if noup == 0 && (*it).node == (*(*it).rt).head {
                    (*it).flags |= (1 as libc::c_int) << 1 as libc::c_int;
                    (*it).stack.items = orig_stack_items;
                    (*it).key_len = orig_key_len;
                    (*it).node = orig_node;
                    return 1 as libc::c_int;
                }
                let mut prevchild: libc::c_uchar = *((*it).key)
                    .offset(
                        ((*it).key_len).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                            as isize,
                    );
                if noup == 0 {
                    (*it).node = raxStackPop(&mut (*it).stack) as *mut raxNode;
                } else {
                    noup = 0 as libc::c_int;
                }
                let mut todel: libc::c_int = if (*(*it).node).iscompr() as libc::c_int
                    != 0
                {
                    (*(*it).node).size() as libc::c_int
                } else {
                    1 as libc::c_int
                };
                raxIteratorDelChars(it, todel as size_t);
                if !((*(*it).node).iscompr() == 0
                    && (*(*it).node).size() as libc::c_int
                        > (if old_noup != 0 {
                            0 as libc::c_int
                        } else {
                            1 as libc::c_int
                        }))
                {
                    continue;
                }
                let mut cp_0: *mut *mut raxNode = ((*(*it).node).data)
                    .as_mut_ptr()
                    .offset((*(*it).node).size() as libc::c_int as isize)
                    .offset(
                        ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*(*it).node).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
                    ) as *mut *mut raxNode;
                let mut i: libc::c_int = 0 as libc::c_int;
                while i < (*(*it).node).size() as libc::c_int {
                    if *((*(*it).node).data).as_mut_ptr().offset(i as isize)
                        as libc::c_int > prevchild as libc::c_int
                    {
                        break;
                    }
                    i += 1;
                    cp_0 = cp_0.offset(1);
                }
                if !(i != (*(*it).node).size() as libc::c_int) {
                    continue;
                }
                raxIteratorAddChars(
                    it,
                    ((*(*it).node).data).as_mut_ptr().offset(i as isize),
                    1 as libc::c_int as size_t,
                );
                if raxStackPush(&mut (*it).stack, (*it).node as *mut libc::c_void) == 0 {
                    return 0 as libc::c_int;
                }
                memcpy(
                    &mut (*it).node as *mut *mut raxNode as *mut libc::c_void,
                    cp_0 as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
                if ((*it).node_cb).is_some()
                    && ((*it).node_cb)
                        .expect("non-null function pointer")(&mut (*it).node) != 0
                {
                    memcpy(
                        cp_0 as *mut libc::c_void,
                        &mut (*it).node as *mut *mut raxNode as *const libc::c_void,
                        core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                    );
                }
                if (*(*it).node).iskey() != 0 {
                    (*it).data = raxGetData((*it).node);
                    return 1 as libc::c_int;
                }
                break;
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn raxSeekGreatest(mut it: *mut raxIterator) -> libc::c_int {
    while (*(*it).node).size() != 0 {
        if (*(*it).node).iscompr() != 0 {
            if raxIteratorAddChars(
                it,
                ((*(*it).node).data).as_mut_ptr(),
                (*(*it).node).size() as size_t,
            ) == 0
            {
                return 0 as libc::c_int;
            }
        } else if raxIteratorAddChars(
            it,
            ((*(*it).node).data)
                .as_mut_ptr()
                .offset((*(*it).node).size() as libc::c_int as isize)
                .offset(-(1 as libc::c_int as isize)),
            1 as libc::c_int as size_t,
        ) == 0
        {
            return 0 as libc::c_int
        }
        let mut cp: *mut *mut raxNode = ((*it).node as *mut libc::c_char)
            .offset(
                (core::mem::size_of::<raxNode>() as libc::c_ulong)
                    .wrapping_add((*(*it).node).size() as libc::c_ulong)
                    .wrapping_add(
                        (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(
                                (((*(*it).node).size() as libc::c_int + 4 as libc::c_int)
                                    as libc::c_ulong)
                                    .wrapping_rem(
                                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                    ),
                            )
                            & (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                    )
                    .wrapping_add(
                        (if (*(*it).node).iscompr() as libc::c_int != 0 {
                            core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                        } else {
                            (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                .wrapping_mul((*(*it).node).size() as libc::c_ulong)
                        }),
                    )
                    .wrapping_add(
                        (((*(*it).node).iskey() as libc::c_int != 0
                            && (*(*it).node).isnull() == 0) as libc::c_int
                            as libc::c_ulong)
                            .wrapping_mul(
                                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                            ),
                    ) as isize,
            )
            .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
            .offset(
                -((if (*(*it).node).iskey() as libc::c_int != 0
                    && (*(*it).node).isnull() == 0
                {
                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                } else {
                    0 as libc::c_int as libc::c_ulong
                }) as isize),
            ) as *mut *mut raxNode;
        if raxStackPush(&mut (*it).stack, (*it).node as *mut libc::c_void) == 0 {
            return 0 as libc::c_int;
        }
        memcpy(
            &mut (*it).node as *mut *mut raxNode as *mut libc::c_void,
            cp as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxIteratorPrevStep(
    mut it: *mut raxIterator,
    mut noup: libc::c_int,
) -> libc::c_int {
    if (*it).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return 1 as libc::c_int
    } else {
        if (*it).flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
            (*it).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            return 1 as libc::c_int;
        }
    }
    let mut orig_key_len: size_t = (*it).key_len;
    let mut orig_stack_items: size_t = (*it).stack.items;
    let mut orig_node: *mut raxNode = (*it).node;
    loop {
        let mut old_noup: libc::c_int = noup;
        if noup == 0 && (*it).node == (*(*it).rt).head {
            (*it).flags |= (1 as libc::c_int) << 1 as libc::c_int;
            (*it).stack.items = orig_stack_items;
            (*it).key_len = orig_key_len;
            (*it).node = orig_node;
            return 1 as libc::c_int;
        }
        let mut prevchild: libc::c_uchar = *((*it).key)
            .offset(
                ((*it).key_len).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            );
        if noup == 0 {
            (*it).node = raxStackPop(&mut (*it).stack) as *mut raxNode;
        } else {
            noup = 0 as libc::c_int;
        }
        let mut todel: libc::c_int = if (*(*it).node).iscompr() as libc::c_int != 0 {
            (*(*it).node).size() as libc::c_int
        } else {
            1 as libc::c_int
        };
        raxIteratorDelChars(it, todel as size_t);
        if (*(*it).node).iscompr() == 0
            && (*(*it).node).size() as libc::c_int
                > (if old_noup != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
        {
            let mut cp: *mut *mut raxNode = ((*it).node as *mut libc::c_char)
                .offset(
                    (core::mem::size_of::<raxNode>() as libc::c_ulong)
                        .wrapping_add((*(*it).node).size() as libc::c_ulong)
                        .wrapping_add(
                            (core::mem::size_of::<*mut libc::c_void>()
                                as libc::c_ulong)
                                .wrapping_sub(
                                    (((*(*it).node).size() as libc::c_int + 4 as libc::c_int)
                                        as libc::c_ulong)
                                        .wrapping_rem(
                                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                        ),
                                )
                                & (core::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        )
                        .wrapping_add(
                            (if (*(*it).node).iscompr() as libc::c_int != 0 {
                                core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                            } else {
                                (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                                    .wrapping_mul((*(*it).node).size() as libc::c_ulong)
                            }),
                        )
                        .wrapping_add(
                            (((*(*it).node).iskey() as libc::c_int != 0
                                && (*(*it).node).isnull() == 0) as libc::c_int
                                as libc::c_ulong)
                                .wrapping_mul(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        ) as isize,
                )
                .offset(
                    -(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize),
                )
                .offset(
                    -((if (*(*it).node).iskey() as libc::c_int != 0
                        && (*(*it).node).isnull() == 0
                    {
                        core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
                    } else {
                        0 as libc::c_int as libc::c_ulong
                    }) as isize),
                ) as *mut *mut raxNode;
            let mut i: libc::c_int = (*(*it).node).size() as libc::c_int
                - 1 as libc::c_int;
            while i >= 0 as libc::c_int {
                if (*((*(*it).node).data).as_mut_ptr().offset(i as isize) as libc::c_int)
                    < prevchild as libc::c_int
                {
                    break;
                }
                i -= 1;
                cp = cp.offset(-1);
            }
            if i != -(1 as libc::c_int) {
                if raxIteratorAddChars(
                    it,
                    ((*(*it).node).data).as_mut_ptr().offset(i as isize),
                    1 as libc::c_int as size_t,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
                if raxStackPush(&mut (*it).stack, (*it).node as *mut libc::c_void) == 0 {
                    return 0 as libc::c_int;
                }
                memcpy(
                    &mut (*it).node as *mut *mut raxNode as *mut libc::c_void,
                    cp as *const libc::c_void,
                    core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
                );
                if raxSeekGreatest(it) == 0 {
                    return 0 as libc::c_int;
                }
            }
        }
        if (*(*it).node).iskey() != 0 {
            (*it).data = raxGetData((*it).node);
            return 1 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn raxSeek(
    mut it: *mut raxIterator,
    mut op: *const libc::c_char,
    mut ele: *mut libc::c_uchar,
    mut len: size_t,
) -> libc::c_int {
    let mut eq: libc::c_int = 0 as libc::c_int;
    let mut lt: libc::c_int = 0 as libc::c_int;
    let mut gt: libc::c_int = 0 as libc::c_int;
    let mut first: libc::c_int = 0 as libc::c_int;
    let mut last: libc::c_int = 0 as libc::c_int;
    (*it).stack.items = 0 as libc::c_int as size_t;
    (*it).flags |= (1 as libc::c_int) << 0 as libc::c_int;
    (*it).flags &= !((1 as libc::c_int) << 1 as libc::c_int);
    (*it).key_len = 0 as libc::c_int as size_t;
    (*it).node = 0 as *mut raxNode;
    if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
        gt = 1 as libc::c_int;
        if *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            eq = 1 as libc::c_int;
        }
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        lt = 1 as libc::c_int;
        if *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32 {
            eq = 1 as libc::c_int;
        }
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32 {
        eq = 1 as libc::c_int;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32 {
        first = 1 as libc::c_int;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '$' as i32 {
        last = 1 as libc::c_int;
    } else {
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*(*it).rt).numele == 0 as libc::c_int as libc::c_ulong {
        (*it).flags |= (1 as libc::c_int) << 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    if first != 0 {
        return raxSeek(
            it,
            b">=\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
    }
    if last != 0 {
        (*it).node = (*(*it).rt).head;
        if raxSeekGreatest(it) == 0 {
            return 0 as libc::c_int;
        }
        if (*(*it).node).iskey() as libc::c_int != 0 {} else {
            __assert_fail(
                b"it->node->iskey\0" as *const u8 as *const libc::c_char,
                b"rax.c\0" as *const u8 as *const libc::c_char,
                1556 as libc::c_int as libc::c_uint,
                (*core::mem::transmute::<
                    &[u8; 66],
                    &[libc::c_char; 66],
                >(
                    b"int raxSeek(raxIterator *, const char *, unsigned char *, size_t)\0",
                ))
                    .as_ptr(),
            );
        };
        (*it).data = raxGetData((*it).node);
        return 1 as libc::c_int;
    }
    let mut splitpos: libc::c_int = 0 as libc::c_int;
    let mut i: size_t = raxLowWalk(
        (*it).rt,
        ele,
        len,
        &mut (*it).node,
        0 as *mut *mut *mut raxNode,
        &mut splitpos,
        &mut (*it).stack,
    );
    if (*it).stack.oom != 0 {
        return 0 as libc::c_int;
    }
    if eq != 0 && i == len
        && ((*(*it).node).iscompr() == 0 || splitpos == 0 as libc::c_int)
        && (*(*it).node).iskey() as libc::c_int != 0
    {
        if raxIteratorAddChars(it, ele, len) == 0 {
            return 0 as libc::c_int;
        }
        (*it).data = raxGetData((*it).node);
    } else if lt != 0 || gt != 0 {
        raxIteratorAddChars(it, ele, i.wrapping_sub(splitpos as libc::c_ulong));
        if i != len && (*(*it).node).iscompr() == 0 {
            if raxIteratorAddChars(
                it,
                ele.offset(i as isize),
                1 as libc::c_int as size_t,
            ) == 0
            {
                return 0 as libc::c_int;
            }
            (*it).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            if lt != 0 && raxIteratorPrevStep(it, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            if gt != 0 && raxIteratorNextStep(it, 1 as libc::c_int) == 0 {
                return 0 as libc::c_int;
            }
            (*it).flags |= (1 as libc::c_int) << 0 as libc::c_int;
        } else if i != len && (*(*it).node).iscompr() as libc::c_int != 0 {
            let mut nodechar: libc::c_int = *((*(*it).node).data)
                .as_mut_ptr()
                .offset(splitpos as isize) as libc::c_int;
            let mut keychar: libc::c_int = *ele.offset(i as isize) as libc::c_int;
            (*it).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            if gt != 0 {
                if nodechar > keychar {
                    if raxIteratorNextStep(it, 0 as libc::c_int) == 0 {
                        return 0 as libc::c_int;
                    }
                } else {
                    if raxIteratorAddChars(
                        it,
                        ((*(*it).node).data).as_mut_ptr(),
                        (*(*it).node).size() as size_t,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    if raxIteratorNextStep(it, 1 as libc::c_int) == 0 {
                        return 0 as libc::c_int;
                    }
                }
            }
            if lt != 0 {
                if nodechar < keychar {
                    if raxSeekGreatest(it) == 0 {
                        return 0 as libc::c_int;
                    }
                    (*it).data = raxGetData((*it).node);
                } else {
                    if raxIteratorAddChars(
                        it,
                        ((*(*it).node).data).as_mut_ptr(),
                        (*(*it).node).size() as size_t,
                    ) == 0
                    {
                        return 0 as libc::c_int;
                    }
                    if raxIteratorPrevStep(it, 1 as libc::c_int) == 0 {
                        return 0 as libc::c_int;
                    }
                }
            }
            (*it).flags |= (1 as libc::c_int) << 0 as libc::c_int;
        } else {
            (*it).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
            if (*(*it).node).iscompr() as libc::c_int != 0
                && (*(*it).node).iskey() as libc::c_int != 0 && splitpos != 0 && lt != 0
            {
                (*it).data = raxGetData((*it).node);
            } else {
                if gt != 0 && raxIteratorNextStep(it, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
                if lt != 0 && raxIteratorPrevStep(it, 0 as libc::c_int) == 0 {
                    return 0 as libc::c_int;
                }
            }
            (*it).flags |= (1 as libc::c_int) << 0 as libc::c_int;
        }
    } else {
        (*it).flags |= (1 as libc::c_int) << 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxNext(mut it: *mut raxIterator) -> libc::c_int {
    if raxIteratorNextStep(it, 0 as libc::c_int) == 0 {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*it).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxPrev(mut it: *mut raxIterator) -> libc::c_int {
    if raxIteratorPrevStep(it, 0 as libc::c_int) == 0 {
        *__errno_location() = 12 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*it).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        *__errno_location() = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxRandomWalk(
    mut it: *mut raxIterator,
    mut steps: size_t,
) -> libc::c_int {
    if (*(*it).rt).numele == 0 as libc::c_int as libc::c_ulong {
        (*it).flags |= (1 as libc::c_int) << 1 as libc::c_int;
        return 0 as libc::c_int;
    }
    if steps == 0 as libc::c_int as libc::c_ulong {
        let mut fle: size_t = (1 as libc::c_int as libc::c_double
            + floor(log((*(*it).rt).numele as libc::c_double))) as size_t;
        fle = (fle as libc::c_ulong).wrapping_mul(2 as libc::c_int as libc::c_ulong)
            as size_t as size_t;
        steps = (1 as libc::c_int as libc::c_ulong)
            .wrapping_add((rand() as libc::c_ulong).wrapping_rem(fle));
    }
    let mut n: *mut raxNode = (*it).node;
    while steps > 0 as libc::c_int as libc::c_ulong || (*n).iskey() == 0 {
        let mut numchildren: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            (*n).size() as libc::c_int
        };
        let mut r: libc::c_int = rand()
            % (numchildren + (n != (*(*it).rt).head) as libc::c_int);
        if r == numchildren {
            n = raxStackPop(&mut (*it).stack) as *mut raxNode;
            let mut todel: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
                (*n).size() as libc::c_int
            } else {
                1 as libc::c_int
            };
            raxIteratorDelChars(it, todel as size_t);
        } else {
            if (*n).iscompr() != 0 {
                if raxIteratorAddChars(
                    it,
                    ((*n).data).as_mut_ptr(),
                    (*n).size() as size_t,
                ) == 0
                {
                    return 0 as libc::c_int;
                }
            } else if raxIteratorAddChars(
                it,
                ((*n).data).as_mut_ptr().offset(r as isize),
                1 as libc::c_int as size_t,
            ) == 0
            {
                return 0 as libc::c_int
            }
            let mut cp: *mut *mut raxNode = (((*n).data)
                .as_mut_ptr()
                .offset((*n).size() as libc::c_int as isize)
                .offset(
                    ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*n).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
                ) as *mut *mut raxNode)
                .offset(r as isize);
            if raxStackPush(&mut (*it).stack, n as *mut libc::c_void) == 0 {
                return 0 as libc::c_int;
            }
            memcpy(
                &mut n as *mut *mut raxNode as *mut libc::c_void,
                cp as *const libc::c_void,
                core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
            );
        }
        if (*n).iskey() != 0 {
            steps = steps.wrapping_sub(1);
        }
    }
    (*it).node = n;
    (*it).data = raxGetData((*it).node);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxCompare(
    mut iter: *mut raxIterator,
    mut op: *const libc::c_char,
    mut key: *mut libc::c_uchar,
    mut key_len: size_t,
) -> libc::c_int {
    let mut eq: libc::c_int = 0 as libc::c_int;
    let mut lt: libc::c_int = 0 as libc::c_int;
    let mut gt: libc::c_int = 0 as libc::c_int;
    if *op.offset(0 as libc::c_int as isize) as libc::c_int == '=' as i32
        || *op.offset(1 as libc::c_int as isize) as libc::c_int == '=' as i32
    {
        eq = 1 as libc::c_int;
    }
    if *op.offset(0 as libc::c_int as isize) as libc::c_int == '>' as i32 {
        gt = 1 as libc::c_int;
    } else if *op.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        lt = 1 as libc::c_int;
    } else if *op.offset(1 as libc::c_int as isize) as libc::c_int != '=' as i32 {
        return 0 as libc::c_int
    }
    let mut minlen: size_t = if key_len < (*iter).key_len {
        key_len
    } else {
        (*iter).key_len
    };
    let mut cmp: libc::c_int = memcmp(
        (*iter).key as *const libc::c_void,
        key as *const libc::c_void,
        minlen,
    );
    if lt == 0 as libc::c_int && gt == 0 as libc::c_int {
        return (cmp == 0 as libc::c_int && key_len == (*iter).key_len) as libc::c_int;
    }
    if cmp == 0 as libc::c_int {
        if eq != 0 && key_len == (*iter).key_len {
            return 1 as libc::c_int
        } else if lt != 0 {
            return ((*iter).key_len < key_len) as libc::c_int
        } else if gt != 0 {
            return ((*iter).key_len > key_len) as libc::c_int
        } else {
            return 0 as libc::c_int
        }
    } else if cmp > 0 as libc::c_int {
        return if gt != 0 { 1 as libc::c_int } else { 0 as libc::c_int }
    } else {
        return if lt != 0 { 1 as libc::c_int } else { 0 as libc::c_int }
    };
}
#[no_mangle]
pub unsafe extern "C" fn raxStop(mut it: *mut raxIterator) {
    if (*it).key != ((*it).key_static_string).as_mut_ptr() {
        zfree((*it).key as *mut libc::c_void);
    }
    raxStackFree(&mut (*it).stack);
}
#[no_mangle]
pub unsafe extern "C" fn raxEOF(mut it: *mut raxIterator) -> libc::c_int {
    return (*it).flags & (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn raxSize(mut rax: *mut rax) -> uint64_t {
    return (*rax).numele;
}
#[no_mangle]
pub unsafe extern "C" fn raxRecursiveShow(
    mut level: libc::c_int,
    mut lpad: libc::c_int,
    mut n: *mut raxNode,
) {
    let mut s: libc::c_char = (if (*n).iscompr() as libc::c_int != 0 {
        '"' as i32
    } else {
        '[' as i32
    }) as libc::c_char;
    let mut e: libc::c_char = (if (*n).iscompr() as libc::c_int != 0 {
        '"' as i32
    } else {
        ']' as i32
    }) as libc::c_char;
    let mut numchars: libc::c_int = printf(
        b"%c%.*s%c\0" as *const u8 as *const libc::c_char,
        s as libc::c_int,
        (*n).size() as libc::c_int,
        ((*n).data).as_mut_ptr(),
        e as libc::c_int,
    );
    if (*n).iskey() != 0 {
        numchars += printf(b"=%p\0" as *const u8 as *const libc::c_char, raxGetData(n));
    }
    let mut numchildren: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        (*n).size() as libc::c_int
    };
    if level != 0 {
        lpad
            += if numchildren > 1 as libc::c_int {
                7 as libc::c_int
            } else {
                4 as libc::c_int
            };
        if numchildren == 1 as libc::c_int {
            lpad += numchars;
        }
    }
    let mut cp: *mut *mut raxNode = ((*n).data)
        .as_mut_ptr()
        .offset((*n).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut *mut raxNode;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numchildren {
        let mut branch: *mut libc::c_char = b" `-(%c) \0" as *const u8
            as *const libc::c_char as *mut libc::c_char;
        if numchildren > 1 as libc::c_int {
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            let mut j: libc::c_int = 0 as libc::c_int;
            while j < lpad {
                putchar(' ' as i32);
                j += 1;
            }
            printf(branch, *((*n).data).as_mut_ptr().offset(i as isize) as libc::c_int);
        } else {
            printf(b" -> \0" as *const u8 as *const libc::c_char);
        }
        let mut child: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut child as *mut *mut raxNode as *mut libc::c_void,
            cp as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        raxRecursiveShow(level + 1 as libc::c_int, lpad, child);
        cp = cp.offset(1);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn raxShow(mut rax: *mut rax) {
    raxRecursiveShow(0 as libc::c_int, 0 as libc::c_int, (*rax).head);
    putchar('\n' as i32);
}
#[no_mangle]
pub unsafe extern "C" fn raxDebugShowNode(
    mut msg: *const libc::c_char,
    mut n: *mut raxNode,
) {
    if raxDebugMsg == 0 as libc::c_int {
        return;
    }
    printf(
        b"%s: %p [%.*s] key:%u size:%u children:\0" as *const u8 as *const libc::c_char,
        msg,
        n as *mut libc::c_void,
        (*n).size() as libc::c_int,
        ((*n).data).as_mut_ptr() as *mut libc::c_char,
        (*n).iskey() as libc::c_int,
        (*n).size() as libc::c_int,
    );
    let mut numcld: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        (*n).size() as libc::c_int
    };
    let mut cldptr: *mut *mut raxNode = ((n as *mut libc::c_char)
        .offset(
            (core::mem::size_of::<raxNode>() as libc::c_ulong)
                .wrapping_add((*n).size() as libc::c_ulong)
                .wrapping_add(
                    (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                        .wrapping_sub(
                            (((*n).size() as libc::c_int + 4 as libc::c_int)
                                as libc::c_ulong)
                                .wrapping_rem(
                                    core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                                ),
                        )
                        & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_add(
                    (if (*n).iscompr() as libc::c_int != 0 {
                        core::mem::size_of::<*mut raxNode>() as libc::c_ulong
                    } else {
                        (core::mem::size_of::<*mut raxNode>() as libc::c_ulong)
                            .wrapping_mul((*n).size() as libc::c_ulong)
                    }),
                )
                .wrapping_add(
                    (((*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0)
                        as libc::c_int as libc::c_ulong)
                        .wrapping_mul(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                ) as isize,
        )
        .offset(-(core::mem::size_of::<*mut raxNode>() as libc::c_ulong as isize))
        .offset(
            -((if (*n).iskey() as libc::c_int != 0 && (*n).isnull() == 0 {
                core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            } else {
                0 as libc::c_int as libc::c_ulong
            }) as isize),
        ) as *mut *mut raxNode)
        .offset(-((numcld - 1 as libc::c_int) as isize));
    loop {
        let fresh2 = numcld;
        numcld = numcld - 1;
        if !(fresh2 != 0) {
            break;
        }
        let mut child: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut child as *mut *mut raxNode as *mut libc::c_void,
            cldptr as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        cldptr = cldptr.offset(1);
        printf(b"%p \0" as *const u8 as *const libc::c_char, child as *mut libc::c_void);
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn raxTouch(mut n: *mut raxNode) -> libc::c_ulong {
    let mut sum: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    if (*n).iskey() != 0 {
        sum = sum.wrapping_add(raxGetData(n) as libc::c_ulong);
    }
    let mut numchildren: libc::c_int = if (*n).iscompr() as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        (*n).size() as libc::c_int
    };
    let mut cp: *mut *mut raxNode = ((*n).data)
        .as_mut_ptr()
        .offset((*n).size() as libc::c_int as isize)
        .offset(
            ((core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_sub(
                    (((*n).size() as libc::c_int + 4 as libc::c_int) as libc::c_ulong)
                        .wrapping_rem(
                            core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong,
                        ),
                )
                & (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)) as isize,
        ) as *mut *mut raxNode;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numchildren {
        if numchildren > 1 as libc::c_int {
            sum = sum
                .wrapping_add(
                    *((*n).data).as_mut_ptr().offset(i as isize) as libc::c_long
                        as libc::c_ulong,
                );
        }
        let mut child: *mut raxNode = 0 as *mut raxNode;
        memcpy(
            &mut child as *mut *mut raxNode as *mut libc::c_void,
            cp as *const libc::c_void,
            core::mem::size_of::<*mut raxNode>() as libc::c_ulong,
        );
        if child == 0x65d1760 as libc::c_int as *mut libc::c_void as *mut raxNode {
            count += 1;
        }
        if count > 1 as libc::c_int {
            exit(1 as libc::c_int);
        }
        sum = sum.wrapping_add(raxTouch(child));
        cp = cp.offset(1);
        i += 1;
    }
    return sum;
}

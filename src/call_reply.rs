extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn parseReply(parser: *mut ReplyParser, p_ctx: *mut libc::c_void) -> libc::c_int;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type size_t = libc::c_ulong;
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
pub struct ReplyParser {
    pub curr_location: *const libc::c_char,
    pub callbacks: ReplyParserCallbacks,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ReplyParserCallbacks {
    pub null_array_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub null_bulk_string_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub bulk_string_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub error_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub simple_str_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub long_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_longlong,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub array_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub set_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub map_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub bool_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_int,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub double_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            libc::c_double,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub big_number_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub verbatim_string_callback: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub attribute_callback: Option::<
        unsafe extern "C" fn(
            *mut ReplyParser,
            *mut libc::c_void,
            size_t,
            *const libc::c_char,
        ) -> (),
    >,
    pub null_callback: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, size_t) -> (),
    >,
    pub error: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CallReply {
    pub private_data: *mut libc::c_void,
    pub original_proto: sds,
    pub proto: *const libc::c_char,
    pub proto_len: size_t,
    pub type_0: libc::c_int,
    pub flags: libc::c_int,
    pub len: size_t,
    pub val: C2RustUnnamed,
    pub deferred_error_list: *mut list,
    pub attribute: *mut CallReply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub str_0: *const libc::c_char,
    pub verbatim_str: C2RustUnnamed_0,
    pub ll: libc::c_longlong,
    pub d: libc::c_double,
    pub array: *mut CallReply,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub str_0: *const libc::c_char,
    pub format: *const libc::c_char,
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
unsafe extern "C" fn callReplySetSharedData(
    mut rep: *mut CallReply,
    mut type_0: libc::c_int,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
    mut extra_flags: libc::c_int,
) {
    (*rep).type_0 = type_0;
    (*rep).proto = proto;
    (*rep).proto_len = proto_len;
    (*rep).flags |= extra_flags;
}
unsafe extern "C" fn callReplyNull(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(
        rep,
        4 as libc::c_int,
        proto,
        proto_len,
        (1 as libc::c_int) << 2 as libc::c_int,
    );
}
unsafe extern "C" fn callReplyNullBulkString(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 4 as libc::c_int, proto, proto_len, 0 as libc::c_int);
}
unsafe extern "C" fn callReplyNullArray(
    mut ctx: *mut libc::c_void,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 4 as libc::c_int, proto, proto_len, 0 as libc::c_int);
}
unsafe extern "C" fn callReplyBulkString(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 0 as libc::c_int, proto, proto_len, 0 as libc::c_int);
    (*rep).len = len;
    (*rep).val.str_0 = str;
}
unsafe extern "C" fn callReplyError(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 1 as libc::c_int, proto, proto_len, 0 as libc::c_int);
    (*rep).len = len;
    (*rep).val.str_0 = str;
}
unsafe extern "C" fn callReplySimpleStr(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 0 as libc::c_int, proto, proto_len, 0 as libc::c_int);
    (*rep).len = len;
    (*rep).val.str_0 = str;
}
unsafe extern "C" fn callReplyLong(
    mut ctx: *mut libc::c_void,
    mut val: libc::c_longlong,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(rep, 2 as libc::c_int, proto, proto_len, 0 as libc::c_int);
    (*rep).val.ll = val;
}
unsafe extern "C" fn callReplyDouble(
    mut ctx: *mut libc::c_void,
    mut val: libc::c_double,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(
        rep,
        8 as libc::c_int,
        proto,
        proto_len,
        (1 as libc::c_int) << 2 as libc::c_int,
    );
    (*rep).val.d = val;
}
unsafe extern "C" fn callReplyVerbatimString(
    mut ctx: *mut libc::c_void,
    mut format: *const libc::c_char,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(
        rep,
        10 as libc::c_int,
        proto,
        proto_len,
        (1 as libc::c_int) << 2 as libc::c_int,
    );
    (*rep).len = len;
    (*rep).val.verbatim_str.str_0 = str;
    (*rep).val.verbatim_str.format = format;
}
unsafe extern "C" fn callReplyBigNumber(
    mut ctx: *mut libc::c_void,
    mut str: *const libc::c_char,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(
        rep,
        9 as libc::c_int,
        proto,
        proto_len,
        (1 as libc::c_int) << 2 as libc::c_int,
    );
    (*rep).len = len;
    (*rep).val.str_0 = str;
}
unsafe extern "C" fn callReplyBool(
    mut ctx: *mut libc::c_void,
    mut val: libc::c_int,
    mut proto: *const libc::c_char,
    mut proto_len: size_t,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    callReplySetSharedData(
        rep,
        7 as libc::c_int,
        proto,
        proto_len,
        (1 as libc::c_int) << 2 as libc::c_int,
    );
    (*rep).val.ll = val as libc::c_longlong;
}
unsafe extern "C" fn callReplyParseCollection(
    mut parser: *mut ReplyParser,
    mut rep: *mut CallReply,
    mut len: size_t,
    mut proto: *const libc::c_char,
    mut elements_per_entry: size_t,
) {
    (*rep).len = len;
    (*rep)
        .val
        .array = zcalloc(
        elements_per_entry
            .wrapping_mul(len)
            .wrapping_mul(core::mem::size_of::<CallReply>() as libc::c_ulong),
    ) as *mut CallReply;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < len.wrapping_mul(elements_per_entry) {
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < elements_per_entry {
            let ref mut fresh0 = (*((*rep).val.array).offset(i.wrapping_add(j) as isize))
                .private_data;
            *fresh0 = (*rep).private_data;
            parseReply(
                parser,
                ((*rep).val.array).offset(i as isize).offset(j as isize)
                    as *mut libc::c_void,
            );
            (*((*rep).val.array).offset(i.wrapping_add(j) as isize)).flags
                |= (1 as libc::c_int) << 1 as libc::c_int;
            if (*((*rep).val.array).offset(i.wrapping_add(j) as isize)).flags
                & (1 as libc::c_int) << 2 as libc::c_int != 0
            {
                (*rep).flags |= (1 as libc::c_int) << 2 as libc::c_int;
            }
            j = j.wrapping_add(1);
        }
        i = (i as libc::c_ulong).wrapping_add(elements_per_entry) as size_t as size_t;
    }
    (*rep).proto = proto;
    (*rep)
        .proto_len = ((*parser).curr_location).offset_from(proto) as libc::c_long
        as size_t;
}
unsafe extern "C" fn callReplyAttribute(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    (*rep)
        .attribute = zcalloc(core::mem::size_of::<CallReply>() as libc::c_ulong)
        as *mut CallReply;
    (*(*rep).attribute).len = len;
    (*(*rep).attribute).type_0 = 11 as libc::c_int;
    callReplyParseCollection(
        parser,
        (*rep).attribute,
        len,
        proto,
        2 as libc::c_int as size_t,
    );
    (*(*rep).attribute).flags
        |= (1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int;
    (*(*rep).attribute).private_data = (*rep).private_data;
    parseReply(parser, rep as *mut libc::c_void);
    (*rep).proto = proto;
    (*rep)
        .proto_len = ((*parser).curr_location).offset_from(proto) as libc::c_long
        as size_t;
    (*rep).flags |= (1 as libc::c_int) << 2 as libc::c_int;
}
unsafe extern "C" fn callReplyArray(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    (*rep).type_0 = 3 as libc::c_int;
    callReplyParseCollection(parser, rep, len, proto, 1 as libc::c_int as size_t);
}
unsafe extern "C" fn callReplySet(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    (*rep).type_0 = 6 as libc::c_int;
    callReplyParseCollection(parser, rep, len, proto, 1 as libc::c_int as size_t);
    (*rep).flags |= (1 as libc::c_int) << 2 as libc::c_int;
}
unsafe extern "C" fn callReplyMap(
    mut parser: *mut ReplyParser,
    mut ctx: *mut libc::c_void,
    mut len: size_t,
    mut proto: *const libc::c_char,
) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    (*rep).type_0 = 5 as libc::c_int;
    callReplyParseCollection(parser, rep, len, proto, 2 as libc::c_int as size_t);
    (*rep).flags |= (1 as libc::c_int) << 2 as libc::c_int;
}
unsafe extern "C" fn callReplyParseError(mut ctx: *mut libc::c_void) {
    let mut rep: *mut CallReply = ctx as *mut CallReply;
    (*rep).type_0 = -(1 as libc::c_int);
}
unsafe extern "C" fn freeCallReplyInternal(mut rep: *mut CallReply) {
    if (*rep).type_0 == 3 as libc::c_int || (*rep).type_0 == 6 as libc::c_int {
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < (*rep).len {
            freeCallReplyInternal(((*rep).val.array).offset(i as isize));
            i = i.wrapping_add(1);
        }
        zfree((*rep).val.array as *mut libc::c_void);
    }
    if (*rep).type_0 == 5 as libc::c_int || (*rep).type_0 == 11 as libc::c_int {
        let mut i_0: size_t = 0 as libc::c_int as size_t;
        while i_0 < (*rep).len {
            freeCallReplyInternal(
                ((*rep).val.array)
                    .offset(i_0.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize),
            );
            freeCallReplyInternal(
                ((*rep).val.array)
                    .offset(i_0.wrapping_mul(2 as libc::c_int as libc::c_ulong) as isize)
                    .offset(1 as libc::c_int as isize),
            );
            i_0 = i_0.wrapping_add(1);
        }
        zfree((*rep).val.array as *mut libc::c_void);
    }
    if !((*rep).attribute).is_null() {
        freeCallReplyInternal((*rep).attribute);
        zfree((*rep).attribute as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeCallReply(mut rep: *mut CallReply) {
    if (*rep).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return;
    }
    if (*rep).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        freeCallReplyInternal(rep);
    }
    sdsfree((*rep).original_proto);
    if !((*rep).deferred_error_list).is_null() {
        listRelease((*rep).deferred_error_list);
    }
    zfree(rep as *mut libc::c_void);
}
static mut DefaultParserCallbacks: ReplyParserCallbacks = unsafe {
    {
        let mut init = ReplyParserCallbacks {
            null_array_callback: Some(
                callReplyNullArray
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            null_bulk_string_callback: Some(
                callReplyNullBulkString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            bulk_string_callback: Some(
                callReplyBulkString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            error_callback: Some(
                callReplyError
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            simple_str_callback: Some(
                callReplySimpleStr
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            long_callback: Some(
                callReplyLong
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_longlong,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            array_callback: Some(
                callReplyArray
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            set_callback: Some(
                callReplySet
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            map_callback: Some(
                callReplyMap
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            bool_callback: Some(
                callReplyBool
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_int,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            double_callback: Some(
                callReplyDouble
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        libc::c_double,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            big_number_callback: Some(
                callReplyBigNumber
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            verbatim_string_callback: Some(
                callReplyVerbatimString
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            attribute_callback: Some(
                callReplyAttribute
                    as unsafe extern "C" fn(
                        *mut ReplyParser,
                        *mut libc::c_void,
                        size_t,
                        *const libc::c_char,
                    ) -> (),
            ),
            null_callback: Some(
                callReplyNull
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            error: Some(
                callReplyParseError as unsafe extern "C" fn(*mut libc::c_void) -> (),
            ),
        };
        init
    }
};
unsafe extern "C" fn callReplyParse(mut rep: *mut CallReply) {
    if (*rep).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return;
    }
    let mut parser: ReplyParser = {
        let mut init = ReplyParser {
            curr_location: (*rep).proto,
            callbacks: DefaultParserCallbacks,
        };
        init
    };
    parseReply(&mut parser, rep as *mut libc::c_void);
    (*rep).flags |= (1 as libc::c_int) << 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyType(mut rep: *mut CallReply) -> libc::c_int {
    if rep.is_null() {
        return -(1 as libc::c_int);
    }
    callReplyParse(rep);
    return (*rep).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetString(
    mut rep: *mut CallReply,
    mut len: *mut size_t,
) -> *const libc::c_char {
    callReplyParse(rep);
    if (*rep).type_0 != 0 as libc::c_int && (*rep).type_0 != 1 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if !len.is_null() {
        *len = (*rep).len;
    }
    return (*rep).val.str_0;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetLongLong(
    mut rep: *mut CallReply,
) -> libc::c_longlong {
    callReplyParse(rep);
    if (*rep).type_0 != 2 as libc::c_int {
        return -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong;
    }
    return (*rep).val.ll;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetDouble(mut rep: *mut CallReply) -> libc::c_double {
    callReplyParse(rep);
    if (*rep).type_0 != 8 as libc::c_int {
        return (-(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong)
            as libc::c_double;
    }
    return (*rep).val.d;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetBool(mut rep: *mut CallReply) -> libc::c_int {
    callReplyParse(rep);
    if (*rep).type_0 != 7 as libc::c_int {
        return -(2147483647 as libc::c_int) - 1 as libc::c_int;
    }
    return (*rep).val.ll as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetLen(mut rep: *mut CallReply) -> size_t {
    callReplyParse(rep);
    match (*rep).type_0 {
        0 | 1 | 3 | 6 | 5 | 11 => return (*rep).len,
        _ => return 0 as libc::c_int as size_t,
    };
}
unsafe extern "C" fn callReplyGetCollectionElement(
    mut rep: *mut CallReply,
    mut idx: size_t,
    mut elements_per_entry: libc::c_int,
) -> *mut CallReply {
    if idx >= ((*rep).len).wrapping_mul(elements_per_entry as libc::c_ulong) {
        return 0 as *mut CallReply;
    }
    return ((*rep).val.array).offset(idx as isize);
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetArrayElement(
    mut rep: *mut CallReply,
    mut idx: size_t,
) -> *mut CallReply {
    callReplyParse(rep);
    if (*rep).type_0 != 3 as libc::c_int {
        return 0 as *mut CallReply;
    }
    return callReplyGetCollectionElement(rep, idx, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetSetElement(
    mut rep: *mut CallReply,
    mut idx: size_t,
) -> *mut CallReply {
    callReplyParse(rep);
    if (*rep).type_0 != 6 as libc::c_int {
        return 0 as *mut CallReply;
    }
    return callReplyGetCollectionElement(rep, idx, 1 as libc::c_int);
}
unsafe extern "C" fn callReplyGetMapElementInternal(
    mut rep: *mut CallReply,
    mut idx: size_t,
    mut key: *mut *mut CallReply,
    mut val: *mut *mut CallReply,
    mut type_0: libc::c_int,
) -> libc::c_int {
    callReplyParse(rep);
    if (*rep).type_0 != type_0 {
        return -(1 as libc::c_int);
    }
    if idx >= (*rep).len {
        return -(1 as libc::c_int);
    }
    if !key.is_null() {
        *key = callReplyGetCollectionElement(
            rep,
            idx.wrapping_mul(2 as libc::c_int as libc::c_ulong),
            2 as libc::c_int,
        );
    }
    if !val.is_null() {
        *val = callReplyGetCollectionElement(
            rep,
            idx
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
            2 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetMapElement(
    mut rep: *mut CallReply,
    mut idx: size_t,
    mut key: *mut *mut CallReply,
    mut val: *mut *mut CallReply,
) -> libc::c_int {
    return callReplyGetMapElementInternal(rep, idx, key, val, 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetAttribute(
    mut rep: *mut CallReply,
) -> *mut CallReply {
    return (*rep).attribute;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetAttributeElement(
    mut rep: *mut CallReply,
    mut idx: size_t,
    mut key: *mut *mut CallReply,
    mut val: *mut *mut CallReply,
) -> libc::c_int {
    return callReplyGetMapElementInternal(rep, idx, key, val, 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetBigNumber(
    mut rep: *mut CallReply,
    mut len: *mut size_t,
) -> *const libc::c_char {
    callReplyParse(rep);
    if (*rep).type_0 != 9 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    *len = (*rep).len;
    return (*rep).val.str_0;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetVerbatim(
    mut rep: *mut CallReply,
    mut len: *mut size_t,
    mut format: *mut *const libc::c_char,
) -> *const libc::c_char {
    callReplyParse(rep);
    if (*rep).type_0 != 10 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    *len = (*rep).len;
    if !format.is_null() {
        *format = (*rep).val.verbatim_str.format;
    }
    return (*rep).val.verbatim_str.str_0;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetProto(
    mut rep: *mut CallReply,
    mut proto_len: *mut size_t,
) -> *const libc::c_char {
    *proto_len = (*rep).proto_len;
    return (*rep).proto;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyGetPrivateData(
    mut rep: *mut CallReply,
) -> *mut libc::c_void {
    return (*rep).private_data;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyIsResp3(mut rep: *mut CallReply) -> libc::c_int {
    return (*rep).flags & (1 as libc::c_int) << 2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyDeferredErrorList(
    mut rep: *mut CallReply,
) -> *mut list {
    return (*rep).deferred_error_list;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyCreate(
    mut reply: sds,
    mut deferred_error_list: *mut list,
    mut private_data: *mut libc::c_void,
) -> *mut CallReply {
    let mut res: *mut CallReply = zmalloc(
        core::mem::size_of::<CallReply>() as libc::c_ulong,
    ) as *mut CallReply;
    (*res).flags = (1 as libc::c_int) << 0 as libc::c_int;
    (*res).original_proto = reply;
    (*res).proto = reply as *const libc::c_char;
    (*res).proto_len = sdslen(reply);
    (*res).private_data = private_data;
    (*res).attribute = 0 as *mut CallReply;
    (*res).deferred_error_list = deferred_error_list;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn callReplyCreateError(
    mut reply: sds,
    mut private_data: *mut libc::c_void,
) -> *mut CallReply {
    let mut err_buff: sds = reply;
    if *err_buff.offset(0 as libc::c_int as isize) as libc::c_int != '-' as i32 {
        err_buff = sdscatfmt(
            sdsempty(),
            b"-ERR %S\r\n\0" as *const u8 as *const libc::c_char,
            reply,
        );
        sdsfree(reply);
    }
    let mut deferred_error_list: *mut list = listCreate();
    (*deferred_error_list)
        .free = core::mem::transmute::<
        Option::<unsafe extern "C" fn(sds) -> ()>,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    >(Some(sdsfree as unsafe extern "C" fn(sds) -> ()));
    listAddNodeTail(
        deferred_error_list,
        sdsnew(err_buff as *const libc::c_char) as *mut libc::c_void,
    );
    return callReplyCreate(err_buff, deferred_error_list, private_data);
}

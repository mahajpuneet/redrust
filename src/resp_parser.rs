extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
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
unsafe extern "C" fn parseBulk(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut bulklen: libc::c_longlong = 0;
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut bulklen,
    );
    if bulklen == -(1 as libc::c_int) as libc::c_longlong {
        ((*parser).callbacks.null_bulk_string_callback)
            .expect(
                "non-null function pointer",
            )(
            p_ctx,
            proto,
            ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
        );
    } else {
        let mut str: *const libc::c_char = (*parser).curr_location;
        (*parser).curr_location = ((*parser).curr_location).offset(bulklen as isize);
        (*parser)
            .curr_location = ((*parser).curr_location).offset(2 as libc::c_int as isize);
        ((*parser).callbacks.bulk_string_callback)
            .expect(
                "non-null function pointer",
            )(
            p_ctx,
            str,
            bulklen as size_t,
            proto,
            ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseSimpleString(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    ((*parser).callbacks.simple_str_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseError(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    ((*parser).callbacks.error_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseLong(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    let mut val: libc::c_longlong = 0;
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut val,
    );
    ((*parser).callbacks.long_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        val,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseAttributes(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut len: libc::c_longlong = 0;
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut len,
    );
    p = p.offset(2 as libc::c_int as isize);
    (*parser).curr_location = p;
    ((*parser).callbacks.attribute_callback)
        .expect("non-null function pointer")(parser, p_ctx, len as size_t, proto);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseVerbatimString(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut bulklen: libc::c_longlong = 0;
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut bulklen,
    );
    let mut format: *const libc::c_char = (*parser).curr_location;
    (*parser).curr_location = ((*parser).curr_location).offset(bulklen as isize);
    (*parser)
        .curr_location = ((*parser).curr_location).offset(2 as libc::c_int as isize);
    ((*parser).callbacks.verbatim_string_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        format,
        format.offset(4 as libc::c_int as isize),
        (bulklen - 4 as libc::c_int as libc::c_longlong) as size_t,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseBigNumber(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    ((*parser).callbacks.big_number_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseNull(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    ((*parser).callbacks.null_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseDouble(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    let mut buf: [libc::c_char; 5121] = [0; 5121];
    let mut len: size_t = (p.offset_from(proto) as libc::c_long
        - 1 as libc::c_int as libc::c_long) as size_t;
    let mut d: libc::c_double = 0.;
    if len <= (5 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong {
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            proto.offset(1 as libc::c_int as isize) as *const libc::c_void,
            len,
        );
        buf[len as usize] = '\0' as i32 as libc::c_char;
        d = strtod(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char);
    } else {
        d = 0 as libc::c_int as libc::c_double;
    }
    ((*parser).callbacks.double_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        d,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseBool(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    (*parser).curr_location = p.offset(2 as libc::c_int as isize);
    ((*parser).callbacks.bool_callback)
        .expect(
            "non-null function pointer",
        )(
        p_ctx,
        (*proto.offset(1 as libc::c_int as isize) as libc::c_int == 't' as i32)
            as libc::c_int,
        proto,
        ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseArray(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut len: libc::c_longlong = 0;
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut len,
    );
    p = p.offset(2 as libc::c_int as isize);
    (*parser).curr_location = p;
    if len == -(1 as libc::c_int) as libc::c_longlong {
        ((*parser).callbacks.null_array_callback)
            .expect(
                "non-null function pointer",
            )(
            p_ctx,
            proto,
            ((*parser).curr_location).offset_from(proto) as libc::c_long as size_t,
        );
    } else {
        ((*parser).callbacks.array_callback)
            .expect("non-null function pointer")(parser, p_ctx, len as size_t, proto);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseSet(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut len: libc::c_longlong = 0;
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut len,
    );
    p = p.offset(2 as libc::c_int as isize);
    (*parser).curr_location = p;
    ((*parser).callbacks.set_callback)
        .expect("non-null function pointer")(parser, p_ctx, len as size_t, proto);
    return 0 as libc::c_int;
}
unsafe extern "C" fn parseMap(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    let mut proto: *const libc::c_char = (*parser).curr_location;
    let mut p: *mut libc::c_char = strchr(
        proto.offset(1 as libc::c_int as isize),
        '\r' as i32,
    );
    let mut len: libc::c_longlong = 0;
    string2ll(
        proto.offset(1 as libc::c_int as isize),
        (p.offset_from(proto) as libc::c_long - 1 as libc::c_int as libc::c_long)
            as size_t,
        &mut len,
    );
    p = p.offset(2 as libc::c_int as isize);
    (*parser).curr_location = p;
    ((*parser).callbacks.map_callback)
        .expect("non-null function pointer")(parser, p_ctx, len as size_t, proto);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parseReply(
    mut parser: *mut ReplyParser,
    mut p_ctx: *mut libc::c_void,
) -> libc::c_int {
    match *((*parser).curr_location).offset(0 as libc::c_int as isize) as libc::c_int {
        36 => return parseBulk(parser, p_ctx),
        43 => return parseSimpleString(parser, p_ctx),
        45 => return parseError(parser, p_ctx),
        58 => return parseLong(parser, p_ctx),
        42 => return parseArray(parser, p_ctx),
        126 => return parseSet(parser, p_ctx),
        37 => return parseMap(parser, p_ctx),
        35 => return parseBool(parser, p_ctx),
        44 => return parseDouble(parser, p_ctx),
        95 => return parseNull(parser, p_ctx),
        40 => return parseBigNumber(parser, p_ctx),
        61 => return parseVerbatimString(parser, p_ctx),
        124 => return parseAttributes(parser, p_ctx),
        _ => {
            if ((*parser).callbacks.error).is_some() {
                ((*parser).callbacks.error).expect("non-null function pointer")(p_ctx);
            }
        }
    }
    return -(1 as libc::c_int);
}

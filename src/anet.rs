
extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getpeername(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn listen(__fd: libc::c_int, __n: libc::c_int) -> libc::c_int;
    fn accept4(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn inet_ntop(
        __af: libc::c_int,
        __cp: *const libc::c_void,
        __buf: *mut libc::c_char,
        __len: socklen_t,
    ) -> *const libc::c_char;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn pipe(__pipedes: *mut libc::c_int) -> libc::c_int;
    fn pipe2(__pipedes: *mut libc::c_int, __flags: libc::c_int) -> libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [libc::c_char; 118],
    pub __ss_align: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [libc::c_char; 108],
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
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2RustUnnamed_0 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_0 = 256;
pub const IPPROTO_RAW: C2RustUnnamed_0 = 255;
pub const IPPROTO_MPLS: C2RustUnnamed_0 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_0 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_0 = 132;
pub const IPPROTO_COMP: C2RustUnnamed_0 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_0 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_0 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_0 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_0 = 92;
pub const IPPROTO_AH: C2RustUnnamed_0 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_0 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_0 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_0 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_0 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_0 = 33;
pub const IPPROTO_TP: C2RustUnnamed_0 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_0 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_0 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_0 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_0 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_0 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_0 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_0 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_0 = 1;
pub const IPPROTO_IP: C2RustUnnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type va_list = __builtin_va_list;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
unsafe extern "C" fn anetSetError(
    mut err: *mut libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    if err.is_null() {
        return;
    }
    ap = args.clone();
    vsnprintf(err, 256 as libc::c_int as libc::c_ulong, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn anetSetBlock(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut non_block: libc::c_int,
) -> libc::c_int {
    let mut flags: libc::c_int = 0;
    flags = fcntl(fd, 3 as libc::c_int);
    if flags == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"fcntl(F_GETFL): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if (flags & 0o4000 as libc::c_int != 0) as libc::c_int
        == (non_block != 0) as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if non_block != 0 {
        flags |= 0o4000 as libc::c_int;
    } else {
        flags &= !(0o4000 as libc::c_int);
    }
    if fcntl(fd, 4 as libc::c_int, flags) == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"fcntl(F_SETFL,O_NONBLOCK): %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anetNonBlock(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    return anetSetBlock(err, fd, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetBlock(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    return anetSetBlock(err, fd, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetCloexec(mut fd: libc::c_int) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    loop {
        r = fcntl(fd, 1 as libc::c_int);
        if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if r == -(1 as libc::c_int) || r & 1 as libc::c_int != 0 {
        return r;
    }
    flags = r | 1 as libc::c_int;
    loop {
        r = fcntl(fd, 2 as libc::c_int, flags);
        if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn anetKeepAlive(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut interval: libc::c_int,
) -> libc::c_int {
    let mut val: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        9 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_KEEPALIVE: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    val = interval;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        4 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        anetSetError(
            err,
            b"setsockopt TCP_KEEPIDLE: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    val = interval / 3 as libc::c_int;
    if val == 0 as libc::c_int {
        val = 1 as libc::c_int;
    }
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        5 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        anetSetError(
            err,
            b"setsockopt TCP_KEEPINTVL: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    val = 3 as libc::c_int;
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        6 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) < 0 as libc::c_int
    {
        anetSetError(
            err,
            b"setsockopt TCP_KEEPCNT: %s\n\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetSetTcpNoDelay(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    if setsockopt(
        fd,
        IPPROTO_TCP as libc::c_int,
        1 as libc::c_int,
        &mut val as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt TCP_NODELAY: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anetEnableTcpNoDelay(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    return anetSetTcpNoDelay(err, fd, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetDisableTcpNoDelay(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    return anetSetTcpNoDelay(err, fd, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetSendTimeout(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut ms: libc::c_longlong,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = (ms / 1000 as libc::c_int as libc::c_longlong) as __time_t;
    tv
        .tv_usec = (ms % 1000 as libc::c_int as libc::c_longlong
        * 1000 as libc::c_int as libc::c_longlong) as __suseconds_t;
    if setsockopt(
        fd,
        1 as libc::c_int,
        21 as libc::c_int,
        &mut tv as *mut timeval as *const libc::c_void,
        core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_SNDTIMEO: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anetRecvTimeout(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut ms: libc::c_longlong,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    tv.tv_sec = (ms / 1000 as libc::c_int as libc::c_longlong) as __time_t;
    tv
        .tv_usec = (ms % 1000 as libc::c_int as libc::c_longlong
        * 1000 as libc::c_int as libc::c_longlong) as __suseconds_t;
    if setsockopt(
        fd,
        1 as libc::c_int,
        20 as libc::c_int,
        &mut tv as *mut timeval as *const libc::c_void,
        core::mem::size_of::<timeval>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_RCVTIMEO: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn anetResolve(
    mut err: *mut libc::c_char,
    mut host: *mut libc::c_char,
    mut ipbuf: *mut libc::c_char,
    mut ipbuf_len: size_t,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut rv: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        hints.ai_flags = 0x4 as libc::c_int;
    }
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    rv = getaddrinfo(host, 0 as *const libc::c_char, &mut hints, &mut info);
    if rv != 0 as libc::c_int {
        anetSetError(err, b"%s\0" as *const u8 as *const libc::c_char, gai_strerror(rv));
        return -(1 as libc::c_int);
    }
    if (*info).ai_family == 2 as libc::c_int {
        let mut sa: *mut sockaddr_in = (*info).ai_addr as *mut sockaddr_in;
        inet_ntop(
            2 as libc::c_int,
            &mut (*sa).sin_addr as *mut in_addr as *const libc::c_void,
            ipbuf,
            ipbuf_len as socklen_t,
        );
    } else {
        let mut sa_0: *mut sockaddr_in6 = (*info).ai_addr as *mut sockaddr_in6;
        inet_ntop(
            10 as libc::c_int,
            &mut (*sa_0).sin6_addr as *mut in6_addr as *const libc::c_void,
            ipbuf,
            ipbuf_len as socklen_t,
        );
    }
    freeaddrinfo(info);
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetSetReuseAddr(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt SO_REUSEADDR: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetCreateSocket(
    mut err: *mut libc::c_char,
    mut domain: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    s = socket(domain, SOCK_STREAM as libc::c_int, 0 as libc::c_int);
    if s == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"creating socket: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    if anetSetReuseAddr(err, s) == -(1 as libc::c_int) {
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
unsafe extern "C" fn anetTcpGenericConnect(
    mut err: *mut libc::c_char,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s: libc::c_int = -(1 as libc::c_int);
    let mut rv: libc::c_int = 0;
    let mut portstr: [libc::c_char; 6] = [0; 6];
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut bservinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    let mut b: *mut addrinfo = 0 as *mut addrinfo;
    snprintf(
        portstr.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = 0 as libc::c_int;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    rv = getaddrinfo(addr, portstr.as_mut_ptr(), &mut hints, &mut servinfo);
    if rv != 0 as libc::c_int {
        anetSetError(err, b"%s\0" as *const u8 as *const libc::c_char, gai_strerror(rv));
        return -(1 as libc::c_int);
    }
    p = servinfo;
    loop {
        if p.is_null() {
            current_block = 4068382217303356765;
            break;
        }
        s = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
        if !(s == -(1 as libc::c_int)) {
            if anetSetReuseAddr(err, s) == -(1 as libc::c_int) {
                current_block = 18230845023186363638;
                break;
            }
            if flags & 1 as libc::c_int != 0 && anetNonBlock(err, s) != 0 as libc::c_int
            {
                current_block = 18230845023186363638;
                break;
            }
            if !source_addr.is_null() {
                let mut bound: libc::c_int = 0 as libc::c_int;
                rv = getaddrinfo(
                    source_addr,
                    0 as *const libc::c_char,
                    &mut hints,
                    &mut bservinfo,
                );
                if rv != 0 as libc::c_int {
                    anetSetError(
                        err,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        gai_strerror(rv),
                    );
                    current_block = 18230845023186363638;
                    break;
                } else {
                    b = bservinfo;
                    while !b.is_null() {
                        if bind(
                            s,
                            __CONST_SOCKADDR_ARG {
                                __sockaddr__: (*b).ai_addr,
                            },
                            (*b).ai_addrlen,
                        ) != -(1 as libc::c_int)
                        {
                            bound = 1 as libc::c_int;
                            break;
                        } else {
                            b = (*b).ai_next;
                        }
                    }
                    freeaddrinfo(bservinfo);
                    if bound == 0 {
                        anetSetError(
                            err,
                            b"bind: %s\0" as *const u8 as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                        current_block = 18230845023186363638;
                        break;
                    }
                }
            }
            if !(connect(
                s,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: (*p).ai_addr,
                },
                (*p).ai_addrlen,
            ) == -(1 as libc::c_int))
            {
                current_block = 12152712920694627287;
                break;
            }
            if *__errno_location() == 115 as libc::c_int && flags & 1 as libc::c_int != 0
            {
                current_block = 12152712920694627287;
                break;
            }
            close(s);
            s = -(1 as libc::c_int);
        }
        p = (*p).ai_next;
    }
    match current_block {
        4068382217303356765 => {
            if p.is_null() {
                anetSetError(
                    err,
                    b"creating socket: %s\0" as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            current_block = 18230845023186363638;
        }
        _ => {}
    }
    match current_block {
        18230845023186363638 => {
            if s != -(1 as libc::c_int) {
                close(s);
                s = -(1 as libc::c_int);
            }
        }
        _ => {}
    }
    freeaddrinfo(servinfo);
    if s == -(1 as libc::c_int) && !source_addr.is_null()
        && flags & 2 as libc::c_int != 0
    {
        return anetTcpGenericConnect(err, addr, port, 0 as *const libc::c_char, flags)
    } else {
        return s
    };
}
#[no_mangle]
pub unsafe extern "C" fn anetTcpNonBlockConnect(
    mut err: *mut libc::c_char,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    return anetTcpGenericConnect(
        err,
        addr,
        port,
        0 as *const libc::c_char,
        1 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn anetTcpNonBlockBestEffortBindConnect(
    mut err: *mut libc::c_char,
    mut addr: *const libc::c_char,
    mut port: libc::c_int,
    mut source_addr: *const libc::c_char,
) -> libc::c_int {
    return anetTcpGenericConnect(
        err,
        addr,
        port,
        source_addr,
        1 as libc::c_int | 2 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn anetUnixGenericConnect(
    mut err: *mut libc::c_char,
    mut path: *const libc::c_char,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path,
        (core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if flags & 1 as libc::c_int != 0 {
        if anetNonBlock(err, s) != 0 as libc::c_int {
            close(s);
            return -(1 as libc::c_int);
        }
    }
    if connect(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &mut sa as *mut sockaddr_un as *mut sockaddr,
        },
        core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        if *__errno_location() == 115 as libc::c_int && flags & 1 as libc::c_int != 0 {
            return s;
        }
        anetSetError(
            err,
            b"connect: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    return s;
}
unsafe extern "C" fn anetListen(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: socklen_t,
    mut backlog: libc::c_int,
) -> libc::c_int {
    if bind(
        s,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: sa,
        },
        len,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"bind: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    if listen(s, backlog) == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"listen: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        close(s);
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn anetV6Only(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut yes: libc::c_int = 1 as libc::c_int;
    if setsockopt(
        s,
        IPPROTO_IPV6 as libc::c_int,
        26 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _anetTcpServer(
    mut err: *mut libc::c_char,
    mut port: libc::c_int,
    mut bindaddr: *mut libc::c_char,
    mut af: libc::c_int,
    mut backlog: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut s: libc::c_int = -(1 as libc::c_int);
    let mut rv: libc::c_int = 0;
    let mut _port: [libc::c_char; 6] = [0; 6];
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut servinfo: *mut addrinfo = 0 as *mut addrinfo;
    let mut p: *mut addrinfo = 0 as *mut addrinfo;
    snprintf(
        _port.as_mut_ptr(),
        6 as libc::c_int as libc::c_ulong,
        b"%d\0" as *const u8 as *const libc::c_char,
        port,
    );
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = af;
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    hints.ai_flags = 0x1 as libc::c_int;
    if !bindaddr.is_null()
        && strcmp(b"*\0" as *const u8 as *const libc::c_char, bindaddr) == 0
    {
        bindaddr = 0 as *mut libc::c_char;
    }
    if af == 10 as libc::c_int && !bindaddr.is_null()
        && strcmp(b"::*\0" as *const u8 as *const libc::c_char, bindaddr) == 0
    {
        bindaddr = 0 as *mut libc::c_char;
    }
    rv = getaddrinfo(bindaddr, _port.as_mut_ptr(), &mut hints, &mut servinfo);
    if rv != 0 as libc::c_int {
        anetSetError(err, b"%s\0" as *const u8 as *const libc::c_char, gai_strerror(rv));
        return -(1 as libc::c_int);
    }
    p = servinfo;
    loop {
        if p.is_null() {
            current_block = 15976848397966268834;
            break;
        }
        s = socket((*p).ai_family, (*p).ai_socktype, (*p).ai_protocol);
        if s == -(1 as libc::c_int) {
            p = (*p).ai_next;
        } else {
            if af == 10 as libc::c_int && anetV6Only(err, s) == -(1 as libc::c_int) {
                current_block = 11020516886334953311;
                break;
            }
            if anetSetReuseAddr(err, s) == -(1 as libc::c_int) {
                current_block = 11020516886334953311;
                break;
            }
            if anetListen(err, s, (*p).ai_addr, (*p).ai_addrlen, backlog)
                == -(1 as libc::c_int)
            {
                s = -(1 as libc::c_int);
            }
            current_block = 15244307943444686946;
            break;
        }
    }
    match current_block {
        15976848397966268834 => {
            if p.is_null() {
                anetSetError(
                    err,
                    b"unable to bind socket, errno: %d\0" as *const u8
                        as *const libc::c_char,
                    *__errno_location(),
                );
                current_block = 11020516886334953311;
            } else {
                current_block = 11020516886334953311;
            }
        }
        _ => {}
    }
    match current_block {
        11020516886334953311 => {
            if s != -(1 as libc::c_int) {
                close(s);
            }
            s = -(1 as libc::c_int);
        }
        _ => {}
    }
    freeaddrinfo(servinfo);
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn anetTcpServer(
    mut err: *mut libc::c_char,
    mut port: libc::c_int,
    mut bindaddr: *mut libc::c_char,
    mut backlog: libc::c_int,
) -> libc::c_int {
    return _anetTcpServer(err, port, bindaddr, 2 as libc::c_int, backlog);
}
#[no_mangle]
pub unsafe extern "C" fn anetTcp6Server(
    mut err: *mut libc::c_char,
    mut port: libc::c_int,
    mut bindaddr: *mut libc::c_char,
    mut backlog: libc::c_int,
) -> libc::c_int {
    return _anetTcpServer(err, port, bindaddr, 10 as libc::c_int, backlog);
}
#[no_mangle]
pub unsafe extern "C" fn anetUnixServer(
    mut err: *mut libc::c_char,
    mut path: *mut libc::c_char,
    mut perm: mode_t,
    mut backlog: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    if strlen(path)
        > (core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        anetSetError(
            err,
            b"unix socket path too long (%zu), must be under %zu\0" as *const u8
                as *const libc::c_char,
            strlen(path),
            core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong,
        );
        return -(1 as libc::c_int);
    }
    s = anetCreateSocket(err, 1 as libc::c_int);
    if s == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    memset(
        &mut sa as *mut sockaddr_un as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<sockaddr_un>() as libc::c_ulong,
    );
    sa.sun_family = 1 as libc::c_int as sa_family_t;
    strncpy(
        (sa.sun_path).as_mut_ptr(),
        path,
        (core::mem::size_of::<[libc::c_char; 108]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
    if anetListen(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        core::mem::size_of::<sockaddr_un>() as libc::c_ulong as socklen_t,
        backlog,
    ) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    if perm != 0 {
        chmod((sa.sun_path).as_mut_ptr(), perm);
    }
    return s;
}
unsafe extern "C" fn anetGenericAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
    mut sa: *mut sockaddr,
    mut len: *mut socklen_t,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    loop {
        fd = accept4(
            s,
            __SOCKADDR_ARG { __sockaddr__: sa },
            len,
            SOCK_NONBLOCK as libc::c_int | SOCK_CLOEXEC as libc::c_int,
        );
        if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if fd == -(1 as libc::c_int) {
        anetSetError(
            err,
            b"accept: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn anetTcpAccept(
    mut err: *mut libc::c_char,
    mut serversock: libc::c_int,
    mut ip: *mut libc::c_char,
    mut ip_len: size_t,
    mut port: *mut libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut salen: socklen_t = core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    fd = anetGenericAccept(
        err,
        serversock,
        &mut sa as *mut sockaddr_storage as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if sa.ss_family as libc::c_int == 2 as libc::c_int {
        let mut s: *mut sockaddr_in = &mut sa as *mut sockaddr_storage
            as *mut sockaddr_in;
        if !ip.is_null() {
            inet_ntop(
                2 as libc::c_int,
                &mut (*s).sin_addr as *mut in_addr as *mut libc::c_void,
                ip,
                ip_len as socklen_t,
            );
        }
        if !port.is_null() {
            *port = __bswap_16((*s).sin_port) as libc::c_int;
        }
    } else {
        let mut s_0: *mut sockaddr_in6 = &mut sa as *mut sockaddr_storage
            as *mut sockaddr_in6;
        if !ip.is_null() {
            inet_ntop(
                10 as libc::c_int,
                &mut (*s_0).sin6_addr as *mut in6_addr as *mut libc::c_void,
                ip,
                ip_len as socklen_t,
            );
        }
        if !port.is_null() {
            *port = __bswap_16((*s_0).sin6_port) as libc::c_int;
        }
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn anetUnixAccept(
    mut err: *mut libc::c_char,
    mut s: libc::c_int,
) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut sa: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut salen: socklen_t = core::mem::size_of::<sockaddr_un>() as libc::c_ulong
        as socklen_t;
    fd = anetGenericAccept(
        err,
        s,
        &mut sa as *mut sockaddr_un as *mut sockaddr,
        &mut salen,
    );
    if fd == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn anetFdToString(
    mut fd: libc::c_int,
    mut ip: *mut libc::c_char,
    mut ip_len: size_t,
    mut port: *mut libc::c_int,
    mut fd_to_str_type: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut salen: socklen_t = core::mem::size_of::<sockaddr_storage>()
        as libc::c_ulong as socklen_t;
    if fd_to_str_type == 0 as libc::c_int {
        if getpeername(
            fd,
            __SOCKADDR_ARG {
                __sockaddr__: &mut sa as *mut sockaddr_storage as *mut sockaddr,
            },
            &mut salen,
        ) == -(1 as libc::c_int)
        {
            current_block = 3946958208285236138;
        } else {
            current_block = 7502529970979898288;
        }
    } else if getsockname(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut sa as *mut sockaddr_storage as *mut sockaddr,
        },
        &mut salen,
    ) == -(1 as libc::c_int)
    {
        current_block = 3946958208285236138;
    } else {
        current_block = 7502529970979898288;
    }
    match current_block {
        7502529970979898288 => {
            if sa.ss_family as libc::c_int == 2 as libc::c_int {
                let mut s: *mut sockaddr_in = &mut sa as *mut sockaddr_storage
                    as *mut sockaddr_in;
                if !ip.is_null() {
                    if (inet_ntop(
                        2 as libc::c_int,
                        &mut (*s).sin_addr as *mut in_addr as *mut libc::c_void,
                        ip,
                        ip_len as socklen_t,
                    ))
                        .is_null()
                    {
                        current_block = 3946958208285236138;
                    } else {
                        current_block = 17216689946888361452;
                    }
                } else {
                    current_block = 17216689946888361452;
                }
                match current_block {
                    3946958208285236138 => {}
                    _ => {
                        if !port.is_null() {
                            *port = __bswap_16((*s).sin_port) as libc::c_int;
                        }
                        current_block = 11307063007268554308;
                    }
                }
            } else if sa.ss_family as libc::c_int == 10 as libc::c_int {
                let mut s_0: *mut sockaddr_in6 = &mut sa as *mut sockaddr_storage
                    as *mut sockaddr_in6;
                if !ip.is_null() {
                    if (inet_ntop(
                        10 as libc::c_int,
                        &mut (*s_0).sin6_addr as *mut in6_addr as *mut libc::c_void,
                        ip,
                        ip_len as socklen_t,
                    ))
                        .is_null()
                    {
                        current_block = 3946958208285236138;
                    } else {
                        current_block = 3512920355445576850;
                    }
                } else {
                    current_block = 3512920355445576850;
                }
                match current_block {
                    3946958208285236138 => {}
                    _ => {
                        if !port.is_null() {
                            *port = __bswap_16((*s_0).sin6_port) as libc::c_int;
                        }
                        current_block = 11307063007268554308;
                    }
                }
            } else if sa.ss_family as libc::c_int == 1 as libc::c_int {
                if !ip.is_null() {
                    let mut res: libc::c_int = snprintf(
                        ip,
                        ip_len,
                        b"/unixsocket\0" as *const u8 as *const libc::c_char,
                    );
                    if res < 0 as libc::c_int
                        || res as libc::c_uint as libc::c_ulong >= ip_len
                    {
                        current_block = 3946958208285236138;
                    } else {
                        current_block = 12124785117276362961;
                    }
                } else {
                    current_block = 12124785117276362961;
                }
                match current_block {
                    3946958208285236138 => {}
                    _ => {
                        if !port.is_null() {
                            *port = 0 as libc::c_int;
                        }
                        current_block = 11307063007268554308;
                    }
                }
            } else {
                current_block = 3946958208285236138;
            }
            match current_block {
                3946958208285236138 => {}
                _ => return 0 as libc::c_int,
            }
        }
        _ => {}
    }
    if !ip.is_null() {
        if ip_len >= 2 as libc::c_int as libc::c_ulong {
            *ip.offset(0 as libc::c_int as isize) = '?' as i32 as libc::c_char;
            *ip.offset(1 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        } else if ip_len == 1 as libc::c_int as libc::c_ulong {
            *ip.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
    }
    if !port.is_null() {
        *port = 0 as libc::c_int;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetFormatAddr(
    mut buf: *mut libc::c_char,
    mut buf_len: size_t,
    mut ip: *mut libc::c_char,
    mut port: libc::c_int,
) -> libc::c_int {
    return snprintf(
        buf,
        buf_len,
        if !(strchr(ip, ':' as i32)).is_null() {
            b"[%s]:%d\0" as *const u8 as *const libc::c_char
        } else {
            b"%s:%d\0" as *const u8 as *const libc::c_char
        },
        ip,
        port,
    );
}
#[no_mangle]
pub unsafe extern "C" fn anetFormatFdAddr(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_char,
    mut buf_len: size_t,
    mut fd_to_str_type: libc::c_int,
) -> libc::c_int {
    let mut ip: [libc::c_char; 46] = [0; 46];
    let mut port: libc::c_int = 0;
    anetFdToString(
        fd,
        ip.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
        &mut port,
        fd_to_str_type,
    );
    return anetFormatAddr(buf, buf_len, ip.as_mut_ptr(), port);
}
#[no_mangle]
pub unsafe extern "C" fn anetPipe(
    mut fds: *mut libc::c_int,
    mut read_flags: libc::c_int,
    mut write_flags: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut pipe_flags: libc::c_int = 0 as libc::c_int;
    pipe_flags = 0o2000000 as libc::c_int | read_flags & write_flags;
    if pipe2(fds, pipe_flags) != 0 {
        if *__errno_location() != 38 as libc::c_int
            && *__errno_location() != 22 as libc::c_int
        {
            return -(1 as libc::c_int);
        }
        pipe_flags = 0 as libc::c_int;
    } else {
        if 0o2000000 as libc::c_int | read_flags
            == 0o2000000 as libc::c_int | write_flags
        {
            return 0 as libc::c_int;
        }
        read_flags &= !pipe_flags;
        write_flags &= !pipe_flags;
    }
    if pipe_flags == 0 as libc::c_int && pipe(fds) != 0 {
        return -(1 as libc::c_int);
    }
    if read_flags & 0o2000000 as libc::c_int != 0 {
        if fcntl(
            *fds.offset(0 as libc::c_int as isize),
            2 as libc::c_int,
            1 as libc::c_int,
        ) != 0
        {
            current_block = 17977527455243449991;
        } else {
            current_block = 7746791466490516765;
        }
    } else {
        current_block = 7746791466490516765;
    }
    match current_block {
        7746791466490516765 => {
            if write_flags & 0o2000000 as libc::c_int != 0 {
                if fcntl(
                    *fds.offset(1 as libc::c_int as isize),
                    2 as libc::c_int,
                    1 as libc::c_int,
                ) != 0
                {
                    current_block = 17977527455243449991;
                } else {
                    current_block = 13586036798005543211;
                }
            } else {
                current_block = 13586036798005543211;
            }
            match current_block {
                17977527455243449991 => {}
                _ => {
                    read_flags &= !(0o2000000 as libc::c_int);
                    if read_flags != 0 {
                        if fcntl(
                            *fds.offset(0 as libc::c_int as isize),
                            4 as libc::c_int,
                            read_flags,
                        ) != 0
                        {
                            current_block = 17977527455243449991;
                        } else {
                            current_block = 17407779659766490442;
                        }
                    } else {
                        current_block = 17407779659766490442;
                    }
                    match current_block {
                        17977527455243449991 => {}
                        _ => {
                            write_flags &= !(0o2000000 as libc::c_int);
                            if write_flags != 0 {
                                if fcntl(
                                    *fds.offset(1 as libc::c_int as isize),
                                    4 as libc::c_int,
                                    write_flags,
                                ) != 0
                                {
                                    current_block = 17977527455243449991;
                                } else {
                                    current_block = 6009453772311597924;
                                }
                            } else {
                                current_block = 6009453772311597924;
                            }
                            match current_block {
                                17977527455243449991 => {}
                                _ => return 0 as libc::c_int,
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    close(*fds.offset(0 as libc::c_int as isize));
    close(*fds.offset(1 as libc::c_int as isize));
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn anetSetSockMarkId(
    mut err: *mut libc::c_char,
    mut fd: libc::c_int,
    mut id: uint32_t,
) -> libc::c_int {
    if setsockopt(
        fd,
        1 as libc::c_int,
        36 as libc::c_int,
        &mut id as *mut uint32_t as *mut libc::c_void,
        core::mem::size_of::<uint32_t>() as libc::c_ulong as socklen_t,
    ) == -(1 as libc::c_int)
    {
        anetSetError(
            err,
            b"setsockopt: %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}

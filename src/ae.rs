
extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn monotonicInit() -> *const libc::c_char;
    fn anetCloexec(fd: libc::c_int) -> libc::c_int;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn poll(__fds: *mut pollfd, __nfds: nfds_t, __timeout: libc::c_int) -> libc::c_int;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn zfree(ptr: *mut libc::c_void);
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn epoll_wait(
        __epfd: libc::c_int,
        __events: *mut epoll_event,
        __maxevents: libc::c_int,
        __timeout: libc::c_int,
    ) -> libc::c_int;
    fn epoll_ctl(
        __epfd: libc::c_int,
        __op: libc::c_int,
        __fd: libc::c_int,
        __event: *mut epoll_event,
    ) -> libc::c_int;
    fn epoll_create(__size: libc::c_int) -> libc::c_int;
}
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int64_t = __int64_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
pub type monotime = uint64_t;
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
pub struct aeApiState {
    pub epfd: libc::c_int,
    pub events: *mut epoll_event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct epoll_event {
    pub events: uint32_t,
    pub data: epoll_data_t,
}
pub type epoll_data_t = epoll_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub union epoll_data {
    pub ptr: *mut libc::c_void,
    pub fd: libc::c_int,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
pub const EPOLLOUT: EPOLL_EVENTS = 4;
pub const EPOLLIN: EPOLL_EVENTS = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub const EPOLLHUP: EPOLL_EVENTS = 16;
pub const EPOLLERR: EPOLL_EVENTS = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: libc::c_int,
    pub events: libc::c_short,
    pub revents: libc::c_short,
}
pub type nfds_t = libc::c_ulong;
pub type EPOLL_EVENTS = libc::c_uint;
pub const EPOLLET: EPOLL_EVENTS = 2147483648;
pub const EPOLLONESHOT: EPOLL_EVENTS = 1073741824;
pub const EPOLLWAKEUP: EPOLL_EVENTS = 536870912;
pub const EPOLLEXCLUSIVE: EPOLL_EVENTS = 268435456;
pub const EPOLLRDHUP: EPOLL_EVENTS = 8192;
pub const EPOLLMSG: EPOLL_EVENTS = 1024;
pub const EPOLLWRBAND: EPOLL_EVENTS = 512;
pub const EPOLLWRNORM: EPOLL_EVENTS = 256;
pub const EPOLLRDBAND: EPOLL_EVENTS = 128;
pub const EPOLLRDNORM: EPOLL_EVENTS = 64;
pub const EPOLLPRI: EPOLL_EVENTS = 2;
unsafe extern "C" fn aeApiPoll(
    mut eventLoop: *mut aeEventLoop,
    mut tvp: *mut timeval,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut retval: libc::c_int = 0;
    let mut numevents: libc::c_int = 0 as libc::c_int;
    retval = epoll_wait(
        (*state).epfd,
        (*state).events,
        (*eventLoop).setsize,
        (if !tvp.is_null() {
            (*tvp).tv_sec * 1000 as libc::c_int as libc::c_long
                + ((*tvp).tv_usec + 999 as libc::c_int as libc::c_long)
                    / 1000 as libc::c_int as libc::c_long
        } else {
            -(1 as libc::c_int) as libc::c_long
        }) as libc::c_int,
    );
    if retval > 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        numevents = retval;
        j = 0 as libc::c_int;
        while j < numevents {
            let mut mask: libc::c_int = 0 as libc::c_int;
            let mut e: *mut epoll_event = ((*state).events).offset(j as isize);
            if (*e).events & EPOLLIN as libc::c_int as libc::c_uint != 0 {
                mask |= 1 as libc::c_int;
            }
            if (*e).events & EPOLLOUT as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int;
            }
            if (*e).events & EPOLLERR as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int | 1 as libc::c_int;
            }
            if (*e).events & EPOLLHUP as libc::c_int as libc::c_uint != 0 {
                mask |= 2 as libc::c_int | 1 as libc::c_int;
            }
            (*((*eventLoop).fired).offset(j as isize)).fd = (*e).data.fd;
            (*((*eventLoop).fired).offset(j as isize)).mask = mask;
            j += 1;
        }
    } else if retval == -(1 as libc::c_int) && *__errno_location() != 4 as libc::c_int {
        _serverPanic(
            b"./ae_epoll.c\0" as *const u8 as *const libc::c_char,
            131 as libc::c_int,
            b"aeApiPoll: epoll_wait, %s\0" as *const u8 as *const libc::c_char,
            strerror(*__errno_location()),
        );
        unreachable!();
    }
    return numevents;
}
unsafe extern "C" fn aeApiResize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    (*state)
        .events = zrealloc(
        (*state).events as *mut libc::c_void,
        (core::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut epoll_event;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiDelEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut delmask: libc::c_int,
) {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut ee: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut mask: libc::c_int = (*((*eventLoop).events).offset(fd as isize)).mask
        & !delmask;
    ee.events = 0 as libc::c_int as uint32_t;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= EPOLLIN as libc::c_int as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= EPOLLOUT as libc::c_int as libc::c_uint;
    }
    ee.data.fd = fd;
    if mask != 0 as libc::c_int {
        epoll_ctl((*state).epfd, 3 as libc::c_int, fd, &mut ee);
    } else {
        epoll_ctl((*state).epfd, 2 as libc::c_int, fd, &mut ee);
    };
}
unsafe extern "C" fn aeApiAddEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) -> libc::c_int {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    let mut ee: epoll_event = {
        let mut init = epoll_event {
            events: 0 as libc::c_int as uint32_t,
            data: epoll_data {
                ptr: 0 as *mut libc::c_void,
            },
        };
        init
    };
    let mut op: libc::c_int = if (*((*eventLoop).events).offset(fd as isize)).mask
        == 0 as libc::c_int
    {
        1 as libc::c_int
    } else {
        3 as libc::c_int
    };
    ee.events = 0 as libc::c_int as uint32_t;
    mask |= (*((*eventLoop).events).offset(fd as isize)).mask;
    if mask & 1 as libc::c_int != 0 {
        ee.events |= EPOLLIN as libc::c_int as libc::c_uint;
    }
    if mask & 2 as libc::c_int != 0 {
        ee.events |= EPOLLOUT as libc::c_int as libc::c_uint;
    }
    ee.data.fd = fd;
    if epoll_ctl((*state).epfd, op, fd, &mut ee) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiFree(mut eventLoop: *mut aeEventLoop) {
    let mut state: *mut aeApiState = (*eventLoop).apidata as *mut aeApiState;
    close((*state).epfd);
    zfree((*state).events as *mut libc::c_void);
    zfree(state as *mut libc::c_void);
}
unsafe extern "C" fn aeApiCreate(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut state: *mut aeApiState = zmalloc(
        core::mem::size_of::<aeApiState>() as libc::c_ulong,
    ) as *mut aeApiState;
    if state.is_null() {
        return -(1 as libc::c_int);
    }
    (*state)
        .events = zmalloc(
        (core::mem::size_of::<epoll_event>() as libc::c_ulong)
            .wrapping_mul((*eventLoop).setsize as libc::c_ulong),
    ) as *mut epoll_event;
    if ((*state).events).is_null() {
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    (*state).epfd = epoll_create(1024 as libc::c_int);
    if (*state).epfd == -(1 as libc::c_int) {
        zfree((*state).events as *mut libc::c_void);
        zfree(state as *mut libc::c_void);
        return -(1 as libc::c_int);
    }
    anetCloexec((*state).epfd);
    (*eventLoop).apidata = state as *mut libc::c_void;
    return 0 as libc::c_int;
}
unsafe extern "C" fn aeApiName() -> *mut libc::c_char {
    return b"epoll\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn aeCreateEventLoop(
    mut setsize: libc::c_int,
) -> *mut aeEventLoop {
    let mut eventLoop: *mut aeEventLoop = 0 as *mut aeEventLoop;
    let mut i: libc::c_int = 0;
    monotonicInit();
    eventLoop = zmalloc(core::mem::size_of::<aeEventLoop>() as libc::c_ulong)
        as *mut aeEventLoop;
    if !eventLoop.is_null() {
        (*eventLoop)
            .events = zmalloc(
            (core::mem::size_of::<aeFileEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        ) as *mut aeFileEvent;
        (*eventLoop)
            .fired = zmalloc(
            (core::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
                .wrapping_mul(setsize as libc::c_ulong),
        ) as *mut aeFiredEvent;
        if !(((*eventLoop).events).is_null() || ((*eventLoop).fired).is_null()) {
            (*eventLoop).setsize = setsize;
            (*eventLoop).timeEventHead = 0 as *mut aeTimeEvent;
            (*eventLoop).timeEventNextId = 0 as libc::c_int as libc::c_longlong;
            (*eventLoop).stop = 0 as libc::c_int;
            (*eventLoop).maxfd = -(1 as libc::c_int);
            (*eventLoop).beforesleep = None;
            (*eventLoop).aftersleep = None;
            (*eventLoop).flags = 0 as libc::c_int;
            if !(aeApiCreate(eventLoop) == -(1 as libc::c_int)) {
                i = 0 as libc::c_int;
                while i < setsize {
                    (*((*eventLoop).events).offset(i as isize)).mask = 0 as libc::c_int;
                    i += 1;
                }
                return eventLoop;
            }
        }
    }
    if !eventLoop.is_null() {
        zfree((*eventLoop).events as *mut libc::c_void);
        zfree((*eventLoop).fired as *mut libc::c_void);
        zfree(eventLoop as *mut libc::c_void);
    }
    return 0 as *mut aeEventLoop;
}
#[no_mangle]
pub unsafe extern "C" fn aeGetSetSize(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    return (*eventLoop).setsize;
}
#[no_mangle]
pub unsafe extern "C" fn aeSetDontWait(
    mut eventLoop: *mut aeEventLoop,
    mut noWait: libc::c_int,
) {
    if noWait != 0 {
        (*eventLoop).flags |= (1 as libc::c_int) << 2 as libc::c_int;
    } else {
        (*eventLoop).flags &= !((1 as libc::c_int) << 2 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn aeResizeSetSize(
    mut eventLoop: *mut aeEventLoop,
    mut setsize: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if setsize == (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    if (*eventLoop).maxfd >= setsize {
        return -(1 as libc::c_int);
    }
    if aeApiResize(eventLoop, setsize) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*eventLoop)
        .events = zrealloc(
        (*eventLoop).events as *mut libc::c_void,
        (core::mem::size_of::<aeFileEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut aeFileEvent;
    (*eventLoop)
        .fired = zrealloc(
        (*eventLoop).fired as *mut libc::c_void,
        (core::mem::size_of::<aeFiredEvent>() as libc::c_ulong)
            .wrapping_mul(setsize as libc::c_ulong),
    ) as *mut aeFiredEvent;
    (*eventLoop).setsize = setsize;
    i = (*eventLoop).maxfd + 1 as libc::c_int;
    while i < setsize {
        (*((*eventLoop).events).offset(i as isize)).mask = 0 as libc::c_int;
        i += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aeDeleteEventLoop(mut eventLoop: *mut aeEventLoop) {
    aeApiFree(eventLoop);
    zfree((*eventLoop).events as *mut libc::c_void);
    zfree((*eventLoop).fired as *mut libc::c_void);
    let mut next_te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut te: *mut aeTimeEvent = (*eventLoop).timeEventHead;
    while !te.is_null() {
        next_te = (*te).next;
        zfree(te as *mut libc::c_void);
        te = next_te;
    }
    zfree(eventLoop as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn aeStop(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aeCreateFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut proc_0: Option::<aeFileProc>,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    if fd >= (*eventLoop).setsize {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int);
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    if aeApiAddEvent(eventLoop, fd, mask) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*fe).mask |= mask;
    if mask & 1 as libc::c_int != 0 {
        (*fe).rfileProc = proc_0;
    }
    if mask & 2 as libc::c_int != 0 {
        (*fe).wfileProc = proc_0;
    }
    (*fe).clientData = clientData;
    if fd > (*eventLoop).maxfd {
        (*eventLoop).maxfd = fd;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn aeDeleteFileEvent(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) {
    if fd >= (*eventLoop).setsize {
        return;
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    if (*fe).mask == 0 as libc::c_int {
        return;
    }
    if mask & 2 as libc::c_int != 0 {
        mask |= 4 as libc::c_int;
    }
    aeApiDelEvent(eventLoop, fd, mask);
    (*fe).mask = (*fe).mask & !mask;
    if fd == (*eventLoop).maxfd && (*fe).mask == 0 as libc::c_int {
        let mut j: libc::c_int = 0;
        j = (*eventLoop).maxfd - 1 as libc::c_int;
        while j >= 0 as libc::c_int {
            if (*((*eventLoop).events).offset(j as isize)).mask != 0 as libc::c_int {
                break;
            }
            j -= 1;
        }
        (*eventLoop).maxfd = j;
    }
}
#[no_mangle]
pub unsafe extern "C" fn aeGetFileClientData(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
) -> *mut libc::c_void {
    if fd >= (*eventLoop).setsize {
        return 0 as *mut libc::c_void;
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    if (*fe).mask == 0 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    return (*fe).clientData;
}
#[no_mangle]
pub unsafe extern "C" fn aeGetFileEvents(
    mut eventLoop: *mut aeEventLoop,
    mut fd: libc::c_int,
) -> libc::c_int {
    if fd >= (*eventLoop).setsize {
        return 0 as libc::c_int;
    }
    let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events).offset(fd as isize)
        as *mut aeFileEvent;
    return (*fe).mask;
}
#[no_mangle]
pub unsafe extern "C" fn aeCreateTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut milliseconds: libc::c_longlong,
    mut proc_0: Option::<aeTimeProc>,
    mut clientData: *mut libc::c_void,
    mut finalizerProc: Option::<aeEventFinalizerProc>,
) -> libc::c_longlong {
    let fresh0 = (*eventLoop).timeEventNextId;
    (*eventLoop).timeEventNextId = (*eventLoop).timeEventNextId + 1;
    let mut id: libc::c_longlong = fresh0;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    te = zmalloc(core::mem::size_of::<aeTimeEvent>() as libc::c_ulong)
        as *mut aeTimeEvent;
    if te.is_null() {
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    (*te).id = id;
    (*te)
        .when = (getMonotonicUs.expect("non-null function pointer")()
        as libc::c_ulonglong)
        .wrapping_add(
            (milliseconds * 1000 as libc::c_int as libc::c_longlong) as libc::c_ulonglong,
        ) as monotime;
    (*te).timeProc = proc_0;
    (*te).finalizerProc = finalizerProc;
    (*te).clientData = clientData;
    (*te).prev = 0 as *mut aeTimeEvent;
    (*te).next = (*eventLoop).timeEventHead;
    (*te).refcount = 0 as libc::c_int;
    if !((*te).next).is_null() {
        (*(*te).next).prev = te;
    }
    (*eventLoop).timeEventHead = te;
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn aeDeleteTimeEvent(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
) -> libc::c_int {
    let mut te: *mut aeTimeEvent = (*eventLoop).timeEventHead;
    while !te.is_null() {
        if (*te).id == id {
            (*te).id = -(1 as libc::c_int) as libc::c_longlong;
            return 0 as libc::c_int;
        }
        te = (*te).next;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn usUntilEarliestTimer(mut eventLoop: *mut aeEventLoop) -> int64_t {
    let mut te: *mut aeTimeEvent = (*eventLoop).timeEventHead;
    if te.is_null() {
        return -(1 as libc::c_int) as int64_t;
    }
    let mut earliest: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    while !te.is_null() {
        if earliest.is_null() || (*te).when < (*earliest).when {
            earliest = te;
        }
        te = (*te).next;
    }
    let mut now: monotime = getMonotonicUs.expect("non-null function pointer")();
    return (if now >= (*earliest).when {
        0 as libc::c_int as libc::c_ulong
    } else {
        ((*earliest).when).wrapping_sub(now)
    }) as int64_t;
}
unsafe extern "C" fn processTimeEvents(mut eventLoop: *mut aeEventLoop) -> libc::c_int {
    let mut processed: libc::c_int = 0 as libc::c_int;
    let mut te: *mut aeTimeEvent = 0 as *mut aeTimeEvent;
    let mut maxId: libc::c_longlong = 0;
    te = (*eventLoop).timeEventHead;
    maxId = (*eventLoop).timeEventNextId - 1 as libc::c_int as libc::c_longlong;
    let mut now: monotime = getMonotonicUs.expect("non-null function pointer")();
    while !te.is_null() {
        let mut id: libc::c_longlong = 0;
        if (*te).id == -(1 as libc::c_int) as libc::c_longlong {
            let mut next: *mut aeTimeEvent = (*te).next;
            if (*te).refcount != 0 {
                te = next;
            } else {
                if !((*te).prev).is_null() {
                    (*(*te).prev).next = (*te).next;
                } else {
                    (*eventLoop).timeEventHead = (*te).next;
                }
                if !((*te).next).is_null() {
                    (*(*te).next).prev = (*te).prev;
                }
                if ((*te).finalizerProc).is_some() {
                    ((*te).finalizerProc)
                        .expect(
                            "non-null function pointer",
                        )(eventLoop, (*te).clientData);
                    now = getMonotonicUs.expect("non-null function pointer")();
                }
                zfree(te as *mut libc::c_void);
                te = next;
            }
        } else if (*te).id > maxId {
            te = (*te).next;
        } else {
            if (*te).when <= now {
                let mut retval: libc::c_int = 0;
                id = (*te).id;
                (*te).refcount += 1;
                retval = ((*te).timeProc)
                    .expect(
                        "non-null function pointer",
                    )(eventLoop, id, (*te).clientData);
                (*te).refcount -= 1;
                processed += 1;
                now = getMonotonicUs.expect("non-null function pointer")();
                if retval != -(1 as libc::c_int) {
                    (*te)
                        .when = now
                        .wrapping_add((retval * 1000 as libc::c_int) as libc::c_ulong);
                } else {
                    (*te).id = -(1 as libc::c_int) as libc::c_longlong;
                }
            }
            te = (*te).next;
        }
    }
    return processed;
}
#[no_mangle]
pub unsafe extern "C" fn aeProcessEvents(
    mut eventLoop: *mut aeEventLoop,
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut processed: libc::c_int = 0 as libc::c_int;
    let mut numevents: libc::c_int = 0;
    if flags & (1 as libc::c_int) << 1 as libc::c_int == 0
        && flags & (1 as libc::c_int) << 0 as libc::c_int == 0
    {
        return 0 as libc::c_int;
    }
    if (*eventLoop).maxfd != -(1 as libc::c_int)
        || flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            && flags & (1 as libc::c_int) << 2 as libc::c_int == 0
    {
        let mut j: libc::c_int = 0;
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut tvp: *mut timeval = 0 as *mut timeval;
        let mut usUntilTimer: int64_t = -(1 as libc::c_int) as int64_t;
        if flags & (1 as libc::c_int) << 1 as libc::c_int != 0
            && flags & (1 as libc::c_int) << 2 as libc::c_int == 0
        {
            usUntilTimer = usUntilEarliestTimer(eventLoop);
        }
        if usUntilTimer >= 0 as libc::c_int as libc::c_long {
            tv.tv_sec = usUntilTimer / 1000000 as libc::c_int as libc::c_long;
            tv.tv_usec = usUntilTimer % 1000000 as libc::c_int as libc::c_long;
            tvp = &mut tv;
        } else if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            tv.tv_sec = tv.tv_usec;
            tvp = &mut tv;
        } else {
            tvp = 0 as *mut timeval;
        }
        if (*eventLoop).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            tv.tv_usec = 0 as libc::c_int as __suseconds_t;
            tv.tv_sec = tv.tv_usec;
            tvp = &mut tv;
        }
        if ((*eventLoop).beforesleep).is_some()
            && flags & (1 as libc::c_int) << 3 as libc::c_int != 0
        {
            ((*eventLoop).beforesleep).expect("non-null function pointer")(eventLoop);
        }
        numevents = aeApiPoll(eventLoop, tvp);
        if ((*eventLoop).aftersleep).is_some()
            && flags & (1 as libc::c_int) << 4 as libc::c_int != 0
        {
            ((*eventLoop).aftersleep).expect("non-null function pointer")(eventLoop);
        }
        j = 0 as libc::c_int;
        while j < numevents {
            let mut fd: libc::c_int = (*((*eventLoop).fired).offset(j as isize)).fd;
            let mut fe: *mut aeFileEvent = &mut *((*eventLoop).events)
                .offset(fd as isize) as *mut aeFileEvent;
            let mut mask: libc::c_int = (*((*eventLoop).fired).offset(j as isize)).mask;
            let mut fired: libc::c_int = 0 as libc::c_int;
            let mut invert: libc::c_int = (*fe).mask & 4 as libc::c_int;
            if invert == 0 && (*fe).mask & mask & 1 as libc::c_int != 0 {
                ((*fe).rfileProc)
                    .expect(
                        "non-null function pointer",
                    )(eventLoop, fd, (*fe).clientData, mask);
                fired += 1;
                fe = &mut *((*eventLoop).events).offset(fd as isize) as *mut aeFileEvent;
            }
            if (*fe).mask & mask & 2 as libc::c_int != 0 {
                if fired == 0 || (*fe).wfileProc != (*fe).rfileProc {
                    ((*fe).wfileProc)
                        .expect(
                            "non-null function pointer",
                        )(eventLoop, fd, (*fe).clientData, mask);
                    fired += 1;
                }
            }
            if invert != 0 {
                fe = &mut *((*eventLoop).events).offset(fd as isize) as *mut aeFileEvent;
                if (*fe).mask & mask & 1 as libc::c_int != 0
                    && (fired == 0 || (*fe).wfileProc != (*fe).rfileProc)
                {
                    ((*fe).rfileProc)
                        .expect(
                            "non-null function pointer",
                        )(eventLoop, fd, (*fe).clientData, mask);
                    fired += 1;
                }
            }
            processed += 1;
            j += 1;
        }
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        processed += processTimeEvents(eventLoop);
    }
    return processed;
}
#[no_mangle]
pub unsafe extern "C" fn aeWait(
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut milliseconds: libc::c_longlong,
) -> libc::c_int {
    let mut pfd: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut retmask: libc::c_int = 0 as libc::c_int;
    let mut retval: libc::c_int = 0;
    memset(
        &mut pfd as *mut pollfd as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<pollfd>() as libc::c_ulong,
    );
    pfd.fd = fd;
    if mask & 1 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 0x1 as libc::c_int) as libc::c_short;
    }
    if mask & 2 as libc::c_int != 0 {
        pfd.events = (pfd.events as libc::c_int | 0x4 as libc::c_int) as libc::c_short;
    }
    retval = poll(&mut pfd, 1 as libc::c_int as nfds_t, milliseconds as libc::c_int);
    if retval == 1 as libc::c_int {
        if pfd.revents as libc::c_int & 0x1 as libc::c_int != 0 {
            retmask |= 1 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x4 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x8 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        if pfd.revents as libc::c_int & 0x10 as libc::c_int != 0 {
            retmask |= 2 as libc::c_int;
        }
        return retmask;
    } else {
        return retval
    };
}
#[no_mangle]
pub unsafe extern "C" fn aeMain(mut eventLoop: *mut aeEventLoop) {
    (*eventLoop).stop = 0 as libc::c_int;
    while (*eventLoop).stop == 0 {
        aeProcessEvents(
            eventLoop,
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn aeGetApiName() -> *mut libc::c_char {
    return aeApiName();
}
#[no_mangle]
pub unsafe extern "C" fn aeSetBeforeSleepProc(
    mut eventLoop: *mut aeEventLoop,
    mut beforesleep: Option::<aeBeforeSleepProc>,
) {
    (*eventLoop).beforesleep = beforesleep;
}
#[no_mangle]
pub unsafe extern "C" fn aeSetAfterSleepProc(
    mut eventLoop: *mut aeEventLoop,
    mut aftersleep: Option::<aeBeforeSleepProc>,
) {
    (*eventLoop).aftersleep = aftersleep;
}

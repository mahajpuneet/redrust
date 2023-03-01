extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn aeWait(
        fd: libc::c_int,
        mask: libc::c_int,
        milliseconds: libc::c_longlong,
    ) -> libc::c_int;
    fn mstime() -> libc::c_longlong;
}
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
#[no_mangle]
pub unsafe extern "C" fn syncWrite(
    mut fd: libc::c_int,
    mut ptr: *mut libc::c_char,
    mut size: ssize_t,
    mut timeout: libc::c_longlong,
) -> ssize_t {
    let mut nwritten: ssize_t = 0;
    let mut ret: ssize_t = size;
    let mut start: libc::c_longlong = mstime();
    let mut remaining: libc::c_longlong = timeout;
    loop {
        let mut wait: libc::c_longlong = if remaining
            > 10 as libc::c_int as libc::c_longlong
        {
            remaining
        } else {
            10 as libc::c_int as libc::c_longlong
        };
        let mut elapsed: libc::c_longlong = 0;
        nwritten = write(fd, ptr as *const libc::c_void, size as size_t);
        if nwritten == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() != 11 as libc::c_int {
                return -(1 as libc::c_int) as ssize_t;
            }
        } else {
            ptr = ptr.offset(nwritten as isize);
            size -= nwritten;
        }
        if size == 0 as libc::c_int as libc::c_long {
            return ret;
        }
        aeWait(fd, 2 as libc::c_int, wait);
        elapsed = mstime() - start;
        if elapsed >= timeout {
            *__errno_location() = 110 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        remaining = timeout - elapsed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn syncRead(
    mut fd: libc::c_int,
    mut ptr: *mut libc::c_char,
    mut size: ssize_t,
    mut timeout: libc::c_longlong,
) -> ssize_t {
    let mut nread: ssize_t = 0;
    let mut totread: ssize_t = 0 as libc::c_int as ssize_t;
    let mut start: libc::c_longlong = mstime();
    let mut remaining: libc::c_longlong = timeout;
    if size == 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as ssize_t;
    }
    loop {
        let mut wait: libc::c_longlong = if remaining
            > 10 as libc::c_int as libc::c_longlong
        {
            remaining
        } else {
            10 as libc::c_int as libc::c_longlong
        };
        let mut elapsed: libc::c_longlong = 0;
        nread = read(fd, ptr as *mut libc::c_void, size as size_t);
        if nread == 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as ssize_t;
        }
        if nread == -(1 as libc::c_int) as libc::c_long {
            if *__errno_location() != 11 as libc::c_int {
                return -(1 as libc::c_int) as ssize_t;
            }
        } else {
            ptr = ptr.offset(nread as isize);
            size -= nread;
            totread += nread;
        }
        if size == 0 as libc::c_int as libc::c_long {
            return totread;
        }
        aeWait(fd, 1 as libc::c_int, wait);
        elapsed = mstime() - start;
        if elapsed >= timeout {
            *__errno_location() = 110 as libc::c_int;
            return -(1 as libc::c_int) as ssize_t;
        }
        remaining = timeout - elapsed;
    };
}
#[no_mangle]
pub unsafe extern "C" fn syncReadLine(
    mut fd: libc::c_int,
    mut ptr: *mut libc::c_char,
    mut size: ssize_t,
    mut timeout: libc::c_longlong,
) -> ssize_t {
    let mut nread: ssize_t = 0 as libc::c_int as ssize_t;
    size -= 1;
    while size != 0 {
        let mut c: libc::c_char = 0;
        if syncRead(fd, &mut c, 1 as libc::c_int as ssize_t, timeout)
            == -(1 as libc::c_int) as libc::c_long
        {
            return -(1 as libc::c_int) as ssize_t;
        }
        if c as libc::c_int == '\n' as i32 {
            *ptr = '\0' as i32 as libc::c_char;
            if nread != 0
                && *ptr.offset(-(1 as libc::c_int as isize)) as libc::c_int
                    == '\r' as i32
            {
                *ptr.offset(-(1 as libc::c_int as isize)) = '\0' as i32 as libc::c_char;
            }
            return nread;
        } else {
            let fresh0 = ptr;
            ptr = ptr.offset(1);
            *fresh0 = c;
            *ptr = '\0' as i32 as libc::c_char;
            nread += 1;
        }
        size -= 1;
    }
    return nread;
}

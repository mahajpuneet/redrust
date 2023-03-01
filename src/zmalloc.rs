extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn je_malloc(size: size_t) -> *mut libc::c_void;
    fn je_mallocx(size: size_t, flags: libc::c_int) -> *mut libc::c_void;
    fn je_realloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn je_calloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn je_mallctl(
        name: *const libc::c_char,
        oldp: *mut libc::c_void,
        oldlenp: *mut size_t,
        newp: *mut libc::c_void,
        newlen: size_t,
    ) -> libc::c_int;
    fn je_dallocx(ptr: *mut libc::c_void, flags: libc::c_int);
    fn je_free(ptr: *mut libc::c_void);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
}
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type ssize_t = __ssize_t;
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
pub type uint64_t = __uint64_t;
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
pub const memory_order_relaxed: memory_order = 0;
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
#[no_mangle]
pub unsafe extern "C" fn zlibc_free(mut ptr: *mut libc::c_void) {
    free(ptr);
}
static mut used_memory: size_t = 0 as libc::c_int as libc::c_ulong;
unsafe extern "C" fn zmalloc_default_oom(mut size: size_t) {
    fprintf(
        stderr,
        b"zmalloc: Out of memory trying to allocate %zu bytes\n\0" as *const u8
            as *const libc::c_char,
        size,
    );
    fflush(stderr);
    abort();
}
static mut zmalloc_oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()> = unsafe {
    Some(zmalloc_default_oom as unsafe extern "C" fn(size_t) -> ())
};
#[no_mangle]
pub unsafe extern "C" fn ztrymalloc_usable(
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = je_malloc(
        (if size > 0 as libc::c_int as libc::c_ulong {
            size
        } else {
            core::mem::size_of::<libc::c_long>() as libc::c_ulong
        })
            .wrapping_add(0 as libc::c_int as libc::c_ulong),
    );
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    size = je_malloc_usable_size(ptr);
    let fresh0 = &mut used_memory;
    let fresh1 = size;
    core::intrinsics::atomic_xadd_relaxed(fresh0, fresh1) + fresh1;
    if !usable.is_null() {
        *usable = size;
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrymalloc_usable(size, 0 as *mut size_t);
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ztrymalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrymalloc_usable(size, 0 as *mut size_t);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_usable(
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrymalloc_usable(size, usable);
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_no_tcache(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = je_mallocx(
        size.wrapping_add(0 as libc::c_int as libc::c_ulong),
        (-(1 as libc::c_int) + 2 as libc::c_int) << 8 as libc::c_int,
    );
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    let fresh2 = &mut used_memory;
    let fresh3 = je_malloc_usable_size(ptr);
    core::intrinsics::atomic_xadd_relaxed(fresh2, fresh3) + fresh3;
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zfree_no_tcache(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let fresh4 = &mut used_memory;
    let fresh5 = je_malloc_usable_size(ptr);
    core::intrinsics::atomic_xsub_relaxed(fresh4, fresh5) - fresh5;
    je_dallocx(ptr, (-(1 as libc::c_int) + 2 as libc::c_int) << 8 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ztrycalloc_usable(
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = je_calloc(
        1 as libc::c_int as size_t,
        (if size > 0 as libc::c_int as libc::c_ulong {
            size
        } else {
            core::mem::size_of::<libc::c_long>() as libc::c_ulong
        })
            .wrapping_add(0 as libc::c_int as libc::c_ulong),
    );
    if ptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    size = je_malloc_usable_size(ptr);
    let fresh6 = &mut used_memory;
    let fresh7 = size;
    core::intrinsics::atomic_xadd_relaxed(fresh6, fresh7) + fresh7;
    if !usable.is_null() {
        *usable = size;
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zcalloc_num(
    mut num: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    if size == 0 as libc::c_int as libc::c_ulong
        || num > (18446744073709551615 as libc::c_ulong).wrapping_div(size)
    {
        zmalloc_oom_handler
            .expect("non-null function pointer")(18446744073709551615 as libc::c_ulong);
        return 0 as *mut libc::c_void;
    }
    let mut ptr: *mut libc::c_void = ztrycalloc_usable(
        num.wrapping_mul(size),
        0 as *mut size_t,
    );
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(num.wrapping_mul(size));
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zcalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrycalloc_usable(size, 0 as *mut size_t);
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ztrycalloc(mut size: size_t) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrycalloc_usable(size, 0 as *mut size_t);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zcalloc_usable(
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = ztrycalloc_usable(size, usable);
    if ptr.is_null() {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ztryrealloc_usable(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    let mut oldsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if size == 0 as libc::c_int as libc::c_ulong && !ptr.is_null() {
        zfree(ptr);
        if !usable.is_null() {
            *usable = 0 as libc::c_int as size_t;
        }
        return 0 as *mut libc::c_void;
    }
    if ptr.is_null() {
        return ztrymalloc_usable(size, usable);
    }
    oldsize = je_malloc_usable_size(ptr);
    newptr = je_realloc(ptr, size);
    if newptr.is_null() {
        if !usable.is_null() {
            *usable = 0 as libc::c_int as size_t;
        }
        return 0 as *mut libc::c_void;
    }
    let fresh8 = &mut used_memory;
    let fresh9 = oldsize;
    core::intrinsics::atomic_xsub_relaxed(fresh8, fresh9) - fresh9;
    size = je_malloc_usable_size(newptr);
    let fresh10 = &mut used_memory;
    let fresh11 = size;
    core::intrinsics::atomic_xadd_relaxed(fresh10, fresh11) + fresh11;
    if !usable.is_null() {
        *usable = size;
    }
    return newptr;
}
#[no_mangle]
pub unsafe extern "C" fn zrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    ptr = ztryrealloc_usable(ptr, size, 0 as *mut size_t);
    if ptr.is_null() && size != 0 as libc::c_int as libc::c_ulong {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn ztryrealloc(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
) -> *mut libc::c_void {
    ptr = ztryrealloc_usable(ptr, size, 0 as *mut size_t);
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zrealloc_usable(
    mut ptr: *mut libc::c_void,
    mut size: size_t,
    mut usable: *mut size_t,
) -> *mut libc::c_void {
    ptr = ztryrealloc_usable(ptr, size, usable);
    if ptr.is_null() && size != 0 as libc::c_int as libc::c_ulong {
        zmalloc_oom_handler.expect("non-null function pointer")(size);
    }
    return ptr;
}
#[no_mangle]
pub unsafe extern "C" fn zfree(mut ptr: *mut libc::c_void) {
    if ptr.is_null() {
        return;
    }
    let fresh12 = &mut used_memory;
    let fresh13 = je_malloc_usable_size(ptr);
    core::intrinsics::atomic_xsub_relaxed(fresh12, fresh13) - fresh13;
    je_free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn zfree_usable(
    mut ptr: *mut libc::c_void,
    mut usable: *mut size_t,
) {
    if ptr.is_null() {
        return;
    }
    *usable = je_malloc_usable_size(ptr);
    let fresh14 = &mut used_memory;
    let fresh15 = *usable;
    core::intrinsics::atomic_xsub_relaxed(fresh14, fresh15) - fresh15;
    je_free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn zstrdup(mut s: *const libc::c_char) -> *mut libc::c_char {
    let mut l: size_t = (strlen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    let mut p: *mut libc::c_char = zmalloc(l) as *mut libc::c_char;
    memcpy(p as *mut libc::c_void, s as *const libc::c_void, l);
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_used_memory() -> size_t {
    let mut um: size_t = 0;
    um = core::intrinsics::atomic_load_relaxed(&mut used_memory);
    return um;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_set_oom_handler(
    mut oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()>,
) {
    zmalloc_oom_handler = oom_handler;
}
#[no_mangle]
pub unsafe extern "C" fn zmadvise_dontneed(mut ptr: *mut libc::c_void) {
    static mut page_size: size_t = 0 as libc::c_int as size_t;
    if page_size == 0 as libc::c_int as libc::c_ulong {
        page_size = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    }
    let mut page_size_mask: size_t = page_size
        .wrapping_sub(1 as libc::c_int as libc::c_ulong);
    let mut real_size: size_t = je_malloc_usable_size(ptr);
    if real_size < page_size {
        return;
    }
    let mut aligned_ptr: *mut libc::c_char = ((ptr as size_t)
        .wrapping_add(page_size_mask) & !page_size_mask) as *mut libc::c_char;
    real_size = (real_size as libc::c_ulong)
        .wrapping_sub(
            aligned_ptr.offset_from(ptr as *mut libc::c_char) as libc::c_long
                as libc::c_ulong,
        ) as size_t as size_t;
    if real_size >= page_size {
        madvise(
            aligned_ptr as *mut libc::c_void,
            real_size & !page_size_mask,
            4 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn get_proc_stat_ll(
    mut i: libc::c_int,
    mut res: *mut libc::c_longlong,
) -> libc::c_int {
    let mut buf: [libc::c_char; 4096] = [0; 4096];
    let mut fd: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut x: *mut libc::c_char = 0 as *mut libc::c_char;
    fd = open(
        b"/proc/self/stat\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if fd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    l = read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        (core::mem::size_of::<[libc::c_char; 4096]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) as libc::c_int;
    if l <= 0 as libc::c_int {
        close(fd);
        return 0 as libc::c_int;
    }
    close(fd);
    buf[l as usize] = '\0' as i32 as libc::c_char;
    if buf[(l - 1 as libc::c_int) as usize] as libc::c_int == '\n' as i32 {
        buf[(l - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    }
    p = strrchr(buf.as_mut_ptr(), ')' as i32);
    if p.is_null() {
        return 0 as libc::c_int;
    }
    p = p.offset(1);
    while *p as libc::c_int == ' ' as i32 {
        p = p.offset(1);
    }
    if *p as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    i -= 3 as libc::c_int;
    if i < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    while !p.is_null()
        && {
            let fresh16 = i;
            i = i - 1;
            fresh16 != 0
        }
    {
        p = strchr(p, ' ' as i32);
        if !p.is_null() {
            p = p.offset(1);
        } else {
            return 0 as libc::c_int
        }
    }
    x = strchr(p, ' ' as i32);
    if !x.is_null() {
        *x = '\0' as i32 as libc::c_char;
    }
    *res = strtoll(p, &mut x, 10 as libc::c_int);
    if *x as libc::c_int != '\0' as i32 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_get_rss() -> size_t {
    let mut page: libc::c_int = sysconf(_SC_PAGESIZE as libc::c_int) as libc::c_int;
    let mut rss: libc::c_longlong = 0;
    if get_proc_stat_ll(24 as libc::c_int, &mut rss) == 0 {
        return 0 as libc::c_int as size_t;
    }
    rss *= page as libc::c_longlong;
    return rss as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_get_allocator_info(
    mut allocated: *mut size_t,
    mut active: *mut size_t,
    mut resident: *mut size_t,
) -> libc::c_int {
    let mut epoch: uint64_t = 1 as libc::c_int as uint64_t;
    let mut sz: size_t = 0;
    *active = 0 as libc::c_int as size_t;
    *resident = *active;
    *allocated = *resident;
    sz = core::mem::size_of::<uint64_t>() as libc::c_ulong;
    je_mallctl(
        b"epoch\0" as *const u8 as *const libc::c_char,
        &mut epoch as *mut uint64_t as *mut libc::c_void,
        &mut sz,
        &mut epoch as *mut uint64_t as *mut libc::c_void,
        sz,
    );
    sz = core::mem::size_of::<size_t>() as libc::c_ulong;
    je_mallctl(
        b"stats.resident\0" as *const u8 as *const libc::c_char,
        resident as *mut libc::c_void,
        &mut sz,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    je_mallctl(
        b"stats.active\0" as *const u8 as *const libc::c_char,
        active as *mut libc::c_void,
        &mut sz,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    je_mallctl(
        b"stats.allocated\0" as *const u8 as *const libc::c_char,
        allocated as *mut libc::c_void,
        &mut sz,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn set_jemalloc_bg_thread(mut enable: libc::c_int) {
    let mut val: libc::c_char = (enable != 0) as libc::c_int as libc::c_char;
    je_mallctl(
        b"background_thread\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_void,
        0 as *mut size_t,
        &mut val as *mut libc::c_char as *mut libc::c_void,
        1 as libc::c_int as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn jemalloc_purge() -> libc::c_int {
    let mut tmp: [libc::c_char; 32] = [0; 32];
    let mut narenas: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut sz: size_t = core::mem::size_of::<libc::c_uint>() as libc::c_ulong;
    if je_mallctl(
        b"arenas.narenas\0" as *const u8 as *const libc::c_char,
        &mut narenas as *mut libc::c_uint as *mut libc::c_void,
        &mut sz,
        0 as *mut libc::c_void,
        0 as libc::c_int as size_t,
    ) == 0
    {
        sprintf(
            tmp.as_mut_ptr(),
            b"arena.%d.purge\0" as *const u8 as *const libc::c_char,
            narenas,
        );
        if je_mallctl(
            tmp.as_mut_ptr(),
            0 as *mut libc::c_void,
            0 as *mut size_t,
            0 as *mut libc::c_void,
            0 as libc::c_int as size_t,
        ) == 0
        {
            return 0 as libc::c_int;
        }
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_get_smap_bytes_by_field(
    mut field: *mut libc::c_char,
    mut pid: libc::c_long,
) -> size_t {
    let mut line: [libc::c_char; 1024] = [0; 1024];
    let mut bytes: size_t = 0 as libc::c_int as size_t;
    let mut flen: libc::c_int = strlen(field) as libc::c_int;
    let mut fp: *mut FILE = 0 as *mut FILE;
    if pid == -(1 as libc::c_int) as libc::c_long {
        fp = fopen(
            b"/proc/self/smaps\0" as *const u8 as *const libc::c_char,
            b"r\0" as *const u8 as *const libc::c_char,
        );
    } else {
        let mut filename: [libc::c_char; 128] = [0; 128];
        snprintf(
            filename.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
            b"/proc/%ld/smaps\0" as *const u8 as *const libc::c_char,
            pid,
        );
        fp = fopen(filename.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    }
    if fp.is_null() {
        return 0 as libc::c_int as size_t;
    }
    while !(fgets(
        line.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        if strncmp(line.as_mut_ptr(), field, flen as libc::c_ulong) == 0 as libc::c_int {
            let mut p: *mut libc::c_char = strchr(line.as_mut_ptr(), 'k' as i32);
            if !p.is_null() {
                *p = '\0' as i32 as libc::c_char;
                bytes = (bytes as libc::c_ulong)
                    .wrapping_add(
                        (strtol(
                            line.as_mut_ptr().offset(flen as isize),
                            0 as *mut *mut libc::c_char,
                            10 as libc::c_int,
                        ) * 1024 as libc::c_int as libc::c_long) as libc::c_ulong,
                    ) as size_t as size_t;
            }
        }
    }
    fclose(fp);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_get_private_dirty(mut pid: libc::c_long) -> size_t {
    return zmalloc_get_smap_bytes_by_field(
        b"Private_Dirty:\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        pid,
    );
}
#[no_mangle]
pub unsafe extern "C" fn zmalloc_get_memory_size() -> size_t {
    return (sysconf(_SC_PHYS_PAGES as libc::c_int) as size_t)
        .wrapping_mul(sysconf(_SC_PAGESIZE as libc::c_int) as size_t);
}

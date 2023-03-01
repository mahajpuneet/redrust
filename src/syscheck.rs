extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsfree(s: sds);
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn anetPipe(
        fds: *mut libc::c_int,
        read_flags: libc::c_int,
        write_flags: libc::c_int,
    ) -> libc::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> libc::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn fork() -> __pid_t;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn madvise(
        __addr: *mut libc::c_void,
        __len: size_t,
        __advice: libc::c_int,
    ) -> libc::c_int;
    fn mprotect(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
    ) -> libc::c_int;
    fn mmap(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
        __flags: libc::c_int,
        __fd: libc::c_int,
        __offset: __off64_t,
    ) -> *mut libc::c_void;
    fn munmap(__addr: *mut libc::c_void, __len: size_t) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __clockid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ssize_t = __ssize_t;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type sds = *mut libc::c_char;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct check {
    pub name: *const libc::c_char,
    pub check_fn: Option::<unsafe extern "C" fn(*mut sds) -> libc::c_int>,
}
pub const _SC_PAGESIZE: C2RustUnnamed_13 = 30;
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_12,
    pub c2rust_unnamed_0: C2RustUnnamed_11,
    pub c2rust_unnamed_1: C2RustUnnamed_10,
    pub c2rust_unnamed_2: C2RustUnnamed_9,
    pub c2rust_unnamed_3: C2RustUnnamed_8,
    pub c2rust_unnamed_4: C2RustUnnamed_7,
    pub c2rust_unnamed_5: C2RustUnnamed_6,
    pub c2rust_unnamed_6: C2RustUnnamed_5,
    pub c2rust_unnamed_7: C2RustUnnamed_4,
    pub c2rust_unnamed_8: C2RustUnnamed_3,
    pub c2rust_unnamed_9: C2RustUnnamed_2,
    pub c2rust_unnamed_10: C2RustUnnamed_1,
    pub c2rust_unnamed_11: C2RustUnnamed_0,
    pub c2rust_unnamed_12: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_10 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_11 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
pub const _SC_CLK_TCK: C2RustUnnamed_13 = 2;
pub type C2RustUnnamed_13 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_13 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_13 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_13 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_13 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_13 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_13 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_13 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_13 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_13 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_13 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_13 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_13 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_13 = 236;
pub const _SC_IPV6: C2RustUnnamed_13 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_13 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_13 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_13 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_13 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_13 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_13 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_13 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_13 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_13 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_13 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_13 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_13 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_13 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_13 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_13 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_13 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_13 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_13 = 182;
pub const _SC_TRACE: C2RustUnnamed_13 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_13 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_13 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_13 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_13 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_13 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_13 = 175;
pub const _SC_STREAMS: C2RustUnnamed_13 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_13 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_13 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_13 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_13 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_13 = 169;
pub const _SC_2_PBS: C2RustUnnamed_13 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_13 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_13 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_13 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_13 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_13 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_13 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_13 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_13 = 160;
pub const _SC_SPAWN: C2RustUnnamed_13 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_13 = 158;
pub const _SC_SHELL: C2RustUnnamed_13 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_13 = 156;
pub const _SC_REGEXP: C2RustUnnamed_13 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_13 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_13 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_13 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_13 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_13 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_13 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_13 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_13 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_13 = 146;
pub const _SC_PIPE: C2RustUnnamed_13 = 145;
pub const _SC_FIFO: C2RustUnnamed_13 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_13 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_13 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_13 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_13 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_13 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_13 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_13 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_13 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_13 = 135;
pub const _SC_BASE: C2RustUnnamed_13 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_13 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_13 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_13 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_13 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_13 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_13 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_13 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_13 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_13 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_13 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_13 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_13 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_13 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_13 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_13 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_13 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_13 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_13 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_13 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_13 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_13 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_13 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_13 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_13 = 110;
pub const _SC_NZERO: C2RustUnnamed_13 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_13 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_13 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_13 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_13 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_13 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_13 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_13 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_13 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_13 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_13 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_13 = 98;
pub const _SC_2_UPE: C2RustUnnamed_13 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_13 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_13 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_13 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_13 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_13 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_13 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_13 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_13 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_13 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_13 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_13 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_13 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_13 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_13 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_13 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_13 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_13 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_13 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_13 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_13 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_13 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_13 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_13 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_13 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_13 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_13 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_13 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_13 = 68;
pub const _SC_THREADS: C2RustUnnamed_13 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_13 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_13 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_13 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_13 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_13 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_13 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_13 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_13 = 60;
pub const _SC_SELECT: C2RustUnnamed_13 = 59;
pub const _SC_POLL: C2RustUnnamed_13 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_13 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_13 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_13 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_13 = 54;
pub const _SC_PII: C2RustUnnamed_13 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_13 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_13 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_13 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_13 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_13 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_13 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_13 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_13 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_13 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_13 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_13 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_13 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_13 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_13 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_13 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_13 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_13 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_13 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_13 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_13 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_13 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_13 = 31;
pub const _SC_VERSION: C2RustUnnamed_13 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_13 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_13 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_13 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_13 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_13 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_13 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_13 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_13 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_13 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_13 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_13 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_13 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_13 = 16;
pub const _SC_FSYNC: C2RustUnnamed_13 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_13 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_13 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_13 = 12;
pub const _SC_TIMERS: C2RustUnnamed_13 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_13 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_13 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_13 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_13 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_13 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_13 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_13 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_13 = 3;
pub const _SC_CHILD_MAX: C2RustUnnamed_13 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_13 = 0;
unsafe extern "C" fn read_sysfs_line(mut path: *mut libc::c_char) -> sds {
    let mut buf: [libc::c_char; 256] = [0; 256];
    let mut f: *mut FILE = fopen(path, b"r\0" as *const u8 as *const libc::c_char);
    if f.is_null() {
        return 0 as sds;
    }
    if (fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 256]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        fclose(f);
        return 0 as sds;
    }
    fclose(f);
    let mut res: sds = sdsnew(buf.as_mut_ptr());
    res = sdstrim(res, b" \n\0" as *const u8 as *const libc::c_char);
    return res;
}
unsafe extern "C" fn checkClocksource(mut error_msg: *mut sds) -> libc::c_int {
    let mut test_time_us: libc::c_ulong = 0;
    let mut system_hz: libc::c_ulong = 0;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    let mut start_us: libc::c_ulonglong = 0;
    let mut ru_start: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    let mut ru_end: rusage = rusage {
        ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
        ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
        c2rust_unnamed: C2RustUnnamed_12 { ru_maxrss: 0 },
        c2rust_unnamed_0: C2RustUnnamed_11 { ru_ixrss: 0 },
        c2rust_unnamed_1: C2RustUnnamed_10 { ru_idrss: 0 },
        c2rust_unnamed_2: C2RustUnnamed_9 { ru_isrss: 0 },
        c2rust_unnamed_3: C2RustUnnamed_8 { ru_minflt: 0 },
        c2rust_unnamed_4: C2RustUnnamed_7 { ru_majflt: 0 },
        c2rust_unnamed_5: C2RustUnnamed_6 { ru_nswap: 0 },
        c2rust_unnamed_6: C2RustUnnamed_5 { ru_inblock: 0 },
        c2rust_unnamed_7: C2RustUnnamed_4 { ru_oublock: 0 },
        c2rust_unnamed_8: C2RustUnnamed_3 { ru_msgsnd: 0 },
        c2rust_unnamed_9: C2RustUnnamed_2 { ru_msgrcv: 0 },
        c2rust_unnamed_10: C2RustUnnamed_1 { ru_nsignals: 0 },
        c2rust_unnamed_11: C2RustUnnamed_0 { ru_nvcsw: 0 },
        c2rust_unnamed_12: C2RustUnnamed { ru_nivcsw: 0 },
    };
    system_hz = sysconf(_SC_CLK_TCK as libc::c_int) as libc::c_ulong;
    if getrusage(RUSAGE_SELF, &mut ru_start) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if clock_gettime(1 as libc::c_int, &mut ts) < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    start_us = (ts.tv_sec * 1000000 as libc::c_int as libc::c_long
        + ts.tv_nsec / 1000 as libc::c_int as libc::c_long) as libc::c_ulonglong;
    test_time_us = ((5 as libc::c_int * 1000000 as libc::c_int) as libc::c_ulong)
        .wrapping_div(system_hz);
    loop {
        let mut d: libc::c_ulonglong = 0;
        if clock_gettime(1 as libc::c_int, &mut ts) < 0 as libc::c_int {
            return 0 as libc::c_int;
        }
        d = ((ts.tv_sec * 1000000 as libc::c_int as libc::c_long
            + ts.tv_nsec / 1000 as libc::c_int as libc::c_long) as libc::c_ulonglong)
            .wrapping_sub(start_us);
        if d >= test_time_us as libc::c_ulonglong {
            break;
        }
    }
    if getrusage(RUSAGE_SELF, &mut ru_end) != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut stime_us: libc::c_longlong = (ru_end.ru_stime.tv_sec
        * 1000000 as libc::c_int as libc::c_long + ru_end.ru_stime.tv_usec
        - (ru_start.ru_stime.tv_sec * 1000000 as libc::c_int as libc::c_long
            + ru_start.ru_stime.tv_usec)) as libc::c_longlong;
    let mut utime_us: libc::c_longlong = (ru_end.ru_utime.tv_sec
        * 1000000 as libc::c_int as libc::c_long + ru_end.ru_utime.tv_usec
        - (ru_start.ru_utime.tv_sec * 1000000 as libc::c_int as libc::c_long
            + ru_start.ru_utime.tv_usec)) as libc::c_longlong;
    if stime_us * 10 as libc::c_int as libc::c_longlong > stime_us + utime_us {
        let mut avail: sds = read_sysfs_line(
            b"/sys/devices/system/clocksource/clocksource0/available_clocksource\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        let mut curr: sds = read_sysfs_line(
            b"/sys/devices/system/clocksource/clocksource0/current_clocksource\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        *error_msg = sdscatprintf(
            sdsempty(),
            b"Slow system clocksource detected. This can result in degraded performance. Consider changing the system's clocksource. Current clocksource: %s. Available clocksources: %s. For example: run the command 'echo tsc > /sys/devices/system/clocksource/clocksource0/current_clocksource' as root. To permanently change the system's clocksource you'll need to set the 'clocksource=' kernel command line parameter.\0"
                as *const u8 as *const libc::c_char,
            if !curr.is_null() {
                curr as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !avail.is_null() {
                avail as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
        );
        sdsfree(avail);
        sdsfree(curr);
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkXenClocksource(mut error_msg: *mut sds) -> libc::c_int {
    let mut curr: sds = read_sysfs_line(
        b"/sys/devices/system/clocksource/clocksource0/current_clocksource\0"
            as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut res: libc::c_int = 1 as libc::c_int;
    if curr.is_null() {
        res = 0 as libc::c_int;
    } else if strcmp(
        curr as *const libc::c_char,
        b"xen\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        *error_msg = sdsnew(
            b"Your system is configured to use the 'xen' clocksource which might lead to degraded performance. Check the result of the [slow-clocksource] system check: run 'redis-server --check-system' to check if the system's clocksource isn't degrading performance.\0"
                as *const u8 as *const libc::c_char,
        );
        res = -(1 as libc::c_int);
    }
    sdsfree(curr);
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn checkOvercommit(mut error_msg: *mut sds) -> libc::c_int {
    let mut fp: *mut FILE = fopen(
        b"/proc/sys/vm/overcommit_memory\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut buf: [libc::c_char; 64] = [0; 64];
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    if (fgets(buf.as_mut_ptr(), 64 as libc::c_int, fp)).is_null() {
        fclose(fp);
        return 0 as libc::c_int;
    }
    fclose(fp);
    if strtol(buf.as_mut_ptr(), 0 as *mut *mut libc::c_char, 10 as libc::c_int)
        != 1 as libc::c_int as libc::c_long
    {
        *error_msg = sdsnew(
            b"Memory overcommit must be enabled! Without it, a background save or replication may fail under low memory condition. Being disabled, it can can also cause failures without low memory condition, see https://github.com/jemalloc/jemalloc/issues/1328. To fix this issue add 'vm.overcommit_memory = 1' to /etc/sysctl.conf and then reboot or run the command 'sysctl vm.overcommit_memory=1' for this to take effect.\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkTHPEnabled(mut error_msg: *mut sds) -> libc::c_int {
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    let mut fp: *mut FILE = fopen(
        b"/sys/kernel/mm/transparent_hugepage/enabled\0" as *const u8
            as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    if (fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        fclose(fp);
        return 0 as libc::c_int;
    }
    fclose(fp);
    if !(strstr(buf.as_mut_ptr(), b"[always]\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        *error_msg = sdsnew(
            b"You have Transparent Huge Pages (THP) support enabled in your kernel. This will create latency and memory usage issues with Redis. To fix this issue run the command 'echo madvise > /sys/kernel/mm/transparent_hugepage/enabled' as root, and add it to your /etc/rc.local in order to retain the setting after a reboot. Redis must be restarted after THP is disabled (set to 'madvise' or 'never').\0"
                as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    } else {
        return 1 as libc::c_int
    };
}
unsafe extern "C" fn smapsGetSharedDirty(mut addr: libc::c_ulong) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut in_mapping: libc::c_int = 0 as libc::c_int;
    let mut val: libc::c_int = -(1 as libc::c_int);
    let mut from: libc::c_ulong = 0;
    let mut to: libc::c_ulong = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(
        b"/proc/self/smaps\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return -(1 as libc::c_int);
    }
    while !(fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        ret = sscanf(
            buf.as_mut_ptr(),
            b"%lx-%lx\0" as *const u8 as *const libc::c_char,
            &mut from as *mut libc::c_ulong,
            &mut to as *mut libc::c_ulong,
        );
        if ret == 2 as libc::c_int {
            in_mapping = (from <= addr && addr < to) as libc::c_int;
        }
        if !(in_mapping != 0
            && memcmp(
                buf.as_mut_ptr() as *const libc::c_void,
                b"Shared_Dirty:\0" as *const u8 as *const libc::c_char
                    as *const libc::c_void,
                13 as libc::c_int as libc::c_ulong,
            ) == 0)
        {
            continue;
        }
        sscanf(
            buf.as_mut_ptr(),
            b"%*s %d\0" as *const u8 as *const libc::c_char,
            &mut val as *mut libc::c_int,
        );
        break;
    }
    fclose(f);
    return val;
}
#[no_mangle]
pub unsafe extern "C" fn checkLinuxMadvFreeForkBug(
    mut error_msg: *mut sds,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut pipefd: [libc::c_int; 2] = [-(1 as libc::c_int), -(1 as libc::c_int)];
    let mut pid: pid_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_int = 1 as libc::c_int;
    let mut page_size: libc::c_long = sysconf(_SC_PAGESIZE as libc::c_int);
    let mut map_size: libc::c_long = 3 as libc::c_int as libc::c_long * page_size;
    p = mmap(
        0 as *mut libc::c_void,
        map_size as size_t,
        0x1 as libc::c_int,
        0x20 as libc::c_int | 0x2 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int as __off64_t,
    ) as *mut libc::c_char;
    if p == -(1 as libc::c_int) as *mut libc::c_void as *mut libc::c_char {
        return 0 as libc::c_int;
    }
    q = p.offset(page_size as isize);
    ret = mprotect(
        q as *mut libc::c_void,
        page_size as size_t,
        0x1 as libc::c_int | 0x2 as libc::c_int,
    );
    if ret < 0 as libc::c_int {
        res = 0 as libc::c_int;
    } else {
        core::ptr::write_volatile(
            (q as *mut libc::c_char),
            0 as libc::c_int as libc::c_char,
        );
        ret = madvise(q as *mut libc::c_void, page_size as size_t, 8 as libc::c_int);
        if ret < 0 as libc::c_int {
            if !(*__errno_location() == 22 as libc::c_int) {
                res = 0 as libc::c_int;
            }
        } else {
            core::ptr::write_volatile(
                (q as *mut libc::c_char),
                0 as libc::c_int as libc::c_char,
            );
            ret = anetPipe(pipefd.as_mut_ptr(), 0 as libc::c_int, 0 as libc::c_int);
            if ret < 0 as libc::c_int {
                res = 0 as libc::c_int;
            } else {
                pid = fork();
                if pid < 0 as libc::c_int {
                    res = 0 as libc::c_int;
                } else if pid == 0 {
                    ret = smapsGetSharedDirty(q as libc::c_ulong);
                    if ret == 0 {
                        res = -(1 as libc::c_int);
                    } else if ret == -(1 as libc::c_int) {
                        res = 0 as libc::c_int;
                    }
                    ret = write(
                        pipefd[1 as libc::c_int as usize],
                        &mut res as *mut libc::c_int as *const libc::c_void,
                        core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) as libc::c_int;
                    exit(0 as libc::c_int);
                } else {
                    ret = read(
                        pipefd[0 as libc::c_int as usize],
                        &mut res as *mut libc::c_int as *mut libc::c_void,
                        core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ) as libc::c_int;
                    if ret < 0 as libc::c_int {
                        res = 0 as libc::c_int;
                    }
                    waitpid(pid, 0 as *mut libc::c_int, 0 as libc::c_int);
                }
            }
        }
    }
    if pipefd[0 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(pipefd[0 as libc::c_int as usize]);
    }
    if pipefd[1 as libc::c_int as usize] != -(1 as libc::c_int) {
        close(pipefd[1 as libc::c_int as usize]);
    }
    if !p.is_null() {
        munmap(p as *mut libc::c_void, map_size as size_t);
    }
    if res == -(1 as libc::c_int) {
        *error_msg = sdsnew(
            b"Your kernel has a bug that could lead to data corruption during background save. Please upgrade to the latest stable kernel.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return res;
}
#[no_mangle]
pub static mut checks: [check; 6] = unsafe {
    [
        {
            let mut init = check {
                name: b"slow-clocksource\0" as *const u8 as *const libc::c_char,
                check_fn: Some(
                    checkClocksource as unsafe extern "C" fn(*mut sds) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = check {
                name: b"xen-clocksource\0" as *const u8 as *const libc::c_char,
                check_fn: Some(
                    checkXenClocksource as unsafe extern "C" fn(*mut sds) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = check {
                name: b"overcommit\0" as *const u8 as *const libc::c_char,
                check_fn: Some(
                    checkOvercommit as unsafe extern "C" fn(*mut sds) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = check {
                name: b"THP\0" as *const u8 as *const libc::c_char,
                check_fn: Some(
                    checkTHPEnabled as unsafe extern "C" fn(*mut sds) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = check {
                name: b"madvise-free-fork-bug\0" as *const u8 as *const libc::c_char,
                check_fn: Some(
                    checkLinuxMadvFreeForkBug
                        as unsafe extern "C" fn(*mut sds) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = check {
                name: 0 as *const libc::c_char,
                check_fn: None,
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn syscheck() -> libc::c_int {
    let mut cur_check: *mut check = checks.as_mut_ptr();
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut err_msg: sds = 0 as sds;
    while ((*cur_check).check_fn).is_some() {
        let mut res: libc::c_int = ((*cur_check).check_fn)
            .expect("non-null function pointer")(&mut err_msg);
        printf(b"[%s]...\0" as *const u8 as *const libc::c_char, (*cur_check).name);
        if res == 0 as libc::c_int {
            printf(b"skipped\n\0" as *const u8 as *const libc::c_char);
        } else if res == 1 as libc::c_int {
            printf(b"OK\n\0" as *const u8 as *const libc::c_char);
        } else {
            printf(b"WARNING:\n\0" as *const u8 as *const libc::c_char);
            printf(b"%s\n\0" as *const u8 as *const libc::c_char, err_msg);
            sdsfree(err_msg);
            ret = 0 as libc::c_int;
        }
        cur_check = cur_check.offset(1);
    }
    return ret;
}

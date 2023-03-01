extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn random() -> libc::c_long;
    fn rand() -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn genrand64_int64() -> libc::c_ulonglong;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn ztrycalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn siphash(in_0: *const uint8_t, inlen: size_t, k: *const uint8_t) -> uint64_t;
    fn siphash_nocase(
        in_0: *const uint8_t,
        inlen: size_t,
        k: *const uint8_t,
    ) -> uint64_t;
}
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type int16_t = __int16_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
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
pub struct dictIterator {
    pub d: *mut dict,
    pub index: libc::c_long,
    pub table: libc::c_int,
    pub safe: libc::c_int,
    pub entry: *mut dictEntry,
    pub nextEntry: *mut dictEntry,
    pub fingerprint: libc::c_ulonglong,
}
pub type dictScanFunction = unsafe extern "C" fn(
    *mut libc::c_void,
    *const dictEntry,
) -> ();
pub type dictScanBucketFunction = unsafe extern "C" fn(
    *mut dict,
    *mut *mut dictEntry,
) -> ();
pub type dictResizeEnable = libc::c_uint;
pub const DICT_RESIZE_FORBID: dictResizeEnable = 2;
pub const DICT_RESIZE_AVOID: dictResizeEnable = 1;
pub const DICT_RESIZE_ENABLE: dictResizeEnable = 0;
static mut dict_can_resize: dictResizeEnable = DICT_RESIZE_ENABLE;
static mut dict_force_resize_ratio: libc::c_uint = 5 as libc::c_int as libc::c_uint;
static mut dict_hash_function_seed: [uint8_t; 16] = [0; 16];
#[no_mangle]
pub unsafe extern "C" fn dictSetHashFunctionSeed(mut seed: *mut uint8_t) {
    memcpy(
        dict_hash_function_seed.as_mut_ptr() as *mut libc::c_void,
        seed as *const libc::c_void,
        core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dictGetHashFunctionSeed() -> *mut uint8_t {
    return dict_hash_function_seed.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn dictGenHashFunction(
    mut key: *const libc::c_void,
    mut len: size_t,
) -> uint64_t {
    return siphash(key as *const uint8_t, len, dict_hash_function_seed.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn dictGenCaseHashFunction(
    mut buf: *const libc::c_uchar,
    mut len: size_t,
) -> uint64_t {
    return siphash_nocase(buf, len, dict_hash_function_seed.as_mut_ptr());
}
unsafe extern "C" fn _dictReset(mut d: *mut dict, mut htidx: libc::c_int) {
    (*d).ht_table[htidx as usize] = 0 as *mut *mut dictEntry;
    (*d).ht_size_exp[htidx as usize] = -(1 as libc::c_int) as libc::c_schar;
    (*d).ht_used[htidx as usize] = 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn dictCreate(mut type_0: *mut dictType) -> *mut dict {
    let mut d: *mut dict = zmalloc(core::mem::size_of::<dict>() as libc::c_ulong)
        as *mut dict;
    _dictInit(d, type_0);
    return d;
}
unsafe extern "C" fn _dictInit(
    mut d: *mut dict,
    mut type_0: *mut dictType,
) -> libc::c_int {
    _dictReset(d, 0 as libc::c_int);
    _dictReset(d, 1 as libc::c_int);
    (*d).type_0 = type_0;
    (*d).rehashidx = -(1 as libc::c_int) as libc::c_long;
    (*d).pauserehash = 0 as libc::c_int as int16_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictResize(mut d: *mut dict) -> libc::c_int {
    let mut minimal: libc::c_ulong = 0;
    if dict_can_resize as libc::c_uint
        != DICT_RESIZE_ENABLE as libc::c_int as libc::c_uint
        || (*d).rehashidx != -(1 as libc::c_int) as libc::c_long
    {
        return 1 as libc::c_int;
    }
    minimal = (*d).ht_used[0 as libc::c_int as usize];
    if minimal < ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong {
        minimal = ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
    }
    return dictExpand(d, minimal);
}
#[no_mangle]
pub unsafe extern "C" fn _dictExpand(
    mut d: *mut dict,
    mut size: libc::c_ulong,
    mut malloc_failed: *mut libc::c_int,
) -> libc::c_int {
    if !malloc_failed.is_null() {
        *malloc_failed = 0 as libc::c_int;
    }
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long
        || (*d).ht_used[0 as libc::c_int as usize] > size
    {
        return 1 as libc::c_int;
    }
    let mut new_ht_table: *mut *mut dictEntry = 0 as *mut *mut dictEntry;
    let mut new_ht_used: libc::c_ulong = 0;
    let mut new_ht_size_exp: libc::c_schar = _dictNextExp(size);
    let mut newsize: size_t = (1 as libc::c_ulong) << new_ht_size_exp as libc::c_int;
    if newsize < size
        || newsize
            .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong)
            < newsize
    {
        return 1 as libc::c_int;
    }
    if new_ht_size_exp as libc::c_int
        == (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if !malloc_failed.is_null() {
        new_ht_table = ztrycalloc(
            newsize
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        ) as *mut *mut dictEntry;
        *malloc_failed = (new_ht_table == 0 as *mut libc::c_void as *mut *mut dictEntry)
            as libc::c_int;
        if *malloc_failed != 0 {
            return 1 as libc::c_int;
        }
    } else {
        new_ht_table = zcalloc(
            newsize
                .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        ) as *mut *mut dictEntry;
    }
    new_ht_used = 0 as libc::c_int as libc::c_ulong;
    if ((*d).ht_table[0 as libc::c_int as usize]).is_null() {
        (*d).ht_size_exp[0 as libc::c_int as usize] = new_ht_size_exp;
        (*d).ht_used[0 as libc::c_int as usize] = new_ht_used;
        (*d).ht_table[0 as libc::c_int as usize] = new_ht_table;
        return 0 as libc::c_int;
    }
    (*d).ht_size_exp[1 as libc::c_int as usize] = new_ht_size_exp;
    (*d).ht_used[1 as libc::c_int as usize] = new_ht_used;
    (*d).ht_table[1 as libc::c_int as usize] = new_ht_table;
    (*d).rehashidx = 0 as libc::c_int as libc::c_long;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictExpand(
    mut d: *mut dict,
    mut size: libc::c_ulong,
) -> libc::c_int {
    return _dictExpand(d, size, 0 as *mut libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dictTryExpand(
    mut d: *mut dict,
    mut size: libc::c_ulong,
) -> libc::c_int {
    let mut malloc_failed: libc::c_int = 0;
    _dictExpand(d, size, &mut malloc_failed);
    return if malloc_failed != 0 { 1 as libc::c_int } else { 0 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn dictRehash(
    mut d: *mut dict,
    mut n: libc::c_int,
) -> libc::c_int {
    let mut empty_visits: libc::c_int = n * 10 as libc::c_int;
    if dict_can_resize as libc::c_uint
        == DICT_RESIZE_FORBID as libc::c_int as libc::c_uint
        || !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long)
    {
        return 0 as libc::c_int;
    }
    if dict_can_resize as libc::c_uint
        == DICT_RESIZE_AVOID as libc::c_int as libc::c_uint
        && (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
        })
            .wrapping_div(
                (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                }),
            ) < dict_force_resize_ratio as libc::c_ulong
    {
        return 0 as libc::c_int;
    }
    loop {
        let fresh0 = n;
        n = n - 1;
        if !(fresh0 != 0
            && (*d).ht_used[0 as libc::c_int as usize]
                != 0 as libc::c_int as libc::c_ulong)
        {
            break;
        }
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut nextde: *mut dictEntry = 0 as *mut dictEntry;
        if ((if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        }) > (*d).rehashidx as libc::c_ulong) as libc::c_int as libc::c_long != 0
        {} else {
            _serverAssert(
                b"DICTHT_SIZE(d->ht_size_exp[0]) > (unsigned long)d->rehashidx\0"
                    as *const u8 as *const libc::c_char,
                b"dict.c\0" as *const u8 as *const libc::c_char,
                225 as libc::c_int,
            );
            unreachable!();
        };
        while (*((*d).ht_table[0 as libc::c_int as usize])
            .offset((*d).rehashidx as isize))
            .is_null()
        {
            (*d).rehashidx += 1;
            empty_visits -= 1;
            if empty_visits == 0 as libc::c_int {
                return 1 as libc::c_int;
            }
        }
        de = *((*d).ht_table[0 as libc::c_int as usize]).offset((*d).rehashidx as isize);
        while !de.is_null() {
            let mut h: uint64_t = 0;
            nextde = (*de).next;
            h = ((*(*d).type_0).hashFunction)
                .expect("non-null function pointer")((*de).key)
                & (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                    })
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                });
            (*de).next = *((*d).ht_table[1 as libc::c_int as usize]).offset(h as isize);
            let ref mut fresh1 = *((*d).ht_table[1 as libc::c_int as usize])
                .offset(h as isize);
            *fresh1 = de;
            (*d)
                .ht_used[0 as libc::c_int
                as usize] = ((*d).ht_used[0 as libc::c_int as usize]).wrapping_sub(1);
            (*d)
                .ht_used[1 as libc::c_int
                as usize] = ((*d).ht_used[1 as libc::c_int as usize]).wrapping_add(1);
            de = nextde;
        }
        let ref mut fresh2 = *((*d).ht_table[0 as libc::c_int as usize])
            .offset((*d).rehashidx as isize);
        *fresh2 = 0 as *mut dictEntry;
        (*d).rehashidx += 1;
    }
    if (*d).ht_used[0 as libc::c_int as usize] == 0 as libc::c_int as libc::c_ulong {
        zfree((*d).ht_table[0 as libc::c_int as usize] as *mut libc::c_void);
        (*d)
            .ht_table[0 as libc::c_int
            as usize] = (*d).ht_table[1 as libc::c_int as usize];
        (*d)
            .ht_used[0 as libc::c_int
            as usize] = (*d).ht_used[1 as libc::c_int as usize];
        (*d)
            .ht_size_exp[0 as libc::c_int
            as usize] = (*d).ht_size_exp[1 as libc::c_int as usize];
        _dictReset(d, 1 as libc::c_int);
        (*d).rehashidx = -(1 as libc::c_int) as libc::c_long;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn timeInMilliseconds() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    return tv.tv_sec as libc::c_longlong * 1000 as libc::c_int as libc::c_longlong
        + (tv.tv_usec / 1000 as libc::c_int as libc::c_long) as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn dictRehashMilliseconds(
    mut d: *mut dict,
    mut ms: libc::c_int,
) -> libc::c_int {
    if (*d).pauserehash as libc::c_int > 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut start: libc::c_longlong = timeInMilliseconds();
    let mut rehashes: libc::c_int = 0 as libc::c_int;
    while dictRehash(d, 100 as libc::c_int) != 0 {
        rehashes += 100 as libc::c_int;
        if timeInMilliseconds() - start > ms as libc::c_longlong {
            break;
        }
    }
    return rehashes;
}
unsafe extern "C" fn _dictRehashStep(mut d: *mut dict) {
    if (*d).pauserehash as libc::c_int == 0 as libc::c_int {
        dictRehash(d, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dictAdd(
    mut d: *mut dict,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut entry: *mut dictEntry = dictAddRaw(d, key, 0 as *mut *mut dictEntry);
    if entry.is_null() {
        return 1 as libc::c_int;
    }
    if ((*(*d).type_0).valDup).is_some() {
        (*entry)
            .v
            .val = ((*(*d).type_0).valDup).expect("non-null function pointer")(d, val);
    } else {
        (*entry).v.val = val;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictAddRaw(
    mut d: *mut dict,
    mut key: *mut libc::c_void,
    mut existing: *mut *mut dictEntry,
) -> *mut dictEntry {
    let mut index: libc::c_long = 0;
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut htidx: libc::c_int = 0;
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        _dictRehashStep(d);
    }
    index = _dictKeyIndex(
        d,
        key,
        ((*(*d).type_0).hashFunction).expect("non-null function pointer")(key),
        existing,
    );
    if index == -(1 as libc::c_int) as libc::c_long {
        return 0 as *mut dictEntry;
    }
    htidx = if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut metasize: size_t = if ((*(*d).type_0).dictEntryMetadataBytes).is_some() {
        ((*(*d).type_0).dictEntryMetadataBytes).expect("non-null function pointer")(d)
    } else {
        0 as libc::c_int as libc::c_ulong
    };
    entry = zmalloc(
        (core::mem::size_of::<dictEntry>() as libc::c_ulong).wrapping_add(metasize),
    ) as *mut dictEntry;
    if metasize > 0 as libc::c_int as libc::c_ulong {
        memset(
            &mut (*entry).metadata as *mut [*mut libc::c_void; 0] as *mut libc::c_void,
            0 as libc::c_int,
            metasize,
        );
    }
    (*entry).next = *((*d).ht_table[htidx as usize]).offset(index as isize);
    let ref mut fresh3 = *((*d).ht_table[htidx as usize]).offset(index as isize);
    *fresh3 = entry;
    (*d).ht_used[htidx as usize] = ((*d).ht_used[htidx as usize]).wrapping_add(1);
    if ((*(*d).type_0).keyDup).is_some() {
        (*entry)
            .key = ((*(*d).type_0).keyDup).expect("non-null function pointer")(d, key);
    } else {
        (*entry).key = key;
    }
    return entry;
}
#[no_mangle]
pub unsafe extern "C" fn dictReplace(
    mut d: *mut dict,
    mut key: *mut libc::c_void,
    mut val: *mut libc::c_void,
) -> libc::c_int {
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut existing: *mut dictEntry = 0 as *mut dictEntry;
    let mut auxentry: dictEntry = dictEntry {
        key: 0 as *mut libc::c_void,
        v: C2RustUnnamed {
            val: 0 as *mut libc::c_void,
        },
        next: 0 as *mut dictEntry,
        metadata: [],
    };
    entry = dictAddRaw(d, key, &mut existing);
    if !entry.is_null() {
        if ((*(*d).type_0).valDup).is_some() {
            (*entry)
                .v
                .val = ((*(*d).type_0).valDup)
                .expect("non-null function pointer")(d, val);
        } else {
            (*entry).v.val = val;
        }
        return 1 as libc::c_int;
    }
    auxentry = *existing;
    if ((*(*d).type_0).valDup).is_some() {
        (*existing)
            .v
            .val = ((*(*d).type_0).valDup).expect("non-null function pointer")(d, val);
    } else {
        (*existing).v.val = val;
    }
    if ((*(*d).type_0).valDestructor).is_some() {
        ((*(*d).type_0).valDestructor)
            .expect("non-null function pointer")(d, auxentry.v.val);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictAddOrFind(
    mut d: *mut dict,
    mut key: *mut libc::c_void,
) -> *mut dictEntry {
    let mut entry: *mut dictEntry = 0 as *mut dictEntry;
    let mut existing: *mut dictEntry = 0 as *mut dictEntry;
    entry = dictAddRaw(d, key, &mut existing);
    return if !entry.is_null() { entry } else { existing };
}
unsafe extern "C" fn dictGenericDelete(
    mut d: *mut dict,
    mut key: *const libc::c_void,
    mut nofree: libc::c_int,
) -> *mut dictEntry {
    let mut h: uint64_t = 0;
    let mut idx: uint64_t = 0;
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    let mut prevHe: *mut dictEntry = 0 as *mut dictEntry;
    let mut table: libc::c_int = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut dictEntry;
    }
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        _dictRehashStep(d);
    }
    h = ((*(*d).type_0).hashFunction).expect("non-null function pointer")(key);
    table = 0 as libc::c_int;
    while table <= 1 as libc::c_int {
        idx = h
            & (if (*d).ht_size_exp[table as usize] as libc::c_int == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if (*d).ht_size_exp[table as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[table as usize] as libc::c_int
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            });
        he = *((*d).ht_table[table as usize]).offset(idx as isize);
        prevHe = 0 as *mut dictEntry;
        while !he.is_null() {
            if key == (*he).key as *const libc::c_void
                || (if ((*(*d).type_0).keyCompare).is_some() {
                    ((*(*d).type_0).keyCompare)
                        .expect("non-null function pointer")(d, key, (*he).key)
                } else {
                    (key == (*he).key as *const libc::c_void) as libc::c_int
                }) != 0
            {
                if !prevHe.is_null() {
                    (*prevHe).next = (*he).next;
                } else {
                    let ref mut fresh4 = *((*d).ht_table[table as usize])
                        .offset(idx as isize);
                    *fresh4 = (*he).next;
                }
                if nofree == 0 {
                    dictFreeUnlinkedEntry(d, he);
                }
                (*d)
                    .ht_used[table
                    as usize] = ((*d).ht_used[table as usize]).wrapping_sub(1);
                return he;
            }
            prevHe = he;
            he = (*he).next;
        }
        if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        table += 1;
    }
    return 0 as *mut dictEntry;
}
#[no_mangle]
pub unsafe extern "C" fn dictDelete(
    mut ht: *mut dict,
    mut key: *const libc::c_void,
) -> libc::c_int {
    return if !(dictGenericDelete(ht, key, 0 as libc::c_int)).is_null() {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn dictUnlink(
    mut d: *mut dict,
    mut key: *const libc::c_void,
) -> *mut dictEntry {
    return dictGenericDelete(d, key, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn dictFreeUnlinkedEntry(
    mut d: *mut dict,
    mut he: *mut dictEntry,
) {
    if he.is_null() {
        return;
    }
    if ((*(*d).type_0).keyDestructor).is_some() {
        ((*(*d).type_0).keyDestructor).expect("non-null function pointer")(d, (*he).key);
    }
    if ((*(*d).type_0).valDestructor).is_some() {
        ((*(*d).type_0).valDestructor)
            .expect("non-null function pointer")(d, (*he).v.val);
    }
    zfree(he as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn _dictClear(
    mut d: *mut dict,
    mut htidx: libc::c_int,
    mut callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
) -> libc::c_int {
    let mut i: libc::c_ulong = 0;
    i = 0 as libc::c_int as libc::c_ulong;
    while i
        < (if (*d).ht_size_exp[htidx as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[htidx as usize] as libc::c_int
        }) && (*d).ht_used[htidx as usize] > 0 as libc::c_int as libc::c_ulong
    {
        let mut he: *mut dictEntry = 0 as *mut dictEntry;
        let mut nextHe: *mut dictEntry = 0 as *mut dictEntry;
        if callback.is_some()
            && i & 65535 as libc::c_int as libc::c_ulong
                == 0 as libc::c_int as libc::c_ulong
        {
            callback.expect("non-null function pointer")(d);
        }
        he = *((*d).ht_table[htidx as usize]).offset(i as isize);
        if !he.is_null() {
            while !he.is_null() {
                nextHe = (*he).next;
                if ((*(*d).type_0).keyDestructor).is_some() {
                    ((*(*d).type_0).keyDestructor)
                        .expect("non-null function pointer")(d, (*he).key);
                }
                if ((*(*d).type_0).valDestructor).is_some() {
                    ((*(*d).type_0).valDestructor)
                        .expect("non-null function pointer")(d, (*he).v.val);
                }
                zfree(he as *mut libc::c_void);
                (*d)
                    .ht_used[htidx
                    as usize] = ((*d).ht_used[htidx as usize]).wrapping_sub(1);
                he = nextHe;
            }
        }
        i = i.wrapping_add(1);
    }
    zfree((*d).ht_table[htidx as usize] as *mut libc::c_void);
    _dictReset(d, htidx);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictRelease(mut d: *mut dict) {
    _dictClear(d, 0 as libc::c_int, None);
    _dictClear(d, 1 as libc::c_int, None);
    zfree(d as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dictFind(
    mut d: *mut dict,
    mut key: *const libc::c_void,
) -> *mut dictEntry {
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    let mut h: uint64_t = 0;
    let mut idx: uint64_t = 0;
    let mut table: uint64_t = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut dictEntry;
    }
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        _dictRehashStep(d);
    }
    h = ((*(*d).type_0).hashFunction).expect("non-null function pointer")(key);
    table = 0 as libc::c_int as uint64_t;
    while table <= 1 as libc::c_int as libc::c_ulong {
        idx = h
            & (if (*d).ht_size_exp[table as usize] as libc::c_int == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if (*d).ht_size_exp[table as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[table as usize] as libc::c_int
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            });
        he = *((*d).ht_table[table as usize]).offset(idx as isize);
        while !he.is_null() {
            if key == (*he).key as *const libc::c_void
                || (if ((*(*d).type_0).keyCompare).is_some() {
                    ((*(*d).type_0).keyCompare)
                        .expect("non-null function pointer")(d, key, (*he).key)
                } else {
                    (key == (*he).key as *const libc::c_void) as libc::c_int
                }) != 0
            {
                return he;
            }
            he = (*he).next;
        }
        if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
            return 0 as *mut dictEntry;
        }
        table = table.wrapping_add(1);
    }
    return 0 as *mut dictEntry;
}
#[no_mangle]
pub unsafe extern "C" fn dictFetchValue(
    mut d: *mut dict,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    he = dictFind(d, key);
    return if !he.is_null() { (*he).v.val } else { 0 as *mut libc::c_void };
}
#[no_mangle]
pub unsafe extern "C" fn dictFingerprint(mut d: *mut dict) -> libc::c_ulonglong {
    let mut integers: [libc::c_ulonglong; 6] = [0; 6];
    let mut hash: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut j: libc::c_int = 0;
    integers[0 as libc::c_int
        as usize] = (*d).ht_table[0 as libc::c_int as usize] as libc::c_long
        as libc::c_ulonglong;
    integers[1 as libc::c_int
        as usize] = (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_ulonglong;
    integers[2 as libc::c_int
        as usize] = (*d).ht_used[0 as libc::c_int as usize] as libc::c_ulonglong;
    integers[3 as libc::c_int
        as usize] = (*d).ht_table[1 as libc::c_int as usize] as libc::c_long
        as libc::c_ulonglong;
    integers[4 as libc::c_int
        as usize] = (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_ulonglong;
    integers[5 as libc::c_int
        as usize] = (*d).ht_used[1 as libc::c_int as usize] as libc::c_ulonglong;
    j = 0 as libc::c_int;
    while j < 6 as libc::c_int {
        hash = hash.wrapping_add(integers[j as usize]);
        hash = (!hash).wrapping_add(hash << 21 as libc::c_int);
        hash = hash ^ hash >> 24 as libc::c_int;
        hash = hash
            .wrapping_add(hash << 3 as libc::c_int)
            .wrapping_add(hash << 8 as libc::c_int);
        hash = hash ^ hash >> 14 as libc::c_int;
        hash = hash
            .wrapping_add(hash << 2 as libc::c_int)
            .wrapping_add(hash << 4 as libc::c_int);
        hash = hash ^ hash >> 28 as libc::c_int;
        hash = hash.wrapping_add(hash << 31 as libc::c_int);
        j += 1;
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn dictGetIterator(mut d: *mut dict) -> *mut dictIterator {
    let mut iter: *mut dictIterator = zmalloc(
        core::mem::size_of::<dictIterator>() as libc::c_ulong,
    ) as *mut dictIterator;
    (*iter).d = d;
    (*iter).table = 0 as libc::c_int;
    (*iter).index = -(1 as libc::c_int) as libc::c_long;
    (*iter).safe = 0 as libc::c_int;
    (*iter).entry = 0 as *mut dictEntry;
    (*iter).nextEntry = 0 as *mut dictEntry;
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn dictGetSafeIterator(mut d: *mut dict) -> *mut dictIterator {
    let mut i: *mut dictIterator = dictGetIterator(d);
    (*i).safe = 1 as libc::c_int;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn dictNext(mut iter: *mut dictIterator) -> *mut dictEntry {
    loop {
        if ((*iter).entry).is_null() {
            if (*iter).index == -(1 as libc::c_int) as libc::c_long
                && (*iter).table == 0 as libc::c_int
            {
                if (*iter).safe != 0 {
                    (*(*iter).d).pauserehash += 1;
                } else {
                    (*iter).fingerprint = dictFingerprint((*iter).d);
                }
            }
            (*iter).index += 1;
            if (*iter).index
                >= (if (*(*iter).d).ht_size_exp[(*iter).table as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*(*iter).d).ht_size_exp[(*iter).table as usize]
                            as libc::c_int
                }) as libc::c_long
            {
                if !((*(*iter).d).rehashidx != -(1 as libc::c_int) as libc::c_long
                    && (*iter).table == 0 as libc::c_int)
                {
                    break;
                }
                (*iter).table += 1;
                (*iter).index = 0 as libc::c_int as libc::c_long;
            }
            (*iter)
                .entry = *((*(*iter).d).ht_table[(*iter).table as usize])
                .offset((*iter).index as isize);
        } else {
            (*iter).entry = (*iter).nextEntry;
        }
        if !((*iter).entry).is_null() {
            (*iter).nextEntry = (*(*iter).entry).next;
            return (*iter).entry;
        }
    }
    return 0 as *mut dictEntry;
}
#[no_mangle]
pub unsafe extern "C" fn dictReleaseIterator(mut iter: *mut dictIterator) {
    if !((*iter).index == -(1 as libc::c_int) as libc::c_long
        && (*iter).table == 0 as libc::c_int)
    {
        if (*iter).safe != 0 {
            (*(*iter).d).pauserehash -= 1;
        } else {
            if ((*iter).fingerprint == dictFingerprint((*iter).d)) as libc::c_int
                as libc::c_long != 0
            {} else {
                _serverAssert(
                    b"iter->fingerprint == dictFingerprint(iter->d)\0" as *const u8
                        as *const libc::c_char,
                    b"dict.c\0" as *const u8 as *const libc::c_char,
                    639 as libc::c_int,
                );
                unreachable!();
            };
        }
    }
    zfree(iter as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn dictGetRandomKey(mut d: *mut dict) -> *mut dictEntry {
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    let mut orighe: *mut dictEntry = 0 as *mut dictEntry;
    let mut h: libc::c_ulong = 0;
    let mut listlen: libc::c_int = 0;
    let mut listele: libc::c_int = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut dictEntry;
    }
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        _dictRehashStep(d);
    }
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        let mut s0: libc::c_ulong = if (*d).ht_size_exp[0 as libc::c_int as usize]
            as libc::c_int == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        };
        loop {
            h = ((*d).rehashidx as libc::c_ulong)
                .wrapping_add(
                    (genrand64_int64() as libc::c_ulong)
                        .wrapping_rem(
                            (if (*d).ht_size_exp[0 as libc::c_int as usize]
                                as libc::c_int == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*d).ht_size_exp[0 as libc::c_int as usize]
                                        as libc::c_int
                            })
                                .wrapping_add(
                                    (if (*d).ht_size_exp[1 as libc::c_int as usize]
                                        as libc::c_int == -(1 as libc::c_int)
                                    {
                                        0 as libc::c_int as libc::c_ulong
                                    } else {
                                        (1 as libc::c_int as libc::c_ulong)
                                            << (*d).ht_size_exp[1 as libc::c_int as usize]
                                                as libc::c_int
                                    }),
                                )
                                .wrapping_sub((*d).rehashidx as libc::c_ulong),
                        ),
                );
            he = if h >= s0 {
                *((*d).ht_table[1 as libc::c_int as usize])
                    .offset(h.wrapping_sub(s0) as isize)
            } else {
                *((*d).ht_table[0 as libc::c_int as usize]).offset(h as isize)
            };
            if !he.is_null() {
                break;
            }
        }
    } else {
        let mut m: libc::c_ulong = if (*d).ht_size_exp[0 as libc::c_int as usize]
            as libc::c_int == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        loop {
            h = genrand64_int64() as libc::c_ulong & m;
            he = *((*d).ht_table[0 as libc::c_int as usize]).offset(h as isize);
            if !he.is_null() {
                break;
            }
        }
    }
    listlen = 0 as libc::c_int;
    orighe = he;
    while !he.is_null() {
        he = (*he).next;
        listlen += 1;
    }
    listele = (random() % listlen as libc::c_long) as libc::c_int;
    he = orighe;
    loop {
        let fresh5 = listele;
        listele = listele - 1;
        if !(fresh5 != 0) {
            break;
        }
        he = (*he).next;
    }
    return he;
}
#[no_mangle]
pub unsafe extern "C" fn dictGetSomeKeys(
    mut d: *mut dict,
    mut des: *mut *mut dictEntry,
    mut count: libc::c_uint,
) -> libc::c_uint {
    let mut j: libc::c_ulong = 0;
    let mut tables: libc::c_ulong = 0;
    let mut stored: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut maxsizemask: libc::c_ulong = 0;
    let mut maxsteps: libc::c_ulong = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize]) < count as libc::c_ulong
    {
        count = ((*d).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*d).ht_used[1 as libc::c_int as usize]) as libc::c_uint;
    }
    maxsteps = count.wrapping_mul(10 as libc::c_int as libc::c_uint) as libc::c_ulong;
    j = 0 as libc::c_int as libc::c_ulong;
    while j < count as libc::c_ulong {
        if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        _dictRehashStep(d);
        j = j.wrapping_add(1);
    }
    tables = (if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        2 as libc::c_int
    } else {
        1 as libc::c_int
    }) as libc::c_ulong;
    maxsizemask = if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        == -(1 as libc::c_int)
    {
        0 as libc::c_int as libc::c_ulong
    } else {
        (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        })
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    };
    if tables > 1 as libc::c_int as libc::c_ulong
        && maxsizemask
            < (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            })
    {
        maxsizemask = if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
    }
    let mut i: libc::c_ulong = genrand64_int64() as libc::c_ulong & maxsizemask;
    let mut emptylen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    while stored < count as libc::c_ulong
        && {
            let fresh6 = maxsteps;
            maxsteps = maxsteps.wrapping_sub(1);
            fresh6 != 0
        }
    {
        let mut current_block_28: u64;
        j = 0 as libc::c_int as libc::c_ulong;
        while j < tables {
            if tables == 2 as libc::c_int as libc::c_ulong
                && j == 0 as libc::c_int as libc::c_ulong
                && i < (*d).rehashidx as libc::c_ulong
            {
                if i
                    >= (if (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*d).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                    })
                {
                    i = (*d).rehashidx as libc::c_ulong;
                    current_block_28 = 17833034027772472439;
                } else {
                    current_block_28 = 12800627514080957624;
                }
            } else {
                current_block_28 = 17833034027772472439;
            }
            match current_block_28 {
                17833034027772472439 => {
                    if !(i
                        >= (if (*d).ht_size_exp[j as usize] as libc::c_int
                            == -(1 as libc::c_int)
                        {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (1 as libc::c_int as libc::c_ulong)
                                << (*d).ht_size_exp[j as usize] as libc::c_int
                        }))
                    {
                        let mut he: *mut dictEntry = *((*d).ht_table[j as usize])
                            .offset(i as isize);
                        if he.is_null() {
                            emptylen = emptylen.wrapping_add(1);
                            if emptylen >= 5 as libc::c_int as libc::c_ulong
                                && emptylen > count as libc::c_ulong
                            {
                                i = genrand64_int64() as libc::c_ulong & maxsizemask;
                                emptylen = 0 as libc::c_int as libc::c_ulong;
                            }
                        } else {
                            emptylen = 0 as libc::c_int as libc::c_ulong;
                            while !he.is_null() {
                                *des = he;
                                des = des.offset(1);
                                he = (*he).next;
                                stored = stored.wrapping_add(1);
                                if stored == count as libc::c_ulong {
                                    return stored as libc::c_uint;
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1 as libc::c_int as libc::c_ulong) & maxsizemask;
    }
    return stored as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dictGetFairRandomKey(mut d: *mut dict) -> *mut dictEntry {
    let mut entries: [*mut dictEntry; 15] = [0 as *mut dictEntry; 15];
    let mut count: libc::c_uint = dictGetSomeKeys(
        d,
        entries.as_mut_ptr(),
        15 as libc::c_int as libc::c_uint,
    );
    if count == 0 as libc::c_int as libc::c_uint {
        return dictGetRandomKey(d);
    }
    let mut idx: libc::c_uint = (rand() as libc::c_uint).wrapping_rem(count);
    return entries[idx as usize];
}
unsafe extern "C" fn rev(mut v: libc::c_ulong) -> libc::c_ulong {
    let mut s: libc::c_ulong = (8 as libc::c_int as libc::c_ulong)
        .wrapping_mul(core::mem::size_of::<libc::c_ulong>() as libc::c_ulong);
    let mut mask: libc::c_ulong = !(0 as libc::c_ulong);
    loop {
        s >>= 1 as libc::c_int;
        if !(s > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
        mask ^= mask << s;
        v = v >> s & mask | v << s & !mask;
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn dictScan(
    mut d: *mut dict,
    mut v: libc::c_ulong,
    mut fn_0: Option::<dictScanFunction>,
    mut bucketfn: Option::<dictScanBucketFunction>,
    mut privdata: *mut libc::c_void,
) -> libc::c_ulong {
    let mut htidx0: libc::c_int = 0;
    let mut htidx1: libc::c_int = 0;
    let mut de: *const dictEntry = 0 as *const dictEntry;
    let mut next: *const dictEntry = 0 as *const dictEntry;
    let mut m0: libc::c_ulong = 0;
    let mut m1: libc::c_ulong = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as libc::c_int as libc::c_ulong;
    }
    (*d).pauserehash += 1;
    if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
        htidx0 = 0 as libc::c_int;
        m0 = if (*d).ht_size_exp[htidx0 as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if (*d).ht_size_exp[htidx0 as usize] as libc::c_int == -(1 as libc::c_int) {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[htidx0 as usize] as libc::c_int
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        if bucketfn.is_some() {
            bucketfn
                .expect(
                    "non-null function pointer",
                )(
                d,
                &mut *(*((*d).ht_table).as_mut_ptr().offset(htidx0 as isize))
                    .offset((v & m0) as isize),
            );
        }
        de = *((*d).ht_table[htidx0 as usize]).offset((v & m0) as isize);
        while !de.is_null() {
            next = (*de).next;
            fn_0.expect("non-null function pointer")(privdata, de);
            de = next;
        }
        v |= !m0;
        v = rev(v);
        v = v.wrapping_add(1);
        v = rev(v);
    } else {
        htidx0 = 0 as libc::c_int;
        htidx1 = 1 as libc::c_int;
        if (if (*d).ht_size_exp[htidx0 as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[htidx0 as usize] as libc::c_int
        })
            > (if (*d).ht_size_exp[htidx1 as usize] as libc::c_int == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[htidx1 as usize] as libc::c_int
            })
        {
            htidx0 = 1 as libc::c_int;
            htidx1 = 0 as libc::c_int;
        }
        m0 = if (*d).ht_size_exp[htidx0 as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if (*d).ht_size_exp[htidx0 as usize] as libc::c_int == -(1 as libc::c_int) {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[htidx0 as usize] as libc::c_int
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        m1 = if (*d).ht_size_exp[htidx1 as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (if (*d).ht_size_exp[htidx1 as usize] as libc::c_int == -(1 as libc::c_int) {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[htidx1 as usize] as libc::c_int
            })
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        };
        if bucketfn.is_some() {
            bucketfn
                .expect(
                    "non-null function pointer",
                )(
                d,
                &mut *(*((*d).ht_table).as_mut_ptr().offset(htidx0 as isize))
                    .offset((v & m0) as isize),
            );
        }
        de = *((*d).ht_table[htidx0 as usize]).offset((v & m0) as isize);
        while !de.is_null() {
            next = (*de).next;
            fn_0.expect("non-null function pointer")(privdata, de);
            de = next;
        }
        loop {
            if bucketfn.is_some() {
                bucketfn
                    .expect(
                        "non-null function pointer",
                    )(
                    d,
                    &mut *(*((*d).ht_table).as_mut_ptr().offset(htidx1 as isize))
                        .offset((v & m1) as isize),
                );
            }
            de = *((*d).ht_table[htidx1 as usize]).offset((v & m1) as isize);
            while !de.is_null() {
                next = (*de).next;
                fn_0.expect("non-null function pointer")(privdata, de);
                de = next;
            }
            v |= !m1;
            v = rev(v);
            v = v.wrapping_add(1);
            v = rev(v);
            if !(v & (m0 ^ m1) != 0) {
                break;
            }
        }
    }
    (*d).pauserehash -= 1;
    return v;
}
unsafe extern "C" fn dictTypeExpandAllowed(mut d: *mut dict) -> libc::c_int {
    if ((*(*d).type_0).expandAllowed).is_none() {
        return 1 as libc::c_int;
    }
    return ((*(*d).type_0).expandAllowed)
        .expect(
            "non-null function pointer",
        )(
        (if _dictNextExp(
            ((*d).ht_used[0 as libc::c_int as usize])
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as libc::c_int == -(1 as libc::c_int)
        {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << _dictNextExp(
                    ((*d).ht_used[0 as libc::c_int as usize])
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as libc::c_int
        })
            .wrapping_mul(core::mem::size_of::<*mut dictEntry>() as libc::c_ulong),
        (*d).ht_used[0 as libc::c_int as usize] as libc::c_double
            / (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            }) as libc::c_double,
    );
}
unsafe extern "C" fn _dictExpandIfNeeded(mut d: *mut dict) -> libc::c_int {
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long {
        return 0 as libc::c_int;
    }
    if (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        == -(1 as libc::c_int)
    {
        0 as libc::c_int as libc::c_ulong
    } else {
        (1 as libc::c_int as libc::c_ulong)
            << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
    }) == 0 as libc::c_int as libc::c_ulong
    {
        return dictExpand(d, ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong);
    }
    if dictTypeExpandAllowed(d) == 0 {
        return 0 as libc::c_int;
    }
    if dict_can_resize as libc::c_uint
        == DICT_RESIZE_ENABLE as libc::c_int as libc::c_uint
        && (*d).ht_used[0 as libc::c_int as usize]
            >= (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
            })
        || dict_can_resize as libc::c_uint
            != DICT_RESIZE_FORBID as libc::c_int as libc::c_uint
            && ((*d).ht_used[0 as libc::c_int as usize])
                .wrapping_div(
                    (if (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                        == -(1 as libc::c_int)
                    {
                        0 as libc::c_int as libc::c_ulong
                    } else {
                        (1 as libc::c_int as libc::c_ulong)
                            << (*d).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    }),
                ) > dict_force_resize_ratio as libc::c_ulong
    {
        return dictExpand(
            d,
            ((*d).ht_used[0 as libc::c_int as usize])
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn _dictNextExp(mut size: libc::c_ulong) -> libc::c_schar {
    let mut e: libc::c_uchar = 2 as libc::c_int as libc::c_uchar;
    if size >= 9223372036854775807 as libc::c_long as libc::c_ulong {
        return (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<libc::c_long>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_schar;
    }
    loop {
        if (1 as libc::c_int as libc::c_ulong) << e as libc::c_int >= size {
            return e as libc::c_schar;
        }
        e = e.wrapping_add(1);
    };
}
unsafe extern "C" fn _dictKeyIndex(
    mut d: *mut dict,
    mut key: *const libc::c_void,
    mut hash: uint64_t,
    mut existing: *mut *mut dictEntry,
) -> libc::c_long {
    let mut idx: libc::c_ulong = 0;
    let mut table: libc::c_ulong = 0;
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    if !existing.is_null() {
        *existing = 0 as *mut dictEntry;
    }
    if _dictExpandIfNeeded(d) == 1 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    table = 0 as libc::c_int as libc::c_ulong;
    while table <= 1 as libc::c_int as libc::c_ulong {
        idx = hash
            & (if (*d).ht_size_exp[table as usize] as libc::c_int == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if (*d).ht_size_exp[table as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[table as usize] as libc::c_int
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            });
        he = *((*d).ht_table[table as usize]).offset(idx as isize);
        while !he.is_null() {
            if key == (*he).key as *const libc::c_void
                || (if ((*(*d).type_0).keyCompare).is_some() {
                    ((*(*d).type_0).keyCompare)
                        .expect("non-null function pointer")(d, key, (*he).key)
                } else {
                    (key == (*he).key as *const libc::c_void) as libc::c_int
                }) != 0
            {
                if !existing.is_null() {
                    *existing = he;
                }
                return -(1 as libc::c_int) as libc::c_long;
            }
            he = (*he).next;
        }
        if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
            break;
        }
        table = table.wrapping_add(1);
    }
    return idx as libc::c_long;
}
#[no_mangle]
pub unsafe extern "C" fn dictEmpty(
    mut d: *mut dict,
    mut callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
) {
    _dictClear(d, 0 as libc::c_int, callback);
    _dictClear(d, 1 as libc::c_int, callback);
    (*d).rehashidx = -(1 as libc::c_int) as libc::c_long;
    (*d).pauserehash = 0 as libc::c_int as int16_t;
}
#[no_mangle]
pub unsafe extern "C" fn dictSetResizeEnabled(mut enable: dictResizeEnable) {
    dict_can_resize = enable;
}
#[no_mangle]
pub unsafe extern "C" fn dictGetHash(
    mut d: *mut dict,
    mut key: *const libc::c_void,
) -> uint64_t {
    return ((*(*d).type_0).hashFunction).expect("non-null function pointer")(key);
}
#[no_mangle]
pub unsafe extern "C" fn dictFindEntryRefByPtrAndHash(
    mut d: *mut dict,
    mut oldptr: *const libc::c_void,
    mut hash: uint64_t,
) -> *mut *mut dictEntry {
    let mut he: *mut dictEntry = 0 as *mut dictEntry;
    let mut heref: *mut *mut dictEntry = 0 as *mut *mut dictEntry;
    let mut idx: libc::c_ulong = 0;
    let mut table: libc::c_ulong = 0;
    if ((*d).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*d).ht_used[1 as libc::c_int as usize])
        == 0 as libc::c_int as libc::c_ulong
    {
        return 0 as *mut *mut dictEntry;
    }
    table = 0 as libc::c_int as libc::c_ulong;
    while table <= 1 as libc::c_int as libc::c_ulong {
        idx = hash
            & (if (*d).ht_size_exp[table as usize] as libc::c_int == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (if (*d).ht_size_exp[table as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[table as usize] as libc::c_int
                })
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            });
        heref = &mut *(*((*d).ht_table).as_mut_ptr().offset(table as isize))
            .offset(idx as isize) as *mut *mut dictEntry;
        he = *heref;
        while !he.is_null() {
            if oldptr == (*he).key as *const libc::c_void {
                return heref;
            }
            heref = &mut (*he).next;
            he = *heref;
        }
        if !((*d).rehashidx != -(1 as libc::c_int) as libc::c_long) {
            return 0 as *mut *mut dictEntry;
        }
        table = table.wrapping_add(1);
    }
    return 0 as *mut *mut dictEntry;
}
#[no_mangle]
pub unsafe extern "C" fn _dictGetStatsHt(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut d: *mut dict,
    mut htidx: libc::c_int,
) -> size_t {
    let mut i: libc::c_ulong = 0;
    let mut slots: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut chainlen: libc::c_ulong = 0;
    let mut maxchainlen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut totchainlen: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut clvector: [libc::c_ulong; 50] = [0; 50];
    let mut l: size_t = 0 as libc::c_int as size_t;
    if (*d).ht_used[htidx as usize] == 0 as libc::c_int as libc::c_ulong {
        return snprintf(
            buf,
            bufsize,
            b"No stats available for empty dictionaries\n\0" as *const u8
                as *const libc::c_char,
        ) as size_t;
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i < 50 as libc::c_int as libc::c_ulong {
        clvector[i as usize] = 0 as libc::c_int as libc::c_ulong;
        i = i.wrapping_add(1);
    }
    i = 0 as libc::c_int as libc::c_ulong;
    while i
        < (if (*d).ht_size_exp[htidx as usize] as libc::c_int == -(1 as libc::c_int) {
            0 as libc::c_int as libc::c_ulong
        } else {
            (1 as libc::c_int as libc::c_ulong)
                << (*d).ht_size_exp[htidx as usize] as libc::c_int
        })
    {
        let mut he: *mut dictEntry = 0 as *mut dictEntry;
        if (*((*d).ht_table[htidx as usize]).offset(i as isize)).is_null() {
            clvector[0 as libc::c_int
                as usize] = (clvector[0 as libc::c_int as usize]).wrapping_add(1);
        } else {
            slots = slots.wrapping_add(1);
            chainlen = 0 as libc::c_int as libc::c_ulong;
            he = *((*d).ht_table[htidx as usize]).offset(i as isize);
            while !he.is_null() {
                chainlen = chainlen.wrapping_add(1);
                he = (*he).next;
            }
            clvector[(if chainlen < 50 as libc::c_int as libc::c_ulong {
                chainlen
            } else {
                (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            })
                as usize] = (clvector[(if chainlen < 50 as libc::c_int as libc::c_ulong {
                chainlen
            } else {
                (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
            }) as usize])
                .wrapping_add(1);
            if chainlen > maxchainlen {
                maxchainlen = chainlen;
            }
            totchainlen = totchainlen.wrapping_add(chainlen);
        }
        i = i.wrapping_add(1);
    }
    l = (l as libc::c_ulong)
        .wrapping_add(
            snprintf(
                buf.offset(l as isize),
                bufsize.wrapping_sub(l),
                b"Hash table %d stats (%s):\n table size: %lu\n number of elements: %lu\n different slots: %lu\n max chain length: %lu\n avg chain length (counted): %.02f\n avg chain length (computed): %.02f\n Chain length distribution:\n\0"
                    as *const u8 as *const libc::c_char,
                htidx,
                if htidx == 0 as libc::c_int {
                    b"main hash table\0" as *const u8 as *const libc::c_char
                } else {
                    b"rehashing target\0" as *const u8 as *const libc::c_char
                },
                if (*d).ht_size_exp[htidx as usize] as libc::c_int == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*d).ht_size_exp[htidx as usize] as libc::c_int
                },
                (*d).ht_used[htidx as usize],
                slots,
                maxchainlen,
                (totchainlen as libc::c_float / slots as libc::c_float)
                    as libc::c_double,
                ((*d).ht_used[htidx as usize] as libc::c_float / slots as libc::c_float)
                    as libc::c_double,
            ) as libc::c_ulong,
        ) as size_t as size_t;
    i = 0 as libc::c_int as libc::c_ulong;
    while i < (50 as libc::c_int - 1 as libc::c_int) as libc::c_ulong {
        if !(clvector[i as usize] == 0 as libc::c_int as libc::c_ulong) {
            if l >= bufsize {
                break;
            }
            l = (l as libc::c_ulong)
                .wrapping_add(
                    snprintf(
                        buf.offset(l as isize),
                        bufsize.wrapping_sub(l),
                        b"   %ld: %ld (%.02f%%)\n\0" as *const u8 as *const libc::c_char,
                        i,
                        clvector[i as usize],
                        (clvector[i as usize] as libc::c_float
                            / (if (*d).ht_size_exp[htidx as usize] as libc::c_int
                                == -(1 as libc::c_int)
                            {
                                0 as libc::c_int as libc::c_ulong
                            } else {
                                (1 as libc::c_int as libc::c_ulong)
                                    << (*d).ht_size_exp[htidx as usize] as libc::c_int
                            }) as libc::c_float * 100 as libc::c_int as libc::c_float)
                            as libc::c_double,
                    ) as libc::c_ulong,
                ) as size_t as size_t;
        }
        i = i.wrapping_add(1);
    }
    if bufsize != 0 {
        *buf
            .offset(
                bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
    return strlen(buf);
}
#[no_mangle]
pub unsafe extern "C" fn dictGetStats(
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
    mut d: *mut dict,
) {
    let mut l: size_t = 0;
    let mut orig_buf: *mut libc::c_char = buf;
    let mut orig_bufsize: size_t = bufsize;
    l = _dictGetStatsHt(buf, bufsize, d, 0 as libc::c_int);
    buf = buf.offset(l as isize);
    bufsize = (bufsize as libc::c_ulong).wrapping_sub(l) as size_t as size_t;
    if (*d).rehashidx != -(1 as libc::c_int) as libc::c_long
        && bufsize > 0 as libc::c_int as libc::c_ulong
    {
        _dictGetStatsHt(buf, bufsize, d, 1 as libc::c_int);
    }
    if orig_bufsize != 0 {
        *orig_buf
            .offset(
                orig_bufsize.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) = '\0' as i32 as libc::c_char;
    }
}

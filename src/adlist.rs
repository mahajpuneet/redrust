
extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
}
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listNode {
    pub prev: *mut listNode,
    pub next: *mut listNode,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listIter {
    pub next: *mut listNode,
    pub direction: libc::c_int,
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
#[no_mangle]
pub unsafe extern "C" fn listCreate() -> *mut list {
    let mut list: *mut list = 0 as *mut list;
    list = zmalloc(core::mem::size_of::<list>() as libc::c_ulong) as *mut list;
    if list.is_null() {
        return 0 as *mut list;
    }
    (*list).tail = 0 as *mut listNode;
    (*list).head = (*list).tail;
    (*list).len = 0 as libc::c_int as libc::c_ulong;
    (*list).dup = None;
    (*list).free = None;
    (*list).match_0 = None;
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn listEmpty(mut list: *mut list) {
    let mut len: libc::c_ulong = 0;
    let mut current: *mut listNode = 0 as *mut listNode;
    let mut next: *mut listNode = 0 as *mut listNode;
    current = (*list).head;
    len = (*list).len;
    loop {
        let fresh0 = len;
        len = len.wrapping_sub(1);
        if !(fresh0 != 0) {
            break;
        }
        next = (*current).next;
        if ((*list).free).is_some() {
            ((*list).free).expect("non-null function pointer")((*current).value);
        }
        zfree(current as *mut libc::c_void);
        current = next;
    }
    (*list).tail = 0 as *mut listNode;
    (*list).head = (*list).tail;
    (*list).len = 0 as libc::c_int as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn listRelease(mut list: *mut list) {
    listEmpty(list);
    zfree(list as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn listAddNodeHead(
    mut list: *mut list,
    mut value: *mut libc::c_void,
) -> *mut list {
    let mut node: *mut listNode = 0 as *mut listNode;
    node = zmalloc(core::mem::size_of::<listNode>() as libc::c_ulong) as *mut listNode;
    if node.is_null() {
        return 0 as *mut list;
    }
    (*node).value = value;
    if (*list).len == 0 as libc::c_int as libc::c_ulong {
        (*list).tail = node;
        (*list).head = (*list).tail;
        (*node).next = 0 as *mut listNode;
        (*node).prev = (*node).next;
    } else {
        (*node).prev = 0 as *mut listNode;
        (*node).next = (*list).head;
        (*(*list).head).prev = node;
        (*list).head = node;
    }
    (*list).len = ((*list).len).wrapping_add(1);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn listAddNodeTail(
    mut list: *mut list,
    mut value: *mut libc::c_void,
) -> *mut list {
    let mut node: *mut listNode = 0 as *mut listNode;
    node = zmalloc(core::mem::size_of::<listNode>() as libc::c_ulong) as *mut listNode;
    if node.is_null() {
        return 0 as *mut list;
    }
    (*node).value = value;
    if (*list).len == 0 as libc::c_int as libc::c_ulong {
        (*list).tail = node;
        (*list).head = (*list).tail;
        (*node).next = 0 as *mut listNode;
        (*node).prev = (*node).next;
    } else {
        (*node).prev = (*list).tail;
        (*node).next = 0 as *mut listNode;
        (*(*list).tail).next = node;
        (*list).tail = node;
    }
    (*list).len = ((*list).len).wrapping_add(1);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn listInsertNode(
    mut list: *mut list,
    mut old_node: *mut listNode,
    mut value: *mut libc::c_void,
    mut after: libc::c_int,
) -> *mut list {
    let mut node: *mut listNode = 0 as *mut listNode;
    node = zmalloc(core::mem::size_of::<listNode>() as libc::c_ulong) as *mut listNode;
    if node.is_null() {
        return 0 as *mut list;
    }
    (*node).value = value;
    if after != 0 {
        (*node).prev = old_node;
        (*node).next = (*old_node).next;
        if (*list).tail == old_node {
            (*list).tail = node;
        }
    } else {
        (*node).next = old_node;
        (*node).prev = (*old_node).prev;
        if (*list).head == old_node {
            (*list).head = node;
        }
    }
    if !((*node).prev).is_null() {
        (*(*node).prev).next = node;
    }
    if !((*node).next).is_null() {
        (*(*node).next).prev = node;
    }
    (*list).len = ((*list).len).wrapping_add(1);
    return list;
}
#[no_mangle]
pub unsafe extern "C" fn listDelNode(mut list: *mut list, mut node: *mut listNode) {
    if !((*node).prev).is_null() {
        (*(*node).prev).next = (*node).next;
    } else {
        (*list).head = (*node).next;
    }
    if !((*node).next).is_null() {
        (*(*node).next).prev = (*node).prev;
    } else {
        (*list).tail = (*node).prev;
    }
    if ((*list).free).is_some() {
        ((*list).free).expect("non-null function pointer")((*node).value);
    }
    zfree(node as *mut libc::c_void);
    (*list).len = ((*list).len).wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn listGetIterator(
    mut list: *mut list,
    mut direction: libc::c_int,
) -> *mut listIter {
    let mut iter: *mut listIter = 0 as *mut listIter;
    iter = zmalloc(core::mem::size_of::<listIter>() as libc::c_ulong) as *mut listIter;
    if iter.is_null() {
        return 0 as *mut listIter;
    }
    if direction == 0 as libc::c_int {
        (*iter).next = (*list).head;
    } else {
        (*iter).next = (*list).tail;
    }
    (*iter).direction = direction;
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn listReleaseIterator(mut iter: *mut listIter) {
    zfree(iter as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn listRewind(mut list: *mut list, mut li: *mut listIter) {
    (*li).next = (*list).head;
    (*li).direction = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn listRewindTail(mut list: *mut list, mut li: *mut listIter) {
    (*li).next = (*list).tail;
    (*li).direction = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn listNext(mut iter: *mut listIter) -> *mut listNode {
    let mut current: *mut listNode = (*iter).next;
    if !current.is_null() {
        if (*iter).direction == 0 as libc::c_int {
            (*iter).next = (*current).next;
        } else {
            (*iter).next = (*current).prev;
        }
    }
    return current;
}
#[no_mangle]
pub unsafe extern "C" fn listDup(mut orig: *mut list) -> *mut list {
    let mut copy: *mut list = 0 as *mut list;
    let mut iter: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut node: *mut listNode = 0 as *mut listNode;
    copy = listCreate();
    if copy.is_null() {
        return 0 as *mut list;
    }
    (*copy).dup = (*orig).dup;
    (*copy).free = (*orig).free;
    (*copy).match_0 = (*orig).match_0;
    listRewind(orig, &mut iter);
    loop {
        node = listNext(&mut iter);
        if node.is_null() {
            break;
        }
        let mut value: *mut libc::c_void = 0 as *mut libc::c_void;
        if ((*copy).dup).is_some() {
            value = ((*copy).dup).expect("non-null function pointer")((*node).value);
            if value.is_null() {
                listRelease(copy);
                return 0 as *mut list;
            }
        } else {
            value = (*node).value;
        }
        if (listAddNodeTail(copy, value)).is_null() {
            if ((*copy).free).is_some() {
                ((*copy).free).expect("non-null function pointer")(value);
            }
            listRelease(copy);
            return 0 as *mut list;
        }
    }
    return copy;
}
#[no_mangle]
pub unsafe extern "C" fn listSearchKey(
    mut list: *mut list,
    mut key: *mut libc::c_void,
) -> *mut listNode {
    let mut iter: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut node: *mut listNode = 0 as *mut listNode;
    listRewind(list, &mut iter);
    loop {
        node = listNext(&mut iter);
        if node.is_null() {
            break;
        }
        if ((*list).match_0).is_some() {
            if ((*list).match_0).expect("non-null function pointer")((*node).value, key)
                != 0
            {
                return node;
            }
        } else if key == (*node).value {
            return node
        }
    }
    return 0 as *mut listNode;
}
#[no_mangle]
pub unsafe extern "C" fn listIndex(
    mut list: *mut list,
    mut index: libc::c_long,
) -> *mut listNode {
    let mut n: *mut listNode = 0 as *mut listNode;
    if index < 0 as libc::c_int as libc::c_long {
        index = -index - 1 as libc::c_int as libc::c_long;
        n = (*list).tail;
        loop {
            let fresh1 = index;
            index = index - 1;
            if !(fresh1 != 0 && !n.is_null()) {
                break;
            }
            n = (*n).prev;
        }
    } else {
        n = (*list).head;
        loop {
            let fresh2 = index;
            index = index - 1;
            if !(fresh2 != 0 && !n.is_null()) {
                break;
            }
            n = (*n).next;
        }
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn listRotateTailToHead(mut list: *mut list) {
    if (*list).len <= 1 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut tail: *mut listNode = (*list).tail;
    (*list).tail = (*tail).prev;
    (*(*list).tail).next = 0 as *mut listNode;
    (*(*list).head).prev = tail;
    (*tail).prev = 0 as *mut listNode;
    (*tail).next = (*list).head;
    (*list).head = tail;
}
#[no_mangle]
pub unsafe extern "C" fn listRotateHeadToTail(mut list: *mut list) {
    if (*list).len <= 1 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut head: *mut listNode = (*list).head;
    (*list).head = (*head).next;
    (*(*list).head).prev = 0 as *mut listNode;
    (*(*list).tail).next = head;
    (*head).next = 0 as *mut listNode;
    (*head).prev = (*list).tail;
    (*list).tail = head;
}
#[no_mangle]
pub unsafe extern "C" fn listJoin(mut l: *mut list, mut o: *mut list) {
    if (*o).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    (*(*o).head).prev = (*l).tail;
    if !((*l).tail).is_null() {
        (*(*l).tail).next = (*o).head;
    } else {
        (*l).head = (*o).head;
    }
    (*l).tail = (*o).tail;
    (*l).len = ((*l).len).wrapping_add((*o).len);
    (*o).tail = 0 as *mut listNode;
    (*o).head = (*o).tail;
    (*o).len = 0 as libc::c_int as libc::c_ulong;
}

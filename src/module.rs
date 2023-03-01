extern crate c2rust_bitfields;
extern crate libc;
extern crate core;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type CallReply;
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
    fn sdsnewlen(init: *const libc::c_void, initlen: size_t) -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn sdsempty() -> sds;
    fn sdsdup(s: sds) -> sds;
    fn sdsfree(s: sds);
    fn sdsgrowzero(s: sds, len: size_t) -> sds;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatvprintf(s: sds, fmt: *const libc::c_char, ap: core::ffi::VaList) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdssubstr(s: sds, start: size_t, len: size_t);
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdsfromlonglong(value: libc::c_longlong) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdsIncrLen(s: sds, incr: ssize_t);
    fn sdsRemoveFreeSpace(s: sds) -> sds;
    fn __errno_location() -> *mut libc::c_int;
    fn connPeerToString(
        conn: *mut connection,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn connTLSGetPeerCert(conn: *mut connection) -> sds;
    fn rioInitWithBuffer(r: *mut rio, s: sds);
    fn rioWriteBulkCount(
        r: *mut rio,
        prefix: libc::c_char,
        count: libc::c_long,
    ) -> size_t;
    fn rioWriteBulkObject(r: *mut rio, obj: *mut redisObject) -> libc::c_int;
    fn pthread_mutex_trylock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn kill(__pid: __pid_t, __sig: libc::c_int) -> libc::c_int;
    fn hdr_close(h: *mut hdr_histogram);
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn aeCreateFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
        proc_0: Option::<aeFileProc>,
        clientData: *mut libc::c_void,
    ) -> libc::c_int;
    fn aeDeleteFileEvent(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
        mask: libc::c_int,
    );
    fn aeGetFileEvents(eventLoop: *mut aeEventLoop, fd: libc::c_int) -> libc::c_int;
    fn aeGetFileClientData(
        eventLoop: *mut aeEventLoop,
        fd: libc::c_int,
    ) -> *mut libc::c_void;
    fn aeCreateTimeEvent(
        eventLoop: *mut aeEventLoop,
        milliseconds: libc::c_longlong,
        proc_0: Option::<aeTimeProc>,
        clientData: *mut libc::c_void,
        finalizerProc: Option::<aeEventFinalizerProc>,
    ) -> libc::c_longlong;
    fn aeDeleteTimeEvent(
        eventLoop: *mut aeEventLoop,
        id: libc::c_longlong,
    ) -> libc::c_int;
    fn aeGetSetSize(eventLoop: *mut aeEventLoop) -> libc::c_int;
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictReplace(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictDelete(d: *mut dict, key: *const libc::c_void) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGenHashFunction(key: *const libc::c_void, len: size_t) -> uint64_t;
    fn dictEmpty(
        d: *mut dict,
        callback: Option::<unsafe extern "C" fn(*mut dict) -> ()>,
    );
    fn dictScan(
        d: *mut dict,
        v: libc::c_ulong,
        fn_0: Option::<dictScanFunction>,
        bucketfn: Option::<dictScanBucketFunction>,
        privdata: *mut libc::c_void,
    ) -> libc::c_ulong;
    fn listCreate() -> *mut list;
    fn listRelease(list: *mut list);
    fn listEmpty(list: *mut list);
    fn listAddNodeHead(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listSearchKey(list: *mut list, key: *mut libc::c_void) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zcalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn ztrymalloc(size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn anetPipe(
        fds: *mut libc::c_int,
        read_flags: libc::c_int,
        write_flags: libc::c_int,
    ) -> libc::c_int;
    fn intsetGet(is: *mut intset, pos: uint32_t, value: *mut int64_t) -> uint8_t;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn ull2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_ulonglong,
    ) -> libc::c_int;
    fn string2ll(
        s: *const libc::c_char,
        slen: size_t,
        value: *mut libc::c_longlong,
    ) -> libc::c_int;
    fn string2ull(s: *const libc::c_char, value: *mut libc::c_ulonglong) -> libc::c_int;
    fn string2ld(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut f64,
    ) -> libc::c_int;
    fn string2d(
        s: *const libc::c_char,
        slen: size_t,
        dp: *mut libc::c_double,
    ) -> libc::c_int;
    fn d2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: libc::c_double,
    ) -> libc::c_int;
    fn ld2string(
        buf: *mut libc::c_char,
        len: size_t,
        value: f64,
        mode: ld2string_mode,
    ) -> libc::c_int;
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    fn quicklistSetOptions(
        quicklist: *mut quicklist,
        fill: libc::c_int,
        depth: libc::c_int,
    );
    static mut raxNotFound: *mut libc::c_void;
    fn raxNew() -> *mut rax;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxTryInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxRemove(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxFind(rax: *mut rax, s: *mut libc::c_uchar, len: size_t) -> *mut libc::c_void;
    fn raxFree(rax: *mut rax);
    fn raxFreeWithCallback(
        rax: *mut rax,
        free_callback: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    );
    fn raxStart(it: *mut raxIterator, rt: *mut rax);
    fn raxSeek(
        it: *mut raxIterator,
        op: *const libc::c_char,
        ele: *mut libc::c_uchar,
        len: size_t,
    ) -> libc::c_int;
    fn raxNext(it: *mut raxIterator) -> libc::c_int;
    fn raxPrev(it: *mut raxIterator) -> libc::c_int;
    fn raxCompare(
        iter: *mut raxIterator,
        op: *const libc::c_char,
        key: *mut libc::c_uchar,
        key_len: size_t,
    ) -> libc::c_int;
    fn raxStop(it: *mut raxIterator);
    fn raxEOF(it: *mut raxIterator) -> libc::c_int;
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn intrev64(v: uint64_t) -> uint64_t;
    fn lpGet(
        p: *mut libc::c_uchar,
        count: *mut int64_t,
        intbuf: *mut libc::c_uchar,
    ) -> *mut libc::c_uchar;
    fn lpGetValue(
        p: *mut libc::c_uchar,
        slen: *mut libc::c_uint,
        lval: *mut libc::c_longlong,
    ) -> *mut libc::c_uchar;
    fn lpFirst(lp: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpNext(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpPrev(lp: *mut libc::c_uchar, p: *mut libc::c_uchar) -> *mut libc::c_uchar;
    fn lpSeek(lp: *mut libc::c_uchar, index: libc::c_long) -> *mut libc::c_uchar;
    fn streamLength(subject: *const robj) -> libc::c_ulong;
    fn streamIteratorStart(
        si: *mut streamIterator,
        s: *mut stream,
        start: *mut streamID,
        end: *mut streamID,
        rev: libc::c_int,
    );
    fn streamIteratorGetID(
        si: *mut streamIterator,
        id: *mut streamID,
        numfields: *mut int64_t,
    ) -> libc::c_int;
    fn streamIteratorGetField(
        si: *mut streamIterator,
        fieldptr: *mut *mut libc::c_uchar,
        valueptr: *mut *mut libc::c_uchar,
        fieldlen: *mut int64_t,
        valuelen: *mut int64_t,
    );
    fn streamIteratorRemoveEntry(si: *mut streamIterator, current: *mut streamID);
    fn streamIteratorStop(si: *mut streamIterator);
    fn streamIncrID(id: *mut streamID) -> libc::c_int;
    fn streamDecrID(id: *mut streamID) -> libc::c_int;
    fn streamParseID(o: *const robj, id: *mut streamID) -> libc::c_int;
    fn createObjectFromStreamID(id: *mut streamID) -> *mut robj;
    fn streamAppendItem(
        s: *mut stream,
        argv: *mut *mut robj,
        numfields: int64_t,
        added_id: *mut streamID,
        use_id: *mut streamID,
        seq_given: libc::c_int,
    ) -> libc::c_int;
    fn streamDeleteItem(s: *mut stream, id: *mut streamID) -> libc::c_int;
    fn streamTrimByLength(
        s: *mut stream,
        maxlen: libc::c_longlong,
        approx: libc::c_int,
    ) -> int64_t;
    fn streamTrimByID(s: *mut stream, minid: streamID, approx: libc::c_int) -> int64_t;
    static mut server: redisServer;
    static mut shared: sharedObjectsStruct;
    static mut modulesDictType: dictType;
    fn populateCommandLegacyRangeSpec(c: *mut redisCommand);
    fn populateArgsStructure(args: *mut redisCommandArg) -> libc::c_int;
    fn _serverLog(level: libc::c_int, fmt: *const libc::c_char, _: ...);
    fn performModuleConfigSetDefaultFromName(
        name: sds,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn performModuleConfigSetFromName(
        name: sds,
        value: sds,
        err: *mut *const libc::c_char,
    ) -> libc::c_int;
    fn addModuleEnumConfig(
        module_name: *const libc::c_char,
        name: *const libc::c_char,
        flags: libc::c_int,
        privdata: *mut libc::c_void,
        default_val: libc::c_int,
        enum_vals: *mut configEnum,
    );
    fn serverLogRaw(level: libc::c_int, msg: *const libc::c_char);
    fn addModuleStringConfig(
        module_name: *const libc::c_char,
        name: *const libc::c_char,
        flags: libc::c_int,
        privdata: *mut libc::c_void,
        default_val: sds,
    );
    fn addModuleNumericConfig(
        module_name: *const libc::c_char,
        name: *const libc::c_char,
        flags: libc::c_int,
        privdata: *mut libc::c_void,
        default_val: libc::c_longlong,
        conf_flags: libc::c_int,
        lower: libc::c_longlong,
        upper: libc::c_longlong,
    );
    fn addModuleBoolConfig(
        module_name: *const libc::c_char,
        name: *const libc::c_char,
        flags: libc::c_int,
        privdata: *mut libc::c_void,
        default_val: libc::c_int,
    );
    fn processEventsWhileBlocked();
    fn protectClient(c: *mut client);
    fn blockingOperationStarts();
    fn ustime() -> libc::c_longlong;
    fn activeDefragStringOb(ob: *mut robj, defragged: *mut libc::c_long) -> *mut robj;
    fn activeDefragAlloc(ptr: *mut libc::c_void) -> *mut libc::c_void;
    fn getKeysFreeResult(result: *mut getKeysResult);
    fn getKeysFromCommand(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn doesCommandHaveKeys(cmd: *mut redisCommand) -> libc::c_int;
    fn lookupCommand(argv: *mut *mut robj, argc: libc::c_int) -> *mut redisCommand;
    fn redactClientCommandArgument(c: *mut client, argc: libc::c_int);
    fn lookupClientByID(id: uint64_t) -> *mut client;
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn ACLGetUserByName(name: *const libc::c_char, namelen: size_t) -> *mut user;
    fn freeClientAsync(c: *mut client);
    static mut DefaultUser: *mut user;
    fn ACLFreeUserAndKillClients(u: *mut user);
    fn addACLLogEntry(
        c: *mut client,
        reason: libc::c_int,
        context: libc::c_int,
        argpos: libc::c_int,
        username: sds,
        object: sds,
    );
    fn ACLUserCheckChannelPerm(
        u: *mut user,
        channel: sds,
        literal: libc::c_int,
    ) -> libc::c_int;
    fn ACLUserCheckKeyPerm(
        u: *mut user,
        key: *const libc::c_char,
        keylen: libc::c_int,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn ACLCheckAllUserCommandPerm(
        u: *mut user,
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        idxptr: *mut libc::c_int,
    ) -> libc::c_int;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn ACLDescribeUser(u: *mut user) -> *mut robj;
    fn ACLStringSetUser(
        u: *mut user,
        username: sds,
        argv: *mut sds,
        argc: libc::c_int,
    ) -> sds;
    fn ACLSetUser(u: *mut user, op: *const libc::c_char, oplen: ssize_t) -> libc::c_int;
    fn ACLCreateUnlinkedUser() -> *mut user;
    fn decrRefCount(o: *mut robj);
    fn createStringObjectFromLongDouble(
        value: f64,
        humanfriendly: libc::c_int,
    ) -> *mut robj;
    fn signalKeyAsReady(db: *mut redisDb, key: *mut robj, type_0: libc::c_int);
    fn zslFreeLexRange(spec: *mut zlexrangespec);
    fn listTypeReleaseIterator(li: *mut listTypeIterator);
    fn signalModifiedKey(c: *mut client, db: *mut redisDb, key: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn getStringObjectSdsUsedMemory(o: *mut robj) -> size_t;
    fn getMaxmemoryState(
        total: *mut size_t,
        logical: *mut size_t,
        tofree: *mut size_t,
        level: *mut libc::c_float,
    ) -> libc::c_int;
    fn blockClient(c: *mut client, btype: libc::c_int);
    fn mstime() -> libc::c_longlong;
    fn blockForKeys(
        c: *mut client,
        btype: libc::c_int,
        keys: *mut *mut robj,
        numkeys: libc::c_int,
        count: libc::c_long,
        timeout: mstime_t,
        target: *mut robj,
        blockpos: *mut blockPos,
        ids: *mut streamID,
    );
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn createClient(conn: *mut connection) -> *mut client;
    fn LFUDecrAndReturn(o: *mut robj) -> libc::c_ulong;
    fn objectSetLRUOrLFU(
        val: *mut robj,
        lfu_freq: libc::c_longlong,
        lru_idle: libc::c_longlong,
        lru_clock: libc::c_longlong,
        lru_multiplier: libc::c_int,
    ) -> libc::c_int;
    fn estimateObjectIdleTime(o: *mut robj) -> libc::c_ulonglong;
    fn LRU_CLOCK() -> libc::c_uint;
    fn pubsubPublishMessageAndPropagateToCluster(
        channel: *mut robj,
        message: *mut robj,
        sharded: libc::c_int,
    ) -> libc::c_int;
    fn clientSetName(c: *mut client, name: *mut robj) -> libc::c_int;
    fn genInfoSectionDict(
        argv: *mut *mut robj,
        argc: libc::c_int,
        defaults: *mut *mut libc::c_char,
        out_all: *mut libc::c_int,
        out_everything: *mut libc::c_int,
    ) -> *mut dict;
    fn releaseInfoSectionDict(sec: *mut dict);
    fn genRedisInfoString(
        section_dict: *mut dict,
        all_sections: libc::c_int,
        everything: libc::c_int,
    ) -> sds;
    fn getSafeInfoString(
        s: *const libc::c_char,
        len: size_t,
        tmp: *mut *mut libc::c_char,
    ) -> *const libc::c_char;
    fn resetChildState();
    fn exitFromChild(retcode: libc::c_int);
    fn sendChildCowInfo(info_type: childInfoType, pname: *mut libc::c_char);
    fn sendChildInfoGeneric(
        info_type: childInfoType,
        keys: size_t,
        progress: libc::c_double,
        pname: *mut libc::c_char,
    );
    fn redisSetProcTitle(title: *mut libc::c_char) -> libc::c_int;
    fn redisFork(purpose: libc::c_int) -> libc::c_int;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn getRandomBytes(p: *mut libc::c_uchar, len: size_t);
    fn freeClient(c: *mut client);
    fn resetClient(c: *mut client);
    fn clearClientConnectionState(c: *mut client);
    fn unblockPostponedClients();
    fn unprotectClient(c: *mut client);
    fn blockingOperationEnds();
    fn propagatePendingCommands();
    fn selectDb(c: *mut client, id: libc::c_int) -> libc::c_int;
    fn notifyKeyspaceEvent(
        type_0: libc::c_int,
        event: *mut libc::c_char,
        key: *mut robj,
        dbid: libc::c_int,
    );
    fn xorDigest(digest: *mut libc::c_uchar, ptr: *const libc::c_void, len: size_t);
    fn mixDigest(digest: *mut libc::c_uchar, ptr: *const libc::c_void, len: size_t);
    fn updateStatsOnUnblock(
        c: *mut client,
        blocked_us: libc::c_long,
        reply_us: libc::c_long,
        had_errors: libc::c_int,
    );
    fn compareStringObjects(a: *mut robj, b: *mut robj) -> libc::c_int;
    fn dupStringObject(o: *const robj) -> *mut robj;
    fn trimStringObjectIfNeeded(o: *mut robj);
    fn lookupCommandByCString(s: *const libc::c_char) -> *mut redisCommand;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn rdbGenericLoadStringObject(
        rdb: *mut rio,
        flags: libc::c_int,
        lenptr: *mut size_t,
    ) -> *mut libc::c_void;
    fn rdbLoadLen(rdb: *mut rio, isencoded: *mut libc::c_int) -> uint64_t;
    fn rdbSaveLen(rdb: *mut rio, len: uint64_t) -> libc::c_int;
    fn rdbSaveRawString(rdb: *mut rio, s: *mut libc::c_uchar, len: size_t) -> ssize_t;
    fn rdbLoadBinaryFloatValue(rdb: *mut rio, val: *mut libc::c_float) -> libc::c_int;
    fn rdbSaveBinaryFloatValue(rdb: *mut rio, val: libc::c_float) -> libc::c_int;
    fn rdbLoadBinaryDoubleValue(rdb: *mut rio, val: *mut libc::c_double) -> libc::c_int;
    fn rdbSaveBinaryDoubleValue(rdb: *mut rio, val: libc::c_double) -> libc::c_int;
    fn rdbSaveStringObject(rdb: *mut rio, obj: *mut robj) -> ssize_t;
    fn rdbLoadLenByRef(
        rdb: *mut rio,
        isencoded: *mut libc::c_int,
        lenptr: *mut uint64_t,
    ) -> libc::c_int;
    fn createModuleObject(mt: *mut moduleType, value: *mut libc::c_void) -> *mut robj;
    fn setKey(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        val: *mut robj,
        flags: libc::c_int,
    );
    fn dbDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int;
    fn hasActiveChildProcess() -> libc::c_int;
    fn getKeysPrepareResult(
        result: *mut getKeysResult,
        numkeys: libc::c_int,
    ) -> *mut keyReference;
    fn createRawStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn dbAdd(db: *mut redisDb, key: *mut robj, val: *mut robj);
    fn createStreamObject() -> *mut robj;
    fn createHashObject() -> *mut robj;
    fn createZsetListpackObject() -> *mut robj;
    fn createQuicklistObject() -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn hashTypeGetValueObject(o: *mut robj, field: sds) -> *mut robj;
    fn hashTypeExists(o: *mut robj, key: sds) -> libc::c_int;
    fn hashTypeLength(o: *const robj) -> libc::c_ulong;
    fn zsetLength(zobj: *const robj) -> libc::c_ulong;
    fn setTypeSize(subject: *const robj) -> libc::c_ulong;
    fn listTypeLength(subject: *const robj) -> libc::c_ulong;
    fn hashTypeSet(
        o: *mut robj,
        field: sds,
        value: sds,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn hashTypeTryConversion(
        subject: *mut robj,
        argv: *mut *mut robj,
        start: libc::c_int,
        end: libc::c_int,
    );
    fn hashTypeDelete(o: *mut robj, key: sds) -> libc::c_int;
    fn zslLexValueGteMin(value: sds, spec: *mut zlexrangespec) -> libc::c_int;
    fn zslValueGteMin(value: libc::c_double, spec: *mut zrangespec) -> libc::c_int;
    fn zzlLexValueGteMin(p: *mut libc::c_uchar, spec: *mut zlexrangespec) -> libc::c_int;
    fn zzlGetScore(sptr: *mut libc::c_uchar) -> libc::c_double;
    fn zslLexValueLteMax(value: sds, spec: *mut zlexrangespec) -> libc::c_int;
    fn zslValueLteMax(value: libc::c_double, spec: *mut zrangespec) -> libc::c_int;
    fn zzlLexValueLteMax(p: *mut libc::c_uchar, spec: *mut zlexrangespec) -> libc::c_int;
    fn lpGetObject(sptr: *mut libc::c_uchar) -> sds;
    fn zslLastInLexRange(
        zsl: *mut zskiplist,
        range: *mut zlexrangespec,
    ) -> *mut zskiplistNode;
    fn zslFirstInLexRange(
        zsl: *mut zskiplist,
        range: *mut zlexrangespec,
    ) -> *mut zskiplistNode;
    fn zzlLastInLexRange(
        zl: *mut libc::c_uchar,
        range: *mut zlexrangespec,
    ) -> *mut libc::c_uchar;
    fn zzlFirstInLexRange(
        zl: *mut libc::c_uchar,
        range: *mut zlexrangespec,
    ) -> *mut libc::c_uchar;
    fn zslParseLexRange(
        min: *mut robj,
        max: *mut robj,
        spec: *mut zlexrangespec,
    ) -> libc::c_int;
    fn zslLastInRange(zsl: *mut zskiplist, range: *mut zrangespec) -> *mut zskiplistNode;
    fn zslFirstInRange(
        zsl: *mut zskiplist,
        range: *mut zrangespec,
    ) -> *mut zskiplistNode;
    fn zzlLastInRange(
        zl: *mut libc::c_uchar,
        range: *mut zrangespec,
    ) -> *mut libc::c_uchar;
    fn zzlFirstInRange(
        zl: *mut libc::c_uchar,
        range: *mut zrangespec,
    ) -> *mut libc::c_uchar;
    fn zsetDel(zobj: *mut robj, ele: sds) -> libc::c_int;
    fn zsetScore(
        zobj: *mut robj,
        member: sds,
        score: *mut libc::c_double,
    ) -> libc::c_int;
    fn zsetAdd(
        zobj: *mut robj,
        score: libc::c_double,
        ele: sds,
        in_flags: libc::c_int,
        out_flags: *mut libc::c_int,
        newscore: *mut libc::c_double,
    ) -> libc::c_int;
    fn dbRandomKey(db: *mut redisDb) -> *mut robj;
    fn restartAOFAfterSYNC();
    fn flushAllDataAndResetRDB(flags: libc::c_int);
    fn stopAppendOnly();
    fn getExpire(db: *mut redisDb, key: *mut robj) -> libc::c_longlong;
    fn removeExpire(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn setExpire(
        c: *mut client,
        db: *mut redisDb,
        key: *mut robj,
        when: libc::c_longlong,
    );
    fn dbUnshareStringValue(db: *mut redisDb, key: *mut robj, o: *mut robj) -> *mut robj;
    fn dbAsyncDelete(db: *mut redisDb, key: *mut robj) -> libc::c_int;
    fn alsoPropagate(
        dbid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
        target: libc::c_int,
    );
    fn call(c: *mut client, flags: libc::c_int);
    fn mustObeyClient(c: *mut client) -> libc::c_int;
    fn getAclErrorMessage(acl_res: libc::c_int) -> *const libc::c_char;
    fn writeCommandsDeniedByDiskError() -> libc::c_int;
    fn writeCommandsGetDiskErrorMessage(_: libc::c_int) -> sds;
    fn checkGoodReplicasStatus() -> libc::c_int;
    fn getCommandFlags(c: *mut client) -> uint64_t;
    fn commandCheckArity(c: *mut client, err: *mut sds) -> libc::c_int;
    fn commandCheckExistence(c: *mut client, err: *mut sds) -> libc::c_int;
    fn getDoubleFromObject(o: *const robj, target: *mut libc::c_double) -> libc::c_int;
    fn listTypeNext(li: *mut listTypeIterator, entry: *mut listTypeEntry) -> libc::c_int;
    fn listTypeDelete(iter: *mut listTypeIterator, entry: *mut listTypeEntry);
    fn listTypeSetIteratorDirection(li: *mut listTypeIterator, direction: libc::c_uchar);
    fn listTypeInitIterator(
        subject: *mut robj,
        index: libc::c_long,
        direction: libc::c_uchar,
    ) -> *mut listTypeIterator;
    fn listTypeInsert(entry: *mut listTypeEntry, value: *mut robj, where_0: libc::c_int);
    fn listTypePush(subject: *mut robj, value: *mut robj, where_0: libc::c_int);
    fn listTypeReplace(entry: *mut listTypeEntry, value: *mut robj);
    fn listTypeGet(entry: *mut listTypeEntry) -> *mut robj;
    fn listTypePop(subject: *mut robj, where_0: libc::c_int) -> *mut robj;
    fn stringObjectLen(o: *mut robj) -> size_t;
    fn lookupKeyReadWithFlags(
        db: *mut redisDb,
        key: *mut robj,
        flags: libc::c_int,
    ) -> *mut robj;
    fn lookupKeyWriteWithFlags(
        db: *mut redisDb,
        key: *mut robj,
        flags: libc::c_int,
    ) -> *mut robj;
    fn addReplyHumanLongDouble(c: *mut client, d: f64);
    fn addReplyBigNum(c: *mut client, num: *const libc::c_char, len: size_t);
    fn addReplyDouble(c: *mut client, d: libc::c_double);
    fn deferredAfterErrorReply(c: *mut client, errors: *mut list);
    fn addReplyProto(c: *mut client, s: *const libc::c_char, len: size_t);
    fn addReplyBool(c: *mut client, b: libc::c_int);
    fn addReplyNull(c: *mut client);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn setDeferredAttributeLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn setDeferredSetLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn addReplyNullArray(c: *mut client);
    fn addReplyAttributeLen(c: *mut client, length: libc::c_long);
    fn addReplySetLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn addReplyErrorFormat(c: *mut client, fmt: *const libc::c_char, _: ...);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyErrorArity(c: *mut client);
    fn catSubCommandFullname(
        parent_name: *const libc::c_char,
        sub_name: *const libc::c_char,
    ) -> sds;
    fn commandAddSubcommand(
        parent: *mut redisCommand,
        subcommand: *mut redisCommand,
        declared_name: *const libc::c_char,
    );
    fn lookupSubcommand(
        container: *mut redisCommand,
        sub_name: sds,
    ) -> *mut redisCommand;
    fn ACLGetCommandID(cmdname: sds) -> libc::c_ulong;
    fn dictSdsDestructor(d: *mut dict, val: *mut libc::c_void);
    fn dictSdsKeyCaseCompare(
        d: *mut dict,
        key1: *const libc::c_void,
        key2: *const libc::c_void,
    ) -> libc::c_int;
    fn dictSdsCaseHash(key: *const libc::c_void) -> uint64_t;
    fn removeConfig(name: sds);
    fn addReplySubcommandSyntaxError(c: *mut client);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn clientHasPendingReplies(c: *mut client) -> libc::c_int;
    fn unblockClient(c: *mut client);
    fn AddReplyFromClient(c: *mut client, src: *mut client);
    fn rdbSaveSingleModuleAux(
        rdb: *mut rio,
        when: libc::c_int,
        mt: *mut moduleType,
    ) -> ssize_t;
    fn exit(_: libc::c_int) -> !;
    fn getNodeByQuery(
        c: *mut client,
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        hashslot: *mut libc::c_int,
        ask: *mut libc::c_int,
    ) -> *mut clusterNode;
    fn clusterLookupNode(
        name: *const libc::c_char,
        length: libc::c_int,
    ) -> *mut clusterNode;
    fn clusterSendModuleMessageToTarget(
        target: *const libc::c_char,
        module_id: uint64_t,
        type_0: uint8_t,
        payload: *const libc::c_char,
        len: uint32_t,
    ) -> libc::c_int;
    fn scriptIsRunning() -> libc::c_int;
    fn callReplyCreate(
        reply: sds,
        deferred_error_list: *mut list,
        private_data: *mut libc::c_void,
    ) -> *mut CallReply;
    fn callReplyCreateError(
        reply: sds,
        private_data: *mut libc::c_void,
    ) -> *mut CallReply;
    fn callReplyType(rep: *mut CallReply) -> libc::c_int;
    fn callReplyGetString(rep: *mut CallReply, len: *mut size_t) -> *const libc::c_char;
    fn callReplyGetLongLong(rep: *mut CallReply) -> libc::c_longlong;
    fn callReplyGetDouble(rep: *mut CallReply) -> libc::c_double;
    fn callReplyGetBool(rep: *mut CallReply) -> libc::c_int;
    fn callReplyGetLen(rep: *mut CallReply) -> size_t;
    fn callReplyGetArrayElement(rep: *mut CallReply, idx: size_t) -> *mut CallReply;
    fn callReplyGetSetElement(rep: *mut CallReply, idx: size_t) -> *mut CallReply;
    fn callReplyGetMapElement(
        rep: *mut CallReply,
        idx: size_t,
        key: *mut *mut CallReply,
        val: *mut *mut CallReply,
    ) -> libc::c_int;
    fn callReplyGetAttribute(rep: *mut CallReply) -> *mut CallReply;
    fn callReplyGetAttributeElement(
        rep: *mut CallReply,
        idx: size_t,
        key: *mut *mut CallReply,
        val: *mut *mut CallReply,
    ) -> libc::c_int;
    fn callReplyGetBigNumber(
        rep: *mut CallReply,
        len: *mut size_t,
    ) -> *const libc::c_char;
    fn callReplyGetVerbatim(
        rep: *mut CallReply,
        len: *mut size_t,
        format: *mut *const libc::c_char,
    ) -> *const libc::c_char;
    fn callReplyGetPrivateData(rep: *mut CallReply) -> *mut libc::c_void;
    fn callReplyIsResp3(rep: *mut CallReply) -> libc::c_int;
    fn callReplyDeferredErrorList(rep: *mut CallReply) -> *mut list;
    fn freeCallReply(rep: *mut CallReply);
    fn callReplyGetProto(rep: *mut CallReply, len: *mut size_t) -> *const libc::c_char;
    fn dlerror() -> *mut libc::c_char;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn dlsym(
        __handle: *mut libc::c_void,
        __name: *const libc::c_char,
    ) -> *mut libc::c_void;
    fn dlopen(__file: *const libc::c_char, __mode: libc::c_int) -> *mut libc::c_void;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
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
pub type __int8_t = libc::c_schar;
pub type __uint8_t = libc::c_uchar;
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino64_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_int;
pub type __blkcnt64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __sig_atomic_t = libc::c_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type mode_t = __mode_t;
pub type off_t = __off64_t;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino64_t,
    pub st_mode: __mode_t,
    pub st_nlink: __nlink_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub st_rdev: __dev_t,
    pub __pad1: __dev_t,
    pub st_size: __off64_t,
    pub st_blksize: __blksize_t,
    pub __pad2: libc::c_int,
    pub st_blocks: __blkcnt64_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [libc::c_int; 2],
}
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_int,
    pub __list: __pthread_list_t,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_long,
}
pub type va_list = __builtin_va_list;
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
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uint_least64_t = __uint_least64_t;
pub type intptr_t = libc::c_long;
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
pub type monotime = uint64_t;
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
pub struct connection {
    pub type_0: *mut ConnectionType,
    pub state: ConnectionState,
    pub flags: libc::c_short,
    pub refs: libc::c_short,
    pub last_errno: libc::c_int,
    pub private_data: *mut libc::c_void,
    pub conn_handler: ConnectionCallbackFunc,
    pub write_handler: ConnectionCallbackFunc,
    pub read_handler: ConnectionCallbackFunc,
    pub fd: libc::c_int,
}
pub type ConnectionCallbackFunc = Option::<unsafe extern "C" fn(*mut connection) -> ()>;
pub type ConnectionState = libc::c_uint;
pub const CONN_STATE_ERROR: ConnectionState = 5;
pub const CONN_STATE_CLOSED: ConnectionState = 4;
pub const CONN_STATE_CONNECTED: ConnectionState = 3;
pub const CONN_STATE_ACCEPTING: ConnectionState = 2;
pub const CONN_STATE_CONNECTING: ConnectionState = 1;
pub const CONN_STATE_NONE: ConnectionState = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ConnectionType {
    pub ae_handler: Option::<
        unsafe extern "C" fn(
            *mut aeEventLoop,
            libc::c_int,
            *mut libc::c_void,
            libc::c_int,
        ) -> (),
    >,
    pub connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            *const libc::c_char,
            ConnectionCallbackFunc,
        ) -> libc::c_int,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut connection, *const libc::c_void, size_t) -> libc::c_int,
    >,
    pub writev: Option::<
        unsafe extern "C" fn(*mut connection, *const iovec, libc::c_int) -> libc::c_int,
    >,
    pub read: Option::<
        unsafe extern "C" fn(*mut connection, *mut libc::c_void, size_t) -> libc::c_int,
    >,
    pub close: Option::<unsafe extern "C" fn(*mut connection) -> ()>,
    pub accept: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub set_write_handler: Option::<
        unsafe extern "C" fn(
            *mut connection,
            ConnectionCallbackFunc,
            libc::c_int,
        ) -> libc::c_int,
    >,
    pub set_read_handler: Option::<
        unsafe extern "C" fn(*mut connection, ConnectionCallbackFunc) -> libc::c_int,
    >,
    pub get_last_error: Option::<
        unsafe extern "C" fn(*mut connection) -> *const libc::c_char,
    >,
    pub blocking_connect: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *const libc::c_char,
            libc::c_int,
            libc::c_longlong,
        ) -> libc::c_int,
    >,
    pub sync_write: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_read: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub sync_readline: Option::<
        unsafe extern "C" fn(
            *mut connection,
            *mut libc::c_char,
            ssize_t,
            libc::c_longlong,
        ) -> ssize_t,
    >,
    pub get_type: Option::<unsafe extern "C" fn(*mut connection) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _rio {
    pub read: Option::<
        unsafe extern "C" fn(*mut _rio, *mut libc::c_void, size_t) -> size_t,
    >,
    pub write: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> size_t,
    >,
    pub tell: Option::<unsafe extern "C" fn(*mut _rio) -> off_t>,
    pub flush: Option::<unsafe extern "C" fn(*mut _rio) -> libc::c_int>,
    pub update_cksum: Option::<
        unsafe extern "C" fn(*mut _rio, *const libc::c_void, size_t) -> (),
    >,
    pub cksum: uint64_t,
    pub flags: uint64_t,
    pub processed_bytes: size_t,
    pub max_processing_chunk: size_t,
    pub io: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub buffer: C2RustUnnamed_3,
    pub file: C2RustUnnamed_2,
    pub conn: C2RustUnnamed_1,
    pub fd: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub fd: libc::c_int,
    pub pos: off_t,
    pub buf: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub conn: *mut connection,
    pub pos: off_t,
    pub buf: sds,
    pub read_limit: size_t,
    pub read_so_far: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub fp: *mut FILE,
    pub buffered: off_t,
    pub autosync: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub ptr: sds,
    pub pos: off_t,
}
pub type rio = _rio;
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
pub type C2RustUnnamed_4 = libc::c_uint;
pub const PTHREAD_MUTEX_FAST_NP: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_DEFAULT: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ERRORCHECK: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_NORMAL: C2RustUnnamed_4 = 0;
pub const PTHREAD_MUTEX_ADAPTIVE_NP: C2RustUnnamed_4 = 3;
pub const PTHREAD_MUTEX_ERRORCHECK_NP: C2RustUnnamed_4 = 2;
pub const PTHREAD_MUTEX_RECURSIVE_NP: C2RustUnnamed_4 = 1;
pub const PTHREAD_MUTEX_TIMED_NP: C2RustUnnamed_4 = 0;
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hdr_histogram {
    pub lowest_discernible_value: int64_t,
    pub highest_trackable_value: int64_t,
    pub unit_magnitude: int32_t,
    pub significant_figures: int32_t,
    pub sub_bucket_half_count_magnitude: int32_t,
    pub sub_bucket_half_count: int32_t,
    pub sub_bucket_mask: int64_t,
    pub sub_bucket_count: int32_t,
    pub bucket_count: int32_t,
    pub min_value: int64_t,
    pub max_value: int64_t,
    pub normalizing_index_offset: int32_t,
    pub conversion_ratio: libc::c_double,
    pub counts_len: int32_t,
    pub total_count: int64_t,
    pub counts: *mut int64_t,
}
pub type mstime_t = libc::c_longlong;
pub type ustime_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub v: C2RustUnnamed_5,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_5 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct intset {
    pub encoding: uint32_t,
    pub length: uint32_t,
    pub contents: [int8_t; 0],
}
pub type ld2string_mode = libc::c_uint;
pub const LD_STR_HEX: ld2string_mode = 2;
pub const LD_STR_HUMAN: ld2string_mode = 1;
pub const LD_STR_AUTO: ld2string_mode = 0;
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct quicklistNode {
    pub prev: *mut quicklistNode,
    pub next: *mut quicklistNode,
    pub entry: *mut libc::c_uchar,
    pub sz: size_t,
    #[bitfield(name = "count", ty = "libc::c_uint", bits = "0..=15")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "16..=17")]
    #[bitfield(name = "container", ty = "libc::c_uint", bits = "18..=19")]
    #[bitfield(name = "recompress", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "attempted_compress", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "dont_compress", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "extra", ty = "libc::c_uint", bits = "23..=31")]
    pub count_encoding_container_recompress_attempted_compress_dont_compress_extra: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistBookmark {
    pub node: *mut quicklistNode,
    pub name: *mut libc::c_char,
}
#[derive(Copy, Clone,c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct quicklist {
    pub head: *mut quicklistNode,
    pub tail: *mut quicklistNode,
    pub count: libc::c_ulong,
    pub len: libc::c_ulong,
    #[bitfield(name = "fill", ty = "libc::c_int", bits = "0..=15")]
    #[bitfield(name = "compress", ty = "libc::c_uint", bits = "16..=31")]
    #[bitfield(name = "bookmark_count", ty = "libc::c_uint", bits = "32..=35")]
    pub fill_compress_bookmark_count: [u8; 5],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub bookmarks: [quicklistBookmark; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistIter {
    pub quicklist: *mut quicklist,
    pub current: *mut quicklistNode,
    pub zi: *mut libc::c_uchar,
    pub offset: libc::c_long,
    pub direction: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quicklistEntry {
    pub quicklist: *const quicklist,
    pub node: *mut quicklistNode,
    pub zi: *mut libc::c_uchar,
    pub value: *mut libc::c_uchar,
    pub longval: libc::c_longlong,
    pub sz: size_t,
    pub offset: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleStreamID {
    pub ms: uint64_t,
    pub seq: uint64_t,
}
pub type RedisModuleTimerID = uint64_t;
pub type RedisModuleCommandArgType = libc::c_uint;
pub const REDISMODULE_ARG_TYPE_BLOCK: RedisModuleCommandArgType = 8;
pub const REDISMODULE_ARG_TYPE_ONEOF: RedisModuleCommandArgType = 7;
pub const REDISMODULE_ARG_TYPE_PURE_TOKEN: RedisModuleCommandArgType = 6;
pub const REDISMODULE_ARG_TYPE_UNIX_TIME: RedisModuleCommandArgType = 5;
pub const REDISMODULE_ARG_TYPE_PATTERN: RedisModuleCommandArgType = 4;
pub const REDISMODULE_ARG_TYPE_KEY: RedisModuleCommandArgType = 3;
pub const REDISMODULE_ARG_TYPE_DOUBLE: RedisModuleCommandArgType = 2;
pub const REDISMODULE_ARG_TYPE_INTEGER: RedisModuleCommandArgType = 1;
pub const REDISMODULE_ARG_TYPE_STRING: RedisModuleCommandArgType = 0;
pub type RedisModuleKeySpecBeginSearchType = libc::c_uint;
pub const REDISMODULE_KSPEC_BS_KEYWORD: RedisModuleKeySpecBeginSearchType = 3;
pub const REDISMODULE_KSPEC_BS_INDEX: RedisModuleKeySpecBeginSearchType = 2;
pub const REDISMODULE_KSPEC_BS_UNKNOWN: RedisModuleKeySpecBeginSearchType = 1;
pub const REDISMODULE_KSPEC_BS_INVALID: RedisModuleKeySpecBeginSearchType = 0;
pub type RedisModuleKeySpecFindKeysType = libc::c_uint;
pub const REDISMODULE_KSPEC_FK_KEYNUM: RedisModuleKeySpecFindKeysType = 3;
pub const REDISMODULE_KSPEC_FK_RANGE: RedisModuleKeySpecFindKeysType = 2;
pub const REDISMODULE_KSPEC_FK_UNKNOWN: RedisModuleKeySpecFindKeysType = 1;
pub const REDISMODULE_KSPEC_FK_OMITTED: RedisModuleKeySpecFindKeysType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandArg {
    pub name: *const libc::c_char,
    pub type_0: RedisModuleCommandArgType,
    pub key_spec_index: libc::c_int,
    pub token: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub since: *const libc::c_char,
    pub flags: libc::c_int,
    pub deprecated_since: *const libc::c_char,
    pub subargs: *mut RedisModuleCommandArg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandHistoryEntry {
    pub since: *const libc::c_char,
    pub changes: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandKeySpec {
    pub notes: *const libc::c_char,
    pub flags: uint64_t,
    pub begin_search_type: RedisModuleKeySpecBeginSearchType,
    pub bs: C2RustUnnamed_9,
    pub find_keys_type: RedisModuleKeySpecFindKeysType,
    pub fk: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub range: C2RustUnnamed_8,
    pub keynum: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub lastkey: libc::c_int,
    pub keystep: libc::c_int,
    pub limit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub index: C2RustUnnamed_11,
    pub keyword: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub pos: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandInfoVersion {
    pub version: libc::c_int,
    pub sizeof_historyentry: size_t,
    pub sizeof_keyspec: size_t,
    pub sizeof_arg: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandInfo {
    pub version: *const RedisModuleCommandInfoVersion,
    pub summary: *const libc::c_char,
    pub complexity: *const libc::c_char,
    pub since: *const libc::c_char,
    pub history: *mut RedisModuleCommandHistoryEntry,
    pub tips: *const libc::c_char,
    pub arity: libc::c_int,
    pub key_specs: *mut RedisModuleCommandKeySpec,
    pub args: *mut RedisModuleCommandArg,
}
pub type RedisModuleEventLoopFunc = Option::<
    unsafe extern "C" fn(libc::c_int, *mut libc::c_void, libc::c_int) -> (),
>;
pub type RedisModuleEventLoopOneShotFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleEvent {
    pub id: uint64_t,
    pub dataver: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCtx {
    pub getapifuncptr: *mut libc::c_void,
    pub module: *mut RedisModule,
    pub client: *mut client,
    pub blocked_client: *mut RedisModuleBlockedClient,
    pub amqueue: *mut AutoMemEntry,
    pub amqueue_len: libc::c_int,
    pub amqueue_used: libc::c_int,
    pub flags: libc::c_int,
    pub postponed_arrays: *mut *mut libc::c_void,
    pub postponed_arrays_count: libc::c_int,
    pub blocked_privdata: *mut libc::c_void,
    pub blocked_ready_key: *mut robj,
    pub keys_result: *mut getKeysResult,
    pub pa_head: *mut RedisModulePoolAllocBlock,
    pub next_yield_time: libc::c_longlong,
    pub user: *const RedisModuleUser,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleUser {
    pub user: *mut user,
    pub free_user: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct user {
    pub name: sds,
    pub flags: uint32_t,
    pub passwords: *mut list,
    pub selectors: *mut list,
    pub acl_string: *mut robj,
}
pub type robj = redisObject;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModulePoolAllocBlock {
    pub size: uint32_t,
    pub used: uint32_t,
    pub next: *mut RedisModulePoolAllocBlock,
    pub memory: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct getKeysResult {
    pub keysbuf: [keyReference; 256],
    pub keys: *mut keyReference,
    pub numkeys: libc::c_int,
    pub size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyReference {
    pub pos: libc::c_int,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AutoMemEntry {
    pub ptr: *mut libc::c_void,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleBlockedClient {
    pub client: *mut client,
    pub module: *mut RedisModule,
    pub reply_callback: RedisModuleCmdFunc,
    pub timeout_callback: RedisModuleCmdFunc,
    pub disconnect_callback: RedisModuleDisconnectFunc,
    pub free_privdata: Option::<
        unsafe extern "C" fn(*mut RedisModuleCtx, *mut libc::c_void) -> (),
    >,
    pub privdata: *mut libc::c_void,
    pub thread_safe_ctx_client: *mut client,
    pub reply_client: *mut client,
    pub dbid: libc::c_int,
    pub blocked_on_keys: libc::c_int,
    pub unblocked: libc::c_int,
    pub background_timer: monotime,
    pub background_duration: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct client {
    pub id: uint64_t,
    pub flags: uint64_t,
    pub conn: *mut connection,
    pub resp: libc::c_int,
    pub db: *mut redisDb,
    pub name: *mut robj,
    pub querybuf: sds,
    pub qb_pos: size_t,
    pub querybuf_peak: size_t,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub original_argc: libc::c_int,
    pub original_argv: *mut *mut robj,
    pub argv_len_sum: size_t,
    pub cmd: *mut redisCommand,
    pub lastcmd: *mut redisCommand,
    pub realcmd: *mut redisCommand,
    pub user: *mut user,
    pub reqtype: libc::c_int,
    pub multibulklen: libc::c_int,
    pub bulklen: libc::c_long,
    pub reply: *mut list,
    pub reply_bytes: libc::c_ulonglong,
    pub deferred_reply_errors: *mut list,
    pub sentlen: size_t,
    pub ctime: time_t,
    pub duration: libc::c_long,
    pub slot: libc::c_int,
    pub cur_script: *mut dictEntry,
    pub lastinteraction: time_t,
    pub obuf_soft_limit_reached_time: time_t,
    pub authenticated: libc::c_int,
    pub replstate: libc::c_int,
    pub repl_start_cmd_stream_on_ack: libc::c_int,
    pub repldbfd: libc::c_int,
    pub repldboff: off_t,
    pub repldbsize: off_t,
    pub replpreamble: sds,
    pub read_reploff: libc::c_longlong,
    pub reploff: libc::c_longlong,
    pub repl_applied: libc::c_longlong,
    pub repl_ack_off: libc::c_longlong,
    pub repl_ack_time: libc::c_longlong,
    pub repl_last_partial_write: libc::c_longlong,
    pub psync_initial_offset: libc::c_longlong,
    pub replid: [libc::c_char; 41],
    pub slave_listening_port: libc::c_int,
    pub slave_addr: *mut libc::c_char,
    pub slave_capa: libc::c_int,
    pub slave_req: libc::c_int,
    pub mstate: multiState,
    pub btype: libc::c_int,
    pub bpop: blockingState,
    pub woff: libc::c_longlong,
    pub watched_keys: *mut list,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut list,
    pub pubsubshard_channels: *mut dict,
    pub peerid: sds,
    pub sockname: sds,
    pub client_list_node: *mut listNode,
    pub postponed_list_node: *mut listNode,
    pub pending_read_list_node: *mut listNode,
    pub auth_callback: RedisModuleUserChangedFunc,
    pub auth_callback_privdata: *mut libc::c_void,
    pub auth_module: *mut libc::c_void,
    pub client_tracking_redirection: uint64_t,
    pub client_tracking_prefixes: *mut rax,
    pub last_memory_usage: size_t,
    pub last_memory_type: libc::c_int,
    pub mem_usage_bucket_node: *mut listNode,
    pub mem_usage_bucket: *mut clientMemUsageBucket,
    pub ref_repl_buf_node: *mut listNode,
    pub ref_block_pos: size_t,
    pub buf_peak: size_t,
    pub buf_peak_last_reset_time: mstime_t,
    pub bufpos: libc::c_int,
    pub buf_usable_size: size_t,
    pub buf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientMemUsageBucket {
    pub clients: *mut list,
    pub mem_usage_sum: size_t,
}
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockingState {
    pub count: libc::c_long,
    pub timeout: mstime_t,
    pub keys: *mut dict,
    pub target: *mut robj,
    pub blockpos: blockPos,
    pub xread_count: size_t,
    pub xread_group: *mut robj,
    pub xread_consumer: *mut robj,
    pub xread_group_noack: libc::c_int,
    pub numreplicas: libc::c_int,
    pub reploffset: libc::c_longlong,
    pub module_blocked_handle: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct blockPos {
    pub wherefrom: libc::c_int,
    pub whereto: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiState {
    pub commands: *mut multiCmd,
    pub count: libc::c_int,
    pub cmd_flags: libc::c_int,
    pub cmd_inv_flags: libc::c_int,
    pub argv_len_sums: size_t,
    pub alloc_count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct multiCmd {
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub argc: libc::c_int,
    pub cmd: *mut redisCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommand {
    pub declared_name: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub complexity: *const libc::c_char,
    pub since: *const libc::c_char,
    pub doc_flags: libc::c_int,
    pub replaced_by: *const libc::c_char,
    pub deprecated_since: *const libc::c_char,
    pub group: redisCommandGroup,
    pub history: *mut commandHistory,
    pub tips: *mut *const libc::c_char,
    pub proc_0: Option::<redisCommandProc>,
    pub arity: libc::c_int,
    pub flags: uint64_t,
    pub acl_categories: uint64_t,
    pub key_specs_static: [keySpec; 4],
    pub getkeys_proc: Option::<redisGetKeysProc>,
    pub subcommands: *mut redisCommand,
    pub args: *mut redisCommandArg,
    pub microseconds: libc::c_longlong,
    pub calls: libc::c_longlong,
    pub rejected_calls: libc::c_longlong,
    pub failed_calls: libc::c_longlong,
    pub id: libc::c_int,
    pub fullname: sds,
    pub latency_histogram: *mut hdr_histogram,
    pub key_specs: *mut keySpec,
    pub legacy_range_key_spec: keySpec,
    pub num_args: libc::c_int,
    pub num_history: libc::c_int,
    pub num_tips: libc::c_int,
    pub key_specs_num: libc::c_int,
    pub key_specs_max: libc::c_int,
    pub subcommands_dict: *mut dict,
    pub parent: *mut redisCommand,
    pub module_cmd: *mut RedisModuleCommand,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommand {
    pub module: *mut RedisModule,
    pub func: RedisModuleCmdFunc,
    pub rediscmd: *mut redisCommand,
}
pub type RedisModuleCmdFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut *mut libc::c_void,
        libc::c_int,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModule {
    pub handle: *mut libc::c_void,
    pub name: *mut libc::c_char,
    pub ver: libc::c_int,
    pub apiver: libc::c_int,
    pub types: *mut list,
    pub usedby: *mut list,
    pub using: *mut list,
    pub filters: *mut list,
    pub module_configs: *mut list,
    pub configs_initialized: libc::c_int,
    pub in_call: libc::c_int,
    pub in_hook: libc::c_int,
    pub options: libc::c_int,
    pub blocked_clients: libc::c_int,
    pub info_cb: RedisModuleInfoFunc,
    pub defrag_cb: RedisModuleDefragFunc,
    pub loadmod: *mut moduleLoadQueueEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleLoadQueueEntry {
    pub path: sds,
    pub argc: libc::c_int,
    pub argv: *mut *mut robj,
}
pub type RedisModuleDefragFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDefragCtx {
    pub defragged: libc::c_long,
    pub endtime: libc::c_longlong,
    pub cursor: *mut libc::c_ulong,
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type RedisModuleInfoFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleInfoCtx, libc::c_int) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleInfoCtx {
    pub module: *mut RedisModule,
    pub requested_sections: *mut dict,
    pub info: sds,
    pub sections: libc::c_int,
    pub in_section: libc::c_int,
    pub in_dict_field: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keySpec {
    pub notes: *const libc::c_char,
    pub flags: uint64_t,
    pub begin_search_type: kspec_bs_type,
    pub bs: C2RustUnnamed_15,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_12,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub range: C2RustUnnamed_14,
    pub keynum: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_13 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub lastkey: libc::c_int,
    pub keystep: libc::c_int,
    pub limit: libc::c_int,
}
pub type kspec_fk_type = libc::c_uint;
pub const KSPEC_FK_KEYNUM: kspec_fk_type = 3;
pub const KSPEC_FK_RANGE: kspec_fk_type = 2;
pub const KSPEC_FK_UNKNOWN: kspec_fk_type = 1;
pub const KSPEC_FK_INVALID: kspec_fk_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_15 {
    pub index: C2RustUnnamed_17,
    pub keyword: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub pos: libc::c_int,
}
pub type kspec_bs_type = libc::c_uint;
pub const KSPEC_BS_KEYWORD: kspec_bs_type = 3;
pub const KSPEC_BS_INDEX: kspec_bs_type = 2;
pub const KSPEC_BS_UNKNOWN: kspec_bs_type = 1;
pub const KSPEC_BS_INVALID: kspec_bs_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisCommandArg {
    pub name: *const libc::c_char,
    pub type_0: redisCommandArgType,
    pub key_spec_index: libc::c_int,
    pub token: *const libc::c_char,
    pub summary: *const libc::c_char,
    pub since: *const libc::c_char,
    pub flags: libc::c_int,
    pub deprecated_since: *const libc::c_char,
    pub subargs: *mut redisCommandArg,
    pub num_args: libc::c_int,
}
pub type redisCommandArgType = libc::c_uint;
pub const ARG_TYPE_BLOCK: redisCommandArgType = 8;
pub const ARG_TYPE_ONEOF: redisCommandArgType = 7;
pub const ARG_TYPE_PURE_TOKEN: redisCommandArgType = 6;
pub const ARG_TYPE_UNIX_TIME: redisCommandArgType = 5;
pub const ARG_TYPE_PATTERN: redisCommandArgType = 4;
pub const ARG_TYPE_KEY: redisCommandArgType = 3;
pub const ARG_TYPE_DOUBLE: redisCommandArgType = 2;
pub const ARG_TYPE_INTEGER: redisCommandArgType = 1;
pub const ARG_TYPE_STRING: redisCommandArgType = 0;
pub type redisGetKeysProc = unsafe extern "C" fn(
    *mut redisCommand,
    *mut *mut robj,
    libc::c_int,
    *mut getKeysResult,
) -> libc::c_int;
pub type redisCommandProc = unsafe extern "C" fn(*mut client) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandHistory {
    pub since: *const libc::c_char,
    pub changes: *const libc::c_char,
}
pub type redisCommandGroup = libc::c_uint;
pub const COMMAND_GROUP_MODULE: redisCommandGroup = 17;
pub const COMMAND_GROUP_BITMAP: redisCommandGroup = 16;
pub const COMMAND_GROUP_STREAM: redisCommandGroup = 15;
pub const COMMAND_GROUP_GEO: redisCommandGroup = 14;
pub const COMMAND_GROUP_SENTINEL: redisCommandGroup = 13;
pub const COMMAND_GROUP_CLUSTER: redisCommandGroup = 12;
pub const COMMAND_GROUP_HYPERLOGLOG: redisCommandGroup = 11;
pub const COMMAND_GROUP_SCRIPTING: redisCommandGroup = 10;
pub const COMMAND_GROUP_SERVER: redisCommandGroup = 9;
pub const COMMAND_GROUP_CONNECTION: redisCommandGroup = 8;
pub const COMMAND_GROUP_TRANSACTIONS: redisCommandGroup = 7;
pub const COMMAND_GROUP_PUBSUB: redisCommandGroup = 6;
pub const COMMAND_GROUP_HASH: redisCommandGroup = 5;
pub const COMMAND_GROUP_SORTED_SET: redisCommandGroup = 4;
pub const COMMAND_GROUP_SET: redisCommandGroup = 3;
pub const COMMAND_GROUP_LIST: redisCommandGroup = 2;
pub const COMMAND_GROUP_STRING: redisCommandGroup = 1;
pub const COMMAND_GROUP_GENERIC: redisCommandGroup = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisDb {
    pub dict: *mut dict,
    pub expires: *mut dict,
    pub blocking_keys: *mut dict,
    pub ready_keys: *mut dict,
    pub watched_keys: *mut dict,
    pub id: libc::c_int,
    pub avg_ttl: libc::c_longlong,
    pub expires_cursor: libc::c_ulong,
    pub defrag_later: *mut list,
    pub slots_to_keys: *mut clusterSlotToKeyMapping,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterSlotToKeyMapping {
    pub by_slot: [slotToKeys; 16384],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slotToKeys {
    pub count: uint64_t,
    pub head: *mut dictEntry,
}
pub type RedisModuleDisconnectFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleBlockedClient) -> (),
>;
pub type RedisModuleEventCallback = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        RedisModuleEvent,
        uint64_t,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleClientInfo {
    pub version: uint64_t,
    pub flags: uint64_t,
    pub id: uint64_t,
    pub addr: [libc::c_char; 46],
    pub port: uint16_t,
    pub db: uint16_t,
}
pub type RedisModuleClientInfoV1 = RedisModuleClientInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleReplicationInfo {
    pub version: uint64_t,
    pub master: libc::c_int,
    pub masterhost: *mut libc::c_char,
    pub masterport: libc::c_int,
    pub replid1: *mut libc::c_char,
    pub replid2: *mut libc::c_char,
    pub repl1_offset: uint64_t,
    pub repl2_offset: uint64_t,
}
pub type RedisModuleReplicationInfoV1 = RedisModuleReplicationInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleFlushInfo {
    pub version: uint64_t,
    pub sync: int32_t,
    pub dbnum: int32_t,
}
pub type RedisModuleFlushInfoV1 = RedisModuleFlushInfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleModuleChange {
    pub version: uint64_t,
    pub module_name: *const libc::c_char,
    pub module_version: int32_t,
}
pub type RedisModuleModuleChangeV1 = RedisModuleModuleChange;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleLoadingProgressInfo {
    pub version: uint64_t,
    pub hz: int32_t,
    pub progress: int32_t,
}
pub type RedisModuleLoadingProgressV1 = RedisModuleLoadingProgressInfo;
pub type RedisModuleACLLogEntryReason = libc::c_uint;
pub const REDISMODULE_ACL_LOG_CHANNEL: RedisModuleACLLogEntryReason = 3;
pub const REDISMODULE_ACL_LOG_KEY: RedisModuleACLLogEntryReason = 2;
pub const REDISMODULE_ACL_LOG_CMD: RedisModuleACLLogEntryReason = 1;
pub const REDISMODULE_ACL_LOG_AUTH: RedisModuleACLLogEntryReason = 0;
pub type C2RustUnnamed_18 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_18 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_18 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_18 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_18 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_18 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_18 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_18 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_18 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_18 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_18 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_18 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_18 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_18 = 0;
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pause_event {
    pub type_0: pause_type,
    pub end: mstime_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleIO {
    pub bytes: size_t,
    pub rio: *mut rio,
    pub type_0: *mut moduleType,
    pub error: libc::c_int,
    pub ver: libc::c_int,
    pub ctx: *mut RedisModuleCtx,
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleType = RedisModuleType;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleType {
    pub id: uint64_t,
    pub module: *mut RedisModule,
    pub rdb_load: moduleTypeLoadFunc,
    pub rdb_save: moduleTypeSaveFunc,
    pub aof_rewrite: moduleTypeRewriteFunc,
    pub mem_usage: moduleTypeMemUsageFunc,
    pub digest: moduleTypeDigestFunc,
    pub free: moduleTypeFreeFunc,
    pub free_effort: moduleTypeFreeEffortFunc,
    pub unlink: moduleTypeUnlinkFunc,
    pub copy: moduleTypeCopyFunc,
    pub defrag: moduleTypeDefragFunc,
    pub aux_load: moduleTypeAuxLoadFunc,
    pub aux_save: moduleTypeAuxSaveFunc,
    pub mem_usage2: moduleTypeMemUsageFunc2,
    pub free_effort2: moduleTypeFreeEffortFunc2,
    pub unlink2: moduleTypeUnlinkFunc2,
    pub copy2: moduleTypeCopyFunc2,
    pub aux_save_triggers: libc::c_int,
    pub name: [libc::c_char; 10],
}
pub type moduleTypeCopyFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleKeyOptCtx {
    pub from_key: *mut redisObject,
    pub to_key: *mut redisObject,
    pub from_dbid: libc::c_int,
    pub to_dbid: libc::c_int,
}
pub type moduleTypeUnlinkFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc2 = Option::<
    unsafe extern "C" fn(*mut RedisModuleKeyOptCtx, *const libc::c_void) -> size_t,
>;
pub type moduleTypeMemUsageFunc2 = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKeyOptCtx,
        *const libc::c_void,
        size_t,
    ) -> size_t,
>;
pub type moduleTypeAuxSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> (),
>;
pub type moduleTypeAuxLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int, libc::c_int) -> libc::c_int,
>;
pub type moduleTypeDefragFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleDefragCtx,
        *mut redisObject,
        *mut *mut libc::c_void,
    ) -> libc::c_int,
>;
pub type moduleTypeCopyFunc = Option::<
    unsafe extern "C" fn(
        *mut redisObject,
        *mut redisObject,
        *const libc::c_void,
    ) -> *mut libc::c_void,
>;
pub type moduleTypeUnlinkFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeFreeEffortFunc = Option::<
    unsafe extern "C" fn(*mut redisObject, *const libc::c_void) -> size_t,
>;
pub type moduleTypeFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type moduleTypeDigestFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleDigest, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDigest {
    pub o: [libc::c_uchar; 20],
    pub x: [libc::c_uchar; 20],
    pub key: *mut redisObject,
    pub dbid: libc::c_int,
}
pub type moduleTypeMemUsageFunc = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> size_t,
>;
pub type moduleTypeRewriteFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut redisObject, *mut libc::c_void) -> (),
>;
pub type moduleTypeSaveFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, *mut libc::c_void) -> (),
>;
pub type moduleTypeLoadFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleIO, libc::c_int) -> *mut libc::c_void,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleValue {
    pub type_0: *mut moduleType,
    pub value: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientReplyBlock {
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replBacklog {
    pub ref_repl_buf_node: *mut listNode,
    pub unindexed_count: size_t,
    pub blocks_index: *mut rax,
    pub histlen: libc::c_longlong,
    pub offset: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saveparam {
    pub seconds: time_t,
    pub changes: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sentinelConfig {
    pub pre_monitor_cfg: *mut list,
    pub monitor_cfg: *mut list,
    pub post_monitor_cfg: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sharedObjectsStruct {
    pub crlf: *mut robj,
    pub ok: *mut robj,
    pub err: *mut robj,
    pub emptybulk: *mut robj,
    pub czero: *mut robj,
    pub cone: *mut robj,
    pub pong: *mut robj,
    pub space: *mut robj,
    pub queued: *mut robj,
    pub null: [*mut robj; 4],
    pub nullarray: [*mut robj; 4],
    pub emptymap: [*mut robj; 4],
    pub emptyset: [*mut robj; 4],
    pub emptyarray: *mut robj,
    pub wrongtypeerr: *mut robj,
    pub nokeyerr: *mut robj,
    pub syntaxerr: *mut robj,
    pub sameobjecterr: *mut robj,
    pub outofrangeerr: *mut robj,
    pub noscripterr: *mut robj,
    pub loadingerr: *mut robj,
    pub slowevalerr: *mut robj,
    pub slowscripterr: *mut robj,
    pub slowmoduleerr: *mut robj,
    pub bgsaveerr: *mut robj,
    pub masterdownerr: *mut robj,
    pub roslaveerr: *mut robj,
    pub execaborterr: *mut robj,
    pub noautherr: *mut robj,
    pub noreplicaserr: *mut robj,
    pub busykeyerr: *mut robj,
    pub oomerr: *mut robj,
    pub plus: *mut robj,
    pub messagebulk: *mut robj,
    pub pmessagebulk: *mut robj,
    pub subscribebulk: *mut robj,
    pub unsubscribebulk: *mut robj,
    pub psubscribebulk: *mut robj,
    pub punsubscribebulk: *mut robj,
    pub del: *mut robj,
    pub unlink: *mut robj,
    pub rpop: *mut robj,
    pub lpop: *mut robj,
    pub lpush: *mut robj,
    pub rpoplpush: *mut robj,
    pub lmove: *mut robj,
    pub blmove: *mut robj,
    pub zpopmin: *mut robj,
    pub zpopmax: *mut robj,
    pub emptyscan: *mut robj,
    pub multi: *mut robj,
    pub exec: *mut robj,
    pub left: *mut robj,
    pub right: *mut robj,
    pub hset: *mut robj,
    pub srem: *mut robj,
    pub xgroup: *mut robj,
    pub xclaim: *mut robj,
    pub script: *mut robj,
    pub replconf: *mut robj,
    pub eval: *mut robj,
    pub persist: *mut robj,
    pub set: *mut robj,
    pub pexpireat: *mut robj,
    pub pexpire: *mut robj,
    pub time: *mut robj,
    pub pxat: *mut robj,
    pub absttl: *mut robj,
    pub retrycount: *mut robj,
    pub force: *mut robj,
    pub justid: *mut robj,
    pub entriesread: *mut robj,
    pub lastid: *mut robj,
    pub ping: *mut robj,
    pub setid: *mut robj,
    pub keepttl: *mut robj,
    pub load: *mut robj,
    pub createconsumer: *mut robj,
    pub getack: *mut robj,
    pub special_asterick: *mut robj,
    pub special_equals: *mut robj,
    pub default_username: *mut robj,
    pub redacted: *mut robj,
    pub ssubscribebulk: *mut robj,
    pub sunsubscribebulk: *mut robj,
    pub smessagebulk: *mut robj,
    pub select: [*mut robj; 10],
    pub integers: [*mut robj; 10000],
    pub mbulkhdr: [*mut robj; 32],
    pub bulkhdr: [*mut robj; 32],
    pub maphdr: [*mut robj; 32],
    pub sethdr: [*mut robj; 32],
    pub minstring: sds,
    pub maxstring: sds,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplistNode {
    pub ele: sds,
    pub score: libc::c_double,
    pub backward: *mut zskiplistNode,
    pub level: [zskiplistLevel; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplistLevel {
    pub forward: *mut zskiplistNode,
    pub span: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zskiplist {
    pub header: *mut zskiplistNode,
    pub tail: *mut zskiplistNode,
    pub length: libc::c_ulong,
    pub level: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zset {
    pub dict: *mut dict,
    pub zsl: *mut zskiplist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientBufferLimitsConfig {
    pub hard_limit_bytes: libc::c_ulonglong,
    pub soft_limit_bytes: libc::c_ulonglong,
    pub soft_limit_seconds: time_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOp {
    pub argv: *mut *mut robj,
    pub argc: libc::c_int,
    pub dbid: libc::c_int,
    pub target: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisOpArray {
    pub ops: *mut redisOp,
    pub numops: libc::c_int,
    pub capacity: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct malloc_stats {
    pub zmalloc_used: size_t,
    pub process_rss: size_t,
    pub allocator_allocated: size_t,
    pub allocator_active: size_t,
    pub allocator_resident: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct socketFds {
    pub fd: [libc::c_int; 16],
    pub count: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisTLSContextConfig {
    pub cert_file: *mut libc::c_char,
    pub key_file: *mut libc::c_char,
    pub key_file_pass: *mut libc::c_char,
    pub client_cert_file: *mut libc::c_char,
    pub client_key_file: *mut libc::c_char,
    pub client_key_file_pass: *mut libc::c_char,
    pub dh_params_file: *mut libc::c_char,
    pub ca_cert_file: *mut libc::c_char,
    pub ca_cert_dir: *mut libc::c_char,
    pub protocols: *mut libc::c_char,
    pub ciphers: *mut libc::c_char,
    pub ciphersuites: *mut libc::c_char,
    pub prefer_server_ciphers: libc::c_int,
    pub session_caching: libc::c_int,
    pub session_cache_size: libc::c_int,
    pub session_cache_timeout: libc::c_int,
}
pub type aof_file_type = libc::c_uint;
pub const AOF_FILE_TYPE_INCR: aof_file_type = 105;
pub const AOF_FILE_TYPE_HIST: aof_file_type = 104;
pub const AOF_FILE_TYPE_BASE: aof_file_type = 98;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofInfo {
    pub file_name: sds,
    pub file_seq: libc::c_longlong,
    pub file_type: aof_file_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aofManifest {
    pub base_aof_info: *mut aofInfo,
    pub incr_aof_list: *mut list,
    pub history_aof_list: *mut list,
    pub curr_base_file_seq: libc::c_longlong,
    pub curr_incr_file_seq: libc::c_longlong,
    pub dirty: libc::c_int,
}
pub type childInfoType = libc::c_uint;
pub const CHILD_INFO_TYPE_MODULE_COW_SIZE: childInfoType = 3;
pub const CHILD_INFO_TYPE_RDB_COW_SIZE: childInfoType = 2;
pub const CHILD_INFO_TYPE_AOF_COW_SIZE: childInfoType = 1;
pub const CHILD_INFO_TYPE_CURRENT_INFO: childInfoType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisServer {
    pub pid: pid_t,
    pub main_thread_id: pthread_t,
    pub configfile: *mut libc::c_char,
    pub executable: *mut libc::c_char,
    pub exec_argv: *mut *mut libc::c_char,
    pub dynamic_hz: libc::c_int,
    pub config_hz: libc::c_int,
    pub umask: mode_t,
    pub hz: libc::c_int,
    pub in_fork_child: libc::c_int,
    pub db: *mut redisDb,
    pub commands: *mut dict,
    pub orig_commands: *mut dict,
    pub el: *mut aeEventLoop,
    pub errors: *mut rax,
    pub lruclock: atomic_uint,
    pub shutdown_asap: sig_atomic_t,
    pub shutdown_mstime: mstime_t,
    pub last_sig_received: libc::c_int,
    pub shutdown_flags: libc::c_int,
    pub activerehashing: libc::c_int,
    pub active_defrag_running: libc::c_int,
    pub pidfile: *mut libc::c_char,
    pub arch_bits: libc::c_int,
    pub cronloops: libc::c_int,
    pub runid: [libc::c_char; 41],
    pub sentinel_mode: libc::c_int,
    pub initial_memory_usage: size_t,
    pub always_show_logo: libc::c_int,
    pub in_exec: libc::c_int,
    pub busy_module_yield_flags: libc::c_int,
    pub busy_module_yield_reply: *const libc::c_char,
    pub core_propagates: libc::c_int,
    pub propagate_no_multi: libc::c_int,
    pub module_ctx_nesting: libc::c_int,
    pub ignore_warnings: *mut libc::c_char,
    pub client_pause_in_transaction: libc::c_int,
    pub thp_enabled: libc::c_int,
    pub page_size: size_t,
    pub moduleapi: *mut dict,
    pub sharedapi: *mut dict,
    pub module_configs_queue: *mut dict,
    pub loadmodule_queue: *mut list,
    pub module_pipe: [libc::c_int; 2],
    pub child_pid: pid_t,
    pub child_type: libc::c_int,
    pub port: libc::c_int,
    pub tls_port: libc::c_int,
    pub tcp_backlog: libc::c_int,
    pub bindaddr: [*mut libc::c_char; 16],
    pub bindaddr_count: libc::c_int,
    pub bind_source_addr: *mut libc::c_char,
    pub unixsocket: *mut libc::c_char,
    pub unixsocketperm: libc::c_uint,
    pub ipfd: socketFds,
    pub tlsfd: socketFds,
    pub sofd: libc::c_int,
    pub socket_mark_id: uint32_t,
    pub cfd: socketFds,
    pub clients: *mut list,
    pub clients_to_close: *mut list,
    pub clients_pending_write: *mut list,
    pub clients_pending_read: *mut list,
    pub slaves: *mut list,
    pub monitors: *mut list,
    pub current_client: *mut client,
    pub client_mem_usage_buckets: *mut clientMemUsageBucket,
    pub clients_timeout_table: *mut rax,
    pub fixed_time_expire: libc::c_long,
    pub in_nested_call: libc::c_int,
    pub clients_index: *mut rax,
    pub client_pause_type: pause_type,
    pub postponed_clients: *mut list,
    pub client_pause_end_time: mstime_t,
    pub client_pause_per_purpose: [*mut pause_event; 3],
    pub neterr: [libc::c_char; 256],
    pub migrate_cached_sockets: *mut dict,
    pub next_client_id: uint_least64_t,
    pub protected_mode: libc::c_int,
    pub io_threads_num: libc::c_int,
    pub io_threads_do_reads: libc::c_int,
    pub io_threads_active: libc::c_int,
    pub events_processed_while_blocked: libc::c_longlong,
    pub enable_protected_configs: libc::c_int,
    pub enable_debug_cmd: libc::c_int,
    pub enable_module_cmd: libc::c_int,
    pub loading: sig_atomic_t,
    pub async_loading: sig_atomic_t,
    pub loading_total_bytes: off_t,
    pub loading_rdb_used_mem: off_t,
    pub loading_loaded_bytes: off_t,
    pub loading_start_time: time_t,
    pub loading_process_events_interval_bytes: off_t,
    pub stat_starttime: time_t,
    pub stat_numcommands: libc::c_longlong,
    pub stat_numconnections: libc::c_longlong,
    pub stat_expiredkeys: libc::c_longlong,
    pub stat_expired_stale_perc: libc::c_double,
    pub stat_expired_time_cap_reached_count: libc::c_longlong,
    pub stat_expire_cycle_time_used: libc::c_longlong,
    pub stat_evictedkeys: libc::c_longlong,
    pub stat_evictedclients: libc::c_longlong,
    pub stat_total_eviction_exceeded_time: libc::c_longlong,
    pub stat_last_eviction_exceeded_time: monotime,
    pub stat_keyspace_hits: libc::c_longlong,
    pub stat_keyspace_misses: libc::c_longlong,
    pub stat_active_defrag_hits: libc::c_longlong,
    pub stat_active_defrag_misses: libc::c_longlong,
    pub stat_active_defrag_key_hits: libc::c_longlong,
    pub stat_active_defrag_key_misses: libc::c_longlong,
    pub stat_active_defrag_scanned: libc::c_longlong,
    pub stat_total_active_defrag_time: libc::c_longlong,
    pub stat_last_active_defrag_time: monotime,
    pub stat_peak_memory: size_t,
    pub stat_aof_rewrites: libc::c_longlong,
    pub stat_aofrw_consecutive_failures: libc::c_longlong,
    pub stat_rdb_saves: libc::c_longlong,
    pub stat_fork_time: libc::c_longlong,
    pub stat_fork_rate: libc::c_double,
    pub stat_total_forks: libc::c_longlong,
    pub stat_rejected_conn: libc::c_longlong,
    pub stat_sync_full: libc::c_longlong,
    pub stat_sync_partial_ok: libc::c_longlong,
    pub stat_sync_partial_err: libc::c_longlong,
    pub slowlog: *mut list,
    pub slowlog_entry_id: libc::c_longlong,
    pub slowlog_log_slower_than: libc::c_longlong,
    pub slowlog_max_len: libc::c_ulong,
    pub cron_malloc_stats: malloc_stats,
    pub stat_net_input_bytes: atomic_llong,
    pub stat_net_output_bytes: atomic_llong,
    pub stat_net_repl_input_bytes: atomic_llong,
    pub stat_net_repl_output_bytes: atomic_llong,
    pub stat_current_cow_peak: size_t,
    pub stat_current_cow_bytes: size_t,
    pub stat_current_cow_updated: monotime,
    pub stat_current_save_keys_processed: size_t,
    pub stat_current_save_keys_total: size_t,
    pub stat_rdb_cow_bytes: size_t,
    pub stat_aof_cow_bytes: size_t,
    pub stat_module_cow_bytes: size_t,
    pub stat_module_progress: libc::c_double,
    pub stat_clients_type_memory: [size_t; 4],
    pub stat_cluster_links_memory: size_t,
    pub stat_unexpected_error_replies: libc::c_longlong,
    pub stat_total_error_replies: libc::c_longlong,
    pub stat_dump_payload_sanitizations: libc::c_longlong,
    pub stat_io_reads_processed: libc::c_longlong,
    pub stat_io_writes_processed: libc::c_longlong,
    pub stat_total_reads_processed: atomic_llong,
    pub stat_total_writes_processed: atomic_llong,
    pub inst_metric: [C2RustUnnamed_19; 5],
    pub stat_reply_buffer_shrinks: libc::c_longlong,
    pub stat_reply_buffer_expands: libc::c_longlong,
    pub verbosity: libc::c_int,
    pub maxidletime: libc::c_int,
    pub tcpkeepalive: libc::c_int,
    pub active_expire_enabled: libc::c_int,
    pub active_expire_effort: libc::c_int,
    pub active_defrag_enabled: libc::c_int,
    pub sanitize_dump_payload: libc::c_int,
    pub skip_checksum_validation: libc::c_int,
    pub jemalloc_bg_thread: libc::c_int,
    pub active_defrag_ignore_bytes: size_t,
    pub active_defrag_threshold_lower: libc::c_int,
    pub active_defrag_threshold_upper: libc::c_int,
    pub active_defrag_cycle_min: libc::c_int,
    pub active_defrag_cycle_max: libc::c_int,
    pub active_defrag_max_scan_fields: libc::c_ulong,
    pub client_max_querybuf_len: size_t,
    pub dbnum: libc::c_int,
    pub supervised: libc::c_int,
    pub supervised_mode: libc::c_int,
    pub daemonize: libc::c_int,
    pub set_proc_title: libc::c_int,
    pub proc_title_template: *mut libc::c_char,
    pub client_obuf_limits: [clientBufferLimitsConfig; 3],
    pub pause_cron: libc::c_int,
    pub latency_tracking_enabled: libc::c_int,
    pub latency_tracking_info_percentiles: *mut libc::c_double,
    pub latency_tracking_info_percentiles_len: libc::c_int,
    pub aof_enabled: libc::c_int,
    pub aof_state: libc::c_int,
    pub aof_fsync: libc::c_int,
    pub aof_filename: *mut libc::c_char,
    pub aof_dirname: *mut libc::c_char,
    pub aof_no_fsync_on_rewrite: libc::c_int,
    pub aof_rewrite_perc: libc::c_int,
    pub aof_rewrite_min_size: off_t,
    pub aof_rewrite_base_size: off_t,
    pub aof_current_size: off_t,
    pub aof_last_incr_size: off_t,
    pub aof_fsync_offset: off_t,
    pub aof_flush_sleep: libc::c_int,
    pub aof_rewrite_scheduled: libc::c_int,
    pub aof_buf: sds,
    pub aof_fd: libc::c_int,
    pub aof_selected_db: libc::c_int,
    pub aof_flush_postponed_start: time_t,
    pub aof_last_fsync: time_t,
    pub aof_rewrite_time_last: time_t,
    pub aof_rewrite_time_start: time_t,
    pub aof_cur_timestamp: time_t,
    pub aof_timestamp_enabled: libc::c_int,
    pub aof_lastbgrewrite_status: libc::c_int,
    pub aof_delayed_fsync: libc::c_ulong,
    pub aof_rewrite_incremental_fsync: libc::c_int,
    pub rdb_save_incremental_fsync: libc::c_int,
    pub aof_last_write_status: libc::c_int,
    pub aof_last_write_errno: libc::c_int,
    pub aof_load_truncated: libc::c_int,
    pub aof_use_rdb_preamble: libc::c_int,
    pub aof_bio_fsync_status: atomic_int,
    pub aof_bio_fsync_errno: atomic_int,
    pub aof_manifest: *mut aofManifest,
    pub aof_disable_auto_gc: libc::c_int,
    pub dirty: libc::c_longlong,
    pub dirty_before_bgsave: libc::c_longlong,
    pub rdb_last_load_keys_expired: libc::c_longlong,
    pub rdb_last_load_keys_loaded: libc::c_longlong,
    pub saveparams: *mut saveparam,
    pub saveparamslen: libc::c_int,
    pub rdb_filename: *mut libc::c_char,
    pub rdb_compression: libc::c_int,
    pub rdb_checksum: libc::c_int,
    pub rdb_del_sync_files: libc::c_int,
    pub lastsave: time_t,
    pub lastbgsave_try: time_t,
    pub rdb_save_time_last: time_t,
    pub rdb_save_time_start: time_t,
    pub rdb_bgsave_scheduled: libc::c_int,
    pub rdb_child_type: libc::c_int,
    pub lastbgsave_status: libc::c_int,
    pub stop_writes_on_bgsave_err: libc::c_int,
    pub rdb_pipe_read: libc::c_int,
    pub rdb_child_exit_pipe: libc::c_int,
    pub rdb_pipe_conns: *mut *mut connection,
    pub rdb_pipe_numconns: libc::c_int,
    pub rdb_pipe_numconns_writing: libc::c_int,
    pub rdb_pipe_buff: *mut libc::c_char,
    pub rdb_pipe_bufflen: libc::c_int,
    pub rdb_key_save_delay: libc::c_int,
    pub key_load_delay: libc::c_int,
    pub child_info_pipe: [libc::c_int; 2],
    pub child_info_nread: libc::c_int,
    pub also_propagate: redisOpArray,
    pub replication_allowed: libc::c_int,
    pub logfile: *mut libc::c_char,
    pub syslog_enabled: libc::c_int,
    pub syslog_ident: *mut libc::c_char,
    pub syslog_facility: libc::c_int,
    pub crashlog_enabled: libc::c_int,
    pub memcheck_enabled: libc::c_int,
    pub use_exit_on_panic: libc::c_int,
    pub shutdown_timeout: libc::c_int,
    pub shutdown_on_sigint: libc::c_int,
    pub shutdown_on_sigterm: libc::c_int,
    pub replid: [libc::c_char; 41],
    pub replid2: [libc::c_char; 41],
    pub master_repl_offset: libc::c_longlong,
    pub second_replid_offset: libc::c_longlong,
    pub slaveseldb: libc::c_int,
    pub repl_ping_slave_period: libc::c_int,
    pub repl_backlog: *mut replBacklog,
    pub repl_backlog_size: libc::c_longlong,
    pub repl_backlog_time_limit: time_t,
    pub repl_no_slaves_since: time_t,
    pub repl_min_slaves_to_write: libc::c_int,
    pub repl_min_slaves_max_lag: libc::c_int,
    pub repl_good_slaves_count: libc::c_int,
    pub repl_diskless_sync: libc::c_int,
    pub repl_diskless_load: libc::c_int,
    pub repl_diskless_sync_delay: libc::c_int,
    pub repl_diskless_sync_max_replicas: libc::c_int,
    pub repl_buffer_mem: size_t,
    pub repl_buffer_blocks: *mut list,
    pub masteruser: *mut libc::c_char,
    pub masterauth: sds,
    pub masterhost: *mut libc::c_char,
    pub masterport: libc::c_int,
    pub repl_timeout: libc::c_int,
    pub master: *mut client,
    pub cached_master: *mut client,
    pub repl_syncio_timeout: libc::c_int,
    pub repl_state: libc::c_int,
    pub repl_transfer_size: off_t,
    pub repl_transfer_read: off_t,
    pub repl_transfer_last_fsync_off: off_t,
    pub repl_transfer_s: *mut connection,
    pub repl_transfer_fd: libc::c_int,
    pub repl_transfer_tmpfile: *mut libc::c_char,
    pub repl_transfer_lastio: time_t,
    pub repl_serve_stale_data: libc::c_int,
    pub repl_slave_ro: libc::c_int,
    pub repl_slave_ignore_maxmemory: libc::c_int,
    pub repl_down_since: time_t,
    pub repl_disable_tcp_nodelay: libc::c_int,
    pub slave_priority: libc::c_int,
    pub replica_announced: libc::c_int,
    pub slave_announce_port: libc::c_int,
    pub slave_announce_ip: *mut libc::c_char,
    pub propagation_error_behavior: libc::c_int,
    pub repl_ignore_disk_write_error: libc::c_int,
    pub master_replid: [libc::c_char; 41],
    pub master_initial_offset: libc::c_longlong,
    pub repl_slave_lazy_flush: libc::c_int,
    pub clients_waiting_acks: *mut list,
    pub get_ack_from_slaves: libc::c_int,
    pub maxclients: libc::c_uint,
    pub maxmemory: libc::c_ulonglong,
    pub maxmemory_clients: ssize_t,
    pub maxmemory_policy: libc::c_int,
    pub maxmemory_samples: libc::c_int,
    pub maxmemory_eviction_tenacity: libc::c_int,
    pub lfu_log_factor: libc::c_int,
    pub lfu_decay_time: libc::c_int,
    pub proto_max_bulk_len: libc::c_longlong,
    pub oom_score_adj_values: [libc::c_int; 3],
    pub oom_score_adj: libc::c_int,
    pub disable_thp: libc::c_int,
    pub blocked_clients: libc::c_uint,
    pub blocked_clients_by_type: [libc::c_uint; 8],
    pub unblocked_clients: *mut list,
    pub ready_keys: *mut list,
    pub tracking_clients: libc::c_uint,
    pub tracking_table_max_keys: size_t,
    pub tracking_pending_keys: *mut list,
    pub sort_desc: libc::c_int,
    pub sort_alpha: libc::c_int,
    pub sort_bypattern: libc::c_int,
    pub sort_store: libc::c_int,
    pub hash_max_listpack_entries: size_t,
    pub hash_max_listpack_value: size_t,
    pub set_max_intset_entries: size_t,
    pub zset_max_listpack_entries: size_t,
    pub zset_max_listpack_value: size_t,
    pub hll_sparse_max_bytes: size_t,
    pub stream_node_max_bytes: size_t,
    pub stream_node_max_entries: libc::c_longlong,
    pub list_max_listpack_size: libc::c_int,
    pub list_compress_depth: libc::c_int,
    pub unixtime: atomic_int,
    pub timezone: time_t,
    pub daylight_active: libc::c_int,
    pub mstime: mstime_t,
    pub ustime: ustime_t,
    pub blocking_op_nesting: size_t,
    pub blocked_last_cron: libc::c_longlong,
    pub pubsub_channels: *mut dict,
    pub pubsub_patterns: *mut dict,
    pub notify_keyspace_events: libc::c_int,
    pub pubsubshard_channels: *mut dict,
    pub cluster_enabled: libc::c_int,
    pub cluster_port: libc::c_int,
    pub cluster_node_timeout: mstime_t,
    pub cluster_configfile: *mut libc::c_char,
    pub cluster: *mut clusterState,
    pub cluster_migration_barrier: libc::c_int,
    pub cluster_allow_replica_migration: libc::c_int,
    pub cluster_slave_validity_factor: libc::c_int,
    pub cluster_require_full_coverage: libc::c_int,
    pub cluster_slave_no_failover: libc::c_int,
    pub cluster_announce_ip: *mut libc::c_char,
    pub cluster_announce_hostname: *mut libc::c_char,
    pub cluster_preferred_endpoint_type: libc::c_int,
    pub cluster_announce_port: libc::c_int,
    pub cluster_announce_tls_port: libc::c_int,
    pub cluster_announce_bus_port: libc::c_int,
    pub cluster_module_flags: libc::c_int,
    pub cluster_allow_reads_when_down: libc::c_int,
    pub cluster_config_file_lock_fd: libc::c_int,
    pub cluster_link_sendbuf_limit_bytes: libc::c_ulonglong,
    pub cluster_drop_packet_filter: libc::c_int,
    pub script_caller: *mut client,
    pub busy_reply_threshold: mstime_t,
    pub pre_command_oom_state: libc::c_int,
    pub script_disable_deny_script: libc::c_int,
    pub lazyfree_lazy_eviction: libc::c_int,
    pub lazyfree_lazy_expire: libc::c_int,
    pub lazyfree_lazy_server_del: libc::c_int,
    pub lazyfree_lazy_user_del: libc::c_int,
    pub lazyfree_lazy_user_flush: libc::c_int,
    pub latency_monitor_threshold: libc::c_longlong,
    pub latency_events: *mut dict,
    pub acl_filename: *mut libc::c_char,
    pub acllog_max_len: libc::c_ulong,
    pub requirepass: sds,
    pub acl_pubsub_default: libc::c_int,
    pub watchdog_period: libc::c_int,
    pub system_memory_size: size_t,
    pub tls_cluster: libc::c_int,
    pub tls_replication: libc::c_int,
    pub tls_auth_clients: libc::c_int,
    pub tls_ctx_config: redisTLSContextConfig,
    pub server_cpulist: *mut libc::c_char,
    pub bio_cpulist: *mut libc::c_char,
    pub aof_rewrite_cpulist: *mut libc::c_char,
    pub bgsave_cpulist: *mut libc::c_char,
    pub sentinel_config: *mut sentinelConfig,
    pub failover_end_time: mstime_t,
    pub force_failover: libc::c_int,
    pub target_replica_host: *mut libc::c_char,
    pub target_replica_port: libc::c_int,
    pub failover_state: libc::c_int,
    pub cluster_allow_pubsubshard_when_down: libc::c_int,
    pub reply_buffer_peak_reset_time: libc::c_long,
    pub reply_buffer_resizing_enabled: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterState {
    pub myself: *mut clusterNode,
    pub currentEpoch: uint64_t,
    pub state: libc::c_int,
    pub size: libc::c_int,
    pub nodes: *mut dict,
    pub nodes_black_list: *mut dict,
    pub migrating_slots_to: [*mut clusterNode; 16384],
    pub importing_slots_from: [*mut clusterNode; 16384],
    pub slots: [*mut clusterNode; 16384],
    pub slots_to_channels: *mut rax,
    pub failover_auth_time: mstime_t,
    pub failover_auth_count: libc::c_int,
    pub failover_auth_sent: libc::c_int,
    pub failover_auth_rank: libc::c_int,
    pub failover_auth_epoch: uint64_t,
    pub cant_failover_reason: libc::c_int,
    pub mf_end: mstime_t,
    pub mf_slave: *mut clusterNode,
    pub mf_master_offset: libc::c_longlong,
    pub mf_can_start: libc::c_int,
    pub lastVoteEpoch: uint64_t,
    pub todo_before_sleep: libc::c_int,
    pub stats_bus_messages_sent: [libc::c_longlong; 11],
    pub stats_bus_messages_received: [libc::c_longlong; 11],
    pub stats_pfail_nodes: libc::c_longlong,
    pub stat_cluster_links_buffer_limit_exceeded: libc::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterNode {
    pub ctime: mstime_t,
    pub name: [libc::c_char; 40],
    pub flags: libc::c_int,
    pub configEpoch: uint64_t,
    pub slots: [libc::c_uchar; 2048],
    pub slot_info_pairs: *mut uint16_t,
    pub slot_info_pairs_count: libc::c_int,
    pub numslots: libc::c_int,
    pub numslaves: libc::c_int,
    pub slaves: *mut *mut clusterNode,
    pub slaveof: *mut clusterNode,
    pub last_in_ping_gossip: libc::c_ulonglong,
    pub ping_sent: mstime_t,
    pub pong_received: mstime_t,
    pub data_received: mstime_t,
    pub fail_time: mstime_t,
    pub voted_time: mstime_t,
    pub repl_offset_time: mstime_t,
    pub orphaned_time: mstime_t,
    pub repl_offset: libc::c_longlong,
    pub ip: [libc::c_char; 46],
    pub hostname: sds,
    pub port: libc::c_int,
    pub pport: libc::c_int,
    pub cport: libc::c_int,
    pub link: *mut clusterLink,
    pub inbound_link: *mut clusterLink,
    pub fail_reports: *mut list,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterLink {
    pub ctime: mstime_t,
    pub conn: *mut connection,
    pub sndbuf: sds,
    pub rcvbuf: *mut libc::c_char,
    pub rcvbuf_len: size_t,
    pub rcvbuf_alloc: size_t,
    pub node: *mut clusterNode,
    pub inbound: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listTypeIterator {
    pub subject: *mut robj,
    pub encoding: libc::c_uchar,
    pub direction: libc::c_uchar,
    pub iter: *mut quicklistIter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct listTypeEntry {
    pub li: *mut listTypeIterator,
    pub entry: quicklistEntry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamID {
    pub ms: uint64_t,
    pub seq: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stream {
    pub rax: *mut rax,
    pub length: uint64_t,
    pub last_id: streamID,
    pub first_id: streamID,
    pub max_deleted_entry_id: streamID,
    pub entries_added: uint64_t,
    pub cgroups: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct streamIterator {
    pub stream: *mut stream,
    pub master_id: streamID,
    pub master_fields_count: uint64_t,
    pub master_fields_start: *mut libc::c_uchar,
    pub master_fields_ptr: *mut libc::c_uchar,
    pub entry_flags: libc::c_int,
    pub rev: libc::c_int,
    pub skip_tombstones: libc::c_int,
    pub start_key: [uint64_t; 2],
    pub end_key: [uint64_t; 2],
    pub ri: raxIterator,
    pub lp: *mut libc::c_uchar,
    pub lp_ele: *mut libc::c_uchar,
    pub lp_flags: *mut libc::c_uchar,
    pub field_buf: [libc::c_uchar; 21],
    pub value_buf: [libc::c_uchar; 21],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModuleConfig {
    pub name: sds,
    pub privdata: *mut libc::c_void,
    pub get_fn: get_fn,
    pub set_fn: set_fn,
    pub apply_fn: RedisModuleConfigApplyFunc,
    pub module: *mut RedisModule,
}
pub type RedisModuleConfigApplyFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut libc::c_void,
        *mut *mut robj,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union set_fn {
    pub set_string: RedisModuleConfigSetStringFunc,
    pub set_numeric: RedisModuleConfigSetNumericFunc,
    pub set_bool: RedisModuleConfigSetBoolFunc,
    pub set_enum: RedisModuleConfigSetEnumFunc,
}
pub type RedisModuleConfigSetEnumFunc = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
        *mut *mut robj,
    ) -> libc::c_int,
>;
pub type RedisModuleConfigSetBoolFunc = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
        *mut *mut robj,
    ) -> libc::c_int,
>;
pub type RedisModuleConfigSetNumericFunc = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        libc::c_longlong,
        *mut libc::c_void,
        *mut *mut robj,
    ) -> libc::c_int,
>;
pub type RedisModuleConfigSetStringFunc = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *mut robj,
        *mut libc::c_void,
        *mut *mut robj,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union get_fn {
    pub get_string: RedisModuleConfigGetStringFunc,
    pub get_numeric: RedisModuleConfigGetNumericFunc,
    pub get_bool: RedisModuleConfigGetBoolFunc,
    pub get_enum: RedisModuleConfigGetEnumFunc,
}
pub type RedisModuleConfigGetEnumFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
>;
pub type RedisModuleConfigGetBoolFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_int,
>;
pub type RedisModuleConfigGetNumericFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> libc::c_longlong,
>;
pub type RedisModuleConfigGetStringFunc = Option::<
    unsafe extern "C" fn(*const libc::c_char, *mut libc::c_void) -> *mut robj,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct configEnum {
    pub name: *mut libc::c_char,
    pub val: libc::c_int,
}
pub type configType = libc::c_uint;
pub const SPECIAL_CONFIG: configType = 5;
pub const ENUM_CONFIG: configType = 4;
pub const SDS_CONFIG: configType = 3;
pub const STRING_CONFIG: configType = 2;
pub const NUMERIC_CONFIG: configType = 1;
pub const BOOL_CONFIG: configType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventLoopOneShot {
    pub func: RedisModuleEventLoopOneShotFunc,
    pub user_data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EventLoopData {
    pub rFunc: RedisModuleEventLoopFunc,
    pub wFunc: RedisModuleEventLoopFunc,
    pub user_data: *mut libc::c_void,
}
pub type RedisModuleScanKeyCB = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleKey,
        *mut robj,
        *mut robj,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleKey {
    pub ctx: *mut RedisModuleCtx,
    pub db: *mut redisDb,
    pub key: *mut robj,
    pub value: *mut robj,
    pub iter: *mut libc::c_void,
    pub mode: libc::c_int,
    pub u: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_20 {
    pub list: C2RustUnnamed_23,
    pub zset: C2RustUnnamed_22,
    pub stream: C2RustUnnamed_21,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub currentid: streamID,
    pub numfieldsleft: int64_t,
    pub signalready: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub type_0: uint32_t,
    pub rs: zrangespec,
    pub lrs: zlexrangespec,
    pub start: uint32_t,
    pub end: uint32_t,
    pub current: *mut libc::c_void,
    pub er: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zlexrangespec {
    pub min: sds,
    pub max: sds,
    pub minex: libc::c_int,
    pub maxex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zrangespec {
    pub min: libc::c_double,
    pub max: libc::c_double,
    pub minex: libc::c_int,
    pub maxex: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_23 {
    pub entry: listTypeEntry,
    pub index: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleScanCursor {
    pub cursor: libc::c_ulong,
    pub done: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanKeyCBData {
    pub key: *mut RedisModuleKey,
    pub user_data: *mut libc::c_void,
    pub fn_0: RedisModuleScanKeyCB,
}
pub type RedisModuleScanCB = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *mut robj,
        *mut RedisModuleKey,
        *mut libc::c_void,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ScanCBData {
    pub ctx: *mut RedisModuleCtx,
    pub user_data: *mut libc::c_void,
    pub fn_0: RedisModuleScanCB,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDict {
    pub rax: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleEventListener {
    pub module: *mut RedisModule,
    pub event: RedisModuleEvent,
    pub callback: RedisModuleEventCallback,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleServerInfoData {
    pub rax: *mut rax,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleForkInfo {
    pub done_handler: RedisModuleForkDoneHandler,
    pub done_handler_user_data: *mut libc::c_void,
}
pub type RedisModuleForkDoneHandler = Option::<
    unsafe extern "C" fn(libc::c_int, libc::c_int, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandFilterCtx {
    pub argv: *mut *mut robj,
    pub argv_len: libc::c_int,
    pub argc: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleCommandFilter {
    pub module: *mut RedisModule,
    pub callback: RedisModuleCommandFilterFunc,
    pub flags: libc::c_int,
}
pub type RedisModuleCommandFilterFunc = Option::<
    unsafe extern "C" fn(*mut RedisModuleCommandFilterCtx) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleSharedAPI {
    pub func: *mut libc::c_void,
    pub module: *mut RedisModule,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleDictIter {
    pub dict: *mut RedisModuleDict,
    pub ri: raxIterator,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleTimer {
    pub module: *mut RedisModule,
    pub callback: RedisModuleTimerProc,
    pub data: *mut libc::c_void,
    pub dbid: libc::c_int,
}
pub type RedisModuleTimerProc = Option::<
    unsafe extern "C" fn(*mut RedisModuleCtx, *mut libc::c_void) -> (),
>;
pub type RedisModuleCallReply = CallReply;
pub type RedisModuleClusterMessageReceiver = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        *const libc::c_char,
        uint8_t,
        *const libc::c_uchar,
        uint32_t,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct moduleClusterReceiver {
    pub module_id: uint64_t,
    pub callback: RedisModuleClusterMessageReceiver,
    pub module: *mut RedisModule,
    pub next: *mut moduleClusterReceiver,
}
pub type RedisModuleNotificationFunc = Option::<
    unsafe extern "C" fn(
        *mut RedisModuleCtx,
        libc::c_int,
        *const libc::c_char,
        *mut robj,
    ) -> libc::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RedisModuleKeyspaceSubscriber {
    pub module: *mut RedisModule,
    pub notify_callback: RedisModuleNotificationFunc,
    pub event_mask: libc::c_int,
    pub active: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_24 {
    pub u: uint64_t,
    pub i: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub u: uint64_t,
    pub i: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_26 {
    pub mem_usage2: moduleTypeMemUsageFunc2,
    pub free_effort2: moduleTypeFreeEffortFunc2,
    pub unlink2: moduleTypeUnlinkFunc2,
    pub copy2: moduleTypeCopyFunc2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct typemethods {
    pub version: uint64_t,
    pub rdb_load: moduleTypeLoadFunc,
    pub rdb_save: moduleTypeSaveFunc,
    pub aof_rewrite: moduleTypeRewriteFunc,
    pub mem_usage: moduleTypeMemUsageFunc,
    pub digest: moduleTypeDigestFunc,
    pub free: moduleTypeFreeFunc,
    pub v2: C2RustUnnamed_28,
    pub v3: C2RustUnnamed_27,
    pub v4: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_27 {
    pub free_effort: moduleTypeFreeEffortFunc,
    pub unlink: moduleTypeUnlinkFunc,
    pub copy: moduleTypeCopyFunc,
    pub defrag: moduleTypeDefragFunc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_28 {
    pub aux_load: moduleTypeAuxLoadFunc,
    pub aux_save: moduleTypeAuxSaveFunc,
    pub aux_save_triggers: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_29 {
    pub id: uint64_t,
    pub mt: *mut moduleType,
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
#[inline]
unsafe extern "C" fn sdsavail(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return 0 as libc::c_int as size_t,
        1 => {
            let mut sh: *mut sdshdr8 = s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr8;
            return ((*sh).alloc as libc::c_int - (*sh).len as libc::c_int) as size_t;
        }
        2 => {
            let mut sh_0: *mut sdshdr16 = s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr16;
            return ((*sh_0).alloc as libc::c_int - (*sh_0).len as libc::c_int) as size_t;
        }
        3 => {
            let mut sh_1: *mut sdshdr32 = s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr32;
            return ((*sh_1).alloc).wrapping_sub((*sh_1).len) as size_t;
        }
        4 => {
            let mut sh_2: *mut sdshdr64 = s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut libc::c_void as *mut sdshdr64;
            return ((*sh_2).alloc).wrapping_sub((*sh_2).len);
        }
        _ => {}
    }
    return 0 as libc::c_int as size_t;
}
#[inline]
unsafe extern "C" fn connGetType(mut conn: *mut connection) -> libc::c_int {
    return ((*(*conn).type_0).get_type).expect("non-null function pointer")(conn);
}
#[inline]
unsafe extern "C" fn elapsedStart(mut start_time: *mut monotime) {
    *start_time = getMonotonicUs.expect("non-null function pointer")();
}
#[inline]
unsafe extern "C" fn elapsedUs(mut start_time: monotime) -> uint64_t {
    return (getMonotonicUs.expect("non-null function pointer")())
        .wrapping_sub(start_time);
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(0 as libc::c_int, __path, __statbuf);
}
#[no_mangle]
pub static mut modules: *mut dict = 0 as *const dict as *mut dict;
static mut moduleUnblockedClientsMutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut moduleUnblockedClients: *mut list = 0 as *const list as *mut list;
static mut moduleTempClients: *mut *mut client = 0 as *const *mut client
    as *mut *mut client;
static mut moduleTempClientCap: size_t = 0 as libc::c_int as size_t;
static mut moduleTempClientCount: size_t = 0 as libc::c_int as size_t;
static mut moduleTempClientMinCount: size_t = 0 as libc::c_int as size_t;
static mut moduleGIL: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
static mut moduleKeyspaceSubscribers: *mut list = 0 as *const list as *mut list;
static mut moduleCommandFilters: *mut list = 0 as *const list as *mut list;
static mut moduleForkInfo: RedisModuleForkInfo = {
    let mut init = RedisModuleForkInfo {
        done_handler: None,
        done_handler_user_data: 0 as *const libc::c_void as *mut libc::c_void,
    };
    init
};
#[no_mangle]
pub static mut RedisModule_EventListeners: *mut list = 0 as *const list as *mut list;
#[no_mangle]
pub unsafe extern "C" fn RM_Alloc(mut bytes: size_t) -> *mut libc::c_void {
    return zmalloc(bytes);
}
#[no_mangle]
pub unsafe extern "C" fn RM_TryAlloc(mut bytes: size_t) -> *mut libc::c_void {
    return ztrymalloc(bytes);
}
#[no_mangle]
pub unsafe extern "C" fn RM_Calloc(
    mut nmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    return zcalloc(nmemb.wrapping_mul(size));
}
#[no_mangle]
pub unsafe extern "C" fn RM_Realloc(
    mut ptr: *mut libc::c_void,
    mut bytes: size_t,
) -> *mut libc::c_void {
    return zrealloc(ptr, bytes);
}
#[no_mangle]
pub unsafe extern "C" fn RM_Free(mut ptr: *mut libc::c_void) {
    zfree(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn RM_Strdup(mut str: *const libc::c_char) -> *mut libc::c_char {
    return zstrdup(str);
}
#[no_mangle]
pub unsafe extern "C" fn poolAllocRelease(mut ctx: *mut RedisModuleCtx) {
    let mut head: *mut RedisModulePoolAllocBlock = (*ctx).pa_head;
    let mut next: *mut RedisModulePoolAllocBlock = 0 as *mut RedisModulePoolAllocBlock;
    while !head.is_null() {
        next = (*head).next;
        zfree(head as *mut libc::c_void);
        head = next;
    }
    (*ctx).pa_head = 0 as *mut RedisModulePoolAllocBlock;
}
#[no_mangle]
pub unsafe extern "C" fn RM_PoolAlloc(
    mut ctx: *mut RedisModuleCtx,
    mut bytes: size_t,
) -> *mut libc::c_void {
    if bytes == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut libc::c_void;
    }
    let mut b: *mut RedisModulePoolAllocBlock = (*ctx).pa_head;
    let mut left: size_t = (if !b.is_null() {
        ((*b).size).wrapping_sub((*b).used)
    } else {
        0 as libc::c_int as libc::c_uint
    }) as size_t;
    if left >= bytes {
        let mut alignment: size_t = core::mem::size_of::<*mut libc::c_void>()
            as libc::c_ulong;
        while bytes < alignment
            && alignment.wrapping_div(2 as libc::c_int as libc::c_ulong) >= bytes
        {
            alignment = (alignment as libc::c_ulong)
                .wrapping_div(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
        }
        if ((*b).used as libc::c_ulong).wrapping_rem(alignment) != 0 {
            (*b)
                .used = ((*b).used as libc::c_ulong)
                .wrapping_add(
                    alignment
                        .wrapping_sub(
                            ((*b).used as libc::c_ulong).wrapping_rem(alignment),
                        ),
                ) as uint32_t as uint32_t;
        }
        left = (if (*b).used > (*b).size {
            0 as libc::c_int as libc::c_uint
        } else {
            ((*b).size).wrapping_sub((*b).used)
        }) as size_t;
    }
    if left < bytes {
        let mut blocksize: size_t = (1024 as libc::c_int * 8 as libc::c_int) as size_t;
        if blocksize < bytes {
            blocksize = bytes;
        }
        b = zmalloc(
            (core::mem::size_of::<RedisModulePoolAllocBlock>() as libc::c_ulong)
                .wrapping_add(blocksize),
        ) as *mut RedisModulePoolAllocBlock;
        (*b).size = blocksize as uint32_t;
        (*b).used = 0 as libc::c_int as uint32_t;
        (*b).next = (*ctx).pa_head;
        (*ctx).pa_head = b;
    }
    let mut retval: *mut libc::c_char = ((*b).memory)
        .as_mut_ptr()
        .offset((*b).used as isize);
    (*b).used = ((*b).used as libc::c_ulong).wrapping_add(bytes) as uint32_t as uint32_t;
    return retval as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn moduleAllocTempClient(mut user: *mut user) -> *mut client {
    let mut c: *mut client = 0 as *mut client;
    if moduleTempClientCount > 0 as libc::c_int as libc::c_ulong {
        moduleTempClientCount = moduleTempClientCount.wrapping_sub(1);
        c = *moduleTempClients.offset(moduleTempClientCount as isize);
        if moduleTempClientCount < moduleTempClientMinCount {
            moduleTempClientMinCount = moduleTempClientCount;
        }
    } else {
        c = createClient(0 as *mut connection);
        (*c).flags |= ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_ulong;
    }
    (*c).user = user;
    return c;
}
#[no_mangle]
pub unsafe extern "C" fn moduleReleaseTempClient(mut c: *mut client) {
    if moduleTempClientCount == moduleTempClientCap {
        moduleTempClientCap = if moduleTempClientCap != 0 {
            moduleTempClientCap.wrapping_mul(2 as libc::c_int as libc::c_ulong)
        } else {
            32 as libc::c_int as libc::c_ulong
        };
        moduleTempClients = zrealloc(
            moduleTempClients as *mut libc::c_void,
            (core::mem::size_of::<*mut client>() as libc::c_ulong)
                .wrapping_mul(moduleTempClientCap),
        ) as *mut *mut client;
    }
    clearClientConnectionState(c);
    listEmpty((*c).reply);
    (*c).reply_bytes = 0 as libc::c_int as libc::c_ulonglong;
    resetClient(c);
    (*c).bufpos = 0 as libc::c_int;
    (*c).flags = ((1 as libc::c_int) << 27 as libc::c_int) as uint64_t;
    (*c).user = 0 as *mut user;
    (*c).realcmd = 0 as *mut redisCommand;
    (*c).lastcmd = (*c).realcmd;
    (*c).cmd = (*c).lastcmd;
    let fresh0 = moduleTempClientCount;
    moduleTempClientCount = moduleTempClientCount.wrapping_add(1);
    let ref mut fresh1 = *moduleTempClients.offset(fresh0 as isize);
    *fresh1 = c;
}
#[no_mangle]
pub unsafe extern "C" fn moduleCreateEmptyKey(
    mut key: *mut RedisModuleKey,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut obj: *mut robj = 0 as *mut robj;
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || !((*key).value).is_null()
    {
        return 1 as libc::c_int;
    }
    match type_0 {
        2 => {
            obj = createQuicklistObject();
            quicklistSetOptions(
                (*obj).ptr as *mut quicklist,
                server.list_max_listpack_size,
                server.list_compress_depth,
            );
        }
        5 => {
            obj = createZsetListpackObject();
        }
        3 => {
            obj = createHashObject();
        }
        7 => {
            obj = createStreamObject();
        }
        _ => return 1 as libc::c_int,
    }
    dbAdd((*key).db, (*key).key, obj);
    (*key).value = obj;
    moduleInitKeyTypeSpecific(key);
    return 0 as libc::c_int;
}
unsafe extern "C" fn moduleFreeKeyIterator(mut key: *mut RedisModuleKey) {
    if !((*key).iter).is_null() {} else {
        _serverAssert(
            b"key->iter != NULL\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            650 as libc::c_int,
        );
        unreachable!();
    };
    match (*(*key).value).type_0() as libc::c_int {
        1 => {
            listTypeReleaseIterator((*key).iter as *mut listTypeIterator);
        }
        6 => {
            streamIteratorStop((*key).iter as *mut streamIterator);
            zfree((*key).iter);
        }
        _ => {
            if 0 as libc::c_int != 0 {} else {
                _serverAssert(
                    b"0\0" as *const u8 as *const libc::c_char,
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    657 as libc::c_int,
                );
                unreachable!();
            };
        }
    }
    (*key).iter = 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn moduleDelKeyIfEmpty(
    mut key: *mut RedisModuleKey,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || ((*key).value).is_null()
    {
        return 0 as libc::c_int;
    }
    let mut isempty: libc::c_int = 0;
    let mut o: *mut robj = (*key).value;
    match (*o).type_0() as libc::c_int {
        1 => {
            isempty = (listTypeLength(o) == 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        2 => {
            isempty = (setTypeSize(o) == 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        3 => {
            isempty = (zsetLength(o) == 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        4 => {
            isempty = (hashTypeLength(o) == 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        6 => {
            isempty = (streamLength(o) == 0 as libc::c_int as libc::c_ulong)
                as libc::c_int;
        }
        _ => {
            isempty = 0 as libc::c_int;
        }
    }
    if isempty != 0 {
        if !((*key).iter).is_null() {
            moduleFreeKeyIterator(key);
        }
        dbDelete((*key).db, (*key).key);
        (*key).value = 0 as *mut robj;
        return 1 as libc::c_int;
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetApi(
    mut funcname: *const libc::c_char,
    mut targetPtrPtr: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut he: *mut dictEntry = dictFind(
        server.moduleapi,
        funcname as *const libc::c_void,
    );
    if he.is_null() {
        return 1 as libc::c_int;
    }
    *targetPtrPtr = (*he).v.val;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleFreeContext(mut ctx: *mut RedisModuleCtx) {
    if (*ctx).flags & (1 as libc::c_int) << 4 as libc::c_int == 0 {
        server.module_ctx_nesting -= 1;
        if server.module_ctx_nesting == 0 as libc::c_int {
            if server.core_propagates == 0 {
                propagatePendingCommands();
            }
            if server.busy_module_yield_flags != 0 {
                blockingOperationEnds();
                server.busy_module_yield_flags = 0 as libc::c_int;
                if !(server.current_client).is_null() {
                    unprotectClient(server.current_client);
                }
                unblockPostponedClients();
            }
        }
    }
    autoMemoryCollect(ctx);
    poolAllocRelease(ctx);
    if !((*ctx).postponed_arrays).is_null() {
        zfree((*ctx).postponed_arrays as *mut libc::c_void);
        (*ctx).postponed_arrays_count = 0 as libc::c_int;
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"API misuse detected in module %s: RedisModule_ReplyWith*(REDISMODULE_POSTPONED_LEN) not matched by the same number of RedisModule_SetReply*Len() calls.\0"
                    as *const u8 as *const libc::c_char,
                (*(*ctx).module).name,
            );
        }
    }
    if (*ctx).flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        moduleReleaseTempClient((*ctx).client);
    } else if (*ctx).flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        freeClient((*ctx).client);
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleCreateContext(
    mut out_ctx: *mut RedisModuleCtx,
    mut module: *mut RedisModule,
    mut ctx_flags: libc::c_int,
) {
    memset(
        out_ctx as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<RedisModuleCtx>() as libc::c_ulong,
    );
    (*out_ctx)
        .getapifuncptr = core::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *const libc::c_char,
                *mut *mut libc::c_void,
            ) -> libc::c_int,
        >,
        libc::c_ulong,
    >(
        Some(
            RM_GetApi
                as unsafe extern "C" fn(
                    *const libc::c_char,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
        ),
    ) as *mut libc::c_void;
    (*out_ctx).module = module;
    (*out_ctx).flags = ctx_flags;
    if ctx_flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        (*out_ctx).client = moduleAllocTempClient(0 as *mut user);
    } else if ctx_flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
        (*out_ctx).client = createClient(0 as *mut connection);
    }
    if server.loading != 0 {
        (*out_ctx)
            .next_yield_time = (getMonotonicUs.expect("non-null function pointer")())
            .wrapping_add((1000000 as libc::c_int / server.hz) as libc::c_ulong)
            as libc::c_longlong;
    } else {
        (*out_ctx)
            .next_yield_time = (getMonotonicUs.expect("non-null function pointer")()
            as libc::c_ulonglong)
            .wrapping_add(
                (server.busy_reply_threshold * 1000 as libc::c_int as libc::c_longlong)
                    as libc::c_ulonglong,
            ) as libc::c_longlong;
    }
    if ctx_flags & (1 as libc::c_int) << 4 as libc::c_int == 0 {
        server.module_ctx_nesting += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RedisModuleCommandDispatcher(mut c: *mut client) {
    let mut cp: *mut RedisModuleCommand = (*(*c).cmd).module_cmd;
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(&mut ctx, (*cp).module, 0 as libc::c_int);
    ctx.client = c;
    ((*cp).func)
        .expect(
            "non-null function pointer",
        )(&mut ctx, (*c).argv as *mut *mut libc::c_void, (*c).argc);
    moduleFreeContext(&mut ctx);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*c).argc {
        if (**((*c).argv).offset(i as isize)).refcount > 1 as libc::c_int {
            trimStringObjectIfNeeded(*((*c).argv).offset(i as isize));
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetCommandKeysViaAPI(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(&mut ctx, (*cp).module, (1 as libc::c_int) << 1 as libc::c_int);
    getKeysPrepareResult(result, 256 as libc::c_int);
    ctx.keys_result = result;
    ((*cp).func)
        .expect(
            "non-null function pointer",
        )(&mut ctx, argv as *mut *mut libc::c_void, argc);
    moduleFreeContext(&mut ctx);
    return (*result).numkeys;
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetCommandChannelsViaAPI(
    mut cmd: *mut redisCommand,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut result: *mut getKeysResult,
) -> libc::c_int {
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(&mut ctx, (*cp).module, (1 as libc::c_int) << 8 as libc::c_int);
    getKeysPrepareResult(result, 256 as libc::c_int);
    ctx.keys_result = result;
    ((*cp).func)
        .expect(
            "non-null function pointer",
        )(&mut ctx, argv as *mut *mut libc::c_void, argc);
    moduleFreeContext(&mut ctx);
    return (*result).numkeys;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsKeysPositionRequest(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    return ((*ctx).flags & (1 as libc::c_int) << 1 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_KeyAtPosWithFlags(
    mut ctx: *mut RedisModuleCtx,
    mut pos: libc::c_int,
    mut flags: libc::c_int,
) {
    if (*ctx).flags & (1 as libc::c_int) << 1 as libc::c_int == 0
        || ((*ctx).keys_result).is_null()
    {
        return;
    }
    if pos <= 0 as libc::c_int {
        return;
    }
    let mut res: *mut getKeysResult = (*ctx).keys_result;
    if (*res).numkeys == (*res).size {
        let mut newsize: libc::c_int = (*res).size
            + (if (*res).size > 8192 as libc::c_int {
                8192 as libc::c_int
            } else {
                (*res).size
            });
        getKeysPrepareResult(res, newsize);
    }
    (*((*res).keys).offset((*res).numkeys as isize)).pos = pos;
    (*((*res).keys).offset((*res).numkeys as isize))
        .flags = moduleConvertKeySpecsFlags(flags as int64_t, 1 as libc::c_int)
        as libc::c_int;
    (*res).numkeys += 1;
}
#[no_mangle]
pub unsafe extern "C" fn RM_KeyAtPos(
    mut ctx: *mut RedisModuleCtx,
    mut pos: libc::c_int,
) {
    let mut flags: libc::c_int = moduleConvertKeySpecsFlags(
        ((1 as libc::c_ulonglong) << 1 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int
            | (1 as libc::c_ulonglong) << 5 as libc::c_int) as int64_t,
        0 as libc::c_int,
    ) as libc::c_int;
    RM_KeyAtPosWithFlags(ctx, pos, flags);
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsChannelsPositionRequest(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    return ((*ctx).flags & (1 as libc::c_int) << 8 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ChannelAtPosWithFlags(
    mut ctx: *mut RedisModuleCtx,
    mut pos: libc::c_int,
    mut flags: libc::c_int,
) {
    if (*ctx).flags & (1 as libc::c_int) << 8 as libc::c_int == 0
        || ((*ctx).keys_result).is_null()
    {
        return;
    }
    if pos <= 0 as libc::c_int {
        return;
    }
    let mut res: *mut getKeysResult = (*ctx).keys_result;
    if (*res).numkeys == (*res).size {
        let mut newsize: libc::c_int = (*res).size
            + (if (*res).size > 8192 as libc::c_int {
                8192 as libc::c_int
            } else {
                (*res).size
            });
        getKeysPrepareResult(res, newsize);
    }
    let mut new_flags: libc::c_int = 0 as libc::c_int;
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 2 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 12 as libc::c_int) as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 13 as libc::c_int) as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 1 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 14 as libc::c_int) as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 11 as libc::c_int) as libc::c_int;
    }
    (*((*res).keys).offset((*res).numkeys as isize)).pos = pos;
    (*((*res).keys).offset((*res).numkeys as isize)).flags = new_flags;
    (*res).numkeys += 1;
}
#[no_mangle]
pub unsafe extern "C" fn commandFlagsFromString(mut s: *mut libc::c_char) -> int64_t {
    let mut count: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut flags: int64_t = 0 as libc::c_int as int64_t;
    let mut tokens: *mut sds = sdssplitlen(
        s,
        strlen(s) as ssize_t,
        b" \0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut count,
    );
    j = 0 as libc::c_int;
    while j < count {
        let mut t: *mut libc::c_char = *tokens.offset(j as isize);
        if strcasecmp(t, b"write\0" as *const u8 as *const libc::c_char) == 0 {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 0 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"readonly\0" as *const u8 as *const libc::c_char) == 0 {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 1 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"admin\0" as *const u8 as *const libc::c_char) == 0 {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 4 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"deny-oom\0" as *const u8 as *const libc::c_char) == 0 {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 2 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"deny-script\0" as *const u8 as *const libc::c_char)
            == 0
        {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 6 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"allow-loading\0" as *const u8 as *const libc::c_char)
            == 0
        {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 9 as libc::c_int) as int64_t;
        } else if strcasecmp(t, b"pubsub\0" as *const u8 as *const libc::c_char) == 0 {
            flags = (flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 5 as libc::c_int) as int64_t;
        } else if !(strcasecmp(t, b"random\0" as *const u8 as *const libc::c_char) == 0)
        {
            if strcasecmp(t, b"blocking\0" as *const u8 as *const libc::c_char) == 0 {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 8 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"allow-stale\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 10 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"no-monitor\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 11 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"no-slowlog\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 12 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"fast\0" as *const u8 as *const libc::c_char) == 0 {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 14 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"no-auth\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 15 as libc::c_int) as int64_t;
            } else if strcasecmp(
                t,
                b"may-replicate\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 16 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"getkeys-api\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 21 as libc::c_int) as int64_t;
            } else if strcasecmp(
                t,
                b"getchannels-api\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 27 as libc::c_int) as int64_t;
            } else if strcasecmp(t, b"no-cluster\0" as *const u8 as *const libc::c_char)
                == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 22 as libc::c_int) as int64_t;
            } else if strcasecmp(
                t,
                b"no-mandatory-keys\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 19 as libc::c_int) as int64_t;
            } else {
                if !(strcasecmp(t, b"allow-busy\0" as *const u8 as *const libc::c_char)
                    == 0)
                {
                    break;
                }
                flags = (flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 26 as libc::c_int) as int64_t;
            }
        }
        j += 1;
    }
    sdsfreesplitres(tokens, count);
    if j != count {
        return -(1 as libc::c_int) as int64_t;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateCommand(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut cmdfunc: RedisModuleCmdFunc,
    mut strflags: *const libc::c_char,
    mut firstkey: libc::c_int,
    mut lastkey: libc::c_int,
    mut keystep: libc::c_int,
) -> libc::c_int {
    let mut flags: int64_t = if !strflags.is_null() {
        commandFlagsFromString(strflags as *mut libc::c_char)
    } else {
        0 as libc::c_int as libc::c_long
    };
    if flags == -(1 as libc::c_int) as libc::c_long {
        return 1 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 22 as libc::c_int != 0
        && server.cluster_enabled != 0
    {
        return 1 as libc::c_int;
    }
    if !(lookupCommandByCString(name)).is_null() {
        return 1 as libc::c_int;
    }
    let mut declared_name: sds = sdsnew(name);
    let mut cp: *mut RedisModuleCommand = moduleCreateCommandProxy(
        (*ctx).module,
        declared_name,
        sdsdup(declared_name),
        cmdfunc,
        flags,
        firstkey,
        lastkey,
        keystep,
    );
    (*(*cp).rediscmd)
        .arity = if cmdfunc.is_some() {
        -(1 as libc::c_int)
    } else {
        -(2 as libc::c_int)
    };
    if dictAdd(
        server.commands,
        sdsdup(declared_name) as *mut libc::c_void,
        (*cp).rediscmd as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        _serverAssert(
            b"dictAdd(server.commands, sdsdup(declared_name), cp->rediscmd) == DICT_OK\0"
                as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            1123 as libc::c_int,
        );
        unreachable!();
    };
    if dictAdd(
        server.orig_commands,
        sdsdup(declared_name) as *mut libc::c_void,
        (*cp).rediscmd as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        _serverAssert(
            b"dictAdd(server.orig_commands, sdsdup(declared_name), cp->rediscmd) == DICT_OK\0"
                as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            1124 as libc::c_int,
        );
        unreachable!();
    };
    (*(*cp).rediscmd).id = ACLGetCommandID(declared_name) as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleCreateCommandProxy(
    mut module: *mut RedisModule,
    mut declared_name: sds,
    mut fullname: sds,
    mut cmdfunc: RedisModuleCmdFunc,
    mut flags: int64_t,
    mut firstkey: libc::c_int,
    mut lastkey: libc::c_int,
    mut keystep: libc::c_int,
) -> *mut RedisModuleCommand {
    let mut rediscmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut cp: *mut RedisModuleCommand = 0 as *mut RedisModuleCommand;
    cp = zcalloc(core::mem::size_of::<RedisModuleCommand>() as libc::c_ulong)
        as *mut RedisModuleCommand;
    (*cp).module = module;
    (*cp).func = cmdfunc;
    (*cp)
        .rediscmd = zcalloc(core::mem::size_of::<redisCommand>() as libc::c_ulong)
        as *mut redisCommand;
    (*(*cp).rediscmd).declared_name = declared_name as *const libc::c_char;
    (*(*cp).rediscmd).fullname = fullname;
    (*(*cp).rediscmd).group = COMMAND_GROUP_MODULE;
    (*(*cp).rediscmd)
        .proc_0 = Some(
        RedisModuleCommandDispatcher as unsafe extern "C" fn(*mut client) -> (),
    );
    (*(*cp).rediscmd)
        .flags = (flags as libc::c_ulonglong
        | (1 as libc::c_ulonglong) << 3 as libc::c_int) as uint64_t;
    (*(*cp).rediscmd).module_cmd = cp;
    (*(*cp).rediscmd).key_specs_max = 4 as libc::c_int;
    (*(*cp).rediscmd).key_specs = ((*(*cp).rediscmd).key_specs_static).as_mut_ptr();
    if firstkey != 0 as libc::c_int {
        (*(*cp).rediscmd).key_specs_num = 1 as libc::c_int;
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .flags = ((1 as libc::c_ulonglong) << 1 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int
            | (1 as libc::c_ulonglong) << 5 as libc::c_int) as uint64_t;
        if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 21 as libc::c_int
            != 0
        {
            let ref mut fresh2 = (*((*(*cp).rediscmd).key_specs)
                .offset(0 as libc::c_int as isize))
                .flags;
            *fresh2 = (*fresh2 as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 10 as libc::c_int) as uint64_t;
        }
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .begin_search_type = KSPEC_BS_INDEX;
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .bs
            .index
            .pos = firstkey;
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .find_keys_type = KSPEC_FK_RANGE;
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .fk
            .range
            .lastkey = if lastkey < 0 as libc::c_int {
            lastkey
        } else {
            lastkey - firstkey
        };
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .fk
            .range
            .keystep = keystep;
        (*((*(*cp).rediscmd).key_specs).offset(0 as libc::c_int as isize))
            .fk
            .range
            .limit = 0 as libc::c_int;
    } else {
        (*(*cp).rediscmd).key_specs_num = 0 as libc::c_int;
    }
    populateCommandLegacyRangeSpec((*cp).rediscmd);
    (*(*cp).rediscmd).microseconds = 0 as libc::c_int as libc::c_longlong;
    (*(*cp).rediscmd).calls = 0 as libc::c_int as libc::c_longlong;
    (*(*cp).rediscmd).rejected_calls = 0 as libc::c_int as libc::c_longlong;
    (*(*cp).rediscmd).failed_calls = 0 as libc::c_int as libc::c_longlong;
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetCommand(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
) -> *mut RedisModuleCommand {
    let mut cmd: *mut redisCommand = lookupCommandByCString(name);
    if cmd.is_null()
        || (*cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 3 as libc::c_int == 0
    {
        return 0 as *mut RedisModuleCommand;
    }
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    if (*cp).module != (*ctx).module {
        return 0 as *mut RedisModuleCommand;
    }
    return cp;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateSubcommand(
    mut parent: *mut RedisModuleCommand,
    mut name: *const libc::c_char,
    mut cmdfunc: RedisModuleCmdFunc,
    mut strflags: *const libc::c_char,
    mut firstkey: libc::c_int,
    mut lastkey: libc::c_int,
    mut keystep: libc::c_int,
) -> libc::c_int {
    let mut flags: int64_t = if !strflags.is_null() {
        commandFlagsFromString(strflags as *mut libc::c_char)
    } else {
        0 as libc::c_int as libc::c_long
    };
    if flags == -(1 as libc::c_int) as libc::c_long {
        return 1 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 22 as libc::c_int != 0
        && server.cluster_enabled != 0
    {
        return 1 as libc::c_int;
    }
    let mut parent_cmd: *mut redisCommand = (*parent).rediscmd;
    if !((*parent_cmd).parent).is_null() {
        return 1 as libc::c_int;
    }
    let mut parent_cp: *mut RedisModuleCommand = (*parent_cmd).module_cmd;
    if ((*parent_cp).func).is_some() {
        return 1 as libc::c_int;
    }
    let mut declared_name: sds = sdsnew(name);
    if !((*parent_cmd).subcommands_dict).is_null()
        && !(lookupSubcommand(parent_cmd, declared_name)).is_null()
    {
        sdsfree(declared_name);
        return 1 as libc::c_int;
    }
    let mut fullname: sds = catSubCommandFullname(
        (*parent_cmd).fullname as *const libc::c_char,
        name,
    );
    let mut cp: *mut RedisModuleCommand = moduleCreateCommandProxy(
        (*parent).module,
        declared_name,
        fullname,
        cmdfunc,
        flags,
        firstkey,
        lastkey,
        keystep,
    );
    (*(*cp).rediscmd).arity = -(2 as libc::c_int);
    commandAddSubcommand(parent_cmd, (*cp).rediscmd, name);
    return 0 as libc::c_int;
}
unsafe extern "C" fn moduleCmdHistoryEntryAt(
    mut version: *const RedisModuleCommandInfoVersion,
    mut entries: *mut RedisModuleCommandHistoryEntry,
    mut index: libc::c_int,
) -> *mut RedisModuleCommandHistoryEntry {
    let mut offset: off_t = (index as libc::c_ulong)
        .wrapping_mul((*version).sizeof_historyentry) as off_t;
    return (entries as *mut libc::c_char).offset(offset as isize)
        as *mut RedisModuleCommandHistoryEntry;
}
unsafe extern "C" fn moduleCmdKeySpecAt(
    mut version: *const RedisModuleCommandInfoVersion,
    mut keyspecs: *mut RedisModuleCommandKeySpec,
    mut index: libc::c_int,
) -> *mut RedisModuleCommandKeySpec {
    let mut offset: off_t = (index as libc::c_ulong)
        .wrapping_mul((*version).sizeof_keyspec) as off_t;
    return (keyspecs as *mut libc::c_char).offset(offset as isize)
        as *mut RedisModuleCommandKeySpec;
}
unsafe extern "C" fn moduleCmdArgAt(
    mut version: *const RedisModuleCommandInfoVersion,
    mut args: *const RedisModuleCommandArg,
    mut index: libc::c_int,
) -> *mut RedisModuleCommandArg {
    let mut offset: off_t = (index as libc::c_ulong).wrapping_mul((*version).sizeof_arg)
        as off_t;
    return (args as *mut libc::c_char).offset(offset as isize)
        as *mut RedisModuleCommandArg;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetCommandInfo(
    mut command: *mut RedisModuleCommand,
    mut info: *const RedisModuleCommandInfo,
) -> libc::c_int {
    if moduleValidateCommandInfo(info) == 0 {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut cmd: *mut redisCommand = (*command).rediscmd;
    if !((*cmd).summary).is_null() || !((*cmd).complexity).is_null()
        || !((*cmd).since).is_null() || !((*cmd).history).is_null()
        || !((*cmd).tips).is_null() || !((*cmd).args).is_null()
        || !((*cmd).key_specs_num == 0 as libc::c_int
            || (*cmd).key_specs_num == 1 as libc::c_int
                && (*cmd).key_specs == ((*cmd).key_specs_static).as_mut_ptr()
                && (*((*cmd).key_specs).offset(0 as libc::c_int as isize))
                    .begin_search_type as libc::c_uint
                    == KSPEC_BS_INDEX as libc::c_int as libc::c_uint
                && (*((*cmd).key_specs).offset(0 as libc::c_int as isize)).find_keys_type
                    as libc::c_uint == KSPEC_FK_RANGE as libc::c_int as libc::c_uint)
    {
        *__errno_location() = 17 as libc::c_int;
        return 1 as libc::c_int;
    }
    if !((*info).summary).is_null() {
        (*cmd).summary = zstrdup((*info).summary);
    }
    if !((*info).complexity).is_null() {
        (*cmd).complexity = zstrdup((*info).complexity);
    }
    if !((*info).since).is_null() {
        (*cmd).since = zstrdup((*info).since);
    }
    let mut version: *const RedisModuleCommandInfoVersion = (*info).version;
    if !((*info).history).is_null() {
        let mut count: size_t = 0 as libc::c_int as size_t;
        while !((*moduleCmdHistoryEntryAt(
            version,
            (*info).history,
            count as libc::c_int,
        ))
            .since)
            .is_null()
        {
            count = count.wrapping_add(1);
        }
        if count
            < (18446744073709551615 as libc::c_ulong)
                .wrapping_div(core::mem::size_of::<commandHistory>() as libc::c_ulong)
        {} else {
            _serverAssert(
                b"count < SIZE_MAX / sizeof(commandHistory)\0" as *const u8
                    as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                1608 as libc::c_int,
            );
            unreachable!();
        };
        (*cmd)
            .history = zmalloc(
            (core::mem::size_of::<commandHistory>() as libc::c_ulong)
                .wrapping_mul(count.wrapping_add(1 as libc::c_int as libc::c_ulong)),
        ) as *mut commandHistory;
        let mut j: size_t = 0 as libc::c_int as size_t;
        while j < count {
            let mut entry: *mut RedisModuleCommandHistoryEntry = moduleCmdHistoryEntryAt(
                version,
                (*info).history,
                j as libc::c_int,
            );
            let ref mut fresh3 = (*((*cmd).history).offset(j as isize)).since;
            *fresh3 = zstrdup((*entry).since);
            let ref mut fresh4 = (*((*cmd).history).offset(j as isize)).changes;
            *fresh4 = zstrdup((*entry).changes);
            j = j.wrapping_add(1);
        }
        let ref mut fresh5 = (*((*cmd).history).offset(count as isize)).since;
        *fresh5 = 0 as *const libc::c_char;
        let ref mut fresh6 = (*((*cmd).history).offset(count as isize)).changes;
        *fresh6 = 0 as *const libc::c_char;
        (*cmd).num_history = count as libc::c_int;
    }
    if !((*info).tips).is_null() {
        let mut count_0: libc::c_int = 0;
        let mut tokens: *mut sds = sdssplitlen(
            (*info).tips,
            strlen((*info).tips) as ssize_t,
            b" \0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
            &mut count_0,
        );
        if !tokens.is_null() {
            (*cmd)
                .tips = zmalloc(
                (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                    .wrapping_mul((count_0 + 1 as libc::c_int) as libc::c_ulong),
            ) as *mut *const libc::c_char;
            let mut j_0: libc::c_int = 0 as libc::c_int;
            while j_0 < count_0 {
                let ref mut fresh7 = *((*cmd).tips).offset(j_0 as isize);
                *fresh7 = zstrdup(*tokens.offset(j_0 as isize) as *const libc::c_char);
                j_0 += 1;
            }
            let ref mut fresh8 = *((*cmd).tips).offset(count_0 as isize);
            *fresh8 = 0 as *const libc::c_char;
            (*cmd).num_tips = count_0;
            sdsfreesplitres(tokens, count_0);
        }
    }
    if (*info).arity != 0 {
        (*cmd).arity = (*info).arity;
    }
    if !((*info).key_specs).is_null() {
        let mut count_1: size_t = 0 as libc::c_int as size_t;
        while (*moduleCmdKeySpecAt(version, (*info).key_specs, count_1 as libc::c_int))
            .begin_search_type as u64 != 0
        {
            count_1 = count_1.wrapping_add(1);
        }
        if count_1 < 2147483647 as libc::c_int as libc::c_ulong {} else {
            _serverAssert(
                b"count < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                1642 as libc::c_int,
            );
            unreachable!();
        };
        if count_1 <= 4 as libc::c_int as libc::c_ulong {
            (*cmd).key_specs_max = 4 as libc::c_int;
            (*cmd).key_specs = ((*cmd).key_specs_static).as_mut_ptr();
        } else {
            (*cmd).key_specs_max = count_1 as libc::c_int;
            (*cmd)
                .key_specs = zmalloc(
                (core::mem::size_of::<keySpec>() as libc::c_ulong)
                    .wrapping_mul(count_1),
            ) as *mut keySpec;
        }
        (*cmd).key_specs_num = count_1 as libc::c_int;
        let mut j_1: size_t = 0 as libc::c_int as size_t;
        while j_1 < count_1 {
            let mut spec: *mut RedisModuleCommandKeySpec = moduleCmdKeySpecAt(
                version,
                (*info).key_specs,
                j_1 as libc::c_int,
            );
            let ref mut fresh9 = (*((*cmd).key_specs).offset(j_1 as isize)).notes;
            *fresh9 = if !((*spec).notes).is_null() {
                zstrdup((*spec).notes)
            } else {
                0 as *mut libc::c_char
            };
            (*((*cmd).key_specs).offset(j_1 as isize))
                .flags = moduleConvertKeySpecsFlags(
                (*spec).flags as int64_t,
                1 as libc::c_int,
            ) as uint64_t;
            match (*spec).begin_search_type as libc::c_uint {
                1 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .begin_search_type = KSPEC_BS_UNKNOWN;
                }
                2 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .begin_search_type = KSPEC_BS_INDEX;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .bs
                        .index
                        .pos = (*spec).bs.index.pos;
                }
                3 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .begin_search_type = KSPEC_BS_KEYWORD;
                    let ref mut fresh10 = (*((*cmd).key_specs).offset(j_1 as isize))
                        .bs
                        .keyword
                        .keyword;
                    *fresh10 = zstrdup((*spec).bs.keyword.keyword);
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .bs
                        .keyword
                        .startfrom = (*spec).bs.keyword.startfrom;
                }
                _ => {
                    _serverPanic(
                        b"module.c\0" as *const u8 as *const libc::c_char,
                        1673 as libc::c_int,
                        b"Unknown begin_search_type\0" as *const u8
                            as *const libc::c_char,
                    );
                    unreachable!();
                }
            }
            match (*spec).find_keys_type as libc::c_uint {
                0 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .find_keys_type = KSPEC_FK_RANGE;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .lastkey = 0 as libc::c_int;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .keystep = 1 as libc::c_int;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .limit = 0 as libc::c_int;
                }
                1 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .find_keys_type = KSPEC_FK_UNKNOWN;
                }
                2 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .find_keys_type = KSPEC_FK_RANGE;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .lastkey = (*spec).fk.range.lastkey;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .keystep = (*spec).fk.range.keystep;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .range
                        .limit = (*spec).fk.range.limit;
                }
                3 => {
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .find_keys_type = KSPEC_FK_KEYNUM;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .keynum
                        .keynumidx = (*spec).fk.keynum.keynumidx;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .keynum
                        .firstkey = (*spec).fk.keynum.firstkey;
                    (*((*cmd).key_specs).offset(j_1 as isize))
                        .fk
                        .keynum
                        .keystep = (*spec).fk.keynum.keystep;
                }
                _ => {
                    _serverPanic(
                        b"module.c\0" as *const u8 as *const libc::c_char,
                        1701 as libc::c_int,
                        b"Unknown find_keys_type\0" as *const u8 as *const libc::c_char,
                    );
                    unreachable!();
                }
            }
            j_1 = j_1.wrapping_add(1);
        }
        populateCommandLegacyRangeSpec(cmd);
    }
    if !((*info).args).is_null() {
        (*cmd).args = moduleCopyCommandArgs((*info).args, version);
        (*cmd).num_args = populateArgsStructure((*cmd).args);
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn isPowerOfTwo(mut v: uint64_t) -> libc::c_int {
    return (v != 0 && v & v.wrapping_sub(1 as libc::c_int as libc::c_ulong) == 0)
        as libc::c_int;
}
unsafe extern "C" fn moduleValidateCommandInfo(
    mut info: *const RedisModuleCommandInfo,
) -> libc::c_int {
    let mut version: *const RedisModuleCommandInfoVersion = (*info).version;
    if version.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Invalid command info: version missing\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if !((*info).history).is_null() {
        let mut j: size_t = 0 as libc::c_int as size_t;
        while !((*moduleCmdHistoryEntryAt(version, (*info).history, j as libc::c_int))
            .since)
            .is_null()
        {
            if ((*moduleCmdHistoryEntryAt(version, (*info).history, j as libc::c_int))
                .changes)
                .is_null()
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: history[%zd].changes missing\0"
                            as *const u8 as *const libc::c_char,
                        j,
                    );
                }
                return 0 as libc::c_int;
            }
            j = j.wrapping_add(1);
        }
    }
    if !((*info).key_specs).is_null() {
        let mut j_0: size_t = 0 as libc::c_int as size_t;
        while (*moduleCmdKeySpecAt(version, (*info).key_specs, j_0 as libc::c_int))
            .begin_search_type as u64 != 0
        {
            let mut spec: *mut RedisModuleCommandKeySpec = moduleCmdKeySpecAt(
                version,
                (*info).key_specs,
                j_0 as libc::c_int,
            );
            if j_0 >= 2147483647 as libc::c_int as libc::c_ulong {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: Too many key specs\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 0 as libc::c_int;
            }
            let mut key_flags: uint64_t = ((1 as libc::c_ulonglong) << 0 as libc::c_int
                | (1 as libc::c_ulonglong) << 1 as libc::c_int
                | (1 as libc::c_ulonglong) << 2 as libc::c_int
                | (1 as libc::c_ulonglong) << 3 as libc::c_int) as uint64_t;
            let mut write_flags: uint64_t = ((1 as libc::c_ulonglong) << 6 as libc::c_int
                | (1 as libc::c_ulonglong) << 7 as libc::c_int
                | (1 as libc::c_ulonglong) << 5 as libc::c_int) as uint64_t;
            if isPowerOfTwo((*spec).flags & key_flags) == 0 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: key_specs[%zd].flags: Exactly one of the flags RO, RW, OW, RM required\0"
                            as *const u8 as *const libc::c_char,
                        j_0,
                    );
                }
                return 0 as libc::c_int;
            }
            if (*spec).flags & write_flags != 0 as libc::c_int as libc::c_ulong
                && isPowerOfTwo((*spec).flags & write_flags) == 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: key_specs[%zd].flags: INSERT, DELETE and UPDATE are mutually exclusive\0"
                            as *const u8 as *const libc::c_char,
                        j_0,
                    );
                }
                return 0 as libc::c_int;
            }
            match (*spec).begin_search_type as libc::c_uint {
                1 | 2 => {}
                3 => {
                    if ((*spec).bs.keyword.keyword).is_null() {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Invalid command info: key_specs[%zd].bs.keyword.keyword required when begin_search_type is KEYWORD\0"
                                    as *const u8 as *const libc::c_char,
                                j_0,
                            );
                        }
                        return 0 as libc::c_int;
                    }
                }
                _ => {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Invalid command info: key_specs[%zd].begin_search_type: Invalid value %d\0"
                                as *const u8 as *const libc::c_char,
                            j_0,
                            (*spec).begin_search_type as libc::c_uint,
                        );
                    }
                    return 0 as libc::c_int;
                }
            }
            match (*spec).find_keys_type as libc::c_uint {
                0 | 1 | 2 | 3 => {}
                _ => {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Invalid command info: key_specs[%zd].find_keys_type: Invalid value %d\0"
                                as *const u8 as *const libc::c_char,
                            j_0,
                            (*spec).find_keys_type as libc::c_uint,
                        );
                    }
                    return 0 as libc::c_int;
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
    }
    return moduleValidateCommandArgs((*info).args, version);
}
unsafe extern "C" fn moduleConvertKeySpecsFlags(
    mut flags: int64_t,
    mut from_api: libc::c_int,
) -> int64_t {
    let mut out: int64_t = 0 as libc::c_int as int64_t;
    let mut map: [[int64_t; 2]; 12] = [
        [
            ((1 as libc::c_ulonglong) << 0 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 0 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 1 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 1 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 2 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 2 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 3 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 3 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 4 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 4 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 6 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 6 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 5 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 5 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 7 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 7 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 8 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 8 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 9 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 9 as libc::c_int) as int64_t,
        ],
        [
            ((1 as libc::c_ulonglong) << 10 as libc::c_int) as int64_t,
            ((1 as libc::c_ulonglong) << 10 as libc::c_int) as int64_t,
        ],
        [0 as libc::c_int as int64_t, 0 as libc::c_int as int64_t],
    ];
    let mut from_idx: libc::c_int = if from_api != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
    let mut to_idx: libc::c_int = (from_idx == 0) as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while map[i as usize][0 as libc::c_int as usize] != 0 {
        if flags & map[i as usize][from_idx as usize] != 0 {
            out |= map[i as usize][to_idx as usize];
        }
        i += 1;
    }
    return out;
}
unsafe extern "C" fn moduleValidateCommandArgs(
    mut args: *mut RedisModuleCommandArg,
    mut version: *const RedisModuleCommandInfoVersion,
) -> libc::c_int {
    if args.is_null() {
        return 1 as libc::c_int;
    }
    let mut j: size_t = 0 as libc::c_int as size_t;
    while !((*moduleCmdArgAt(version, args, j as libc::c_int)).name).is_null() {
        let mut arg: *mut RedisModuleCommandArg = moduleCmdArgAt(
            version,
            args,
            j as libc::c_int,
        );
        let mut arg_type_error: libc::c_int = 0 as libc::c_int;
        moduleConvertArgType((*arg).type_0, &mut arg_type_error);
        if arg_type_error != 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid command info: Argument \"%s\": Undefined type %d\0"
                        as *const u8 as *const libc::c_char,
                    (*arg).name,
                    (*arg).type_0 as libc::c_uint,
                );
            }
            return 0 as libc::c_int;
        }
        if (*arg).type_0 as libc::c_uint
            == REDISMODULE_ARG_TYPE_PURE_TOKEN as libc::c_int as libc::c_uint
            && ((*arg).token).is_null()
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid command info: Argument \"%s\": token required when type is PURE_TOKEN\0"
                        as *const u8 as *const libc::c_char,
                    (*args.offset(j as isize)).name,
                );
            }
            return 0 as libc::c_int;
        }
        if (*arg).type_0 as libc::c_uint
            == REDISMODULE_ARG_TYPE_KEY as libc::c_int as libc::c_uint
        {
            if (*arg).key_spec_index < 0 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: Argument \"%s\": key_spec_index required when type is KEY\0"
                            as *const u8 as *const libc::c_char,
                        (*arg).name,
                    );
                }
                return 0 as libc::c_int;
            }
        } else if (*arg).key_spec_index != -(1 as libc::c_int)
            && (*arg).key_spec_index != 0 as libc::c_int
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid command info: Argument \"%s\": key_spec_index specified but type isn't KEY\0"
                        as *const u8 as *const libc::c_char,
                    (*arg).name,
                );
            }
            return 0 as libc::c_int;
        }
        if (*arg).flags & !(((1 as libc::c_int) << 3 as libc::c_int) - 1 as libc::c_int)
            != 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid command info: Argument \"%s\": Invalid flags\0"
                        as *const u8 as *const libc::c_char,
                    (*arg).name,
                );
            }
            return 0 as libc::c_int;
        }
        if (*arg).type_0 as libc::c_uint
            == REDISMODULE_ARG_TYPE_ONEOF as libc::c_int as libc::c_uint
            || (*arg).type_0 as libc::c_uint
                == REDISMODULE_ARG_TYPE_BLOCK as libc::c_int as libc::c_uint
        {
            if ((*arg).subargs).is_null() {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Invalid command info: Argument \"%s\": subargs required when type is ONEOF or BLOCK\0"
                            as *const u8 as *const libc::c_char,
                        (*arg).name,
                    );
                }
                return 0 as libc::c_int;
            }
            if moduleValidateCommandArgs((*arg).subargs, version) == 0 {
                return 0 as libc::c_int;
            }
        } else if !((*arg).subargs).is_null() {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid command info: Argument \"%s\": subargs specified but type isn't ONEOF nor BLOCK\0"
                        as *const u8 as *const libc::c_char,
                    (*arg).name,
                );
            }
            return 0 as libc::c_int;
        }
        j = j.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn moduleCopyCommandArgs(
    mut args: *mut RedisModuleCommandArg,
    mut version: *const RedisModuleCommandInfoVersion,
) -> *mut redisCommandArg {
    let mut count: size_t = 0 as libc::c_int as size_t;
    while !((*moduleCmdArgAt(version, args, count as libc::c_int)).name).is_null() {
        count = count.wrapping_add(1);
    }
    if count
        < (18446744073709551615 as libc::c_ulong)
            .wrapping_div(core::mem::size_of::<redisCommandArg>() as libc::c_ulong)
    {} else {
        _serverAssert(
            b"count < SIZE_MAX / sizeof(struct redisCommandArg)\0" as *const u8
                as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            1925 as libc::c_int,
        );
        unreachable!();
    };
    let mut realargs: *mut redisCommandArg = zcalloc(
        count
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<redisCommandArg>() as libc::c_ulong),
    ) as *mut redisCommandArg;
    let mut j: size_t = 0 as libc::c_int as size_t;
    while j < count {
        let mut arg: *mut RedisModuleCommandArg = moduleCmdArgAt(
            version,
            args,
            j as libc::c_int,
        );
        let ref mut fresh11 = (*realargs.offset(j as isize)).name;
        *fresh11 = zstrdup((*arg).name);
        (*realargs.offset(j as isize))
            .type_0 = moduleConvertArgType((*arg).type_0, 0 as *mut libc::c_int);
        if (*arg).type_0 as libc::c_uint
            == REDISMODULE_ARG_TYPE_KEY as libc::c_int as libc::c_uint
        {
            (*realargs.offset(j as isize)).key_spec_index = (*arg).key_spec_index;
        } else {
            (*realargs.offset(j as isize)).key_spec_index = -(1 as libc::c_int);
        }
        if !((*arg).token).is_null() {
            let ref mut fresh12 = (*realargs.offset(j as isize)).token;
            *fresh12 = zstrdup((*arg).token);
        }
        if !((*arg).summary).is_null() {
            let ref mut fresh13 = (*realargs.offset(j as isize)).summary;
            *fresh13 = zstrdup((*arg).summary);
        }
        if !((*arg).since).is_null() {
            let ref mut fresh14 = (*realargs.offset(j as isize)).since;
            *fresh14 = zstrdup((*arg).since);
        }
        if !((*arg).deprecated_since).is_null() {
            let ref mut fresh15 = (*realargs.offset(j as isize)).deprecated_since;
            *fresh15 = zstrdup((*arg).deprecated_since);
        }
        (*realargs.offset(j as isize)).flags = moduleConvertArgFlags((*arg).flags);
        if !((*arg).subargs).is_null() {
            let ref mut fresh16 = (*realargs.offset(j as isize)).subargs;
            *fresh16 = moduleCopyCommandArgs((*arg).subargs, version);
        }
        j = j.wrapping_add(1);
    }
    return realargs;
}
unsafe extern "C" fn moduleConvertArgType(
    mut type_0: RedisModuleCommandArgType,
    mut error: *mut libc::c_int,
) -> redisCommandArgType {
    if !error.is_null() {
        *error = 0 as libc::c_int;
    }
    match type_0 as libc::c_uint {
        0 => return ARG_TYPE_STRING,
        1 => return ARG_TYPE_INTEGER,
        2 => return ARG_TYPE_DOUBLE,
        3 => return ARG_TYPE_KEY,
        4 => return ARG_TYPE_PATTERN,
        5 => return ARG_TYPE_UNIX_TIME,
        6 => return ARG_TYPE_PURE_TOKEN,
        7 => return ARG_TYPE_ONEOF,
        8 => return ARG_TYPE_BLOCK,
        _ => {
            if !error.is_null() {
                *error = 1 as libc::c_int;
            }
            return 4294967295 as redisCommandArgType;
        }
    };
}
unsafe extern "C" fn moduleConvertArgFlags(mut flags: libc::c_int) -> libc::c_int {
    let mut realflags: libc::c_int = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        realflags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        realflags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        realflags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    return realflags;
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetHandleByName(
    mut modulename: *mut libc::c_char,
) -> *mut libc::c_void {
    return dictFetchValue(modules, modulename as *const libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleIsModuleCommand(
    mut module_handle: *mut libc::c_void,
    mut cmd: *mut redisCommand,
) -> libc::c_int {
    if (*cmd).proc_0
        != Some(RedisModuleCommandDispatcher as unsafe extern "C" fn(*mut client) -> ())
    {
        return 0 as libc::c_int;
    }
    if module_handle.is_null() {
        return 0 as libc::c_int;
    }
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    return ((*cp).module == module_handle as *mut RedisModule) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleListConfigMatch(
    mut config: *mut libc::c_void,
    mut name: *mut libc::c_void,
) -> libc::c_int {
    return (strcasecmp(
        (*(config as *mut ModuleConfig)).name as *const libc::c_char,
        name as *mut libc::c_char,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleListFree(mut config: *mut libc::c_void) {
    let mut module_config: *mut ModuleConfig = config as *mut ModuleConfig;
    sdsfree((*module_config).name);
    zfree(config);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetModuleAttribs(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut ver: libc::c_int,
    mut apiver: libc::c_int,
) {
    let mut module: *mut RedisModule = 0 as *mut RedisModule;
    if !((*ctx).module).is_null() {
        return;
    }
    module = zmalloc(core::mem::size_of::<RedisModule>() as libc::c_ulong)
        as *mut RedisModule;
    (*module).name = sdsnew(name);
    (*module).ver = ver;
    (*module).apiver = apiver;
    (*module).types = listCreate();
    (*module).usedby = listCreate();
    (*module).using = listCreate();
    (*module).filters = listCreate();
    (*module).module_configs = listCreate();
    (*(*module).module_configs)
        .match_0 = Some(
        moduleListConfigMatch
            as unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> libc::c_int,
    );
    (*(*module).module_configs)
        .free = Some(moduleListFree as unsafe extern "C" fn(*mut libc::c_void) -> ());
    (*module).in_call = 0 as libc::c_int;
    (*module).configs_initialized = 0 as libc::c_int;
    (*module).in_hook = 0 as libc::c_int;
    (*module).options = 0 as libc::c_int;
    (*module).info_cb = None;
    (*module).defrag_cb = None;
    (*module).loadmod = 0 as *mut moduleLoadQueueEntry;
    (*ctx).module = module;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsModuleNameBusy(
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut modulename: sds = sdsnew(name);
    let mut de: *mut dictEntry = dictFind(modules, modulename as *const libc::c_void);
    sdsfree(modulename);
    return (de != 0 as *mut libc::c_void as *mut dictEntry) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_Milliseconds() -> libc::c_longlong {
    return mstime();
}
#[no_mangle]
pub unsafe extern "C" fn RM_MonotonicMicroseconds() -> uint64_t {
    return getMonotonicUs.expect("non-null function pointer")();
}
#[no_mangle]
pub unsafe extern "C" fn RM_BlockedClientMeasureTimeStart(
    mut bc: *mut RedisModuleBlockedClient,
) -> libc::c_int {
    elapsedStart(&mut (*bc).background_timer);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_BlockedClientMeasureTimeEnd(
    mut bc: *mut RedisModuleBlockedClient,
) -> libc::c_int {
    if (*bc).background_timer == 0 {
        return 1 as libc::c_int;
    }
    (*bc)
        .background_duration = ((*bc).background_duration as libc::c_ulong)
        .wrapping_add(elapsedUs((*bc).background_timer)) as uint64_t as uint64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_Yield(
    mut ctx: *mut RedisModuleCtx,
    mut flags: libc::c_int,
    mut busy_reply: *const libc::c_char,
) {
    static mut yield_nesting: libc::c_int = 0 as libc::c_int;
    if yield_nesting != 0 {
        return;
    }
    yield_nesting += 1;
    let mut now: libc::c_longlong = getMonotonicUs.expect("non-null function pointer")()
        as libc::c_longlong;
    if now >= (*ctx).next_yield_time {
        if server.loading != 0 {
            processEventsWhileBlocked();
        } else {
            let mut prev_busy_module_yield_reply: *const libc::c_char = server
                .busy_module_yield_reply;
            server.busy_module_yield_reply = busy_reply;
            if server.busy_module_yield_flags == 0 {
                server.busy_module_yield_flags = (1 as libc::c_int) << 0 as libc::c_int;
                blockingOperationStarts();
                if !(server.current_client).is_null() {
                    protectClient(server.current_client);
                }
            }
            if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                server.busy_module_yield_flags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
            processEventsWhileBlocked();
            server.busy_module_yield_reply = prev_busy_module_yield_reply;
            server.busy_module_yield_flags &= !((1 as libc::c_int) << 1 as libc::c_int);
        }
        (*ctx)
            .next_yield_time = now
            + (1000000 as libc::c_int / server.hz) as libc::c_longlong;
    }
    yield_nesting -= 1;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetModuleOptions(
    mut ctx: *mut RedisModuleCtx,
    mut options: libc::c_int,
) {
    (*(*ctx).module).options = options;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SignalModifiedKey(
    mut ctx: *mut RedisModuleCtx,
    mut keyname: *mut robj,
) -> libc::c_int {
    signalModifiedKey((*ctx).client, (*(*ctx).client).db, keyname);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_AutoMemory(mut ctx: *mut RedisModuleCtx) {
    (*ctx).flags |= (1 as libc::c_int) << 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn autoMemoryAdd(
    mut ctx: *mut RedisModuleCtx,
    mut type_0: libc::c_int,
    mut ptr: *mut libc::c_void,
) {
    if (*ctx).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return;
    }
    if (*ctx).amqueue_used == (*ctx).amqueue_len {
        (*ctx).amqueue_len *= 2 as libc::c_int;
        if (*ctx).amqueue_len < 16 as libc::c_int {
            (*ctx).amqueue_len = 16 as libc::c_int;
        }
        (*ctx)
            .amqueue = zrealloc(
            (*ctx).amqueue as *mut libc::c_void,
            (core::mem::size_of::<AutoMemEntry>() as libc::c_ulong)
                .wrapping_mul((*ctx).amqueue_len as libc::c_ulong),
        ) as *mut AutoMemEntry;
    }
    (*((*ctx).amqueue).offset((*ctx).amqueue_used as isize)).type_0 = type_0;
    let ref mut fresh17 = (*((*ctx).amqueue).offset((*ctx).amqueue_used as isize)).ptr;
    *fresh17 = ptr;
    (*ctx).amqueue_used += 1;
}
#[no_mangle]
pub unsafe extern "C" fn autoMemoryFreed(
    mut ctx: *mut RedisModuleCtx,
    mut type_0: libc::c_int,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    if (*ctx).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return 0 as libc::c_int;
    }
    let mut count: libc::c_int = ((*ctx).amqueue_used + 1 as libc::c_int)
        / 2 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < count {
        let mut side: libc::c_int = 0 as libc::c_int;
        while side < 2 as libc::c_int {
            let mut i: libc::c_int = if side == 0 as libc::c_int {
                (*ctx).amqueue_used - 1 as libc::c_int - j
            } else {
                j
            };
            if (*((*ctx).amqueue).offset(i as isize)).type_0 == type_0
                && (*((*ctx).amqueue).offset(i as isize)).ptr == ptr
            {
                (*((*ctx).amqueue).offset(i as isize)).type_0 = 3 as libc::c_int;
                if i != (*ctx).amqueue_used - 1 as libc::c_int {
                    *((*ctx).amqueue)
                        .offset(
                            i as isize,
                        ) = *((*ctx).amqueue)
                        .offset(((*ctx).amqueue_used - 1 as libc::c_int) as isize);
                }
                (*ctx).amqueue_used -= 1;
                return 1 as libc::c_int;
            }
            side += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn autoMemoryCollect(mut ctx: *mut RedisModuleCtx) {
    if (*ctx).flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        return;
    }
    (*ctx).flags &= !((1 as libc::c_int) << 0 as libc::c_int);
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*ctx).amqueue_used {
        let mut ptr: *mut libc::c_void = (*((*ctx).amqueue).offset(j as isize)).ptr;
        match (*((*ctx).amqueue).offset(j as isize)).type_0 {
            1 => {
                decrRefCount(ptr as *mut robj);
            }
            2 => {
                RM_FreeCallReply(ptr as *mut RedisModuleCallReply);
            }
            0 => {
                RM_CloseKey(ptr as *mut RedisModuleKey);
            }
            4 => {
                RM_FreeDict(0 as *mut RedisModuleCtx, ptr as *mut RedisModuleDict);
            }
            5 => {
                RM_FreeServerInfo(
                    0 as *mut RedisModuleCtx,
                    ptr as *mut RedisModuleServerInfoData,
                );
            }
            _ => {}
        }
        j += 1;
    }
    (*ctx).flags |= (1 as libc::c_int) << 0 as libc::c_int;
    zfree((*ctx).amqueue as *mut libc::c_void);
    (*ctx).amqueue = 0 as *mut AutoMemEntry;
    (*ctx).amqueue_len = 0 as libc::c_int;
    (*ctx).amqueue_used = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateString(
    mut ctx: *mut RedisModuleCtx,
    mut ptr: *const libc::c_char,
    mut len: size_t,
) -> *mut robj {
    let mut o: *mut robj = createStringObject(ptr, len);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, o as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringPrintf(
    mut ctx: *mut RedisModuleCtx,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut robj {
    let mut s: sds = sdsempty();
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    s = sdscatvprintf(s, fmt, ap.as_va_list());
    let mut o: *mut robj = createObject(0 as libc::c_int, s as *mut libc::c_void);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, o as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromLongLong(
    mut ctx: *mut RedisModuleCtx,
    mut ll: libc::c_longlong,
) -> *mut robj {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut len: size_t = ll2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
        ll,
    ) as size_t;
    return RM_CreateString(ctx, buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromULongLong(
    mut ctx: *mut RedisModuleCtx,
    mut ull: libc::c_ulonglong,
) -> *mut robj {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut len: size_t = ull2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
        ull,
    ) as size_t;
    return RM_CreateString(ctx, buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromDouble(
    mut ctx: *mut RedisModuleCtx,
    mut d: libc::c_double,
) -> *mut robj {
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = d2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
        d,
    ) as size_t;
    return RM_CreateString(ctx, buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromLongDouble(
    mut ctx: *mut RedisModuleCtx,
    mut ld: f64,
    mut humanfriendly: libc::c_int,
) -> *mut robj {
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut len: size_t = ld2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 5120]>() as libc::c_ulong,
        ld,
        (if humanfriendly != 0 {
            LD_STR_HUMAN as libc::c_int
        } else {
            LD_STR_AUTO as libc::c_int
        }) as ld2string_mode,
    ) as size_t;
    return RM_CreateString(ctx, buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromString(
    mut ctx: *mut RedisModuleCtx,
    mut str: *const robj,
) -> *mut robj {
    let mut o: *mut robj = dupStringObject(str);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, o as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromStreamID(
    mut ctx: *mut RedisModuleCtx,
    mut id: *const RedisModuleStreamID,
) -> *mut robj {
    let mut streamid: streamID = {
        let mut init = streamID {
            ms: (*id).ms,
            seq: (*id).seq,
        };
        init
    };
    let mut o: *mut robj = createObjectFromStreamID(&mut streamid);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, o as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeString(
    mut ctx: *mut RedisModuleCtx,
    mut str: *mut robj,
) {
    decrRefCount(str);
    if !ctx.is_null() {
        autoMemoryFreed(ctx, 1 as libc::c_int, str as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_RetainString(
    mut ctx: *mut RedisModuleCtx,
    mut str: *mut robj,
) {
    if ctx.is_null()
        || autoMemoryFreed(ctx, 1 as libc::c_int, str as *mut libc::c_void) == 0
    {
        incrRefCount(str);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_HoldString(
    mut ctx: *mut RedisModuleCtx,
    mut str: *mut robj,
) -> *mut robj {
    if (*str).refcount == 2147483647 as libc::c_int - 1 as libc::c_int {
        return RM_CreateStringFromString(ctx, str);
    }
    incrRefCount(str);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, str as *mut libc::c_void);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringPtrLen(
    mut str: *const robj,
    mut len: *mut size_t,
) -> *const libc::c_char {
    if str.is_null() {
        let mut errmsg: *const libc::c_char = b"(NULL string reply referenced in module)\0"
            as *const u8 as *const libc::c_char;
        if !len.is_null() {
            *len = strlen(errmsg);
        }
        return errmsg;
    }
    if !len.is_null() {
        *len = sdslen((*str).ptr as sds);
    }
    return (*str).ptr as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringToLongLong(
    mut str: *const robj,
    mut ll: *mut libc::c_longlong,
) -> libc::c_int {
    return if string2ll((*str).ptr as *const libc::c_char, sdslen((*str).ptr as sds), ll)
        != 0
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringToULongLong(
    mut str: *const robj,
    mut ull: *mut libc::c_ulonglong,
) -> libc::c_int {
    return if string2ull((*str).ptr as *const libc::c_char, ull) != 0 {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringToDouble(
    mut str: *const robj,
    mut d: *mut libc::c_double,
) -> libc::c_int {
    let mut retval: libc::c_int = getDoubleFromObject(str, d);
    return if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringToLongDouble(
    mut str: *const robj,
    mut ld: *mut f64,
) -> libc::c_int {
    let mut retval: libc::c_int = string2ld(
        (*str).ptr as *const libc::c_char,
        sdslen((*str).ptr as sds),
        ld,
    );
    return if retval != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringToStreamID(
    mut str: *const robj,
    mut id: *mut RedisModuleStreamID,
) -> libc::c_int {
    let mut streamid: streamID = streamID { ms: 0, seq: 0 };
    if streamParseID(str, &mut streamid) == 0 as libc::c_int {
        (*id).ms = streamid.ms;
        (*id).seq = streamid.seq;
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringCompare(
    mut a: *mut robj,
    mut b: *mut robj,
) -> libc::c_int {
    return compareStringObjects(a, b);
}
#[no_mangle]
pub unsafe extern "C" fn moduleAssertUnsharedString(mut str: *mut robj) -> *mut robj {
    if (*str).refcount != 1 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Module attempted to use an in-place string modify operation with a string referenced multiple times. Please check the code for API usage correctness.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return 0 as *mut robj;
    }
    if (*str).encoding() as libc::c_int == 8 as libc::c_int {
        (*str)
            .ptr = sdsnewlen((*str).ptr, sdslen((*str).ptr as sds)) as *mut libc::c_void;
        (*str).set_encoding(0 as libc::c_int as libc::c_uint);
    } else if (*str).encoding() as libc::c_int == 1 as libc::c_int {
        (*str)
            .ptr = sdsfromlonglong((*str).ptr as libc::c_long as libc::c_longlong)
            as *mut libc::c_void;
        (*str).set_encoding(0 as libc::c_int as libc::c_uint);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringAppendBuffer(
    mut ctx: *mut RedisModuleCtx,
    mut str: *mut robj,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    str = moduleAssertUnsharedString(str);
    if str.is_null() {
        return 1 as libc::c_int;
    }
    (*str)
        .ptr = sdscatlen((*str).ptr as sds, buf as *const libc::c_void, len)
        as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_TrimStringAllocation(mut str: *mut robj) {
    if str.is_null() {
        return;
    }
    trimStringObjectIfNeeded(str);
}
#[no_mangle]
pub unsafe extern "C" fn RM_WrongArity(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    addReplyErrorArity((*ctx).client);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetReplyClient(
    mut ctx: *mut RedisModuleCtx,
) -> *mut client {
    if (*ctx).flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        if !((*ctx).blocked_client).is_null() {
            return (*(*ctx).blocked_client).reply_client
        } else {
            return 0 as *mut client
        }
    } else {
        return (*ctx).client
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithLongLong(
    mut ctx: *mut RedisModuleCtx,
    mut ll: libc::c_longlong,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyLongLong(c, ll);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithError(
    mut ctx: *mut RedisModuleCtx,
    mut err: *const libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyErrorFormat(c, b"-%s\0" as *const u8 as *const libc::c_char, err);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithSimpleString(
    mut ctx: *mut RedisModuleCtx,
    mut msg: *const libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyProto(
        c,
        b"+\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    addReplyProto(c, msg, strlen(msg));
    addReplyProto(
        c,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleReplyWithCollection(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
    mut type_0: libc::c_int,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    if len == -(1 as libc::c_int) as libc::c_long {
        (*ctx)
            .postponed_arrays = zrealloc(
            (*ctx).postponed_arrays as *mut libc::c_void,
            (core::mem::size_of::<*mut libc::c_void>() as libc::c_ulong)
                .wrapping_mul(
                    ((*ctx).postponed_arrays_count + 1 as libc::c_int) as libc::c_ulong,
                ),
        ) as *mut *mut libc::c_void;
        let ref mut fresh18 = *((*ctx).postponed_arrays)
            .offset((*ctx).postponed_arrays_count as isize);
        *fresh18 = addReplyDeferredLen(c);
        (*ctx).postponed_arrays_count += 1;
    } else if len == 0 as libc::c_int as libc::c_long {
        match type_0 {
            1 => {
                addReply(c, shared.emptyarray);
            }
            2 => {
                addReply(c, shared.emptymap[(*c).resp as usize]);
            }
            3 => {
                addReply(c, shared.emptyset[(*c).resp as usize]);
            }
            4 => {
                addReplyAttributeLen(c, len);
            }
            _ => {
                _serverPanic(
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    2793 as libc::c_int,
                    b"Invalid module empty reply type %d\0" as *const u8
                        as *const libc::c_char,
                    type_0,
                );
                unreachable!();
            }
        }
    } else {
        match type_0 {
            1 => {
                addReplyArrayLen(c, len);
            }
            2 => {
                addReplyMapLen(c, len);
            }
            3 => {
                addReplySetLen(c, len);
            }
            4 => {
                addReplyAttributeLen(c, len);
            }
            _ => {
                _serverPanic(
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    2809 as libc::c_int,
                    b"Invalid module reply type %d\0" as *const u8
                        as *const libc::c_char,
                    type_0,
                );
                unreachable!();
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithArray(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) -> libc::c_int {
    return moduleReplyWithCollection(ctx, len, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithMap(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) -> libc::c_int {
    return moduleReplyWithCollection(ctx, len, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithSet(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) -> libc::c_int {
    return moduleReplyWithCollection(ctx, len, 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithAttribute(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) -> libc::c_int {
    if (*(*ctx).client).resp == 2 as libc::c_int {
        return 1 as libc::c_int;
    }
    return moduleReplyWithCollection(ctx, len, 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithNullArray(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyNullArray(c);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithEmptyArray(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReply(c, shared.emptyarray);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleReplySetCollectionLength(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
    mut type_0: libc::c_int,
) {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return;
    }
    if (*ctx).postponed_arrays_count == 0 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"API misuse detected in module %s: RedisModule_ReplySet*Length() called without previous RedisModule_ReplyWith*(ctx,REDISMODULE_POSTPONED_LEN) call.\0"
                    as *const u8 as *const libc::c_char,
                (*(*ctx).module).name,
            );
        }
        return;
    }
    (*ctx).postponed_arrays_count -= 1;
    match type_0 {
        1 => {
            setDeferredArrayLen(
                c,
                *((*ctx).postponed_arrays)
                    .offset((*ctx).postponed_arrays_count as isize),
                len,
            );
        }
        2 => {
            setDeferredMapLen(
                c,
                *((*ctx).postponed_arrays)
                    .offset((*ctx).postponed_arrays_count as isize),
                len,
            );
        }
        3 => {
            setDeferredSetLen(
                c,
                *((*ctx).postponed_arrays)
                    .offset((*ctx).postponed_arrays_count as isize),
                len,
            );
        }
        4 => {
            setDeferredAttributeLen(
                c,
                *((*ctx).postponed_arrays)
                    .offset((*ctx).postponed_arrays_count as isize),
                len,
            );
        }
        _ => {
            _serverPanic(
                b"module.c\0" as *const u8 as *const libc::c_char,
                2931 as libc::c_int,
                b"Invalid module reply type %d\0" as *const u8 as *const libc::c_char,
                type_0,
            );
            unreachable!();
        }
    }
    if (*ctx).postponed_arrays_count == 0 as libc::c_int {
        zfree((*ctx).postponed_arrays as *mut libc::c_void);
        (*ctx).postponed_arrays = 0 as *mut *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplySetArrayLength(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) {
    moduleReplySetCollectionLength(ctx, len, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplySetMapLength(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) {
    moduleReplySetCollectionLength(ctx, len, 2 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplySetSetLength(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) {
    moduleReplySetCollectionLength(ctx, len, 3 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplySetAttributeLength(
    mut ctx: *mut RedisModuleCtx,
    mut len: libc::c_long,
) {
    if (*(*ctx).client).resp == 2 as libc::c_int {
        return;
    }
    moduleReplySetCollectionLength(ctx, len, 4 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithStringBuffer(
    mut ctx: *mut RedisModuleCtx,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyBulkCBuffer(c, buf as *mut libc::c_char as *const libc::c_void, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithCString(
    mut ctx: *mut RedisModuleCtx,
    mut buf: *const libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyBulkCString(c, buf as *mut libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithString(
    mut ctx: *mut RedisModuleCtx,
    mut str: *mut robj,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyBulk(c, str);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithEmptyString(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReply(c, shared.emptybulk);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithVerbatimStringType(
    mut ctx: *mut RedisModuleCtx,
    mut buf: *const libc::c_char,
    mut len: size_t,
    mut ext: *const libc::c_char,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyVerbatim(c, buf, len, ext);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithVerbatimString(
    mut ctx: *mut RedisModuleCtx,
    mut buf: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    return RM_ReplyWithVerbatimStringType(
        ctx,
        buf,
        len,
        b"txt\0" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithNull(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyNull(c);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithBool(
    mut ctx: *mut RedisModuleCtx,
    mut b: libc::c_int,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyBool(c, b);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithCallReply(
    mut ctx: *mut RedisModuleCtx,
    mut reply: *mut RedisModuleCallReply,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    if (*c).resp == 2 as libc::c_int && callReplyIsResp3(reply) != 0 {
        return 1 as libc::c_int;
    }
    let mut proto_len: size_t = 0;
    let mut proto: *const libc::c_char = callReplyGetProto(reply, &mut proto_len);
    addReplyProto(c, proto, proto_len);
    let mut errors: *mut list = callReplyDeferredErrorList(reply);
    if !errors.is_null() {
        deferredAfterErrorReply(c, errors);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithDouble(
    mut ctx: *mut RedisModuleCtx,
    mut d: libc::c_double,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyDouble(c, d);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithBigNumber(
    mut ctx: *mut RedisModuleCtx,
    mut bignum: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyBigNum(c, bignum, len);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplyWithLongDouble(
    mut ctx: *mut RedisModuleCtx,
    mut ld: f64,
) -> libc::c_int {
    let mut c: *mut client = moduleGetReplyClient(ctx);
    if c.is_null() {
        return 0 as libc::c_int;
    }
    addReplyHumanLongDouble(c, ld);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_Replicate(
    mut ctx: *mut RedisModuleCtx,
    mut cmdname: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut argv: *mut *mut robj = 0 as *mut *mut robj;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut ap: core::ffi::VaListImpl;
    cmd = lookupCommandByCString(cmdname as *mut libc::c_char);
    if cmd.is_null() {
        return 1 as libc::c_int;
    }
    ap = args.clone();
    argv = moduleCreateArgvFromUserFormat(
        cmdname,
        fmt,
        &mut argc,
        0 as *mut libc::c_int,
        &mut flags,
        ap.as_va_list(),
    );
    if argv.is_null() {
        return 1 as libc::c_int;
    }
    let mut target: libc::c_int = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        target |= 1 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
        target |= 2 as libc::c_int;
    }
    alsoPropagate((*(*(*ctx).client).db).id, argv, argc, target);
    j = 0 as libc::c_int;
    while j < argc {
        decrRefCount(*argv.offset(j as isize));
        j += 1;
    }
    zfree(argv as *mut libc::c_void);
    server.dirty += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ReplicateVerbatim(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    alsoPropagate(
        (*(*(*ctx).client).db).id,
        (*(*ctx).client).argv,
        (*(*ctx).client).argc,
        1 as libc::c_int | 2 as libc::c_int,
    );
    server.dirty += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClientId(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_ulonglong {
    if ((*ctx).client).is_null() {
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    return (*(*ctx).client).id as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClientUserNameById(
    mut ctx: *mut RedisModuleCtx,
    mut id: uint64_t,
) -> *mut robj {
    let mut client: *mut client = lookupClientByID(id);
    if client.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return 0 as *mut robj;
    }
    if ((*client).user).is_null() {
        *__errno_location() = 95 as libc::c_int;
        return 0 as *mut robj;
    }
    let mut name: sds = sdsnew((*(*client).user).name as *const libc::c_char);
    let mut str: *mut robj = createObject(0 as libc::c_int, name as *mut libc::c_void);
    autoMemoryAdd(ctx, 1 as libc::c_int, str as *mut libc::c_void);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn modulePopulateClientInfoStructure(
    mut ci: *mut libc::c_void,
    mut client: *mut client,
    mut structver: libc::c_int,
) -> libc::c_int {
    if structver != 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut ci1: *mut RedisModuleClientInfoV1 = ci as *mut RedisModuleClientInfoV1;
    memset(
        ci1 as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<RedisModuleClientInfoV1>() as libc::c_ulong,
    );
    (*ci1).version = structver as uint64_t;
    if (*client).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0 {
        (*ci1).flags |= ((1 as libc::c_int) << 5 as libc::c_int) as libc::c_ulong;
    }
    if (*client).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
    {
        (*ci1).flags |= ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong;
    }
    if (*client).flags & ((1 as libc::c_int) << 11 as libc::c_int) as libc::c_ulong != 0
    {
        (*ci1).flags |= ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong;
    }
    if (*client).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
    {
        (*ci1).flags |= ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong;
    }
    if (*client).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong != 0 {
        (*ci1).flags |= ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
    }
    if connGetType((*client).conn) == 2 as libc::c_int {
        (*ci1).flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong;
    }
    let mut port: libc::c_int = 0;
    connPeerToString(
        (*client).conn,
        ((*ci1).addr).as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
        &mut port,
    );
    (*ci1).port = port as uint16_t;
    (*ci1).db = (*(*client).db).id as uint16_t;
    (*ci1).id = (*client).id;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modulePopulateReplicationInfoStructure(
    mut ri: *mut libc::c_void,
    mut structver: libc::c_int,
) -> libc::c_int {
    if structver != 1 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut ri1: *mut RedisModuleReplicationInfoV1 = ri
        as *mut RedisModuleReplicationInfoV1;
    memset(
        ri1 as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<RedisModuleReplicationInfoV1>() as libc::c_ulong,
    );
    (*ri1).version = structver as uint64_t;
    (*ri1)
        .master = (server.masterhost == 0 as *mut libc::c_void as *mut libc::c_char)
        as libc::c_int;
    (*ri1)
        .masterhost = (if !(server.masterhost).is_null() {
        server.masterhost as *const libc::c_char
    } else {
        b"\0" as *const u8 as *const libc::c_char
    }) as *mut libc::c_char;
    (*ri1).masterport = server.masterport;
    (*ri1).replid1 = (server.replid).as_mut_ptr();
    (*ri1).replid2 = (server.replid2).as_mut_ptr();
    (*ri1).repl1_offset = server.master_repl_offset as uint64_t;
    (*ri1).repl2_offset = server.second_replid_offset as uint64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClientInfoById(
    mut ci: *mut libc::c_void,
    mut id: uint64_t,
) -> libc::c_int {
    let mut client: *mut client = lookupClientByID(id);
    if client.is_null() {
        return 1 as libc::c_int;
    }
    if ci.is_null() {
        return 0 as libc::c_int;
    }
    let mut structver: uint64_t = *(ci as *mut uint64_t)
        .offset(0 as libc::c_int as isize);
    return modulePopulateClientInfoStructure(ci, client, structver as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClientNameById(
    mut ctx: *mut RedisModuleCtx,
    mut id: uint64_t,
) -> *mut robj {
    let mut client: *mut client = lookupClientByID(id);
    if client.is_null() || ((*client).name).is_null() {
        return 0 as *mut robj;
    }
    let mut name: *mut robj = (*client).name;
    incrRefCount(name);
    autoMemoryAdd(ctx, 1 as libc::c_int, name as *mut libc::c_void);
    return name;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetClientNameById(
    mut id: uint64_t,
    mut name: *mut robj,
) -> libc::c_int {
    let mut client: *mut client = lookupClientByID(id);
    if client.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return 1 as libc::c_int;
    }
    if clientSetName(client, name) == -(1 as libc::c_int) {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_PublishMessage(
    mut ctx: *mut RedisModuleCtx,
    mut channel: *mut robj,
    mut message: *mut robj,
) -> libc::c_int {
    return pubsubPublishMessageAndPropagateToCluster(channel, message, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_PublishMessageShard(
    mut ctx: *mut RedisModuleCtx,
    mut channel: *mut robj,
    mut message: *mut robj,
) -> libc::c_int {
    return pubsubPublishMessageAndPropagateToCluster(channel, message, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetSelectedDb(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    return (*(*(*ctx).client).db).id;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetContextFlags(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    let mut flags: libc::c_int = 0 as libc::c_int;
    if !ctx.is_null() {
        if !((*ctx).client).is_null() {
            if (*(*ctx).client).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 41 as libc::c_int != 0
            {
                flags |= (1 as libc::c_int) << 21 as libc::c_int;
            }
            if (*(*ctx).client).flags
                & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0
            {
                flags |= (1 as libc::c_int) << 12 as libc::c_int;
            }
            if (*(*ctx).client).resp == 3 as libc::c_int {
                flags |= (1 as libc::c_int) << 22 as libc::c_int;
            }
        }
        let mut c: *mut client = if !((*ctx).blocked_client).is_null() {
            (*(*ctx).blocked_client).client
        } else {
            (*ctx).client
        };
        if !c.is_null()
            && (*c).flags
                & ((1 as libc::c_int) << 5 as libc::c_int
                    | (1 as libc::c_int) << 12 as libc::c_int) as libc::c_ulong != 0
        {
            flags |= (1 as libc::c_int) << 19 as libc::c_int;
        }
    }
    if scriptIsRunning() != 0 {
        flags |= (1 as libc::c_int) << 0 as libc::c_int;
    }
    if server.in_exec != 0 {
        flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if server.cluster_enabled != 0 {
        flags |= (1 as libc::c_int) << 5 as libc::c_int;
    }
    if server.async_loading != 0 {
        flags |= (1 as libc::c_int) << 23 as libc::c_int;
    } else if server.loading != 0 {
        flags |= (1 as libc::c_int) << 13 as libc::c_int;
    }
    if server.maxmemory > 0 as libc::c_int as libc::c_ulonglong
        && ((server.masterhost).is_null() || server.repl_slave_ignore_maxmemory == 0)
    {
        flags |= (1 as libc::c_int) << 8 as libc::c_int;
        if server.maxmemory_policy != (7 as libc::c_int) << 8 as libc::c_int {
            flags |= (1 as libc::c_int) << 9 as libc::c_int;
        }
    }
    if server.aof_state != 0 as libc::c_int {
        flags |= (1 as libc::c_int) << 6 as libc::c_int;
    }
    if server.saveparamslen > 0 as libc::c_int {
        flags |= (1 as libc::c_int) << 7 as libc::c_int;
    }
    if (server.masterhost).is_null() {
        flags |= (1 as libc::c_int) << 2 as libc::c_int;
    } else {
        flags |= (1 as libc::c_int) << 3 as libc::c_int;
        if server.repl_slave_ro != 0 {
            flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        if server.repl_state == REPL_STATE_CONNECT as libc::c_int
            || server.repl_state == REPL_STATE_CONNECTING as libc::c_int
        {
            flags |= (1 as libc::c_int) << 15 as libc::c_int;
        } else if server.repl_state == REPL_STATE_TRANSFER as libc::c_int {
            flags |= (1 as libc::c_int) << 16 as libc::c_int;
        } else if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
            flags |= (1 as libc::c_int) << 17 as libc::c_int;
        }
        if server.repl_state != REPL_STATE_CONNECTED as libc::c_int {
            flags |= (1 as libc::c_int) << 14 as libc::c_int;
        }
    }
    let mut level: libc::c_float = 0.;
    let mut retval: libc::c_int = getMaxmemoryState(
        0 as *mut size_t,
        0 as *mut size_t,
        0 as *mut size_t,
        &mut level,
    );
    if retval == -(1 as libc::c_int) {
        flags |= (1 as libc::c_int) << 10 as libc::c_int;
    }
    if level as libc::c_double > 0.75f64 {
        flags |= (1 as libc::c_int) << 11 as libc::c_int;
    }
    if hasActiveChildProcess() != 0 {
        flags |= (1 as libc::c_int) << 18 as libc::c_int;
    }
    if server.in_fork_child != 0 {
        flags |= (1 as libc::c_int) << 20 as libc::c_int;
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn RM_AvoidReplicaTraffic() -> libc::c_int {
    return checkClientPauseTimeoutAndReturnIfPaused();
}
#[no_mangle]
pub unsafe extern "C" fn RM_SelectDb(
    mut ctx: *mut RedisModuleCtx,
    mut newid: libc::c_int,
) -> libc::c_int {
    let mut retval: libc::c_int = selectDb((*ctx).client, newid);
    return if retval == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_KeyExists(
    mut ctx: *mut RedisModuleCtx,
    mut keyname: *mut robj,
) -> libc::c_int {
    let mut value: *mut robj = lookupKeyReadWithFlags(
        (*(*ctx).client).db,
        keyname,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    return (value != 0 as *mut libc::c_void as *mut robj) as libc::c_int;
}
unsafe extern "C" fn moduleInitKey(
    mut kp: *mut RedisModuleKey,
    mut ctx: *mut RedisModuleCtx,
    mut keyname: *mut robj,
    mut value: *mut robj,
    mut mode: libc::c_int,
) {
    (*kp).ctx = ctx;
    (*kp).db = (*(*ctx).client).db;
    (*kp).key = keyname;
    incrRefCount(keyname);
    (*kp).value = value;
    (*kp).iter = 0 as *mut libc::c_void;
    (*kp).mode = mode;
    if !((*kp).value).is_null() {
        moduleInitKeyTypeSpecific(kp);
    }
}
unsafe extern "C" fn moduleInitKeyTypeSpecific(mut key: *mut RedisModuleKey) {
    match (*(*key).value).type_0() as libc::c_int {
        3 => {
            zsetKeyReset(key);
        }
        6 => {
            (*key).u.stream.signalready = 0 as libc::c_int;
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_OpenKey(
    mut ctx: *mut RedisModuleCtx,
    mut keyname: *mut robj,
    mut mode: libc::c_int,
) -> *mut RedisModuleKey {
    let mut kp: *mut RedisModuleKey = 0 as *mut RedisModuleKey;
    let mut value: *mut robj = 0 as *mut robj;
    let mut flags: libc::c_int = if mode & (1 as libc::c_int) << 16 as libc::c_int != 0 {
        (1 as libc::c_int) << 0 as libc::c_int
    } else {
        0 as libc::c_int
    };
    if mode & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        value = lookupKeyWriteWithFlags((*(*ctx).client).db, keyname, flags);
    } else {
        value = lookupKeyReadWithFlags((*(*ctx).client).db, keyname, flags);
        if value.is_null() {
            return 0 as *mut RedisModuleKey;
        }
    }
    kp = zmalloc(core::mem::size_of::<RedisModuleKey>() as libc::c_ulong)
        as *mut RedisModuleKey;
    moduleInitKey(kp, ctx, keyname, value, mode);
    autoMemoryAdd(ctx, 0 as libc::c_int, kp as *mut libc::c_void);
    return kp;
}
unsafe extern "C" fn moduleCloseKey(mut key: *mut RedisModuleKey) {
    let mut signal: libc::c_int = if !((*(*key).ctx).module).is_null() {
        ((*(*(*key).ctx).module).options & (1 as libc::c_int) << 1 as libc::c_int == 0)
            as libc::c_int
    } else {
        1 as libc::c_int
    };
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int != 0 && signal != 0 {
        signalModifiedKey((*(*key).ctx).client, (*key).db, (*key).key);
    }
    if !((*key).value).is_null() {
        if !((*key).iter).is_null() {
            moduleFreeKeyIterator(key);
        }
        match (*(*key).value).type_0() as libc::c_int {
            3 => {
                RM_ZsetRangeStop(key);
            }
            6 => {
                if (*key).u.stream.signalready != 0 {
                    signalKeyAsReady((*key).db, (*key).key, 6 as libc::c_int);
                }
            }
            _ => {}
        }
    }
    if ((*key).iter).is_null() {} else {
        _serverAssert(
            b"key->iter == NULL\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            3737 as libc::c_int,
        );
        unreachable!();
    };
    decrRefCount((*key).key);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CloseKey(mut key: *mut RedisModuleKey) {
    if key.is_null() {
        return;
    }
    moduleCloseKey(key);
    autoMemoryFreed((*key).ctx, 0 as libc::c_int, key as *mut libc::c_void);
    zfree(key as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_KeyType(mut key: *mut RedisModuleKey) -> libc::c_int {
    if key.is_null() || ((*key).value).is_null() {
        return 0 as libc::c_int;
    }
    match (*(*key).value).type_0() as libc::c_int {
        0 => return 1 as libc::c_int,
        1 => return 2 as libc::c_int,
        2 => return 4 as libc::c_int,
        3 => return 5 as libc::c_int,
        4 => return 3 as libc::c_int,
        5 => return 6 as libc::c_int,
        6 => return 7 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_ValueLength(mut key: *mut RedisModuleKey) -> size_t {
    if key.is_null() || ((*key).value).is_null() {
        return 0 as libc::c_int as size_t;
    }
    match (*(*key).value).type_0() as libc::c_int {
        0 => return stringObjectLen((*key).value),
        1 => return listTypeLength((*key).value),
        2 => return setTypeSize((*key).value),
        3 => return zsetLength((*key).value),
        4 => return hashTypeLength((*key).value),
        6 => return streamLength((*key).value),
        _ => return 0 as libc::c_int as size_t,
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DeleteKey(mut key: *mut RedisModuleKey) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null() {
        dbDelete((*key).db, (*key).key);
        (*key).value = 0 as *mut robj;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_UnlinkKey(mut key: *mut RedisModuleKey) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null() {
        dbAsyncDelete((*key).db, (*key).key);
        (*key).value = 0 as *mut robj;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetExpire(mut key: *mut RedisModuleKey) -> mstime_t {
    let mut expire: mstime_t = getExpire((*key).db, (*key).key);
    if expire == -(1 as libc::c_int) as libc::c_longlong || ((*key).value).is_null() {
        return -(1 as libc::c_int) as mstime_t;
    }
    expire -= mstime();
    return if expire >= 0 as libc::c_int as libc::c_longlong {
        expire
    } else {
        0 as libc::c_int as libc::c_longlong
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetExpire(
    mut key: *mut RedisModuleKey,
    mut expire: mstime_t,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || ((*key).value).is_null()
        || expire < 0 as libc::c_int as libc::c_longlong
            && expire != -(1 as libc::c_int) as libc::c_longlong
    {
        return 1 as libc::c_int;
    }
    if expire != -(1 as libc::c_int) as libc::c_longlong {
        expire += mstime();
        setExpire((*(*key).ctx).client, (*key).db, (*key).key, expire);
    } else {
        removeExpire((*key).db, (*key).key);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetAbsExpire(mut key: *mut RedisModuleKey) -> mstime_t {
    let mut expire: mstime_t = getExpire((*key).db, (*key).key);
    if expire == -(1 as libc::c_int) as libc::c_longlong || ((*key).value).is_null() {
        return -(1 as libc::c_int) as mstime_t;
    }
    return expire;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetAbsExpire(
    mut key: *mut RedisModuleKey,
    mut expire: mstime_t,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || ((*key).value).is_null()
        || expire < 0 as libc::c_int as libc::c_longlong
            && expire != -(1 as libc::c_int) as libc::c_longlong
    {
        return 1 as libc::c_int;
    }
    if expire != -(1 as libc::c_int) as libc::c_longlong {
        setExpire((*(*key).ctx).client, (*key).db, (*key).key, expire);
    } else {
        removeExpire((*key).db, (*key).key);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ResetDataset(
    mut restart_aof: libc::c_int,
    mut async_0: libc::c_int,
) {
    if restart_aof != 0 && server.aof_state != 0 as libc::c_int {
        stopAppendOnly();
    }
    flushAllDataAndResetRDB(
        (if async_0 != 0 {
            (1 as libc::c_int) << 0 as libc::c_int
        } else {
            0 as libc::c_int
        }) | (1 as libc::c_int) << 1 as libc::c_int,
    );
    if server.aof_enabled != 0 && restart_aof != 0 {
        restartAOFAfterSYNC();
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_DbSize(mut ctx: *mut RedisModuleCtx) -> libc::c_ulonglong {
    return ((*(*(*(*ctx).client).db).dict).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*(*(*ctx).client).db).dict).ht_used[1 as libc::c_int as usize])
        as libc::c_ulonglong;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RandomKey(mut ctx: *mut RedisModuleCtx) -> *mut robj {
    let mut key: *mut robj = dbRandomKey((*(*ctx).client).db);
    autoMemoryAdd(ctx, 1 as libc::c_int, key as *mut libc::c_void);
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyNameFromOptCtx(
    mut ctx: *mut RedisModuleKeyOptCtx,
) -> *const robj {
    return (*ctx).from_key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetToKeyNameFromOptCtx(
    mut ctx: *mut RedisModuleKeyOptCtx,
) -> *const robj {
    return (*ctx).to_key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDbIdFromOptCtx(
    mut ctx: *mut RedisModuleKeyOptCtx,
) -> libc::c_int {
    return (*ctx).from_dbid;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetToDbIdFromOptCtx(
    mut ctx: *mut RedisModuleKeyOptCtx,
) -> libc::c_int {
    return (*ctx).to_dbid;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringSet(
    mut key: *mut RedisModuleKey,
    mut str: *mut robj,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || !((*key).iter).is_null()
    {
        return 1 as libc::c_int;
    }
    RM_DeleteKey(key);
    setKey((*(*key).ctx).client, (*key).db, (*key).key, str, 2 as libc::c_int);
    (*key).value = str;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringDMA(
    mut key: *mut RedisModuleKey,
    mut len: *mut size_t,
    mut mode: libc::c_int,
) -> *mut libc::c_char {
    let mut emptystring: *mut libc::c_char = b"<dma-empty-string>\0" as *const u8
        as *const libc::c_char as *mut libc::c_char;
    if ((*key).value).is_null() {
        *len = 0 as libc::c_int as size_t;
        return emptystring;
    }
    if (*(*key).value).type_0() as libc::c_int != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if mode & (1 as libc::c_int) << 1 as libc::c_int != 0
        || (*(*key).value).encoding() as libc::c_int != 0 as libc::c_int
    {
        (*key).value = dbUnshareStringValue((*key).db, (*key).key, (*key).value);
    }
    *len = sdslen((*(*key).value).ptr as sds);
    return (*(*key).value).ptr as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StringTruncate(
    mut key: *mut RedisModuleKey,
    mut newlen: size_t,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if newlen
        > (512 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
            as libc::c_ulong
    {
        return 1 as libc::c_int;
    }
    if ((*key).value).is_null() && newlen == 0 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int;
    }
    if ((*key).value).is_null() {
        let mut o: *mut robj = createObject(
            0 as libc::c_int,
            sdsnewlen(0 as *const libc::c_void, newlen) as *mut libc::c_void,
        );
        setKey((*(*key).ctx).client, (*key).db, (*key).key, o, 2 as libc::c_int);
        (*key).value = o;
        decrRefCount(o);
    } else {
        (*key).value = dbUnshareStringValue((*key).db, (*key).key, (*key).value);
        let mut curlen: size_t = sdslen((*(*key).value).ptr as sds);
        if newlen > curlen {
            (*(*key).value)
                .ptr = sdsgrowzero((*(*key).value).ptr as sds, newlen)
                as *mut libc::c_void;
        } else if newlen < curlen {
            sdssubstr((*(*key).value).ptr as sds, 0 as libc::c_int as size_t, newlen);
            if sdslen((*(*key).value).ptr as sds) < sdsavail((*(*key).value).ptr as sds)
            {
                (*(*key).value)
                    .ptr = sdsRemoveFreeSpace((*(*key).value).ptr as sds)
                    as *mut libc::c_void;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleListIteratorSeek(
    mut key: *mut RedisModuleKey,
    mut index: libc::c_long,
    mut mode: libc::c_int,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 1 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 0 as libc::c_int;
        }
    }
    if (*key).mode & mode == 0 {
        *__errno_location() = 9 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut length: libc::c_long = listTypeLength((*key).value) as libc::c_long;
    if index < -length || index >= length {
        *__errno_location() = 33 as libc::c_int;
        return 0 as libc::c_int;
    }
    if ((*key).iter).is_null() {
        (*key)
            .iter = listTypeInitIterator(
            (*key).value,
            index,
            1 as libc::c_int as libc::c_uchar,
        ) as *mut libc::c_void;
        if !((*key).iter).is_null() {} else {
            _serverAssert(
                b"key->iter != NULL\0" as *const u8 as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                4080 as libc::c_int,
            );
            unreachable!();
        };
        if listTypeNext((*key).iter as *mut listTypeIterator, &mut (*key).u.list.entry)
            != 0
        {} else {
            _serverAssert(
                b"listTypeNext(key->iter, &key->u.list.entry)\0" as *const u8
                    as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                4081 as libc::c_int,
            );
            unreachable!();
        };
        (*key).u.list.index = index;
        return 1 as libc::c_int;
    }
    if index < 0 as libc::c_int as libc::c_long
        && (*key).u.list.index >= 0 as libc::c_int as libc::c_long
    {
        index += length;
    } else if index >= 0 as libc::c_int as libc::c_long
        && (*key).u.list.index < 0 as libc::c_int as libc::c_long
    {
        index -= length;
    }
    if index == (*key).u.list.index {
        return 1 as libc::c_int;
    }
    let mut dir: libc::c_uchar = (if (*key).u.list.index < index {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uchar;
    listTypeSetIteratorDirection((*key).iter as *mut listTypeIterator, dir);
    while (*key).u.list.index != index {
        if listTypeNext((*key).iter as *mut listTypeIterator, &mut (*key).u.list.entry)
            != 0
        {} else {
            _serverAssert(
                b"listTypeNext(key->iter, &key->u.list.entry)\0" as *const u8
                    as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                4097 as libc::c_int,
            );
            unreachable!();
        };
        (*key).u.list.index
            += (if dir as libc::c_int == 0 as libc::c_int {
                -(1 as libc::c_int)
            } else {
                1 as libc::c_int
            }) as libc::c_long;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListPush(
    mut key: *mut RedisModuleKey,
    mut where_0: libc::c_int,
    mut ele: *mut robj,
) -> libc::c_int {
    if key.is_null() || ele.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if !((*key).value).is_null()
            && (*(*key).value).type_0() as libc::c_int != 1 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        }
    }
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        *__errno_location() = 9 as libc::c_int;
        return 1 as libc::c_int;
    }
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if !((*key).iter).is_null() {
        moduleFreeKeyIterator(key);
    }
    if ((*key).value).is_null() {
        moduleCreateEmptyKey(key, 2 as libc::c_int);
    }
    listTypePush(
        (*key).value,
        ele,
        if where_0 == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListPop(
    mut key: *mut RedisModuleKey,
    mut where_0: libc::c_int,
) -> *mut robj {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut robj;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 1 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 0 as *mut robj;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                *__errno_location() = 9 as libc::c_int;
                return 0 as *mut robj;
            }
        }
    }
    if !((*key).iter).is_null() {
        moduleFreeKeyIterator(key);
    }
    let mut ele: *mut robj = listTypePop(
        (*key).value,
        if where_0 == 0 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int },
    );
    let mut decoded: *mut robj = getDecodedObject(ele);
    decrRefCount(ele);
    moduleDelKeyIfEmpty(key);
    autoMemoryAdd((*key).ctx, 1 as libc::c_int, decoded as *mut libc::c_void);
    return decoded;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListGet(
    mut key: *mut RedisModuleKey,
    mut index: libc::c_long,
) -> *mut robj {
    if moduleListIteratorSeek(key, index, (1 as libc::c_int) << 0 as libc::c_int) != 0 {
        let mut elem: *mut robj = listTypeGet(&mut (*key).u.list.entry);
        let mut decoded: *mut robj = getDecodedObject(elem);
        decrRefCount(elem);
        autoMemoryAdd((*key).ctx, 1 as libc::c_int, decoded as *mut libc::c_void);
        return decoded;
    } else {
        return 0 as *mut robj
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListSet(
    mut key: *mut RedisModuleKey,
    mut index: libc::c_long,
    mut value: *mut robj,
) -> libc::c_int {
    if value.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    if moduleListIteratorSeek(key, index, (1 as libc::c_int) << 1 as libc::c_int) != 0 {
        listTypeReplace(&mut (*key).u.list.entry, value);
        moduleFreeKeyIterator(key);
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListInsert(
    mut key: *mut RedisModuleKey,
    mut index: libc::c_long,
    mut value: *mut robj,
) -> libc::c_int {
    if value.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if !key.is_null() && ((*key).value).is_null()
            && (index == 0 as libc::c_int as libc::c_long
                || index == -(1 as libc::c_int) as libc::c_long)
        {
            return RM_ListPush(key, 1 as libc::c_int, value)
        } else {
            if !key.is_null() && !((*key).value).is_null()
                && (*(*key).value).type_0() as libc::c_int == 1 as libc::c_int
                && (index == listTypeLength((*key).value) as libc::c_long
                    || index == -(1 as libc::c_int) as libc::c_long)
            {
                return RM_ListPush(key, 1 as libc::c_int, value)
            } else {
                if !key.is_null() && !((*key).value).is_null()
                    && (*(*key).value).type_0() as libc::c_int == 1 as libc::c_int
                    && (index == 0 as libc::c_int as libc::c_long
                        || index
                            == -(listTypeLength((*key).value) as libc::c_long)
                                - 1 as libc::c_int as libc::c_long)
                {
                    return RM_ListPush(key, 0 as libc::c_int, value);
                }
            }
        }
    }
    if moduleListIteratorSeek(key, index, (1 as libc::c_int) << 1 as libc::c_int) != 0 {
        let mut where_0: libc::c_int = if index < 0 as libc::c_int as libc::c_long {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        listTypeInsert(&mut (*key).u.list.entry, value, where_0);
        moduleFreeKeyIterator(key);
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_ListDelete(
    mut key: *mut RedisModuleKey,
    mut index: libc::c_long,
) -> libc::c_int {
    if moduleListIteratorSeek(key, index, (1 as libc::c_int) << 1 as libc::c_int) != 0 {
        listTypeDelete((*key).iter as *mut listTypeIterator, &mut (*key).u.list.entry);
        if moduleDelKeyIfEmpty(key) != 0 {
            return 0 as libc::c_int;
        }
        if listTypeNext((*key).iter as *mut listTypeIterator, &mut (*key).u.list.entry)
            != 0
        {
            let mut li: *mut listTypeIterator = (*key).iter as *mut listTypeIterator;
            let mut reverse: libc::c_int = ((*li).direction as libc::c_int
                == 0 as libc::c_int) as libc::c_int;
            if (*key).u.list.index < 0 as libc::c_int as libc::c_long {
                (*key).u.list.index
                    += (if reverse != 0 { 0 as libc::c_int } else { 1 as libc::c_int })
                        as libc::c_long;
            } else {
                (*key).u.list.index
                    += (if reverse != 0 {
                        -(1 as libc::c_int)
                    } else {
                        0 as libc::c_int
                    }) as libc::c_long;
            }
        } else {
            moduleFreeKeyIterator(key);
        }
        return 0 as libc::c_int;
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn moduleZsetAddFlagsToCoreFlags(
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut retflags: libc::c_int = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    return retflags;
}
#[no_mangle]
pub unsafe extern "C" fn moduleZsetAddFlagsFromCoreFlags(
    mut flags: libc::c_int,
) -> libc::c_int {
    let mut retflags: libc::c_int = 0 as libc::c_int;
    if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 3 as libc::c_int;
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        retflags |= (1 as libc::c_int) << 4 as libc::c_int;
    }
    return retflags;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetAdd(
    mut key: *mut RedisModuleKey,
    mut score: libc::c_double,
    mut ele: *mut robj,
    mut flagsptr: *mut libc::c_int,
) -> libc::c_int {
    let mut in_flags: libc::c_int = 0 as libc::c_int;
    let mut out_flags: libc::c_int = 0 as libc::c_int;
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if ((*key).value).is_null() {
        moduleCreateEmptyKey(key, 5 as libc::c_int);
    }
    if !flagsptr.is_null() {
        in_flags = moduleZsetAddFlagsToCoreFlags(*flagsptr);
    }
    if zsetAdd(
        (*key).value,
        score,
        (*ele).ptr as sds,
        in_flags,
        &mut out_flags,
        0 as *mut libc::c_double,
    ) == 0 as libc::c_int
    {
        if !flagsptr.is_null() {
            *flagsptr = 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if !flagsptr.is_null() {
        *flagsptr = moduleZsetAddFlagsFromCoreFlags(out_flags);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetIncrby(
    mut key: *mut RedisModuleKey,
    mut score: libc::c_double,
    mut ele: *mut robj,
    mut flagsptr: *mut libc::c_int,
    mut newscore: *mut libc::c_double,
) -> libc::c_int {
    let mut in_flags: libc::c_int = 0 as libc::c_int;
    let mut out_flags: libc::c_int = 0 as libc::c_int;
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if ((*key).value).is_null() {
        moduleCreateEmptyKey(key, 5 as libc::c_int);
    }
    if !flagsptr.is_null() {
        in_flags = moduleZsetAddFlagsToCoreFlags(*flagsptr);
    }
    in_flags |= (1 as libc::c_int) << 0 as libc::c_int;
    if zsetAdd(
        (*key).value,
        score,
        (*ele).ptr as sds,
        in_flags,
        &mut out_flags,
        newscore,
    ) == 0 as libc::c_int
    {
        if !flagsptr.is_null() {
            *flagsptr = 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    if !flagsptr.is_null() {
        *flagsptr = moduleZsetAddFlagsFromCoreFlags(out_flags);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRem(
    mut key: *mut RedisModuleKey,
    mut ele: *mut robj,
    mut deleted: *mut libc::c_int,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if !((*key).value).is_null() && zsetDel((*key).value, (*ele).ptr as sds) != 0 {
        if !deleted.is_null() {
            *deleted = 1 as libc::c_int;
        }
        moduleDelKeyIfEmpty(key);
    } else if !deleted.is_null() {
        *deleted = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetScore(
    mut key: *mut RedisModuleKey,
    mut ele: *mut robj,
    mut score: *mut libc::c_double,
) -> libc::c_int {
    if ((*key).value).is_null() {
        return 1 as libc::c_int;
    }
    if (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    if zsetScore((*key).value, (*ele).ptr as sds, score) == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn zsetKeyReset(mut key: *mut RedisModuleKey) {
    (*key).u.zset.type_0 = 0 as libc::c_int as uint32_t;
    (*key).u.zset.current = 0 as *mut libc::c_void;
    (*key).u.zset.er = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRangeStop(mut key: *mut RedisModuleKey) {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return;
    }
    if (*key).u.zset.type_0 == 1 as libc::c_int as libc::c_uint {
        zslFreeLexRange(&mut (*key).u.zset.lrs);
    }
    zsetKeyReset(key);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRangeEndReached(
    mut key: *mut RedisModuleKey,
) -> libc::c_int {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return (*key).u.zset.er;
}
#[no_mangle]
pub unsafe extern "C" fn zsetInitScoreRange(
    mut key: *mut RedisModuleKey,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut minex: libc::c_int,
    mut maxex: libc::c_int,
    mut first: libc::c_int,
) -> libc::c_int {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    RM_ZsetRangeStop(key);
    (*key).u.zset.type_0 = 2 as libc::c_int as uint32_t;
    (*key).u.zset.er = 0 as libc::c_int;
    let mut zrs: *mut zrangespec = &mut (*key).u.zset.rs;
    (*zrs).min = min;
    (*zrs).max = max;
    (*zrs).minex = minex;
    (*zrs).maxex = maxex;
    if (*(*key).value).encoding() as libc::c_int == 11 as libc::c_int {
        (*key)
            .u
            .zset
            .current = (if first != 0 {
            zzlFirstInRange((*(*key).value).ptr as *mut libc::c_uchar, zrs)
        } else {
            zzlLastInRange((*(*key).value).ptr as *mut libc::c_uchar, zrs)
        }) as *mut libc::c_void;
    } else if (*(*key).value).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*(*key).value).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        (*key)
            .u
            .zset
            .current = (if first != 0 {
            zslFirstInRange(zsl, zrs)
        } else {
            zslLastInRange(zsl, zrs)
        }) as *mut libc::c_void;
    } else {
        _serverPanic(
            b"module.c\0" as *const u8 as *const libc::c_char,
            4514 as libc::c_int,
            b"Unsupported zset encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    if ((*key).u.zset.current).is_null() {
        (*key).u.zset.er = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetFirstInScoreRange(
    mut key: *mut RedisModuleKey,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut minex: libc::c_int,
    mut maxex: libc::c_int,
) -> libc::c_int {
    return zsetInitScoreRange(key, min, max, minex, maxex, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetLastInScoreRange(
    mut key: *mut RedisModuleKey,
    mut min: libc::c_double,
    mut max: libc::c_double,
    mut minex: libc::c_int,
    mut maxex: libc::c_int,
) -> libc::c_int {
    return zsetInitScoreRange(key, min, max, minex, maxex, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn zsetInitLexRange(
    mut key: *mut RedisModuleKey,
    mut min: *mut robj,
    mut max: *mut robj,
    mut first: libc::c_int,
) -> libc::c_int {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    RM_ZsetRangeStop(key);
    (*key).u.zset.er = 0 as libc::c_int;
    let mut zlrs: *mut zlexrangespec = &mut (*key).u.zset.lrs;
    if zslParseLexRange(min, max, zlrs) == -(1 as libc::c_int) {
        return 1 as libc::c_int;
    }
    (*key).u.zset.type_0 = 1 as libc::c_int as uint32_t;
    if (*(*key).value).encoding() as libc::c_int == 11 as libc::c_int {
        (*key)
            .u
            .zset
            .current = (if first != 0 {
            zzlFirstInLexRange((*(*key).value).ptr as *mut libc::c_uchar, zlrs)
        } else {
            zzlLastInLexRange((*(*key).value).ptr as *mut libc::c_uchar, zlrs)
        }) as *mut libc::c_void;
    } else if (*(*key).value).encoding() as libc::c_int == 7 as libc::c_int {
        let mut zs: *mut zset = (*(*key).value).ptr as *mut zset;
        let mut zsl: *mut zskiplist = (*zs).zsl;
        (*key)
            .u
            .zset
            .current = (if first != 0 {
            zslFirstInLexRange(zsl, zlrs)
        } else {
            zslLastInLexRange(zsl, zlrs)
        }) as *mut libc::c_void;
    } else {
        _serverPanic(
            b"module.c\0" as *const u8 as *const libc::c_char,
            4578 as libc::c_int,
            b"Unsupported zset encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    if ((*key).u.zset.current).is_null() {
        (*key).u.zset.er = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetFirstInLexRange(
    mut key: *mut RedisModuleKey,
    mut min: *mut robj,
    mut max: *mut robj,
) -> libc::c_int {
    return zsetInitLexRange(key, min, max, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetLastInLexRange(
    mut key: *mut RedisModuleKey,
    mut min: *mut robj,
    mut max: *mut robj,
) -> libc::c_int {
    return zsetInitLexRange(key, min, max, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRangeCurrentElement(
    mut key: *mut RedisModuleKey,
    mut score: *mut libc::c_double,
) -> *mut robj {
    let mut str: *mut robj = 0 as *mut robj;
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 0 as *mut robj;
    }
    if ((*key).u.zset.current).is_null() {
        return 0 as *mut robj;
    }
    if (*(*key).value).encoding() as libc::c_int == 11 as libc::c_int {
        let mut eptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut sptr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        eptr = (*key).u.zset.current as *mut libc::c_uchar;
        let mut ele: sds = lpGetObject(eptr);
        if !score.is_null() {
            sptr = lpNext((*(*key).value).ptr as *mut libc::c_uchar, eptr);
            *score = zzlGetScore(sptr);
        }
        str = createObject(0 as libc::c_int, ele as *mut libc::c_void);
    } else if (*(*key).value).encoding() as libc::c_int == 7 as libc::c_int {
        let mut ln: *mut zskiplistNode = (*key).u.zset.current as *mut zskiplistNode;
        if !score.is_null() {
            *score = (*ln).score;
        }
        str = createStringObject((*ln).ele as *const libc::c_char, sdslen((*ln).ele));
    } else {
        _serverPanic(
            b"module.c\0" as *const u8 as *const libc::c_char,
            4629 as libc::c_int,
            b"Unsupported zset encoding\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
    }
    autoMemoryAdd((*key).ctx, 1 as libc::c_int, str as *mut libc::c_void);
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRangeNext(mut key: *mut RedisModuleKey) -> libc::c_int {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*key).u.zset.type_0 == 0 || ((*key).u.zset.current).is_null() {
        return 0 as libc::c_int;
    }
    if (*(*key).value).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*(*key).value).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = (*key).u.zset.current as *mut libc::c_uchar;
        let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        next = lpNext(zl, eptr);
        if !next.is_null() {
            next = lpNext(zl, next);
        }
        if next.is_null() {
            (*key).u.zset.er = 1 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            if (*key).u.zset.type_0 == 2 as libc::c_int as libc::c_uint {
                let mut saved_next: *mut libc::c_uchar = next;
                next = lpNext(zl, next);
                let mut score: libc::c_double = zzlGetScore(next);
                if zslValueLteMax(score, &mut (*key).u.zset.rs) == 0 {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
                next = saved_next;
            } else if (*key).u.zset.type_0 == 1 as libc::c_int as libc::c_uint {
                if zzlLexValueLteMax(next, &mut (*key).u.zset.lrs) == 0 {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
            (*key).u.zset.current = next as *mut libc::c_void;
            return 1 as libc::c_int;
        }
    } else {
        if (*(*key).value).encoding() as libc::c_int == 7 as libc::c_int {
            let mut ln: *mut zskiplistNode = (*key).u.zset.current as *mut zskiplistNode;
            let mut next_0: *mut zskiplistNode = (*((*ln).level)
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .forward;
            if next_0.is_null() {
                (*key).u.zset.er = 1 as libc::c_int;
                return 0 as libc::c_int;
            } else {
                if (*key).u.zset.type_0 == 2 as libc::c_int as libc::c_uint
                    && zslValueLteMax((*next_0).score, &mut (*key).u.zset.rs) == 0
                {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                } else {
                    if (*key).u.zset.type_0 == 1 as libc::c_int as libc::c_uint {
                        if zslLexValueLteMax((*next_0).ele, &mut (*key).u.zset.lrs) == 0
                        {
                            (*key).u.zset.er = 1 as libc::c_int;
                            return 0 as libc::c_int;
                        }
                    }
                }
                (*key).u.zset.current = next_0 as *mut libc::c_void;
                return 1 as libc::c_int;
            }
        } else {
            _serverPanic(
                b"module.c\0" as *const u8 as *const libc::c_char,
                4695 as libc::c_int,
                b"Unsupported zset encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn RM_ZsetRangePrev(mut key: *mut RedisModuleKey) -> libc::c_int {
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 3 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*key).u.zset.type_0 == 0 || ((*key).u.zset.current).is_null() {
        return 0 as libc::c_int;
    }
    if (*(*key).value).encoding() as libc::c_int == 11 as libc::c_int {
        let mut zl: *mut libc::c_uchar = (*(*key).value).ptr as *mut libc::c_uchar;
        let mut eptr: *mut libc::c_uchar = (*key).u.zset.current as *mut libc::c_uchar;
        let mut prev: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        prev = lpPrev(zl, eptr);
        if !prev.is_null() {
            prev = lpPrev(zl, prev);
        }
        if prev.is_null() {
            (*key).u.zset.er = 1 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            if (*key).u.zset.type_0 == 2 as libc::c_int as libc::c_uint {
                let mut saved_prev: *mut libc::c_uchar = prev;
                prev = lpNext(zl, prev);
                let mut score: libc::c_double = zzlGetScore(prev);
                if zslValueGteMin(score, &mut (*key).u.zset.rs) == 0 {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
                prev = saved_prev;
            } else if (*key).u.zset.type_0 == 1 as libc::c_int as libc::c_uint {
                if zzlLexValueGteMin(prev, &mut (*key).u.zset.lrs) == 0 {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                }
            }
            (*key).u.zset.current = prev as *mut libc::c_void;
            return 1 as libc::c_int;
        }
    } else {
        if (*(*key).value).encoding() as libc::c_int == 7 as libc::c_int {
            let mut ln: *mut zskiplistNode = (*key).u.zset.current as *mut zskiplistNode;
            let mut prev_0: *mut zskiplistNode = (*ln).backward;
            if prev_0.is_null() {
                (*key).u.zset.er = 1 as libc::c_int;
                return 0 as libc::c_int;
            } else {
                if (*key).u.zset.type_0 == 2 as libc::c_int as libc::c_uint
                    && zslValueGteMin((*prev_0).score, &mut (*key).u.zset.rs) == 0
                {
                    (*key).u.zset.er = 1 as libc::c_int;
                    return 0 as libc::c_int;
                } else {
                    if (*key).u.zset.type_0 == 1 as libc::c_int as libc::c_uint {
                        if zslLexValueGteMin((*prev_0).ele, &mut (*key).u.zset.lrs) == 0
                        {
                            (*key).u.zset.er = 1 as libc::c_int;
                            return 0 as libc::c_int;
                        }
                    }
                }
                (*key).u.zset.current = prev_0 as *mut libc::c_void;
                return 1 as libc::c_int;
            }
        } else {
            _serverPanic(
                b"module.c\0" as *const u8 as *const libc::c_char,
                4759 as libc::c_int,
                b"Unsupported zset encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn RM_HashSet(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ap: core::ffi::VaListImpl;
    if key.is_null()
        || flags
            & !((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 2 as libc::c_int
                | (1 as libc::c_int) << 4 as libc::c_int) != 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    } else {
        if !((*key).value).is_null()
            && (*(*key).value).type_0() as libc::c_int != 4 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                *__errno_location() = 9 as libc::c_int;
                return 0 as libc::c_int;
            }
        }
    }
    if ((*key).value).is_null() {
        moduleCreateEmptyKey(key, 3 as libc::c_int);
    }
    let mut count: libc::c_int = 0 as libc::c_int;
    ap = args.clone();
    loop {
        let mut field: *mut robj = 0 as *mut robj;
        let mut value: *mut robj = 0 as *mut robj;
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            let mut cfield: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
            if cfield.is_null() {
                break;
            }
            field = createRawStringObject(cfield, strlen(cfield));
        } else {
            field = ap.arg::<*mut robj>();
            if field.is_null() {
                break;
            }
        }
        value = ap.arg::<*mut robj>();
        if flags
            & ((1 as libc::c_int) << 1 as libc::c_int
                | (1 as libc::c_int) << 0 as libc::c_int) != 0
        {
            let mut exists: libc::c_int = hashTypeExists(
                (*key).value,
                (*field).ptr as sds,
            );
            if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 && exists == 0
                || flags & (1 as libc::c_int) << 0 as libc::c_int != 0 && exists != 0
            {
                if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                    decrRefCount(field);
                }
                continue;
            }
        }
        if value == 1 as libc::c_int as libc::c_long as *mut robj {
            count += hashTypeDelete((*key).value, (*field).ptr as sds);
            if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                decrRefCount(field);
            }
        } else {
            let mut low_flags: libc::c_int = 0 as libc::c_int;
            if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                low_flags |= (1 as libc::c_int) << 0 as libc::c_int;
            }
            let mut argv: [*mut robj; 2] = [field, value];
            hashTypeTryConversion(
                (*key).value,
                argv.as_mut_ptr(),
                0 as libc::c_int,
                1 as libc::c_int,
            );
            let mut updated: libc::c_int = hashTypeSet(
                (*key).value,
                (*field).ptr as sds,
                (*value).ptr as sds,
                low_flags,
            );
            count
                += if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
                    1 as libc::c_int
                } else {
                    updated
                };
            if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
                (*field).ptr = 0 as *mut libc::c_void;
                decrRefCount(field);
            }
        }
    }
    moduleDelKeyIfEmpty(key);
    if count == 0 as libc::c_int {
        *__errno_location() = 2 as libc::c_int;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn RM_HashGet(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut args: ...
) -> libc::c_int {
    let mut ap: core::ffi::VaListImpl;
    if !((*key).value).is_null()
        && (*(*key).value).type_0() as libc::c_int != 4 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    ap = args.clone();
    loop {
        let mut field: *mut robj = 0 as *mut robj;
        let mut valueptr: *mut *mut robj = 0 as *mut *mut robj;
        let mut existsptr: *mut libc::c_int = 0 as *mut libc::c_int;
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            let mut cfield: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
            if cfield.is_null() {
                break;
            }
            field = createRawStringObject(cfield, strlen(cfield));
        } else {
            field = ap.arg::<*mut robj>();
            if field.is_null() {
                break;
            }
        }
        if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
            existsptr = ap.arg::<*mut libc::c_int>();
            if !((*key).value).is_null() {
                *existsptr = hashTypeExists((*key).value, (*field).ptr as sds);
            } else {
                *existsptr = 0 as libc::c_int;
            }
        } else {
            valueptr = ap.arg::<*mut *mut robj>();
            if !((*key).value).is_null() {
                *valueptr = hashTypeGetValueObject((*key).value, (*field).ptr as sds);
                if !(*valueptr).is_null() {
                    let mut decoded: *mut robj = getDecodedObject(*valueptr);
                    decrRefCount(*valueptr);
                    *valueptr = decoded;
                }
                if !(*valueptr).is_null() {
                    autoMemoryAdd(
                        (*key).ctx,
                        1 as libc::c_int,
                        *valueptr as *mut libc::c_void,
                    );
                }
            } else {
                *valueptr = 0 as *mut robj;
            }
        }
        if flags & (1 as libc::c_int) << 2 as libc::c_int != 0 {
            decrRefCount(field);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamAdd(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut id: *mut RedisModuleStreamID,
    mut argv: *mut *mut robj,
    mut numfields: libc::c_long,
) -> libc::c_int {
    if key.is_null() || numfields != 0 as libc::c_int as libc::c_long && argv.is_null()
        || flags & !((1 as libc::c_int) << 0 as libc::c_int) != 0
        || flags & (1 as libc::c_int) << 0 as libc::c_int == 0 && id.is_null()
    {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if !((*key).value).is_null()
            && (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            } else {
                if flags & (1 as libc::c_int) << 0 as libc::c_int == 0
                    && (*id).ms == 0 as libc::c_int as libc::c_ulong
                    && (*id).seq == 0 as libc::c_int as libc::c_ulong
                {
                    *__errno_location() = 33 as libc::c_int;
                    return 1 as libc::c_int;
                }
            }
        }
    }
    let mut created: libc::c_int = 0 as libc::c_int;
    if ((*key).value).is_null() {
        moduleCreateEmptyKey(key, 7 as libc::c_int);
        created = 1 as libc::c_int;
    }
    let mut s: *mut stream = (*(*key).value).ptr as *mut stream;
    if (*s).last_id.ms == 18446744073709551615 as libc::c_ulong
        && (*s).last_id.seq == 18446744073709551615 as libc::c_ulong
    {
        *__errno_location() = 27 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut added_id: streamID = streamID { ms: 0, seq: 0 };
    let mut use_id: streamID = streamID { ms: 0, seq: 0 };
    let mut use_id_ptr: *mut streamID = 0 as *mut streamID;
    if flags & (1 as libc::c_int) << 0 as libc::c_int == 0 {
        use_id.ms = (*id).ms;
        use_id.seq = (*id).seq;
        use_id_ptr = &mut use_id;
    }
    if streamAppendItem(s, argv, numfields, &mut added_id, use_id_ptr, 1 as libc::c_int)
        == -(1 as libc::c_int)
    {
        return 1 as libc::c_int;
    }
    if created == 0 {
        (*key).u.stream.signalready = 1 as libc::c_int;
    }
    if !id.is_null() {
        (*id).ms = added_id.ms;
        (*id).seq = added_id.seq;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamDelete(
    mut key: *mut RedisModuleKey,
    mut id: *mut RedisModuleStreamID,
) -> libc::c_int {
    if key.is_null() || id.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
                || !((*key).iter).is_null()
            {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    let mut s: *mut stream = (*(*key).value).ptr as *mut stream;
    let mut streamid: streamID = {
        let mut init = streamID {
            ms: (*id).ms,
            seq: (*id).seq,
        };
        init
    };
    if streamDeleteItem(s, &mut streamid) != 0 {
        return 0 as libc::c_int
    } else {
        *__errno_location() = 2 as libc::c_int;
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamIteratorStart(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut start: *mut RedisModuleStreamID,
    mut end: *mut RedisModuleStreamID,
) -> libc::c_int {
    if key.is_null()
        || flags
            & !((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int) != 0
    {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if !((*key).iter).is_null() {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    let mut lower: streamID = streamID { ms: 0, seq: 0 };
    let mut upper: streamID = streamID { ms: 0, seq: 0 };
    if !start.is_null() {
        lower = {
            let mut init = streamID {
                ms: (*start).ms,
                seq: (*start).seq,
            };
            init
        };
    }
    if !end.is_null() {
        upper = {
            let mut init = streamID {
                ms: (*end).ms,
                seq: (*end).seq,
            };
            init
        };
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        if !start.is_null() && streamIncrID(&mut lower) != 0 as libc::c_int
            || !end.is_null() && streamDecrID(&mut upper) != 0 as libc::c_int
        {
            *__errno_location() = 33 as libc::c_int;
            return 1 as libc::c_int;
        }
    }
    let mut s: *mut stream = (*(*key).value).ptr as *mut stream;
    let mut rev: libc::c_int = flags & (1 as libc::c_int) << 1 as libc::c_int;
    let mut si: *mut streamIterator = zmalloc(
        core::mem::size_of::<streamIterator>() as libc::c_ulong,
    ) as *mut streamIterator;
    streamIteratorStart(
        si,
        s,
        if !start.is_null() { &mut lower } else { 0 as *mut streamID },
        if !end.is_null() { &mut upper } else { 0 as *mut streamID },
        rev,
    );
    (*key).iter = si as *mut libc::c_void;
    (*key).u.stream.currentid.ms = 0 as libc::c_int as uint64_t;
    (*key).u.stream.currentid.seq = 0 as libc::c_int as uint64_t;
    (*key).u.stream.numfieldsleft = 0 as libc::c_int as int64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamIteratorStop(
    mut key: *mut RedisModuleKey,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if ((*key).iter).is_null() {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    streamIteratorStop((*key).iter as *mut streamIterator);
    zfree((*key).iter);
    (*key).iter = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamIteratorNextID(
    mut key: *mut RedisModuleKey,
    mut id: *mut RedisModuleStreamID,
    mut numfields: *mut libc::c_long,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if ((*key).iter).is_null() {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            }
        }
    }
    let mut si: *mut streamIterator = (*key).iter as *mut streamIterator;
    let mut num_ptr: *mut int64_t = &mut (*key).u.stream.numfieldsleft;
    let mut streamid_ptr: *mut streamID = &mut (*key).u.stream.currentid;
    if streamIteratorGetID(si, streamid_ptr, num_ptr) != 0 {
        if !id.is_null() {
            (*id).ms = (*streamid_ptr).ms;
            (*id).seq = (*streamid_ptr).seq;
        }
        if !numfields.is_null() {
            *numfields = *num_ptr;
        }
        return 0 as libc::c_int;
    } else {
        (*key).u.stream.currentid.ms = 0 as libc::c_int as uint64_t;
        (*key).u.stream.currentid.seq = 0 as libc::c_int as uint64_t;
        (*key).u.stream.numfieldsleft = 0 as libc::c_int as int64_t;
        *__errno_location() = 2 as libc::c_int;
        return 1 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamIteratorNextField(
    mut key: *mut RedisModuleKey,
    mut field_ptr: *mut *mut robj,
    mut value_ptr: *mut *mut robj,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if ((*key).iter).is_null() {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            } else {
                if (*key).u.stream.numfieldsleft <= 0 as libc::c_int as libc::c_long {
                    *__errno_location() = 2 as libc::c_int;
                    return 1 as libc::c_int;
                }
            }
        }
    }
    let mut si: *mut streamIterator = (*key).iter as *mut streamIterator;
    let mut field: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut value: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut field_len: int64_t = 0;
    let mut value_len: int64_t = 0;
    streamIteratorGetField(si, &mut field, &mut value, &mut field_len, &mut value_len);
    if !field_ptr.is_null() {
        *field_ptr = createRawStringObject(
            field as *mut libc::c_char,
            field_len as size_t,
        );
        autoMemoryAdd((*key).ctx, 1 as libc::c_int, *field_ptr as *mut libc::c_void);
    }
    if !value_ptr.is_null() {
        *value_ptr = createRawStringObject(
            value as *mut libc::c_char,
            value_len as size_t,
        );
        autoMemoryAdd((*key).ctx, 1 as libc::c_int, *value_ptr as *mut libc::c_void);
    }
    (*key).u.stream.numfieldsleft -= 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamIteratorDelete(
    mut key: *mut RedisModuleKey,
) -> libc::c_int {
    if key.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return 1 as libc::c_int;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
                || ((*key).iter).is_null()
            {
                *__errno_location() = 9 as libc::c_int;
                return 1 as libc::c_int;
            } else {
                if (*key).u.stream.currentid.ms == 0 as libc::c_int as libc::c_ulong
                    && (*key).u.stream.currentid.seq == 0 as libc::c_int as libc::c_ulong
                {
                    *__errno_location() = 2 as libc::c_int;
                    return 1 as libc::c_int;
                }
            }
        }
    }
    let mut si: *mut streamIterator = (*key).iter as *mut streamIterator;
    streamIteratorRemoveEntry(si, &mut (*key).u.stream.currentid);
    (*key).u.stream.currentid.ms = 0 as libc::c_int as uint64_t;
    (*key).u.stream.currentid.seq = 0 as libc::c_int as uint64_t;
    (*key).u.stream.numfieldsleft = 0 as libc::c_int as int64_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamTrimByLength(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut length: libc::c_longlong,
) -> libc::c_longlong {
    if key.is_null() || flags & !((1 as libc::c_int) << 0 as libc::c_int) != 0
        || length < 0 as libc::c_int as libc::c_longlong
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_longlong;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return -(1 as libc::c_int) as libc::c_longlong;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                *__errno_location() = 9 as libc::c_int;
                return -(1 as libc::c_int) as libc::c_longlong;
            }
        }
    }
    let mut approx: libc::c_int = if flags & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    return streamTrimByLength((*(*key).value).ptr as *mut stream, length, approx)
        as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StreamTrimByID(
    mut key: *mut RedisModuleKey,
    mut flags: libc::c_int,
    mut id: *mut RedisModuleStreamID,
) -> libc::c_longlong {
    if key.is_null() || flags & !((1 as libc::c_int) << 0 as libc::c_int) != 0
        || id.is_null()
    {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int) as libc::c_longlong;
    } else {
        if ((*key).value).is_null()
            || (*(*key).value).type_0() as libc::c_int != 6 as libc::c_int
        {
            *__errno_location() = 95 as libc::c_int;
            return -(1 as libc::c_int) as libc::c_longlong;
        } else {
            if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                *__errno_location() = 9 as libc::c_int;
                return -(1 as libc::c_int) as libc::c_longlong;
            }
        }
    }
    let mut approx: libc::c_int = if flags & (1 as libc::c_int) << 0 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    let mut minid: streamID = {
        let mut init = streamID {
            ms: (*id).ms,
            seq: (*id).seq,
        };
        init
    };
    return streamTrimByID((*(*key).value).ptr as *mut stream, minid, approx)
        as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeCallReply(mut reply: *mut RedisModuleCallReply) {
    let mut ctx: *mut RedisModuleCtx = callReplyGetPrivateData(reply)
        as *mut RedisModuleCtx;
    freeCallReply(reply);
    autoMemoryFreed(ctx, 2 as libc::c_int, reply as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyType(
    mut reply: *mut RedisModuleCallReply,
) -> libc::c_int {
    return callReplyType(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyLength(
    mut reply: *mut RedisModuleCallReply,
) -> size_t {
    return callReplyGetLen(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyArrayElement(
    mut reply: *mut RedisModuleCallReply,
    mut idx: size_t,
) -> *mut RedisModuleCallReply {
    return callReplyGetArrayElement(reply, idx);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyInteger(
    mut reply: *mut RedisModuleCallReply,
) -> libc::c_longlong {
    return callReplyGetLongLong(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyDouble(
    mut reply: *mut RedisModuleCallReply,
) -> libc::c_double {
    return callReplyGetDouble(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyBigNumber(
    mut reply: *mut RedisModuleCallReply,
    mut len: *mut size_t,
) -> *const libc::c_char {
    return callReplyGetBigNumber(reply, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyVerbatim(
    mut reply: *mut RedisModuleCallReply,
    mut len: *mut size_t,
    mut format: *mut *const libc::c_char,
) -> *const libc::c_char {
    return callReplyGetVerbatim(reply, len, format);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyBool(
    mut reply: *mut RedisModuleCallReply,
) -> libc::c_int {
    return callReplyGetBool(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplySetElement(
    mut reply: *mut RedisModuleCallReply,
    mut idx: size_t,
) -> *mut RedisModuleCallReply {
    return callReplyGetSetElement(reply, idx);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyMapElement(
    mut reply: *mut RedisModuleCallReply,
    mut idx: size_t,
    mut key: *mut *mut RedisModuleCallReply,
    mut val: *mut *mut RedisModuleCallReply,
) -> libc::c_int {
    if callReplyGetMapElement(reply, idx, key, val) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyAttribute(
    mut reply: *mut RedisModuleCallReply,
) -> *mut RedisModuleCallReply {
    return callReplyGetAttribute(reply);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyAttributeElement(
    mut reply: *mut RedisModuleCallReply,
    mut idx: size_t,
    mut key: *mut *mut RedisModuleCallReply,
    mut val: *mut *mut RedisModuleCallReply,
) -> libc::c_int {
    if callReplyGetAttributeElement(reply, idx, key, val) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyStringPtr(
    mut reply: *mut RedisModuleCallReply,
    mut len: *mut size_t,
) -> *const libc::c_char {
    let mut private_len: size_t = 0;
    if len.is_null() {
        len = &mut private_len;
    }
    return callReplyGetString(reply, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateStringFromCallReply(
    mut reply: *mut RedisModuleCallReply,
) -> *mut robj {
    let mut ctx: *mut RedisModuleCtx = callReplyGetPrivateData(reply)
        as *mut RedisModuleCtx;
    let mut len: size_t = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    match callReplyType(reply) {
        0 | 1 => {
            str = callReplyGetString(reply, &mut len);
            return RM_CreateString(ctx, str, len);
        }
        2 => {
            let mut buf: [libc::c_char; 64] = [0; 64];
            let mut len_0: libc::c_int = ll2string(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                callReplyGetLongLong(reply),
            );
            return RM_CreateString(ctx, buf.as_mut_ptr(), len_0 as size_t);
        }
        _ => return 0 as *mut robj,
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetContextUser(
    mut ctx: *mut RedisModuleCtx,
    mut user: *const RedisModuleUser,
) {
    (*ctx).user = user;
}
#[no_mangle]
pub unsafe extern "C" fn moduleCreateArgvFromUserFormat(
    mut cmdname: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut argcp: *mut libc::c_int,
    mut argvlenp: *mut libc::c_int,
    mut flags: *mut libc::c_int,
    mut ap: core::ffi::VaList,
) -> *mut *mut robj {
    let mut current_block: u64;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut argv_size: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut argv: *mut *mut robj = 0 as *mut *mut robj;
    argv_size = (strlen(fmt)).wrapping_add(1 as libc::c_int as libc::c_ulong)
        as libc::c_int;
    argv = zrealloc(
        argv as *mut libc::c_void,
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(argv_size as libc::c_ulong),
    ) as *mut *mut robj;
    let ref mut fresh19 = *argv.offset(0 as libc::c_int as isize);
    *fresh19 = createStringObject(cmdname, strlen(cmdname));
    argc += 1;
    let mut p: *const libc::c_char = fmt;
    loop {
        if !(*p != 0) {
            current_block = 4888910987971495881;
            break;
        }
        if *p as libc::c_int == 'c' as i32 {
            let mut cstr: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
            let fresh20 = argc;
            argc = argc + 1;
            let ref mut fresh21 = *argv.offset(fresh20 as isize);
            *fresh21 = createStringObject(cstr, strlen(cstr));
        } else if *p as libc::c_int == 's' as i32 {
            let mut obj: *mut robj = ap.arg::<*mut libc::c_void>() as *mut robj;
            if (*obj).refcount == 2147483647 as libc::c_int - 1 as libc::c_int {
                obj = createStringObject(
                    (*obj).ptr as *const libc::c_char,
                    sdslen((*obj).ptr as sds),
                );
            } else {
                incrRefCount(obj);
            }
            let fresh22 = argc;
            argc = argc + 1;
            let ref mut fresh23 = *argv.offset(fresh22 as isize);
            *fresh23 = obj;
        } else if *p as libc::c_int == 'b' as i32 {
            let mut buf: *mut libc::c_char = ap.arg::<*mut libc::c_char>();
            let mut len: size_t = ap.arg::<size_t>();
            let fresh24 = argc;
            argc = argc + 1;
            let ref mut fresh25 = *argv.offset(fresh24 as isize);
            *fresh25 = createStringObject(buf, len);
        } else if *p as libc::c_int == 'l' as i32 {
            let mut ll: libc::c_longlong = ap.arg::<libc::c_longlong>();
            let fresh26 = argc;
            argc = argc + 1;
            let ref mut fresh27 = *argv.offset(fresh26 as isize);
            *fresh27 = createObject(
                0 as libc::c_int,
                sdsfromlonglong(ll) as *mut libc::c_void,
            );
        } else if *p as libc::c_int == 'v' as i32 {
            let mut v: *mut *mut robj = ap.arg::<*mut libc::c_void>() as *mut *mut robj;
            let mut vlen: size_t = ap.arg::<size_t>();
            argv_size = (argv_size as libc::c_ulong)
                .wrapping_add(vlen.wrapping_sub(1 as libc::c_int as libc::c_ulong))
                as libc::c_int as libc::c_int;
            argv = zrealloc(
                argv as *mut libc::c_void,
                (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                    .wrapping_mul(argv_size as libc::c_ulong),
            ) as *mut *mut robj;
            let mut i: size_t = 0 as libc::c_int as size_t;
            i = 0 as libc::c_int as size_t;
            while i < vlen {
                incrRefCount(*v.offset(i as isize));
                let fresh28 = argc;
                argc = argc + 1;
                let ref mut fresh29 = *argv.offset(fresh28 as isize);
                *fresh29 = *v.offset(i as isize);
                i = i.wrapping_add(1);
            }
        } else if *p as libc::c_int == '!' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 0 as libc::c_int;
            }
        } else if *p as libc::c_int == 'A' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 1 as libc::c_int;
            }
        } else if *p as libc::c_int == 'R' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 2 as libc::c_int;
            }
        } else if *p as libc::c_int == '3' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 3 as libc::c_int;
            }
        } else if *p as libc::c_int == '0' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 4 as libc::c_int;
            }
        } else if *p as libc::c_int == 'C' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 5 as libc::c_int;
            }
        } else if *p as libc::c_int == 'S' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 6 as libc::c_int;
            }
        } else if *p as libc::c_int == 'W' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 7 as libc::c_int;
            }
        } else if *p as libc::c_int == 'M' as i32 {
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 9 as libc::c_int;
            }
        } else {
            if !(*p as libc::c_int == 'E' as i32) {
                current_block = 7895087294770463614;
                break;
            }
            if !flags.is_null() {
                *flags |= (1 as libc::c_int) << 8 as libc::c_int;
            }
        }
        p = p.offset(1);
    }
    match current_block {
        7895087294770463614 => {
            j = 0 as libc::c_int;
            while j < argc {
                decrRefCount(*argv.offset(j as isize));
                j += 1;
            }
            zfree(argv as *mut libc::c_void);
            return 0 as *mut *mut robj;
        }
        _ => {
            if !argcp.is_null() {
                *argcp = argc;
            }
            if !argvlenp.is_null() {
                *argvlenp = argv_size;
            }
            return argv;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_Call(
    mut ctx: *mut RedisModuleCtx,
    mut cmdname: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> *mut RedisModuleCallReply {
    let mut err: sds = 0 as *mut libc::c_char;
    let mut prev_replication_allowed: libc::c_int = 0;
    let mut call_flags: libc::c_int = 0;
    let mut proto: sds = 0 as *mut libc::c_char;
    let mut current_block: u64;
    let mut c: *mut client = 0 as *mut client;
    let mut argv: *mut *mut robj = 0 as *mut *mut robj;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut argv_len: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut ap: core::ffi::VaListImpl;
    let mut reply: *mut RedisModuleCallReply = 0 as *mut RedisModuleCallReply;
    let mut replicate: libc::c_int = 0 as libc::c_int;
    let mut error_as_call_replies: libc::c_int = 0 as libc::c_int;
    let mut cmd_flags: uint64_t = 0;
    ap = args.clone();
    argv = moduleCreateArgvFromUserFormat(
        cmdname,
        fmt,
        &mut argc,
        &mut argv_len,
        &mut flags,
        ap.as_va_list(),
    );
    replicate = flags & (1 as libc::c_int) << 0 as libc::c_int;
    error_as_call_replies = flags & (1 as libc::c_int) << 8 as libc::c_int;
    let mut user: *mut user = 0 as *mut user;
    if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
        user = if !((*ctx).user).is_null() {
            (*(*ctx).user).user
        } else {
            (*(*ctx).client).user
        };
        if user.is_null() {
            *__errno_location() = 95 as libc::c_int;
            if error_as_call_replies != 0 {
                let mut msg: sds = sdsnew(
                    b"cannot run as user, no user directly attached to context or context's client\0"
                        as *const u8 as *const libc::c_char,
                );
                reply = callReplyCreateError(msg, ctx as *mut libc::c_void);
            }
            return reply;
        }
    }
    c = moduleAllocTempClient(user);
    (*c)
        .flags = ((*c).flags as libc::c_ulonglong
        | (1 as libc::c_ulonglong) << 41 as libc::c_int) as uint64_t;
    (*c).db = (*(*ctx).client).db;
    (*c).argv = argv;
    (*c).argc = argc;
    (*c).argv_len = argv_len;
    (*c).resp = 2 as libc::c_int;
    if flags & (1 as libc::c_int) << 3 as libc::c_int != 0 {
        (*c).resp = 3 as libc::c_int;
    } else if flags & (1 as libc::c_int) << 4 as libc::c_int != 0 {
        (*c).resp = (*(*ctx).client).resp;
    }
    if !((*ctx).module).is_null() {
        (*(*ctx).module).in_call += 1;
    }
    if argv.is_null() {
        *__errno_location() = 9 as libc::c_int;
    } else {
        moduleCallCommandFilters(c);
        (*c).realcmd = lookupCommand((*c).argv, (*c).argc);
        (*c).lastcmd = (*c).realcmd;
        (*c).cmd = (*c).lastcmd;
        err = 0 as *mut libc::c_char;
        if commandCheckExistence(
            c,
            if error_as_call_replies != 0 { &mut err } else { 0 as *mut sds },
        ) == 0
        {
            *__errno_location() = 2 as libc::c_int;
            if error_as_call_replies != 0 {
                reply = callReplyCreateError(err, ctx as *mut libc::c_void);
            }
        } else if commandCheckArity(
            c,
            if error_as_call_replies != 0 { &mut err } else { 0 as *mut sds },
        ) == 0
        {
            *__errno_location() = 22 as libc::c_int;
            if error_as_call_replies != 0 {
                reply = callReplyCreateError(err, ctx as *mut libc::c_void);
            }
        } else {
            cmd_flags = getCommandFlags(c);
            if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                if cmd_flags as libc::c_ulonglong
                    & (1 as libc::c_ulonglong) << 6 as libc::c_int != 0
                {
                    *__errno_location() = 29 as libc::c_int;
                    if error_as_call_replies != 0 {
                        let mut msg_0: sds = sdscatfmt(
                            sdsempty(),
                            b"command '%S' is not allowed on script mode\0" as *const u8
                                as *const libc::c_char,
                            (*(*c).cmd).fullname,
                        );
                        reply = callReplyCreateError(msg_0, ctx as *mut libc::c_void);
                    }
                    current_block = 5642327860040497898;
                } else {
                    current_block = 15512526488502093901;
                }
            } else {
                current_block = 15512526488502093901;
            }
            match current_block {
                5642327860040497898 => {}
                _ => {
                    if flags & (1 as libc::c_int) << 9 as libc::c_int != 0 {
                        if cmd_flags as libc::c_ulonglong
                            & (1 as libc::c_ulonglong) << 2 as libc::c_int != 0
                        {
                            let mut oom_state: libc::c_int = 0;
                            if (*ctx).flags & (1 as libc::c_int) << 4 as libc::c_int != 0
                            {
                                oom_state = (getMaxmemoryState(
                                    0 as *mut size_t,
                                    0 as *mut size_t,
                                    0 as *mut size_t,
                                    0 as *mut libc::c_float,
                                ) == -(1 as libc::c_int)) as libc::c_int;
                            } else {
                                oom_state = server.pre_command_oom_state;
                            }
                            if oom_state != 0 {
                                *__errno_location() = 28 as libc::c_int;
                                if error_as_call_replies != 0 {
                                    let mut msg_1: sds = sdsdup((*shared.oomerr).ptr as sds);
                                    reply = callReplyCreateError(
                                        msg_1,
                                        ctx as *mut libc::c_void,
                                    );
                                }
                                current_block = 5642327860040497898;
                            } else {
                                current_block = 1854459640724737493;
                            }
                        } else {
                            current_block = 1854459640724737493;
                        }
                    } else {
                        current_block = 1854459640724737493;
                    }
                    match current_block {
                        5642327860040497898 => {}
                        _ => {
                            if flags & (1 as libc::c_int) << 7 as libc::c_int != 0 {
                                if cmd_flags as libc::c_ulonglong
                                    & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
                                {
                                    *__errno_location() = 28 as libc::c_int;
                                    if error_as_call_replies != 0 {
                                        let mut msg_2: sds = sdscatfmt(
                                            sdsempty(),
                                            b"Write command '%S' was called while write is not allowed.\0"
                                                as *const u8 as *const libc::c_char,
                                            (*(*c).cmd).fullname,
                                        );
                                        reply = callReplyCreateError(
                                            msg_2,
                                            ctx as *mut libc::c_void,
                                        );
                                    }
                                    current_block = 5642327860040497898;
                                } else {
                                    current_block = 8835654301469918283;
                                }
                            } else {
                                current_block = 8835654301469918283;
                            }
                            match current_block {
                                5642327860040497898 => {}
                                _ => {
                                    if flags & (1 as libc::c_int) << 6 as libc::c_int != 0 {
                                        if cmd_flags as libc::c_ulonglong
                                            & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
                                        {
                                            if checkGoodReplicasStatus() == 0 {
                                                *__errno_location() = 29 as libc::c_int;
                                                if error_as_call_replies != 0 {
                                                    let mut msg_3: sds = sdsdup(
                                                        (*shared.noreplicaserr).ptr as sds,
                                                    );
                                                    reply = callReplyCreateError(
                                                        msg_3,
                                                        ctx as *mut libc::c_void,
                                                    );
                                                }
                                                current_block = 5642327860040497898;
                                            } else {
                                                let mut deny_write_type: libc::c_int = writeCommandsDeniedByDiskError();
                                                let mut obey_client: libc::c_int = (!(server.current_client)
                                                    .is_null() && mustObeyClient(server.current_client) != 0)
                                                    as libc::c_int;
                                                if deny_write_type != 0 as libc::c_int && obey_client == 0 {
                                                    *__errno_location() = 29 as libc::c_int;
                                                    if error_as_call_replies != 0 {
                                                        let mut msg_4: sds = writeCommandsGetDiskErrorMessage(
                                                            deny_write_type,
                                                        );
                                                        reply = callReplyCreateError(
                                                            msg_4,
                                                            ctx as *mut libc::c_void,
                                                        );
                                                    }
                                                    current_block = 5642327860040497898;
                                                } else if !(server.masterhost).is_null()
                                                    && server.repl_slave_ro != 0 && obey_client == 0
                                                {
                                                    *__errno_location() = 29 as libc::c_int;
                                                    if error_as_call_replies != 0 {
                                                        let mut msg_5: sds = sdsdup(
                                                            (*shared.roslaveerr).ptr as sds,
                                                        );
                                                        reply = callReplyCreateError(
                                                            msg_5,
                                                            ctx as *mut libc::c_void,
                                                        );
                                                    }
                                                    current_block = 5642327860040497898;
                                                } else {
                                                    current_block = 15514718523126015390;
                                                }
                                            }
                                        } else {
                                            current_block = 15514718523126015390;
                                        }
                                        match current_block {
                                            5642327860040497898 => {}
                                            _ => {
                                                if !(server.masterhost).is_null()
                                                    && server.repl_state != REPL_STATE_CONNECTED as libc::c_int
                                                    && server.repl_serve_stale_data == 0 as libc::c_int
                                                    && cmd_flags as libc::c_ulonglong
                                                        & (1 as libc::c_ulonglong) << 10 as libc::c_int == 0
                                                {
                                                    *__errno_location() = 29 as libc::c_int;
                                                    if error_as_call_replies != 0 {
                                                        let mut msg_6: sds = sdsdup(
                                                            (*shared.masterdownerr).ptr as sds,
                                                        );
                                                        reply = callReplyCreateError(
                                                            msg_6,
                                                            ctx as *mut libc::c_void,
                                                        );
                                                    }
                                                    current_block = 5642327860040497898;
                                                } else {
                                                    current_block = 18325745679564279244;
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 18325745679564279244;
                                    }
                                    match current_block {
                                        5642327860040497898 => {}
                                        _ => {
                                            if flags & (1 as libc::c_int) << 5 as libc::c_int != 0 {
                                                let mut acl_errpos: libc::c_int = 0;
                                                let mut acl_retval: libc::c_int = 0;
                                                acl_retval = ACLCheckAllUserCommandPerm(
                                                    user,
                                                    (*c).cmd,
                                                    (*c).argv,
                                                    (*c).argc,
                                                    &mut acl_errpos,
                                                );
                                                if acl_retval != 0 as libc::c_int {
                                                    let mut object: sds = if acl_retval == 1 as libc::c_int {
                                                        sdsdup((*(*c).cmd).fullname)
                                                    } else {
                                                        sdsdup(
                                                            (**((*c).argv).offset(acl_errpos as isize)).ptr as sds,
                                                        )
                                                    };
                                                    addACLLogEntry(
                                                        (*ctx).client,
                                                        acl_retval,
                                                        3 as libc::c_int,
                                                        -(1 as libc::c_int),
                                                        (*(*(*ctx).client).user).name,
                                                        object,
                                                    );
                                                    if error_as_call_replies != 0 {
                                                        let mut msg_7: sds = sdscatfmt(
                                                            sdsempty(),
                                                            b"acl verification failed, %s.\0" as *const u8
                                                                as *const libc::c_char,
                                                            getAclErrorMessage(acl_retval),
                                                        );
                                                        reply = callReplyCreateError(
                                                            msg_7,
                                                            ctx as *mut libc::c_void,
                                                        );
                                                    }
                                                    *__errno_location() = 13 as libc::c_int;
                                                    current_block = 5642327860040497898;
                                                } else {
                                                    current_block = 5687667889785024198;
                                                }
                                            } else {
                                                current_block = 5687667889785024198;
                                            }
                                            match current_block {
                                                5642327860040497898 => {}
                                                _ => {
                                                    if server.cluster_enabled != 0
                                                        && mustObeyClient((*ctx).client) == 0
                                                    {
                                                        let mut error_code: libc::c_int = 0;
                                                        (*c).flags
                                                            &= !((1 as libc::c_int) << 17 as libc::c_int
                                                                | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong;
                                                        (*c).flags
                                                            |= (*(*ctx).client).flags
                                                                & ((1 as libc::c_int) << 17 as libc::c_int
                                                                    | (1 as libc::c_int) << 9 as libc::c_int) as libc::c_ulong;
                                                        if getNodeByQuery(
                                                            c,
                                                            (*c).cmd,
                                                            (*c).argv,
                                                            (*c).argc,
                                                            0 as *mut libc::c_int,
                                                            &mut error_code,
                                                        ) != (*server.cluster).myself
                                                        {
                                                            let mut msg_8: sds = 0 as sds;
                                                            if error_code == 7 as libc::c_int {
                                                                if error_as_call_replies != 0 {
                                                                    msg_8 = sdscatfmt(
                                                                        sdsempty(),
                                                                        b"Can not execute a write command '%S' while the cluster is down and readonly\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        (*(*c).cmd).fullname,
                                                                    );
                                                                }
                                                                *__errno_location() = 30 as libc::c_int;
                                                            } else if error_code == 5 as libc::c_int {
                                                                if error_as_call_replies != 0 {
                                                                    msg_8 = sdscatfmt(
                                                                        sdsempty(),
                                                                        b"Can not execute a command '%S' while the cluster is down\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                        (*(*c).cmd).fullname,
                                                                    );
                                                                }
                                                                *__errno_location() = 100 as libc::c_int;
                                                            } else {
                                                                if error_as_call_replies != 0 {
                                                                    msg_8 = sdsnew(
                                                                        b"Attempted to access a non local key in a cluster node\0"
                                                                            as *const u8 as *const libc::c_char,
                                                                    );
                                                                }
                                                                *__errno_location() = 1 as libc::c_int;
                                                            }
                                                            if !msg_8.is_null() {
                                                                reply = callReplyCreateError(
                                                                    msg_8,
                                                                    ctx as *mut libc::c_void,
                                                                );
                                                            }
                                                            current_block = 5642327860040497898;
                                                        } else {
                                                            current_block = 2522825242109451841;
                                                        }
                                                    } else {
                                                        current_block = 2522825242109451841;
                                                    }
                                                    match current_block {
                                                        5642327860040497898 => {}
                                                        _ => {
                                                            prev_replication_allowed = server.replication_allowed;
                                                            server
                                                                .replication_allowed = (replicate != 0
                                                                && server.replication_allowed != 0) as libc::c_int;
                                                            call_flags = (1 as libc::c_int) << 0 as libc::c_int
                                                                | (1 as libc::c_int) << 1 as libc::c_int
                                                                | (1 as libc::c_int) << 4 as libc::c_int;
                                                            if replicate != 0 {
                                                                if flags & (1 as libc::c_int) << 1 as libc::c_int == 0 {
                                                                    call_flags |= (1 as libc::c_int) << 2 as libc::c_int;
                                                                }
                                                                if flags & (1 as libc::c_int) << 2 as libc::c_int == 0 {
                                                                    call_flags |= (1 as libc::c_int) << 3 as libc::c_int;
                                                                }
                                                            }
                                                            call(c, call_flags);
                                                            server.replication_allowed = prev_replication_allowed;
                                                            if (*c).flags
                                                                & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong
                                                                == 0 as libc::c_int as libc::c_ulong
                                                            {} else {
                                                                _serverAssert(
                                                                    b"(c->flags & CLIENT_BLOCKED) == 0\0" as *const u8
                                                                        as *const libc::c_char,
                                                                    b"module.c\0" as *const u8 as *const libc::c_char,
                                                                    6050 as libc::c_int,
                                                                );
                                                                unreachable!();
                                                            };
                                                            proto = sdsnewlen(
                                                                (*c).buf as *const libc::c_void,
                                                                (*c).bufpos as size_t,
                                                            );
                                                            (*c).bufpos = 0 as libc::c_int;
                                                            while (*(*c).reply).len != 0 {
                                                                let mut o: *mut clientReplyBlock = (*(*(*c).reply).head)
                                                                    .value as *mut clientReplyBlock;
                                                                proto = sdscatlen(
                                                                    proto,
                                                                    ((*o).buf).as_mut_ptr() as *const libc::c_void,
                                                                    (*o).used,
                                                                );
                                                                listDelNode((*c).reply, (*(*c).reply).head);
                                                            }
                                                            reply = callReplyCreate(
                                                                proto,
                                                                (*c).deferred_reply_errors,
                                                                ctx as *mut libc::c_void,
                                                            );
                                                            (*c).deferred_reply_errors = 0 as *mut list;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !reply.is_null() {
        autoMemoryAdd(ctx, 2 as libc::c_int, reply as *mut libc::c_void);
    }
    if !((*ctx).module).is_null() {
        (*(*ctx).module).in_call -= 1;
    }
    moduleReleaseTempClient(c);
    return reply;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CallReplyProto(
    mut reply: *mut RedisModuleCallReply,
    mut len: *mut size_t,
) -> *const libc::c_char {
    return callReplyGetProto(reply, len);
}
#[no_mangle]
pub static mut ModuleTypeNameCharSet: *const libc::c_char = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_\0"
    as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn moduleTypeEncodeId(
    mut name: *const libc::c_char,
    mut encver: libc::c_int,
) -> uint64_t {
    let mut cset: *const libc::c_char = ModuleTypeNameCharSet;
    if strlen(name) != 9 as libc::c_int as libc::c_ulong {
        return 0 as libc::c_int as uint64_t;
    }
    if encver < 0 as libc::c_int || encver > 1023 as libc::c_int {
        return 0 as libc::c_int as uint64_t;
    }
    let mut id: uint64_t = 0 as libc::c_int as uint64_t;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        let mut p: *mut libc::c_char = strchr(
            cset,
            *name.offset(j as isize) as libc::c_int,
        );
        if p.is_null() {
            return 0 as libc::c_int as uint64_t;
        }
        let mut pos: libc::c_ulong = p.offset_from(cset) as libc::c_long
            as libc::c_ulong;
        id = id << 6 as libc::c_int | pos;
        j += 1;
    }
    id = id << 10 as libc::c_int | encver as libc::c_ulong;
    return id;
}
#[no_mangle]
pub unsafe extern "C" fn moduleTypeLookupModuleByName(
    mut name: *const libc::c_char,
) -> *mut moduleType {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind((*module).types, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut mt: *mut moduleType = (*ln).value as *mut moduleType;
            if memcmp(
                name as *const libc::c_void,
                ((*mt).name).as_mut_ptr() as *const libc::c_void,
                core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                dictReleaseIterator(di);
                return mt;
            }
        }
    }
    dictReleaseIterator(di);
    return 0 as *mut moduleType;
}
#[no_mangle]
pub unsafe extern "C" fn moduleTypeLookupModuleByID(
    mut id: uint64_t,
) -> *mut moduleType {
    static mut cache: [C2RustUnnamed_29; 3] = [C2RustUnnamed_29 {
        id: 0,
        mt: 0 as *const moduleType as *mut moduleType,
    }; 3];
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int && !(cache[j as usize].mt).is_null() {
        if cache[j as usize].id == id {
            return cache[j as usize].mt;
        }
        j += 1;
    }
    let mut mt: *mut moduleType = 0 as *mut moduleType;
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if !(!de.is_null() && mt.is_null()) {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind((*module).types, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut this_mt: *mut moduleType = (*ln).value as *mut moduleType;
            if !((*this_mt).id >> 10 as libc::c_int == id >> 10 as libc::c_int) {
                continue;
            }
            mt = this_mt;
            break;
        }
    }
    dictReleaseIterator(di);
    if !mt.is_null() && j < 3 as libc::c_int {
        cache[j as usize].id = id;
        cache[j as usize].mt = mt;
    }
    return mt;
}
#[no_mangle]
pub unsafe extern "C" fn moduleTypeNameByID(
    mut name: *mut libc::c_char,
    mut moduleid: uint64_t,
) {
    let mut cset: *const libc::c_char = ModuleTypeNameCharSet;
    *name.offset(9 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    let mut p: *mut libc::c_char = name.offset(8 as libc::c_int as isize);
    moduleid >>= 10 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 9 as libc::c_int {
        let fresh30 = p;
        p = p.offset(-1);
        *fresh30 = *cset
            .offset((moduleid & 63 as libc::c_int as libc::c_ulong) as isize);
        moduleid >>= 6 as libc::c_int;
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleTypeModuleName(
    mut mt: *mut moduleType,
) -> *const libc::c_char {
    if mt.is_null() || ((*mt).module).is_null() {
        return 0 as *const libc::c_char;
    }
    return (*(*mt).module).name;
}
#[no_mangle]
pub unsafe extern "C" fn moduleNameFromCommand(
    mut cmd: *mut redisCommand,
) -> *const libc::c_char {
    if (*cmd).proc_0
        == Some(RedisModuleCommandDispatcher as unsafe extern "C" fn(*mut client) -> ())
    {} else {
        _serverAssert(
            b"cmd->proc == RedisModuleCommandDispatcher\0" as *const u8
                as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            6229 as libc::c_int,
        );
        unreachable!();
    };
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    return (*(*cp).module).name;
}
#[no_mangle]
pub unsafe extern "C" fn moduleTypeDupOrReply(
    mut c: *mut client,
    mut fromkey: *mut robj,
    mut tokey: *mut robj,
    mut todb: libc::c_int,
    mut value: *mut robj,
) -> *mut robj {
    let mut mv: *mut moduleValue = (*value).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    if ((*mt).copy).is_none() && ((*mt).copy2).is_none() {
        addReplyError(
            c,
            b"not supported for this module key\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut robj;
    }
    let mut newval: *mut libc::c_void = 0 as *mut libc::c_void;
    if ((*mt).copy2).is_some() {
        let mut ctx: RedisModuleKeyOptCtx = {
            let mut init = RedisModuleKeyOptCtx {
                from_key: fromkey,
                to_key: tokey,
                from_dbid: (*(*c).db).id,
                to_dbid: todb,
            };
            init
        };
        newval = ((*mt).copy2)
            .expect("non-null function pointer")(&mut ctx, (*mv).value);
    } else {
        newval = ((*mt).copy)
            .expect("non-null function pointer")(fromkey, tokey, (*mv).value);
    }
    if newval.is_null() {
        addReplyError(
            c,
            b"module key failed to copy\0" as *const u8 as *const libc::c_char,
        );
        return 0 as *mut robj;
    }
    return createModuleObject(mt, newval);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateDataType(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut encver: libc::c_int,
    mut typemethods_ptr: *mut libc::c_void,
) -> *mut moduleType {
    let mut id: uint64_t = moduleTypeEncodeId(name, encver);
    if id == 0 as libc::c_int as libc::c_ulong {
        return 0 as *mut moduleType;
    }
    if !(moduleTypeLookupModuleByName(name)).is_null() {
        return 0 as *mut moduleType;
    }
    let mut typemethods_version: libc::c_long = *(typemethods_ptr as *mut libc::c_long)
        .offset(0 as libc::c_int as isize);
    if typemethods_version == 0 as libc::c_int as libc::c_long {
        return 0 as *mut moduleType;
    }
    let mut tms: *mut typemethods = typemethods_ptr as *mut typemethods;
    let mut mt: *mut moduleType = zcalloc(
        core::mem::size_of::<moduleType>() as libc::c_ulong,
    ) as *mut moduleType;
    (*mt).id = id;
    (*mt).module = (*ctx).module;
    (*mt).rdb_load = (*tms).rdb_load;
    (*mt).rdb_save = (*tms).rdb_save;
    (*mt).aof_rewrite = (*tms).aof_rewrite;
    (*mt).mem_usage = (*tms).mem_usage;
    (*mt).digest = (*tms).digest;
    (*mt).free = (*tms).free;
    if (*tms).version >= 2 as libc::c_int as libc::c_ulong {
        (*mt).aux_load = (*tms).v2.aux_load;
        (*mt).aux_save = (*tms).v2.aux_save;
        (*mt).aux_save_triggers = (*tms).v2.aux_save_triggers;
    }
    if (*tms).version >= 3 as libc::c_int as libc::c_ulong {
        (*mt).free_effort = (*tms).v3.free_effort;
        (*mt).unlink = (*tms).v3.unlink;
        (*mt).copy = (*tms).v3.copy;
        (*mt).defrag = (*tms).v3.defrag;
    }
    if (*tms).version >= 4 as libc::c_int as libc::c_ulong {
        (*mt).mem_usage2 = (*tms).v4.mem_usage2;
        (*mt).unlink2 = (*tms).v4.unlink2;
        (*mt).free_effort2 = (*tms).v4.free_effort2;
        (*mt).copy2 = (*tms).v4.copy2;
    }
    memcpy(
        ((*mt).name).as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
    );
    listAddNodeTail((*(*ctx).module).types, mt as *mut libc::c_void);
    return mt;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ModuleTypeSetValue(
    mut key: *mut RedisModuleKey,
    mut mt: *mut moduleType,
    mut value: *mut libc::c_void,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || !((*key).iter).is_null()
    {
        return 1 as libc::c_int;
    }
    RM_DeleteKey(key);
    let mut o: *mut robj = createModuleObject(mt, value);
    setKey((*(*key).ctx).client, (*key).db, (*key).key, o, 2 as libc::c_int);
    decrRefCount(o);
    (*key).value = o;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ModuleTypeGetType(
    mut key: *mut RedisModuleKey,
) -> *mut moduleType {
    if key.is_null() || ((*key).value).is_null() || RM_KeyType(key) != 6 as libc::c_int {
        return 0 as *mut moduleType;
    }
    let mut mv: *mut moduleValue = (*(*key).value).ptr as *mut moduleValue;
    return (*mv).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ModuleTypeGetValue(
    mut key: *mut RedisModuleKey,
) -> *mut libc::c_void {
    if key.is_null() || ((*key).value).is_null() || RM_KeyType(key) != 6 as libc::c_int {
        return 0 as *mut libc::c_void;
    }
    let mut mv: *mut moduleValue = (*(*key).value).ptr as *mut moduleValue;
    return (*mv).value;
}
#[no_mangle]
pub unsafe extern "C" fn moduleRDBLoadError(mut io: *mut RedisModuleIO) {
    if (*(*(*io).type_0).module).options & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        (*io).error = 1 as libc::c_int;
        return;
    }
    _serverPanic(
        b"module.c\0" as *const u8 as *const libc::c_char,
        6504 as libc::c_int,
        b"Error loading data from RDB (short read or EOF). Read performed by module '%s' about type '%s' after reading '%llu' bytes of a value for key named: '%s'.\0"
            as *const u8 as *const libc::c_char,
        (*(*(*io).type_0).module).name,
        ((*(*io).type_0).name).as_mut_ptr(),
        (*io).bytes as libc::c_ulonglong,
        (if !((*io).key).is_null() {
            (*(*io).key).ptr as *mut libc::c_char as *const libc::c_char
        } else {
            b"(null)\0" as *const u8 as *const libc::c_char
        }),
    );
    unreachable!();
}
#[no_mangle]
pub unsafe extern "C" fn moduleAllDatatypesHandleErrors() -> libc::c_int {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        if (*(*module).types).len != 0
            && (*module).options & (1 as libc::c_int) << 0 as libc::c_int == 0
        {
            dictReleaseIterator(di);
            return 0 as libc::c_int;
        }
    }
    dictReleaseIterator(di);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleAllModulesHandleReplAsyncLoad() -> libc::c_int {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        if (*module).options & (1 as libc::c_int) << 2 as libc::c_int == 0 {
            dictReleaseIterator(di);
            return 0 as libc::c_int;
        }
    }
    dictReleaseIterator(di);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsIOError(mut io: *mut RedisModuleIO) -> libc::c_int {
    return (*io).error;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveUnsigned(
    mut io: *mut RedisModuleIO,
    mut value: uint64_t,
) {
    if (*io).error != 0 {
        return;
    }
    let mut retval: libc::c_int = rdbSaveLen((*io).rio, 2 as libc::c_int as uint64_t);
    if !(retval == -(1 as libc::c_int)) {
        (*io)
            .bytes = ((*io).bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        retval = rdbSaveLen((*io).rio, value);
        if !(retval == -(1 as libc::c_int)) {
            (*io)
                .bytes = ((*io).bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
            return;
        }
    }
    (*io).error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadUnsigned(mut io: *mut RedisModuleIO) -> uint64_t {
    let mut value: uint64_t = 0;
    let mut retval: libc::c_int = 0;
    let mut current_block: u64;
    if (*io).error != 0 {
        return 0 as libc::c_int as uint64_t;
    }
    if (*io).ver == 2 as libc::c_int {
        let mut opcode: uint64_t = rdbLoadLen((*io).rio, 0 as *mut libc::c_int);
        if opcode != 2 as libc::c_int as libc::c_ulong {
            current_block = 15120159549690775694;
        } else {
            current_block = 17778012151635330486;
        }
    } else {
        current_block = 17778012151635330486;
    }
    match current_block {
        17778012151635330486 => {
            value = 0;
            retval = rdbLoadLenByRef((*io).rio, 0 as *mut libc::c_int, &mut value);
            if !(retval == -(1 as libc::c_int)) {
                return value;
            }
        }
        _ => {}
    }
    moduleRDBLoadError(io);
    return 0 as libc::c_int as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveSigned(mut io: *mut RedisModuleIO, mut value: int64_t) {
    let mut conv: C2RustUnnamed_25 = C2RustUnnamed_25 { u: 0 };
    conv.i = value;
    RM_SaveUnsigned(io, conv.u);
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadSigned(mut io: *mut RedisModuleIO) -> int64_t {
    let mut conv: C2RustUnnamed_24 = C2RustUnnamed_24 { u: 0 };
    conv.u = RM_LoadUnsigned(io);
    return conv.i;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveString(mut io: *mut RedisModuleIO, mut s: *mut robj) {
    if (*io).error != 0 {
        return;
    }
    let mut retval: ssize_t = rdbSaveLen((*io).rio, 5 as libc::c_int as uint64_t)
        as ssize_t;
    if !(retval == -(1 as libc::c_int) as libc::c_long) {
        (*io)
            .bytes = ((*io).bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        retval = rdbSaveStringObject((*io).rio, s);
        if !(retval == -(1 as libc::c_int) as libc::c_long) {
            (*io)
                .bytes = ((*io).bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
            return;
        }
    }
    (*io).error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveStringBuffer(
    mut io: *mut RedisModuleIO,
    mut str: *const libc::c_char,
    mut len: size_t,
) {
    if (*io).error != 0 {
        return;
    }
    let mut retval: ssize_t = rdbSaveLen((*io).rio, 5 as libc::c_int as uint64_t)
        as ssize_t;
    if !(retval == -(1 as libc::c_int) as libc::c_long) {
        (*io)
            .bytes = ((*io).bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        retval = rdbSaveRawString((*io).rio, str as *mut libc::c_uchar, len);
        if !(retval == -(1 as libc::c_int) as libc::c_long) {
            (*io)
                .bytes = ((*io).bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
            return;
        }
    }
    (*io).error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleLoadString(
    mut io: *mut RedisModuleIO,
    mut plain: libc::c_int,
    mut lenptr: *mut size_t,
) -> *mut libc::c_void {
    let mut s: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut current_block: u64;
    if (*io).error != 0 {
        return 0 as *mut libc::c_void;
    }
    if (*io).ver == 2 as libc::c_int {
        let mut opcode: uint64_t = rdbLoadLen((*io).rio, 0 as *mut libc::c_int);
        if opcode != 5 as libc::c_int as libc::c_ulong {
            current_block = 11094523046203751053;
        } else {
            current_block = 17778012151635330486;
        }
    } else {
        current_block = 17778012151635330486;
    }
    match current_block {
        17778012151635330486 => {
            s = rdbGenericLoadStringObject(
                (*io).rio,
                if plain != 0 {
                    (1 as libc::c_int) << 1 as libc::c_int
                } else {
                    0 as libc::c_int
                },
                lenptr,
            );
            if !s.is_null() {
                return s;
            }
        }
        _ => {}
    }
    moduleRDBLoadError(io);
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadString(mut io: *mut RedisModuleIO) -> *mut robj {
    return moduleLoadString(io, 0 as libc::c_int, 0 as *mut size_t) as *mut robj;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadStringBuffer(
    mut io: *mut RedisModuleIO,
    mut lenptr: *mut size_t,
) -> *mut libc::c_char {
    return moduleLoadString(io, 1 as libc::c_int, lenptr) as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveDouble(
    mut io: *mut RedisModuleIO,
    mut value: libc::c_double,
) {
    if (*io).error != 0 {
        return;
    }
    let mut retval: libc::c_int = rdbSaveLen((*io).rio, 4 as libc::c_int as uint64_t);
    if !(retval == -(1 as libc::c_int)) {
        (*io)
            .bytes = ((*io).bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        retval = rdbSaveBinaryDoubleValue((*io).rio, value);
        if !(retval == -(1 as libc::c_int)) {
            (*io)
                .bytes = ((*io).bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
            return;
        }
    }
    (*io).error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadDouble(mut io: *mut RedisModuleIO) -> libc::c_double {
    let mut value: libc::c_double = 0.;
    let mut retval: libc::c_int = 0;
    let mut current_block: u64;
    if (*io).error != 0 {
        return 0 as libc::c_int as libc::c_double;
    }
    if (*io).ver == 2 as libc::c_int {
        let mut opcode: uint64_t = rdbLoadLen((*io).rio, 0 as *mut libc::c_int);
        if opcode != 4 as libc::c_int as libc::c_ulong {
            current_block = 16848152805145497277;
        } else {
            current_block = 17778012151635330486;
        }
    } else {
        current_block = 17778012151635330486;
    }
    match current_block {
        17778012151635330486 => {
            value = 0.;
            retval = rdbLoadBinaryDoubleValue((*io).rio, &mut value);
            if !(retval == -(1 as libc::c_int)) {
                return value;
            }
        }
        _ => {}
    }
    moduleRDBLoadError(io);
    return 0 as libc::c_int as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveFloat(
    mut io: *mut RedisModuleIO,
    mut value: libc::c_float,
) {
    if (*io).error != 0 {
        return;
    }
    let mut retval: libc::c_int = rdbSaveLen((*io).rio, 3 as libc::c_int as uint64_t);
    if !(retval == -(1 as libc::c_int)) {
        (*io)
            .bytes = ((*io).bytes as libc::c_ulong).wrapping_add(retval as libc::c_ulong)
            as size_t as size_t;
        retval = rdbSaveBinaryFloatValue((*io).rio, value);
        if !(retval == -(1 as libc::c_int)) {
            (*io)
                .bytes = ((*io).bytes as libc::c_ulong)
                .wrapping_add(retval as libc::c_ulong) as size_t as size_t;
            return;
        }
    }
    (*io).error = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadFloat(mut io: *mut RedisModuleIO) -> libc::c_float {
    let mut value: libc::c_float = 0.;
    let mut retval: libc::c_int = 0;
    let mut current_block: u64;
    if (*io).error != 0 {
        return 0 as libc::c_int as libc::c_float;
    }
    if (*io).ver == 2 as libc::c_int {
        let mut opcode: uint64_t = rdbLoadLen((*io).rio, 0 as *mut libc::c_int);
        if opcode != 3 as libc::c_int as libc::c_ulong {
            current_block = 2548736969172793501;
        } else {
            current_block = 17778012151635330486;
        }
    } else {
        current_block = 17778012151635330486;
    }
    match current_block {
        17778012151635330486 => {
            value = 0.;
            retval = rdbLoadBinaryFloatValue((*io).rio, &mut value);
            if !(retval == -(1 as libc::c_int)) {
                return value;
            }
        }
        _ => {}
    }
    moduleRDBLoadError(io);
    return 0 as libc::c_int as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveLongDouble(
    mut io: *mut RedisModuleIO,
    mut value: f64,
) {
    if (*io).error != 0 {
        return;
    }
    let mut buf: [libc::c_char; 5120] = [0; 5120];
    let mut len: size_t = ld2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 5120]>() as libc::c_ulong,
        value,
        LD_STR_HEX,
    ) as size_t;
    RM_SaveStringBuffer(io, buf.as_mut_ptr(), len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadLongDouble(mut io: *mut RedisModuleIO) -> f64 {
    if (*io).error != 0 {
        return (0 as libc::c_int) as f64;
    }
    let mut value: f64 = 0 as f64;
    let mut len: size_t = 0;
    let mut str: *mut libc::c_char = RM_LoadStringBuffer(io, &mut len);
    if str.is_null() {
        return (0 as libc::c_int) as f64;
    }
    string2ld(str, len, &mut value);
    RM_Free(str as *mut libc::c_void);
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn rdbSaveModulesAux(
    mut rdb: *mut rio,
    mut when: libc::c_int,
) -> ssize_t {
    let mut total_written: size_t = 0 as libc::c_int as size_t;
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind((*module).types, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut mt: *mut moduleType = (*ln).value as *mut moduleType;
            if ((*mt).aux_save).is_none() || (*mt).aux_save_triggers & when == 0 {
                continue;
            }
            let mut ret: ssize_t = rdbSaveSingleModuleAux(rdb, when, mt);
            if ret == -(1 as libc::c_int) as libc::c_long {
                dictReleaseIterator(di);
                return -(1 as libc::c_int) as ssize_t;
            }
            total_written = (total_written as libc::c_ulong)
                .wrapping_add(ret as libc::c_ulong) as size_t as size_t;
        }
    }
    dictReleaseIterator(di);
    return total_written as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DigestAddStringBuffer(
    mut md: *mut RedisModuleDigest,
    mut ele: *const libc::c_char,
    mut len: size_t,
) {
    mixDigest(((*md).o).as_mut_ptr(), ele as *const libc::c_void, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DigestAddLongLong(
    mut md: *mut RedisModuleDigest,
    mut ll: libc::c_longlong,
) {
    let mut buf: [libc::c_char; 21] = [0; 21];
    let mut len: size_t = ll2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong,
        ll,
    ) as size_t;
    mixDigest(((*md).o).as_mut_ptr(), buf.as_mut_ptr() as *const libc::c_void, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DigestEndSequence(mut md: *mut RedisModuleDigest) {
    xorDigest(
        ((*md).x).as_mut_ptr(),
        ((*md).o).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
    );
    memset(
        ((*md).o).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_uchar; 20]>() as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadDataTypeFromStringEncver(
    mut str: *const robj,
    mut mt: *const moduleType,
    mut encver: libc::c_int,
) -> *mut libc::c_void {
    let mut payload: rio = rio {
        read: None,
        write: None,
        tell: None,
        flush: None,
        update_cksum: None,
        cksum: 0,
        flags: 0,
        processed_bytes: 0,
        max_processing_chunk: 0,
        io: C2RustUnnamed {
            buffer: C2RustUnnamed_3 {
                ptr: 0 as *mut libc::c_char,
                pos: 0,
            },
        },
    };
    let mut io: RedisModuleIO = RedisModuleIO {
        bytes: 0,
        rio: 0 as *mut rio,
        type_0: 0 as *mut moduleType,
        error: 0,
        ver: 0,
        ctx: 0 as *mut RedisModuleCtx,
        key: 0 as *mut redisObject,
        dbid: 0,
    };
    let mut ret: *mut libc::c_void = 0 as *mut libc::c_void;
    rioInitWithBuffer(&mut payload, (*str).ptr as sds);
    io.rio = &mut payload;
    io.type_0 = mt as *mut moduleType;
    io.bytes = 0 as libc::c_int as size_t;
    io.error = 0 as libc::c_int;
    io.ver = 0 as libc::c_int;
    io.key = 0 as *mut redisObject;
    io.dbid = -(1 as libc::c_int);
    io.ctx = 0 as *mut RedisModuleCtx;
    io.ver = 2 as libc::c_int;
    ret = ((*mt).rdb_load).expect("non-null function pointer")(&mut io, encver);
    if !(io.ctx).is_null() {
        moduleFreeContext(io.ctx);
        zfree(io.ctx as *mut libc::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadDataTypeFromString(
    mut str: *const robj,
    mut mt: *const moduleType,
) -> *mut libc::c_void {
    return RM_LoadDataTypeFromStringEncver(str, mt, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SaveDataTypeToString(
    mut ctx: *mut RedisModuleCtx,
    mut data: *mut libc::c_void,
    mut mt: *const moduleType,
) -> *mut robj {
    let mut payload: rio = rio {
        read: None,
        write: None,
        tell: None,
        flush: None,
        update_cksum: None,
        cksum: 0,
        flags: 0,
        processed_bytes: 0,
        max_processing_chunk: 0,
        io: C2RustUnnamed {
            buffer: C2RustUnnamed_3 {
                ptr: 0 as *mut libc::c_char,
                pos: 0,
            },
        },
    };
    let mut io: RedisModuleIO = RedisModuleIO {
        bytes: 0,
        rio: 0 as *mut rio,
        type_0: 0 as *mut moduleType,
        error: 0,
        ver: 0,
        ctx: 0 as *mut RedisModuleCtx,
        key: 0 as *mut redisObject,
        dbid: 0,
    };
    rioInitWithBuffer(&mut payload, sdsempty());
    io.rio = &mut payload;
    io.type_0 = mt as *mut moduleType;
    io.bytes = 0 as libc::c_int as size_t;
    io.error = 0 as libc::c_int;
    io.ver = 0 as libc::c_int;
    io.key = 0 as *mut redisObject;
    io.dbid = -(1 as libc::c_int);
    io.ctx = 0 as *mut RedisModuleCtx;
    ((*mt).rdb_save).expect("non-null function pointer")(&mut io, data);
    if !(io.ctx).is_null() {
        moduleFreeContext(io.ctx);
        zfree(io.ctx as *mut libc::c_void);
    }
    if io.error != 0 {
        return 0 as *mut robj
    } else {
        let mut str: *mut robj = createObject(
            0 as libc::c_int,
            payload.io.buffer.ptr as *mut libc::c_void,
        );
        if !ctx.is_null() {
            autoMemoryAdd(ctx, 1 as libc::c_int, str as *mut libc::c_void);
        }
        return str;
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyNameFromDigest(
    mut dig: *mut RedisModuleDigest,
) -> *const robj {
    return (*dig).key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDbIdFromDigest(
    mut dig: *mut RedisModuleDigest,
) -> libc::c_int {
    return (*dig).dbid;
}
#[no_mangle]
pub unsafe extern "C" fn RM_EmitAOF(
    mut io: *mut RedisModuleIO,
    mut cmdname: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    if (*io).error != 0 {
        return;
    }
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut argv: *mut *mut robj = 0 as *mut *mut robj;
    let mut argc: libc::c_int = 0 as libc::c_int;
    let mut flags: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0;
    let mut ap: core::ffi::VaListImpl;
    cmd = lookupCommandByCString(cmdname as *mut libc::c_char);
    if cmd.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fatal: AOF method for module data type '%s' tried to emit unknown command '%s'\0"
                    as *const u8 as *const libc::c_char,
                ((*(*io).type_0).name).as_mut_ptr(),
                cmdname,
            );
        }
        (*io).error = 1 as libc::c_int;
        *__errno_location() = 22 as libc::c_int;
        return;
    }
    ap = args.clone();
    argv = moduleCreateArgvFromUserFormat(
        cmdname,
        fmt,
        &mut argc,
        0 as *mut libc::c_int,
        &mut flags,
        ap.as_va_list(),
    );
    if argv.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Fatal: AOF method for module data type '%s' tried to call RedisModule_EmitAOF() with wrong format specifiers '%s'\0"
                    as *const u8 as *const libc::c_char,
                ((*(*io).type_0).name).as_mut_ptr(),
                fmt,
            );
        }
        (*io).error = 1 as libc::c_int;
        *__errno_location() = 22 as libc::c_int;
        return;
    }
    if (*io).error == 0
        && rioWriteBulkCount((*io).rio, '*' as i32 as libc::c_char, argc as libc::c_long)
            == 0 as libc::c_int as libc::c_ulong
    {
        (*io).error = 1 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < argc {
        if (*io).error == 0
            && rioWriteBulkObject((*io).rio, *argv.offset(j as isize))
                == 0 as libc::c_int
        {
            (*io).error = 1 as libc::c_int;
        }
        decrRefCount(*argv.offset(j as isize));
        j += 1;
    }
    zfree(argv as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetContextFromIO(
    mut io: *mut RedisModuleIO,
) -> *mut RedisModuleCtx {
    if !((*io).ctx).is_null() {
        return (*io).ctx;
    }
    (*io)
        .ctx = zmalloc(core::mem::size_of::<RedisModuleCtx>() as libc::c_ulong)
        as *mut RedisModuleCtx;
    moduleCreateContext((*io).ctx, (*(*io).type_0).module, 0 as libc::c_int);
    return (*io).ctx;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyNameFromIO(mut io: *mut RedisModuleIO) -> *const robj {
    return (*io).key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyNameFromModuleKey(
    mut key: *mut RedisModuleKey,
) -> *const robj {
    return if !key.is_null() { (*key).key } else { 0 as *mut robj };
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDbIdFromModuleKey(
    mut key: *mut RedisModuleKey,
) -> libc::c_int {
    return if !key.is_null() { (*(*key).db).id } else { -(1 as libc::c_int) };
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDbIdFromIO(mut io: *mut RedisModuleIO) -> libc::c_int {
    return (*io).dbid;
}
#[no_mangle]
pub unsafe extern "C" fn moduleLogRaw(
    mut module: *mut RedisModule,
    mut levelstr: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut ap: core::ffi::VaList,
) {
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    let mut name_len: size_t = 0;
    let mut level: libc::c_int = 0;
    if strcasecmp(levelstr, b"debug\0" as *const u8 as *const libc::c_char) == 0 {
        level = 0 as libc::c_int;
    } else if strcasecmp(levelstr, b"verbose\0" as *const u8 as *const libc::c_char) == 0
    {
        level = 1 as libc::c_int;
    } else if strcasecmp(levelstr, b"notice\0" as *const u8 as *const libc::c_char) == 0
    {
        level = 2 as libc::c_int;
    } else if strcasecmp(levelstr, b"warning\0" as *const u8 as *const libc::c_char) == 0
    {
        level = 3 as libc::c_int;
    } else {
        level = 1 as libc::c_int;
    }
    if level < server.verbosity {
        return;
    }
    name_len = snprintf(
        msg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        b"<%s> \0" as *const u8 as *const libc::c_char,
        if !module.is_null() {
            (*module).name as *const libc::c_char
        } else {
            b"module\0" as *const u8 as *const libc::c_char
        },
    ) as size_t;
    vsnprintf(
        msg.as_mut_ptr().offset(name_len as isize),
        (core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong)
            .wrapping_sub(name_len),
        fmt,
        ap.as_va_list(),
    );
    serverLogRaw(level, msg.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn RM_Log(
    mut ctx: *mut RedisModuleCtx,
    mut levelstr: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    moduleLogRaw(
        if !ctx.is_null() { (*ctx).module } else { 0 as *mut RedisModule },
        levelstr,
        fmt,
        ap.as_va_list(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_LogIOError(
    mut io: *mut RedisModuleIO,
    mut levelstr: *const libc::c_char,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    moduleLogRaw((*(*io).type_0).module, levelstr, fmt, ap.as_va_list());
}
#[no_mangle]
pub unsafe extern "C" fn RM__Assert(
    mut estr: *const libc::c_char,
    mut file: *const libc::c_char,
    mut line: libc::c_int,
) {
    _serverAssert(estr, file, line);
}
#[no_mangle]
pub unsafe extern "C" fn RM_LatencyAddSample(
    mut event: *const libc::c_char,
    mut latency: mstime_t,
) {
    if latency >= server.latency_monitor_threshold {
        latencyAddSample(event, latency);
    }
}
#[no_mangle]
pub unsafe extern "C" fn unblockClientFromModule(mut c: *mut client) {
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    if ((*bc).disconnect_callback).is_some() {
        let mut ctx: RedisModuleCtx = RedisModuleCtx {
            getapifuncptr: 0 as *mut libc::c_void,
            module: 0 as *mut RedisModule,
            client: 0 as *mut client,
            blocked_client: 0 as *mut RedisModuleBlockedClient,
            amqueue: 0 as *mut AutoMemEntry,
            amqueue_len: 0,
            amqueue_used: 0,
            flags: 0,
            postponed_arrays: 0 as *mut *mut libc::c_void,
            postponed_arrays_count: 0,
            blocked_privdata: 0 as *mut libc::c_void,
            blocked_ready_key: 0 as *mut robj,
            keys_result: 0 as *mut getKeysResult,
            pa_head: 0 as *mut RedisModulePoolAllocBlock,
            next_yield_time: 0,
            user: 0 as *const RedisModuleUser,
        };
        moduleCreateContext(&mut ctx, (*bc).module, 0 as libc::c_int);
        ctx.blocked_privdata = (*bc).privdata;
        ctx.client = (*bc).client;
        ((*bc).disconnect_callback).expect("non-null function pointer")(&mut ctx, bc);
        moduleFreeContext(&mut ctx);
    }
    if (*bc).blocked_on_keys != 0 && (*bc).unblocked == 0 {
        moduleUnblockClient(c);
    }
    (*bc).client = 0 as *mut client;
}
#[no_mangle]
pub unsafe extern "C" fn moduleBlockClient(
    mut ctx: *mut RedisModuleCtx,
    mut reply_callback: RedisModuleCmdFunc,
    mut timeout_callback: RedisModuleCmdFunc,
    mut free_privdata: Option::<
        unsafe extern "C" fn(*mut RedisModuleCtx, *mut libc::c_void) -> (),
    >,
    mut timeout_ms: libc::c_longlong,
    mut keys: *mut *mut robj,
    mut numkeys: libc::c_int,
    mut privdata: *mut libc::c_void,
) -> *mut RedisModuleBlockedClient {
    let mut c: *mut client = (*ctx).client;
    let mut islua: libc::c_int = scriptIsRunning();
    let mut ismulti: libc::c_int = server.in_exec;
    (*c)
        .bpop
        .module_blocked_handle = zmalloc(
        core::mem::size_of::<RedisModuleBlockedClient>() as libc::c_ulong,
    );
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    (*(*ctx).module).blocked_clients += 1;
    let mut timeout: mstime_t = if timeout_ms != 0 {
        mstime() + timeout_ms
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    (*bc).client = if islua != 0 || ismulti != 0 { 0 as *mut client } else { c };
    (*bc).module = (*ctx).module;
    (*bc).reply_callback = reply_callback;
    (*bc).timeout_callback = timeout_callback;
    (*bc).disconnect_callback = None;
    (*bc).free_privdata = free_privdata;
    (*bc).privdata = privdata;
    (*bc).reply_client = moduleAllocTempClient(0 as *mut user);
    (*bc).thread_safe_ctx_client = moduleAllocTempClient(0 as *mut user);
    if !((*bc).client).is_null() {
        (*(*bc).reply_client).resp = (*(*bc).client).resp;
    }
    (*bc).dbid = (*(*c).db).id;
    (*bc)
        .blocked_on_keys = (keys != 0 as *mut libc::c_void as *mut *mut robj)
        as libc::c_int;
    (*bc).unblocked = 0 as libc::c_int;
    (*bc).background_timer = 0 as libc::c_int as monotime;
    (*bc).background_duration = 0 as libc::c_int as uint64_t;
    (*c).bpop.timeout = timeout;
    if islua != 0 || ismulti != 0 {
        (*c).bpop.module_blocked_handle = 0 as *mut libc::c_void;
        addReplyError(
            c,
            if islua != 0 {
                b"Blocking module command called from Lua script\0" as *const u8
                    as *const libc::c_char
            } else {
                b"Blocking module command called from transaction\0" as *const u8
                    as *const libc::c_char
            },
        );
    } else if !keys.is_null() {
        blockForKeys(
            c,
            3 as libc::c_int,
            keys,
            numkeys,
            -(1 as libc::c_int) as libc::c_long,
            timeout,
            0 as *mut robj,
            0 as *mut blockPos,
            0 as *mut streamID,
        );
    } else {
        blockClient(c, 3 as libc::c_int);
    }
    return bc;
}
#[no_mangle]
pub unsafe extern "C" fn moduleTryServeClientBlockedOnKey(
    mut c: *mut client,
    mut key: *mut robj,
) -> libc::c_int {
    let mut served: libc::c_int = 0 as libc::c_int;
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    if (*bc).unblocked != 0 {
        return 0 as libc::c_int;
    }
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(&mut ctx, (*bc).module, (1 as libc::c_int) << 2 as libc::c_int);
    ctx.blocked_ready_key = key;
    ctx.blocked_privdata = (*bc).privdata;
    ctx.client = (*bc).client;
    ctx.blocked_client = bc;
    if ((*bc).reply_callback)
        .expect(
            "non-null function pointer",
        )(&mut ctx, (*c).argv as *mut *mut libc::c_void, (*c).argc) == 0 as libc::c_int
    {
        served = 1 as libc::c_int;
    }
    moduleFreeContext(&mut ctx);
    return served;
}
#[no_mangle]
pub unsafe extern "C" fn RM_BlockClient(
    mut ctx: *mut RedisModuleCtx,
    mut reply_callback: RedisModuleCmdFunc,
    mut timeout_callback: RedisModuleCmdFunc,
    mut free_privdata: Option::<
        unsafe extern "C" fn(*mut RedisModuleCtx, *mut libc::c_void) -> (),
    >,
    mut timeout_ms: libc::c_longlong,
) -> *mut RedisModuleBlockedClient {
    return moduleBlockClient(
        ctx,
        reply_callback,
        timeout_callback,
        free_privdata,
        timeout_ms,
        0 as *mut *mut robj,
        0 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_BlockClientOnKeys(
    mut ctx: *mut RedisModuleCtx,
    mut reply_callback: RedisModuleCmdFunc,
    mut timeout_callback: RedisModuleCmdFunc,
    mut free_privdata: Option::<
        unsafe extern "C" fn(*mut RedisModuleCtx, *mut libc::c_void) -> (),
    >,
    mut timeout_ms: libc::c_longlong,
    mut keys: *mut *mut robj,
    mut numkeys: libc::c_int,
    mut privdata: *mut libc::c_void,
) -> *mut RedisModuleBlockedClient {
    return moduleBlockClient(
        ctx,
        reply_callback,
        timeout_callback,
        free_privdata,
        timeout_ms,
        keys,
        numkeys,
        privdata,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_SignalKeyAsReady(
    mut ctx: *mut RedisModuleCtx,
    mut key: *mut robj,
) {
    signalKeyAsReady((*(*ctx).client).db, key, 5 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnblockClientByHandle(
    mut bc: *mut RedisModuleBlockedClient,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    pthread_mutex_lock(&mut moduleUnblockedClientsMutex);
    if (*bc).blocked_on_keys == 0 {
        (*bc).privdata = privdata;
    }
    (*bc).unblocked = 1 as libc::c_int;
    if (*moduleUnblockedClients).len == 0 as libc::c_int as libc::c_ulong {
        write(
            server.module_pipe[1 as libc::c_int as usize],
            b"A\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) != 1 as libc::c_int as libc::c_long;
    }
    listAddNodeTail(moduleUnblockedClients, bc as *mut libc::c_void);
    pthread_mutex_unlock(&mut moduleUnblockedClientsMutex);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnblockClient(mut c: *mut client) {
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    moduleUnblockClientByHandle(bc, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleClientIsBlockedOnKeys(mut c: *mut client) -> libc::c_int {
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    return (*bc).blocked_on_keys;
}
#[no_mangle]
pub unsafe extern "C" fn RM_UnblockClient(
    mut bc: *mut RedisModuleBlockedClient,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    if (*bc).blocked_on_keys != 0 {
        if ((*bc).timeout_callback).is_none() {
            return 1 as libc::c_int;
        }
        if (*bc).unblocked != 0 {
            return 0 as libc::c_int;
        }
        if !((*bc).client).is_null() {
            moduleBlockedClientTimedOut((*bc).client);
        }
    }
    moduleUnblockClientByHandle(bc, privdata);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_AbortBlock(
    mut bc: *mut RedisModuleBlockedClient,
) -> libc::c_int {
    (*bc).reply_callback = None;
    (*bc).disconnect_callback = None;
    return RM_UnblockClient(bc, 0 as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetDisconnectCallback(
    mut bc: *mut RedisModuleBlockedClient,
    mut callback: RedisModuleDisconnectFunc,
) {
    (*bc).disconnect_callback = callback;
}
#[no_mangle]
pub unsafe extern "C" fn moduleHandleBlockedClients() {
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut bc: *mut RedisModuleBlockedClient = 0 as *mut RedisModuleBlockedClient;
    pthread_mutex_lock(&mut moduleUnblockedClientsMutex);
    while (*moduleUnblockedClients).len != 0 {
        ln = (*moduleUnblockedClients).head;
        bc = (*ln).value as *mut RedisModuleBlockedClient;
        let mut c: *mut client = (*bc).client;
        listDelNode(moduleUnblockedClients, ln);
        pthread_mutex_unlock(&mut moduleUnblockedClientsMutex);
        let mut prev_error_replies: libc::c_longlong = server.stat_total_error_replies;
        let mut reply_us: uint64_t = 0 as libc::c_int as uint64_t;
        if !c.is_null() && (*bc).blocked_on_keys == 0 && ((*bc).reply_callback).is_some()
        {
            let mut ctx: RedisModuleCtx = RedisModuleCtx {
                getapifuncptr: 0 as *mut libc::c_void,
                module: 0 as *mut RedisModule,
                client: 0 as *mut client,
                blocked_client: 0 as *mut RedisModuleBlockedClient,
                amqueue: 0 as *mut AutoMemEntry,
                amqueue_len: 0,
                amqueue_used: 0,
                flags: 0,
                postponed_arrays: 0 as *mut *mut libc::c_void,
                postponed_arrays_count: 0,
                blocked_privdata: 0 as *mut libc::c_void,
                blocked_ready_key: 0 as *mut robj,
                keys_result: 0 as *mut getKeysResult,
                pa_head: 0 as *mut RedisModulePoolAllocBlock,
                next_yield_time: 0,
                user: 0 as *const RedisModuleUser,
            };
            moduleCreateContext(
                &mut ctx,
                (*bc).module,
                (1 as libc::c_int) << 2 as libc::c_int,
            );
            ctx.blocked_privdata = (*bc).privdata;
            ctx.blocked_ready_key = 0 as *mut robj;
            ctx.client = (*bc).client;
            ctx.blocked_client = bc;
            let mut replyTimer: monotime = 0;
            elapsedStart(&mut replyTimer);
            ((*bc).reply_callback)
                .expect(
                    "non-null function pointer",
                )(&mut ctx, (*c).argv as *mut *mut libc::c_void, (*c).argc);
            reply_us = elapsedUs(replyTimer);
            moduleFreeContext(&mut ctx);
        }
        if !((*bc).privdata).is_null() && ((*bc).free_privdata).is_some() {
            let mut ctx_0: RedisModuleCtx = RedisModuleCtx {
                getapifuncptr: 0 as *mut libc::c_void,
                module: 0 as *mut RedisModule,
                client: 0 as *mut client,
                blocked_client: 0 as *mut RedisModuleBlockedClient,
                amqueue: 0 as *mut AutoMemEntry,
                amqueue_len: 0,
                amqueue_used: 0,
                flags: 0,
                postponed_arrays: 0 as *mut *mut libc::c_void,
                postponed_arrays_count: 0,
                blocked_privdata: 0 as *mut libc::c_void,
                blocked_ready_key: 0 as *mut robj,
                keys_result: 0 as *mut getKeysResult,
                pa_head: 0 as *mut RedisModulePoolAllocBlock,
                next_yield_time: 0,
                user: 0 as *const RedisModuleUser,
            };
            let mut ctx_flags: libc::c_int = if c.is_null() {
                (1 as libc::c_int) << 5 as libc::c_int
            } else {
                0 as libc::c_int
            };
            moduleCreateContext(&mut ctx_0, (*bc).module, ctx_flags);
            ctx_0.blocked_privdata = (*bc).privdata;
            ctx_0.client = (*bc).client;
            ((*bc).free_privdata)
                .expect("non-null function pointer")(&mut ctx_0, (*bc).privdata);
            moduleFreeContext(&mut ctx_0);
        }
        if !c.is_null() {
            AddReplyFromClient(c, (*bc).reply_client);
        }
        moduleReleaseTempClient((*bc).reply_client);
        moduleReleaseTempClient((*bc).thread_safe_ctx_client);
        if !c.is_null() && (*bc).blocked_on_keys == 0 {
            updateStatsOnUnblock(
                c,
                (*bc).background_duration as libc::c_long,
                reply_us as libc::c_long,
                (server.stat_total_error_replies != prev_error_replies) as libc::c_int,
            );
        }
        if !c.is_null() {
            (*bc).disconnect_callback = None;
            unblockClient(c);
            if clientHasPendingReplies(c) != 0
                && (*c).flags
                    & ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong == 0
            {
                (*c).flags |= ((1 as libc::c_int) << 21 as libc::c_int) as libc::c_ulong;
                listAddNodeHead(server.clients_pending_write, c as *mut libc::c_void);
            }
        }
        (*(*bc).module).blocked_clients -= 1;
        zfree(bc as *mut libc::c_void);
        pthread_mutex_lock(&mut moduleUnblockedClientsMutex);
    }
    pthread_mutex_unlock(&mut moduleUnblockedClientsMutex);
}
#[no_mangle]
pub unsafe extern "C" fn moduleBlockedClientMayTimeout(
    mut c: *mut client,
) -> libc::c_int {
    if (*c).btype != 3 as libc::c_int {
        return 1 as libc::c_int;
    }
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    return (!bc.is_null() && ((*bc).timeout_callback).is_some()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleBlockedClientTimedOut(mut c: *mut client) {
    let mut bc: *mut RedisModuleBlockedClient = (*c).bpop.module_blocked_handle
        as *mut RedisModuleBlockedClient;
    if (*bc).unblocked != 0 {
        return;
    }
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(&mut ctx, (*bc).module, (1 as libc::c_int) << 3 as libc::c_int);
    ctx.client = (*bc).client;
    ctx.blocked_client = bc;
    ctx.blocked_privdata = (*bc).privdata;
    let mut prev_error_replies: libc::c_longlong = server.stat_total_error_replies;
    ((*bc).timeout_callback)
        .expect(
            "non-null function pointer",
        )(&mut ctx, (*c).argv as *mut *mut libc::c_void, (*c).argc);
    moduleFreeContext(&mut ctx);
    if (*bc).blocked_on_keys == 0 {
        updateStatsOnUnblock(
            c,
            (*bc).background_duration as libc::c_long,
            0 as libc::c_int as libc::c_long,
            (server.stat_total_error_replies != prev_error_replies) as libc::c_int,
        );
    }
    (*bc).disconnect_callback = None;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsBlockedReplyRequest(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    return ((*ctx).flags & (1 as libc::c_int) << 2 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsBlockedTimeoutRequest(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    return ((*ctx).flags & (1 as libc::c_int) << 3 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetBlockedClientPrivateData(
    mut ctx: *mut RedisModuleCtx,
) -> *mut libc::c_void {
    return (*ctx).blocked_privdata;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetBlockedClientReadyKey(
    mut ctx: *mut RedisModuleCtx,
) -> *mut robj {
    return (*ctx).blocked_ready_key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetBlockedClientHandle(
    mut ctx: *mut RedisModuleCtx,
) -> *mut RedisModuleBlockedClient {
    return (*ctx).blocked_client;
}
#[no_mangle]
pub unsafe extern "C" fn RM_BlockedClientDisconnected(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    return ((*ctx).flags & (1 as libc::c_int) << 5 as libc::c_int != 0 as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetThreadSafeContext(
    mut bc: *mut RedisModuleBlockedClient,
) -> *mut RedisModuleCtx {
    let mut ctx: *mut RedisModuleCtx = zmalloc(
        core::mem::size_of::<RedisModuleCtx>() as libc::c_ulong,
    ) as *mut RedisModuleCtx;
    let mut module: *mut RedisModule = if !bc.is_null() {
        (*bc).module
    } else {
        0 as *mut RedisModule
    };
    let mut flags: libc::c_int = (1 as libc::c_int) << 4 as libc::c_int;
    if bc.is_null() {
        flags |= (1 as libc::c_int) << 7 as libc::c_int;
    }
    moduleCreateContext(ctx, module, flags);
    if !bc.is_null() {
        (*ctx).blocked_client = bc;
        (*ctx).client = (*bc).thread_safe_ctx_client;
        selectDb((*ctx).client, (*bc).dbid);
        if !((*bc).client).is_null() {
            (*(*ctx).client).id = (*(*bc).client).id;
            (*(*ctx).client).resp = (*(*bc).client).resp;
        }
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDetachedThreadSafeContext(
    mut ctx: *mut RedisModuleCtx,
) -> *mut RedisModuleCtx {
    let mut new_ctx: *mut RedisModuleCtx = zmalloc(
        core::mem::size_of::<RedisModuleCtx>() as libc::c_ulong,
    ) as *mut RedisModuleCtx;
    moduleCreateContext(
        new_ctx,
        (*ctx).module,
        (1 as libc::c_int) << 4 as libc::c_int | (1 as libc::c_int) << 7 as libc::c_int,
    );
    return new_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeThreadSafeContext(mut ctx: *mut RedisModuleCtx) {
    moduleFreeContext(ctx);
    zfree(ctx as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleGILAfterLock() {
    if server.module_ctx_nesting == 0 as libc::c_int {} else {
        _serverAssert(
            b"server.module_ctx_nesting == 0\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            7737 as libc::c_int,
        );
        unreachable!();
    };
    server.module_ctx_nesting += 1;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ThreadSafeContextLock(mut ctx: *mut RedisModuleCtx) {
    moduleAcquireGIL();
    moduleGILAfterLock();
}
#[no_mangle]
pub unsafe extern "C" fn RM_ThreadSafeContextTryLock(
    mut ctx: *mut RedisModuleCtx,
) -> libc::c_int {
    let mut res: libc::c_int = moduleTryAcquireGIL();
    if res != 0 as libc::c_int {
        *__errno_location() = res;
        return 1 as libc::c_int;
    }
    moduleGILAfterLock();
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleGILBeforeUnlock() {
    if server.module_ctx_nesting == 1 as libc::c_int {} else {
        _serverAssert(
            b"server.module_ctx_nesting == 1\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            7774 as libc::c_int,
        );
        unreachable!();
    };
    server.module_ctx_nesting -= 1;
    propagatePendingCommands();
    if server.busy_module_yield_flags != 0 {
        blockingOperationEnds();
        server.busy_module_yield_flags = 0 as libc::c_int;
        if !(server.current_client).is_null() {
            unprotectClient(server.current_client);
        }
        unblockPostponedClients();
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_ThreadSafeContextUnlock(mut ctx: *mut RedisModuleCtx) {
    moduleGILBeforeUnlock();
    moduleReleaseGIL();
}
#[no_mangle]
pub unsafe extern "C" fn moduleAcquireGIL() {
    pthread_mutex_lock(&mut moduleGIL);
}
#[no_mangle]
pub unsafe extern "C" fn moduleTryAcquireGIL() -> libc::c_int {
    return pthread_mutex_trylock(&mut moduleGIL);
}
#[no_mangle]
pub unsafe extern "C" fn moduleReleaseGIL() {
    pthread_mutex_unlock(&mut moduleGIL);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SubscribeToKeyspaceEvents(
    mut ctx: *mut RedisModuleCtx,
    mut types: libc::c_int,
    mut callback: RedisModuleNotificationFunc,
) -> libc::c_int {
    let mut sub: *mut RedisModuleKeyspaceSubscriber = zmalloc(
        core::mem::size_of::<RedisModuleKeyspaceSubscriber>() as libc::c_ulong,
    ) as *mut RedisModuleKeyspaceSubscriber;
    (*sub).module = (*ctx).module;
    (*sub).event_mask = types;
    (*sub).notify_callback = callback;
    (*sub).active = 0 as libc::c_int;
    listAddNodeTail(moduleKeyspaceSubscribers, sub as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetNotifyKeyspaceEvents() -> libc::c_int {
    return server.notify_keyspace_events;
}
#[no_mangle]
pub unsafe extern "C" fn RM_NotifyKeyspaceEvent(
    mut ctx: *mut RedisModuleCtx,
    mut type_0: libc::c_int,
    mut event: *const libc::c_char,
    mut key: *mut robj,
) -> libc::c_int {
    if ctx.is_null() || ((*ctx).client).is_null() {
        return 1 as libc::c_int;
    }
    notifyKeyspaceEvent(
        type_0,
        event as *mut libc::c_char,
        key,
        (*(*(*ctx).client).db).id,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleNotifyKeyspaceEvent(
    mut type_0: libc::c_int,
    mut event: *const libc::c_char,
    mut key: *mut robj,
    mut dbid: libc::c_int,
) {
    if (*moduleKeyspaceSubscribers).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(moduleKeyspaceSubscribers, &mut li);
    type_0
        &= !((1 as libc::c_int) << 1 as libc::c_int
            | (1 as libc::c_int) << 0 as libc::c_int);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut sub: *mut RedisModuleKeyspaceSubscriber = (*ln).value
            as *mut RedisModuleKeyspaceSubscriber;
        if (*sub).event_mask & type_0 != 0 && (*sub).active == 0 as libc::c_int {
            let mut ctx: RedisModuleCtx = RedisModuleCtx {
                getapifuncptr: 0 as *mut libc::c_void,
                module: 0 as *mut RedisModule,
                client: 0 as *mut client,
                blocked_client: 0 as *mut RedisModuleBlockedClient,
                amqueue: 0 as *mut AutoMemEntry,
                amqueue_len: 0,
                amqueue_used: 0,
                flags: 0,
                postponed_arrays: 0 as *mut *mut libc::c_void,
                postponed_arrays_count: 0,
                blocked_privdata: 0 as *mut libc::c_void,
                blocked_ready_key: 0 as *mut robj,
                keys_result: 0 as *mut getKeysResult,
                pa_head: 0 as *mut RedisModulePoolAllocBlock,
                next_yield_time: 0,
                user: 0 as *const RedisModuleUser,
            };
            moduleCreateContext(
                &mut ctx,
                (*sub).module,
                (1 as libc::c_int) << 6 as libc::c_int,
            );
            selectDb(ctx.client, dbid);
            (*sub).active = 1 as libc::c_int;
            ((*sub).notify_callback)
                .expect("non-null function pointer")(&mut ctx, type_0, event, key);
            (*sub).active = 0 as libc::c_int;
            moduleFreeContext(&mut ctx);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnsubscribeNotifications(mut module: *mut RedisModule) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(moduleKeyspaceSubscribers, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut sub: *mut RedisModuleKeyspaceSubscriber = (*ln).value
            as *mut RedisModuleKeyspaceSubscriber;
        if (*sub).module == module {
            listDelNode(moduleKeyspaceSubscribers, ln);
            zfree(sub as *mut libc::c_void);
        }
    };
}
static mut clusterReceivers: [*mut moduleClusterReceiver; 255] = [0
    as *const moduleClusterReceiver as *mut moduleClusterReceiver; 255];
#[no_mangle]
pub unsafe extern "C" fn moduleCallClusterReceivers(
    mut sender_id: *const libc::c_char,
    mut module_id: uint64_t,
    mut type_0: uint8_t,
    mut payload: *const libc::c_uchar,
    mut len: uint32_t,
) {
    let mut r: *mut moduleClusterReceiver = clusterReceivers[type_0 as usize];
    while !r.is_null() {
        if (*r).module_id == module_id {
            let mut ctx: RedisModuleCtx = RedisModuleCtx {
                getapifuncptr: 0 as *mut libc::c_void,
                module: 0 as *mut RedisModule,
                client: 0 as *mut client,
                blocked_client: 0 as *mut RedisModuleBlockedClient,
                amqueue: 0 as *mut AutoMemEntry,
                amqueue_len: 0,
                amqueue_used: 0,
                flags: 0,
                postponed_arrays: 0 as *mut *mut libc::c_void,
                postponed_arrays_count: 0,
                blocked_privdata: 0 as *mut libc::c_void,
                blocked_ready_key: 0 as *mut robj,
                keys_result: 0 as *mut getKeysResult,
                pa_head: 0 as *mut RedisModulePoolAllocBlock,
                next_yield_time: 0,
                user: 0 as *const RedisModuleUser,
            };
            moduleCreateContext(
                &mut ctx,
                (*r).module,
                (1 as libc::c_int) << 6 as libc::c_int,
            );
            ((*r).callback)
                .expect(
                    "non-null function pointer",
                )(&mut ctx, sender_id, type_0, payload, len);
            moduleFreeContext(&mut ctx);
            return;
        }
        r = (*r).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterClusterMessageReceiver(
    mut ctx: *mut RedisModuleCtx,
    mut type_0: uint8_t,
    mut callback: RedisModuleClusterMessageReceiver,
) {
    if server.cluster_enabled == 0 {
        return;
    }
    let mut module_id: uint64_t = moduleTypeEncodeId(
        (*(*ctx).module).name,
        0 as libc::c_int,
    );
    let mut r: *mut moduleClusterReceiver = clusterReceivers[type_0 as usize];
    let mut prev: *mut moduleClusterReceiver = 0 as *mut moduleClusterReceiver;
    while !r.is_null() {
        if (*r).module_id == module_id {
            if callback.is_some() {
                (*r).callback = callback;
            } else {
                if !prev.is_null() {
                    (*prev).next = (*r).next;
                } else {
                    (*clusterReceivers[type_0 as usize]).next = (*r).next;
                }
                zfree(r as *mut libc::c_void);
            }
            return;
        }
        prev = r;
        r = (*r).next;
    }
    if callback.is_some() {
        r = zmalloc(core::mem::size_of::<moduleClusterReceiver>() as libc::c_ulong)
            as *mut moduleClusterReceiver;
        (*r).module_id = module_id;
        (*r).module = (*ctx).module;
        (*r).callback = callback;
        (*r).next = clusterReceivers[type_0 as usize];
        clusterReceivers[type_0 as usize] = r;
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_SendClusterMessage(
    mut ctx: *mut RedisModuleCtx,
    mut target_id: *const libc::c_char,
    mut type_0: uint8_t,
    mut msg: *const libc::c_char,
    mut len: uint32_t,
) -> libc::c_int {
    if server.cluster_enabled == 0 {
        return 1 as libc::c_int;
    }
    let mut module_id: uint64_t = moduleTypeEncodeId(
        (*(*ctx).module).name,
        0 as libc::c_int,
    );
    if clusterSendModuleMessageToTarget(target_id, module_id, type_0, msg, len)
        == 0 as libc::c_int
    {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClusterNodesList(
    mut ctx: *mut RedisModuleCtx,
    mut numnodes: *mut size_t,
) -> *mut *mut libc::c_char {
    if server.cluster_enabled == 0 {
        return 0 as *mut *mut libc::c_char;
    }
    let mut count: size_t = ((*(*server.cluster).nodes)
        .ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize]);
    let mut ids: *mut *mut libc::c_char = zmalloc(
        count
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_mul(40 as libc::c_int as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let mut di: *mut dictIterator = dictGetIterator((*server.cluster).nodes);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut j: libc::c_int = 0 as libc::c_int;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut node: *mut clusterNode = (*de).v.val as *mut clusterNode;
        if (*node).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
            continue;
        }
        let ref mut fresh31 = *ids.offset(j as isize);
        *fresh31 = zmalloc(40 as libc::c_int as size_t) as *mut libc::c_char;
        memcpy(
            *ids.offset(j as isize) as *mut libc::c_void,
            ((*node).name).as_mut_ptr() as *const libc::c_void,
            40 as libc::c_int as libc::c_ulong,
        );
        j += 1;
    }
    *numnodes = j as size_t;
    let ref mut fresh32 = *ids.offset(j as isize);
    *fresh32 = 0 as *mut libc::c_char;
    dictReleaseIterator(di);
    return ids;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeClusterNodesList(mut ids: *mut *mut libc::c_char) {
    if ids.is_null() {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while !(*ids.offset(j as isize)).is_null() {
        zfree(*ids.offset(j as isize) as *mut libc::c_void);
        j += 1;
    }
    zfree(ids as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetMyClusterID() -> *const libc::c_char {
    if server.cluster_enabled == 0 {
        return 0 as *const libc::c_char;
    }
    return ((*(*server.cluster).myself).name).as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClusterSize() -> size_t {
    if server.cluster_enabled == 0 {
        return 0 as libc::c_int as size_t;
    }
    return ((*(*server.cluster).nodes).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*(*server.cluster).nodes).ht_used[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClusterNodeInfo(
    mut ctx: *mut RedisModuleCtx,
    mut id: *const libc::c_char,
    mut ip: *mut libc::c_char,
    mut master_id: *mut libc::c_char,
    mut port: *mut libc::c_int,
    mut flags: *mut libc::c_int,
) -> libc::c_int {
    let mut node: *mut clusterNode = clusterLookupNode(id, strlen(id) as libc::c_int);
    if node.is_null() || (*node).flags & (64 as libc::c_int | 32 as libc::c_int) != 0 {
        return 1 as libc::c_int;
    }
    if !ip.is_null() {
        strncpy(ip, ((*node).ip).as_mut_ptr(), 46 as libc::c_int as libc::c_ulong);
    }
    if !master_id.is_null() {
        if (*node).flags & 2 as libc::c_int != 0 && !((*node).slaveof).is_null() {
            memcpy(
                master_id as *mut libc::c_void,
                ((*(*node).slaveof).name).as_mut_ptr() as *const libc::c_void,
                40 as libc::c_int as libc::c_ulong,
            );
        } else {
            memset(
                master_id as *mut libc::c_void,
                0 as libc::c_int,
                40 as libc::c_int as libc::c_ulong,
            );
        }
    }
    if !port.is_null() {
        *port = (*node).port;
    }
    if !flags.is_null() {
        *flags = 0 as libc::c_int;
        if (*node).flags & 16 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 0 as libc::c_int;
        }
        if (*node).flags & 1 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 1 as libc::c_int;
        }
        if (*node).flags & 2 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 2 as libc::c_int;
        }
        if (*node).flags & 4 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 3 as libc::c_int;
        }
        if (*node).flags & 8 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 4 as libc::c_int;
        }
        if (*node).flags & 512 as libc::c_int != 0 {
            *flags |= (1 as libc::c_int) << 5 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetClusterFlags(
    mut ctx: *mut RedisModuleCtx,
    mut flags: uint64_t,
) {
    if flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0 {
        server.cluster_module_flags |= (1 as libc::c_int) << 1 as libc::c_int;
    }
    if flags & ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong != 0 {
        server.cluster_module_flags |= (1 as libc::c_int) << 2 as libc::c_int;
    }
}
static mut Timers: *mut rax = 0 as *const rax as *mut rax;
#[no_mangle]
pub static mut aeTimer: libc::c_longlong = -(1 as libc::c_int) as libc::c_longlong;
#[no_mangle]
pub unsafe extern "C" fn moduleTimerHandler(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    let mut ri: raxIterator = raxIterator {
        flags: 0,
        rt: 0 as *mut rax,
        key: 0 as *mut libc::c_uchar,
        data: 0 as *mut libc::c_void,
        key_len: 0,
        key_max: 0,
        key_static_string: [0; 128],
        node: 0 as *mut raxNode,
        stack: raxStack {
            stack: 0 as *mut *mut libc::c_void,
            items: 0,
            maxitems: 0,
            static_items: [0 as *mut libc::c_void; 32],
            oom: 0,
        },
        node_cb: None,
    };
    raxStart(&mut ri, Timers);
    let mut now: uint64_t = ustime() as uint64_t;
    let mut next_period: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    loop {
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        if raxNext(&mut ri) == 0 {
            break;
        }
        let mut expiretime: uint64_t = 0;
        memcpy(
            &mut expiretime as *mut uint64_t as *mut libc::c_void,
            ri.key as *const libc::c_void,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        );
        expiretime = intrev64(expiretime);
        if now >= expiretime {
            let mut timer: *mut RedisModuleTimer = ri.data as *mut RedisModuleTimer;
            let mut ctx: RedisModuleCtx = RedisModuleCtx {
                getapifuncptr: 0 as *mut libc::c_void,
                module: 0 as *mut RedisModule,
                client: 0 as *mut client,
                blocked_client: 0 as *mut RedisModuleBlockedClient,
                amqueue: 0 as *mut AutoMemEntry,
                amqueue_len: 0,
                amqueue_used: 0,
                flags: 0,
                postponed_arrays: 0 as *mut *mut libc::c_void,
                postponed_arrays_count: 0,
                blocked_privdata: 0 as *mut libc::c_void,
                blocked_ready_key: 0 as *mut robj,
                keys_result: 0 as *mut getKeysResult,
                pa_head: 0 as *mut RedisModulePoolAllocBlock,
                next_yield_time: 0,
                user: 0 as *const RedisModuleUser,
            };
            moduleCreateContext(
                &mut ctx,
                (*timer).module,
                (1 as libc::c_int) << 6 as libc::c_int,
            );
            selectDb(ctx.client, (*timer).dbid);
            ((*timer).callback)
                .expect("non-null function pointer")(&mut ctx, (*timer).data);
            moduleFreeContext(&mut ctx);
            raxRemove(Timers, ri.key, ri.key_len, 0 as *mut *mut libc::c_void);
            zfree(timer as *mut libc::c_void);
        } else {
            next_period = (expiretime as libc::c_longlong - ustime())
                / 1000 as libc::c_int as libc::c_longlong;
            break;
        }
    }
    raxStop(&mut ri);
    if next_period <= 0 as libc::c_int as libc::c_longlong {
        next_period = 1 as libc::c_int as libc::c_longlong;
    }
    if raxSize(Timers) > 0 as libc::c_int as libc::c_ulong {
        return next_period as libc::c_int
    } else {
        aeTimer = -(1 as libc::c_int) as libc::c_longlong;
        return -(1 as libc::c_int);
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateTimer(
    mut ctx: *mut RedisModuleCtx,
    mut period: mstime_t,
    mut callback: RedisModuleTimerProc,
    mut data: *mut libc::c_void,
) -> RedisModuleTimerID {
    let mut timer: *mut RedisModuleTimer = zmalloc(
        core::mem::size_of::<RedisModuleTimer>() as libc::c_ulong,
    ) as *mut RedisModuleTimer;
    (*timer).module = (*ctx).module;
    (*timer).callback = callback;
    (*timer).data = data;
    (*timer)
        .dbid = if !((*ctx).client).is_null() {
        (*(*(*ctx).client).db).id
    } else {
        0 as libc::c_int
    };
    let mut expiretime: uint64_t = (ustime()
        + period * 1000 as libc::c_int as libc::c_longlong) as uint64_t;
    let mut key: uint64_t = 0;
    loop {
        key = intrev64(expiretime);
        if raxFind(
            Timers,
            &mut key as *mut uint64_t as *mut libc::c_uchar,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) == raxNotFound
        {
            raxInsert(
                Timers,
                &mut key as *mut uint64_t as *mut libc::c_uchar,
                core::mem::size_of::<uint64_t>() as libc::c_ulong,
                timer as *mut libc::c_void,
                0 as *mut *mut libc::c_void,
            );
            break;
        } else {
            expiretime = expiretime.wrapping_add(1);
        }
    }
    if aeTimer != -(1 as libc::c_int) as libc::c_longlong {
        let mut ri: raxIterator = raxIterator {
            flags: 0,
            rt: 0 as *mut rax,
            key: 0 as *mut libc::c_uchar,
            data: 0 as *mut libc::c_void,
            key_len: 0,
            key_max: 0,
            key_static_string: [0; 128],
            node: 0 as *mut raxNode,
            stack: raxStack {
                stack: 0 as *mut *mut libc::c_void,
                items: 0,
                maxitems: 0,
                static_items: [0 as *mut libc::c_void; 32],
                oom: 0,
            },
            node_cb: None,
        };
        raxStart(&mut ri, Timers);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        raxNext(&mut ri);
        if memcmp(
            ri.key as *const libc::c_void,
            &mut key as *mut uint64_t as *const libc::c_void,
            core::mem::size_of::<uint64_t>() as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            aeDeleteTimeEvent(server.el, aeTimer);
            aeTimer = -(1 as libc::c_int) as libc::c_longlong;
        }
        raxStop(&mut ri);
    }
    if aeTimer == -(1 as libc::c_int) as libc::c_longlong {
        aeTimer = aeCreateTimeEvent(
            server.el,
            period,
            Some(
                moduleTimerHandler
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_longlong,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
            0 as *mut libc::c_void,
            None,
        );
    }
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_StopTimer(
    mut ctx: *mut RedisModuleCtx,
    mut id: RedisModuleTimerID,
    mut data: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut timer: *mut RedisModuleTimer = raxFind(
        Timers,
        &mut id as *mut RedisModuleTimerID as *mut libc::c_uchar,
        core::mem::size_of::<RedisModuleTimerID>() as libc::c_ulong,
    ) as *mut RedisModuleTimer;
    if timer == raxNotFound as *mut RedisModuleTimer || (*timer).module != (*ctx).module
    {
        return 1 as libc::c_int;
    }
    if !data.is_null() {
        *data = (*timer).data;
    }
    raxRemove(
        Timers,
        &mut id as *mut RedisModuleTimerID as *mut libc::c_uchar,
        core::mem::size_of::<RedisModuleTimerID>() as libc::c_ulong,
        0 as *mut *mut libc::c_void,
    );
    zfree(timer as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetTimerInfo(
    mut ctx: *mut RedisModuleCtx,
    mut id: RedisModuleTimerID,
    mut remaining: *mut uint64_t,
    mut data: *mut *mut libc::c_void,
) -> libc::c_int {
    let mut timer: *mut RedisModuleTimer = raxFind(
        Timers,
        &mut id as *mut RedisModuleTimerID as *mut libc::c_uchar,
        core::mem::size_of::<RedisModuleTimerID>() as libc::c_ulong,
    ) as *mut RedisModuleTimer;
    if timer == raxNotFound as *mut RedisModuleTimer || (*timer).module != (*ctx).module
    {
        return 1 as libc::c_int;
    }
    if !remaining.is_null() {
        let mut rem: int64_t = (intrev64(id) as libc::c_ulonglong)
            .wrapping_sub(ustime() as libc::c_ulonglong) as int64_t;
        if rem < 0 as libc::c_int as libc::c_long {
            rem = 0 as libc::c_int as int64_t;
        }
        *remaining = (rem / 1000 as libc::c_int as libc::c_long) as uint64_t;
    }
    if !data.is_null() {
        *data = (*timer).data;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleHoldsTimer(mut module: *mut RedisModule) -> libc::c_int {
    let mut iter: raxIterator = raxIterator {
        flags: 0,
        rt: 0 as *mut rax,
        key: 0 as *mut libc::c_uchar,
        data: 0 as *mut libc::c_void,
        key_len: 0,
        key_max: 0,
        key_static_string: [0; 128],
        node: 0 as *mut raxNode,
        stack: raxStack {
            stack: 0 as *mut *mut libc::c_void,
            items: 0,
            maxitems: 0,
            static_items: [0 as *mut libc::c_void; 32],
            oom: 0,
        },
        node_cb: None,
    };
    let mut found: libc::c_int = 0 as libc::c_int;
    raxStart(&mut iter, Timers);
    raxSeek(
        &mut iter,
        b"^\0" as *const u8 as *const libc::c_char,
        0 as *mut libc::c_uchar,
        0 as libc::c_int as size_t,
    );
    while raxNext(&mut iter) != 0 {
        let mut timer: *mut RedisModuleTimer = iter.data as *mut RedisModuleTimer;
        if !((*timer).module == module) {
            continue;
        }
        found = 1 as libc::c_int;
        break;
    }
    raxStop(&mut iter);
    return found;
}
#[no_mangle]
pub static mut moduleEventLoopOneShots: *mut list = 0 as *const list as *mut list;
static mut moduleEventLoopMutex: pthread_mutex_t = pthread_mutex_t {
    __data: {
        let mut init = __pthread_mutex_s {
            __lock: 0 as libc::c_int,
            __count: 0 as libc::c_int as libc::c_uint,
            __owner: 0 as libc::c_int,
            __nusers: 0 as libc::c_int as libc::c_uint,
            __kind: PTHREAD_MUTEX_TIMED_NP as libc::c_int,
            __spins: 0 as libc::c_int,
            __list: {
                let mut init = __pthread_internal_list {
                    __prev: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                    __next: 0 as *const __pthread_internal_list
                        as *mut __pthread_internal_list,
                };
                init
            },
        };
        init
    },
};
unsafe extern "C" fn eventLoopToAeMask(mut mask: libc::c_int) -> libc::c_int {
    let mut aeMask: libc::c_int = 0 as libc::c_int;
    if mask & 1 as libc::c_int != 0 {
        aeMask |= 1 as libc::c_int;
    }
    if mask & 2 as libc::c_int != 0 {
        aeMask |= 2 as libc::c_int;
    }
    return aeMask;
}
unsafe extern "C" fn eventLoopFromAeMask(mut ae_mask: libc::c_int) -> libc::c_int {
    let mut mask: libc::c_int = 0 as libc::c_int;
    if ae_mask & 1 as libc::c_int != 0 {
        mask |= 1 as libc::c_int;
    }
    if ae_mask & 2 as libc::c_int != 0 {
        mask |= 2 as libc::c_int;
    }
    return mask;
}
unsafe extern "C" fn eventLoopCbReadable(
    mut ae: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut user_data: *mut libc::c_void,
    mut ae_mask: libc::c_int,
) {
    let mut data: *mut EventLoopData = user_data as *mut EventLoopData;
    ((*data).rFunc)
        .expect(
            "non-null function pointer",
        )(fd, (*data).user_data, eventLoopFromAeMask(ae_mask));
}
unsafe extern "C" fn eventLoopCbWritable(
    mut ae: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut user_data: *mut libc::c_void,
    mut ae_mask: libc::c_int,
) {
    let mut data: *mut EventLoopData = user_data as *mut EventLoopData;
    ((*data).wFunc)
        .expect(
            "non-null function pointer",
        )(fd, (*data).user_data, eventLoopFromAeMask(ae_mask));
}
#[no_mangle]
pub unsafe extern "C" fn RM_EventLoopAdd(
    mut fd: libc::c_int,
    mut mask: libc::c_int,
    mut func: RedisModuleEventLoopFunc,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    if fd < 0 as libc::c_int || fd >= aeGetSetSize(server.el) {
        *__errno_location() = 34 as libc::c_int;
        return 1 as libc::c_int;
    }
    if func.is_none() || mask & !(1 as libc::c_int | 2 as libc::c_int) != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut data: *mut EventLoopData = aeGetFileClientData(server.el, fd)
        as *mut EventLoopData;
    if data.is_null() {
        data = zcalloc(core::mem::size_of::<EventLoopData>() as libc::c_ulong)
            as *mut EventLoopData;
    }
    let mut aeProc: Option::<aeFileProc> = None;
    if mask & 1 as libc::c_int != 0 {
        aeProc = Some(
            eventLoopCbReadable
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        );
    } else {
        aeProc = Some(
            eventLoopCbWritable
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        );
    }
    let mut aeMask: libc::c_int = eventLoopToAeMask(mask);
    if aeCreateFileEvent(server.el, fd, aeMask, aeProc, data as *mut libc::c_void)
        != 0 as libc::c_int
    {
        if aeGetFileEvents(server.el, fd) == 0 as libc::c_int {
            zfree(data as *mut libc::c_void);
        }
        return 1 as libc::c_int;
    }
    (*data).user_data = user_data;
    if mask & 1 as libc::c_int != 0 {
        (*data).rFunc = func;
    }
    if mask & 2 as libc::c_int != 0 {
        (*data).wFunc = func;
    }
    *__errno_location() = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_EventLoopDel(
    mut fd: libc::c_int,
    mut mask: libc::c_int,
) -> libc::c_int {
    if fd < 0 as libc::c_int || fd >= aeGetSetSize(server.el) {
        *__errno_location() = 34 as libc::c_int;
        return 1 as libc::c_int;
    }
    if mask & !(1 as libc::c_int | 2 as libc::c_int) != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut data: *mut EventLoopData = aeGetFileClientData(server.el, fd)
        as *mut EventLoopData;
    aeDeleteFileEvent(server.el, fd, eventLoopToAeMask(mask));
    if aeGetFileEvents(server.el, fd) == 0 as libc::c_int {
        zfree(data as *mut libc::c_void);
    }
    *__errno_location() = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_EventLoopAddOneShot(
    mut func: RedisModuleEventLoopOneShotFunc,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    if func.is_none() {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut oneshot: *mut EventLoopOneShot = zmalloc(
        core::mem::size_of::<EventLoopOneShot>() as libc::c_ulong,
    ) as *mut EventLoopOneShot;
    (*oneshot).func = func;
    (*oneshot).user_data = user_data;
    pthread_mutex_lock(&mut moduleEventLoopMutex);
    if moduleEventLoopOneShots.is_null() {
        moduleEventLoopOneShots = listCreate();
    }
    listAddNodeTail(moduleEventLoopOneShots, oneshot as *mut libc::c_void);
    pthread_mutex_unlock(&mut moduleEventLoopMutex);
    write(
        server.module_pipe[1 as libc::c_int as usize],
        b"A\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) != 1 as libc::c_int as libc::c_long;
    *__errno_location() = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn eventLoopHandleOneShotEvents() {
    pthread_mutex_lock(&mut moduleEventLoopMutex);
    if !moduleEventLoopOneShots.is_null() {
        while (*moduleEventLoopOneShots).len != 0 {
            let mut ln: *mut listNode = (*moduleEventLoopOneShots).head;
            let mut oneshot: *mut EventLoopOneShot = (*ln).value
                as *mut EventLoopOneShot;
            listDelNode(moduleEventLoopOneShots, ln);
            pthread_mutex_unlock(&mut moduleEventLoopMutex);
            ((*oneshot).func).expect("non-null function pointer")((*oneshot).user_data);
            zfree(oneshot as *mut libc::c_void);
            pthread_mutex_lock(&mut moduleEventLoopMutex);
        }
    }
    pthread_mutex_unlock(&mut moduleEventLoopMutex);
}
#[no_mangle]
pub unsafe extern "C" fn moduleNotifyUserChanged(mut c: *mut client) {
    if ((*c).auth_callback).is_some() {
        ((*c).auth_callback)
            .expect("non-null function pointer")((*c).id, (*c).auth_callback_privdata);
        (*c).auth_callback = None;
        (*c).auth_callback_privdata = 0 as *mut libc::c_void;
        (*c).auth_module = 0 as *mut libc::c_void;
    }
}
#[no_mangle]
pub unsafe extern "C" fn revokeClientAuthentication(mut c: *mut client) {
    moduleNotifyUserChanged(c);
    (*c).user = DefaultUser;
    (*c).authenticated = 0 as libc::c_int;
    if c == server.current_client {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 40 as libc::c_int) as uint64_t;
    } else {
        freeClientAsync(c);
    };
}
unsafe extern "C" fn moduleFreeAuthenticatedClients(mut module: *mut RedisModule) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        if ((*c).auth_module).is_null() {
            continue;
        }
        let mut auth_module: *mut RedisModule = (*c).auth_module as *mut RedisModule;
        if auth_module == module {
            revokeClientAuthentication(c);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateModuleUser(
    mut name: *const libc::c_char,
) -> *mut RedisModuleUser {
    let mut new_user: *mut RedisModuleUser = zmalloc(
        core::mem::size_of::<RedisModuleUser>() as libc::c_ulong,
    ) as *mut RedisModuleUser;
    (*new_user).user = ACLCreateUnlinkedUser();
    (*new_user).free_user = 1 as libc::c_int;
    sdsfree((*(*new_user).user).name);
    (*(*new_user).user).name = sdsnew(name);
    return new_user;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeModuleUser(
    mut user: *mut RedisModuleUser,
) -> libc::c_int {
    if (*user).free_user != 0 {
        ACLFreeUserAndKillClients((*user).user);
    }
    zfree(user as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetModuleUserACL(
    mut user: *mut RedisModuleUser,
    mut acl: *const libc::c_char,
) -> libc::c_int {
    return ACLSetUser((*user).user, acl, -(1 as libc::c_int) as ssize_t);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetModuleUserACLString(
    mut ctx: *mut RedisModuleCtx,
    mut user: *mut RedisModuleUser,
    mut acl: *const libc::c_char,
    mut error: *mut *mut robj,
) -> libc::c_int {
    if !user.is_null() {} else {
        _serverAssert(
            b"user != NULL\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            8714 as libc::c_int,
        );
        unreachable!();
    };
    let mut argc: libc::c_int = 0;
    let mut argv: *mut sds = sdssplitargs(acl, &mut argc);
    let mut err: sds = ACLStringSetUser((*user).user, 0 as sds, argv, argc);
    sdsfreesplitres(argv, argc);
    if !err.is_null() {
        if !error.is_null() {
            *error = createObject(0 as libc::c_int, err as *mut libc::c_void);
            if !ctx.is_null() {
                autoMemoryAdd(ctx, 1 as libc::c_int, *error as *mut libc::c_void);
            }
        } else {
            sdsfree(err);
        }
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetModuleUserACLString(
    mut user: *mut RedisModuleUser,
) -> *mut robj {
    if !user.is_null() {} else {
        _serverAssert(
            b"user != NULL\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            8741 as libc::c_int,
        );
        unreachable!();
    };
    return ACLDescribeUser((*user).user);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetCurrentUserName(
    mut ctx: *mut RedisModuleCtx,
) -> *mut robj {
    return RM_CreateString(
        ctx,
        (*(*(*ctx).client).user).name as *const libc::c_char,
        sdslen((*(*(*ctx).client).user).name),
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetModuleUserFromUserName(
    mut name: *mut robj,
) -> *mut RedisModuleUser {
    let mut acl_user: *mut user = ACLGetUserByName(
        (*name).ptr as *const libc::c_char,
        sdslen((*name).ptr as sds),
    );
    if acl_user.is_null() {
        return 0 as *mut RedisModuleUser;
    }
    let mut new_user: *mut RedisModuleUser = zmalloc(
        core::mem::size_of::<RedisModuleUser>() as libc::c_ulong,
    ) as *mut RedisModuleUser;
    (*new_user).user = acl_user;
    (*new_user).free_user = 0 as libc::c_int;
    return new_user;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ACLCheckCommandPermissions(
    mut user: *mut RedisModuleUser,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) -> libc::c_int {
    let mut keyidxptr: libc::c_int = 0;
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    cmd = lookupCommand(argv, argc);
    if cmd.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return 1 as libc::c_int;
    }
    if ACLCheckAllUserCommandPerm((*user).user, cmd, argv, argc, &mut keyidxptr)
        != 0 as libc::c_int
    {
        *__errno_location() = 13 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ACLCheckKeyPermissions(
    mut user: *mut RedisModuleUser,
    mut key: *mut robj,
    mut flags: libc::c_int,
) -> libc::c_int {
    let allow_mask: libc::c_int = ((1 as libc::c_ulonglong) << 4 as libc::c_int
        | (1 as libc::c_ulonglong) << 6 as libc::c_int
        | (1 as libc::c_ulonglong) << 7 as libc::c_int
        | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_int;
    if flags & allow_mask != flags {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut keyspec_flags: libc::c_int = moduleConvertKeySpecsFlags(
        flags as int64_t,
        0 as libc::c_int,
    ) as libc::c_int;
    if ACLUserCheckKeyPerm(
        (*user).user,
        (*key).ptr as *const libc::c_char,
        sdslen((*key).ptr as sds) as libc::c_int,
        keyspec_flags,
    ) != 0 as libc::c_int
    {
        *__errno_location() = 13 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ACLCheckChannelPermissions(
    mut user: *mut RedisModuleUser,
    mut ch: *mut robj,
    mut flags: libc::c_int,
) -> libc::c_int {
    let allow_mask: libc::c_int = ((1 as libc::c_ulonglong) << 1 as libc::c_int
        | (1 as libc::c_ulonglong) << 2 as libc::c_int
        | (1 as libc::c_ulonglong) << 3 as libc::c_int
        | (1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_int;
    if flags & allow_mask != flags {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    let mut is_pattern: libc::c_int = (flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_int;
    if ACLUserCheckChannelPerm((*user).user, (*ch).ptr as sds, is_pattern)
        != 0 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ACLAddLogEntry(
    mut ctx: *mut RedisModuleCtx,
    mut user: *mut RedisModuleUser,
    mut object: *mut robj,
    mut reason: RedisModuleACLLogEntryReason,
) -> libc::c_int {
    let mut acl_reason: libc::c_int = 0;
    match reason as libc::c_uint {
        0 => {
            acl_reason = 3 as libc::c_int;
        }
        2 => {
            acl_reason = 2 as libc::c_int;
        }
        3 => {
            acl_reason = 4 as libc::c_int;
        }
        1 => {
            acl_reason = 1 as libc::c_int;
        }
        _ => return 1 as libc::c_int,
    }
    addACLLogEntry(
        (*ctx).client,
        acl_reason,
        3 as libc::c_int,
        -(1 as libc::c_int),
        (*(*user).user).name,
        sdsdup((*object).ptr as sds),
    );
    return 0 as libc::c_int;
}
unsafe extern "C" fn authenticateClientWithUser(
    mut ctx: *mut RedisModuleCtx,
    mut user: *mut user,
    mut callback: RedisModuleUserChangedFunc,
    mut privdata: *mut libc::c_void,
    mut client_id: *mut uint64_t,
) -> libc::c_int {
    if (*user).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_uint != 0 {
        return 1 as libc::c_int;
    }
    if ((*ctx).client).is_null()
        || (*(*ctx).client).flags
            & ((1 as libc::c_int) << 27 as libc::c_int) as libc::c_ulong != 0
    {
        return 1 as libc::c_int;
    }
    moduleNotifyUserChanged((*ctx).client);
    (*(*ctx).client).user = user;
    (*(*ctx).client).authenticated = 1 as libc::c_int;
    if callback.is_some() {
        (*(*ctx).client).auth_callback = callback;
        (*(*ctx).client).auth_callback_privdata = privdata;
        (*(*ctx).client).auth_module = (*ctx).module as *mut libc::c_void;
    }
    if !client_id.is_null() {
        *client_id = (*(*ctx).client).id;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_AuthenticateClientWithUser(
    mut ctx: *mut RedisModuleCtx,
    mut module_user: *mut RedisModuleUser,
    mut callback: RedisModuleUserChangedFunc,
    mut privdata: *mut libc::c_void,
    mut client_id: *mut uint64_t,
) -> libc::c_int {
    return authenticateClientWithUser(
        ctx,
        (*module_user).user,
        callback,
        privdata,
        client_id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_AuthenticateClientWithACLUser(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut len: size_t,
    mut callback: RedisModuleUserChangedFunc,
    mut privdata: *mut libc::c_void,
    mut client_id: *mut uint64_t,
) -> libc::c_int {
    let mut acl_user: *mut user = ACLGetUserByName(name, len);
    if acl_user.is_null() {
        return 1 as libc::c_int;
    }
    return authenticateClientWithUser(ctx, acl_user, callback, privdata, client_id);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DeauthenticateAndCloseClient(
    mut ctx: *mut RedisModuleCtx,
    mut client_id: uint64_t,
) -> libc::c_int {
    let mut c: *mut client = lookupClientByID(client_id);
    if c.is_null() {
        return 1 as libc::c_int;
    }
    revokeClientAuthentication(c);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RedactClientCommandArgument(
    mut ctx: *mut RedisModuleCtx,
    mut pos: libc::c_int,
) -> libc::c_int {
    if ctx.is_null() || ((*ctx).client).is_null() || pos <= 0 as libc::c_int
        || (*(*ctx).client).argc <= pos
    {
        return 1 as libc::c_int;
    }
    redactClientCommandArgument((*ctx).client, pos);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetClientCertificate(
    mut ctx: *mut RedisModuleCtx,
    mut client_id: uint64_t,
) -> *mut robj {
    let mut c: *mut client = lookupClientByID(client_id);
    if c.is_null() {
        return 0 as *mut robj;
    }
    let mut cert: sds = connTLSGetPeerCert((*c).conn);
    if cert.is_null() {
        return 0 as *mut robj;
    }
    let mut s: *mut robj = createObject(0 as libc::c_int, cert as *mut libc::c_void);
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, s as *mut libc::c_void);
    }
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CreateDict(
    mut ctx: *mut RedisModuleCtx,
) -> *mut RedisModuleDict {
    let mut d: *mut RedisModuleDict = zmalloc(
        core::mem::size_of::<RedisModuleDict>() as libc::c_ulong,
    ) as *mut RedisModuleDict;
    (*d).rax = raxNew();
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 4 as libc::c_int, d as *mut libc::c_void);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeDict(
    mut ctx: *mut RedisModuleCtx,
    mut d: *mut RedisModuleDict,
) {
    if !ctx.is_null() {
        autoMemoryFreed(ctx, 4 as libc::c_int, d as *mut libc::c_void);
    }
    raxFree((*d).rax);
    zfree(d as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictSize(mut d: *mut RedisModuleDict) -> uint64_t {
    return raxSize((*d).rax);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictSetC(
    mut d: *mut RedisModuleDict,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    let mut retval: libc::c_int = raxTryInsert(
        (*d).rax,
        key as *mut libc::c_uchar,
        keylen,
        ptr,
        0 as *mut *mut libc::c_void,
    );
    return if retval == 1 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictReplaceC(
    mut d: *mut RedisModuleDict,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    let mut retval: libc::c_int = raxInsert(
        (*d).rax,
        key as *mut libc::c_uchar,
        keylen,
        ptr,
        0 as *mut *mut libc::c_void,
    );
    return if retval == 1 as libc::c_int { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictSet(
    mut d: *mut RedisModuleDict,
    mut key: *mut robj,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    return RM_DictSetC(d, (*key).ptr, sdslen((*key).ptr as sds), ptr);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictReplace(
    mut d: *mut RedisModuleDict,
    mut key: *mut robj,
    mut ptr: *mut libc::c_void,
) -> libc::c_int {
    return RM_DictReplaceC(d, (*key).ptr, sdslen((*key).ptr as sds), ptr);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictGetC(
    mut d: *mut RedisModuleDict,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
    mut nokey: *mut libc::c_int,
) -> *mut libc::c_void {
    let mut res: *mut libc::c_void = raxFind(
        (*d).rax,
        key as *mut libc::c_uchar,
        keylen,
    );
    if !nokey.is_null() {
        *nokey = (res == raxNotFound) as libc::c_int;
    }
    return if res == raxNotFound { 0 as *mut libc::c_void } else { res };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictGet(
    mut d: *mut RedisModuleDict,
    mut key: *mut robj,
    mut nokey: *mut libc::c_int,
) -> *mut libc::c_void {
    return RM_DictGetC(d, (*key).ptr, sdslen((*key).ptr as sds), nokey);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictDelC(
    mut d: *mut RedisModuleDict,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
    mut oldval: *mut libc::c_void,
) -> libc::c_int {
    let mut retval: libc::c_int = raxRemove(
        (*d).rax,
        key as *mut libc::c_uchar,
        keylen,
        oldval as *mut *mut libc::c_void,
    );
    return if retval != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictDel(
    mut d: *mut RedisModuleDict,
    mut key: *mut robj,
    mut oldval: *mut libc::c_void,
) -> libc::c_int {
    return RM_DictDelC(d, (*key).ptr, sdslen((*key).ptr as sds), oldval);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictIteratorStartC(
    mut d: *mut RedisModuleDict,
    mut op: *const libc::c_char,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
) -> *mut RedisModuleDictIter {
    let mut di: *mut RedisModuleDictIter = zmalloc(
        core::mem::size_of::<RedisModuleDictIter>() as libc::c_ulong,
    ) as *mut RedisModuleDictIter;
    (*di).dict = d;
    raxStart(&mut (*di).ri, (*d).rax);
    raxSeek(&mut (*di).ri, op, key as *mut libc::c_uchar, keylen);
    return di;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictIteratorStart(
    mut d: *mut RedisModuleDict,
    mut op: *const libc::c_char,
    mut key: *mut robj,
) -> *mut RedisModuleDictIter {
    return RM_DictIteratorStartC(d, op, (*key).ptr, sdslen((*key).ptr as sds));
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictIteratorStop(mut di: *mut RedisModuleDictIter) {
    raxStop(&mut (*di).ri);
    zfree(di as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictIteratorReseekC(
    mut di: *mut RedisModuleDictIter,
    mut op: *const libc::c_char,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    return raxSeek(&mut (*di).ri, op, key as *mut libc::c_uchar, keylen);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictIteratorReseek(
    mut di: *mut RedisModuleDictIter,
    mut op: *const libc::c_char,
    mut key: *mut robj,
) -> libc::c_int {
    return RM_DictIteratorReseekC(di, op, (*key).ptr, sdslen((*key).ptr as sds));
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictNextC(
    mut di: *mut RedisModuleDictIter,
    mut keylen: *mut size_t,
    mut dataptr: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    if raxNext(&mut (*di).ri) == 0 {
        return 0 as *mut libc::c_void;
    }
    if !keylen.is_null() {
        *keylen = (*di).ri.key_len;
    }
    if !dataptr.is_null() {
        *dataptr = (*di).ri.data;
    }
    return (*di).ri.key as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictPrevC(
    mut di: *mut RedisModuleDictIter,
    mut keylen: *mut size_t,
    mut dataptr: *mut *mut libc::c_void,
) -> *mut libc::c_void {
    if raxPrev(&mut (*di).ri) == 0 {
        return 0 as *mut libc::c_void;
    }
    if !keylen.is_null() {
        *keylen = (*di).ri.key_len;
    }
    if !dataptr.is_null() {
        *dataptr = (*di).ri.data;
    }
    return (*di).ri.key as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictNext(
    mut ctx: *mut RedisModuleCtx,
    mut di: *mut RedisModuleDictIter,
    mut dataptr: *mut *mut libc::c_void,
) -> *mut robj {
    let mut keylen: size_t = 0;
    let mut key: *mut libc::c_void = RM_DictNextC(di, &mut keylen, dataptr);
    if key.is_null() {
        return 0 as *mut robj;
    }
    return RM_CreateString(ctx, key as *const libc::c_char, keylen);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictPrev(
    mut ctx: *mut RedisModuleCtx,
    mut di: *mut RedisModuleDictIter,
    mut dataptr: *mut *mut libc::c_void,
) -> *mut robj {
    let mut keylen: size_t = 0;
    let mut key: *mut libc::c_void = RM_DictPrevC(di, &mut keylen, dataptr);
    if key.is_null() {
        return 0 as *mut robj;
    }
    return RM_CreateString(ctx, key as *const libc::c_char, keylen);
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictCompareC(
    mut di: *mut RedisModuleDictIter,
    mut op: *const libc::c_char,
    mut key: *mut libc::c_void,
    mut keylen: size_t,
) -> libc::c_int {
    if raxEOF(&mut (*di).ri) != 0 {
        return 1 as libc::c_int;
    }
    let mut res: libc::c_int = raxCompare(
        &mut (*di).ri,
        op,
        key as *mut libc::c_uchar,
        keylen,
    );
    return if res != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_DictCompare(
    mut di: *mut RedisModuleDictIter,
    mut op: *const libc::c_char,
    mut key: *mut robj,
) -> libc::c_int {
    if raxEOF(&mut (*di).ri) != 0 {
        return 1 as libc::c_int;
    }
    let mut res: libc::c_int = raxCompare(
        &mut (*di).ri,
        op,
        (*key).ptr as *mut libc::c_uchar,
        sdslen((*key).ptr as sds),
    );
    return if res != 0 { 0 as libc::c_int } else { 1 as libc::c_int };
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddSection(
    mut ctx: *mut RedisModuleInfoCtx,
    mut name: *const libc::c_char,
) -> libc::c_int {
    let mut full_name: sds = sdsdup((*(*ctx).module).name);
    if !name.is_null() && strlen(name) > 0 as libc::c_int as libc::c_ulong {
        full_name = sdscatfmt(
            full_name,
            b"_%s\0" as *const u8 as *const libc::c_char,
            name,
        );
    }
    if (*ctx).in_dict_field != 0 {
        RM_InfoEndDictField(ctx);
    }
    if !((*ctx).requested_sections).is_null() {
        if (full_name.is_null()
            || (dictFind((*ctx).requested_sections, full_name as *const libc::c_void))
                .is_null())
            && (dictFind(
                (*ctx).requested_sections,
                (*(*ctx).module).name as *const libc::c_void,
            ))
                .is_null()
        {
            sdsfree(full_name);
            (*ctx).in_section = 0 as libc::c_int;
            return 1 as libc::c_int;
        }
    }
    let fresh33 = (*ctx).sections;
    (*ctx).sections = (*ctx).sections + 1;
    if fresh33 != 0 {
        (*ctx).info = sdscat((*ctx).info, b"\r\n\0" as *const u8 as *const libc::c_char);
    }
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"# %S\r\n\0" as *const u8 as *const libc::c_char,
        full_name,
    );
    (*ctx).in_section = 1 as libc::c_int;
    sdsfree(full_name);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoBeginDictField(
    mut ctx: *mut RedisModuleInfoCtx,
    mut name: *const libc::c_char,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        RM_InfoEndDictField(ctx);
    }
    let mut tmpmodname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmpname: *mut libc::c_char = 0 as *mut libc::c_char;
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"%s_%s:\0" as *const u8 as *const libc::c_char,
        getSafeInfoString(
            (*(*ctx).module).name,
            strlen((*(*ctx).module).name),
            &mut tmpmodname,
        ),
        getSafeInfoString(name, strlen(name), &mut tmpname),
    );
    if !tmpmodname.is_null() {
        zfree(tmpmodname as *mut libc::c_void);
    }
    if !tmpname.is_null() {
        zfree(tmpname as *mut libc::c_void);
    }
    (*ctx).in_dict_field = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoEndDictField(
    mut ctx: *mut RedisModuleInfoCtx,
) -> libc::c_int {
    if (*ctx).in_dict_field == 0 {
        return 1 as libc::c_int;
    }
    if *((*ctx).info)
        .offset(
            (sdslen((*ctx).info)).wrapping_sub(1 as libc::c_int as libc::c_ulong)
                as isize,
        ) as libc::c_int == ',' as i32
    {
        sdsIncrLen((*ctx).info, -(1 as libc::c_int) as ssize_t);
    }
    (*ctx).info = sdscat((*ctx).info, b"\r\n\0" as *const u8 as *const libc::c_char);
    (*ctx).in_dict_field = 0 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddFieldString(
    mut ctx: *mut RedisModuleInfoCtx,
    mut field: *const libc::c_char,
    mut value: *mut robj,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        (*ctx)
            .info = sdscatfmt(
            (*ctx).info,
            b"%s=%S,\0" as *const u8 as *const libc::c_char,
            field,
            (*value).ptr as sds,
        );
        return 0 as libc::c_int;
    }
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"%s_%s:%S\r\n\0" as *const u8 as *const libc::c_char,
        (*(*ctx).module).name,
        field,
        (*value).ptr as sds,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddFieldCString(
    mut ctx: *mut RedisModuleInfoCtx,
    mut field: *const libc::c_char,
    mut value: *const libc::c_char,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        (*ctx)
            .info = sdscatfmt(
            (*ctx).info,
            b"%s=%s,\0" as *const u8 as *const libc::c_char,
            field,
            value,
        );
        return 0 as libc::c_int;
    }
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"%s_%s:%s\r\n\0" as *const u8 as *const libc::c_char,
        (*(*ctx).module).name,
        field,
        value,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddFieldDouble(
    mut ctx: *mut RedisModuleInfoCtx,
    mut field: *const libc::c_char,
    mut value: libc::c_double,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        (*ctx)
            .info = sdscatprintf(
            (*ctx).info,
            b"%s=%.17g,\0" as *const u8 as *const libc::c_char,
            field,
            value,
        );
        return 0 as libc::c_int;
    }
    (*ctx)
        .info = sdscatprintf(
        (*ctx).info,
        b"%s_%s:%.17g\r\n\0" as *const u8 as *const libc::c_char,
        (*(*ctx).module).name,
        field,
        value,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddFieldLongLong(
    mut ctx: *mut RedisModuleInfoCtx,
    mut field: *const libc::c_char,
    mut value: libc::c_longlong,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        (*ctx)
            .info = sdscatfmt(
            (*ctx).info,
            b"%s=%I,\0" as *const u8 as *const libc::c_char,
            field,
            value,
        );
        return 0 as libc::c_int;
    }
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"%s_%s:%I\r\n\0" as *const u8 as *const libc::c_char,
        (*(*ctx).module).name,
        field,
        value,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_InfoAddFieldULongLong(
    mut ctx: *mut RedisModuleInfoCtx,
    mut field: *const libc::c_char,
    mut value: libc::c_ulonglong,
) -> libc::c_int {
    if (*ctx).in_section == 0 {
        return 1 as libc::c_int;
    }
    if (*ctx).in_dict_field != 0 {
        (*ctx)
            .info = sdscatfmt(
            (*ctx).info,
            b"%s=%U,\0" as *const u8 as *const libc::c_char,
            field,
            value,
        );
        return 0 as libc::c_int;
    }
    (*ctx)
        .info = sdscatfmt(
        (*ctx).info,
        b"%s_%s:%U\r\n\0" as *const u8 as *const libc::c_char,
        (*(*ctx).module).name,
        field,
        value,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterInfoFunc(
    mut ctx: *mut RedisModuleCtx,
    mut cb: RedisModuleInfoFunc,
) -> libc::c_int {
    (*(*ctx).module).info_cb = cb;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modulesCollectInfo(
    mut info: sds,
    mut sections_dict: *mut dict,
    mut for_crash_report: libc::c_int,
    mut sections: libc::c_int,
) -> sds {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        if ((*module).info_cb).is_none() {
            continue;
        }
        let mut info_ctx: RedisModuleInfoCtx = {
            let mut init = RedisModuleInfoCtx {
                module: module,
                requested_sections: sections_dict,
                info: info,
                sections: sections,
                in_section: 0 as libc::c_int,
                in_dict_field: 0 as libc::c_int,
            };
            init
        };
        ((*module).info_cb)
            .expect("non-null function pointer")(&mut info_ctx, for_crash_report);
        if info_ctx.in_dict_field != 0 {
            RM_InfoEndDictField(&mut info_ctx);
        }
        info = info_ctx.info;
        sections = info_ctx.sections;
    }
    dictReleaseIterator(di);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetServerInfo(
    mut ctx: *mut RedisModuleCtx,
    mut section: *const libc::c_char,
) -> *mut RedisModuleServerInfoData {
    let mut d: *mut RedisModuleServerInfoData = zmalloc(
        core::mem::size_of::<RedisModuleServerInfoData>() as libc::c_ulong,
    ) as *mut RedisModuleServerInfoData;
    (*d).rax = raxNew();
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 5 as libc::c_int, d as *mut libc::c_void);
    }
    let mut all: libc::c_int = 0 as libc::c_int;
    let mut everything: libc::c_int = 0 as libc::c_int;
    let mut argv: [*mut robj; 1] = [0 as *mut robj; 1];
    argv[0 as libc::c_int
        as usize] = if !section.is_null() {
        createStringObject(section, strlen(section))
    } else {
        0 as *mut robj
    };
    let mut section_dict: *mut dict = genInfoSectionDict(
        argv.as_mut_ptr(),
        if !section.is_null() { 1 as libc::c_int } else { 0 as libc::c_int },
        0 as *mut *mut libc::c_char,
        &mut all,
        &mut everything,
    );
    let mut info: sds = genRedisInfoString(section_dict, all, everything);
    let mut totlines: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut lines: *mut sds = sdssplitlen(
        info as *const libc::c_char,
        sdslen(info) as ssize_t,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        &mut totlines,
    );
    i = 0 as libc::c_int;
    while i < totlines {
        let mut line: sds = *lines.offset(i as isize);
        if !(*line.offset(0 as libc::c_int as isize) as libc::c_int == '#' as i32) {
            let mut sep: *mut libc::c_char = strchr(
                line as *const libc::c_char,
                ':' as i32,
            );
            if !sep.is_null() {
                let mut key: *mut libc::c_uchar = line as *mut libc::c_uchar;
                let mut keylen: size_t = (sep as intptr_t - line as intptr_t) as size_t;
                let mut val: sds = sdsnewlen(
                    sep.offset(1 as libc::c_int as isize) as *const libc::c_void,
                    (sdslen(line))
                        .wrapping_sub(
                            (sep as intptr_t - line as intptr_t) as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                );
                if raxTryInsert(
                    (*d).rax,
                    key,
                    keylen,
                    val as *mut libc::c_void,
                    0 as *mut *mut libc::c_void,
                ) == 0
                {
                    sdsfree(val);
                }
            }
        }
        i += 1;
    }
    sdsfree(info);
    sdsfreesplitres(lines, totlines);
    releaseInfoSectionDict(section_dict);
    if !(argv[0 as libc::c_int as usize]).is_null() {
        decrRefCount(argv[0 as libc::c_int as usize]);
    }
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn RM_FreeServerInfo(
    mut ctx: *mut RedisModuleCtx,
    mut data: *mut RedisModuleServerInfoData,
) {
    if !ctx.is_null() {
        autoMemoryFreed(ctx, 5 as libc::c_int, data as *mut libc::c_void);
    }
    raxFreeWithCallback(
        (*data).rax,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(sds) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdsfree as unsafe extern "C" fn(sds) -> ())),
    );
    zfree(data as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ServerInfoGetField(
    mut ctx: *mut RedisModuleCtx,
    mut data: *mut RedisModuleServerInfoData,
    mut field: *const libc::c_char,
) -> *mut robj {
    let mut val: sds = raxFind((*data).rax, field as *mut libc::c_uchar, strlen(field))
        as sds;
    if val == raxNotFound as sds {
        return 0 as *mut robj;
    }
    let mut o: *mut robj = createStringObject(val as *const libc::c_char, sdslen(val));
    if !ctx.is_null() {
        autoMemoryAdd(ctx, 1 as libc::c_int, o as *mut libc::c_void);
    }
    return o;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ServerInfoGetFieldC(
    mut data: *mut RedisModuleServerInfoData,
    mut field: *const libc::c_char,
) -> *const libc::c_char {
    let mut val: sds = raxFind((*data).rax, field as *mut libc::c_uchar, strlen(field))
        as sds;
    if val == raxNotFound as sds {
        return 0 as *const libc::c_char;
    }
    return val as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ServerInfoGetFieldSigned(
    mut data: *mut RedisModuleServerInfoData,
    mut field: *const libc::c_char,
    mut out_err: *mut libc::c_int,
) -> libc::c_longlong {
    let mut ll: libc::c_longlong = 0;
    let mut val: sds = raxFind((*data).rax, field as *mut libc::c_uchar, strlen(field))
        as sds;
    if val == raxNotFound as sds {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_longlong;
    }
    if string2ll(val as *const libc::c_char, sdslen(val), &mut ll) == 0 {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_longlong;
    }
    if !out_err.is_null() {
        *out_err = 0 as libc::c_int;
    }
    return ll;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ServerInfoGetFieldUnsigned(
    mut data: *mut RedisModuleServerInfoData,
    mut field: *const libc::c_char,
    mut out_err: *mut libc::c_int,
) -> libc::c_ulonglong {
    let mut ll: libc::c_ulonglong = 0;
    let mut val: sds = raxFind((*data).rax, field as *mut libc::c_uchar, strlen(field))
        as sds;
    if val == raxNotFound as sds {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    if string2ull(val as *const libc::c_char, &mut ll) == 0 {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_ulonglong;
    }
    if !out_err.is_null() {
        *out_err = 0 as libc::c_int;
    }
    return ll;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ServerInfoGetFieldDouble(
    mut data: *mut RedisModuleServerInfoData,
    mut field: *const libc::c_char,
    mut out_err: *mut libc::c_int,
) -> libc::c_double {
    let mut dbl: libc::c_double = 0.;
    let mut val: sds = raxFind((*data).rax, field as *mut libc::c_uchar, strlen(field))
        as sds;
    if val == raxNotFound as sds {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_double;
    }
    if string2d(val as *const libc::c_char, sdslen(val), &mut dbl) == 0 {
        if !out_err.is_null() {
            *out_err = 1 as libc::c_int;
        }
        return 0 as libc::c_int as libc::c_double;
    }
    if !out_err.is_null() {
        *out_err = 0 as libc::c_int;
    }
    return dbl;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetRandomBytes(
    mut dst: *mut libc::c_uchar,
    mut len: size_t,
) {
    getRandomBytes(dst, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetRandomHexChars(
    mut dst: *mut libc::c_char,
    mut len: size_t,
) {
    getRandomHexChars(dst, len);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ExportSharedAPI(
    mut ctx: *mut RedisModuleCtx,
    mut apiname: *const libc::c_char,
    mut func: *mut libc::c_void,
) -> libc::c_int {
    let mut sapi: *mut RedisModuleSharedAPI = zmalloc(
        core::mem::size_of::<RedisModuleSharedAPI>() as libc::c_ulong,
    ) as *mut RedisModuleSharedAPI;
    (*sapi).module = (*ctx).module;
    (*sapi).func = func;
    if dictAdd(
        server.sharedapi,
        apiname as *mut libc::c_char as *mut libc::c_void,
        sapi as *mut libc::c_void,
    ) != 0 as libc::c_int
    {
        zfree(sapi as *mut libc::c_void);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetSharedAPI(
    mut ctx: *mut RedisModuleCtx,
    mut apiname: *const libc::c_char,
) -> *mut libc::c_void {
    let mut de: *mut dictEntry = dictFind(
        server.sharedapi,
        apiname as *const libc::c_void,
    );
    if de.is_null() {
        return 0 as *mut libc::c_void;
    }
    let mut sapi: *mut RedisModuleSharedAPI = (*de).v.val as *mut RedisModuleSharedAPI;
    if (listSearchKey((*(*sapi).module).usedby, (*ctx).module as *mut libc::c_void))
        .is_null()
    {
        listAddNodeTail((*(*sapi).module).usedby, (*ctx).module as *mut libc::c_void);
        listAddNodeTail((*(*ctx).module).using, (*sapi).module as *mut libc::c_void);
    }
    return (*sapi).func;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnregisterSharedAPI(
    mut module: *mut RedisModule,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut di: *mut dictIterator = dictGetSafeIterator(server.sharedapi);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut apiname: *const libc::c_char = (*de).key as *const libc::c_char;
        let mut sapi: *mut RedisModuleSharedAPI = (*de).v.val
            as *mut RedisModuleSharedAPI;
        if (*sapi).module == module {
            dictDelete(server.sharedapi, apiname as *const libc::c_void);
            zfree(sapi as *mut libc::c_void);
            count += 1;
        }
    }
    dictReleaseIterator(di);
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnregisterUsedAPI(
    mut module: *mut RedisModule,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut count: libc::c_int = 0 as libc::c_int;
    listRewind((*module).using, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut used: *mut RedisModule = (*ln).value as *mut RedisModule;
        let mut ln_0: *mut listNode = listSearchKey(
            (*used).usedby,
            module as *mut libc::c_void,
        );
        if !ln_0.is_null() {
            listDelNode((*used).usedby, ln_0);
            count += 1;
        }
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnregisterFilters(
    mut module: *mut RedisModule,
) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut count: libc::c_int = 0 as libc::c_int;
    listRewind((*module).filters, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut filter: *mut RedisModuleCommandFilter = (*ln).value
            as *mut RedisModuleCommandFilter;
        let mut ln_0: *mut listNode = listSearchKey(
            moduleCommandFilters,
            filter as *mut libc::c_void,
        );
        if !ln_0.is_null() {
            listDelNode(moduleCommandFilters, ln_0);
            count += 1;
        }
        zfree(filter as *mut libc::c_void);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterCommandFilter(
    mut ctx: *mut RedisModuleCtx,
    mut callback: RedisModuleCommandFilterFunc,
    mut flags: libc::c_int,
) -> *mut RedisModuleCommandFilter {
    let mut filter: *mut RedisModuleCommandFilter = zmalloc(
        core::mem::size_of::<RedisModuleCommandFilter>() as libc::c_ulong,
    ) as *mut RedisModuleCommandFilter;
    (*filter).module = (*ctx).module;
    (*filter).callback = callback;
    (*filter).flags = flags;
    listAddNodeTail(moduleCommandFilters, filter as *mut libc::c_void);
    listAddNodeTail((*(*ctx).module).filters, filter as *mut libc::c_void);
    return filter;
}
#[no_mangle]
pub unsafe extern "C" fn RM_UnregisterCommandFilter(
    mut ctx: *mut RedisModuleCtx,
    mut filter: *mut RedisModuleCommandFilter,
) -> libc::c_int {
    let mut ln: *mut listNode = 0 as *mut listNode;
    if (*filter).module != (*ctx).module {
        return 1 as libc::c_int;
    }
    ln = listSearchKey(moduleCommandFilters, filter as *mut libc::c_void);
    if ln.is_null() {
        return 1 as libc::c_int;
    }
    listDelNode(moduleCommandFilters, ln);
    ln = listSearchKey((*(*ctx).module).filters, filter as *mut libc::c_void);
    if ln.is_null() {
        return 1 as libc::c_int;
    }
    listDelNode((*(*ctx).module).filters, ln);
    zfree(filter as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleCallCommandFilters(mut c: *mut client) {
    if (*moduleCommandFilters).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(moduleCommandFilters, &mut li);
    let mut filter: RedisModuleCommandFilterCtx = {
        let mut init = RedisModuleCommandFilterCtx {
            argv: (*c).argv,
            argv_len: (*c).argv_len,
            argc: (*c).argc,
        };
        init
    };
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut f: *mut RedisModuleCommandFilter = (*ln).value
            as *mut RedisModuleCommandFilter;
        if (*f).flags & (1 as libc::c_int) << 0 as libc::c_int != 0
            && (*(*f).module).in_call != 0
        {
            continue;
        }
        ((*f).callback).expect("non-null function pointer")(&mut filter);
    }
    (*c).argv = filter.argv;
    (*c).argv_len = filter.argv_len;
    (*c).argc = filter.argc;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CommandFilterArgsCount(
    mut fctx: *mut RedisModuleCommandFilterCtx,
) -> libc::c_int {
    return (*fctx).argc;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CommandFilterArgGet(
    mut fctx: *mut RedisModuleCommandFilterCtx,
    mut pos: libc::c_int,
) -> *mut robj {
    if pos < 0 as libc::c_int || pos >= (*fctx).argc {
        return 0 as *mut robj;
    }
    return *((*fctx).argv).offset(pos as isize);
}
#[no_mangle]
pub unsafe extern "C" fn RM_CommandFilterArgInsert(
    mut fctx: *mut RedisModuleCommandFilterCtx,
    mut pos: libc::c_int,
    mut arg: *mut robj,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if pos < 0 as libc::c_int || pos > (*fctx).argc {
        return 1 as libc::c_int;
    }
    if (*fctx).argv_len < (*fctx).argc + 1 as libc::c_int {
        (*fctx).argv_len = (*fctx).argc + 1 as libc::c_int;
        (*fctx)
            .argv = zrealloc(
            (*fctx).argv as *mut libc::c_void,
            ((*fctx).argv_len as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<*mut robj>() as libc::c_ulong),
        ) as *mut *mut robj;
    }
    i = (*fctx).argc;
    while i > pos {
        let ref mut fresh34 = *((*fctx).argv).offset(i as isize);
        *fresh34 = *((*fctx).argv).offset((i - 1 as libc::c_int) as isize);
        i -= 1;
    }
    let ref mut fresh35 = *((*fctx).argv).offset(pos as isize);
    *fresh35 = arg;
    (*fctx).argc += 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CommandFilterArgReplace(
    mut fctx: *mut RedisModuleCommandFilterCtx,
    mut pos: libc::c_int,
    mut arg: *mut robj,
) -> libc::c_int {
    if pos < 0 as libc::c_int || pos >= (*fctx).argc {
        return 1 as libc::c_int;
    }
    decrRefCount(*((*fctx).argv).offset(pos as isize));
    let ref mut fresh36 = *((*fctx).argv).offset(pos as isize);
    *fresh36 = arg;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_CommandFilterArgDelete(
    mut fctx: *mut RedisModuleCommandFilterCtx,
    mut pos: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if pos < 0 as libc::c_int || pos >= (*fctx).argc {
        return 1 as libc::c_int;
    }
    decrRefCount(*((*fctx).argv).offset(pos as isize));
    i = pos;
    while i < (*fctx).argc - 1 as libc::c_int {
        let ref mut fresh37 = *((*fctx).argv).offset(i as isize);
        *fresh37 = *((*fctx).argv).offset((i + 1 as libc::c_int) as isize);
        i += 1;
    }
    (*fctx).argc -= 1;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_MallocSize(mut ptr: *mut libc::c_void) -> size_t {
    return je_malloc_usable_size(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn RM_MallocUsableSize(mut ptr: *mut libc::c_void) -> size_t {
    return je_malloc_usable_size(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn RM_MallocSizeString(mut str: *mut robj) -> size_t {
    if (*str).type_0() as libc::c_int == 0 as libc::c_int {} else {
        _serverAssert(
            b"str->type == OBJ_STRING\0" as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            9960 as libc::c_int,
        );
        unreachable!();
    };
    return (core::mem::size_of::<robj>() as libc::c_ulong)
        .wrapping_add(getStringObjectSdsUsedMemory(str));
}
#[no_mangle]
pub unsafe extern "C" fn RM_MallocSizeDict(mut dict: *mut RedisModuleDict) -> size_t {
    let mut size: size_t = (core::mem::size_of::<RedisModuleDict>() as libc::c_ulong)
        .wrapping_add(core::mem::size_of::<rax>() as libc::c_ulong);
    size = (size as libc::c_ulong)
        .wrapping_add(
            ((*(*dict).rax).numnodes)
                .wrapping_mul(core::mem::size_of::<raxNode>() as libc::c_ulong),
        ) as size_t as size_t;
    size = (size as libc::c_ulong)
        .wrapping_add(
            ((*(*dict).rax).numnodes)
                .wrapping_mul(core::mem::size_of::<libc::c_long>() as libc::c_ulong)
                .wrapping_mul(30 as libc::c_int as libc::c_ulong),
        ) as size_t as size_t;
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetUsedMemoryRatio() -> libc::c_float {
    let mut level: libc::c_float = 0.;
    getMaxmemoryState(0 as *mut size_t, 0 as *mut size_t, 0 as *mut size_t, &mut level);
    return level;
}
unsafe extern "C" fn moduleScanCallback(
    mut privdata: *mut libc::c_void,
    mut de: *const dictEntry,
) {
    let mut data: *mut ScanCBData = privdata as *mut ScanCBData;
    let mut key: sds = (*de).key as sds;
    let mut val: *mut robj = (*de).v.val as *mut robj;
    let mut keyname: *mut robj = createObject(
        0 as libc::c_int,
        sdsdup(key) as *mut libc::c_void,
    );
    let mut kp: RedisModuleKey = {
        let mut init = RedisModuleKey {
            ctx: 0 as *mut RedisModuleCtx,
            db: 0 as *mut redisDb,
            key: 0 as *mut robj,
            value: 0 as *mut robj,
            iter: 0 as *mut libc::c_void,
            mode: 0,
            u: C2RustUnnamed_20 {
                list: C2RustUnnamed_23 {
                    entry: listTypeEntry {
                        li: 0 as *mut listTypeIterator,
                        entry: quicklistEntry {
                            quicklist: 0 as *const quicklist,
                            node: 0 as *mut quicklistNode,
                            zi: 0 as *mut libc::c_uchar,
                            value: 0 as *mut libc::c_uchar,
                            longval: 0,
                            sz: 0,
                            offset: 0,
                        },
                    },
                    index: 0,
                },
            },
        };
        init
    };
    moduleInitKey(
        &mut kp,
        (*data).ctx,
        keyname,
        val,
        (1 as libc::c_int) << 0 as libc::c_int,
    );
    ((*data).fn_0)
        .expect(
            "non-null function pointer",
        )((*data).ctx, keyname, &mut kp, (*data).user_data);
    moduleCloseKey(&mut kp);
    decrRefCount(keyname);
}
#[no_mangle]
pub unsafe extern "C" fn RM_ScanCursorCreate() -> *mut RedisModuleScanCursor {
    let mut cursor: *mut RedisModuleScanCursor = zmalloc(
        core::mem::size_of::<RedisModuleScanCursor>() as libc::c_ulong,
    ) as *mut RedisModuleScanCursor;
    (*cursor).cursor = 0 as libc::c_int as libc::c_ulong;
    (*cursor).done = 0 as libc::c_int;
    return cursor;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ScanCursorRestart(mut cursor: *mut RedisModuleScanCursor) {
    (*cursor).cursor = 0 as libc::c_int as libc::c_ulong;
    (*cursor).done = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ScanCursorDestroy(mut cursor: *mut RedisModuleScanCursor) {
    zfree(cursor as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn RM_Scan(
    mut ctx: *mut RedisModuleCtx,
    mut cursor: *mut RedisModuleScanCursor,
    mut fn_0: RedisModuleScanCB,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    if (*cursor).done != 0 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = 1 as libc::c_int;
    let mut data: ScanCBData = {
        let mut init = ScanCBData {
            ctx: ctx,
            user_data: privdata,
            fn_0: fn_0,
        };
        init
    };
    (*cursor)
        .cursor = dictScan(
        (*(*(*ctx).client).db).dict,
        (*cursor).cursor,
        Some(
            moduleScanCallback
                as unsafe extern "C" fn(*mut libc::c_void, *const dictEntry) -> (),
        ),
        None,
        &mut data as *mut ScanCBData as *mut libc::c_void,
    );
    if (*cursor).cursor == 0 as libc::c_int as libc::c_ulong {
        (*cursor).done = 1 as libc::c_int;
        ret = 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    return ret;
}
unsafe extern "C" fn moduleScanKeyCallback(
    mut privdata: *mut libc::c_void,
    mut de: *const dictEntry,
) {
    let mut data: *mut ScanKeyCBData = privdata as *mut ScanKeyCBData;
    let mut key: sds = (*de).key as sds;
    let mut o: *mut robj = (*(*data).key).value;
    let mut field: *mut robj = createStringObject(
        key as *const libc::c_char,
        sdslen(key),
    );
    let mut value: *mut robj = 0 as *mut robj;
    if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        value = 0 as *mut robj;
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        let mut val: sds = (*de).v.val as sds;
        value = createStringObject(val as *const libc::c_char, sdslen(val));
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        let mut val_0: *mut libc::c_double = (*de).v.val as *mut libc::c_double;
        value = createStringObjectFromLongDouble(
            (*val_0) as f64,
            0 as libc::c_int,
        );
    }
    ((*data).fn_0)
        .expect(
            "non-null function pointer",
        )((*data).key, field, value, (*data).user_data);
    decrRefCount(field);
    if !value.is_null() {
        decrRefCount(value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn RM_ScanKey(
    mut key: *mut RedisModuleKey,
    mut cursor: *mut RedisModuleScanCursor,
    mut fn_0: RedisModuleScanKeyCB,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    if key.is_null() || ((*key).value).is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut ht: *mut dict = 0 as *mut dict;
    let mut o: *mut robj = (*key).value;
    if (*o).type_0() as libc::c_int == 2 as libc::c_int {
        if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            ht = (*o).ptr as *mut dict;
        }
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        if (*o).encoding() as libc::c_int == 2 as libc::c_int {
            ht = (*o).ptr as *mut dict;
        }
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        if (*o).encoding() as libc::c_int == 7 as libc::c_int {
            ht = (*((*o).ptr as *mut zset)).dict;
        }
    } else {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int;
    }
    if (*cursor).done != 0 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as libc::c_int;
    }
    let mut ret: libc::c_int = 1 as libc::c_int;
    if !ht.is_null() {
        let mut data: ScanKeyCBData = {
            let mut init = ScanKeyCBData {
                key: key,
                user_data: privdata,
                fn_0: fn_0,
            };
            init
        };
        (*cursor)
            .cursor = dictScan(
            ht,
            (*cursor).cursor,
            Some(
                moduleScanKeyCallback
                    as unsafe extern "C" fn(*mut libc::c_void, *const dictEntry) -> (),
            ),
            None,
            &mut data as *mut ScanKeyCBData as *mut libc::c_void,
        );
        if (*cursor).cursor == 0 as libc::c_int as libc::c_ulong {
            (*cursor).done = 1 as libc::c_int;
            ret = 0 as libc::c_int;
        }
    } else if (*o).type_0() as libc::c_int == 2 as libc::c_int
        && (*o).encoding() as libc::c_int == 6 as libc::c_int
    {
        let mut pos: libc::c_int = 0 as libc::c_int;
        let mut ll: int64_t = 0;
        loop {
            let fresh38 = pos;
            pos = pos + 1;
            if !(intsetGet((*o).ptr as *mut intset, fresh38 as uint32_t, &mut ll) != 0) {
                break;
            }
            let mut field: *mut robj = createObject(
                0 as libc::c_int,
                sdsfromlonglong(ll as libc::c_longlong) as *mut libc::c_void,
            );
            fn_0
                .expect(
                    "non-null function pointer",
                )(key, field, 0 as *mut robj, privdata);
            decrRefCount(field);
        }
        (*cursor).cursor = 1 as libc::c_int as libc::c_ulong;
        (*cursor).done = 1 as libc::c_int;
        ret = 0 as libc::c_int;
    } else if (*o).type_0() as libc::c_int == 3 as libc::c_int {
        let mut p: *mut libc::c_uchar = lpSeek(
            (*o).ptr as *mut libc::c_uchar,
            0 as libc::c_int as libc::c_long,
        );
        let mut vstr: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen: libc::c_uint = 0;
        let mut vll: libc::c_longlong = 0;
        while !p.is_null() {
            vstr = lpGetValue(p, &mut vlen, &mut vll);
            let mut field_0: *mut robj = if !vstr.is_null() {
                createStringObject(vstr as *mut libc::c_char, vlen as size_t)
            } else {
                createObject(0 as libc::c_int, sdsfromlonglong(vll) as *mut libc::c_void)
            };
            p = lpNext((*o).ptr as *mut libc::c_uchar, p);
            vstr = lpGetValue(p, &mut vlen, &mut vll);
            let mut value: *mut robj = if !vstr.is_null() {
                createStringObject(vstr as *mut libc::c_char, vlen as size_t)
            } else {
                createObject(0 as libc::c_int, sdsfromlonglong(vll) as *mut libc::c_void)
            };
            fn_0.expect("non-null function pointer")(key, field_0, value, privdata);
            p = lpNext((*o).ptr as *mut libc::c_uchar, p);
            decrRefCount(field_0);
            decrRefCount(value);
        }
        (*cursor).cursor = 1 as libc::c_int as libc::c_ulong;
        (*cursor).done = 1 as libc::c_int;
        ret = 0 as libc::c_int;
    } else if (*o).type_0() as libc::c_int == 4 as libc::c_int {
        let mut p_0: *mut libc::c_uchar = lpFirst((*o).ptr as *mut libc::c_uchar);
        let mut vstr_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut vlen_0: int64_t = 0;
        let mut intbuf: [libc::c_uchar; 21] = [0; 21];
        while !p_0.is_null() {
            vstr_0 = lpGet(p_0, &mut vlen_0, intbuf.as_mut_ptr());
            let mut field_1: *mut robj = createStringObject(
                vstr_0 as *mut libc::c_char,
                vlen_0 as size_t,
            );
            p_0 = lpNext((*o).ptr as *mut libc::c_uchar, p_0);
            vstr_0 = lpGet(p_0, &mut vlen_0, intbuf.as_mut_ptr());
            let mut value_0: *mut robj = createStringObject(
                vstr_0 as *mut libc::c_char,
                vlen_0 as size_t,
            );
            fn_0.expect("non-null function pointer")(key, field_1, value_0, privdata);
            p_0 = lpNext((*o).ptr as *mut libc::c_uchar, p_0);
            decrRefCount(field_1);
            decrRefCount(value_0);
        }
        (*cursor).cursor = 1 as libc::c_int as libc::c_ulong;
        (*cursor).done = 1 as libc::c_int;
        ret = 0 as libc::c_int;
    }
    *__errno_location() = 0 as libc::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn RM_Fork(
    mut cb: RedisModuleForkDoneHandler,
    mut user_data: *mut libc::c_void,
) -> libc::c_int {
    let mut childpid: pid_t = 0;
    childpid = redisFork(4 as libc::c_int);
    if childpid == 0 as libc::c_int {
        redisSetProcTitle(
            b"redis-module-fork\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    } else if childpid == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't fork for module: %s\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
    } else {
        moduleForkInfo.done_handler = cb;
        moduleForkInfo.done_handler_user_data = user_data;
        if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                1 as libc::c_int,
                b"Module fork started pid: %ld \0" as *const u8 as *const libc::c_char,
                childpid as libc::c_long,
            );
        }
    }
    return childpid;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SendChildHeartbeat(mut progress: libc::c_double) {
    sendChildInfoGeneric(
        CHILD_INFO_TYPE_CURRENT_INFO,
        0 as libc::c_int as size_t,
        progress,
        b"Module fork\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_ExitFromChild(mut retcode: libc::c_int) -> libc::c_int {
    sendChildCowInfo(
        CHILD_INFO_TYPE_MODULE_COW_SIZE,
        b"Module fork\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    exitFromChild(retcode);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn TerminateModuleForkChild(
    mut child_pid: libc::c_int,
    mut wait: libc::c_int,
) -> libc::c_int {
    if server.child_type != 4 as libc::c_int || server.child_pid != child_pid {
        return -(1 as libc::c_int);
    }
    let mut statloc: libc::c_int = 0;
    if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            1 as libc::c_int,
            b"Killing running module fork child: %ld\0" as *const u8
                as *const libc::c_char,
            server.child_pid as libc::c_long,
        );
    }
    if kill(server.child_pid, 10 as libc::c_int) != -(1 as libc::c_int) && wait != 0 {
        while waitpid(server.child_pid, &mut statloc, 0 as libc::c_int)
            != server.child_pid
        {}
    }
    resetChildState();
    moduleForkInfo.done_handler = None;
    moduleForkInfo.done_handler_user_data = 0 as *mut libc::c_void;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_KillForkChild(mut child_pid: libc::c_int) -> libc::c_int {
    if TerminateModuleForkChild(child_pid, 1 as libc::c_int) == 0 as libc::c_int {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn ModuleForkDoneHandler(
    mut exitcode: libc::c_int,
    mut bysignal: libc::c_int,
) {
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Module fork exited pid: %ld, retcode: %d, bysignal: %d\0" as *const u8
                as *const libc::c_char,
            server.child_pid as libc::c_long,
            exitcode,
            bysignal,
        );
    }
    if (moduleForkInfo.done_handler).is_some() {
        (moduleForkInfo.done_handler)
            .expect(
                "non-null function pointer",
            )(exitcode, bysignal, moduleForkInfo.done_handler_user_data);
    }
    moduleForkInfo.done_handler = None;
    moduleForkInfo.done_handler_user_data = 0 as *mut libc::c_void;
}
static mut moduleEventVersions: [uint64_t; 17] = [
    1 as libc::c_int as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    1 as libc::c_int as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    1 as libc::c_int as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    1 as libc::c_int as uint64_t,
    1 as libc::c_int as uint64_t,
    1 as libc::c_int as uint64_t,
    1 as libc::c_int as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
    -(1 as libc::c_int) as uint64_t,
];
#[no_mangle]
pub unsafe extern "C" fn RM_SubscribeToServerEvent(
    mut ctx: *mut RedisModuleCtx,
    mut event: RedisModuleEvent,
    mut callback: RedisModuleEventCallback,
) -> libc::c_int {
    let mut el: *mut RedisModuleEventListener = 0 as *mut RedisModuleEventListener;
    if ((*ctx).module).is_null() {
        return 1 as libc::c_int;
    }
    if event.id >= 17 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    if event.dataver > moduleEventVersions[event.id as usize] {
        return 1 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(RedisModule_EventListeners, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        el = (*ln).value as *mut RedisModuleEventListener;
        if (*el).module == (*ctx).module && (*el).event.id == event.id {
            break;
        }
    }
    if !ln.is_null() {
        if callback.is_none() {
            listDelNode(RedisModule_EventListeners, ln);
            zfree(el as *mut libc::c_void);
        } else {
            (*el).callback = callback;
        }
        return 0 as libc::c_int;
    }
    el = zmalloc(core::mem::size_of::<RedisModuleEventListener>() as libc::c_ulong)
        as *mut RedisModuleEventListener;
    (*el).module = (*ctx).module;
    (*el).event = event;
    (*el).callback = callback;
    listAddNodeTail(RedisModule_EventListeners, el as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_IsSubEventSupported(
    mut event: RedisModuleEvent,
    mut subevent: int64_t,
) -> libc::c_int {
    match event.id {
        0 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        1 => return (subevent < 6 as libc::c_int as libc::c_long) as libc::c_int,
        2 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        3 => return (subevent < 5 as libc::c_int as libc::c_long) as libc::c_int,
        4 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        5 => return (subevent < 0 as libc::c_int as libc::c_long) as libc::c_int,
        6 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        7 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        8 => return (subevent < 0 as libc::c_int as libc::c_long) as libc::c_int,
        9 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        10 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        11 => return (subevent < 0 as libc::c_int as libc::c_long) as libc::c_int,
        14 => return (subevent < 3 as libc::c_int as libc::c_long) as libc::c_int,
        13 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        15 => return (subevent < 2 as libc::c_int as libc::c_long) as libc::c_int,
        16 => return (subevent < 1 as libc::c_int as libc::c_long) as libc::c_int,
        _ => {}
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleFireServerEvent(
    mut eid: uint64_t,
    mut subid: libc::c_int,
    mut data: *mut libc::c_void,
) {
    if (*RedisModule_EventListeners).len == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(RedisModule_EventListeners, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut el: *mut RedisModuleEventListener = (*ln).value
            as *mut RedisModuleEventListener;
        if !((*el).event.id == eid) {
            continue;
        }
        let mut ctx: RedisModuleCtx = RedisModuleCtx {
            getapifuncptr: 0 as *mut libc::c_void,
            module: 0 as *mut RedisModule,
            client: 0 as *mut client,
            blocked_client: 0 as *mut RedisModuleBlockedClient,
            amqueue: 0 as *mut AutoMemEntry,
            amqueue_len: 0,
            amqueue_used: 0,
            flags: 0,
            postponed_arrays: 0 as *mut *mut libc::c_void,
            postponed_arrays_count: 0,
            blocked_privdata: 0 as *mut libc::c_void,
            blocked_ready_key: 0 as *mut robj,
            keys_result: 0 as *mut getKeysResult,
            pa_head: 0 as *mut RedisModulePoolAllocBlock,
            next_yield_time: 0,
            user: 0 as *const RedisModuleUser,
        };
        if eid == 4 as libc::c_int as libc::c_ulong {
            moduleCreateContext(&mut ctx, (*el).module, 0 as libc::c_int);
            ctx.client = data as *mut client;
        } else {
            moduleCreateContext(
                &mut ctx,
                (*el).module,
                (1 as libc::c_int) << 6 as libc::c_int,
            );
        }
        let mut moduledata: *mut libc::c_void = 0 as *mut libc::c_void;
        let mut civ1: RedisModuleClientInfoV1 = RedisModuleClientInfoV1 {
            version: 0,
            flags: 0,
            id: 0,
            addr: [0; 46],
            port: 0,
            db: 0,
        };
        let mut riv1: RedisModuleReplicationInfoV1 = RedisModuleReplicationInfoV1 {
            version: 0,
            master: 0,
            masterhost: 0 as *mut libc::c_char,
            masterport: 0,
            replid1: 0 as *mut libc::c_char,
            replid2: 0 as *mut libc::c_char,
            repl1_offset: 0,
            repl2_offset: 0,
        };
        let mut mcv1: RedisModuleModuleChangeV1 = RedisModuleModuleChangeV1 {
            version: 0,
            module_name: 0 as *const libc::c_char,
            module_version: 0,
        };
        if eid == 4 as libc::c_int as libc::c_ulong {
            if modulePopulateClientInfoStructure(
                &mut civ1 as *mut RedisModuleClientInfoV1 as *mut libc::c_void,
                data as *mut client,
                (*el).event.dataver as libc::c_int,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"modulePopulateClientInfoStructure(&civ1,data, el->event.dataver) == REDISMODULE_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    10798 as libc::c_int,
                );
                unreachable!();
            };
            moduledata = &mut civ1 as *mut RedisModuleClientInfoV1 as *mut libc::c_void;
        } else if eid == 0 as libc::c_int as libc::c_ulong {
            if modulePopulateReplicationInfoStructure(
                &mut riv1 as *mut RedisModuleReplicationInfoV1 as *mut libc::c_void,
                (*el).event.dataver as libc::c_int,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"modulePopulateReplicationInfoStructure(&riv1,el->event.dataver) == REDISMODULE_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    10801 as libc::c_int,
                );
                unreachable!();
            };
            moduledata = &mut riv1 as *mut RedisModuleReplicationInfoV1
                as *mut libc::c_void;
        } else if eid == 2 as libc::c_int as libc::c_ulong {
            moduledata = data;
            let mut fi: *mut RedisModuleFlushInfoV1 = data
                as *mut RedisModuleFlushInfoV1;
            if (*fi).dbnum != -(1 as libc::c_int) {
                selectDb(ctx.client, (*fi).dbnum);
            }
        } else if eid == 9 as libc::c_int as libc::c_ulong {
            let mut m: *mut RedisModule = data as *mut RedisModule;
            if m == (*el).module {
                moduleFreeContext(&mut ctx);
                continue;
            } else {
                mcv1.version = 1 as libc::c_int as uint64_t;
                mcv1.module_name = (*m).name;
                mcv1.module_version = (*m).ver;
                moduledata = &mut mcv1 as *mut RedisModuleModuleChangeV1
                    as *mut libc::c_void;
            }
        } else if eid == 10 as libc::c_int as libc::c_ulong {
            moduledata = data;
        } else if eid == 8 as libc::c_int as libc::c_ulong {
            moduledata = data;
        } else if eid == 11 as libc::c_int as libc::c_ulong {
            moduledata = data;
        } else if eid == 16 as libc::c_int as libc::c_ulong {
            moduledata = data;
        }
        (*(*el).module).in_hook += 1;
        ((*el).callback)
            .expect(
                "non-null function pointer",
            )(&mut ctx, (*el).event, subid as uint64_t, moduledata);
        (*(*el).module).in_hook -= 1;
        moduleFreeContext(&mut ctx);
    };
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnsubscribeAllServerEvents(mut module: *mut RedisModule) {
    let mut el: *mut RedisModuleEventListener = 0 as *mut RedisModuleEventListener;
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(RedisModule_EventListeners, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        el = (*ln).value as *mut RedisModuleEventListener;
        if (*el).module == module {
            listDelNode(RedisModule_EventListeners, ln);
            zfree(el as *mut libc::c_void);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn processModuleLoadingProgressEvent(mut is_aof: libc::c_int) {
    let mut now: libc::c_longlong = server.ustime;
    static mut next_event: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    if now >= next_event {
        let mut progress: libc::c_int = -(1 as libc::c_int);
        if server.loading_total_bytes != 0 {
            progress = ((server.loading_loaded_bytes << 10 as libc::c_int)
                / server.loading_total_bytes) as libc::c_int;
        }
        let mut fi: RedisModuleLoadingProgressV1 = {
            let mut init = RedisModuleLoadingProgressInfo {
                version: 1 as libc::c_int as uint64_t,
                hz: server.hz,
                progress: progress,
            };
            init
        };
        moduleFireServerEvent(
            10 as libc::c_int as uint64_t,
            if is_aof != 0 { 1 as libc::c_int } else { 0 as libc::c_int },
            &mut fi as *mut RedisModuleLoadingProgressV1 as *mut libc::c_void,
        );
        next_event = now + (1000000 as libc::c_int / server.hz) as libc::c_longlong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleNotifyKeyUnlink(
    mut key: *mut robj,
    mut val: *mut robj,
    mut dbid: libc::c_int,
) {
    if (*val).type_0() as libc::c_int == 5 as libc::c_int {
        let mut mv: *mut moduleValue = (*val).ptr as *mut moduleValue;
        let mut mt: *mut moduleType = (*mv).type_0;
        if ((*mt).unlink2).is_some() {
            let mut ctx: RedisModuleKeyOptCtx = {
                let mut init = RedisModuleKeyOptCtx {
                    from_key: key,
                    to_key: 0 as *mut redisObject,
                    from_dbid: dbid,
                    to_dbid: -(1 as libc::c_int),
                };
                init
            };
            ((*mt).unlink2).expect("non-null function pointer")(&mut ctx, (*mv).value);
        } else if ((*mt).unlink).is_some() {
            ((*mt).unlink).expect("non-null function pointer")(key, (*mv).value);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetFreeEffort(
    mut key: *mut robj,
    mut val: *mut robj,
    mut dbid: libc::c_int,
) -> size_t {
    let mut mv: *mut moduleValue = (*val).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    let mut effort: size_t = 1 as libc::c_int as size_t;
    if ((*mt).free_effort2).is_some() {
        let mut ctx: RedisModuleKeyOptCtx = {
            let mut init = RedisModuleKeyOptCtx {
                from_key: key,
                to_key: 0 as *mut redisObject,
                from_dbid: dbid,
                to_dbid: -(1 as libc::c_int),
            };
            init
        };
        effort = ((*mt).free_effort2)
            .expect("non-null function pointer")(&mut ctx, (*mv).value);
    } else if ((*mt).free_effort).is_some() {
        effort = ((*mt).free_effort)
            .expect("non-null function pointer")(key, (*mv).value);
    }
    return effort;
}
#[no_mangle]
pub unsafe extern "C" fn moduleGetMemUsage(
    mut key: *mut robj,
    mut val: *mut robj,
    mut sample_size: size_t,
    mut dbid: libc::c_int,
) -> size_t {
    let mut mv: *mut moduleValue = (*val).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    let mut size: size_t = 0 as libc::c_int as size_t;
    if ((*mt).mem_usage2).is_some() {
        let mut ctx: RedisModuleKeyOptCtx = {
            let mut init = RedisModuleKeyOptCtx {
                from_key: key,
                to_key: 0 as *mut redisObject,
                from_dbid: dbid,
                to_dbid: -(1 as libc::c_int),
            };
            init
        };
        size = ((*mt).mem_usage2)
            .expect("non-null function pointer")(&mut ctx, (*mv).value, sample_size);
    } else if ((*mt).mem_usage).is_some() {
        size = ((*mt).mem_usage).expect("non-null function pointer")((*mv).value);
    }
    return size;
}
#[no_mangle]
pub unsafe extern "C" fn dictCStringKeyHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenHashFunction(
        key as *mut libc::c_uchar as *const libc::c_void,
        strlen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dictCStringKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return (strcmp(key1 as *const libc::c_char, key2 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub static mut moduleAPIDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictCStringKeyHash
                    as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictCStringKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn moduleRegisterApi(
    mut funcname: *const libc::c_char,
    mut funcptr: *mut libc::c_void,
) -> libc::c_int {
    return dictAdd(
        server.moduleapi,
        funcname as *mut libc::c_char as *mut libc::c_void,
        funcptr,
    );
}
#[no_mangle]
pub unsafe extern "C" fn moduleInitModulesSystemLast() {}
#[no_mangle]
pub static mut sdsKeyValueHashDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCaseCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictSdsDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn moduleInitModulesSystem() {
    moduleUnblockedClients = listCreate();
    server.loadmodule_queue = listCreate();
    server.module_configs_queue = dictCreate(&mut sdsKeyValueHashDictType);
    modules = dictCreate(&mut modulesDictType);
    moduleKeyspaceSubscribers = listCreate();
    moduleCommandFilters = listCreate();
    moduleRegisterCoreAPI();
    if anetPipe(
        (server.module_pipe).as_mut_ptr(),
        0o2000000 as libc::c_int | 0o4000 as libc::c_int,
        0o2000000 as libc::c_int | 0o4000 as libc::c_int,
    ) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't create the pipe for module threads: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    Timers = raxNew();
    RedisModule_EventListeners = listCreate();
    if (core::mem::size_of::<[uint64_t; 17]>() as libc::c_ulong)
        .wrapping_div(core::mem::size_of::<uint64_t>() as libc::c_ulong)
        == 17 as libc::c_int as libc::c_ulong
    {} else {
        _serverAssert(
            b"sizeof(moduleEventVersions)/sizeof(moduleEventVersions[0]) == _REDISMODULE_EVENT_NEXT\0"
                as *const u8 as *const libc::c_char,
            b"module.c\0" as *const u8 as *const libc::c_char,
            11012 as libc::c_int,
        );
        unreachable!();
    };
    pthread_mutex_lock(&mut moduleGIL);
}
#[no_mangle]
pub unsafe extern "C" fn modulesCron() {
    let mut iteration: libc::c_int = 50 as libc::c_int;
    let min_client: libc::c_uint = 8 as libc::c_int as libc::c_uint;
    while iteration > 0 as libc::c_int
        && moduleTempClientCount > 0 as libc::c_int as libc::c_ulong
        && moduleTempClientMinCount > min_client as libc::c_ulong
    {
        moduleTempClientCount = moduleTempClientCount.wrapping_sub(1);
        let mut c: *mut client = *moduleTempClients
            .offset(moduleTempClientCount as isize);
        freeClient(c);
        iteration -= 1;
        moduleTempClientMinCount = moduleTempClientMinCount.wrapping_sub(1);
    }
    moduleTempClientMinCount = moduleTempClientCount;
    if moduleTempClientCap > 32 as libc::c_int as libc::c_ulong
        && moduleTempClientCap
            > moduleTempClientCount.wrapping_mul(4 as libc::c_int as libc::c_ulong)
    {
        moduleTempClientCap = (moduleTempClientCap as libc::c_ulong)
            .wrapping_div(4 as libc::c_int as libc::c_ulong) as size_t as size_t;
        moduleTempClients = zrealloc(
            moduleTempClients as *mut libc::c_void,
            (core::mem::size_of::<*mut client>() as libc::c_ulong)
                .wrapping_mul(moduleTempClientCap),
        ) as *mut *mut client;
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleLoadQueueEntryFree(
    mut loadmod: *mut moduleLoadQueueEntry,
) {
    if loadmod.is_null() {
        return;
    }
    sdsfree((*loadmod).path);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*loadmod).argc {
        decrRefCount(*((*loadmod).argv).offset(i as isize));
        i += 1;
    }
    zfree((*loadmod).argv as *mut libc::c_void);
    zfree(loadmod as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleRemoveConfigs(mut module: *mut RedisModule) {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind((*module).module_configs, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut config: *mut ModuleConfig = (*ln).value as *mut ModuleConfig;
        let mut module_name: sds = sdsnew((*module).name);
        let mut full_name: sds = sdscat(
            sdscat(module_name, b".\0" as *const u8 as *const libc::c_char),
            (*config).name as *const libc::c_char,
        );
        removeConfig(full_name);
        sdsfree(full_name);
    };
}
#[no_mangle]
pub unsafe extern "C" fn moduleLoadFromQueue() {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.loadmodule_queue, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut loadmod: *mut moduleLoadQueueEntry = (*ln).value
            as *mut moduleLoadQueueEntry;
        if moduleLoad(
            (*loadmod).path as *const libc::c_char,
            (*loadmod).argv as *mut *mut libc::c_void,
            (*loadmod).argc,
            0 as libc::c_int,
        ) == -(1 as libc::c_int)
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Can't load module from %s: server aborting\0" as *const u8
                        as *const libc::c_char,
                    (*loadmod).path,
                );
            }
            exit(1 as libc::c_int);
        }
        moduleLoadQueueEntryFree(loadmod);
        listDelNode(server.loadmodule_queue, ln);
    }
    if ((*server.module_configs_queue).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*server.module_configs_queue).ht_used[1 as libc::c_int as usize])
        != 0
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Module Configuration detected without loadmodule directive or no ApplyConfig call: aborting\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn moduleFreeModuleStructure(mut module: *mut RedisModule) {
    listRelease((*module).types);
    listRelease((*module).filters);
    listRelease((*module).usedby);
    listRelease((*module).using);
    listRelease((*module).module_configs);
    sdsfree((*module).name);
    moduleLoadQueueEntryFree((*module).loadmod);
    zfree(module as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleFreeArgs(
    mut args: *mut redisCommandArg,
    mut num_args: libc::c_int,
) {
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < num_args {
        zfree((*args.offset(j as isize)).name as *mut libc::c_char as *mut libc::c_void);
        zfree(
            (*args.offset(j as isize)).token as *mut libc::c_char as *mut libc::c_void,
        );
        zfree(
            (*args.offset(j as isize)).summary as *mut libc::c_char as *mut libc::c_void,
        );
        zfree(
            (*args.offset(j as isize)).since as *mut libc::c_char as *mut libc::c_void,
        );
        zfree(
            (*args.offset(j as isize)).deprecated_since as *mut libc::c_char
                as *mut libc::c_void,
        );
        if !((*args.offset(j as isize)).subargs).is_null() {
            moduleFreeArgs(
                (*args.offset(j as isize)).subargs,
                (*args.offset(j as isize)).num_args,
            );
        }
        j += 1;
    }
    zfree(args as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleFreeCommand(
    mut module: *mut RedisModule,
    mut cmd: *mut redisCommand,
) -> libc::c_int {
    if (*cmd).proc_0
        != Some(RedisModuleCommandDispatcher as unsafe extern "C" fn(*mut client) -> ())
    {
        return -(1 as libc::c_int);
    }
    let mut cp: *mut RedisModuleCommand = (*cmd).module_cmd;
    if (*cp).module != module {
        return -(1 as libc::c_int);
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*cmd).key_specs_num {
        if !((*((*cmd).key_specs).offset(j as isize)).notes).is_null() {
            zfree(
                (*((*cmd).key_specs).offset(j as isize)).notes as *mut libc::c_char
                    as *mut libc::c_void,
            );
        }
        if (*((*cmd).key_specs).offset(j as isize)).begin_search_type as libc::c_uint
            == KSPEC_BS_KEYWORD as libc::c_int as libc::c_uint
        {
            zfree(
                (*((*cmd).key_specs).offset(j as isize)).bs.keyword.keyword
                    as *mut libc::c_char as *mut libc::c_void,
            );
        }
        j += 1;
    }
    if (*cmd).key_specs != ((*cmd).key_specs_static).as_mut_ptr() {
        zfree((*cmd).key_specs as *mut libc::c_void);
    }
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while !((*cmd).tips).is_null() && !(*((*cmd).tips).offset(j_0 as isize)).is_null() {
        zfree(
            *((*cmd).tips).offset(j_0 as isize) as *mut libc::c_char as *mut libc::c_void,
        );
        j_0 += 1;
    }
    zfree((*cmd).tips as *mut libc::c_void);
    let mut j_1: libc::c_int = 0 as libc::c_int;
    while !((*cmd).history).is_null()
        && !((*((*cmd).history).offset(j_1 as isize)).since).is_null()
    {
        zfree(
            (*((*cmd).history).offset(j_1 as isize)).since as *mut libc::c_char
                as *mut libc::c_void,
        );
        zfree(
            (*((*cmd).history).offset(j_1 as isize)).changes as *mut libc::c_char
                as *mut libc::c_void,
        );
        j_1 += 1;
    }
    zfree((*cmd).history as *mut libc::c_void);
    zfree((*cmd).summary as *mut libc::c_char as *mut libc::c_void);
    zfree((*cmd).since as *mut libc::c_char as *mut libc::c_void);
    zfree((*cmd).deprecated_since as *mut libc::c_char as *mut libc::c_void);
    zfree((*cmd).complexity as *mut libc::c_char as *mut libc::c_void);
    if !((*cmd).latency_histogram).is_null() {
        hdr_close((*cmd).latency_histogram);
        (*cmd).latency_histogram = 0 as *mut hdr_histogram;
    }
    moduleFreeArgs((*cmd).args, (*cmd).num_args);
    zfree(cp as *mut libc::c_void);
    if !((*cmd).subcommands_dict).is_null() {
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        let mut di: *mut dictIterator = dictGetSafeIterator((*cmd).subcommands_dict);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut sub: *mut redisCommand = (*de).v.val as *mut redisCommand;
            if moduleFreeCommand(module, sub) != 0 as libc::c_int {
                continue;
            }
            if dictDelete(
                (*cmd).subcommands_dict,
                (*sub).declared_name as *const libc::c_void,
            ) == 0 as libc::c_int
            {} else {
                _serverAssert(
                    b"dictDelete(cmd->subcommands_dict, sub->declared_name) == DICT_OK\0"
                        as *const u8 as *const libc::c_char,
                    b"module.c\0" as *const u8 as *const libc::c_char,
                    11178 as libc::c_int,
                );
                unreachable!();
            };
            sdsfree((*sub).declared_name as sds);
            sdsfree((*sub).fullname);
            zfree(sub as *mut libc::c_void);
        }
        dictReleaseIterator(di);
        dictRelease((*cmd).subcommands_dict);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnregisterCommands(mut module: *mut RedisModule) {
    let mut di: *mut dictIterator = dictGetSafeIterator(server.commands);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if moduleFreeCommand(module, cmd) != 0 as libc::c_int {
            continue;
        }
        if dictDelete(server.commands, (*cmd).fullname as *const libc::c_void)
            == 0 as libc::c_int
        {} else {
            _serverAssert(
                b"dictDelete(server.commands, cmd->fullname) == DICT_OK\0" as *const u8
                    as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                11198 as libc::c_int,
            );
            unreachable!();
        };
        if dictDelete(server.orig_commands, (*cmd).fullname as *const libc::c_void)
            == 0 as libc::c_int
        {} else {
            _serverAssert(
                b"dictDelete(server.orig_commands, cmd->fullname) == DICT_OK\0"
                    as *const u8 as *const libc::c_char,
                b"module.c\0" as *const u8 as *const libc::c_char,
                11199 as libc::c_int,
            );
            unreachable!();
        };
        sdsfree((*cmd).declared_name as sds);
        sdsfree((*cmd).fullname);
        zfree(cmd as *mut libc::c_void);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn parseLoadexArguments(
    mut module_argv: *mut *mut *mut robj,
    mut module_argc: *mut libc::c_int,
) -> libc::c_int {
    let mut args_specified: libc::c_int = 0 as libc::c_int;
    let mut argv: *mut *mut robj = *module_argv;
    let mut argc: libc::c_int = *module_argc;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        let mut arg_val: *mut libc::c_char = (**argv.offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(arg_val, b"CONFIG\0" as *const u8 as *const libc::c_char) == 0 {
            if i + 2 as libc::c_int >= argc {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"CONFIG specified without name value pair\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                return 1 as libc::c_int;
            }
            let mut name: sds = sdsdup(
                (**argv.offset((i + 1 as libc::c_int) as isize)).ptr as sds,
            );
            let mut value: sds = sdsdup(
                (**argv.offset((i + 2 as libc::c_int) as isize)).ptr as sds,
            );
            if dictReplace(
                server.module_configs_queue,
                name as *mut libc::c_void,
                value as *mut libc::c_void,
            ) == 0
            {
                sdsfree(name);
            }
            i += 2 as libc::c_int;
            i += 1;
        } else if strcasecmp(arg_val, b"ARGS\0" as *const u8 as *const libc::c_char) == 0
        {
            args_specified = 1 as libc::c_int;
            i += 1;
            if i >= argc {
                *module_argv = 0 as *mut *mut robj;
                *module_argc = 0 as libc::c_int;
            } else {
                *module_argv = argv.offset(i as isize);
                *module_argc = argc - i;
            }
            break;
        } else {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Syntax Error from arguments to loadex around %s.\0" as *const u8
                        as *const libc::c_char,
                    arg_val,
                );
            }
            return 1 as libc::c_int;
        }
    }
    if args_specified == 0 {
        *module_argv = 0 as *mut *mut robj;
        *module_argc = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleLoad(
    mut path: *const libc::c_char,
    mut module_argv: *mut *mut libc::c_void,
    mut module_argc: libc::c_int,
    mut is_loadex: libc::c_int,
) -> libc::c_int {
    let mut onload: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *mut *mut libc::c_void,
            libc::c_int,
        ) -> libc::c_int,
    > = None;
    let mut handle: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_mode: 0,
        st_nlink: 0,
        st_uid: 0,
        st_gid: 0,
        st_rdev: 0,
        __pad1: 0,
        st_size: 0,
        st_blksize: 0,
        __pad2: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 2],
    };
    if stat(path, &mut st) == 0 as libc::c_int {
        if st.st_mode
            & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                as libc::c_uint == 0
        {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Module %s failed to load: It does not have execute permissions.\0"
                        as *const u8 as *const libc::c_char,
                    path,
                );
            }
            return -(1 as libc::c_int);
        }
    }
    handle = dlopen(path, 0x2 as libc::c_int | 0 as libc::c_int);
    if handle.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Module %s failed to load: %s\0" as *const u8 as *const libc::c_char,
                path,
                dlerror(),
            );
        }
        return -(1 as libc::c_int);
    }
    onload = core::mem::transmute::<
        libc::intptr_t,
        Option::<
            unsafe extern "C" fn(
                *mut libc::c_void,
                *mut *mut libc::c_void,
                libc::c_int,
            ) -> libc::c_int,
        >,
    >(
        dlsym(handle, b"RedisModule_OnLoad\0" as *const u8 as *const libc::c_char)
            as libc::c_ulong as libc::intptr_t,
    );
    if onload.is_none() {
        dlclose(handle);
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Module %s does not export RedisModule_OnLoad() symbol. Module not loaded.\0"
                    as *const u8 as *const libc::c_char,
                path,
            );
        }
        return -(1 as libc::c_int);
    }
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    moduleCreateContext(
        &mut ctx,
        0 as *mut RedisModule,
        (1 as libc::c_int) << 6 as libc::c_int,
    );
    if onload
        .expect(
            "non-null function pointer",
        )(&mut ctx as *mut RedisModuleCtx as *mut libc::c_void, module_argv, module_argc)
        == 1 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Module %s initialization failed. Module not loaded\0" as *const u8
                    as *const libc::c_char,
                path,
            );
        }
        if !(ctx.module).is_null() {
            moduleUnregisterCommands(ctx.module);
            moduleUnregisterSharedAPI(ctx.module);
            moduleUnregisterUsedAPI(ctx.module);
            moduleRemoveConfigs(ctx.module);
            moduleFreeModuleStructure(ctx.module);
        }
        moduleFreeContext(&mut ctx);
        dlclose(handle);
        return -(1 as libc::c_int);
    }
    dictAdd(
        modules,
        (*ctx.module).name as *mut libc::c_void,
        ctx.module as *mut libc::c_void,
    );
    (*ctx.module).blocked_clients = 0 as libc::c_int;
    (*ctx.module).handle = handle;
    (*ctx.module)
        .loadmod = zmalloc(
        core::mem::size_of::<moduleLoadQueueEntry>() as libc::c_ulong,
    ) as *mut moduleLoadQueueEntry;
    (*(*ctx.module).loadmod).path = sdsnew(path);
    (*(*ctx.module).loadmod)
        .argv = (if module_argc != 0 {
        zmalloc(
            (core::mem::size_of::<*mut robj>() as libc::c_ulong)
                .wrapping_mul(module_argc as libc::c_ulong),
        )
    } else {
        0 as *mut libc::c_void
    }) as *mut *mut robj;
    (*(*ctx.module).loadmod).argc = module_argc;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < module_argc {
        let ref mut fresh39 = *((*(*ctx.module).loadmod).argv).offset(i as isize);
        *fresh39 = *module_argv.offset(i as isize) as *mut robj;
        incrRefCount(*((*(*ctx.module).loadmod).argv).offset(i as isize));
        i += 1;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Module '%s' loaded from %s\0" as *const u8 as *const libc::c_char,
            (*ctx.module).name,
            path,
        );
    }
    if (*(*ctx.module).module_configs).len != 0 && (*ctx.module).configs_initialized == 0
    {
        serverLogRaw(
            3 as libc::c_int,
            b"Module Configurations were not set, likely a missing LoadConfigs call. Unloading the module.\0"
                as *const u8 as *const libc::c_char,
        );
        moduleUnload((*ctx.module).name);
        moduleFreeContext(&mut ctx);
        return -(1 as libc::c_int);
    }
    if is_loadex != 0
        && ((*server.module_configs_queue).ht_used[0 as libc::c_int as usize])
            .wrapping_add(
                (*server.module_configs_queue).ht_used[1 as libc::c_int as usize],
            ) != 0
    {
        serverLogRaw(
            3 as libc::c_int,
            b"Loadex configurations were not applied, likely due to invalid arguments. Unloading the module.\0"
                as *const u8 as *const libc::c_char,
        );
        moduleUnload((*ctx.module).name);
        moduleFreeContext(&mut ctx);
        return -(1 as libc::c_int);
    }
    moduleFireServerEvent(
        9 as libc::c_int as uint64_t,
        0 as libc::c_int,
        ctx.module as *mut libc::c_void,
    );
    moduleFreeContext(&mut ctx);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleUnload(mut name: sds) -> libc::c_int {
    let mut module: *mut RedisModule = dictFetchValue(
        modules,
        name as *const libc::c_void,
    ) as *mut RedisModule;
    if module.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return -(1 as libc::c_int);
    } else {
        if (*(*module).types).len != 0 {
            *__errno_location() = 16 as libc::c_int;
            return -(1 as libc::c_int);
        } else {
            if (*(*module).usedby).len != 0 {
                *__errno_location() = 1 as libc::c_int;
                return -(1 as libc::c_int);
            } else {
                if (*module).blocked_clients != 0 {
                    *__errno_location() = 11 as libc::c_int;
                    return -(1 as libc::c_int);
                } else {
                    if moduleHoldsTimer(module) != 0 {
                        *__errno_location() = 115 as libc::c_int;
                        return -(1 as libc::c_int);
                    }
                }
            }
        }
    }
    let mut onunload: Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int> = None;
    onunload = core::mem::transmute::<
        libc::intptr_t,
        Option::<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    >(
        dlsym(
            (*module).handle,
            b"RedisModule_OnUnload\0" as *const u8 as *const libc::c_char,
        ) as libc::c_ulong as libc::intptr_t,
    );
    if onunload.is_some() {
        let mut ctx: RedisModuleCtx = RedisModuleCtx {
            getapifuncptr: 0 as *mut libc::c_void,
            module: 0 as *mut RedisModule,
            client: 0 as *mut client,
            blocked_client: 0 as *mut RedisModuleBlockedClient,
            amqueue: 0 as *mut AutoMemEntry,
            amqueue_len: 0,
            amqueue_used: 0,
            flags: 0,
            postponed_arrays: 0 as *mut *mut libc::c_void,
            postponed_arrays_count: 0,
            blocked_privdata: 0 as *mut libc::c_void,
            blocked_ready_key: 0 as *mut robj,
            keys_result: 0 as *mut getKeysResult,
            pa_head: 0 as *mut RedisModulePoolAllocBlock,
            next_yield_time: 0,
            user: 0 as *const RedisModuleUser,
        };
        moduleCreateContext(&mut ctx, module, (1 as libc::c_int) << 6 as libc::c_int);
        let mut unload_status: libc::c_int = onunload
            .expect(
                "non-null function pointer",
            )(&mut ctx as *mut RedisModuleCtx as *mut libc::c_void);
        moduleFreeContext(&mut ctx);
        if unload_status == 1 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Module %s OnUnload failed.  Unload canceled.\0" as *const u8
                        as *const libc::c_char,
                    name,
                );
            }
            *__errno_location() = 125 as libc::c_int;
            return -(1 as libc::c_int);
        }
    }
    moduleFreeAuthenticatedClients(module);
    moduleUnregisterCommands(module);
    moduleUnregisterSharedAPI(module);
    moduleUnregisterUsedAPI(module);
    moduleUnregisterFilters(module);
    moduleRemoveConfigs(module);
    moduleUnsubscribeNotifications(module);
    moduleUnsubscribeAllServerEvents(module);
    if dlclose((*module).handle) == -(1 as libc::c_int) {
        let mut error: *mut libc::c_char = dlerror();
        if error.is_null() {
            error = b"Unknown error\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Error when trying to close the %s module: %s\0" as *const u8
                    as *const libc::c_char,
                (*module).name,
                error,
            );
        }
    }
    moduleFireServerEvent(
        9 as libc::c_int as uint64_t,
        1 as libc::c_int,
        module as *mut libc::c_void,
    );
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Module %s unloaded\0" as *const u8 as *const libc::c_char,
            (*module).name,
        );
    }
    dictDelete(modules, (*module).name as *const libc::c_void);
    (*module).name = 0 as *mut libc::c_char;
    moduleFreeModuleStructure(module);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn modulePipeReadable(
    mut el: *mut aeEventLoop,
    mut fd: libc::c_int,
    mut privdata: *mut libc::c_void,
    mut mask: libc::c_int,
) {
    let mut buf: [libc::c_char; 128] = [0; 128];
    while read(
        fd,
        buf.as_mut_ptr() as *mut libc::c_void,
        core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
    ) as libc::c_ulong == core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong
    {}
    eventLoopHandleOneShotEvents();
}
#[no_mangle]
pub unsafe extern "C" fn addReplyLoadedModules(mut c: *mut client) {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    addReplyArrayLen(
        c,
        ((*modules).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*modules).ht_used[1 as libc::c_int as usize]) as libc::c_long,
    );
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut name: sds = (*de).key as sds;
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        let mut path: sds = (*(*module).loadmod).path;
        addReplyMapLen(c, 4 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(c, name as *const libc::c_void, sdslen(name));
        addReplyBulkCString(c, b"ver\0" as *const u8 as *const libc::c_char);
        addReplyLongLong(c, (*module).ver as libc::c_longlong);
        addReplyBulkCString(c, b"path\0" as *const u8 as *const libc::c_char);
        addReplyBulkCBuffer(c, path as *const libc::c_void, sdslen(path));
        addReplyBulkCString(c, b"args\0" as *const u8 as *const libc::c_char);
        addReplyArrayLen(c, (*(*module).loadmod).argc as libc::c_long);
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*(*module).loadmod).argc {
            addReplyBulk(c, *((*(*module).loadmod).argv).offset(i as isize));
            i += 1;
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn genModulesInfoStringRenderModulesList(mut l: *mut list) -> sds {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(l, &mut li);
    let mut output: sds = sdsnew(b"[\0" as *const u8 as *const libc::c_char);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*ln).value as *mut RedisModule;
        output = sdscat(output, (*module).name);
        if ln != (*l).tail {
            output = sdscat(output, b"|\0" as *const u8 as *const libc::c_char);
        }
    }
    output = sdscat(output, b"]\0" as *const u8 as *const libc::c_char);
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn genModulesInfoStringRenderModuleOptions(
    mut module: *mut RedisModule,
) -> sds {
    let mut output: sds = sdsnew(b"[\0" as *const u8 as *const libc::c_char);
    if (*module).options & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        output = sdscat(
            output,
            b"handle-io-errors|\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*module).options & (1 as libc::c_int) << 2 as libc::c_int != 0 {
        output = sdscat(
            output,
            b"handle-repl-async-load|\0" as *const u8 as *const libc::c_char,
        );
    }
    if (*module).options & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        output = sdscat(
            output,
            b"no-implicit-signal-modified|\0" as *const u8 as *const libc::c_char,
        );
    }
    output = sdstrim(output, b"|\0" as *const u8 as *const libc::c_char);
    output = sdscat(output, b"]\0" as *const u8 as *const libc::c_char);
    return output;
}
#[no_mangle]
pub unsafe extern "C" fn genModulesInfoString(mut info: sds) -> sds {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut name: sds = (*de).key as sds;
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        let mut usedby: sds = genModulesInfoStringRenderModulesList((*module).usedby);
        let mut using: sds = genModulesInfoStringRenderModulesList((*module).using);
        let mut options: sds = genModulesInfoStringRenderModuleOptions(module);
        info = sdscatfmt(
            info,
            b"module:name=%S,ver=%i,api=%i,filters=%i,usedby=%S,using=%S,options=%S\r\n\0"
                as *const u8 as *const libc::c_char,
            name,
            (*module).ver,
            (*module).apiver,
            (*(*module).filters).len as libc::c_int,
            usedby,
            using,
            options,
        );
        sdsfree(usedby);
        sdsfree(using);
        sdsfree(options);
    }
    dictReleaseIterator(di);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn isModuleConfigNameRegistered(
    mut module: *mut RedisModule,
    mut name: sds,
) -> libc::c_int {
    let mut match_0: *mut listNode = listSearchKey(
        (*module).module_configs,
        name as *mut libc::c_void,
    );
    return (match_0 != 0 as *mut libc::c_void as *mut listNode) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleVerifyConfigFlags(
    mut flags: libc::c_uint,
    mut type_0: configType,
) -> libc::c_int {
    if flags as libc::c_ulonglong
        & !(0 as libc::c_int as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 0 as libc::c_int
            | (1 as libc::c_ulonglong) << 1 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int
            | (1 as libc::c_ulonglong) << 5 as libc::c_int
            | (1 as libc::c_ulonglong) << 6 as libc::c_int
            | (1 as libc::c_ulonglong) << 8 as libc::c_int
            | (1 as libc::c_ulonglong) << 7 as libc::c_int) != 0
    {
        serverLogRaw(
            3 as libc::c_int,
            b"Invalid flag(s) for configuration\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if type_0 as libc::c_uint != NUMERIC_CONFIG as libc::c_int as libc::c_uint
        && flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0
    {
        serverLogRaw(
            3 as libc::c_int,
            b"Numeric flag provided for non-numeric configuration.\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if type_0 as libc::c_uint != ENUM_CONFIG as libc::c_int as libc::c_uint
        && flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0
    {
        serverLogRaw(
            3 as libc::c_int,
            b"Enum flag provided for non-enum configuration.\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleVerifyConfigName(mut name: sds) -> libc::c_int {
    if sdslen(name) == 0 as libc::c_int as libc::c_ulong {
        serverLogRaw(
            3 as libc::c_int,
            b"Module config names cannot be an empty string.\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < sdslen(name) {
        let mut curr_char: libc::c_char = *name.offset(i as isize);
        if curr_char as libc::c_int >= 'a' as i32
            && curr_char as libc::c_int <= 'z' as i32
            || curr_char as libc::c_int >= 'A' as i32
                && curr_char as libc::c_int <= 'Z' as i32
            || curr_char as libc::c_int >= '0' as i32
                && curr_char as libc::c_int <= '9' as i32
            || curr_char as libc::c_int == '_' as i32
            || curr_char as libc::c_int == '-' as i32
        {
            i = i.wrapping_add(1);
        } else {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Invalid character %c in Module Config name %s.\0" as *const u8
                        as *const libc::c_char,
                    curr_char as libc::c_int,
                    name,
                );
            }
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
static mut configerr: [libc::c_char; 256] = [0; 256];
unsafe extern "C" fn propagateErrorString(
    mut err_in: *mut robj,
    mut err: *mut *const libc::c_char,
) {
    if !err_in.is_null() {
        strncpy(
            configerr.as_mut_ptr(),
            (*err_in).ptr as *const libc::c_char,
            256 as libc::c_int as libc::c_ulong,
        );
        configerr[(256 as libc::c_int - 1 as libc::c_int)
            as usize] = '\0' as i32 as libc::c_char;
        decrRefCount(err_in);
        *err = configerr.as_mut_ptr();
    }
}
#[no_mangle]
pub unsafe extern "C" fn setModuleBoolConfig(
    mut config: *mut ModuleConfig,
    mut val: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut error: *mut robj = 0 as *mut robj;
    let mut return_code: libc::c_int = ((*config).set_fn.set_bool)
        .expect(
            "non-null function pointer",
        )((*config).name as *const libc::c_char, val, (*config).privdata, &mut error);
    propagateErrorString(error, err);
    return if return_code == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn setModuleStringConfig(
    mut config: *mut ModuleConfig,
    mut strval: sds,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut error: *mut robj = 0 as *mut robj;
    let mut new: *mut robj = createStringObject(
        strval as *const libc::c_char,
        sdslen(strval),
    );
    let mut return_code: libc::c_int = ((*config).set_fn.set_string)
        .expect(
            "non-null function pointer",
        )((*config).name as *const libc::c_char, new, (*config).privdata, &mut error);
    propagateErrorString(error, err);
    decrRefCount(new);
    return if return_code == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn setModuleEnumConfig(
    mut config: *mut ModuleConfig,
    mut val: libc::c_int,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut error: *mut robj = 0 as *mut robj;
    let mut return_code: libc::c_int = ((*config).set_fn.set_enum)
        .expect(
            "non-null function pointer",
        )((*config).name as *const libc::c_char, val, (*config).privdata, &mut error);
    propagateErrorString(error, err);
    return if return_code == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn setModuleNumericConfig(
    mut config: *mut ModuleConfig,
    mut val: libc::c_longlong,
    mut err: *mut *const libc::c_char,
) -> libc::c_int {
    let mut error: *mut robj = 0 as *mut robj;
    let mut return_code: libc::c_int = ((*config).set_fn.set_numeric)
        .expect(
            "non-null function pointer",
        )((*config).name as *const libc::c_char, val, (*config).privdata, &mut error);
    propagateErrorString(error, err);
    return if return_code == 0 as libc::c_int {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn getModuleBoolConfig(
    mut module_config: *mut ModuleConfig,
) -> libc::c_int {
    return ((*module_config).get_fn.get_bool)
        .expect(
            "non-null function pointer",
        )((*module_config).name as *const libc::c_char, (*module_config).privdata);
}
#[no_mangle]
pub unsafe extern "C" fn getModuleStringConfig(
    mut module_config: *mut ModuleConfig,
) -> sds {
    let mut val: *mut robj = ((*module_config).get_fn.get_string)
        .expect(
            "non-null function pointer",
        )((*module_config).name as *const libc::c_char, (*module_config).privdata);
    return if !val.is_null() { sdsdup((*val).ptr as sds) } else { 0 as sds };
}
#[no_mangle]
pub unsafe extern "C" fn getModuleEnumConfig(
    mut module_config: *mut ModuleConfig,
) -> libc::c_int {
    return ((*module_config).get_fn.get_enum)
        .expect(
            "non-null function pointer",
        )((*module_config).name as *const libc::c_char, (*module_config).privdata);
}
#[no_mangle]
pub unsafe extern "C" fn getModuleNumericConfig(
    mut module_config: *mut ModuleConfig,
) -> libc::c_longlong {
    return ((*module_config).get_fn.get_numeric)
        .expect(
            "non-null function pointer",
        )((*module_config).name as *const libc::c_char, (*module_config).privdata);
}
#[no_mangle]
pub unsafe extern "C" fn loadModuleConfigs(mut module: *mut RedisModule) -> libc::c_int {
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut err: *const libc::c_char = 0 as *const libc::c_char;
    listRewind((*module).module_configs, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut module_config: *mut ModuleConfig = (*ln).value as *mut ModuleConfig;
        let mut config_name: sds = sdscatfmt(
            sdsempty(),
            b"%s.%s\0" as *const u8 as *const libc::c_char,
            (*module).name,
            (*module_config).name,
        );
        let mut config_argument: *mut dictEntry = dictFind(
            server.module_configs_queue,
            config_name as *const libc::c_void,
        );
        if !config_argument.is_null() {
            if performModuleConfigSetFromName(
                (*config_argument).key as sds,
                (*config_argument).v.val as sds,
                &mut err,
            ) == 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Issue during loading of configuration %s : %s\0" as *const u8
                            as *const libc::c_char,
                        (*config_argument).key as sds,
                        err,
                    );
                }
                sdsfree(config_name);
                dictEmpty(server.module_configs_queue, None);
                return 1 as libc::c_int;
            }
        } else if performModuleConfigSetDefaultFromName(config_name, &mut err) == 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Issue attempting to set default value of configuration %s : %s\0"
                        as *const u8 as *const libc::c_char,
                    (*module_config).name,
                    err,
                );
            }
            sdsfree(config_name);
            dictEmpty(server.module_configs_queue, None);
            return 1 as libc::c_int;
        }
        dictDelete(server.module_configs_queue, config_name as *const libc::c_void);
        sdsfree(config_name);
    }
    (*module).configs_initialized = 1 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn addModuleConfigApply(
    mut module_configs: *mut list,
    mut module_config: *mut ModuleConfig,
) {
    if ((*module_config).apply_fn).is_none() {
        return;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut pending_apply: *mut ModuleConfig = 0 as *mut ModuleConfig;
    listRewind(module_configs, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        pending_apply = (*ln).value as *mut ModuleConfig;
        if (*pending_apply).apply_fn == (*module_config).apply_fn
            && (*pending_apply).privdata == (*module_config).privdata
        {
            return;
        }
    }
    listAddNodeTail(module_configs, module_config as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn moduleConfigApplyConfig(
    mut module_configs: *mut list,
    mut err: *mut *const libc::c_char,
    mut err_arg_name: *mut *const libc::c_char,
) -> libc::c_int {
    if (*module_configs).len == 0 {
        return 1 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    let mut module_config: *mut ModuleConfig = 0 as *mut ModuleConfig;
    let mut error: *mut robj = 0 as *mut robj;
    let mut ctx: RedisModuleCtx = RedisModuleCtx {
        getapifuncptr: 0 as *mut libc::c_void,
        module: 0 as *mut RedisModule,
        client: 0 as *mut client,
        blocked_client: 0 as *mut RedisModuleBlockedClient,
        amqueue: 0 as *mut AutoMemEntry,
        amqueue_len: 0,
        amqueue_used: 0,
        flags: 0,
        postponed_arrays: 0 as *mut *mut libc::c_void,
        postponed_arrays_count: 0,
        blocked_privdata: 0 as *mut libc::c_void,
        blocked_ready_key: 0 as *mut robj,
        keys_result: 0 as *mut getKeysResult,
        pa_head: 0 as *mut RedisModulePoolAllocBlock,
        next_yield_time: 0,
        user: 0 as *const RedisModuleUser,
    };
    listRewind(module_configs, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        module_config = (*ln).value as *mut ModuleConfig;
        moduleCreateContext(&mut ctx, (*module_config).module, 0 as libc::c_int);
        if ((*module_config).apply_fn)
            .expect(
                "non-null function pointer",
            )(&mut ctx, (*module_config).privdata, &mut error) != 0
        {
            if !err_arg_name.is_null() {
                *err_arg_name = (*module_config).name as *const libc::c_char;
            }
            propagateErrorString(error, err);
            moduleFreeContext(&mut ctx);
            return 0 as libc::c_int;
        }
        moduleFreeContext(&mut ctx);
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn createModuleConfig(
    mut name: sds,
    mut apply_fn: RedisModuleConfigApplyFunc,
    mut privdata: *mut libc::c_void,
    mut module: *mut RedisModule,
) -> *mut ModuleConfig {
    let mut new_config: *mut ModuleConfig = zmalloc(
        core::mem::size_of::<ModuleConfig>() as libc::c_ulong,
    ) as *mut ModuleConfig;
    (*new_config).name = sdsdup(name);
    (*new_config).apply_fn = apply_fn;
    (*new_config).privdata = privdata;
    (*new_config).module = module;
    return new_config;
}
#[no_mangle]
pub unsafe extern "C" fn moduleConfigValidityCheck(
    mut module: *mut RedisModule,
    mut name: sds,
    mut flags: libc::c_uint,
    mut type_0: configType,
) -> libc::c_int {
    if moduleVerifyConfigFlags(flags, type_0) != 0 || moduleVerifyConfigName(name) != 0 {
        *__errno_location() = 22 as libc::c_int;
        return 1 as libc::c_int;
    }
    if isModuleConfigNameRegistered(module, name) != 0 {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Configuration by the name: %s already registered\0" as *const u8
                    as *const libc::c_char,
                name,
            );
        }
        *__errno_location() = 114 as libc::c_int;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn maskModuleConfigFlags(mut flags: libc::c_uint) -> libc::c_uint {
    let mut new_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if flags & 0 as libc::c_int as libc::c_uint != 0 {
        new_flags |= 0 as libc::c_int as libc::c_uint;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 0 as libc::c_int) as libc::c_uint;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 4 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 4 as libc::c_int) as libc::c_uint;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 5 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 5 as libc::c_int) as libc::c_uint;
    }
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 6 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 6 as libc::c_int) as libc::c_uint;
    }
    return new_flags;
}
#[no_mangle]
pub unsafe extern "C" fn maskModuleNumericConfigFlags(
    mut flags: libc::c_uint,
) -> libc::c_uint {
    let mut new_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 7 as libc::c_int != 0 {
        new_flags |= ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_uint;
    }
    return new_flags;
}
#[no_mangle]
pub unsafe extern "C" fn maskModuleEnumConfigFlags(
    mut flags: libc::c_uint,
) -> libc::c_uint {
    let mut new_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    if flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 8 as libc::c_int != 0 {
        new_flags = (new_flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 3 as libc::c_int) as libc::c_uint;
    }
    return new_flags;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterStringConfig(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut default_val: *const libc::c_char,
    mut flags: libc::c_uint,
    mut getfn: RedisModuleConfigGetStringFunc,
    mut setfn: RedisModuleConfigSetStringFunc,
    mut applyfn: RedisModuleConfigApplyFunc,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    let mut module: *mut RedisModule = (*ctx).module;
    let mut config_name: sds = sdsnew(name);
    if moduleConfigValidityCheck(module, config_name, flags, NUMERIC_CONFIG) != 0 {
        sdsfree(config_name);
        return 1 as libc::c_int;
    }
    let mut new_config: *mut ModuleConfig = createModuleConfig(
        config_name,
        applyfn,
        privdata,
        module,
    );
    sdsfree(config_name);
    (*new_config).get_fn.get_string = getfn;
    (*new_config).set_fn.set_string = setfn;
    listAddNodeTail((*module).module_configs, new_config as *mut libc::c_void);
    flags = maskModuleConfigFlags(flags);
    addModuleStringConfig(
        (*module).name,
        name,
        flags as libc::c_int,
        new_config as *mut libc::c_void,
        if !default_val.is_null() { sdsnew(default_val) } else { 0 as sds },
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterBoolConfig(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut default_val: libc::c_int,
    mut flags: libc::c_uint,
    mut getfn: RedisModuleConfigGetBoolFunc,
    mut setfn: RedisModuleConfigSetBoolFunc,
    mut applyfn: RedisModuleConfigApplyFunc,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    let mut module: *mut RedisModule = (*ctx).module;
    let mut config_name: sds = sdsnew(name);
    if moduleConfigValidityCheck(module, config_name, flags, BOOL_CONFIG) != 0 {
        sdsfree(config_name);
        return 1 as libc::c_int;
    }
    let mut new_config: *mut ModuleConfig = createModuleConfig(
        config_name,
        applyfn,
        privdata,
        module,
    );
    sdsfree(config_name);
    (*new_config).get_fn.get_bool = getfn;
    (*new_config).set_fn.set_bool = setfn;
    listAddNodeTail((*module).module_configs, new_config as *mut libc::c_void);
    flags = maskModuleConfigFlags(flags);
    addModuleBoolConfig(
        (*module).name,
        name,
        flags as libc::c_int,
        new_config as *mut libc::c_void,
        default_val,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterEnumConfig(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut default_val: libc::c_int,
    mut flags: libc::c_uint,
    mut enum_values: *mut *const libc::c_char,
    mut int_values: *const libc::c_int,
    mut num_enum_vals: libc::c_int,
    mut getfn: RedisModuleConfigGetEnumFunc,
    mut setfn: RedisModuleConfigSetEnumFunc,
    mut applyfn: RedisModuleConfigApplyFunc,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    let mut module: *mut RedisModule = (*ctx).module;
    let mut config_name: sds = sdsnew(name);
    if moduleConfigValidityCheck(module, config_name, flags, ENUM_CONFIG) != 0 {
        sdsfree(config_name);
        return 1 as libc::c_int;
    }
    let mut new_config: *mut ModuleConfig = createModuleConfig(
        config_name,
        applyfn,
        privdata,
        module,
    );
    sdsfree(config_name);
    (*new_config).get_fn.get_enum = getfn;
    (*new_config).set_fn.set_enum = setfn;
    let mut enum_vals: *mut configEnum = zmalloc(
        ((num_enum_vals + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(core::mem::size_of::<configEnum>() as libc::c_ulong),
    ) as *mut configEnum;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < num_enum_vals {
        let ref mut fresh40 = (*enum_vals.offset(i as isize)).name;
        *fresh40 = zstrdup(*enum_values.offset(i as isize));
        (*enum_vals.offset(i as isize)).val = *int_values.offset(i as isize);
        i += 1;
    }
    let ref mut fresh41 = (*enum_vals.offset(num_enum_vals as isize)).name;
    *fresh41 = 0 as *mut libc::c_char;
    (*enum_vals.offset(num_enum_vals as isize)).val = 0 as libc::c_int;
    listAddNodeTail((*module).module_configs, new_config as *mut libc::c_void);
    flags = maskModuleConfigFlags(flags) | maskModuleEnumConfigFlags(flags);
    addModuleEnumConfig(
        (*module).name,
        name,
        flags as libc::c_int,
        new_config as *mut libc::c_void,
        default_val,
        enum_vals,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterNumericConfig(
    mut ctx: *mut RedisModuleCtx,
    mut name: *const libc::c_char,
    mut default_val: libc::c_longlong,
    mut flags: libc::c_uint,
    mut min: libc::c_longlong,
    mut max: libc::c_longlong,
    mut getfn: RedisModuleConfigGetNumericFunc,
    mut setfn: RedisModuleConfigSetNumericFunc,
    mut applyfn: RedisModuleConfigApplyFunc,
    mut privdata: *mut libc::c_void,
) -> libc::c_int {
    let mut module: *mut RedisModule = (*ctx).module;
    let mut config_name: sds = sdsnew(name);
    if moduleConfigValidityCheck(module, config_name, flags, NUMERIC_CONFIG) != 0 {
        sdsfree(config_name);
        return 1 as libc::c_int;
    }
    let mut new_config: *mut ModuleConfig = createModuleConfig(
        config_name,
        applyfn,
        privdata,
        module,
    );
    sdsfree(config_name);
    (*new_config).get_fn.get_numeric = getfn;
    (*new_config).set_fn.set_numeric = setfn;
    listAddNodeTail((*module).module_configs, new_config as *mut libc::c_void);
    let mut numeric_flags: libc::c_uint = maskModuleNumericConfigFlags(flags);
    flags = maskModuleConfigFlags(flags);
    addModuleNumericConfig(
        (*module).name,
        name,
        flags as libc::c_int,
        new_config as *mut libc::c_void,
        default_val,
        numeric_flags as libc::c_int,
        min,
        max,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_LoadConfigs(mut ctx: *mut RedisModuleCtx) -> libc::c_int {
    if ctx.is_null() || ((*ctx).module).is_null() {
        return 1 as libc::c_int;
    }
    let mut module: *mut RedisModule = (*ctx).module;
    if loadModuleConfigs(module) != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleCommand(mut c: *mut client) {
    let mut subcmd: *mut libc::c_char = (**((*c).argv).offset(1 as libc::c_int as isize))
        .ptr as *mut libc::c_char;
    if (*c).argc == 2 as libc::c_int
        && strcasecmp(subcmd, b"help\0" as *const u8 as *const libc::c_char) == 0
    {
        let mut help: [*const libc::c_char; 9] = [
            b"LIST\0" as *const u8 as *const libc::c_char,
            b"    Return a list of loaded modules.\0" as *const u8
                as *const libc::c_char,
            b"LOAD <path> [<arg> ...]\0" as *const u8 as *const libc::c_char,
            b"    Load a module library from <path>, passing to it any optional arguments.\0"
                as *const u8 as *const libc::c_char,
            b"LOADEX <path> [[CONFIG NAME VALUE] [CONFIG NAME VALUE]] [ARGS ...]\0"
                as *const u8 as *const libc::c_char,
            b"    Load a module library from <path>, while passing it module configurations and optional arguments.\0"
                as *const u8 as *const libc::c_char,
            b"UNLOAD <name>\0" as *const u8 as *const libc::c_char,
            b"    Unload a module.\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        ];
        addReplyHelp(c, help.as_mut_ptr());
    } else if strcasecmp(subcmd, b"load\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc >= 3 as libc::c_int
    {
        let mut argv: *mut *mut robj = 0 as *mut *mut robj;
        let mut argc: libc::c_int = 0 as libc::c_int;
        if (*c).argc > 3 as libc::c_int {
            argc = (*c).argc - 3 as libc::c_int;
            argv = &mut *((*c).argv).offset(3 as libc::c_int as isize) as *mut *mut robj;
        }
        if moduleLoad(
            (**((*c).argv).offset(2 as libc::c_int as isize)).ptr as *const libc::c_char,
            argv as *mut *mut libc::c_void,
            argc,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {
            addReply(c, shared.ok);
        } else {
            addReplyError(
                c,
                b"Error loading the extension. Please check the server logs.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if strcasecmp(subcmd, b"loadex\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc >= 3 as libc::c_int
    {
        let mut argv_0: *mut *mut robj = 0 as *mut *mut robj;
        let mut argc_0: libc::c_int = 0 as libc::c_int;
        if (*c).argc > 3 as libc::c_int {
            argc_0 = (*c).argc - 3 as libc::c_int;
            argv_0 = &mut *((*c).argv).offset(3 as libc::c_int as isize)
                as *mut *mut robj;
        }
        if parseLoadexArguments(&mut argv_0 as *mut *mut *mut robj, &mut argc_0)
            == 0 as libc::c_int
            && moduleLoad(
                (**((*c).argv).offset(2 as libc::c_int as isize)).ptr
                    as *const libc::c_char,
                argv_0 as *mut *mut libc::c_void,
                argc_0,
                1 as libc::c_int,
            ) == 0 as libc::c_int
        {
            addReply(c, shared.ok);
        } else {
            dictEmpty(server.module_configs_queue, None);
            addReplyError(
                c,
                b"Error loading the extension. Please check the server logs.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    } else if strcasecmp(subcmd, b"unload\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 3 as libc::c_int
    {
        if moduleUnload((**((*c).argv).offset(2 as libc::c_int as isize)).ptr as sds)
            == 0 as libc::c_int
        {
            addReply(c, shared.ok);
        } else {
            let mut errmsg: *mut libc::c_char = 0 as *mut libc::c_char;
            match *__errno_location() {
                2 => {
                    errmsg = b"no such module with that name\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
                16 => {
                    errmsg = b"the module exports one or more module-side data types, can't unload\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                1 => {
                    errmsg = b"the module exports APIs used by other modules. Please unload them first and try again\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                11 => {
                    errmsg = b"the module has blocked clients. Please wait them unblocked and try again\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                115 => {
                    errmsg = b"the module holds timer that is not fired. Please stop the timer or wait until it fires.\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char;
                }
                _ => {
                    errmsg = b"operation not possible.\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                }
            }
            addReplyErrorFormat(
                c,
                b"Error unloading module: %s\0" as *const u8 as *const libc::c_char,
                errmsg,
            );
        }
    } else if strcasecmp(subcmd, b"list\0" as *const u8 as *const libc::c_char) == 0
        && (*c).argc == 2 as libc::c_int
    {
        addReplyLoadedModules(c);
    } else {
        addReplySubcommandSyntaxError(c);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn moduleCount() -> size_t {
    return ((*modules).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*modules).ht_used[1 as libc::c_int as usize]);
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetLRU(
    mut key: *mut RedisModuleKey,
    mut lru_idle: mstime_t,
) -> libc::c_int {
    if ((*key).value).is_null() {
        return 1 as libc::c_int;
    }
    if objectSetLRUOrLFU(
        (*key).value,
        -(1 as libc::c_int) as libc::c_longlong,
        lru_idle,
        (if lru_idle >= 0 as libc::c_int as libc::c_longlong {
            LRU_CLOCK()
        } else {
            0 as libc::c_int as libc::c_uint
        }) as libc::c_longlong,
        1 as libc::c_int,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetLRU(
    mut key: *mut RedisModuleKey,
    mut lru_idle: *mut mstime_t,
) -> libc::c_int {
    *lru_idle = -(1 as libc::c_int) as mstime_t;
    if ((*key).value).is_null() {
        return 1 as libc::c_int;
    }
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        return 0 as libc::c_int;
    }
    *lru_idle = estimateObjectIdleTime((*key).value) as mstime_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_SetLFU(
    mut key: *mut RedisModuleKey,
    mut lfu_freq: libc::c_longlong,
) -> libc::c_int {
    if ((*key).value).is_null() {
        return 1 as libc::c_int;
    }
    if objectSetLRUOrLFU(
        (*key).value,
        lfu_freq,
        -(1 as libc::c_int) as libc::c_longlong,
        0 as libc::c_int as libc::c_longlong,
        1 as libc::c_int,
    ) != 0
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetLFU(
    mut key: *mut RedisModuleKey,
    mut lfu_freq: *mut libc::c_longlong,
) -> libc::c_int {
    *lfu_freq = -(1 as libc::c_int) as libc::c_longlong;
    if ((*key).value).is_null() {
        return 1 as libc::c_int;
    }
    if server.maxmemory_policy & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        *lfu_freq = LFUDecrAndReturn((*key).value) as libc::c_longlong;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetContextFlagsAll() -> libc::c_int {
    return ((1 as libc::c_int) << 24 as libc::c_int) - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyspaceNotificationFlagsAll() -> libc::c_int {
    return ((1 as libc::c_int) << 15 as libc::c_int) - 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetServerVersion() -> libc::c_int {
    return 0x70008 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetTypeMethodVersion() -> libc::c_int {
    return 4 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_ModuleTypeReplaceValue(
    mut key: *mut RedisModuleKey,
    mut mt: *mut moduleType,
    mut new_value: *mut libc::c_void,
    mut old_value: *mut *mut libc::c_void,
) -> libc::c_int {
    if (*key).mode & (1 as libc::c_int) << 1 as libc::c_int == 0
        || !((*key).iter).is_null()
    {
        return 1 as libc::c_int;
    }
    if ((*key).value).is_null()
        || (*(*key).value).type_0() as libc::c_int != 5 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    let mut mv: *mut moduleValue = (*(*key).value).ptr as *mut moduleValue;
    if (*mv).type_0 != mt {
        return 1 as libc::c_int;
    }
    if !old_value.is_null() {
        *old_value = (*mv).value;
    }
    (*mv).value = new_value;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetCommandKeysWithFlags(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut num_keys: *mut libc::c_int,
    mut out_flags: *mut *mut libc::c_int,
) -> *mut libc::c_int {
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut res: *mut libc::c_int = 0 as *mut libc::c_int;
    cmd = lookupCommand(argv, argc);
    if cmd.is_null() {
        *__errno_location() = 2 as libc::c_int;
        return 0 as *mut libc::c_int;
    }
    if doesCommandHaveKeys(cmd) == 0 {
        *__errno_location() = 0 as libc::c_int;
        return 0 as *mut libc::c_int;
    }
    if (*cmd).arity > 0 as libc::c_int && (*cmd).arity != argc || argc < -(*cmd).arity {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_int;
    }
    let mut result: getKeysResult = {
        let mut init = getKeysResult {
            keysbuf: [
                {
                    let mut init = keyReference {
                        pos: 0 as libc::c_int,
                        flags: 0,
                    };
                    init
                },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
                keyReference { pos: 0, flags: 0 },
            ],
            keys: 0 as *mut keyReference,
            numkeys: 0 as libc::c_int,
            size: 256 as libc::c_int,
        };
        init
    };
    getKeysFromCommand(cmd, argv, argc, &mut result);
    *num_keys = result.numkeys;
    if result.numkeys == 0 {
        *__errno_location() = 0 as libc::c_int;
        getKeysFreeResult(&mut result);
        return 0 as *mut libc::c_int;
    }
    let mut size: libc::c_ulong = (core::mem::size_of::<libc::c_int>()
        as libc::c_ulong)
        .wrapping_mul(result.numkeys as libc::c_ulong);
    res = zmalloc(size) as *mut libc::c_int;
    if !out_flags.is_null() {
        *out_flags = zmalloc(size) as *mut libc::c_int;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < result.numkeys {
        *res.offset(i as isize) = (*(result.keys).offset(i as isize)).pos;
        if !out_flags.is_null() {
            *(*out_flags)
                .offset(
                    i as isize,
                ) = moduleConvertKeySpecsFlags(
                (*(result.keys).offset(i as isize)).flags as int64_t,
                0 as libc::c_int,
            ) as libc::c_int;
        }
        i += 1;
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetCommandKeys(
    mut ctx: *mut RedisModuleCtx,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut num_keys: *mut libc::c_int,
) -> *mut libc::c_int {
    return RM_GetCommandKeysWithFlags(
        ctx,
        argv,
        argc,
        num_keys,
        0 as *mut *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetCurrentCommandName(
    mut ctx: *mut RedisModuleCtx,
) -> *const libc::c_char {
    if ctx.is_null() || ((*ctx).client).is_null() || ((*(*ctx).client).cmd).is_null() {
        return 0 as *const libc::c_char;
    }
    return (*(*(*ctx).client).cmd).fullname as *const libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn RM_RegisterDefragFunc(
    mut ctx: *mut RedisModuleCtx,
    mut cb: RedisModuleDefragFunc,
) -> libc::c_int {
    (*(*ctx).module).defrag_cb = cb;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DefragShouldStop(
    mut ctx: *mut RedisModuleDefragCtx,
) -> libc::c_int {
    return ((*ctx).endtime != 0 as libc::c_int as libc::c_longlong
        && (*ctx).endtime < ustime()) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DefragCursorSet(
    mut ctx: *mut RedisModuleDefragCtx,
    mut cursor: libc::c_ulong,
) -> libc::c_int {
    if ((*ctx).cursor).is_null() {
        return 1 as libc::c_int;
    }
    *(*ctx).cursor = cursor;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DefragCursorGet(
    mut ctx: *mut RedisModuleDefragCtx,
    mut cursor: *mut libc::c_ulong,
) -> libc::c_int {
    if ((*ctx).cursor).is_null() {
        return 1 as libc::c_int;
    }
    *cursor = *(*ctx).cursor;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DefragAlloc(
    mut ctx: *mut RedisModuleDefragCtx,
    mut ptr: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut newptr: *mut libc::c_void = activeDefragAlloc(ptr);
    if !newptr.is_null() {
        (*ctx).defragged += 1;
    }
    return newptr;
}
#[no_mangle]
pub unsafe extern "C" fn RM_DefragRedisModuleString(
    mut ctx: *mut RedisModuleDefragCtx,
    mut str: *mut robj,
) -> *mut robj {
    return activeDefragStringOb(str, &mut (*ctx).defragged);
}
#[no_mangle]
pub unsafe extern "C" fn moduleLateDefrag(
    mut key: *mut robj,
    mut value: *mut robj,
    mut cursor: *mut libc::c_ulong,
    mut endtime: libc::c_longlong,
    mut defragged: *mut libc::c_longlong,
    mut dbid: libc::c_int,
) -> libc::c_int {
    let mut mv: *mut moduleValue = (*value).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    let mut defrag_ctx: RedisModuleDefragCtx = {
        let mut init = RedisModuleDefragCtx {
            defragged: 0 as libc::c_int as libc::c_long,
            endtime: endtime,
            cursor: cursor,
            key: key,
            dbid: dbid,
        };
        init
    };
    let mut ret: libc::c_int = 0 as libc::c_int;
    if ((*mt).defrag).is_some() {
        ret = ((*mt).defrag)
            .expect("non-null function pointer")(&mut defrag_ctx, key, &mut (*mv).value);
    }
    *defragged += defrag_ctx.defragged as libc::c_longlong;
    if ret == 0 {
        *cursor = 0 as libc::c_int as libc::c_ulong;
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleDefragValue(
    mut key: *mut robj,
    mut value: *mut robj,
    mut defragged: *mut libc::c_long,
    mut dbid: libc::c_int,
) -> libc::c_int {
    let mut mv: *mut moduleValue = (*value).ptr as *mut moduleValue;
    let mut mt: *mut moduleType = (*mv).type_0;
    let mut newmv: *mut moduleValue = activeDefragAlloc(mv as *mut libc::c_void)
        as *mut moduleValue;
    if !newmv.is_null() {
        *defragged += 1;
        mv = newmv;
        (*value).ptr = mv as *mut libc::c_void;
    }
    if ((*mt).defrag).is_none() {
        return 1 as libc::c_int;
    }
    let mut effort: size_t = moduleGetFreeEffort(key, value, dbid);
    if effort == 0 {
        effort = 18446744073709551615 as libc::c_ulong;
    }
    if effort > server.active_defrag_max_scan_fields {
        return 0 as libc::c_int;
    }
    let mut defrag_ctx: RedisModuleDefragCtx = {
        let mut init = RedisModuleDefragCtx {
            defragged: 0 as libc::c_int as libc::c_long,
            endtime: 0 as libc::c_int as libc::c_longlong,
            cursor: 0 as *mut libc::c_ulong,
            key: key,
            dbid: dbid,
        };
        init
    };
    ((*mt).defrag)
        .expect("non-null function pointer")(&mut defrag_ctx, key, &mut (*mv).value);
    *defragged += defrag_ctx.defragged;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn moduleDefragGlobals() -> libc::c_long {
    let mut di: *mut dictIterator = dictGetIterator(modules);
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut defragged: libc::c_long = 0 as libc::c_int as libc::c_long;
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut module: *mut RedisModule = (*de).v.val as *mut RedisModule;
        if ((*module).defrag_cb).is_none() {
            continue;
        }
        let mut defrag_ctx: RedisModuleDefragCtx = {
            let mut init = RedisModuleDefragCtx {
                defragged: 0 as libc::c_int as libc::c_long,
                endtime: 0 as libc::c_int as libc::c_longlong,
                cursor: 0 as *mut libc::c_ulong,
                key: 0 as *mut redisObject,
                dbid: -(1 as libc::c_int),
            };
            init
        };
        ((*module).defrag_cb).expect("non-null function pointer")(&mut defrag_ctx);
        defragged += defrag_ctx.defragged;
    }
    dictReleaseIterator(di);
    return defragged;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetKeyNameFromDefragCtx(
    mut ctx: *mut RedisModuleDefragCtx,
) -> *const robj {
    return (*ctx).key;
}
#[no_mangle]
pub unsafe extern "C" fn RM_GetDbIdFromDefragCtx(
    mut ctx: *mut RedisModuleDefragCtx,
) -> libc::c_int {
    return (*ctx).dbid;
}
#[no_mangle]
pub unsafe extern "C" fn moduleRegisterCoreAPI() {
    server.moduleapi = dictCreate(&mut moduleAPIDictType);
    server.sharedapi = dictCreate(&mut moduleAPIDictType);
    moduleRegisterApi(
        b"RedisModule_Alloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
            libc::c_ulong,
        >(Some(RM_Alloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_TryAlloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
            libc::c_ulong,
        >(Some(RM_TryAlloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Calloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void>,
            libc::c_ulong,
        >(Some(RM_Calloc as unsafe extern "C" fn(size_t, size_t) -> *mut libc::c_void))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Realloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Realloc
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Free\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
            libc::c_ulong,
        >(Some(RM_Free as unsafe extern "C" fn(*mut libc::c_void) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Strdup\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char>,
            libc::c_ulong,
        >(
            Some(
                RM_Strdup
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateCommand\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    RedisModuleCmdFunc,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateCommand
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        RedisModuleCmdFunc,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetCommand\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> *mut RedisModuleCommand,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetCommand
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> *mut RedisModuleCommand,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateSubcommand\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommand,
                    *const libc::c_char,
                    RedisModuleCmdFunc,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateSubcommand
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommand,
                        *const libc::c_char,
                        RedisModuleCmdFunc,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetCommandInfo\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommand,
                    *const RedisModuleCommandInfo,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetCommandInfo
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommand,
                        *const RedisModuleCommandInfo,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetModuleAttribs\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_int,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetModuleAttribs
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsModuleNameBusy\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsModuleNameBusy
                    as unsafe extern "C" fn(*const libc::c_char) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_WrongArity\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_WrongArity as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithLongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_longlong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithLongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_longlong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithError\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithError
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithSimpleString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithSimpleString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithArray\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithArray
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithMap\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithMap
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithAttribute\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithAttribute
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithNullArray\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithNullArray
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithEmptyArray\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithEmptyArray
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplySetArrayLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplySetArrayLength
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplySetMapLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplySetMapLength
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplySetSetLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplySetSetLength
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplySetAttributeLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplySetAttributeLength
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_long) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithEmptyString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithEmptyString
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithVerbatimString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithVerbatimString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithVerbatimStringType\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithVerbatimStringType
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithStringBuffer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithStringBuffer
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithCString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithCString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithNull\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithNull
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithBool\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithBool
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithCallReply\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleCallReply,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithCallReply
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleCallReply,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithBigNumber\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithBigNumber
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplyWithLongDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, f64) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ReplyWithLongDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        f64,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetSelectedDb\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetSelectedDb
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SelectDb\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SelectDb
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_KeyExists\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_KeyExists
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_OpenKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut robj,
                    libc::c_int,
                ) -> *mut RedisModuleKey,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_OpenKey
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                        libc::c_int,
                    ) -> *mut RedisModuleKey,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CloseKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>,
            libc::c_ulong,
        >(Some(RM_CloseKey as unsafe extern "C" fn(*mut RedisModuleKey) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_KeyType\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_KeyType as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ValueLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> size_t>,
            libc::c_ulong,
        >(Some(RM_ValueLength as unsafe extern "C" fn(*mut RedisModuleKey) -> size_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListPush\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListPush
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListPop\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, libc::c_int) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListPop
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListGet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, libc::c_long) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListGet
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_long,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_long,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_long,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListInsert\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_long,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListInsert
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_long,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ListDelete\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, libc::c_long) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ListDelete
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringToLongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const robj, *mut libc::c_longlong) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringToLongLong
                    as unsafe extern "C" fn(
                        *const robj,
                        *mut libc::c_longlong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringToULongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const robj, *mut libc::c_ulonglong) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringToULongLong
                    as unsafe extern "C" fn(
                        *const robj,
                        *mut libc::c_ulonglong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringToDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const robj, *mut libc::c_double) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringToDouble
                    as unsafe extern "C" fn(
                        *const robj,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringToLongDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const robj, *mut f64) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_StringToLongDouble
                    as unsafe extern "C" fn(*const robj, *mut f64) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringToStreamID\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const robj,
                    *mut RedisModuleStreamID,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringToStreamID
                    as unsafe extern "C" fn(
                        *const robj,
                        *mut RedisModuleStreamID,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Call\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> *mut RedisModuleCallReply,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Call
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *const libc::c_char,
                        ...
                    ) -> *mut RedisModuleCallReply,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyProto\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyProto
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeCallReply\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_FreeCallReply as unsafe extern "C" fn(*mut RedisModuleCallReply) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyInteger\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_longlong,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyInteger
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                    ) -> libc::c_longlong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_double>,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyDouble
                    as unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_double,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyBigNumber\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyBigNumber
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyVerbatim\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                    *mut *const libc::c_char,
                ) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyVerbatim
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        *mut size_t,
                        *mut *const libc::c_char,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyBool\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyBool
                    as unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplySetElement\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                ) -> *mut RedisModuleCallReply,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplySetElement
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        size_t,
                    ) -> *mut RedisModuleCallReply,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyMapElement\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                    *mut *mut RedisModuleCallReply,
                    *mut *mut RedisModuleCallReply,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyMapElement
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        size_t,
                        *mut *mut RedisModuleCallReply,
                        *mut *mut RedisModuleCallReply,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyAttributeElement\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                    *mut *mut RedisModuleCallReply,
                    *mut *mut RedisModuleCallReply,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyAttributeElement
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        size_t,
                        *mut *mut RedisModuleCallReply,
                        *mut *mut RedisModuleCallReply,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyAttribute\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                ) -> *mut RedisModuleCallReply,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyAttribute
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                    ) -> *mut RedisModuleCallReply,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyType\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyType
                    as unsafe extern "C" fn(*mut RedisModuleCallReply) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t>,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyLength
                    as unsafe extern "C" fn(*mut RedisModuleCallReply) -> size_t,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyArrayElement\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    size_t,
                ) -> *mut RedisModuleCallReply,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyArrayElement
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        size_t,
                    ) -> *mut RedisModuleCallReply,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CallReplyStringPtr\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCallReply,
                    *mut size_t,
                ) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CallReplyStringPtr
                    as unsafe extern "C" fn(
                        *mut RedisModuleCallReply,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromCallReply\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromCallReply
                    as unsafe extern "C" fn(*mut RedisModuleCallReply) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromLongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_longlong) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromLongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_longlong,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromULongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_ulonglong) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromULongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_ulonglong,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_double) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_double,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromLongDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    f64,
                    libc::c_int,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromLongDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        f64,
                        libc::c_int,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *const robj) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const robj,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringFromStreamID\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const RedisModuleStreamID,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringFromStreamID
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const RedisModuleStreamID,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateStringPrintf\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    ...
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateStringPrintf
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        ...
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_FreeString
                    as unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringPtrLen\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const robj, *mut size_t) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringPtrLen
                    as unsafe extern "C" fn(
                        *const robj,
                        *mut size_t,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_AutoMemory\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>,
            libc::c_ulong,
        >(Some(RM_AutoMemory as unsafe extern "C" fn(*mut RedisModuleCtx) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Replicate\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Replicate
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *const libc::c_char,
                        ...
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ReplicateVerbatim\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ReplicateVerbatim
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DeleteKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_DeleteKey as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_UnlinkKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_UnlinkKey as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, *mut robj) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringDMA\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut size_t,
                    libc::c_int,
                ) -> *mut libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringDMA
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut size_t,
                        libc::c_int,
                    ) -> *mut libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringTruncate\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_StringTruncate
                    as unsafe extern "C" fn(*mut RedisModuleKey, size_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetExpire\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_SetExpire
                    as unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetExpire\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t>,
            libc::c_ulong,
        >(Some(RM_GetExpire as unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetAbsExpire\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_SetAbsExpire
                    as unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetAbsExpire\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t>,
            libc::c_ulong,
        >(Some(RM_GetAbsExpire as unsafe extern "C" fn(*mut RedisModuleKey) -> mstime_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ResetDataset\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()>,
            libc::c_ulong,
        >(Some(RM_ResetDataset as unsafe extern "C" fn(libc::c_int, libc::c_int) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DbSize\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong>,
            libc::c_ulong,
        >(
            Some(
                RM_DbSize
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RandomKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj>,
            libc::c_ulong,
        >(Some(RM_RandomKey as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetAdd\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut robj,
                    *mut libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetAdd
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_double,
                        *mut robj,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetIncrby\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    *mut robj,
                    *mut libc::c_int,
                    *mut libc::c_double,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetIncrby
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_double,
                        *mut robj,
                        *mut libc::c_int,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetScore\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut robj,
                    *mut libc::c_double,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetScore
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut robj,
                        *mut libc::c_double,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRem\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut robj,
                    *mut libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetRem
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut robj,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRangeStop\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> ()>,
            libc::c_ulong,
        >(Some(RM_ZsetRangeStop as unsafe extern "C" fn(*mut RedisModuleKey) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetFirstInScoreRange\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetFirstInScoreRange
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_double,
                        libc::c_double,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetLastInScoreRange\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_double,
                    libc::c_double,
                    libc::c_int,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetLastInScoreRange
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_double,
                        libc::c_double,
                        libc::c_int,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetFirstInLexRange\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut robj,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetFirstInLexRange
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut robj,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetLastInLexRange\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut robj,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetLastInLexRange
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut robj,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRangeCurrentElement\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut libc::c_double,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetRangeCurrentElement
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut libc::c_double,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRangeNext\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetRangeNext
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRangePrev\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetRangePrev
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ZsetRangeEndReached\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ZsetRangeEndReached
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_HashSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_HashSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        ...
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_HashGet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    ...
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_HashGet
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        ...
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamAdd\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut RedisModuleStreamID,
                    *mut *mut robj,
                    libc::c_long,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamAdd
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        *mut RedisModuleStreamID,
                        *mut *mut robj,
                        libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamDelete\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleStreamID,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamDelete
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut RedisModuleStreamID,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamIteratorStart\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut RedisModuleStreamID,
                    *mut RedisModuleStreamID,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamIteratorStart
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        *mut RedisModuleStreamID,
                        *mut RedisModuleStreamID,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamIteratorStop\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_StreamIteratorStop
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamIteratorNextID\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleStreamID,
                    *mut libc::c_long,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamIteratorNextID
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut RedisModuleStreamID,
                        *mut libc::c_long,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamIteratorNextField\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut *mut robj,
                    *mut *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamIteratorNextField
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut *mut robj,
                        *mut *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamIteratorDelete\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_StreamIteratorDelete
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamTrimByLength\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    libc::c_longlong,
                ) -> libc::c_longlong,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamTrimByLength
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        libc::c_longlong,
                    ) -> libc::c_longlong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StreamTrimByID\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_int,
                    *mut RedisModuleStreamID,
                ) -> libc::c_longlong,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StreamTrimByID
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_int,
                        *mut RedisModuleStreamID,
                    ) -> libc::c_longlong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsKeysPositionRequest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsKeysPositionRequest
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_KeyAtPos\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_KeyAtPos
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_KeyAtPosWithFlags\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int, libc::c_int) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_KeyAtPosWithFlags
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsChannelsPositionRequest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsChannelsPositionRequest
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ChannelAtPosWithFlags\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int, libc::c_int) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ChannelAtPosWithFlags
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                        libc::c_int,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClientId\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong>,
            libc::c_ulong,
        >(
            Some(
                RM_GetClientId
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_ulonglong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClientUserNameById\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetClientUserNameById
                    as unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetContextFlags\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetContextFlags
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_AvoidReplicaTraffic\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_AvoidReplicaTraffic),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_PoolAlloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, size_t) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_PoolAlloc
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        size_t,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateDataType\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut moduleType,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateDataType
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> *mut moduleType,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ModuleTypeSetValue\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut moduleType,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ModuleTypeSetValue
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut moduleType,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ModuleTypeReplaceValue\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut moduleType,
                    *mut libc::c_void,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ModuleTypeReplaceValue
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut moduleType,
                        *mut libc::c_void,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ModuleTypeGetType\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> *mut moduleType>,
            libc::c_ulong,
        >(
            Some(
                RM_ModuleTypeGetType
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> *mut moduleType,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ModuleTypeGetValue\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void>,
            libc::c_ulong,
        >(
            Some(
                RM_ModuleTypeGetValue
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsIOError\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_IsIOError as unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetModuleOptions\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SetModuleOptions
                    as unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SignalModifiedKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SignalModifiedKey
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveUnsigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SaveUnsigned
                    as unsafe extern "C" fn(*mut RedisModuleIO, uint64_t) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadUnsigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t>,
            libc::c_ulong,
        >(Some(RM_LoadUnsigned as unsafe extern "C" fn(*mut RedisModuleIO) -> uint64_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveSigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> ()>,
            libc::c_ulong,
        >(Some(RM_SaveSigned as unsafe extern "C" fn(*mut RedisModuleIO, int64_t) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadSigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t>,
            libc::c_ulong,
        >(Some(RM_LoadSigned as unsafe extern "C" fn(*mut RedisModuleIO) -> int64_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, *mut robj) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SaveString
                    as unsafe extern "C" fn(*mut RedisModuleIO, *mut robj) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveStringBuffer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    size_t,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SaveStringBuffer
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> *mut robj>,
            libc::c_ulong,
        >(Some(RM_LoadString as unsafe extern "C" fn(*mut RedisModuleIO) -> *mut robj))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadStringBuffer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *mut size_t,
                ) -> *mut libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_LoadStringBuffer
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        *mut size_t,
                    ) -> *mut libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SaveDouble
                    as unsafe extern "C" fn(*mut RedisModuleIO, libc::c_double) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double>,
            libc::c_ulong,
        >(
            Some(
                RM_LoadDouble
                    as unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_double,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveFloat\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SaveFloat
                    as unsafe extern "C" fn(*mut RedisModuleIO, libc::c_float) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadFloat\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float>,
            libc::c_ulong,
        >(
            Some(
                RM_LoadFloat as unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_float,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveLongDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO, f64) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SaveLongDouble
                    as unsafe extern "C" fn(*mut RedisModuleIO, f64) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadLongDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> f64>,
            libc::c_ulong,
        >(
            Some(
                RM_LoadLongDouble
                    as unsafe extern "C" fn(*mut RedisModuleIO) -> f64,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SaveDataTypeToString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut libc::c_void,
                    *const moduleType,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SaveDataTypeToString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut libc::c_void,
                        *const moduleType,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadDataTypeFromString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*const robj, *const moduleType) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_LoadDataTypeFromString
                    as unsafe extern "C" fn(
                        *const robj,
                        *const moduleType,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadDataTypeFromStringEncver\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const robj,
                    *const moduleType,
                    libc::c_int,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_LoadDataTypeFromStringEncver
                    as unsafe extern "C" fn(
                        *const robj,
                        *const moduleType,
                        libc::c_int,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_EmitAOF\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_EmitAOF
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        *const libc::c_char,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Log\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Log
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LogIOError\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleIO,
                    *const libc::c_char,
                    *const libc::c_char,
                    ...
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_LogIOError
                    as unsafe extern "C" fn(
                        *mut RedisModuleIO,
                        *const libc::c_char,
                        *const libc::c_char,
                        ...
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule__Assert\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_int,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM__Assert
                    as unsafe extern "C" fn(
                        *const libc::c_char,
                        *const libc::c_char,
                        libc::c_int,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LatencyAddSample\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char, mstime_t) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_LatencyAddSample
                    as unsafe extern "C" fn(*const libc::c_char, mstime_t) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringAppendBuffer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut robj,
                    *const libc::c_char,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StringAppendBuffer
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                        *const libc::c_char,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_TrimStringAllocation\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut robj) -> ()>,
            libc::c_ulong,
        >(Some(RM_TrimStringAllocation as unsafe extern "C" fn(*mut robj) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RetainString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_RetainString
                    as unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_HoldString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_HoldString
                    as unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StringCompare\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut robj, *mut robj) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_StringCompare
                    as unsafe extern "C" fn(*mut robj, *mut robj) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetContextFromIO\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx>,
            libc::c_ulong,
        >(
            Some(
                RM_GetContextFromIO
                    as unsafe extern "C" fn(*mut RedisModuleIO) -> *mut RedisModuleCtx,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyNameFromIO\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetKeyNameFromIO
                    as unsafe extern "C" fn(*mut RedisModuleIO) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyNameFromModuleKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetKeyNameFromModuleKey
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDbIdFromModuleKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDbIdFromModuleKey
                    as unsafe extern "C" fn(*mut RedisModuleKey) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDbIdFromIO\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDbIdFromIO
                    as unsafe extern "C" fn(*mut RedisModuleIO) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyNameFromOptCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetKeyNameFromOptCtx
                    as unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetToKeyNameFromOptCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetToKeyNameFromOptCtx
                    as unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDbIdFromOptCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDbIdFromOptCtx
                    as unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetToDbIdFromOptCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetToDbIdFromOptCtx
                    as unsafe extern "C" fn(*mut RedisModuleKeyOptCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyNameFromDefragCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetKeyNameFromDefragCtx
                    as unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDbIdFromDefragCtx\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDbIdFromDefragCtx
                    as unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyNameFromDigest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDigest) -> *const robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetKeyNameFromDigest
                    as unsafe extern "C" fn(*mut RedisModuleDigest) -> *const robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDbIdFromDigest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDigest) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDbIdFromDigest
                    as unsafe extern "C" fn(*mut RedisModuleDigest) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_BlockClient\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleCmdFunc,
                    RedisModuleCmdFunc,
                    Option::<
                        unsafe extern "C" fn(
                            *mut RedisModuleCtx,
                            *mut libc::c_void,
                        ) -> (),
                    >,
                    libc::c_longlong,
                ) -> *mut RedisModuleBlockedClient,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_BlockClient
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleCmdFunc,
                        RedisModuleCmdFunc,
                        Option::<
                            unsafe extern "C" fn(
                                *mut RedisModuleCtx,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        libc::c_longlong,
                    ) -> *mut RedisModuleBlockedClient,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_UnblockClient\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleBlockedClient,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_UnblockClient
                    as unsafe extern "C" fn(
                        *mut RedisModuleBlockedClient,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsBlockedReplyRequest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsBlockedReplyRequest
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsBlockedTimeoutRequest\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsBlockedTimeoutRequest
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetBlockedClientPrivateData\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void>,
            libc::c_ulong,
        >(
            Some(
                RM_GetBlockedClientPrivateData
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_AbortBlock\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_AbortBlock
                    as unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Milliseconds\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_longlong>,
            libc::c_ulong,
        >(Some(RM_Milliseconds as unsafe extern "C" fn() -> libc::c_longlong))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_MonotonicMicroseconds\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> uint64_t>,
            libc::c_ulong,
        >(Some(RM_MonotonicMicroseconds as unsafe extern "C" fn() -> uint64_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_BlockedClientMeasureTimeStart\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_BlockedClientMeasureTimeStart
                    as unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_BlockedClientMeasureTimeEnd\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_BlockedClientMeasureTimeEnd
                    as unsafe extern "C" fn(*mut RedisModuleBlockedClient) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetThreadSafeContext\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleBlockedClient,
                ) -> *mut RedisModuleCtx,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetThreadSafeContext
                    as unsafe extern "C" fn(
                        *mut RedisModuleBlockedClient,
                    ) -> *mut RedisModuleCtx,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetDetachedThreadSafeContext\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut RedisModuleCtx>,
            libc::c_ulong,
        >(
            Some(
                RM_GetDetachedThreadSafeContext
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut RedisModuleCtx,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeThreadSafeContext\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_FreeThreadSafeContext
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ThreadSafeContextLock\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ThreadSafeContextLock
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ThreadSafeContextTryLock\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_ThreadSafeContextTryLock
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ThreadSafeContextUnlock\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ThreadSafeContextUnlock
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DigestAddStringBuffer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDigest,
                    *const libc::c_char,
                    size_t,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DigestAddStringBuffer
                    as unsafe extern "C" fn(
                        *mut RedisModuleDigest,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DigestAddLongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleDigest, libc::c_longlong) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DigestAddLongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleDigest,
                        libc::c_longlong,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DigestEndSequence\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDigest) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_DigestEndSequence
                    as unsafe extern "C" fn(*mut RedisModuleDigest) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_NotifyKeyspaceEvent\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_int,
                    *const libc::c_char,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_NotifyKeyspaceEvent
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                        *const libc::c_char,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetNotifyKeyspaceEvents\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_GetNotifyKeyspaceEvents),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SubscribeToKeyspaceEvents\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_int,
                    RedisModuleNotificationFunc,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SubscribeToKeyspaceEvents
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                        RedisModuleNotificationFunc,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterClusterMessageReceiver\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    uint8_t,
                    RedisModuleClusterMessageReceiver,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterClusterMessageReceiver
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        uint8_t,
                        RedisModuleClusterMessageReceiver,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SendClusterMessage\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    uint8_t,
                    *const libc::c_char,
                    uint32_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SendClusterMessage
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        uint8_t,
                        *const libc::c_char,
                        uint32_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClusterNodeInfo\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *mut libc::c_char,
                    *mut libc::c_char,
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetClusterNodeInfo
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_char,
                        *mut libc::c_int,
                        *mut libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClusterNodesList\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut size_t,
                ) -> *mut *mut libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetClusterNodesList
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut size_t,
                    ) -> *mut *mut libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeClusterNodesList\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut *mut libc::c_char) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_FreeClusterNodesList
                    as unsafe extern "C" fn(*mut *mut libc::c_char) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateTimer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    mstime_t,
                    RedisModuleTimerProc,
                    *mut libc::c_void,
                ) -> RedisModuleTimerID,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CreateTimer
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        mstime_t,
                        RedisModuleTimerProc,
                        *mut libc::c_void,
                    ) -> RedisModuleTimerID,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_StopTimer\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleTimerID,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_StopTimer
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleTimerID,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetTimerInfo\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleTimerID,
                    *mut uint64_t,
                    *mut *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetTimerInfo
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleTimerID,
                        *mut uint64_t,
                        *mut *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetMyClusterID\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *const libc::c_char>,
            libc::c_ulong,
        >(Some(RM_GetMyClusterID as unsafe extern "C" fn() -> *const libc::c_char))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClusterSize\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> size_t>,
            libc::c_ulong,
        >(Some(RM_GetClusterSize as unsafe extern "C" fn() -> size_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetRandomBytes\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_uchar, size_t) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_GetRandomBytes
                    as unsafe extern "C" fn(*mut libc::c_uchar, size_t) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetRandomHexChars\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_char, size_t) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_GetRandomHexChars
                    as unsafe extern "C" fn(*mut libc::c_char, size_t) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_BlockedClientDisconnected\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_BlockedClientDisconnected
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetDisconnectCallback\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleBlockedClient,
                    RedisModuleDisconnectFunc,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetDisconnectCallback
                    as unsafe extern "C" fn(
                        *mut RedisModuleBlockedClient,
                        RedisModuleDisconnectFunc,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetBlockedClientHandle\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                ) -> *mut RedisModuleBlockedClient,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetBlockedClientHandle
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                    ) -> *mut RedisModuleBlockedClient,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetClusterFlags\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SetClusterFlags
                    as unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateDict\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut RedisModuleDict>,
            libc::c_ulong,
        >(
            Some(
                RM_CreateDict
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut RedisModuleDict,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeDict\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *mut RedisModuleDict) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_FreeDict
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleDict,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictSize\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDict) -> uint64_t>,
            libc::c_ulong,
        >(Some(RM_DictSize as unsafe extern "C" fn(*mut RedisModuleDict) -> uint64_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictSetC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictSetC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictReplaceC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictReplaceC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut robj,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut robj,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictReplace\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut robj,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictReplace
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut robj,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictGetC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_int,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictGetC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictGet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut robj,
                    *mut libc::c_int,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictGet
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut robj,
                        *mut libc::c_int,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictDelC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut libc::c_void,
                    size_t,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictDelC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut libc::c_void,
                        size_t,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictDel\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *mut robj,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictDel
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *mut robj,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictIteratorStartC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *const libc::c_char,
                    *mut libc::c_void,
                    size_t,
                ) -> *mut RedisModuleDictIter,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictIteratorStartC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *const libc::c_char,
                        *mut libc::c_void,
                        size_t,
                    ) -> *mut RedisModuleDictIter,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictIteratorStart\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDict,
                    *const libc::c_char,
                    *mut robj,
                ) -> *mut RedisModuleDictIter,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictIteratorStart
                    as unsafe extern "C" fn(
                        *mut RedisModuleDict,
                        *const libc::c_char,
                        *mut robj,
                    ) -> *mut RedisModuleDictIter,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictIteratorStop\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDictIter) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_DictIteratorStop
                    as unsafe extern "C" fn(*mut RedisModuleDictIter) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictIteratorReseekC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *const libc::c_char,
                    *mut libc::c_void,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictIteratorReseekC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *const libc::c_char,
                        *mut libc::c_void,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictIteratorReseek\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *const libc::c_char,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictIteratorReseek
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *const libc::c_char,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictNextC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *mut size_t,
                    *mut *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictNextC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *mut size_t,
                        *mut *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictPrevC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *mut size_t,
                    *mut *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictPrevC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *mut size_t,
                        *mut *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictNext\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleDictIter,
                    *mut *mut libc::c_void,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictNext
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleDictIter,
                        *mut *mut libc::c_void,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictPrev\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleDictIter,
                    *mut *mut libc::c_void,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictPrev
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleDictIter,
                        *mut *mut libc::c_void,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictCompareC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *const libc::c_char,
                    *mut libc::c_void,
                    size_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictCompareC
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *const libc::c_char,
                        *mut libc::c_void,
                        size_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DictCompare\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDictIter,
                    *const libc::c_char,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DictCompare
                    as unsafe extern "C" fn(
                        *mut RedisModuleDictIter,
                        *const libc::c_char,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ExportSharedAPI\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ExportSharedAPI
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetSharedAPI\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetSharedAPI
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterCommandFilter\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleCommandFilterFunc,
                    libc::c_int,
                ) -> *mut RedisModuleCommandFilter,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterCommandFilter
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleCommandFilterFunc,
                        libc::c_int,
                    ) -> *mut RedisModuleCommandFilter,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_UnregisterCommandFilter\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleCommandFilter,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_UnregisterCommandFilter
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleCommandFilter,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CommandFilterArgsCount\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCommandFilterCtx) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CommandFilterArgsCount
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommandFilterCtx,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CommandFilterArgGet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommandFilterCtx,
                    libc::c_int,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CommandFilterArgGet
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommandFilterCtx,
                        libc::c_int,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CommandFilterArgInsert\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommandFilterCtx,
                    libc::c_int,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CommandFilterArgInsert
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommandFilterCtx,
                        libc::c_int,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CommandFilterArgReplace\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommandFilterCtx,
                    libc::c_int,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CommandFilterArgReplace
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommandFilterCtx,
                        libc::c_int,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CommandFilterArgDelete\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCommandFilterCtx,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_CommandFilterArgDelete
                    as unsafe extern "C" fn(
                        *mut RedisModuleCommandFilterCtx,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Fork\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    RedisModuleForkDoneHandler,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Fork
                    as unsafe extern "C" fn(
                        RedisModuleForkDoneHandler,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SendChildHeartbeat\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_double) -> ()>,
            libc::c_ulong,
        >(Some(RM_SendChildHeartbeat as unsafe extern "C" fn(libc::c_double) -> ()))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ExitFromChild\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_ExitFromChild as unsafe extern "C" fn(libc::c_int) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_KillForkChild\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int) -> libc::c_int>,
            libc::c_ulong,
        >(Some(RM_KillForkChild as unsafe extern "C" fn(libc::c_int) -> libc::c_int))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterInfoFunc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleInfoFunc,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterInfoFunc
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleInfoFunc,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddSection\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddSection
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoBeginDictField\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoBeginDictField
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoEndDictField\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleInfoCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_InfoEndDictField
                    as unsafe extern "C" fn(*mut RedisModuleInfoCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddFieldString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddFieldString
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddFieldCString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddFieldCString
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddFieldDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                    libc::c_double,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddFieldDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                        libc::c_double,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddFieldLongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                    libc::c_longlong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddFieldLongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                        libc::c_longlong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_InfoAddFieldULongLong\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleInfoCtx,
                    *const libc::c_char,
                    libc::c_ulonglong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_InfoAddFieldULongLong
                    as unsafe extern "C" fn(
                        *mut RedisModuleInfoCtx,
                        *const libc::c_char,
                        libc::c_ulonglong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetServerInfo\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                ) -> *mut RedisModuleServerInfoData,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetServerInfo
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                    ) -> *mut RedisModuleServerInfoData,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeServerInfo\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleServerInfoData,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_FreeServerInfo
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleServerInfoData,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ServerInfoGetField\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleServerInfoData,
                    *const libc::c_char,
                ) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ServerInfoGetField
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleServerInfoData,
                        *const libc::c_char,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ServerInfoGetFieldC\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleServerInfoData,
                    *const libc::c_char,
                ) -> *const libc::c_char,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ServerInfoGetFieldC
                    as unsafe extern "C" fn(
                        *mut RedisModuleServerInfoData,
                        *const libc::c_char,
                    ) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ServerInfoGetFieldSigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleServerInfoData,
                    *const libc::c_char,
                    *mut libc::c_int,
                ) -> libc::c_longlong,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ServerInfoGetFieldSigned
                    as unsafe extern "C" fn(
                        *mut RedisModuleServerInfoData,
                        *const libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_longlong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ServerInfoGetFieldUnsigned\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleServerInfoData,
                    *const libc::c_char,
                    *mut libc::c_int,
                ) -> libc::c_ulonglong,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ServerInfoGetFieldUnsigned
                    as unsafe extern "C" fn(
                        *mut RedisModuleServerInfoData,
                        *const libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_ulonglong,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ServerInfoGetFieldDouble\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleServerInfoData,
                    *const libc::c_char,
                    *mut libc::c_int,
                ) -> libc::c_double,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ServerInfoGetFieldDouble
                    as unsafe extern "C" fn(
                        *mut RedisModuleServerInfoData,
                        *const libc::c_char,
                        *mut libc::c_int,
                    ) -> libc::c_double,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClientInfoById\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void, uint64_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_GetClientInfoById
                    as unsafe extern "C" fn(*mut libc::c_void, uint64_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClientNameById\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetClientNameById
                    as unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetClientNameById\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(uint64_t, *mut robj) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_SetClientNameById
                    as unsafe extern "C" fn(uint64_t, *mut robj) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_PublishMessage\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut robj,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_PublishMessage
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_PublishMessageShard\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut robj,
                    *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_PublishMessageShard
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut robj,
                        *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SubscribeToServerEvent\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleEvent,
                    RedisModuleEventCallback,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SubscribeToServerEvent
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleEvent,
                        RedisModuleEventCallback,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetLRU\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_SetLRU
                    as unsafe extern "C" fn(*mut RedisModuleKey, mstime_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetLRU\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleKey, *mut mstime_t) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetLRU
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut mstime_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetLFU\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    libc::c_longlong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetLFU
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        libc::c_longlong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetLFU\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut libc::c_longlong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetLFU
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut libc::c_longlong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_BlockClientOnKeys\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleCmdFunc,
                    RedisModuleCmdFunc,
                    Option::<
                        unsafe extern "C" fn(
                            *mut RedisModuleCtx,
                            *mut libc::c_void,
                        ) -> (),
                    >,
                    libc::c_longlong,
                    *mut *mut robj,
                    libc::c_int,
                    *mut libc::c_void,
                ) -> *mut RedisModuleBlockedClient,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_BlockClientOnKeys
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleCmdFunc,
                        RedisModuleCmdFunc,
                        Option::<
                            unsafe extern "C" fn(
                                *mut RedisModuleCtx,
                                *mut libc::c_void,
                            ) -> (),
                        >,
                        libc::c_longlong,
                        *mut *mut robj,
                        libc::c_int,
                        *mut libc::c_void,
                    ) -> *mut RedisModuleBlockedClient,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SignalKeyAsReady\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_SignalKeyAsReady
                    as unsafe extern "C" fn(*mut RedisModuleCtx, *mut robj) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetBlockedClientReadyKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetBlockedClientReadyKey
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetUsedMemoryRatio\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_float>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_float,
                    unsafe extern "C" fn() -> libc::c_float,
                >(RM_GetUsedMemoryRatio),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_MallocSize\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>,
            libc::c_ulong,
        >(Some(RM_MallocSize as unsafe extern "C" fn(*mut libc::c_void) -> size_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_MallocUsableSize\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> size_t>,
            libc::c_ulong,
        >(Some(RM_MallocUsableSize as unsafe extern "C" fn(*mut libc::c_void) -> size_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_MallocSizeString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut robj) -> size_t>,
            libc::c_ulong,
        >(Some(RM_MallocSizeString as unsafe extern "C" fn(*mut robj) -> size_t))
            as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_MallocSizeDict\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDict) -> size_t>,
            libc::c_ulong,
        >(
            Some(
                RM_MallocSizeDict as unsafe extern "C" fn(*mut RedisModuleDict) -> size_t,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ScanCursorCreate\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> *mut RedisModuleScanCursor>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> *mut RedisModuleScanCursor,
                    unsafe extern "C" fn() -> *mut RedisModuleScanCursor,
                >(RM_ScanCursorCreate),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ScanCursorDestroy\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleScanCursor) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ScanCursorDestroy
                    as unsafe extern "C" fn(*mut RedisModuleScanCursor) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ScanCursorRestart\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleScanCursor) -> ()>,
            libc::c_ulong,
        >(
            Some(
                RM_ScanCursorRestart
                    as unsafe extern "C" fn(*mut RedisModuleScanCursor) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Scan\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleScanCursor,
                    RedisModuleScanCB,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Scan
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleScanCursor,
                        RedisModuleScanCB,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ScanKey\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleKey,
                    *mut RedisModuleScanCursor,
                    RedisModuleScanKeyCB,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ScanKey
                    as unsafe extern "C" fn(
                        *mut RedisModuleKey,
                        *mut RedisModuleScanCursor,
                        RedisModuleScanKeyCB,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_CreateModuleUser\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*const libc::c_char) -> *mut RedisModuleUser>,
            libc::c_ulong,
        >(
            Some(
                RM_CreateModuleUser
                    as unsafe extern "C" fn(*const libc::c_char) -> *mut RedisModuleUser,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetContextUser\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, *const RedisModuleUser) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetContextUser
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const RedisModuleUser,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetModuleUserACL\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleUser,
                    *const libc::c_char,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetModuleUserACL
                    as unsafe extern "C" fn(
                        *mut RedisModuleUser,
                        *const libc::c_char,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_SetModuleUserACLString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleUser,
                    *const libc::c_char,
                    *mut *mut robj,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_SetModuleUserACLString
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleUser,
                        *const libc::c_char,
                        *mut *mut robj,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetModuleUserACLString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleUser) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetModuleUserACLString
                    as unsafe extern "C" fn(*mut RedisModuleUser) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetCurrentUserName\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetCurrentUserName
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetModuleUserFromUserName\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut robj) -> *mut RedisModuleUser>,
            libc::c_ulong,
        >(
            Some(
                RM_GetModuleUserFromUserName
                    as unsafe extern "C" fn(*mut robj) -> *mut RedisModuleUser,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ACLCheckCommandPermissions\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleUser,
                    *mut *mut robj,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ACLCheckCommandPermissions
                    as unsafe extern "C" fn(
                        *mut RedisModuleUser,
                        *mut *mut robj,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ACLCheckKeyPermissions\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleUser,
                    *mut robj,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ACLCheckKeyPermissions
                    as unsafe extern "C" fn(
                        *mut RedisModuleUser,
                        *mut robj,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ACLCheckChannelPermissions\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleUser,
                    *mut robj,
                    libc::c_int,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ACLCheckChannelPermissions
                    as unsafe extern "C" fn(
                        *mut RedisModuleUser,
                        *mut robj,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_ACLAddLogEntry\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleUser,
                    *mut robj,
                    RedisModuleACLLogEntryReason,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_ACLAddLogEntry
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleUser,
                        *mut robj,
                        RedisModuleACLLogEntryReason,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_FreeModuleUser\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleUser) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_FreeModuleUser
                    as unsafe extern "C" fn(*mut RedisModuleUser) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DeauthenticateAndCloseClient\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_DeauthenticateAndCloseClient
                    as unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_AuthenticateClientWithACLUser\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    size_t,
                    RedisModuleUserChangedFunc,
                    *mut libc::c_void,
                    *mut uint64_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_AuthenticateClientWithACLUser
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        size_t,
                        RedisModuleUserChangedFunc,
                        *mut libc::c_void,
                        *mut uint64_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_AuthenticateClientWithUser\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut RedisModuleUser,
                    RedisModuleUserChangedFunc,
                    *mut libc::c_void,
                    *mut uint64_t,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_AuthenticateClientWithUser
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut RedisModuleUser,
                        RedisModuleUserChangedFunc,
                        *mut libc::c_void,
                        *mut uint64_t,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetContextFlagsAll\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_GetContextFlagsAll),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetKeyspaceNotificationFlagsAll\0" as *const u8
            as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_GetKeyspaceNotificationFlagsAll),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_IsSubEventSupported\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(RedisModuleEvent, int64_t) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_IsSubEventSupported
                    as unsafe extern "C" fn(RedisModuleEvent, int64_t) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetServerVersion\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_GetServerVersion),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetClientCertificate\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj>,
            libc::c_ulong,
        >(
            Some(
                RM_GetClientCertificate
                    as unsafe extern "C" fn(*mut RedisModuleCtx, uint64_t) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RedactClientCommandArgument\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleCtx, libc::c_int) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RedactClientCommandArgument
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetCommandKeys\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut robj,
                    libc::c_int,
                    *mut libc::c_int,
                ) -> *mut libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetCommandKeys
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut *mut robj,
                        libc::c_int,
                        *mut libc::c_int,
                    ) -> *mut libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetCommandKeysWithFlags\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *mut *mut robj,
                    libc::c_int,
                    *mut libc::c_int,
                    *mut *mut libc::c_int,
                ) -> *mut libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_GetCommandKeysWithFlags
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *mut *mut robj,
                        libc::c_int,
                        *mut libc::c_int,
                        *mut *mut libc::c_int,
                    ) -> *mut libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetCurrentCommandName\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> *const libc::c_char>,
            libc::c_ulong,
        >(
            Some(
                RM_GetCurrentCommandName
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> *const libc::c_char,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_GetTypeMethodVersion\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                core::mem::transmute::<
                    unsafe extern "C" fn() -> libc::c_int,
                    unsafe extern "C" fn() -> libc::c_int,
                >(RM_GetTypeMethodVersion),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterDefragFunc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    RedisModuleDefragFunc,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterDefragFunc
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        RedisModuleDefragFunc,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DefragAlloc\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDefragCtx,
                    *mut libc::c_void,
                ) -> *mut libc::c_void,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DefragAlloc
                    as unsafe extern "C" fn(
                        *mut RedisModuleDefragCtx,
                        *mut libc::c_void,
                    ) -> *mut libc::c_void,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DefragRedisModuleString\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(*mut RedisModuleDefragCtx, *mut robj) -> *mut robj,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DefragRedisModuleString
                    as unsafe extern "C" fn(
                        *mut RedisModuleDefragCtx,
                        *mut robj,
                    ) -> *mut robj,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DefragShouldStop\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_DefragShouldStop
                    as unsafe extern "C" fn(*mut RedisModuleDefragCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DefragCursorSet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDefragCtx,
                    libc::c_ulong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DefragCursorSet
                    as unsafe extern "C" fn(
                        *mut RedisModuleDefragCtx,
                        libc::c_ulong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_DefragCursorGet\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleDefragCtx,
                    *mut libc::c_ulong,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_DefragCursorGet
                    as unsafe extern "C" fn(
                        *mut RedisModuleDefragCtx,
                        *mut libc::c_ulong,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_EventLoopAdd\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    libc::c_int,
                    libc::c_int,
                    RedisModuleEventLoopFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_EventLoopAdd
                    as unsafe extern "C" fn(
                        libc::c_int,
                        libc::c_int,
                        RedisModuleEventLoopFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_EventLoopDel\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_EventLoopDel
                    as unsafe extern "C" fn(libc::c_int, libc::c_int) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_EventLoopAddOneShot\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    RedisModuleEventLoopOneShotFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_EventLoopAddOneShot
                    as unsafe extern "C" fn(
                        RedisModuleEventLoopOneShotFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_Yield\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    libc::c_int,
                    *const libc::c_char,
                ) -> (),
            >,
            libc::c_ulong,
        >(
            Some(
                RM_Yield
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        libc::c_int,
                        *const libc::c_char,
                    ) -> (),
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterBoolConfig\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_uint,
                    RedisModuleConfigGetBoolFunc,
                    RedisModuleConfigSetBoolFunc,
                    RedisModuleConfigApplyFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterBoolConfig
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_uint,
                        RedisModuleConfigGetBoolFunc,
                        RedisModuleConfigSetBoolFunc,
                        RedisModuleConfigApplyFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterNumericConfig\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_longlong,
                    libc::c_uint,
                    libc::c_longlong,
                    libc::c_longlong,
                    RedisModuleConfigGetNumericFunc,
                    RedisModuleConfigSetNumericFunc,
                    RedisModuleConfigApplyFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterNumericConfig
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        libc::c_longlong,
                        libc::c_uint,
                        libc::c_longlong,
                        libc::c_longlong,
                        RedisModuleConfigGetNumericFunc,
                        RedisModuleConfigSetNumericFunc,
                        RedisModuleConfigApplyFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterStringConfig\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    *const libc::c_char,
                    libc::c_uint,
                    RedisModuleConfigGetStringFunc,
                    RedisModuleConfigSetStringFunc,
                    RedisModuleConfigApplyFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterStringConfig
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        *const libc::c_char,
                        libc::c_uint,
                        RedisModuleConfigGetStringFunc,
                        RedisModuleConfigSetStringFunc,
                        RedisModuleConfigApplyFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_RegisterEnumConfig\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    *mut RedisModuleCtx,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_uint,
                    *mut *const libc::c_char,
                    *const libc::c_int,
                    libc::c_int,
                    RedisModuleConfigGetEnumFunc,
                    RedisModuleConfigSetEnumFunc,
                    RedisModuleConfigApplyFunc,
                    *mut libc::c_void,
                ) -> libc::c_int,
            >,
            libc::c_ulong,
        >(
            Some(
                RM_RegisterEnumConfig
                    as unsafe extern "C" fn(
                        *mut RedisModuleCtx,
                        *const libc::c_char,
                        libc::c_int,
                        libc::c_uint,
                        *mut *const libc::c_char,
                        *const libc::c_int,
                        libc::c_int,
                        RedisModuleConfigGetEnumFunc,
                        RedisModuleConfigSetEnumFunc,
                        RedisModuleConfigApplyFunc,
                        *mut libc::c_void,
                    ) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
    moduleRegisterApi(
        b"RedisModule_LoadConfigs\0" as *const u8 as *const libc::c_char,
        core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int>,
            libc::c_ulong,
        >(
            Some(
                RM_LoadConfigs
                    as unsafe extern "C" fn(*mut RedisModuleCtx) -> libc::c_int,
            ),
        ) as *mut libc::c_void,
    );
}

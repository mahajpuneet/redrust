use server::core::cell::UnsafeCell;

extern crate c2rust_bitfields;
extern crate libc;
extern crate core;

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type RedisModuleCommand;
    static mut configOOMScoreAdjValuesDefaults: [libc::c_int; 3];
    static mut clientBufferLimitsDefaults: [clientBufferLimitsConfig; 3];
    static mut io_threads_op: libc::c_int;
    fn moduleInitModulesSystem();
    fn moduleInitModulesSystemLast();
    fn modulesCron();
    fn moduleLoadFromQueue();
    fn moduleNameFromCommand(cmd: *mut redisCommand) -> *const libc::c_char;
    fn moduleHandleBlockedClients();
    fn modulePipeReadable(
        el: *mut aeEventLoop,
        fd: libc::c_int,
        privdata: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn moduleCount() -> size_t;
    fn moduleAcquireGIL();
    fn moduleReleaseGIL();
    fn moduleCallCommandFilters(c: *mut client);
    fn ModuleForkDoneHandler(exitcode: libc::c_int, bysignal: libc::c_int);
    fn TerminateModuleForkChild(
        child_pid: libc::c_int,
        wait: libc::c_int,
    ) -> libc::c_int;
    fn modulesCollectInfo(
        info: sds,
        sections_dict: *mut dict,
        for_crash_report: libc::c_int,
        sections: libc::c_int,
    ) -> sds;
    fn moduleFireServerEvent(eid: uint64_t, subid: libc::c_int, data: *mut libc::c_void);
    fn moduleGetHandleByName(modulename: *mut libc::c_char) -> *mut libc::c_void;
    fn moduleIsModuleCommand(
        module_handle: *mut libc::c_void,
        cmd: *mut redisCommand,
    ) -> libc::c_int;
    fn getRandomHexChars(p: *mut libc::c_char, len: size_t);
    fn getRandomBytes(p: *mut libc::c_uchar, len: size_t);
    fn freeClientOriginalArgv(c: *mut client);
    fn addReplyDeferredLen(c: *mut client) -> *mut libc::c_void;
    fn setDeferredArrayLen(
        c: *mut client,
        node: *mut libc::c_void,
        length: libc::c_long,
    );
    fn setDeferredMapLen(c: *mut client, node: *mut libc::c_void, length: libc::c_long);
    fn acceptTcpHandler(
        el: *mut aeEventLoop,
        fd: libc::c_int,
        privdata: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn acceptTLSHandler(
        el: *mut aeEventLoop,
        fd: libc::c_int,
        privdata: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn acceptUnixHandler(
        el: *mut aeEventLoop,
        fd: libc::c_int,
        privdata: *mut libc::c_void,
        mask: libc::c_int,
    );
    fn addReplyNull(c: *mut client);
    fn addReplyVerbatim(
        c: *mut client,
        s: *const libc::c_char,
        len: size_t,
        ext: *const libc::c_char,
    );
    fn addReplyBulk(c: *mut client, obj: *mut robj);
    fn addReplyBulkCString(c: *mut client, s: *const libc::c_char);
    fn addReplyBulkCBuffer(c: *mut client, p: *const libc::c_void, len: size_t);
    fn addReplyBulkLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReply(c: *mut client, obj: *mut robj);
    fn addReplyErrorObject(c: *mut client, err: *mut robj);
    fn addReplyErrorSds(c: *mut client, err: sds);
    fn addReplyError(c: *mut client, err: *const libc::c_char);
    fn addReplyErrorArity(c: *mut client);
    fn addReplyStatus(c: *mut client, status: *const libc::c_char);
    fn addReplyLongLong(c: *mut client, ll: libc::c_longlong);
    fn addReplyArrayLen(c: *mut client, length: libc::c_long);
    fn addReplyMapLen(c: *mut client, length: libc::c_long);
    fn addReplySetLen(c: *mut client, length: libc::c_long);
    fn addReplyHelp(c: *mut client, help: *mut *const libc::c_char);
    fn sdsZmallocSize(s: sds) -> size_t;
    fn getClientOutputBufferMemoryUsage(c: *mut client) -> size_t;
    fn getClientMemoryUsage(
        c: *mut client,
        output_buffer_mem_usage: *mut size_t,
    ) -> size_t;
    fn freeClientsInAsyncFreeQueue() -> libc::c_int;
    fn closeClientOnOutputBufferLimitReached(
        c: *mut client,
        async_0: libc::c_int,
    ) -> libc::c_int;
    fn getClientType(c: *mut client) -> libc::c_int;
    fn flushSlavesOutputBuffers();
    fn evictClients();
    fn pauseClients(purpose: pause_purpose, end: mstime_t, type_0: pause_type);
    fn unpauseClients(purpose: pause_purpose);
    fn areClientsPaused() -> libc::c_int;
    fn checkClientPauseTimeoutAndReturnIfPaused() -> libc::c_int;
    fn handleClientsWithPendingWrites() -> libc::c_int;
    fn handleClientsWithPendingWritesUsingThreads() -> libc::c_int;
    fn handleClientsWithPendingReadsUsingThreads() -> libc::c_int;
    fn stopThreadedIOIfNeeded() -> libc::c_int;
    fn initThreadedIO();
    fn authRequired(c: *mut client) -> libc::c_int;
    fn trackingRememberKeys(c: *mut client);
    fn trackingHandlePendingKeyInvalidations();
    fn trackingLimitUsedSlots();
    fn trackingGetTotalItems() -> uint64_t;
    fn trackingGetTotalKeys() -> uint64_t;
    fn trackingGetTotalPrefixes() -> uint64_t;
    fn trackingBroadcastInvalidationMessages();
    fn queueMultiCommand(c: *mut client, cmd_flags: uint64_t);
    fn discardTransaction(c: *mut client);
    fn flagTransaction(c: *mut client);
    fn execCommandAbort(c: *mut client, error: sds);
    fn decrRefCount(o: *mut robj);
    fn incrRefCount(o: *mut robj);
    fn makeObjectShared(o: *mut robj) -> *mut robj;
    fn dismissObject(o: *mut robj, dump_size: size_t);
    fn createObject(type_0: libc::c_int, ptr: *mut libc::c_void) -> *mut robj;
    fn createStringObject(ptr: *const libc::c_char, len: size_t) -> *mut robj;
    fn getDecodedObject(o: *mut robj) -> *mut robj;
    fn replicationFeedSlaves(
        slaves: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn resetReplicationBuffer();
    fn replicationFeedMonitors(
        c: *mut client,
        monitors: *mut list,
        dictid: libc::c_int,
        argv: *mut *mut robj,
        argc: libc::c_int,
    );
    fn replicationCron();
    fn replicationStartPendingFork();
    fn checkGoodReplicasStatus() -> libc::c_int;
    fn processClientsWaitingReplicas();
    fn replicationGetSlaveName(c: *mut client) -> *mut libc::c_char;
    fn changeReplicationId();
    fn clearReplicationId2();
    fn createReplicationBacklog();
    fn freeReplicationBacklog();
    fn replicationCacheMasterUsingMyself();
    fn incrementalTrimReplicationBacklog(blocks: size_t);
    fn rebaseReplicationBuffer(base_repl_offset: libc::c_longlong);
    fn updateFailoverStatus();
    fn getFailoverStateString() -> *const libc::c_char;
    fn killRDBChild();
    fn flushAppendOnlyFile(force: libc::c_int);
    fn feedAppendOnlyFile(dictid: libc::c_int, argv: *mut *mut robj, argc: libc::c_int);
    fn rewriteAppendOnlyFileBackground() -> libc::c_int;
    fn loadAppendOnlyFiles(am: *mut aofManifest) -> libc::c_int;
    fn backgroundRewriteDoneHandler(exitcode: libc::c_int, bysignal: libc::c_int);
    fn killAppendOnlyChild();
    fn aofLoadManifestFromDisk();
    fn aofOpenIfNeededOnServerStart();
    fn aofManifestFree(am: *mut aofManifest);
    fn aofDelHistoryFiles() -> libc::c_int;
    fn aofRewriteLimited() -> libc::c_int;
    fn openChildInfoPipe();
    fn closeChildInfoPipe();
    fn sendChildInfoGeneric(
        info_type: childInfoType,
        keys: size_t,
        progress: libc::c_double,
        pname: *mut libc::c_char,
    );
    fn receiveChildInfo();
    fn ACLInit();
    fn ACLGetCommandID(cmdname: sds) -> libc::c_ulong;
    fn ACLCheckAllPerm(c: *mut client, idxptr: *mut libc::c_int) -> libc::c_int;
    fn ACLGetCommandCategoryFlagByName(name: *const libc::c_char) -> uint64_t;
    fn ACLLoadUsersAtStartup();
    fn addReplyCommandCategories(c: *mut client, cmd: *mut redisCommand);
    fn addACLLogEntry(
        c: *mut client,
        reason: libc::c_int,
        context: libc::c_int,
        argpos: libc::c_int,
        username: sds,
        object: sds,
    );
    fn ACLUpdateDefaultUserPassword(password: sds);
    fn freeMemoryGetNotCountedMemory() -> size_t;
    fn overMaxmemoryAfterAlloc(moremem: size_t) -> libc::c_int;
    fn replyToClientsBlockedOnShutdown();
    fn activeDefragCycle();
    fn getLRUClock() -> libc::c_uint;
    fn evictPolicyToString() -> *const libc::c_char;
    fn getMemoryOverheadData() -> *mut redisMemOverhead;
    fn freeMemoryOverheadData(mh: *mut redisMemOverhead);
    fn dismissSds(s: sds);
    fn listRelease(list: *mut list);
    fn srandom(__seed: libc::c_uint);
    fn srand(__seed: libc::c_uint);
    fn exit(_: libc::c_int) -> !;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn unsetenv(__name: *const libc::c_char) -> libc::c_int;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn strcasecmp(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
    fn dup2(__fd: libc::c_int, __fd2: libc::c_int) -> libc::c_int;
    static mut environ: *mut *mut libc::c_char;
    fn execve(
        __path: *const libc::c_char,
        __argv: *const *mut libc::c_char,
        __envp: *const *mut libc::c_char,
    ) -> libc::c_int;
    fn _exit(__status: libc::c_int) -> !;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: libc::c_int) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn fdatasync(__fildes: libc::c_int) -> libc::c_int;
    fn openlog(
        __ident: *const libc::c_char,
        __option: libc::c_int,
        __facility: libc::c_int,
    );
    fn syslog(__pri: libc::c_int, __fmt: *const libc::c_char, _: ...);
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: libc::c_int) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn hdr_init(
        lowest_discernible_value: int64_t,
        highest_trackable_value: int64_t,
        significant_figures: libc::c_int,
        result: *mut *mut hdr_histogram,
    ) -> libc::c_int;
    fn hdr_close(h: *mut hdr_histogram);
    fn hdr_record_value(h: *mut hdr_histogram, value: int64_t) -> bool;
    fn hdr_value_at_percentile(
        h: *const hdr_histogram,
        percentile: libc::c_double,
    ) -> int64_t;
    static mut getMonotonicUs: Option::<unsafe extern "C" fn() -> monotime>;
    fn monotonicInit() -> *const libc::c_char;
    fn monotonicInfoString() -> *const libc::c_char;
    fn monotonicGetType() -> monotonic_clock_type;
    fn aeCreateEventLoop(setsize: libc::c_int) -> *mut aeEventLoop;
    fn aeDeleteEventLoop(eventLoop: *mut aeEventLoop);
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
    fn aeCreateTimeEvent(
        eventLoop: *mut aeEventLoop,
        milliseconds: libc::c_longlong,
        proc_0: Option::<aeTimeProc>,
        clientData: *mut libc::c_void,
        finalizerProc: Option::<aeEventFinalizerProc>,
    ) -> libc::c_longlong;
    fn aeMain(eventLoop: *mut aeEventLoop);
    fn aeGetApiName() -> *mut libc::c_char;
    fn aeSetBeforeSleepProc(
        eventLoop: *mut aeEventLoop,
        beforesleep: Option::<aeBeforeSleepProc>,
    );
    fn aeSetAfterSleepProc(
        eventLoop: *mut aeEventLoop,
        aftersleep: Option::<aeBeforeSleepProc>,
    );
    fn aeSetDontWait(eventLoop: *mut aeEventLoop, noWait: libc::c_int);
    fn init_genrand64(seed: libc::c_ulonglong);
    fn dictCreate(type_0: *mut dictType) -> *mut dict;
    fn dictExpand(d: *mut dict, size: libc::c_ulong) -> libc::c_int;
    fn dictAdd(
        d: *mut dict,
        key: *mut libc::c_void,
        val: *mut libc::c_void,
    ) -> libc::c_int;
    fn dictRelease(d: *mut dict);
    fn dictFind(d: *mut dict, key: *const libc::c_void) -> *mut dictEntry;
    fn dictFetchValue(d: *mut dict, key: *const libc::c_void) -> *mut libc::c_void;
    fn dictResize(d: *mut dict) -> libc::c_int;
    fn dictGetIterator(d: *mut dict) -> *mut dictIterator;
    fn dictGetSafeIterator(d: *mut dict) -> *mut dictIterator;
    fn dictNext(iter: *mut dictIterator) -> *mut dictEntry;
    fn dictReleaseIterator(iter: *mut dictIterator);
    fn dictGenHashFunction(key: *const libc::c_void, len: size_t) -> uint64_t;
    fn dictGenCaseHashFunction(buf: *const libc::c_uchar, len: size_t) -> uint64_t;
    fn dictSetResizeEnabled(enable: dictResizeEnable);
    fn dictRehashMilliseconds(d: *mut dict, ms: libc::c_int) -> libc::c_int;
    fn dictSetHashFunctionSeed(seed: *mut uint8_t);
    fn listCreate() -> *mut list;
    fn initSentinel();
    fn je_malloc_usable_size(ptr: *mut libc::c_void) -> size_t;
    fn zmalloc(size: size_t) -> *mut libc::c_void;
    fn zrealloc(ptr: *mut libc::c_void, size: size_t) -> *mut libc::c_void;
    fn zfree(ptr: *mut libc::c_void);
    fn zmalloc_usable(size: size_t, usable: *mut size_t) -> *mut libc::c_void;
    fn zstrdup(s: *const libc::c_char) -> *mut libc::c_char;
    fn zmalloc_used_memory() -> size_t;
    fn zmalloc_set_oom_handler(
        oom_handler: Option::<unsafe extern "C" fn(size_t) -> ()>,
    );
    fn zmalloc_get_rss() -> size_t;
    fn zmalloc_get_allocator_info(
        allocated: *mut size_t,
        active: *mut size_t,
        resident: *mut size_t,
    ) -> libc::c_int;
    fn set_jemalloc_bg_thread(enable: libc::c_int);
    fn zmalloc_get_memory_size() -> size_t;
    fn zmadvise_dontneed(ptr: *mut libc::c_void);
    fn anetTcpServer(
        err: *mut libc::c_char,
        port: libc::c_int,
        bindaddr: *mut libc::c_char,
        backlog: libc::c_int,
    ) -> libc::c_int;
    fn anetTcp6Server(
        err: *mut libc::c_char,
        port: libc::c_int,
        bindaddr: *mut libc::c_char,
        backlog: libc::c_int,
    ) -> libc::c_int;
    fn anetUnixServer(
        err: *mut libc::c_char,
        path: *mut libc::c_char,
        perm: mode_t,
        backlog: libc::c_int,
    ) -> libc::c_int;
    fn anetNonBlock(err: *mut libc::c_char, fd: libc::c_int) -> libc::c_int;
    fn anetCloexec(fd: libc::c_int) -> libc::c_int;
    fn anetSetSockMarkId(
        err: *mut libc::c_char,
        fd: libc::c_int,
        id: uint32_t,
    ) -> libc::c_int;
    fn stringmatchlen(
        p: *const libc::c_char,
        plen: libc::c_int,
        s: *const libc::c_char,
        slen: libc::c_int,
        nocase: libc::c_int,
    ) -> libc::c_int;
    fn mempbrk(
        s: *const libc::c_char,
        len: size_t,
        chars: *const libc::c_char,
        charslen: size_t,
    ) -> *const libc::c_char;
    fn memmapchars(
        s: *mut libc::c_char,
        len: size_t,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> *mut libc::c_char;
    fn ll2string(
        s: *mut libc::c_char,
        len: size_t,
        value: libc::c_longlong,
    ) -> libc::c_int;
    fn trimDoubleString(buf: *mut libc::c_char, len: size_t) -> libc::c_int;
    fn getAbsolutePath(filename: *mut libc::c_char) -> sds;
    fn getTimeZone() -> libc::c_long;
    fn latencyMonitorInit();
    fn latencyAddSample(event: *const libc::c_char, latency: mstime_t);
    static mut raxNotFound: *mut libc::c_void;
    fn raxNew() -> *mut rax;
    fn raxInsert(
        rax: *mut rax,
        s: *mut libc::c_uchar,
        len: size_t,
        data: *mut libc::c_void,
        old: *mut *mut libc::c_void,
    ) -> libc::c_int;
    fn raxFind(rax: *mut rax, s: *mut libc::c_uchar, len: size_t) -> *mut libc::c_void;
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
    fn raxStop(it: *mut raxIterator);
    fn raxSize(rax: *mut rax) -> uint64_t;
    fn crc64_init();
    fn pthread_self() -> pthread_t;
    fn tzset();
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn strftime(
        __s: *mut libc::c_char,
        __maxsize: size_t,
        __format: *const libc::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn tlsProcessPendingData() -> libc::c_int;
    fn tlsHasPendingData() -> libc::c_int;
    fn connPeerToString(
        conn: *mut connection,
        ip: *mut libc::c_char,
        ip_len: size_t,
        port: *mut libc::c_int,
    ) -> libc::c_int;
    fn _serverPanic(
        file: *const libc::c_char,
        line: libc::c_int,
        msg: *const libc::c_char,
        _: ...
    );
    fn pthread_setcancelstate(
        __state: libc::c_int,
        __oldstate: *mut libc::c_int,
    ) -> libc::c_int;
    fn pthread_setcanceltype(
        __type: libc::c_int,
        __oldtype: *mut libc::c_int,
    ) -> libc::c_int;
    fn _serverAssert(
        estr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn __errno_location() -> *mut libc::c_int;
    fn sdsResize(s: sds, size: size_t) -> sds;
    fn sdsRemoveFreeSpace(s: sds) -> sds;
    fn sdstemplate(
        template: *const libc::c_char,
        cb_func: sdstemplate_callback_t,
        cb_arg: *mut libc::c_void,
    ) -> sds;
    fn sdsmapchars(
        s: sds,
        from: *const libc::c_char,
        to: *const libc::c_char,
        setlen: size_t,
    ) -> sds;
    fn sdssplitargs(line: *const libc::c_char, argc: *mut libc::c_int) -> *mut sds;
    fn sdscatrepr(s: sds, p: *const libc::c_char, len: size_t) -> sds;
    fn sdstoupper(s: sds);
    fn sdsfreesplitres(tokens: *mut sds, count: libc::c_int);
    fn sdssplitlen(
        s: *const libc::c_char,
        len: ssize_t,
        sep: *const libc::c_char,
        seplen: libc::c_int,
        count: *mut libc::c_int,
    ) -> *mut sds;
    fn sdssubstr(s: sds, start: size_t, len: size_t);
    fn sdstrim(s: sds, cset: *const libc::c_char) -> sds;
    fn sdscatfmt(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatprintf(s: sds, fmt: *const libc::c_char, _: ...) -> sds;
    fn sdscatvprintf(s: sds, fmt: *const libc::c_char, ap: core::ffi::VaList) -> sds;
    fn sdscat(s: sds, t: *const libc::c_char) -> sds;
    fn sdscatlen(s: sds, t: *const libc::c_void, len: size_t) -> sds;
    fn sdsfree(s: sds);
    fn sdsdup(s: sds) -> sds;
    fn sdsempty() -> sds;
    fn sdsnew(init: *const libc::c_char) -> sds;
    fn rdbSave(
        req: libc::c_int,
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn rdbPopulateSaveInfo(rsi: *mut rdbSaveInfo) -> *mut rdbSaveInfo;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn rdbRemoveTempFile(childpid: pid_t, from_signal: libc::c_int);
    fn ldbKillForkedSessions();
    fn evalMemory() -> libc::c_ulong;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: core::ffi::VaList,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;
    fn setcpuaffinity(cpulist: *const libc::c_char);
    fn rdbLoad(
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
        rdbflags: libc::c_int,
    ) -> libc::c_int;
    fn rdbSaveBackground(
        req: libc::c_int,
        filename: *mut libc::c_char,
        rsi: *mut rdbSaveInfo,
    ) -> libc::c_int;
    fn backgroundSaveDoneHandler(exitcode: libc::c_int, bysignal: libc::c_int);
    fn dbTotalServerKeyCount() -> libc::c_longlong;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn evalGetCommandFlags(c: *mut client, orig_flags: uint64_t) -> uint64_t;
    fn evalShaRoCommand(c: *mut client);
    fn evalShaCommand(c: *mut client);
    fn evalRoCommand(c: *mut client);
    fn evalCommand(c: *mut client);
    fn fcallGetCommandFlags(c: *mut client, orig_flags: uint64_t) -> uint64_t;
    fn fcallroCommand(c: *mut client);
    fn fcallCommand(c: *mut client);
    fn handleClientsBlockedOnKeys();
    fn execCommand(c: *mut client);
    fn resetCommand(c: *mut client);
    fn quitCommand(c: *mut client);
    fn watchCommand(c: *mut client);
    fn multiCommand(c: *mut client);
    fn discardCommand(c: *mut client);
    fn blockClient(c: *mut client, btype: libc::c_int);
    fn punsubscribeCommand(c: *mut client);
    fn psubscribeCommand(c: *mut client);
    fn sunsubscribeCommand(c: *mut client);
    fn unsubscribeCommand(c: *mut client);
    fn ssubscribeCommand(c: *mut client);
    fn subscribeCommand(c: *mut client);
    fn performEvictions() -> libc::c_int;
    fn debugCommand(c: *mut client);
    fn allowProtectedAction(config: libc::c_int, c: *mut client) -> libc::c_int;
    fn moduleCommand(c: *mut client);
    fn securityWarningCommand(c: *mut client);
    fn sigsegvHandler(sig: libc::c_int, info: *mut siginfo_t, secret: *mut libc::c_void);
    fn tlsConfigure(ctx_config: *mut redisTLSContextConfig) -> libc::c_int;
    fn tlsInit();
    fn setproctitle(fmt: *const libc::c_char, _: ...);
    fn spt_init(argc: libc::c_int, argv: *mut *mut libc::c_char);
    fn watchdogScheduleSignal(period: libc::c_int);
    fn applyWatchdogPeriod();
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lazyfreeResetStats();
    fn ldbRemoveChild(pid: pid_t) -> libc::c_int;
    fn initSentinelConfig();
    fn rewriteConfig(path: *mut libc::c_char, force_write: libc::c_int) -> libc::c_int;
    fn loadServerConfig(
        filename: *mut libc::c_char,
        config_from_stdin: libc::c_char,
        options: *mut libc::c_char,
    );
    fn appendServerSaveParams(seconds: time_t, changes: libc::c_int);
    fn resetServerSaveParams();
    fn initConfigValues();
    fn selectDb(c: *mut client, id: libc::c_int) -> libc::c_int;
    fn lazyfreeGetPendingObjectsCount() -> size_t;
    fn lazyfreeGetFreedObjectsCount() -> size_t;
    fn getKeysFromCommandWithSpecs(
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        search_flags: libc::c_int,
        result: *mut getKeysResult,
    ) -> libc::c_int;
    fn doesCommandHaveKeys(cmd: *mut redisCommand) -> libc::c_int;
    fn getKeysFreeResult(result: *mut getKeysResult);
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    fn sentinelTimer();
    fn loadSentinelConfigFromQueue();
    fn sentinelIsRunning();
    fn sentinelCheckConfigFile();
    fn sentinelInfoCommand(c: *mut client);
    fn redis_check_rdb_main(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        fp: *mut FILE,
    ) -> libc::c_int;
    fn redis_check_aof_main(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn scriptingInit(setup: libc::c_int);
    fn ldbPendingChildren() -> libc::c_int;
    fn evalScriptsDict() -> *mut dict;
    fn processUnblockedClients();
    fn handleBlockedClientsTimeout();
    fn clientsCronHandleTimeout(c: *mut client, now_ms: mstime_t) -> libc::c_int;
    fn activeExpireCycle(type_0: libc::c_int);
    fn expireSlaveKeys();
    fn getSlaveKeyWithExpireCount() -> size_t;
    fn evictionPoolAlloc();
    fn redisGitSHA1() -> *mut libc::c_char;
    fn redisGitDirty() -> *mut libc::c_char;
    fn redisBuildId() -> uint64_t;
    fn redisBuildIdString() -> *mut libc::c_char;
    fn genModulesInfoString(info: sds) -> sds;
    fn listAddNodeTail(list: *mut list, value: *mut libc::c_void) -> *mut list;
    fn listDelNode(list: *mut list, node: *mut listNode);
    fn listNext(iter: *mut listIter) -> *mut listNode;
    fn listRewind(list: *mut list, li: *mut listIter);
    fn listRotateTailToHead(list: *mut list);
    fn clusterInit();
    fn clusterCron();
    fn clusterBeforeSleep();
    fn getNodeByQuery(
        c: *mut client,
        cmd: *mut redisCommand,
        argv: *mut *mut robj,
        argc: libc::c_int,
        hashslot: *mut libc::c_int,
        ask: *mut libc::c_int,
    ) -> *mut clusterNode;
    fn clusterRedirectClient(
        c: *mut client,
        n: *mut clusterNode,
        hashslot: libc::c_int,
        error_code: libc::c_int,
    );
    fn migrateCloseTimedoutSockets();
    fn verifyClusterConfigWithData() -> libc::c_int;
    fn getClusterConnectionsCount() -> libc::c_ulong;
    fn slowlogInit();
    fn slowlogPushEntryIfNeeded(
        c: *mut client,
        argv: *mut *mut robj,
        argc: libc::c_int,
        duration: libc::c_longlong,
    );
    fn bioInit();
    fn bioPendingJobsOfType(type_0: libc::c_int) -> libc::c_ulonglong;
    fn scriptIsRunning() -> libc::c_int;
    fn scriptIsTimedout() -> libc::c_int;
    fn scriptIsEval() -> libc::c_int;
    fn functionsMemory() -> libc::c_ulong;
    fn functionsNum() -> libc::c_ulong;
    fn functionsLibNum() -> libc::c_ulong;
    fn functionsInit() -> libc::c_int;
    fn syscheck() -> libc::c_int;
    fn checkXenClocksource(error_msg: *mut sds) -> libc::c_int;
    fn checkTHPEnabled(error_msg: *mut sds) -> libc::c_int;
    fn checkOvercommit(error_msg: *mut sds) -> libc::c_int;
    fn checkLinuxMadvFreeForkBug(error_msg: *mut sds) -> libc::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn flock(__fd: libc::c_int, __operation: libc::c_int) -> libc::c_int;
    fn getrlimit(__resource: __rlimit_resource_t, __rlimits: *mut rlimit) -> libc::c_int;
    fn setrlimit(
        __resource: __rlimit_resource_t,
        __rlimits: *const rlimit,
    ) -> libc::c_int;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> libc::c_int;
    fn uname(__name: *mut utsname) -> libc::c_int;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn nolocks_localtime(tmp: *mut tm, t: time_t, tz: time_t, dst: libc::c_int);
    static mut ProcessingEventsWhileBlocked: libc::c_int;
    static mut redisCommandTable: [redisCommand; 0];
    fn prctl(__option: libc::c_int, _: ...) -> libc::c_int;
    fn memtest(megabytes: size_t, passes: libc::c_int);
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
pub type __int16_t = libc::c_short;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uint_least64_t = __uint64_t;
pub type __intmax_t = libc::c_long;
pub type __uid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type __rlim64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
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
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type int16_t = __int16_t;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type pthread_t = libc::c_ulong;
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
pub type intmax_t = __intmax_t;
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
pub type sdstemplate_callback_t = Option::<
    unsafe extern "C" fn(sds, *mut libc::c_void) -> sds,
>;
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
#[repr(C)]
pub struct redisObject {
    #[bitfield(name = "type_0", ty = "libc::c_uint", bits = "0..=3")]
    #[bitfield(name = "encoding", ty = "libc::c_uint", bits = "4..=7")]
    #[bitfield(name = "lru", ty = "libc::c_uint", bits = "8..=31")]
    pub type_0_encoding_lru: [u8; 4],
    pub refcount: libc::c_int,
    pub ptr: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type C2RustUnnamed = libc::c_uint;
pub const PTHREAD_CANCEL_DISABLE: C2RustUnnamed = 1;
pub const PTHREAD_CANCEL_ENABLE: C2RustUnnamed = 0;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const PTHREAD_CANCEL_ASYNCHRONOUS: C2RustUnnamed_0 = 1;
pub const PTHREAD_CANCEL_DEFERRED: C2RustUnnamed_0 = 0;
pub type memory_order = libc::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type atomic_int = libc::c_int;
pub type atomic_uint = libc::c_uint;
pub type atomic_llong = libc::c_longlong;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed_1 = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed_1 = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed_1 = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed_1 = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed_1 = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed_1 = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed_1 = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed_1 = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed_1 = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed_1 = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed_1 = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed_1 = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed_1 = 236;
pub const _SC_IPV6: C2RustUnnamed_1 = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed_1 = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed_1 = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed_1 = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed_1 = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed_1 = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed_1 = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed_1 = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed_1 = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed_1 = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed_1 = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed_1 = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed_1 = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed_1 = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed_1 = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed_1 = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed_1 = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed_1 = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed_1 = 182;
pub const _SC_TRACE: C2RustUnnamed_1 = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed_1 = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed_1 = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed_1 = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed_1 = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed_1 = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed_1 = 175;
pub const _SC_STREAMS: C2RustUnnamed_1 = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed_1 = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed_1 = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed_1 = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed_1 = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed_1 = 169;
pub const _SC_2_PBS: C2RustUnnamed_1 = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed_1 = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed_1 = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed_1 = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed_1 = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed_1 = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed_1 = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed_1 = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed_1 = 160;
pub const _SC_SPAWN: C2RustUnnamed_1 = 159;
pub const _SC_SIGNALS: C2RustUnnamed_1 = 158;
pub const _SC_SHELL: C2RustUnnamed_1 = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed_1 = 156;
pub const _SC_REGEXP: C2RustUnnamed_1 = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed_1 = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed_1 = 153;
pub const _SC_NETWORKING: C2RustUnnamed_1 = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed_1 = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed_1 = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed_1 = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed_1 = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed_1 = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed_1 = 146;
pub const _SC_PIPE: C2RustUnnamed_1 = 145;
pub const _SC_FIFO: C2RustUnnamed_1 = 144;
pub const _SC_FD_MGMT: C2RustUnnamed_1 = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed_1 = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed_1 = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed_1 = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed_1 = 139;
pub const _SC_CPUTIME: C2RustUnnamed_1 = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed_1 = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed_1 = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed_1 = 135;
pub const _SC_BASE: C2RustUnnamed_1 = 134;
pub const _SC_BARRIERS: C2RustUnnamed_1 = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed_1 = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed_1 = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed_1 = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed_1 = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed_1 = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed_1 = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed_1 = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed_1 = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed_1 = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed_1 = 123;
pub const _SC_NL_NMAX: C2RustUnnamed_1 = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed_1 = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed_1 = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed_1 = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed_1 = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed_1 = 117;
pub const _SC_UINT_MAX: C2RustUnnamed_1 = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed_1 = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed_1 = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed_1 = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed_1 = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed_1 = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed_1 = 110;
pub const _SC_NZERO: C2RustUnnamed_1 = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed_1 = 108;
pub const _SC_WORD_BIT: C2RustUnnamed_1 = 107;
pub const _SC_LONG_BIT: C2RustUnnamed_1 = 106;
pub const _SC_INT_MIN: C2RustUnnamed_1 = 105;
pub const _SC_INT_MAX: C2RustUnnamed_1 = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed_1 = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed_1 = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed_1 = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed_1 = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed_1 = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed_1 = 98;
pub const _SC_2_UPE: C2RustUnnamed_1 = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed_1 = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed_1 = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed_1 = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed_1 = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed_1 = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed_1 = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed_1 = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed_1 = 89;
pub const _SC_PASS_MAX: C2RustUnnamed_1 = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed_1 = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed_1 = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed_1 = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed_1 = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed_1 = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed_1 = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed_1 = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed_1 = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed_1 = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed_1 = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed_1 = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed_1 = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed_1 = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed_1 = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed_1 = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed_1 = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed_1 = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed_1 = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed_1 = 68;
pub const _SC_THREADS: C2RustUnnamed_1 = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed_1 = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed_1 = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed_1 = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed_1 = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed_1 = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed_1 = 61;
pub const _SC_IOV_MAX: C2RustUnnamed_1 = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed_1 = 60;
pub const _SC_SELECT: C2RustUnnamed_1 = 59;
pub const _SC_POLL: C2RustUnnamed_1 = 58;
pub const _SC_PII_OSI: C2RustUnnamed_1 = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed_1 = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed_1 = 55;
pub const _SC_PII_XTI: C2RustUnnamed_1 = 54;
pub const _SC_PII: C2RustUnnamed_1 = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed_1 = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed_1 = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed_1 = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed_1 = 49;
pub const _SC_2_C_DEV: C2RustUnnamed_1 = 48;
pub const _SC_2_C_BIND: C2RustUnnamed_1 = 47;
pub const _SC_2_VERSION: C2RustUnnamed_1 = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed_1 = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed_1 = 44;
pub const _SC_LINE_MAX: C2RustUnnamed_1 = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed_1 = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed_1 = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed_1 = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed_1 = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed_1 = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed_1 = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed_1 = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed_1 = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed_1 = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed_1 = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed_1 = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed_1 = 31;
pub const _SC_PAGESIZE: C2RustUnnamed_1 = 30;
pub const _SC_VERSION: C2RustUnnamed_1 = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed_1 = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed_1 = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed_1 = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed_1 = 25;
pub const _SC_AIO_MAX: C2RustUnnamed_1 = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed_1 = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed_1 = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed_1 = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed_1 = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed_1 = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed_1 = 18;
pub const _SC_MEMLOCK: C2RustUnnamed_1 = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed_1 = 16;
pub const _SC_FSYNC: C2RustUnnamed_1 = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed_1 = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed_1 = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed_1 = 12;
pub const _SC_TIMERS: C2RustUnnamed_1 = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed_1 = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed_1 = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed_1 = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed_1 = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed_1 = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed_1 = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed_1 = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed_1 = 3;
pub const _SC_CLK_TCK: C2RustUnnamed_1 = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed_1 = 1;
pub const _SC_ARG_MAX: C2RustUnnamed_1 = 0;
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_11,
    pub _timer: C2RustUnnamed_10,
    pub _rt: C2RustUnnamed_9,
    pub _sigchld: C2RustUnnamed_8,
    pub _sigfault: C2RustUnnamed_5,
    pub _sigpoll: C2RustUnnamed_4,
    pub _sigsys: C2RustUnnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_4 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_6 {
    pub _addr_bnd: C2RustUnnamed_7,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_7 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_12,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_12 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
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
pub type monotonic_clock_type = libc::c_uint;
pub const MONOTONIC_CLOCK_HW: monotonic_clock_type = 1;
pub const MONOTONIC_CLOCK_POSIX: monotonic_clock_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dictEntry {
    pub key: *mut libc::c_void,
    pub v: C2RustUnnamed_13,
    pub next: *mut dictEntry,
    pub metadata: [*mut libc::c_void; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
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
pub type dictResizeEnable = libc::c_uint;
pub const DICT_RESIZE_FORBID: dictResizeEnable = 2;
pub const DICT_RESIZE_AVOID: dictResizeEnable = 1;
pub const DICT_RESIZE_ENABLE: dictResizeEnable = 0;
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
#[derive(Copy, Clone,c2rust_bitfields:: BitfieldStruct)]
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
pub struct RedisModuleCronLoopInfo {
    pub version: uint64_t,
    pub hz: int32_t,
}
pub type RedisModuleCronLoopV1 = RedisModuleCronLoopInfo;
pub type C2RustUnnamed_14 = libc::c_uint;
pub const REPL_STATE_CONNECTED: C2RustUnnamed_14 = 12;
pub const REPL_STATE_TRANSFER: C2RustUnnamed_14 = 11;
pub const REPL_STATE_RECEIVE_PSYNC_REPLY: C2RustUnnamed_14 = 10;
pub const REPL_STATE_SEND_PSYNC: C2RustUnnamed_14 = 9;
pub const REPL_STATE_RECEIVE_CAPA_REPLY: C2RustUnnamed_14 = 8;
pub const REPL_STATE_RECEIVE_IP_REPLY: C2RustUnnamed_14 = 7;
pub const REPL_STATE_RECEIVE_PORT_REPLY: C2RustUnnamed_14 = 6;
pub const REPL_STATE_RECEIVE_AUTH_REPLY: C2RustUnnamed_14 = 5;
pub const REPL_STATE_SEND_HANDSHAKE: C2RustUnnamed_14 = 4;
pub const REPL_STATE_RECEIVE_PING_REPLY: C2RustUnnamed_14 = 3;
pub const REPL_STATE_CONNECTING: C2RustUnnamed_14 = 2;
pub const REPL_STATE_CONNECT: C2RustUnnamed_14 = 1;
pub const REPL_STATE_NONE: C2RustUnnamed_14 = 0;
pub type C2RustUnnamed_15 = libc::c_uint;
pub const FAILOVER_IN_PROGRESS: C2RustUnnamed_15 = 2;
pub const FAILOVER_WAIT_FOR_SYNC: C2RustUnnamed_15 = 1;
pub const NO_FAILOVER: C2RustUnnamed_15 = 0;
pub type pause_type = libc::c_uint;
pub const CLIENT_PAUSE_ALL: pause_type = 2;
pub const CLIENT_PAUSE_WRITE: pause_type = 1;
pub const CLIENT_PAUSE_OFF: pause_type = 0;
pub type pause_purpose = libc::c_uint;
pub const NUM_PAUSE_PURPOSES: pause_purpose = 3;
pub const PAUSE_DURING_FAILOVER: pause_purpose = 2;
pub const PAUSE_DURING_SHUTDOWN: pause_purpose = 1;
pub const PAUSE_BY_CLIENT_COMMAND: pause_purpose = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pause_event {
    pub type_0: pause_type,
    pub end: mstime_t,
}
pub type robj = redisObject;
pub type RedisModuleUserChangedFunc = Option::<
    unsafe extern "C" fn(uint64_t, *mut libc::c_void) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clientReplyBlock {
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replBufBlock {
    pub refcount: libc::c_int,
    pub id: libc::c_longlong,
    pub repl_offset: libc::c_longlong,
    pub size: size_t,
    pub used: size_t,
    pub buf: [libc::c_char; 0],
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
pub struct keySpec {
    pub notes: *const libc::c_char,
    pub flags: uint64_t,
    pub begin_search_type: kspec_bs_type,
    pub bs: C2RustUnnamed_19,
    pub find_keys_type: kspec_fk_type,
    pub fk: C2RustUnnamed_16,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_16 {
    pub range: C2RustUnnamed_18,
    pub keynum: C2RustUnnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub keynumidx: libc::c_int,
    pub firstkey: libc::c_int,
    pub keystep: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
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
pub union C2RustUnnamed_19 {
    pub index: C2RustUnnamed_21,
    pub keyword: C2RustUnnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub keyword: *const libc::c_char,
    pub startfrom: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
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
pub type redisCommandProc = unsafe extern "C" fn(*mut client) -> ();
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
pub struct user {
    pub name: sds,
    pub flags: uint32_t,
    pub passwords: *mut list,
    pub selectors: *mut list,
    pub acl_string: *mut robj,
}
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
pub struct redisMemOverhead {
    pub peak_allocated: size_t,
    pub total_allocated: size_t,
    pub startup_allocated: size_t,
    pub repl_backlog: size_t,
    pub clients_slaves: size_t,
    pub clients_normal: size_t,
    pub cluster_links: size_t,
    pub aof_buffer: size_t,
    pub lua_caches: size_t,
    pub functions_caches: size_t,
    pub overhead_total: size_t,
    pub dataset: size_t,
    pub total_keys: size_t,
    pub bytes_per_key: size_t,
    pub dataset_perc: libc::c_float,
    pub peak_perc: libc::c_float,
    pub total_frag: libc::c_float,
    pub total_frag_bytes: ssize_t,
    pub allocator_frag: libc::c_float,
    pub allocator_frag_bytes: ssize_t,
    pub allocator_rss: libc::c_float,
    pub allocator_rss_bytes: ssize_t,
    pub rss_extra: libc::c_float,
    pub rss_extra_bytes: size_t,
    pub num_dbs: size_t,
    pub db: *mut C2RustUnnamed_22,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_22 {
    pub dbid: size_t,
    pub overhead_ht_main: size_t,
    pub overhead_ht_expires: size_t,
    pub overhead_ht_slot_to_keys: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rdbSaveInfo {
    pub repl_stream_db: libc::c_int,
    pub repl_id_is_set: libc::c_int,
    pub repl_id: [libc::c_char; 41],
    pub repl_offset: libc::c_longlong,
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
    pub inst_metric: [C2RustUnnamed_23; 5],
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
pub struct C2RustUnnamed_23 {
    pub last_sample_time: libc::c_longlong,
    pub last_sample_count: libc::c_longlong,
    pub samples: [libc::c_longlong; 16],
    pub idx: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct redisError {
    pub count: libc::c_longlong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clusterDictEntryMetadata {
    pub prev: *mut dictEntry,
    pub next: *mut dictEntry,
}
pub type rlim_t = __rlim64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
pub type __rlimit_resource_t = __rlimit_resource;
pub type __rlimit_resource = libc::c_uint;
pub const __RLIM_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_NLIMITS: __rlimit_resource = 16;
pub const __RLIMIT_RTTIME: __rlimit_resource = 15;
pub const __RLIMIT_RTPRIO: __rlimit_resource = 14;
pub const __RLIMIT_NICE: __rlimit_resource = 13;
pub const __RLIMIT_MSGQUEUE: __rlimit_resource = 12;
pub const __RLIMIT_SIGPENDING: __rlimit_resource = 11;
pub const __RLIMIT_LOCKS: __rlimit_resource = 10;
pub const __RLIMIT_MEMLOCK: __rlimit_resource = 8;
pub const __RLIMIT_NPROC: __rlimit_resource = 6;
pub const RLIMIT_AS: __rlimit_resource = 9;
pub const __RLIMIT_OFILE: __rlimit_resource = 7;
pub const RLIMIT_NOFILE: __rlimit_resource = 7;
pub const __RLIMIT_RSS: __rlimit_resource = 5;
pub const RLIMIT_CORE: __rlimit_resource = 4;
pub const RLIMIT_STACK: __rlimit_resource = 3;
pub const RLIMIT_DATA: __rlimit_resource = 2;
pub const RLIMIT_FSIZE: __rlimit_resource = 1;
pub const RLIMIT_CPU: __rlimit_resource = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replyFlagNames {
    pub flag: uint64_t,
    pub name: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct commandListFilter {
    pub type_0: commandListFilterType,
    pub arg: sds,
    pub cache: C2RustUnnamed_24,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_24 {
    pub valid: libc::c_int,
    pub u: C2RustUnnamed_25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_25 {
    pub aclcat: uint64_t,
    pub module_handle: *mut libc::c_void,
}
pub type commandListFilterType = libc::c_uint;
pub const COMMAND_LIST_FILTER_PATTERN: commandListFilterType = 2;
pub const COMMAND_LIST_FILTER_ACLCAT: commandListFilterType = 1;
pub const COMMAND_LIST_FILTER_MODULE: commandListFilterType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2RustUnnamed_39,
    pub c2rust_unnamed_0: C2RustUnnamed_38,
    pub c2rust_unnamed_1: C2RustUnnamed_37,
    pub c2rust_unnamed_2: C2RustUnnamed_36,
    pub c2rust_unnamed_3: C2RustUnnamed_35,
    pub c2rust_unnamed_4: C2RustUnnamed_34,
    pub c2rust_unnamed_5: C2RustUnnamed_33,
    pub c2rust_unnamed_6: C2RustUnnamed_32,
    pub c2rust_unnamed_7: C2RustUnnamed_31,
    pub c2rust_unnamed_8: C2RustUnnamed_30,
    pub c2rust_unnamed_9: C2RustUnnamed_29,
    pub c2rust_unnamed_10: C2RustUnnamed_28,
    pub c2rust_unnamed_11: C2RustUnnamed_27,
    pub c2rust_unnamed_12: C2RustUnnamed_26,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_26 {
    pub ru_nivcsw: libc::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_27 {
    pub ru_nvcsw: libc::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_28 {
    pub ru_nsignals: libc::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_29 {
    pub ru_msgrcv: libc::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_30 {
    pub ru_msgsnd: libc::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_31 {
    pub ru_oublock: libc::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_32 {
    pub ru_inblock: libc::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_33 {
    pub ru_nswap: libc::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_34 {
    pub ru_majflt: libc::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_35 {
    pub ru_minflt: libc::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_36 {
    pub ru_isrss: libc::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_37 {
    pub ru_idrss: libc::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_38 {
    pub ru_ixrss: libc::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_39 {
    pub ru_maxrss: libc::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
pub type __rusage_who_t = __rusage_who;
pub type __rusage_who = libc::c_int;
pub const RUSAGE_THREAD: __rusage_who = 1;
pub const RUSAGE_CHILDREN: __rusage_who = -1;
pub const RUSAGE_SELF: __rusage_who = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [libc::c_char; 65],
    pub nodename: [libc::c_char; 65],
    pub release: [libc::c_char; 65],
    pub version: [libc::c_char; 65],
    pub machine: [libc::c_char; 65],
    pub domainname: [libc::c_char; 65],
}
#[inline]
unsafe extern "C" fn elapsedUs(mut start_time: monotime) -> uint64_t {
    return (getMonotonicUs.expect("non-null function pointer")())
        .wrapping_sub(start_time);
}
#[inline]
unsafe extern "C" fn elapsedMs(mut start_time: monotime) -> uint64_t {
    return (elapsedUs(start_time)).wrapping_div(1000 as libc::c_int as libc::c_ulong);
}
#[inline]
unsafe extern "C" fn sdsalloc(s: sds) -> size_t {
    let mut flags: libc::c_uchar = *s.offset(-(1 as libc::c_int) as isize)
        as libc::c_uchar;
    match flags as libc::c_int & 7 as libc::c_int {
        0 => return (flags as libc::c_int >> 3 as libc::c_int) as size_t,
        1 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr8>() as libc::c_ulong as isize))
                as *mut sdshdr8))
                .alloc as size_t;
        }
        2 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr16>() as libc::c_ulong as isize))
                as *mut sdshdr16))
                .alloc as size_t;
        }
        3 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr32>() as libc::c_ulong as isize))
                as *mut sdshdr32))
                .alloc as size_t;
        }
        4 => {
            return (*(s
                .offset(-(core::mem::size_of::<sdshdr64>() as libc::c_ulong as isize))
                as *mut sdshdr64))
                .alloc;
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[no_mangle]
pub static mut server:redisServer = redisServer {
    pid: 0,
    main_thread_id: 0,
    configfile: std::ptr::null_mut(),
    executable: std::ptr::null_mut(),
    exec_argv: std::ptr::null_mut(),
    dynamic_hz: 0,
    config_hz: 0,
    umask: 0,
    hz: 0,
    in_fork_child: 0,
    db: std::ptr::null_mut(),
    commands: std::ptr::null_mut(),
    orig_commands: std::ptr::null_mut(),
    el: std::ptr::null_mut(),
    errors: std::ptr::null_mut(),
    lruclock: 0 as u32,
    shutdown_asap: 0,
    shutdown_mstime: 0,
    last_sig_received: 0,
    shutdown_flags: 0,
    activerehashing: 0,
    active_defrag_running: 0,
    pidfile: std::ptr::null_mut(),
    arch_bits: 0,
    cronloops: 0,
    runid: [0; 41],
    sentinel_mode: 0,
    initial_memory_usage: 0,
    always_show_logo: 0,
    in_exec: 0,
    busy_module_yield_flags: 0,
    busy_module_yield_reply: std::ptr::null(),
    core_propagates: 0,
    propagate_no_multi: 0,
    module_ctx_nesting: 0,
    ignore_warnings: std::ptr::null_mut(),
    client_pause_in_transaction: 0,
    thp_enabled: 0,
    page_size: 0,
    moduleapi: std::ptr::null_mut(),
    sharedapi: std::ptr::null_mut(),
    module_configs_queue: std::ptr::null_mut(),
    loadmodule_queue: std::ptr::null_mut(),
    module_pipe: [0, 0],
    child_pid: 0,
    child_type: 0,
    port: 0,
    tls_port: 0,
    tcp_backlog: 0,
    bindaddr: [std::ptr::null_mut(); 16],
    bindaddr_count: 0,
    bind_source_addr: std::ptr::null_mut(),
    unixsocket: std::ptr::null_mut(),
    unixsocketperm: 0,
    ipfd: socketFds { fd: [0; 16], count: 0},
    tlsfd: socketFds { fd: [0; 16], count: 0},
    sofd: -1,
    socket_mark_id: 0,
    cfd: socketFds { fd: [0; 16], count: 0},
    clients: 0 as *mut list,
    clients_to_close: 0 as *mut list,
    clients_pending_write: 0 as *mut list,
    clients_pending_read: 0 as *mut list,
    slaves: 0 as *mut list,
    monitors: 0 as *mut list,
    current_client: 0 as *mut client,
    client_mem_usage_buckets: 0 as *mut clientMemUsageBucket,
    clients_timeout_table: 0 as *mut rax,
    fixed_time_expire: 0,
    in_nested_call: 0,
    clients_index: 0 as *mut rax,
    client_pause_type: 0,
    postponed_clients: 0 as *mut list,
    client_pause_end_time: 0,
    client_pause_per_purpose: [0 as *mut pause_event; 3],
    neterr: [0; 256],
    migrate_cached_sockets: 0 as *mut dict,
    next_client_id: 0,
    protected_mode: 0,
    io_threads_num: 0,
    io_threads_do_reads: 0,
    io_threads_active: 0,
    events_processed_while_blocked: 0,
    enable_protected_configs: 0,
    enable_debug_cmd: 0,
    enable_module_cmd: 0,
    loading: 0,
    async_loading: 0,
    loading_total_bytes: 0,
    loading_rdb_used_mem: 0,
    loading_loaded_bytes: 0,
    loading_start_time: 0,
    loading_process_events_interval_bytes: 0,
    stat_starttime: 0,
    stat_numcommands: 0,
    stat_numconnections: 0,
    stat_expiredkeys: 0,
    stat_expired_stale_perc: 0 as f64,
    stat_expired_time_cap_reached_count: 0,
    stat_expire_cycle_time_used: 0,
    stat_evictedkeys: 0,
    stat_evictedclients: 0,
    stat_total_eviction_exceeded_time: 0,
    stat_last_eviction_exceeded_time: 0,
    stat_keyspace_hits: 0,
    stat_keyspace_misses: 0,
    stat_active_defrag_hits: 0,
    stat_active_defrag_misses: 0,
    stat_active_defrag_key_hits: 0,
    stat_active_defrag_key_misses: 0,
    stat_active_defrag_scanned: 0,
    stat_total_active_defrag_time: 0,
    stat_last_active_defrag_time: 0,
    stat_peak_memory: 0,
    stat_aof_rewrites: 0,
    stat_aofrw_consecutive_failures: 0,
    stat_rdb_saves: 0,
    stat_fork_time: 0,
    stat_fork_rate: 0 as f64,
    stat_total_forks: 0,
    stat_rejected_conn: 0,
    stat_sync_full: 0,
    stat_sync_partial_ok: 0,
    stat_sync_partial_err: 0,
    slowlog: 0 as *mut list,
    slowlog_entry_id: 0,
    slowlog_log_slower_than: 0,
    slowlog_max_len: 0,
    cron_malloc_stats: malloc_stats { zmalloc_used: 0, process_rss: 0, allocator_allocated: 0, allocator_active: 0, allocator_resident: 0 },
    stat_net_input_bytes: 0,
    stat_net_output_bytes: 0,
    stat_net_repl_input_bytes: 0,
    stat_net_repl_output_bytes: 0,
    stat_current_cow_peak: 0,
    stat_current_cow_bytes: 0,
    stat_current_cow_updated: 0,
    stat_current_save_keys_processed: 0,
    stat_current_save_keys_total: 0,
    stat_rdb_cow_bytes: 0,
    stat_aof_cow_bytes: 0,
    stat_module_cow_bytes: 0,
    stat_module_progress: 0 as f64,
    stat_clients_type_memory: [0 as u64;4],
    stat_cluster_links_memory: 0,
    stat_unexpected_error_replies: 0,
    stat_total_error_replies: 0,
    stat_dump_payload_sanitizations: 0,
    stat_io_reads_processed: 0,
    stat_io_writes_processed: 0,
    stat_total_reads_processed: 0,
    stat_total_writes_processed: 0,
    inst_metric: [C2RustUnnamed_23{ last_sample_time: 0, last_sample_count: 0, samples: [0 as i64;16], idx: 0 }; 5],
    stat_reply_buffer_shrinks: 0,
    stat_reply_buffer_expands: 0,
    verbosity: 0,
    maxidletime: 0,
    tcpkeepalive: 0,
    active_expire_enabled: 0,
    active_expire_effort: 0,
    active_defrag_enabled: 0,
    sanitize_dump_payload: 0,
    skip_checksum_validation: 0,
    jemalloc_bg_thread: 0,
    active_defrag_ignore_bytes: 0,
    active_defrag_threshold_lower: 0,
    active_defrag_threshold_upper: 0,
    active_defrag_cycle_min: 0,
    active_defrag_cycle_max: 0,
    active_defrag_max_scan_fields: 0,
    client_max_querybuf_len: 0,
    dbnum: 0,
    supervised: 0,
    supervised_mode: 0,
    daemonize: 0,
    set_proc_title: 0,
    proc_title_template: 0 as *mut i8,
    client_obuf_limits: [clientBufferLimitsConfig{ hard_limit_bytes: 0, soft_limit_bytes: 0, soft_limit_seconds: 0 }; 3],
    pause_cron: 0,
    latency_tracking_enabled: 0,
    latency_tracking_info_percentiles: 0 as *mut f64,
    latency_tracking_info_percentiles_len: 0,
    aof_enabled: 0,
    aof_state: 0,
    aof_fsync: 0,
    aof_filename: 0 as *mut i8,
    aof_dirname: 0 as *mut i8,
    aof_no_fsync_on_rewrite: 0,
    aof_rewrite_perc: 0,
    aof_rewrite_min_size: 0,
    aof_rewrite_base_size: 0,
    aof_current_size: 0,
    aof_last_incr_size: 0,
    aof_fsync_offset: 0,
    aof_flush_sleep: 0,
    aof_rewrite_scheduled: 0,
    aof_buf: 0 as *mut i8,
    aof_fd: 0,
    aof_selected_db: 0,
    aof_flush_postponed_start: 0,
    aof_last_fsync: 0,
    aof_rewrite_time_last: 0,
    aof_rewrite_time_start: 0,
    aof_cur_timestamp: 0,
    aof_timestamp_enabled: 0,
    aof_lastbgrewrite_status: 0,
    aof_delayed_fsync: 0,
    aof_rewrite_incremental_fsync: 0,
    rdb_save_incremental_fsync: 0,
    aof_last_write_status: 0,
    aof_last_write_errno: 0,
    aof_load_truncated: 0,
    aof_use_rdb_preamble: 0,
    aof_bio_fsync_status: 0,
    aof_bio_fsync_errno: 0,
    aof_manifest: 0 as *mut aofManifest,
    aof_disable_auto_gc: 0,
    dirty: 0,
    dirty_before_bgsave: 0,
    rdb_last_load_keys_expired: 0,
    rdb_last_load_keys_loaded: 0,
    saveparams: 0 as *mut saveparam,
    saveparamslen: 0,
    rdb_filename: 0 as *mut i8,
    rdb_compression: 0,
    rdb_checksum: 0,
    rdb_del_sync_files: 0,
    lastsave: 0,
    lastbgsave_try: 0,
    rdb_save_time_last: 0,
    rdb_save_time_start: 0,
    rdb_bgsave_scheduled: 0,
    rdb_child_type: 0,
    lastbgsave_status: 0,
    stop_writes_on_bgsave_err: 0,
    rdb_pipe_read: 0,
    rdb_child_exit_pipe: 0,
    rdb_pipe_conns: 0 as *mut *mut connection,
    rdb_pipe_numconns: 0,
    rdb_pipe_numconns_writing: 0,
    rdb_pipe_buff: 0 as *mut i8,
    rdb_pipe_bufflen: 0,
    rdb_key_save_delay: 0,
    key_load_delay: 0,
    child_info_pipe: [0 as i32;2],
    child_info_nread: 0,
    also_propagate: redisOpArray { ops: 0 as *mut redisOp, numops: 0, capacity: 0 },
    replication_allowed: 0,
    logfile: 0 as *mut i8,
    syslog_enabled: 0,
    syslog_ident: 0 as *mut i8,
    syslog_facility: 0,
    crashlog_enabled: 0,
    memcheck_enabled: 0,
    use_exit_on_panic: 0,
    shutdown_timeout: 0,
    shutdown_on_sigint: 0,
    shutdown_on_sigterm: 0,
    replid: [0 as i8;41],
    replid2: [0 as i8;41],
    master_repl_offset: 0,
    second_replid_offset: 0,
    slaveseldb: 0,
    repl_ping_slave_period: 0,
    repl_backlog: 0 as *mut replBacklog,
    repl_backlog_size: 0,
    repl_backlog_time_limit: 0,
    repl_no_slaves_since: 0,
    repl_min_slaves_to_write: 0,
    repl_min_slaves_max_lag: 0,
    repl_good_slaves_count: 0,
    repl_diskless_sync: 0,
    repl_diskless_load: 0,
    repl_diskless_sync_delay: 0,
    repl_diskless_sync_max_replicas: 0,
    repl_buffer_mem: 0,
    repl_buffer_blocks: 0 as *mut list,
    masteruser: 0 as *mut i8,
    masterauth: 0 as *mut i8,
    masterhost: 0 as *mut i8,
    masterport: 0,
    repl_timeout: 0,
    master: 0 as *mut client,
    cached_master: 0 as *mut client,
    repl_syncio_timeout: 0,
    repl_state: 0,
    repl_transfer_size: 0,
    repl_transfer_read: 0,
    repl_transfer_last_fsync_off: 0,
    repl_transfer_s: 0 as *mut connection,
    repl_transfer_fd: 0,
    repl_transfer_tmpfile: 0 as *mut i8,
    repl_transfer_lastio: 0,
    repl_serve_stale_data: 0,
    repl_slave_ro: 0,
    repl_slave_ignore_maxmemory: 0,
    repl_down_since: 0,
    repl_disable_tcp_nodelay: 0,
    slave_priority: 0,
    replica_announced: 0,
    slave_announce_port: 0,
    slave_announce_ip: 0 as *mut i8,
    propagation_error_behavior: 0,
    repl_ignore_disk_write_error: 0,
    master_replid: [0 as i8;41],
    master_initial_offset: 0,
    repl_slave_lazy_flush: 0,
    clients_waiting_acks: 0 as *mut list,
    get_ack_from_slaves: 0,
    maxclients: 0,
    maxmemory: 0,
    maxmemory_clients: 0,
    maxmemory_policy: 0,
    maxmemory_samples: 0,
    maxmemory_eviction_tenacity: 0,
    lfu_log_factor: 0,
    lfu_decay_time: 0,
    proto_max_bulk_len: 0,
    oom_score_adj_values: [0 as i32;3],
    oom_score_adj: 0,
    disable_thp: 0,
    blocked_clients: 0,
    blocked_clients_by_type: [0 as u32;8],
    unblocked_clients: 0 as *mut list,
    ready_keys: 0 as *mut list,
    tracking_clients: 0,
    tracking_table_max_keys: 0,
    tracking_pending_keys: 0 as *mut list,
    sort_desc: 0,
    sort_alpha: 0,
    sort_bypattern: 0,
    sort_store: 0,
    hash_max_listpack_entries: 0,
    hash_max_listpack_value: 0,
    set_max_intset_entries: 0,
    zset_max_listpack_entries: 0,
    zset_max_listpack_value: 0,
    hll_sparse_max_bytes: 0,
    stream_node_max_bytes: 0,
    stream_node_max_entries: 0,
    list_max_listpack_size: 0,
    list_compress_depth: 0,
    unixtime: 0,
    timezone: 0,
    daylight_active: 0,
    mstime: 0,
    ustime: 0,
    blocking_op_nesting: 0,
    blocked_last_cron: 0,
    pubsub_channels: 0 as *mut dict,
    pubsub_patterns: 0 as *mut dict,
    notify_keyspace_events: 0,
    pubsubshard_channels: 0 as *mut dict,
    cluster_enabled: 0,
    cluster_port: 0,
    cluster_node_timeout: 0,
    cluster_configfile: 0 as *mut i8,
    cluster: 0 as *mut clusterState,
    cluster_migration_barrier: 0,
    cluster_allow_replica_migration: 0,
    cluster_slave_validity_factor: 0,
    cluster_require_full_coverage: 0,
    cluster_slave_no_failover: 0,
    cluster_announce_ip: 0 as *mut i8,
    cluster_announce_hostname: 0 as *mut i8,
    cluster_preferred_endpoint_type: 0,
    cluster_announce_port: 0,
    cluster_announce_tls_port: 0,
    cluster_announce_bus_port: 0,
    cluster_module_flags: 0,
    cluster_allow_reads_when_down: 0,
    cluster_config_file_lock_fd: 0,
    cluster_link_sendbuf_limit_bytes: 0,
    cluster_drop_packet_filter: 0,
    script_caller: 0 as *mut client,
    busy_reply_threshold: 0,
    pre_command_oom_state: 0,
    script_disable_deny_script: 0,
    lazyfree_lazy_eviction: 0,
    lazyfree_lazy_expire: 0,
    lazyfree_lazy_server_del: 0,
    lazyfree_lazy_user_del: 0,
    lazyfree_lazy_user_flush: 0,
    latency_monitor_threshold: 0,
    latency_events: 0 as *mut dict,
    acl_filename: 0 as *mut i8,
    acllog_max_len: 0,
    requirepass: 0 as *mut i8,
    acl_pubsub_default: 0,
    watchdog_period: 0,
    system_memory_size: 0,
    tls_cluster: 0,
    tls_replication: 0,
    tls_auth_clients: 0,
    tls_ctx_config: redisTLSContextConfig{ cert_file: 0 as *mut i8, key_file: 0 as *mut i8, key_file_pass: 0 as *mut i8, client_cert_file: 0 as *mut i8, client_key_file: 0 as *mut i8, client_key_file_pass: 0 as *mut i8, dh_params_file: 0 as *mut i8, ca_cert_file: 0 as *mut i8, ca_cert_dir: 0 as *mut i8, protocols: 0 as *mut i8, ciphers: 0 as *mut i8, ciphersuites: 0 as *mut i8, prefer_server_ciphers: 0, session_caching: 0, session_cache_size: 0, session_cache_timeout: 0 },
    server_cpulist: 0 as *mut i8,
    bio_cpulist: 0 as *mut i8,
    aof_rewrite_cpulist: 0 as *mut i8,
    bgsave_cpulist: 0 as *mut i8,
    sentinel_config: 0 as *mut sentinelConfig,
    failover_end_time: 0,
    force_failover: 0,
    target_replica_host: 0 as *mut i8,
    target_replica_port: 0,
    failover_state: 0,
    cluster_allow_pubsubshard_when_down: 0,
    reply_buffer_peak_reset_time: 0,
    reply_buffer_resizing_enabled: 0,
};
#[no_mangle]
pub static mut shared: sharedObjectsStruct = sharedObjectsStruct {
    crlf: 0 as *const robj as *mut robj,
    ok: 0 as *const robj as *mut robj,
    err: 0 as *const robj as *mut robj,
    emptybulk: 0 as *const robj as *mut robj,
    czero: 0 as *const robj as *mut robj,
    cone: 0 as *const robj as *mut robj,
    pong: 0 as *const robj as *mut robj,
    space: 0 as *const robj as *mut robj,
    queued: 0 as *const robj as *mut robj,
    null: [0 as *const robj as *mut robj; 4],
    nullarray: [0 as *const robj as *mut robj; 4],
    emptymap: [0 as *const robj as *mut robj; 4],
    emptyset: [0 as *const robj as *mut robj; 4],
    emptyarray: 0 as *const robj as *mut robj,
    wrongtypeerr: 0 as *const robj as *mut robj,
    nokeyerr: 0 as *const robj as *mut robj,
    syntaxerr: 0 as *const robj as *mut robj,
    sameobjecterr: 0 as *const robj as *mut robj,
    outofrangeerr: 0 as *const robj as *mut robj,
    noscripterr: 0 as *const robj as *mut robj,
    loadingerr: 0 as *const robj as *mut robj,
    slowevalerr: 0 as *const robj as *mut robj,
    slowscripterr: 0 as *const robj as *mut robj,
    slowmoduleerr: 0 as *const robj as *mut robj,
    bgsaveerr: 0 as *const robj as *mut robj,
    masterdownerr: 0 as *const robj as *mut robj,
    roslaveerr: 0 as *const robj as *mut robj,
    execaborterr: 0 as *const robj as *mut robj,
    noautherr: 0 as *const robj as *mut robj,
    noreplicaserr: 0 as *const robj as *mut robj,
    busykeyerr: 0 as *const robj as *mut robj,
    oomerr: 0 as *const robj as *mut robj,
    plus: 0 as *const robj as *mut robj,
    messagebulk: 0 as *const robj as *mut robj,
    pmessagebulk: 0 as *const robj as *mut robj,
    subscribebulk: 0 as *const robj as *mut robj,
    unsubscribebulk: 0 as *const robj as *mut robj,
    psubscribebulk: 0 as *const robj as *mut robj,
    punsubscribebulk: 0 as *const robj as *mut robj,
    del: 0 as *const robj as *mut robj,
    unlink: 0 as *const robj as *mut robj,
    rpop: 0 as *const robj as *mut robj,
    lpop: 0 as *const robj as *mut robj,
    lpush: 0 as *const robj as *mut robj,
    rpoplpush: 0 as *const robj as *mut robj,
    lmove: 0 as *const robj as *mut robj,
    blmove: 0 as *const robj as *mut robj,
    zpopmin: 0 as *const robj as *mut robj,
    zpopmax: 0 as *const robj as *mut robj,
    emptyscan: 0 as *const robj as *mut robj,
    multi: 0 as *const robj as *mut robj,
    exec: 0 as *const robj as *mut robj,
    left: 0 as *const robj as *mut robj,
    right: 0 as *const robj as *mut robj,
    hset: 0 as *const robj as *mut robj,
    srem: 0 as *const robj as *mut robj,
    xgroup: 0 as *const robj as *mut robj,
    xclaim: 0 as *const robj as *mut robj,
    script: 0 as *const robj as *mut robj,
    replconf: 0 as *const robj as *mut robj,
    eval: 0 as *const robj as *mut robj,
    persist: 0 as *const robj as *mut robj,
    set: 0 as *const robj as *mut robj,
    pexpireat: 0 as *const robj as *mut robj,
    pexpire: 0 as *const robj as *mut robj,
    time: 0 as *const robj as *mut robj,
    pxat: 0 as *const robj as *mut robj,
    absttl: 0 as *const robj as *mut robj,
    retrycount: 0 as *const robj as *mut robj,
    force: 0 as *const robj as *mut robj,
    justid: 0 as *const robj as *mut robj,
    entriesread: 0 as *const robj as *mut robj,
    lastid: 0 as *const robj as *mut robj,
    ping: 0 as *const robj as *mut robj,
    setid: 0 as *const robj as *mut robj,
    keepttl: 0 as *const robj as *mut robj,
    load: 0 as *const robj as *mut robj,
    createconsumer: 0 as *const robj as *mut robj,
    getack: 0 as *const robj as *mut robj,
    special_asterick: 0 as *const robj as *mut robj,
    special_equals: 0 as *const robj as *mut robj,
    default_username: 0 as *const robj as *mut robj,
    redacted: 0 as *const robj as *mut robj,
    ssubscribebulk: 0 as *const robj as *mut robj,
    sunsubscribebulk: 0 as *const robj as *mut robj,
    smessagebulk: 0 as *const robj as *mut robj,
    select: [0 as *const robj as *mut robj; 10],
    integers: [0 as *const robj as *mut robj; 10000],
    mbulkhdr: [0 as *const robj as *mut robj; 32],
    bulkhdr: [0 as *const robj as *mut robj; 32],
    maphdr: [0 as *const robj as *mut robj; 32],
    sethdr: [0 as *const robj as *mut robj; 32],
    minstring: 0 as *const libc::c_char as *mut libc::c_char,
    maxstring: 0 as *const libc::c_char as *mut libc::c_char,
};
#[no_mangle]

#[no_mangle]
pub static mut R_Zero: libc::c_double = 0.;
#[no_mangle]
pub static mut R_PosInf: libc::c_double = 0.;
#[no_mangle]
pub static mut R_NegInf: libc::c_double = 0.;
#[no_mangle]
pub static mut R_Nan: libc::c_double = 0.;
#[no_mangle]
pub unsafe extern "C" fn serverLogRaw(
    mut level: libc::c_int,
    mut msg: *const libc::c_char,
) {
    let syslogLevelMap: [libc::c_int; 4] = [
        7 as libc::c_int,
        6 as libc::c_int,
        5 as libc::c_int,
        4 as libc::c_int,
    ];
    let mut c: *const libc::c_char = b".-*#\0" as *const u8 as *const libc::c_char;
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 64] = [0; 64];
    let mut rawmode: libc::c_int = level & (1 as libc::c_int) << 10 as libc::c_int;
    let mut log_to_stdout: libc::c_int = (*(server.logfile)
        .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32) as libc::c_int;
    level &= 0xff as libc::c_int;
    if level < server.verbosity {
        return;
    }
    fp = if log_to_stdout != 0 {
        stdout
    } else {
        fopen(server.logfile, b"a\0" as *const u8 as *const libc::c_char)
    };
    if fp.is_null() {
        return;
    }
    if rawmode != 0 {
        fprintf(fp, b"%s\0" as *const u8 as *const libc::c_char, msg);
    } else {
        let mut off: libc::c_int = 0;
        let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
        let mut role_char: libc::c_int = 0;
        let mut pid: pid_t = getpid();
        gettimeofday(&mut tv, 0 as *mut libc::c_void);
        let mut tm: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        nolocks_localtime(&mut tm, tv.tv_sec, server.timezone, server.daylight_active);
        off = strftime(
            buf.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            b"%d %b %Y %H:%M:%S.\0" as *const u8 as *const libc::c_char,
            &mut tm,
        ) as libc::c_int;
        snprintf(
            buf.as_mut_ptr().offset(off as isize),
            (core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
                .wrapping_sub(off as libc::c_ulong),
            b"%03d\0" as *const u8 as *const libc::c_char,
            tv.tv_usec as libc::c_int / 1000 as libc::c_int,
        );
        if server.sentinel_mode != 0 {
            role_char = 'X' as i32;
        } else if pid != server.pid {
            role_char = 'C' as i32;
        } else {
            role_char = if !(server.masterhost).is_null() {
                'S' as i32
            } else {
                'M' as i32
            };
        }
        fprintf(
            fp,
            b"%d:%c %s %c %s\n\0" as *const u8 as *const libc::c_char,
            getpid(),
            role_char,
            buf.as_mut_ptr(),
            *c.offset(level as isize) as libc::c_int,
            msg,
        );
    }
    fflush(fp);
    if log_to_stdout == 0 {
        fclose(fp);
    }
    if server.syslog_enabled != 0 {
        syslog(
            syslogLevelMap[level as usize],
            b"%s\0" as *const u8 as *const libc::c_char,
            msg,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn _serverLog(
    mut level: libc::c_int,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    let mut msg: [libc::c_char; 1024] = [0; 1024];
    ap = args.clone();
    vsnprintf(
        msg.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    serverLogRaw(level, msg.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn serverLogFromHandler(
    mut level: libc::c_int,
    mut msg: *const libc::c_char,
) {
    let mut fd: libc::c_int = 0;
    let mut log_to_stdout: libc::c_int = (*(server.logfile)
        .offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32) as libc::c_int;
    let mut buf: [libc::c_char; 64] = [0; 64];
    if (level & 0xff as libc::c_int) < server.verbosity
        || log_to_stdout != 0 && server.daemonize != 0
    {
        return;
    }
    fd = if log_to_stdout != 0 {
        1 as libc::c_int
    } else {
        open(
            server.logfile,
            0o2000 as libc::c_int | 0o100 as libc::c_int | 0o1 as libc::c_int,
            0o644 as libc::c_int,
        )
    };
    if fd == -(1 as libc::c_int) {
        return;
    }
    ll2string(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
        getpid() as libc::c_longlong,
    );
    if !(write(fd, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()))
        == -(1 as libc::c_int) as libc::c_long)
    {
        if !(write(
            fd,
            b":signal-handler (\0" as *const u8 as *const libc::c_char
                as *const libc::c_void,
            17 as libc::c_int as size_t,
        ) == -(1 as libc::c_int) as libc::c_long)
        {
            ll2string(
                buf.as_mut_ptr(),
                core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                time(0 as *mut time_t) as libc::c_longlong,
            );
            if !(write(
                fd,
                buf.as_mut_ptr() as *const libc::c_void,
                strlen(buf.as_mut_ptr()),
            ) == -(1 as libc::c_int) as libc::c_long)
            {
                if !(write(
                    fd,
                    b") \0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as size_t,
                ) == -(1 as libc::c_int) as libc::c_long)
                {
                    if !(write(fd, msg as *const libc::c_void, strlen(msg))
                        == -(1 as libc::c_int) as libc::c_long)
                    {
                        write(
                            fd,
                            b"\n\0" as *const u8 as *const libc::c_char
                                as *const libc::c_void,
                            1 as libc::c_int as size_t,
                        ) == -(1 as libc::c_int) as libc::c_long;
                    }
                }
            }
        }
    }
    if log_to_stdout == 0 {
        close(fd);
    }
}
#[no_mangle]
pub unsafe extern "C" fn ustime() -> libc::c_longlong {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ust: libc::c_longlong = 0;
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    ust = tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong;
    ust += tv.tv_usec as libc::c_longlong;
    return ust;
}
#[no_mangle]
pub unsafe extern "C" fn mstime() -> libc::c_longlong {
    return ustime() / 1000 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn exitFromChild(mut retcode: libc::c_int) {
    _exit(retcode);
}
#[no_mangle]
pub unsafe extern "C" fn dictVanillaFree(mut d: *mut dict, mut val: *mut libc::c_void) {
    zfree(val);
}
#[no_mangle]
pub unsafe extern "C" fn dictListDestructor(
    mut d: *mut dict,
    mut val: *mut libc::c_void,
) {
    listRelease(val as *mut list);
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = sdslen(key1 as sds) as libc::c_int;
    l2 = sdslen(key2 as sds) as libc::c_int;
    if l1 != l2 {
        return 0 as libc::c_int;
    }
    return (memcmp(key1, key2, l1 as libc::c_ulong) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsKeyCaseCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return (strcasecmp(key1 as *const libc::c_char, key2 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictObjectDestructor(
    mut d: *mut dict,
    mut val: *mut libc::c_void,
) {
    if val.is_null() {
        return;
    }
    decrRefCount(val as *mut robj);
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsDestructor(
    mut d: *mut dict,
    mut val: *mut libc::c_void,
) {
    sdsfree(val as sds);
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsDup(
    mut d: *mut dict,
    mut key: *const libc::c_void,
) -> *mut libc::c_void {
    return sdsdup(key as sds) as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn dictObjKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut o1: *const robj = key1 as *const robj;
    let mut o2: *const robj = key2 as *const robj;
    return dictSdsKeyCompare(d, (*o1).ptr, (*o2).ptr);
}
#[no_mangle]
pub unsafe extern "C" fn dictObjHash(mut key: *const libc::c_void) -> uint64_t {
    let mut o: *const robj = key as *const robj;
    return dictGenHashFunction((*o).ptr, sdslen((*o).ptr as sds));
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenHashFunction(
        key as *mut libc::c_uchar as *const libc::c_void,
        sdslen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dictSdsCaseHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenCaseHashFunction(
        key as *mut libc::c_uchar,
        sdslen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn distCStrHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenHashFunction(
        key as *mut libc::c_uchar as *const libc::c_void,
        strlen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn distCStrCaseHash(mut key: *const libc::c_void) -> uint64_t {
    return dictGenCaseHashFunction(
        key as *mut libc::c_uchar,
        strlen(key as *mut libc::c_char),
    );
}
#[no_mangle]
pub unsafe extern "C" fn distCStrKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut l1: libc::c_int = 0;
    let mut l2: libc::c_int = 0;
    l1 = strlen(key1 as *mut libc::c_char) as libc::c_int;
    l2 = strlen(key2 as *mut libc::c_char) as libc::c_int;
    if l1 != l2 {
        return 0 as libc::c_int;
    }
    return (memcmp(key1, key2, l1 as libc::c_ulong) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn distCStrKeyCaseCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    return (strcasecmp(key1 as *const libc::c_char, key2 as *const libc::c_char)
        == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dictEncObjKeyCompare(
    mut d: *mut dict,
    mut key1: *const libc::c_void,
    mut key2: *const libc::c_void,
) -> libc::c_int {
    let mut o1: *mut robj = key1 as *mut robj;
    let mut o2: *mut robj = key2 as *mut robj;
    let mut cmp: libc::c_int = 0;
    if (*o1).encoding() as libc::c_int == 1 as libc::c_int
        && (*o2).encoding() as libc::c_int == 1 as libc::c_int
    {
        return ((*o1).ptr == (*o2).ptr) as libc::c_int;
    }
    if (*o1).refcount != 2147483647 as libc::c_int - 1 as libc::c_int {
        o1 = getDecodedObject(o1);
    }
    if (*o2).refcount != 2147483647 as libc::c_int - 1 as libc::c_int {
        o2 = getDecodedObject(o2);
    }
    cmp = dictSdsKeyCompare(d, (*o1).ptr, (*o2).ptr);
    if (*o1).refcount != 2147483647 as libc::c_int - 1 as libc::c_int {
        decrRefCount(o1);
    }
    if (*o2).refcount != 2147483647 as libc::c_int - 1 as libc::c_int {
        decrRefCount(o2);
    }
    return cmp;
}
#[no_mangle]
pub unsafe extern "C" fn dictEncObjHash(mut key: *const libc::c_void) -> uint64_t {
    let mut o: *mut robj = key as *mut robj;
    if (*o).encoding() as libc::c_int == 0 as libc::c_int
        || (*o).encoding() as libc::c_int == 8 as libc::c_int
    {
        return dictGenHashFunction((*o).ptr, sdslen((*o).ptr as sds))
    } else {
        if (*o).encoding() as libc::c_int == 1 as libc::c_int {
            let mut buf: [libc::c_char; 32] = [0; 32];
            let mut len: libc::c_int = 0;
            len = ll2string(
                buf.as_mut_ptr(),
                32 as libc::c_int as size_t,
                (*o).ptr as libc::c_long as libc::c_longlong,
            );
            return dictGenHashFunction(
                buf.as_mut_ptr() as *mut libc::c_uchar as *const libc::c_void,
                len as size_t,
            );
        } else {
            _serverPanic(
                b"server.c\0" as *const u8 as *const libc::c_char,
                353 as libc::c_int,
                b"Unknown string encoding\0" as *const u8 as *const libc::c_char,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn dictExpandAllowed(
    mut moreMem: size_t,
    mut usedRatio: libc::c_double,
) -> libc::c_int {
    if usedRatio <= 1.618f64 {
        return (overMaxmemoryAfterAlloc(moreMem) == 0) as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn dictEntryMetadataSize(mut d: *mut dict) -> size_t {
    return if server.cluster_enabled != 0 {
        core::mem::size_of::<clusterDictEntryMetadata>() as libc::c_ulong
    } else {
        0 as libc::c_int as libc::c_ulong
    };
}
#[no_mangle]
pub static mut objectKeyPointerValueDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictEncObjHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictEncObjKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictObjectDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut objectKeyHeapPointerValueDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictEncObjHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictEncObjKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictObjectDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictVanillaFree
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut setDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut zsetDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
pub static mut dbDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
                dictObjectDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: Some(
                dictExpandAllowed
                    as unsafe extern "C" fn(size_t, libc::c_double) -> libc::c_int,
            ),
            dictEntryMetadataBytes: Some(
                dictEntryMetadataSize as unsafe extern "C" fn(*mut dict) -> size_t,
            ),
        };
        init
    }
};
#[no_mangle]
pub static mut dbExpiresDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: None,
            valDestructor: None,
            expandAllowed: Some(
                dictExpandAllowed
                    as unsafe extern "C" fn(size_t, libc::c_double) -> libc::c_int,
            ),
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut commandTableDictType: dictType = unsafe {
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut hashDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
pub static mut sdsReplyDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
pub static mut keylistDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictObjHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictObjKeyCompare
                    as unsafe extern "C" fn(
                        *mut dict,
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> libc::c_int,
            ),
            keyDestructor: Some(
                dictObjectDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            valDestructor: Some(
                dictListDestructor
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut modulesDictType: dictType = unsafe {
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut migrateCacheDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                dictSdsHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                dictSdsKeyCompare
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut stringSetDictType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                distCStrCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                distCStrKeyCaseCompare
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
            valDestructor: None,
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub static mut externalStringType: dictType = unsafe {
    {
        let mut init = dictType {
            hashFunction: Some(
                distCStrCaseHash as unsafe extern "C" fn(*const libc::c_void) -> uint64_t,
            ),
            keyDup: None,
            valDup: None,
            keyCompare: Some(
                distCStrKeyCaseCompare
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
pub static mut sdsHashDictType: dictType = unsafe {
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
                dictVanillaFree
                    as unsafe extern "C" fn(*mut dict, *mut libc::c_void) -> (),
            ),
            expandAllowed: None,
            dictEntryMetadataBytes: None,
        };
        init
    }
};
#[no_mangle]
pub unsafe extern "C" fn htNeedsResize(mut dict: *mut dict) -> libc::c_int {
    let mut size: libc::c_longlong = 0;
    let mut used: libc::c_longlong = 0;
    size = (if (*dict).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
        == -(1 as libc::c_int)
    {
        0 as libc::c_int as libc::c_ulong
    } else {
        (1 as libc::c_int as libc::c_ulong)
            << (*dict).ht_size_exp[0 as libc::c_int as usize] as libc::c_int
    })
        .wrapping_add(
            (if (*dict).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                == -(1 as libc::c_int)
            {
                0 as libc::c_int as libc::c_ulong
            } else {
                (1 as libc::c_int as libc::c_ulong)
                    << (*dict).ht_size_exp[1 as libc::c_int as usize] as libc::c_int
            }),
        ) as libc::c_longlong;
    used = ((*dict).ht_used[0 as libc::c_int as usize])
        .wrapping_add((*dict).ht_used[1 as libc::c_int as usize]) as libc::c_longlong;
    return (size > ((1 as libc::c_int) << 2 as libc::c_int) as libc::c_longlong
        && used * 100 as libc::c_int as libc::c_longlong / size
            < 10 as libc::c_int as libc::c_longlong) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn tryResizeHashTables(mut dbid: libc::c_int) {
    if htNeedsResize((*(server.db).offset(dbid as isize)).dict) != 0 {
        dictResize((*(server.db).offset(dbid as isize)).dict);
    }
    if htNeedsResize((*(server.db).offset(dbid as isize)).expires) != 0 {
        dictResize((*(server.db).offset(dbid as isize)).expires);
    }
}
#[no_mangle]
pub unsafe extern "C" fn incrementallyRehash(mut dbid: libc::c_int) -> libc::c_int {
    if (*(*(server.db).offset(dbid as isize)).dict).rehashidx
        != -(1 as libc::c_int) as libc::c_long
    {
        dictRehashMilliseconds(
            (*(server.db).offset(dbid as isize)).dict,
            1 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    if (*(*(server.db).offset(dbid as isize)).expires).rehashidx
        != -(1 as libc::c_int) as libc::c_long
    {
        dictRehashMilliseconds(
            (*(server.db).offset(dbid as isize)).expires,
            1 as libc::c_int,
        );
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn updateDictResizePolicy() {
    if server.in_fork_child != 0 as libc::c_int {
        dictSetResizeEnabled(DICT_RESIZE_FORBID);
    } else if hasActiveChildProcess() != 0 {
        dictSetResizeEnabled(DICT_RESIZE_AVOID);
    } else {
        dictSetResizeEnabled(DICT_RESIZE_ENABLE);
    };
}
#[no_mangle]
pub unsafe extern "C" fn strChildType(mut type_0: libc::c_int) -> *const libc::c_char {
    match type_0 {
        1 => return b"RDB\0" as *const u8 as *const libc::c_char,
        2 => return b"AOF\0" as *const u8 as *const libc::c_char,
        3 => return b"LDB\0" as *const u8 as *const libc::c_char,
        4 => return b"MODULE\0" as *const u8 as *const libc::c_char,
        _ => return b"Unknown\0" as *const u8 as *const libc::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn hasActiveChildProcess() -> libc::c_int {
    return (server.child_pid != -(1 as libc::c_int)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn resetChildState() {
    server.child_type = 0 as libc::c_int;
    server.child_pid = -(1 as libc::c_int);
    server.stat_current_cow_peak = 0 as libc::c_int as size_t;
    server.stat_current_cow_bytes = 0 as libc::c_int as size_t;
    server.stat_current_cow_updated = 0 as libc::c_int as monotime;
    server.stat_current_save_keys_processed = 0 as libc::c_int as size_t;
    server.stat_module_progress = 0 as libc::c_int as libc::c_double;
    server.stat_current_save_keys_total = 0 as libc::c_int as size_t;
    updateDictResizePolicy();
    closeChildInfoPipe();
    moduleFireServerEvent(
        13 as libc::c_int as uint64_t,
        1 as libc::c_int,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn isMutuallyExclusiveChildType(
    mut type_0: libc::c_int,
) -> libc::c_int {
    return (type_0 == 1 as libc::c_int || type_0 == 2 as libc::c_int
        || type_0 == 4 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isInsideYieldingLongCommand() -> libc::c_int {
    return (scriptIsTimedout() != 0 || server.busy_module_yield_flags != 0)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn allPersistenceDisabled() -> libc::c_int {
    return (server.saveparamslen == 0 as libc::c_int
        && server.aof_state == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn trackInstantaneousMetric(
    mut metric: libc::c_int,
    mut current_reading: libc::c_longlong,
) {
    let mut now: libc::c_longlong = mstime();
    let mut t: libc::c_longlong = now
        - server.inst_metric[metric as usize].last_sample_time;
    let mut ops: libc::c_longlong = current_reading
        - server.inst_metric[metric as usize].last_sample_count;
    let mut ops_sec: libc::c_longlong = 0;
    ops_sec = if t > 0 as libc::c_int as libc::c_longlong {
        ops * 1000 as libc::c_int as libc::c_longlong / t
    } else {
        0 as libc::c_int as libc::c_longlong
    };
    server
        .inst_metric[metric as usize]
        .samples[server.inst_metric[metric as usize].idx as usize] = ops_sec;
    server.inst_metric[metric as usize].idx += 1;
    server.inst_metric[metric as usize].idx %= 16 as libc::c_int;
    server.inst_metric[metric as usize].last_sample_time = now;
    server.inst_metric[metric as usize].last_sample_count = current_reading;
}
#[no_mangle]
pub unsafe extern "C" fn getInstantaneousMetric(
    mut metric: libc::c_int,
) -> libc::c_longlong {
    let mut j: libc::c_int = 0;
    let mut sum: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    j = 0 as libc::c_int;
    while j < 16 as libc::c_int {
        sum += server.inst_metric[metric as usize].samples[j as usize];
        j += 1;
    }
    return sum / 16 as libc::c_int as libc::c_longlong;
}
#[no_mangle]
pub unsafe extern "C" fn clientsCronResizeQueryBuffer(
    mut c: *mut client,
) -> libc::c_int {
    let mut querybuf_size: size_t = sdsalloc((*c).querybuf);
    let mut idletime: time_t = server.unixtime as libc::c_long - (*c).lastinteraction;
    if sdsavail((*c).querybuf)
        > (1024 as libc::c_int * 4 as libc::c_int) as libc::c_ulong
    {
        if idletime > 2 as libc::c_int as libc::c_long {
            (*c).querybuf = sdsRemoveFreeSpace((*c).querybuf);
        } else if querybuf_size
            > (1024 as libc::c_int * 32 as libc::c_int) as libc::c_ulong
            && querybuf_size.wrapping_div(2 as libc::c_int as libc::c_ulong)
                > (*c).querybuf_peak
        {
            let mut resize: size_t = sdslen((*c).querybuf);
            if resize < (*c).querybuf_peak {
                resize = (*c).querybuf_peak;
            }
            if (*c).bulklen != -(1 as libc::c_int) as libc::c_long
                && resize < (*c).bulklen as size_t
            {
                resize = (*c).bulklen as size_t;
            }
            (*c).querybuf = sdsResize((*c).querybuf, resize);
        }
    }
    (*c).querybuf_peak = sdslen((*c).querybuf);
    if (*c).bulklen != -(1 as libc::c_int) as libc::c_long
        && (*c).bulklen as size_t > (*c).querybuf_peak
    {
        (*c).querybuf_peak = (*c).bulklen as size_t;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn clientsCronResizeOutputBuffer(
    mut c: *mut client,
    mut now_ms: mstime_t,
) -> libc::c_int {
    let mut new_buffer_size: size_t = 0 as libc::c_int as size_t;
    let mut oldbuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let buffer_target_shrink_size: size_t = ((*c).buf_usable_size)
        .wrapping_div(2 as libc::c_int as libc::c_ulong);
    let buffer_target_expand_size: size_t = ((*c).buf_usable_size)
        .wrapping_mul(2 as libc::c_int as libc::c_ulong);
    if server.reply_buffer_resizing_enabled == 0 {
        return 0 as libc::c_int;
    }
    if buffer_target_shrink_size >= 1024 as libc::c_int as libc::c_ulong
        && (*c).buf_peak < buffer_target_shrink_size
    {
        new_buffer_size = if 1024 as libc::c_int as libc::c_ulong
            > ((*c).buf_peak).wrapping_add(1 as libc::c_int as libc::c_ulong)
        {
            1024 as libc::c_int as libc::c_ulong
        } else {
            ((*c).buf_peak).wrapping_add(1 as libc::c_int as libc::c_ulong)
        };
        server.stat_reply_buffer_shrinks += 1;
    } else if buffer_target_expand_size
        < (16 as libc::c_int * 1024 as libc::c_int * 2 as libc::c_int) as libc::c_ulong
        && (*c).buf_peak == (*c).buf_usable_size
    {
        new_buffer_size = if ((16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong)
            < buffer_target_expand_size
        {
            (16 as libc::c_int * 1024 as libc::c_int) as libc::c_ulong
        } else {
            buffer_target_expand_size
        };
        server.stat_reply_buffer_expands += 1;
    }
    if server.reply_buffer_peak_reset_time >= 0 as libc::c_int as libc::c_long
        && now_ms - (*c).buf_peak_last_reset_time
            >= server.reply_buffer_peak_reset_time as libc::c_longlong
    {
        (*c).buf_peak = (*c).bufpos as size_t;
        (*c).buf_peak_last_reset_time = now_ms;
    }
    if new_buffer_size != 0 {
        oldbuf = (*c).buf;
        (*c)
            .buf = zmalloc_usable(new_buffer_size, &mut (*c).buf_usable_size)
            as *mut libc::c_char;
        memcpy(
            (*c).buf as *mut libc::c_void,
            oldbuf as *const libc::c_void,
            (*c).bufpos as libc::c_ulong,
        );
        zfree(oldbuf as *mut libc::c_void);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub static mut ClientsPeakMemInput: [size_t; 8] = [
    0 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut ClientsPeakMemOutput: [size_t; 8] = [
    0 as libc::c_int as size_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub unsafe extern "C" fn clientsCronTrackExpansiveClients(
    mut c: *mut client,
    mut time_idx: libc::c_int,
) -> libc::c_int {
    let mut in_usage: size_t = (sdsZmallocSize((*c).querybuf))
        .wrapping_add((*c).argv_len_sum)
        .wrapping_add(
            (if !((*c).argv).is_null() {
                je_malloc_usable_size((*c).argv as *mut libc::c_void)
            } else {
                0 as libc::c_int as libc::c_ulong
            }),
        );
    let mut out_usage: size_t = getClientOutputBufferMemoryUsage(c);
    if in_usage > ClientsPeakMemInput[time_idx as usize] {
        ClientsPeakMemInput[time_idx as usize] = in_usage;
    }
    if out_usage > ClientsPeakMemOutput[time_idx as usize] {
        ClientsPeakMemOutput[time_idx as usize] = out_usage;
    }
    return 0 as libc::c_int;
}
#[inline]
unsafe extern "C" fn getMemUsageBucket(mut mem: size_t) -> *mut clientMemUsageBucket {
    let mut size_in_bits: libc::c_int = 8 as libc::c_int
        * core::mem::size_of::<size_t>() as libc::c_ulong as libc::c_int;
    let mut clz: libc::c_int = if mem > 0 as libc::c_int as libc::c_ulong {
        mem.leading_zeros() as i32
    } else {
        size_in_bits
    };
    let mut bucket_idx: libc::c_int = size_in_bits - clz;
    if bucket_idx > 33 as libc::c_int {
        bucket_idx = 33 as libc::c_int;
    } else if bucket_idx < 15 as libc::c_int {
        bucket_idx = 15 as libc::c_int;
    }
    bucket_idx -= 15 as libc::c_int;
    return &mut *(server.client_mem_usage_buckets).offset(bucket_idx as isize)
        as *mut clientMemUsageBucket;
}
#[no_mangle]
pub unsafe extern "C" fn updateClientMemoryUsage(mut c: *mut client) {
    let mut mem: size_t = getClientMemoryUsage(c, 0 as *mut size_t);
    let mut type_0: libc::c_int = getClientType(c);
    server
        .stat_clients_type_memory[(*c).last_memory_type
        as usize] = (server.stat_clients_type_memory[(*c).last_memory_type as usize]
        as libc::c_ulong)
        .wrapping_sub((*c).last_memory_usage) as size_t as size_t;
    server
        .stat_clients_type_memory[type_0
        as usize] = (server.stat_clients_type_memory[type_0 as usize] as libc::c_ulong)
        .wrapping_add(mem) as size_t as size_t;
    (*c).last_memory_type = type_0;
    (*c).last_memory_usage = mem;
}
#[no_mangle]
pub unsafe extern "C" fn clientEvictionAllowed(mut c: *mut client) -> libc::c_int {
    if server.maxmemory_clients == 0 as libc::c_int as libc::c_long
        || (*c).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 43 as libc::c_int != 0
    {
        return 0 as libc::c_int;
    }
    let mut type_0: libc::c_int = getClientType(c);
    return (type_0 == 0 as libc::c_int || type_0 == 2 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn removeClientFromMemUsageBucket(
    mut c: *mut client,
    mut allow_eviction: libc::c_int,
) {
    if !((*c).mem_usage_bucket).is_null() {
        (*(*c).mem_usage_bucket)
            .mem_usage_sum = ((*(*c).mem_usage_bucket).mem_usage_sum as libc::c_ulong)
            .wrapping_sub((*c).last_memory_usage) as size_t as size_t;
        if allow_eviction == 0 {
            listDelNode((*(*c).mem_usage_bucket).clients, (*c).mem_usage_bucket_node);
            (*c).mem_usage_bucket = 0 as *mut clientMemUsageBucket;
            (*c).mem_usage_bucket_node = 0 as *mut listNode;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateClientMemUsageAndBucket(
    mut c: *mut client,
) -> libc::c_int {
    if io_threads_op == 0 as libc::c_int {} else {
        _serverAssert(
            b"io_threads_op == IO_THREADS_OP_IDLE\0" as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            887 as libc::c_int,
        );
        unreachable!();
    };
    let mut allow_eviction: libc::c_int = clientEvictionAllowed(c);
    removeClientFromMemUsageBucket(c, allow_eviction);
    if allow_eviction == 0 {
        return 0 as libc::c_int;
    }
    updateClientMemoryUsage(c);
    let mut bucket: *mut clientMemUsageBucket = getMemUsageBucket(
        (*c).last_memory_usage,
    );
    (*bucket)
        .mem_usage_sum = ((*bucket).mem_usage_sum as libc::c_ulong)
        .wrapping_add((*c).last_memory_usage) as size_t as size_t;
    if bucket != (*c).mem_usage_bucket {
        if !((*c).mem_usage_bucket).is_null() {
            listDelNode((*(*c).mem_usage_bucket).clients, (*c).mem_usage_bucket_node);
        }
        (*c).mem_usage_bucket = bucket;
        listAddNodeTail((*bucket).clients, c as *mut libc::c_void);
        (*c).mem_usage_bucket_node = (*(*bucket).clients).tail;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getExpansiveClientsInfo(
    mut in_usage: *mut size_t,
    mut out_usage: *mut size_t,
) {
    let mut i: size_t = 0 as libc::c_int as size_t;
    let mut o: size_t = 0 as libc::c_int as size_t;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 8 as libc::c_int {
        if ClientsPeakMemInput[j as usize] > i {
            i = ClientsPeakMemInput[j as usize];
        }
        if ClientsPeakMemOutput[j as usize] > o {
            o = ClientsPeakMemOutput[j as usize];
        }
        j += 1;
    }
    *in_usage = i;
    *out_usage = o;
}
#[no_mangle]
pub unsafe extern "C" fn clientsCron() {
    let mut numclients: libc::c_int = (*server.clients).len as libc::c_int;
    let mut iterations: libc::c_int = numclients / server.hz;
    let mut now: mstime_t = mstime();
    if iterations < 5 as libc::c_int {
        iterations = if numclients < 5 as libc::c_int {
            numclients
        } else {
            5 as libc::c_int
        };
    }
    let mut curr_peak_mem_usage_slot: libc::c_int = server.unixtime % 8 as libc::c_int;
    let mut zeroidx: libc::c_int = (curr_peak_mem_usage_slot + 1 as libc::c_int)
        % 8 as libc::c_int;
    ClientsPeakMemInput[zeroidx as usize] = 0 as libc::c_int as size_t;
    ClientsPeakMemOutput[zeroidx as usize] = 0 as libc::c_int as size_t;
    while (*server.clients).len != 0
        && {
            let fresh0 = iterations;
            iterations = iterations - 1;
            fresh0 != 0
        }
    {
        let mut c: *mut client = 0 as *mut client;
        let mut head: *mut listNode = 0 as *mut listNode;
        listRotateTailToHead(server.clients);
        head = (*server.clients).head;
        c = (*head).value as *mut client;
        if clientsCronHandleTimeout(c, now) != 0 {
            continue;
        }
        if clientsCronResizeQueryBuffer(c) != 0 {
            continue;
        }
        if clientsCronResizeOutputBuffer(c, now) != 0 {
            continue;
        }
        if clientsCronTrackExpansiveClients(c, curr_peak_mem_usage_slot) != 0 {
            continue;
        }
        if updateClientMemUsageAndBucket(c) == 0 {
            updateClientMemoryUsage(c);
        }
        closeClientOnOutputBufferLimitReached(c, 0 as libc::c_int) != 0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn databasesCron() {
    if server.active_expire_enabled != 0 {
        if iAmMaster() != 0 {
            activeExpireCycle(0 as libc::c_int);
        } else {
            expireSlaveKeys();
        }
    }
    activeDefragCycle();
    if hasActiveChildProcess() == 0 {
        static mut resize_db: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        static mut rehash_db: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut dbs_per_call: libc::c_int = 16 as libc::c_int;
        let mut j: libc::c_int = 0;
        if dbs_per_call > server.dbnum {
            dbs_per_call = server.dbnum;
        }
        j = 0 as libc::c_int;
        while j < dbs_per_call {
            tryResizeHashTables(
                resize_db.wrapping_rem(server.dbnum as libc::c_uint) as libc::c_int,
            );
            resize_db = resize_db.wrapping_add(1);
            j += 1;
        }
        if server.activerehashing != 0 {
            j = 0 as libc::c_int;
            while j < dbs_per_call {
                let mut work_done: libc::c_int = incrementallyRehash(
                    rehash_db as libc::c_int,
                );
                if work_done != 0 {
                    break;
                }
                rehash_db = rehash_db.wrapping_add(1);
                rehash_db = rehash_db.wrapping_rem(server.dbnum as libc::c_uint);
                j += 1;
            }
        }
    }
}
#[inline]
unsafe extern "C" fn updateCachedTimeWithUs(
    mut update_daylight_info: libc::c_int,
    ustime_0: libc::c_longlong,
) {
    server.ustime = ustime_0;
    server.mstime = server.ustime / 1000 as libc::c_int as libc::c_longlong;
    let mut unixtime: time_t = (server.mstime / 1000 as libc::c_int as libc::c_longlong)
        as time_t;
    core::intrinsics::atomic_store_relaxed(
        &mut server.unixtime,
        unixtime as libc::c_int,
    );
    if update_daylight_info != 0 {
        let mut tm: tm = tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
            tm_gmtoff: 0,
            tm_zone: 0 as *const libc::c_char,
        };
        let mut ut: time_t = server.unixtime as time_t;
        localtime_r(&mut ut, &mut tm);
        server.daylight_active = tm.tm_isdst;
    }
}
#[no_mangle]
pub unsafe extern "C" fn updateCachedTime(mut update_daylight_info: libc::c_int) {
    let us: libc::c_longlong = ustime();
    updateCachedTimeWithUs(update_daylight_info, us);
}
#[no_mangle]
pub unsafe extern "C" fn checkChildrenDone() {
    let mut statloc: libc::c_int = 0 as libc::c_int;
    let mut pid: pid_t = 0;
    pid = waitpid(-(1 as libc::c_int), &mut statloc, 1 as libc::c_int);
    if pid != 0 as libc::c_int {
        let mut exitcode: libc::c_int = if statloc & 0x7f as libc::c_int
            == 0 as libc::c_int
        {
            (statloc & 0xff00 as libc::c_int) >> 8 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
        let mut bysignal: libc::c_int = 0 as libc::c_int;
        if ((statloc & 0x7f as libc::c_int) + 1 as libc::c_int) as libc::c_schar
            as libc::c_int >> 1 as libc::c_int > 0 as libc::c_int
        {
            bysignal = statloc & 0x7f as libc::c_int;
        }
        if exitcode == 255 as libc::c_int {
            bysignal = 10 as libc::c_int;
            exitcode = 1 as libc::c_int;
        }
        if pid == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"waitpid() returned an error: %s. child_type: %s, child_pid = %d\0"
                        as *const u8 as *const libc::c_char,
                    strerror(*__errno_location()),
                    strChildType(server.child_type),
                    server.child_pid,
                );
            }
        } else if pid == server.child_pid {
            if server.child_type == 1 as libc::c_int {
                backgroundSaveDoneHandler(exitcode, bysignal);
            } else if server.child_type == 2 as libc::c_int {
                backgroundRewriteDoneHandler(exitcode, bysignal);
            } else if server.child_type == 4 as libc::c_int {
                ModuleForkDoneHandler(exitcode, bysignal);
            } else {
                _serverPanic(
                    b"server.c\0" as *const u8 as *const libc::c_char,
                    1129 as libc::c_int,
                    b"Unknown child type %d for child pid %d\0" as *const u8
                        as *const libc::c_char,
                    server.child_type,
                    server.child_pid,
                );
                unreachable!();
                exit(1 as libc::c_int);
            }
            if bysignal == 0 && exitcode == 0 as libc::c_int {
                receiveChildInfo();
            }
            resetChildState();
        } else if ldbRemoveChild(pid) == 0 {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Warning, detected child with unmatched pid: %ld\0" as *const u8
                        as *const libc::c_char,
                    pid as libc::c_long,
                );
            }
        }
        replicationStartPendingFork();
    }
}
#[no_mangle]
pub unsafe extern "C" fn cronUpdateMemoryStats() {
    if zmalloc_used_memory() > server.stat_peak_memory {
        server.stat_peak_memory = zmalloc_used_memory();
    }
    if 100 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (100 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        server.cron_malloc_stats.process_rss = zmalloc_get_rss();
        server.cron_malloc_stats.zmalloc_used = zmalloc_used_memory();
        zmalloc_get_allocator_info(
            &mut server.cron_malloc_stats.allocator_allocated,
            &mut server.cron_malloc_stats.allocator_active,
            &mut server.cron_malloc_stats.allocator_resident,
        );
        if server.cron_malloc_stats.allocator_resident == 0 {
            let mut lua_memory: size_t = evalMemory();
            server
                .cron_malloc_stats
                .allocator_resident = (server.cron_malloc_stats.process_rss)
                .wrapping_sub(lua_memory);
        }
        if server.cron_malloc_stats.allocator_active == 0 {
            server
                .cron_malloc_stats
                .allocator_active = server.cron_malloc_stats.allocator_resident;
        }
        if server.cron_malloc_stats.allocator_allocated == 0 {
            server
                .cron_malloc_stats
                .allocator_allocated = server.cron_malloc_stats.zmalloc_used;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn serverCron(
    mut eventLoop: *mut aeEventLoop,
    mut id: libc::c_longlong,
    mut clientData: *mut libc::c_void,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if server.watchdog_period != 0 {
        watchdogScheduleSignal(server.watchdog_period);
    }
    updateCachedTime(1 as libc::c_int);
    server.hz = server.config_hz;
    if server.dynamic_hz != 0 {
        while ((*server.clients).len).wrapping_div(server.hz as libc::c_ulong)
            > 200 as libc::c_int as libc::c_ulong
        {
            server.hz *= 2 as libc::c_int;
            if !(server.hz > 500 as libc::c_int) {
                continue;
            }
            server.hz = 500 as libc::c_int;
            break;
        }
    }
    if server.pause_cron != 0 {
        return 1000 as libc::c_int / server.hz;
    }
    if 100 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (100 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        let mut stat_net_input_bytes: libc::c_longlong = 0;
        let mut stat_net_output_bytes: libc::c_longlong = 0;
        let mut stat_net_repl_input_bytes: libc::c_longlong = 0;
        let mut stat_net_repl_output_bytes: libc::c_longlong = 0;
        stat_net_input_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_input_bytes,
        );
        stat_net_output_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_output_bytes,
        );
        stat_net_repl_input_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_repl_input_bytes,
        );
        stat_net_repl_output_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_repl_output_bytes,
        );
        trackInstantaneousMetric(0 as libc::c_int, server.stat_numcommands);
        trackInstantaneousMetric(
            1 as libc::c_int,
            stat_net_input_bytes + stat_net_repl_input_bytes,
        );
        trackInstantaneousMetric(
            2 as libc::c_int,
            stat_net_output_bytes + stat_net_repl_output_bytes,
        );
        trackInstantaneousMetric(3 as libc::c_int, stat_net_repl_input_bytes);
        trackInstantaneousMetric(4 as libc::c_int, stat_net_repl_output_bytes);
    }
    let mut lruclock: libc::c_uint = getLRUClock();
    core::intrinsics::atomic_store_relaxed(&mut server.lruclock, lruclock);
    cronUpdateMemoryStats();
    if server.shutdown_asap != 0 && isShutdownInitiated() == 0 {
        let mut shutdownFlags: libc::c_int = 0 as libc::c_int;
        if server.last_sig_received == 2 as libc::c_int && server.shutdown_on_sigint != 0
        {
            shutdownFlags = server.shutdown_on_sigint;
        } else if server.last_sig_received == 15 as libc::c_int
            && server.shutdown_on_sigterm != 0
        {
            shutdownFlags = server.shutdown_on_sigterm;
        }
        if prepareForShutdown(shutdownFlags) == 0 as libc::c_int {
            exit(0 as libc::c_int);
        }
    } else if isShutdownInitiated() != 0 {
        if server.mstime >= server.shutdown_mstime || isReadyToShutdown() != 0 {
            if finishShutdown() == 0 as libc::c_int {
                exit(0 as libc::c_int);
            }
        }
    }
    if server.verbosity <= 1 as libc::c_int {
        if 5000 as libc::c_int <= 1000 as libc::c_int / server.hz
            || server.cronloops
                % (5000 as libc::c_int / (1000 as libc::c_int / server.hz)) == 0
        {
            j = 0 as libc::c_int;
            while j < server.dbnum {
                let mut size: libc::c_longlong = 0;
                let mut used: libc::c_longlong = 0;
                let mut vkeys: libc::c_longlong = 0;
                size = (if (*(*(server.db).offset(j as isize)).dict)
                    .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                    == -(1 as libc::c_int)
                {
                    0 as libc::c_int as libc::c_ulong
                } else {
                    (1 as libc::c_int as libc::c_ulong)
                        << (*(*(server.db).offset(j as isize)).dict)
                            .ht_size_exp[0 as libc::c_int as usize] as libc::c_int
                })
                    .wrapping_add(
                        (if (*(*(server.db).offset(j as isize)).dict)
                            .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                            == -(1 as libc::c_int)
                        {
                            0 as libc::c_int as libc::c_ulong
                        } else {
                            (1 as libc::c_int as libc::c_ulong)
                                << (*(*(server.db).offset(j as isize)).dict)
                                    .ht_size_exp[1 as libc::c_int as usize] as libc::c_int
                        }),
                    ) as libc::c_longlong;
                used = ((*(*(server.db).offset(j as isize)).dict)
                    .ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*(*(server.db).offset(j as isize)).dict)
                            .ht_used[1 as libc::c_int as usize],
                    ) as libc::c_longlong;
                vkeys = ((*(*(server.db).offset(j as isize)).expires)
                    .ht_used[0 as libc::c_int as usize])
                    .wrapping_add(
                        (*(*(server.db).offset(j as isize)).expires)
                            .ht_used[1 as libc::c_int as usize],
                    ) as libc::c_longlong;
                if used != 0 || vkeys != 0 {
                    if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            1 as libc::c_int,
                            b"DB %d: %lld keys (%lld volatile) in %lld slots HT.\0"
                                as *const u8 as *const libc::c_char,
                            j,
                            used,
                            vkeys,
                            size,
                        );
                    }
                }
                j += 1;
            }
        }
    }
    if server.sentinel_mode == 0 {
        if 5000 as libc::c_int <= 1000 as libc::c_int / server.hz
            || server.cronloops
                % (5000 as libc::c_int / (1000 as libc::c_int / server.hz)) == 0
        {
            if !((0 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    0 as libc::c_int,
                    b"%lu clients connected (%lu replicas), %zu bytes in use\0"
                        as *const u8 as *const libc::c_char,
                    ((*server.clients).len).wrapping_sub((*server.slaves).len),
                    (*server.slaves).len,
                    zmalloc_used_memory(),
                );
            }
        }
    }
    clientsCron();
    databasesCron();
    if hasActiveChildProcess() == 0 && server.aof_rewrite_scheduled != 0
        && aofRewriteLimited() == 0
    {
        rewriteAppendOnlyFileBackground();
    }
    if hasActiveChildProcess() != 0 || ldbPendingChildren() != 0 {
        if 1000 as libc::c_int <= 1000 as libc::c_int / server.hz
            || server.cronloops
                % (1000 as libc::c_int / (1000 as libc::c_int / server.hz)) == 0
        {
            receiveChildInfo();
        }
        checkChildrenDone();
    } else {
        j = 0 as libc::c_int;
        while j < server.saveparamslen {
            let mut sp: *mut saveparam = (server.saveparams).offset(j as isize);
            if server.dirty >= (*sp).changes as libc::c_longlong
                && server.unixtime as libc::c_long - server.lastsave > (*sp).seconds
                && (server.unixtime as libc::c_long - server.lastbgsave_try
                    > 5 as libc::c_int as libc::c_long
                    || server.lastbgsave_status == 0 as libc::c_int)
            {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"%d changes in %d seconds. Saving...\0" as *const u8
                            as *const libc::c_char,
                        (*sp).changes,
                        (*sp).seconds as libc::c_int,
                    );
                }
                let mut rsi: rdbSaveInfo = rdbSaveInfo {
                    repl_stream_db: 0,
                    repl_id_is_set: 0,
                    repl_id: [0; 41],
                    repl_offset: 0,
                };
                let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
                rsiptr = rdbPopulateSaveInfo(&mut rsi);
                rdbSaveBackground(0 as libc::c_int, server.rdb_filename, rsiptr);
                break;
            } else {
                j += 1;
            }
        }
        if server.aof_state == 1 as libc::c_int && hasActiveChildProcess() == 0
            && server.aof_rewrite_perc != 0
            && server.aof_current_size > server.aof_rewrite_min_size
        {
            let mut base: libc::c_longlong = (if server.aof_rewrite_base_size != 0 {
                server.aof_rewrite_base_size
            } else {
                1 as libc::c_int as libc::c_long
            }) as libc::c_longlong;
            let mut growth: libc::c_longlong = (server.aof_current_size
                * 100 as libc::c_int as libc::c_long) as libc::c_longlong / base
                - 100 as libc::c_int as libc::c_longlong;
            if growth >= server.aof_rewrite_perc as libc::c_longlong
                && aofRewriteLimited() == 0
            {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Starting automatic rewriting of AOF on %lld%% growth\0"
                            as *const u8 as *const libc::c_char,
                        growth,
                    );
                }
                rewriteAppendOnlyFileBackground();
            }
        }
    }
    updateDictResizePolicy();
    if (server.aof_state == 1 as libc::c_int || server.aof_state == 2 as libc::c_int)
        && server.aof_flush_postponed_start != 0
    {
        flushAppendOnlyFile(0 as libc::c_int);
    }
    if 1000 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (1000 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        if (server.aof_state == 1 as libc::c_int || server.aof_state == 2 as libc::c_int)
            && server.aof_last_write_status == -(1 as libc::c_int)
        {
            flushAppendOnlyFile(0 as libc::c_int);
        }
    }
    checkClientPauseTimeoutAndReturnIfPaused();
    if server.failover_state != NO_FAILOVER as libc::c_int {
        if 100 as libc::c_int <= 1000 as libc::c_int / server.hz
            || server.cronloops
                % (100 as libc::c_int / (1000 as libc::c_int / server.hz)) == 0
        {
            replicationCron();
        }
    } else if 1000 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (1000 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        replicationCron();
    }
    if 100 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (100 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        if server.cluster_enabled != 0 {
            clusterCron();
        }
    }
    if server.sentinel_mode != 0 {
        sentinelTimer();
    }
    if 1000 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (1000 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        migrateCloseTimedoutSockets();
    }
    stopThreadedIOIfNeeded();
    if server.tracking_clients != 0 {
        trackingLimitUsedSlots();
    }
    if hasActiveChildProcess() == 0 && server.rdb_bgsave_scheduled != 0
        && (server.unixtime as libc::c_long - server.lastbgsave_try
            > 5 as libc::c_int as libc::c_long
            || server.lastbgsave_status == 0 as libc::c_int)
    {
        let mut rsi_0: rdbSaveInfo = rdbSaveInfo {
            repl_stream_db: 0,
            repl_id_is_set: 0,
            repl_id: [0; 41],
            repl_offset: 0,
        };
        let mut rsiptr_0: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
        rsiptr_0 = rdbPopulateSaveInfo(&mut rsi_0);
        if rdbSaveBackground(0 as libc::c_int, server.rdb_filename, rsiptr_0)
            == 0 as libc::c_int
        {
            server.rdb_bgsave_scheduled = 0 as libc::c_int;
        }
    }
    if 100 as libc::c_int <= 1000 as libc::c_int / server.hz
        || server.cronloops % (100 as libc::c_int / (1000 as libc::c_int / server.hz))
            == 0
    {
        if moduleCount() != 0 {
            modulesCron();
        }
    }
    let mut ei: RedisModuleCronLoopV1 = {
        let mut init = RedisModuleCronLoopInfo {
            version: 1 as libc::c_int as uint64_t,
            hz: server.hz,
        };
        init
    };
    moduleFireServerEvent(
        8 as libc::c_int as uint64_t,
        0 as libc::c_int,
        &mut ei as *mut RedisModuleCronLoopV1 as *mut libc::c_void,
    );
    server.cronloops += 1;
    return 1000 as libc::c_int / server.hz;
}
#[no_mangle]
pub unsafe extern "C" fn blockingOperationStarts() {
    let fresh1 = server.blocking_op_nesting;
    server.blocking_op_nesting = (server.blocking_op_nesting).wrapping_add(1);
    if fresh1 == 0 {
        updateCachedTime(0 as libc::c_int);
        server.blocked_last_cron = server.mstime;
    }
}
#[no_mangle]
pub unsafe extern "C" fn blockingOperationEnds() {
    server.blocking_op_nesting = (server.blocking_op_nesting).wrapping_sub(1);
    if server.blocking_op_nesting == 0 {
        server.blocked_last_cron = 0 as libc::c_int as libc::c_longlong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn whileBlockedCron() {
    if server.blocked_last_cron != 0 {} else {
        _serverAssert(
            b"server.blocked_last_cron\0" as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            1489 as libc::c_int,
        );
        unreachable!();
    };
    if server.blocked_last_cron >= server.mstime {
        return;
    }
    let mut latency: mstime_t = 0;
    if server.latency_monitor_threshold != 0 {
        latency = mstime();
    } else {
        latency = 0 as libc::c_int as mstime_t;
    }
    let mut hz_ms: libc::c_long = (1000 as libc::c_int / server.hz) as libc::c_long;
    while server.blocked_last_cron < server.mstime {
        activeDefragCycle();
        server.blocked_last_cron += hz_ms as libc::c_longlong;
        server.cronloops += 1;
    }
    if server.loading != 0 {
        cronUpdateMemoryStats();
    }
    if server.latency_monitor_threshold != 0 {
        latency = mstime() - latency;
    }
    if server.latency_monitor_threshold != 0
        && latency >= server.latency_monitor_threshold
    {
        latencyAddSample(
            b"while-blocked-cron\0" as *const u8 as *const libc::c_char,
            latency,
        );
    }
    if server.shutdown_asap != 0 && server.loading != 0 {
        if prepareForShutdown(2 as libc::c_int) == 0 as libc::c_int {
            exit(0 as libc::c_int);
        }
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"SIGTERM received but errors trying to shut down the server, check the logs for more information\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        core::ptr::write_volatile(
            &mut server.shutdown_asap as *mut sig_atomic_t,
            0 as libc::c_int,
        );
        server.last_sig_received = 0 as libc::c_int;
    }
}
unsafe extern "C" fn sendGetackToReplicas() {
    let mut argv: [*mut robj; 3] = [0 as *mut robj; 3];
    argv[0 as libc::c_int as usize] = shared.replconf;
    argv[1 as libc::c_int as usize] = shared.getack;
    argv[2 as libc::c_int as usize] = shared.special_asterick;
    replicationFeedSlaves(
        server.slaves,
        server.slaveseldb,
        argv.as_mut_ptr(),
        3 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn beforeSleep(mut eventLoop: *mut aeEventLoop) {
    let mut zmalloc_used: size_t = zmalloc_used_memory();
    if zmalloc_used > server.stat_peak_memory {
        server.stat_peak_memory = zmalloc_used;
    }
    if ProcessingEventsWhileBlocked != 0 {
        let mut processed: uint64_t = 0 as libc::c_int as uint64_t;
        processed = (processed as libc::c_ulong)
            .wrapping_add(handleClientsWithPendingReadsUsingThreads() as libc::c_ulong)
            as uint64_t as uint64_t;
        processed = (processed as libc::c_ulong)
            .wrapping_add(tlsProcessPendingData() as libc::c_ulong) as uint64_t
            as uint64_t;
        if server.aof_state == 1 as libc::c_int || server.aof_state == 2 as libc::c_int {
            flushAppendOnlyFile(0 as libc::c_int);
        }
        processed = (processed as libc::c_ulong)
            .wrapping_add(handleClientsWithPendingWrites() as libc::c_ulong) as uint64_t
            as uint64_t;
        processed = (processed as libc::c_ulong)
            .wrapping_add(freeClientsInAsyncFreeQueue() as libc::c_ulong) as uint64_t
            as uint64_t;
        server
            .events_processed_while_blocked = (server.events_processed_while_blocked
            as libc::c_ulonglong)
            .wrapping_add(processed as libc::c_ulonglong) as libc::c_longlong
            as libc::c_longlong;
        return;
    }
    handleBlockedClientsTimeout();
    handleClientsWithPendingReadsUsingThreads();
    tlsProcessPendingData();
    aeSetDontWait(server.el, tlsHasPendingData());
    if server.cluster_enabled != 0 {
        clusterBeforeSleep();
    }
    if server.active_expire_enabled != 0 && (server.masterhost).is_null() {
        activeExpireCycle(1 as libc::c_int);
    }
    if (*server.clients_waiting_acks).len != 0 {
        processClientsWaitingReplicas();
    }
    if moduleCount() != 0 {
        moduleFireServerEvent(
            15 as libc::c_int as uint64_t,
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
        moduleHandleBlockedClients();
    }
    if (*server.unblocked_clients).len != 0 {
        processUnblockedClients();
    }
    if server.get_ack_from_slaves != 0 && checkClientPauseTimeoutAndReturnIfPaused() == 0
    {
        sendGetackToReplicas();
        server.get_ack_from_slaves = 0 as libc::c_int;
    }
    updateFailoverStatus();
    if (*server.tracking_pending_keys).len == 0 as libc::c_int as libc::c_ulong {} else {
        _serverAssert(
            b"listLength(server.tracking_pending_keys) == 0\0" as *const u8
                as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            1647 as libc::c_int,
        );
        unreachable!();
    };
    trackingBroadcastInvalidationMessages();
    handleClientsBlockedOnKeys();
    if server.aof_state == 1 as libc::c_int || server.aof_state == 2 as libc::c_int {
        flushAppendOnlyFile(0 as libc::c_int);
    }
    handleClientsWithPendingWritesUsingThreads();
    freeClientsInAsyncFreeQueue();
    if !(server.repl_backlog).is_null() {
        incrementalTrimReplicationBacklog(
            (10 as libc::c_int * 64 as libc::c_int) as size_t,
        );
    }
    evictClients();
    if moduleCount() != 0 {
        moduleReleaseGIL();
    }
}
#[no_mangle]
pub unsafe extern "C" fn afterSleep(mut eventLoop: *mut aeEventLoop) {
    if ProcessingEventsWhileBlocked == 0 {
        if moduleCount() != 0 {
            let mut latency: mstime_t = 0;
            if server.latency_monitor_threshold != 0 {
                latency = mstime();
            } else {
                latency = 0 as libc::c_int as mstime_t;
            }
            moduleAcquireGIL();
            moduleFireServerEvent(
                15 as libc::c_int as uint64_t,
                1 as libc::c_int,
                0 as *mut libc::c_void,
            );
            if server.latency_monitor_threshold != 0 {
                latency = mstime() - latency;
            }
            if server.latency_monitor_threshold != 0
                && latency >= server.latency_monitor_threshold
            {
                latencyAddSample(
                    b"module-acquire-GIL\0" as *const u8 as *const libc::c_char,
                    latency,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn createSharedObjects() {
    let mut j: libc::c_int = 0;
    shared
        .crlf = createObject(
        0 as libc::c_int,
        sdsnew(b"\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .ok = createObject(
        0 as libc::c_int,
        sdsnew(b"+OK\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .emptybulk = createObject(
        0 as libc::c_int,
        sdsnew(b"$0\r\n\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .czero = createObject(
        0 as libc::c_int,
        sdsnew(b":0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .cone = createObject(
        0 as libc::c_int,
        sdsnew(b":1\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .emptyarray = createObject(
        0 as libc::c_int,
        sdsnew(b"*0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .pong = createObject(
        0 as libc::c_int,
        sdsnew(b"+PONG\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .queued = createObject(
        0 as libc::c_int,
        sdsnew(b"+QUEUED\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .emptyscan = createObject(
        0 as libc::c_int,
        sdsnew(b"*2\r\n$1\r\n0\r\n*0\r\n\0" as *const u8 as *const libc::c_char)
            as *mut libc::c_void,
    );
    shared
        .space = createObject(
        0 as libc::c_int,
        sdsnew(b" \0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .plus = createObject(
        0 as libc::c_int,
        sdsnew(b"+\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .wrongtypeerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-WRONGTYPE Operation against a key holding the wrong kind of value\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .err = createObject(
        0 as libc::c_int,
        sdsnew(b"-ERR\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .nokeyerr = createObject(
        0 as libc::c_int,
        sdsnew(b"-ERR no such key\r\n\0" as *const u8 as *const libc::c_char)
            as *mut libc::c_void,
    );
    shared
        .syntaxerr = createObject(
        0 as libc::c_int,
        sdsnew(b"-ERR syntax error\r\n\0" as *const u8 as *const libc::c_char)
            as *mut libc::c_void,
    );
    shared
        .sameobjecterr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-ERR source and destination objects are the same\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .outofrangeerr = createObject(
        0 as libc::c_int,
        sdsnew(b"-ERR index out of range\r\n\0" as *const u8 as *const libc::c_char)
            as *mut libc::c_void,
    );
    shared
        .noscripterr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-NOSCRIPT No matching script. Please use EVAL.\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .loadingerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-LOADING Redis is loading the dataset in memory\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .slowevalerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-BUSY Redis is busy running a script. You can only call SCRIPT KILL or SHUTDOWN NOSAVE.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .slowscripterr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-BUSY Redis is busy running a script. You can only call FUNCTION KILL or SHUTDOWN NOSAVE.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .slowmoduleerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-BUSY Redis is busy running a module command.\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .masterdownerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-MASTERDOWN Link with MASTER is down and replica-serve-stale-data is set to 'no'.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .bgsaveerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-MISCONF Redis is configured to save RDB snapshots, but it's currently unable to persist to disk. Commands that may modify the data set are disabled, because this instance is configured to report errors during writes if RDB snapshotting fails (stop-writes-on-bgsave-error option). Please check the Redis logs for details about the RDB error.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .roslaveerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-READONLY You can't write against a read only replica.\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .noautherr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-NOAUTH Authentication required.\r\n\0" as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .oomerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-OOM command not allowed when used memory > 'maxmemory'.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .execaborterr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-EXECABORT Transaction discarded because of previous errors.\r\n\0"
                as *const u8 as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .noreplicaserr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-NOREPLICAS Not enough good replicas to write.\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared
        .busykeyerr = createObject(
        0 as libc::c_int,
        sdsnew(
            b"-BUSYKEY Target key name already exists.\r\n\0" as *const u8
                as *const libc::c_char,
        ) as *mut libc::c_void,
    );
    shared.null[0 as libc::c_int as usize] = 0 as *mut robj;
    shared.null[1 as libc::c_int as usize] = 0 as *mut robj;
    shared
        .null[2 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"$-1\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .null[3 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"_\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared.nullarray[0 as libc::c_int as usize] = 0 as *mut robj;
    shared.nullarray[1 as libc::c_int as usize] = 0 as *mut robj;
    shared
        .nullarray[2 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"*-1\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .nullarray[3 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"_\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared.emptymap[0 as libc::c_int as usize] = 0 as *mut robj;
    shared.emptymap[1 as libc::c_int as usize] = 0 as *mut robj;
    shared
        .emptymap[2 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"*0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .emptymap[3 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"%0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared.emptyset[0 as libc::c_int as usize] = 0 as *mut robj;
    shared.emptyset[1 as libc::c_int as usize] = 0 as *mut robj;
    shared
        .emptyset[2 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"*0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    shared
        .emptyset[3 as libc::c_int
        as usize] = createObject(
        0 as libc::c_int,
        sdsnew(b"~0\r\n\0" as *const u8 as *const libc::c_char) as *mut libc::c_void,
    );
    j = 0 as libc::c_int;
    while j < 10 as libc::c_int {
        let mut dictid_str: [libc::c_char; 64] = [0; 64];
        let mut dictid_len: libc::c_int = 0;
        dictid_len = ll2string(
            dictid_str.as_mut_ptr(),
            core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
            j as libc::c_longlong,
        );
        shared
            .select[j
            as usize] = createObject(
            0 as libc::c_int,
            sdscatprintf(
                sdsempty(),
                b"*2\r\n$6\r\nSELECT\r\n$%d\r\n%s\r\n\0" as *const u8
                    as *const libc::c_char,
                dictid_len,
                dictid_str.as_mut_ptr(),
            ) as *mut libc::c_void,
        );
        j += 1;
    }
    shared
        .messagebulk = createStringObject(
        b"$7\r\nmessage\r\n\0" as *const u8 as *const libc::c_char,
        13 as libc::c_int as size_t,
    );
    shared
        .pmessagebulk = createStringObject(
        b"$8\r\npmessage\r\n\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int as size_t,
    );
    shared
        .subscribebulk = createStringObject(
        b"$9\r\nsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        15 as libc::c_int as size_t,
    );
    shared
        .unsubscribebulk = createStringObject(
        b"$11\r\nunsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        18 as libc::c_int as size_t,
    );
    shared
        .ssubscribebulk = createStringObject(
        b"$10\r\nssubscribe\r\n\0" as *const u8 as *const libc::c_char,
        17 as libc::c_int as size_t,
    );
    shared
        .sunsubscribebulk = createStringObject(
        b"$12\r\nsunsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        19 as libc::c_int as size_t,
    );
    shared
        .smessagebulk = createStringObject(
        b"$8\r\nsmessage\r\n\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int as size_t,
    );
    shared
        .psubscribebulk = createStringObject(
        b"$10\r\npsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        17 as libc::c_int as size_t,
    );
    shared
        .punsubscribebulk = createStringObject(
        b"$12\r\npunsubscribe\r\n\0" as *const u8 as *const libc::c_char,
        19 as libc::c_int as size_t,
    );
    shared
        .del = createStringObject(
        b"DEL\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    );
    shared
        .unlink = createStringObject(
        b"UNLINK\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .rpop = createStringObject(
        b"RPOP\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .lpop = createStringObject(
        b"LPOP\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .lpush = createStringObject(
        b"LPUSH\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .rpoplpush = createStringObject(
        b"RPOPLPUSH\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as size_t,
    );
    shared
        .lmove = createStringObject(
        b"LMOVE\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .blmove = createStringObject(
        b"BLMOVE\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .zpopmin = createStringObject(
        b"ZPOPMIN\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .zpopmax = createStringObject(
        b"ZPOPMAX\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .multi = createStringObject(
        b"MULTI\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .exec = createStringObject(
        b"EXEC\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .hset = createStringObject(
        b"HSET\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .srem = createStringObject(
        b"SREM\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .xgroup = createStringObject(
        b"XGROUP\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .xclaim = createStringObject(
        b"XCLAIM\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .script = createStringObject(
        b"SCRIPT\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .replconf = createStringObject(
        b"REPLCONF\0" as *const u8 as *const libc::c_char,
        8 as libc::c_int as size_t,
    );
    shared
        .pexpireat = createStringObject(
        b"PEXPIREAT\0" as *const u8 as *const libc::c_char,
        9 as libc::c_int as size_t,
    );
    shared
        .pexpire = createStringObject(
        b"PEXPIRE\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .persist = createStringObject(
        b"PERSIST\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .set = createStringObject(
        b"SET\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as size_t,
    );
    shared
        .eval = createStringObject(
        b"EVAL\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .left = createStringObject(
        b"left\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .right = createStringObject(
        b"right\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .pxat = createStringObject(
        b"PXAT\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .time = createStringObject(
        b"TIME\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .retrycount = createStringObject(
        b"RETRYCOUNT\0" as *const u8 as *const libc::c_char,
        10 as libc::c_int as size_t,
    );
    shared
        .force = createStringObject(
        b"FORCE\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .justid = createStringObject(
        b"JUSTID\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .entriesread = createStringObject(
        b"ENTRIESREAD\0" as *const u8 as *const libc::c_char,
        11 as libc::c_int as size_t,
    );
    shared
        .lastid = createStringObject(
        b"LASTID\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .default_username = createStringObject(
        b"default\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .ping = createStringObject(
        b"ping\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .setid = createStringObject(
        b"SETID\0" as *const u8 as *const libc::c_char,
        5 as libc::c_int as size_t,
    );
    shared
        .keepttl = createStringObject(
        b"KEEPTTL\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as size_t,
    );
    shared
        .absttl = createStringObject(
        b"ABSTTL\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .load = createStringObject(
        b"LOAD\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as size_t,
    );
    shared
        .createconsumer = createStringObject(
        b"CREATECONSUMER\0" as *const u8 as *const libc::c_char,
        14 as libc::c_int as size_t,
    );
    shared
        .getack = createStringObject(
        b"GETACK\0" as *const u8 as *const libc::c_char,
        6 as libc::c_int as size_t,
    );
    shared
        .special_asterick = createStringObject(
        b"*\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    shared
        .special_equals = createStringObject(
        b"=\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as size_t,
    );
    shared
        .redacted = makeObjectShared(
        createStringObject(
            b"(redacted)\0" as *const u8 as *const libc::c_char,
            10 as libc::c_int as size_t,
        ),
    );
    j = 0 as libc::c_int;
    while j < 10000 as libc::c_int {
        shared
            .integers[j
            as usize] = makeObjectShared(
            createObject(0 as libc::c_int, j as libc::c_long as *mut libc::c_void),
        );
        (*shared.integers[j as usize]).set_encoding(1 as libc::c_int as libc::c_uint);
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < 32 as libc::c_int {
        shared
            .mbulkhdr[j
            as usize] = createObject(
            0 as libc::c_int,
            sdscatprintf(sdsempty(), b"*%d\r\n\0" as *const u8 as *const libc::c_char, j)
                as *mut libc::c_void,
        );
        shared
            .bulkhdr[j
            as usize] = createObject(
            0 as libc::c_int,
            sdscatprintf(sdsempty(), b"$%d\r\n\0" as *const u8 as *const libc::c_char, j)
                as *mut libc::c_void,
        );
        shared
            .maphdr[j
            as usize] = createObject(
            0 as libc::c_int,
            sdscatprintf(
                sdsempty(),
                b"%%%d\r\n\0" as *const u8 as *const libc::c_char,
                j,
            ) as *mut libc::c_void,
        );
        shared
            .sethdr[j
            as usize] = createObject(
            0 as libc::c_int,
            sdscatprintf(sdsempty(), b"~%d\r\n\0" as *const u8 as *const libc::c_char, j)
                as *mut libc::c_void,
        );
        j += 1;
    }
    shared.minstring = sdsnew(b"minstring\0" as *const u8 as *const libc::c_char);
    shared.maxstring = sdsnew(b"maxstring\0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn initServerClientMemUsageBuckets() {
    if !(server.client_mem_usage_buckets).is_null() {
        return;
    }
    server
        .client_mem_usage_buckets = zmalloc(
        (core::mem::size_of::<clientMemUsageBucket>() as libc::c_ulong)
            .wrapping_mul(
                (1 as libc::c_int + 33 as libc::c_int - 15 as libc::c_int)
                    as libc::c_ulong,
            ),
    ) as *mut clientMemUsageBucket;
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 1 as libc::c_int + 33 as libc::c_int - 15 as libc::c_int {
        (*(server.client_mem_usage_buckets).offset(j as isize))
            .mem_usage_sum = 0 as libc::c_int as size_t;
        let ref mut fresh2 = (*(server.client_mem_usage_buckets).offset(j as isize))
            .clients;
        *fresh2 = listCreate();
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn freeServerClientMemUsageBuckets() {
    if (server.client_mem_usage_buckets).is_null() {
        return;
    }
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < 1 as libc::c_int + 33 as libc::c_int - 15 as libc::c_int {
        listRelease((*(server.client_mem_usage_buckets).offset(j as isize)).clients);
        j += 1;
    }
    zfree(server.client_mem_usage_buckets as *mut libc::c_void);
    server.client_mem_usage_buckets = 0 as *mut clientMemUsageBucket;
}
#[no_mangle]
pub unsafe extern "C" fn initServerConfig() {
    let mut j: libc::c_int = 0;
    let mut default_bindaddr: [*mut libc::c_char; 2] = [
        b"*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"-::*\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ];
    initConfigValues();
    updateCachedTime(1 as libc::c_int);
    getRandomHexChars((server.runid).as_mut_ptr(), 40 as libc::c_int as size_t);
    server.runid[40 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    changeReplicationId();
    clearReplicationId2();
    server.hz = 10 as libc::c_int;
    server.timezone = getTimeZone();
    server.configfile = 0 as *mut libc::c_char;
    server.executable = 0 as *mut libc::c_char;
    server
        .arch_bits = if core::mem::size_of::<libc::c_long>() as libc::c_ulong
        == 8 as libc::c_int as libc::c_ulong
    {
        64 as libc::c_int
    } else {
        32 as libc::c_int
    };
    server.bindaddr_count = 2 as libc::c_int;
    j = 0 as libc::c_int;
    while j < 2 as libc::c_int {
        server.bindaddr[j as usize] = zstrdup(default_bindaddr[j as usize]);
        j += 1;
    }
    server.ipfd.count = 0 as libc::c_int;
    server.tlsfd.count = 0 as libc::c_int;
    server.sofd = -(1 as libc::c_int);
    server.active_expire_enabled = 1 as libc::c_int;
    server.skip_checksum_validation = 0 as libc::c_int;
    core::ptr::write_volatile(
        &mut server.loading as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    core::ptr::write_volatile(
        &mut server.async_loading as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    server.loading_rdb_used_mem = 0 as libc::c_int as off_t;
    server.aof_state = 0 as libc::c_int;
    server.aof_rewrite_base_size = 0 as libc::c_int as off_t;
    server.aof_rewrite_scheduled = 0 as libc::c_int;
    server.aof_flush_sleep = 0 as libc::c_int;
    server.aof_last_fsync = time(0 as *mut time_t);
    server.aof_cur_timestamp = 0 as libc::c_int as time_t;
    core::intrinsics::atomic_store_relaxed(
        &mut server.aof_bio_fsync_status,
        0 as libc::c_int,
    );
    server.aof_rewrite_time_last = -(1 as libc::c_int) as time_t;
    server.aof_rewrite_time_start = -(1 as libc::c_int) as time_t;
    server.aof_lastbgrewrite_status = 0 as libc::c_int;
    server.aof_delayed_fsync = 0 as libc::c_int as libc::c_ulong;
    server.aof_fd = -(1 as libc::c_int);
    server.aof_selected_db = -(1 as libc::c_int);
    server.aof_flush_postponed_start = 0 as libc::c_int as time_t;
    server.aof_last_incr_size = 0 as libc::c_int as off_t;
    server.active_defrag_running = 0 as libc::c_int;
    server.notify_keyspace_events = 0 as libc::c_int;
    server.blocked_clients = 0 as libc::c_int as libc::c_uint;
    memset(
        (server.blocked_clients_by_type).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[libc::c_uint; 8]>() as libc::c_ulong,
    );
    core::ptr::write_volatile(
        &mut server.shutdown_asap as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    server.shutdown_flags = 0 as libc::c_int;
    server.shutdown_mstime = 0 as libc::c_int as mstime_t;
    server.cluster_module_flags = 0 as libc::c_int;
    server.migrate_cached_sockets = dictCreate(&mut migrateCacheDictType);
    server.next_client_id = 1 as libc::c_int as uint_least64_t;
    server.page_size = sysconf(_SC_PAGESIZE as libc::c_int) as size_t;
    server.pause_cron = 0 as libc::c_int;
    server.latency_tracking_info_percentiles_len = 3 as libc::c_int;
    server
        .latency_tracking_info_percentiles = zmalloc(
        (core::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(server.latency_tracking_info_percentiles_len as libc::c_ulong),
    ) as *mut libc::c_double;
    *(server.latency_tracking_info_percentiles)
        .offset(0 as libc::c_int as isize) = 50.0f64;
    *(server.latency_tracking_info_percentiles)
        .offset(1 as libc::c_int as isize) = 99.0f64;
    *(server.latency_tracking_info_percentiles)
        .offset(2 as libc::c_int as isize) = 99.9f64;
    let mut lruclock: libc::c_uint = getLRUClock();
    core::intrinsics::atomic_store_relaxed(&mut server.lruclock, lruclock);
    resetServerSaveParams();
    appendServerSaveParams(
        (60 as libc::c_int * 60 as libc::c_int) as time_t,
        1 as libc::c_int,
    );
    appendServerSaveParams(300 as libc::c_int as time_t, 100 as libc::c_int);
    appendServerSaveParams(60 as libc::c_int as time_t, 10000 as libc::c_int);
    server.masterhost = 0 as *mut libc::c_char;
    server.masterport = 6379 as libc::c_int;
    server.master = 0 as *mut client;
    server.cached_master = 0 as *mut client;
    server.master_initial_offset = -(1 as libc::c_int) as libc::c_longlong;
    server.repl_state = REPL_STATE_NONE as libc::c_int;
    server.repl_transfer_tmpfile = 0 as *mut libc::c_char;
    server.repl_transfer_fd = -(1 as libc::c_int);
    server.repl_transfer_s = 0 as *mut connection;
    server.repl_syncio_timeout = 5 as libc::c_int;
    server.repl_down_since = 0 as libc::c_int as time_t;
    server.master_repl_offset = 0 as libc::c_int as libc::c_longlong;
    server.repl_backlog = 0 as *mut replBacklog;
    server.repl_no_slaves_since = time(0 as *mut time_t);
    server.failover_end_time = 0 as libc::c_int as mstime_t;
    server.force_failover = 0 as libc::c_int;
    server.target_replica_host = 0 as *mut libc::c_char;
    server.target_replica_port = 0 as libc::c_int;
    server.failover_state = NO_FAILOVER as libc::c_int;
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        server.client_obuf_limits[j as usize] = clientBufferLimitsDefaults[j as usize];
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < 3 as libc::c_int {
        server
            .oom_score_adj_values[j
            as usize] = configOOMScoreAdjValuesDefaults[j as usize];
        j += 1;
    }
    R_Zero = 0.0f64;
    R_PosInf = 1.0f64 / R_Zero;
    R_NegInf = -1.0f64 / R_Zero;
    R_Nan = R_Zero / R_Zero;
    server.commands = dictCreate(&mut commandTableDictType);
    server.orig_commands = dictCreate(&mut commandTableDictType);
    populateCommandTable();
    server.watchdog_period = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn restartServer(
    mut flags: libc::c_int,
    mut delay: mstime_t,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    if access(server.executable, 1 as libc::c_int) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't restart: this process has no permissions to execute %s\0"
                    as *const u8 as *const libc::c_char,
                server.executable,
            );
        }
        return -(1 as libc::c_int);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0
        && !(server.configfile).is_null()
        && rewriteConfig(server.configfile, 0 as libc::c_int) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't restart: configuration rewrite process failed: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        return -(1 as libc::c_int);
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0
        && prepareForShutdown(4 as libc::c_int) != 0 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Can't restart: error preparing for shutdown\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    j = 3 as libc::c_int;
    while j < server.maxclients as libc::c_int + 1024 as libc::c_int {
        if fcntl(j, 1 as libc::c_int) != -(1 as libc::c_int) {
            close(j);
        }
        j += 1;
    }
    if delay != 0 {
        usleep((delay * 1000 as libc::c_int as libc::c_longlong) as __useconds_t);
    }
    zfree(*(server.exec_argv).offset(0 as libc::c_int as isize) as *mut libc::c_void);
    let ref mut fresh3 = *(server.exec_argv).offset(0 as libc::c_int as isize);
    *fresh3 = zstrdup(server.executable);
    execve(
        server.executable,
        server.exec_argv as *const *mut libc::c_char,
        environ as *const *mut libc::c_char,
    );
    _exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn setOOMScoreAdj(mut process_class: libc::c_int) -> libc::c_int {
    if process_class == -(1 as libc::c_int) {
        process_class = if !(server.masterhost).is_null() {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
    }
    if process_class >= 0 as libc::c_int && process_class < 3 as libc::c_int {} else {
        _serverAssert(
            b"process_class >= 0 && process_class < CONFIG_OOM_COUNT\0" as *const u8
                as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            2100 as libc::c_int,
        );
        unreachable!();
    };
    static mut oom_score_adjusted_by_redis: libc::c_int = 0 as libc::c_int;
    static mut oom_score_adj_base: libc::c_int = 0 as libc::c_int;
    let mut fd: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let mut buf: [libc::c_char; 64] = [0; 64];
    if server.oom_score_adj != 0 as libc::c_int {
        if oom_score_adjusted_by_redis == 0 {
            oom_score_adjusted_by_redis = 1 as libc::c_int;
            fd = open(
                b"/proc/self/oom_score_adj\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            if fd < 0 as libc::c_int
                || read(
                    fd,
                    buf.as_mut_ptr() as *mut libc::c_void,
                    core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                ) < 0 as libc::c_int as libc::c_long
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Unable to read oom_score_adj: %s\0" as *const u8
                            as *const libc::c_char,
                        strerror(*__errno_location()),
                    );
                }
                if fd != -(1 as libc::c_int) {
                    close(fd);
                }
                return -(1 as libc::c_int);
            }
            oom_score_adj_base = atoi(buf.as_mut_ptr());
            close(fd);
        }
        val = server.oom_score_adj_values[process_class as usize];
        if server.oom_score_adj == 1 as libc::c_int {
            val += oom_score_adj_base;
        }
        if val > 1000 as libc::c_int {
            val = 1000 as libc::c_int;
        }
        if val < -(1000 as libc::c_int) {
            val = -(1000 as libc::c_int);
        }
    } else if oom_score_adjusted_by_redis != 0 {
        oom_score_adjusted_by_redis = 0 as libc::c_int;
        val = oom_score_adj_base;
    } else {
        return 0 as libc::c_int
    }
    snprintf(
        buf.as_mut_ptr(),
        (core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        b"%d\n\0" as *const u8 as *const libc::c_char,
        val,
    );
    fd = open(
        b"/proc/self/oom_score_adj\0" as *const u8 as *const libc::c_char,
        0o1 as libc::c_int,
    );
    if fd < 0 as libc::c_int
        || write(fd, buf.as_mut_ptr() as *const libc::c_void, strlen(buf.as_mut_ptr()))
            < 0 as libc::c_int as libc::c_long
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unable to write oom_score_adj: %s\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        if fd != -(1 as libc::c_int) {
            close(fd);
        }
        return -(1 as libc::c_int);
    }
    close(fd);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn adjustOpenFilesLimit() {
    let mut maxfiles: rlim_t = (server.maxclients)
        .wrapping_add(32 as libc::c_int as libc::c_uint) as rlim_t;
    let mut limit: rlimit = rlimit { rlim_cur: 0, rlim_max: 0 };
    if getrlimit(RLIMIT_NOFILE, &mut limit) == -(1 as libc::c_int) {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Unable to obtain the current NOFILE limit (%s), assuming 1024 and setting the max clients configuration accordingly.\0"
                    as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        server.maxclients = (1024 as libc::c_int - 32 as libc::c_int) as libc::c_uint;
    } else {
        let mut oldlimit: rlim_t = limit.rlim_cur;
        if oldlimit < maxfiles {
            let mut bestlimit: rlim_t = 0;
            let mut setrlimit_error: libc::c_int = 0 as libc::c_int;
            bestlimit = maxfiles;
            while bestlimit > oldlimit {
                let mut decr_step: rlim_t = 16 as libc::c_int as rlim_t;
                limit.rlim_cur = bestlimit;
                limit.rlim_max = bestlimit;
                if setrlimit(RLIMIT_NOFILE, &mut limit) != -(1 as libc::c_int) {
                    break;
                }
                setrlimit_error = *__errno_location();
                if bestlimit < decr_step {
                    bestlimit = oldlimit;
                    break;
                } else {
                    bestlimit = (bestlimit as libc::c_ulong).wrapping_sub(decr_step)
                        as rlim_t as rlim_t;
                }
            }
            if bestlimit < oldlimit {
                bestlimit = oldlimit;
            }
            if bestlimit < maxfiles {
                let mut old_maxclients: libc::c_uint = server.maxclients;
                server
                    .maxclients = bestlimit
                    .wrapping_sub(32 as libc::c_int as libc::c_ulong) as libc::c_uint;
                if bestlimit <= 32 as libc::c_int as libc::c_ulong {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Your current 'ulimit -n' of %llu is not enough for the server to start. Please increase your open file limit to at least %llu. Exiting.\0"
                                as *const u8 as *const libc::c_char,
                            oldlimit as libc::c_ulonglong,
                            maxfiles as libc::c_ulonglong,
                        );
                    }
                    exit(1 as libc::c_int);
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"You requested maxclients of %d requiring at least %llu max file descriptors.\0"
                            as *const u8 as *const libc::c_char,
                        old_maxclients,
                        maxfiles as libc::c_ulonglong,
                    );
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Server can't set maximum open files to %llu because of OS error: %s.\0"
                            as *const u8 as *const libc::c_char,
                        maxfiles as libc::c_ulonglong,
                        strerror(setrlimit_error),
                    );
                }
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Current maximum open files is %llu. maxclients has been reduced to %d to compensate for low ulimit. If you need higher maxclients increase 'ulimit -n'.\0"
                            as *const u8 as *const libc::c_char,
                        bestlimit as libc::c_ulonglong,
                        server.maxclients,
                    );
                }
            } else if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Increased maximum number of open files to %llu (it was originally set to %llu).\0"
                        as *const u8 as *const libc::c_char,
                    maxfiles as libc::c_ulonglong,
                    oldlimit as libc::c_ulonglong,
                );
            }
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkTcpBacklogSettings() {
    let mut fp: *mut FILE = fopen(
        b"/proc/sys/net/core/somaxconn\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    let mut buf: [libc::c_char; 1024] = [0; 1024];
    if fp.is_null() {
        return;
    }
    if !(fgets(
        buf.as_mut_ptr(),
        core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong as libc::c_int,
        fp,
    ))
        .is_null()
    {
        let mut somaxconn: libc::c_int = atoi(buf.as_mut_ptr());
        if somaxconn > 0 as libc::c_int && somaxconn < server.tcp_backlog {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"WARNING: The TCP backlog setting of %d cannot be enforced because /proc/sys/net/core/somaxconn is set to the lower value of %d.\0"
                        as *const u8 as *const libc::c_char,
                    server.tcp_backlog,
                    somaxconn,
                );
            }
        }
    }
    fclose(fp);
}
#[no_mangle]
pub unsafe extern "C" fn closeSocketListeners(mut sfd: *mut socketFds) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*sfd).count {
        if !((*sfd).fd[j as usize] == -(1 as libc::c_int)) {
            aeDeleteFileEvent(server.el, (*sfd).fd[j as usize], 1 as libc::c_int);
            close((*sfd).fd[j as usize]);
        }
        j += 1;
    }
    (*sfd).count = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn createSocketAcceptHandler(
    mut sfd: *mut socketFds,
    mut accept_handler: Option::<aeFileProc>,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < (*sfd).count {
        if aeCreateFileEvent(
            server.el,
            (*sfd).fd[j as usize],
            1 as libc::c_int,
            accept_handler,
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int)
        {
            j = j - 1 as libc::c_int;
            while j >= 0 as libc::c_int {
                aeDeleteFileEvent(server.el, (*sfd).fd[j as usize], 1 as libc::c_int);
                j -= 1;
            }
            return -(1 as libc::c_int);
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn listenToPort(
    mut port: libc::c_int,
    mut sfd: *mut socketFds,
) -> libc::c_int {
    let mut j: libc::c_int = 0;
    let mut bindaddr: *mut *mut libc::c_char = (server.bindaddr).as_mut_ptr();
    if server.bindaddr_count == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < server.bindaddr_count {
        let mut addr: *mut libc::c_char = *bindaddr.offset(j as isize);
        let mut optional: libc::c_int = (*addr as libc::c_int == '-' as i32)
            as libc::c_int;
        if optional != 0 {
            addr = addr.offset(1);
        }
        if !(strchr(addr, ':' as i32)).is_null() {
            (*sfd)
                .fd[(*sfd).count
                as usize] = anetTcp6Server(
                (server.neterr).as_mut_ptr(),
                port,
                addr,
                server.tcp_backlog,
            );
        } else {
            (*sfd)
                .fd[(*sfd).count
                as usize] = anetTcpServer(
                (server.neterr).as_mut_ptr(),
                port,
                addr,
                server.tcp_backlog,
            );
        }
        if (*sfd).fd[(*sfd).count as usize] == -(1 as libc::c_int) {
            let mut net_errno: libc::c_int = *__errno_location();
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Warning: Could not create server TCP listening socket %s:%d: %s\0"
                        as *const u8 as *const libc::c_char,
                    addr,
                    port,
                    (server.neterr).as_mut_ptr(),
                );
            }
            if !(net_errno == 99 as libc::c_int && optional != 0) {
                if !(net_errno == 92 as libc::c_int || net_errno == 93 as libc::c_int
                    || net_errno == 94 as libc::c_int || net_errno == 96 as libc::c_int
                    || net_errno == 97 as libc::c_int)
                {
                    closeSocketListeners(sfd);
                    return -(1 as libc::c_int);
                }
            }
        } else {
            if server.socket_mark_id > 0 as libc::c_int as libc::c_uint {
                anetSetSockMarkId(
                    0 as *mut libc::c_char,
                    (*sfd).fd[(*sfd).count as usize],
                    server.socket_mark_id,
                );
            }
            anetNonBlock(0 as *mut libc::c_char, (*sfd).fd[(*sfd).count as usize]);
            anetCloexec((*sfd).fd[(*sfd).count as usize]);
            (*sfd).count += 1;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn resetServerStats() {
    let mut j: libc::c_int = 0;
    server.stat_numcommands = 0 as libc::c_int as libc::c_longlong;
    server.stat_numconnections = 0 as libc::c_int as libc::c_longlong;
    server.stat_expiredkeys = 0 as libc::c_int as libc::c_longlong;
    server.stat_expired_stale_perc = 0 as libc::c_int as libc::c_double;
    server.stat_expired_time_cap_reached_count = 0 as libc::c_int as libc::c_longlong;
    server.stat_expire_cycle_time_used = 0 as libc::c_int as libc::c_longlong;
    server.stat_evictedkeys = 0 as libc::c_int as libc::c_longlong;
    server.stat_evictedclients = 0 as libc::c_int as libc::c_longlong;
    server.stat_total_eviction_exceeded_time = 0 as libc::c_int as libc::c_longlong;
    server.stat_last_eviction_exceeded_time = 0 as libc::c_int as monotime;
    server.stat_keyspace_misses = 0 as libc::c_int as libc::c_longlong;
    server.stat_keyspace_hits = 0 as libc::c_int as libc::c_longlong;
    server.stat_active_defrag_hits = 0 as libc::c_int as libc::c_longlong;
    server.stat_active_defrag_misses = 0 as libc::c_int as libc::c_longlong;
    server.stat_active_defrag_key_hits = 0 as libc::c_int as libc::c_longlong;
    server.stat_active_defrag_key_misses = 0 as libc::c_int as libc::c_longlong;
    server.stat_active_defrag_scanned = 0 as libc::c_int as libc::c_longlong;
    server.stat_total_active_defrag_time = 0 as libc::c_int as libc::c_longlong;
    server.stat_last_active_defrag_time = 0 as libc::c_int as monotime;
    server.stat_fork_time = 0 as libc::c_int as libc::c_longlong;
    server.stat_fork_rate = 0 as libc::c_int as libc::c_double;
    server.stat_total_forks = 0 as libc::c_int as libc::c_longlong;
    server.stat_rejected_conn = 0 as libc::c_int as libc::c_longlong;
    server.stat_sync_full = 0 as libc::c_int as libc::c_longlong;
    server.stat_sync_partial_ok = 0 as libc::c_int as libc::c_longlong;
    server.stat_sync_partial_err = 0 as libc::c_int as libc::c_longlong;
    server.stat_io_reads_processed = 0 as libc::c_int as libc::c_longlong;
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_total_reads_processed,
        0 as libc::c_int as libc::c_longlong,
    );
    server.stat_io_writes_processed = 0 as libc::c_int as libc::c_longlong;
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_total_writes_processed,
        0 as libc::c_int as libc::c_longlong,
    );
    j = 0 as libc::c_int;
    while j < 5 as libc::c_int {
        server.inst_metric[j as usize].idx = 0 as libc::c_int;
        server.inst_metric[j as usize].last_sample_time = mstime();
        server
            .inst_metric[j as usize]
            .last_sample_count = 0 as libc::c_int as libc::c_longlong;
        memset(
            (server.inst_metric[j as usize].samples).as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            core::mem::size_of::<[libc::c_longlong; 16]>() as libc::c_ulong,
        );
        j += 1;
    }
    server.stat_aof_rewrites = 0 as libc::c_int as libc::c_longlong;
    server.stat_rdb_saves = 0 as libc::c_int as libc::c_longlong;
    server.stat_aofrw_consecutive_failures = 0 as libc::c_int as libc::c_longlong;
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_net_input_bytes,
        0 as libc::c_int as libc::c_longlong,
    );
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_net_output_bytes,
        0 as libc::c_int as libc::c_longlong,
    );
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_net_repl_input_bytes,
        0 as libc::c_int as libc::c_longlong,
    );
    core::intrinsics::atomic_store_relaxed(
        &mut server.stat_net_repl_output_bytes,
        0 as libc::c_int as libc::c_longlong,
    );
    server.stat_unexpected_error_replies = 0 as libc::c_int as libc::c_longlong;
    server.stat_total_error_replies = 0 as libc::c_int as libc::c_longlong;
    server.stat_dump_payload_sanitizations = 0 as libc::c_int as libc::c_longlong;
    server.aof_delayed_fsync = 0 as libc::c_int as libc::c_ulong;
    server.stat_reply_buffer_shrinks = 0 as libc::c_int as libc::c_longlong;
    server.stat_reply_buffer_expands = 0 as libc::c_int as libc::c_longlong;
    lazyfreeResetStats();
}
#[no_mangle]
pub unsafe extern "C" fn makeThreadKillable() {
    pthread_setcancelstate(PTHREAD_CANCEL_ENABLE as libc::c_int, 0 as *mut libc::c_int);
    pthread_setcanceltype(
        PTHREAD_CANCEL_ASYNCHRONOUS as libc::c_int,
        0 as *mut libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn initServer() {
    let mut j: libc::c_int = 0;
    signal(
        1 as libc::c_int,
        core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    signal(
        13 as libc::c_int,
        core::mem::transmute::<
            libc::intptr_t,
            __sighandler_t,
        >(1 as libc::c_int as libc::intptr_t),
    );
    setupSignalHandlers();
    makeThreadKillable();
    if server.syslog_enabled != 0 {
        openlog(
            server.syslog_ident,
            0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int,
            server.syslog_facility,
        );
    }
    server
        .aof_state = if server.aof_enabled != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
    server.hz = server.config_hz;
    server.pid = getpid();
    server.in_fork_child = 0 as libc::c_int;
    server.main_thread_id = pthread_self();
    server.current_client = 0 as *mut client;
    server.errors = raxNew();
    server.fixed_time_expire = 0 as libc::c_int as libc::c_long;
    server.in_nested_call = 0 as libc::c_int;
    server.clients = listCreate();
    server.clients_index = raxNew();
    server.clients_to_close = listCreate();
    server.slaves = listCreate();
    server.monitors = listCreate();
    server.clients_pending_write = listCreate();
    server.clients_pending_read = listCreate();
    server.clients_timeout_table = raxNew();
    server.replication_allowed = 1 as libc::c_int;
    server.slaveseldb = -(1 as libc::c_int);
    server.unblocked_clients = listCreate();
    server.ready_keys = listCreate();
    server.tracking_pending_keys = listCreate();
    server.clients_waiting_acks = listCreate();
    server.get_ack_from_slaves = 0 as libc::c_int;
    server.client_pause_type = CLIENT_PAUSE_OFF;
    server.client_pause_end_time = 0 as libc::c_int as mstime_t;
    memset(
        (server.client_pause_per_purpose).as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<[*mut pause_event; 3]>() as libc::c_ulong,
    );
    server.postponed_clients = listCreate();
    server.events_processed_while_blocked = 0 as libc::c_int as libc::c_longlong;
    server.system_memory_size = zmalloc_get_memory_size();
    server.blocked_last_cron = 0 as libc::c_int as libc::c_longlong;
    server.blocking_op_nesting = 0 as libc::c_int as size_t;
    server.thp_enabled = 0 as libc::c_int;
    server.cluster_drop_packet_filter = -(1 as libc::c_int);
    server.reply_buffer_peak_reset_time = 5000 as libc::c_int as libc::c_long;
    server.reply_buffer_resizing_enabled = 1 as libc::c_int;
    server.client_mem_usage_buckets = 0 as *mut clientMemUsageBucket;
    resetReplicationBuffer();
    if (server.tls_port != 0 || server.tls_replication != 0 || server.tls_cluster != 0)
        && tlsConfigure(&mut server.tls_ctx_config) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed to configure TLS. Check logs for more info.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    createSharedObjects();
    adjustOpenFilesLimit();
    let mut clk_msg: *const libc::c_char = monotonicInit();
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"monotonic clock: %s\0" as *const u8 as *const libc::c_char,
            clk_msg,
        );
    }
    server
        .el = aeCreateEventLoop(
        (server.maxclients)
            .wrapping_add((32 as libc::c_int + 96 as libc::c_int) as libc::c_uint)
            as libc::c_int,
    );
    if (server.el).is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed creating the event loop. Error message: '%s'\0" as *const u8
                    as *const libc::c_char,
                strerror(*__errno_location()),
            );
        }
        exit(1 as libc::c_int);
    }
    server
        .db = zmalloc(
        (core::mem::size_of::<redisDb>() as libc::c_ulong)
            .wrapping_mul(server.dbnum as libc::c_ulong),
    ) as *mut redisDb;
    if server.port != 0 as libc::c_int
        && listenToPort(server.port, &mut server.ipfd) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed listening on port %u (TCP), aborting.\0" as *const u8
                    as *const libc::c_char,
                server.port,
            );
        }
        exit(1 as libc::c_int);
    }
    if server.tls_port != 0 as libc::c_int
        && listenToPort(server.tls_port, &mut server.tlsfd) == -(1 as libc::c_int)
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed listening on port %u (TLS), aborting.\0" as *const u8
                    as *const libc::c_char,
                server.tls_port,
            );
        }
        exit(1 as libc::c_int);
    }
    if !(server.unixsocket).is_null() {
        unlink(server.unixsocket);
        server
            .sofd = anetUnixServer(
            (server.neterr).as_mut_ptr(),
            server.unixsocket,
            server.unixsocketperm,
            server.tcp_backlog,
        );
        if server.sofd == -(1 as libc::c_int) {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed opening Unix socket: %s\0" as *const u8
                        as *const libc::c_char,
                    (server.neterr).as_mut_ptr(),
                );
            }
            exit(1 as libc::c_int);
        }
        anetNonBlock(0 as *mut libc::c_char, server.sofd);
        anetCloexec(server.sofd);
    }
    if server.ipfd.count == 0 as libc::c_int && server.tlsfd.count == 0 as libc::c_int
        && server.sofd < 0 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Configured to not listen anywhere, exiting.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        exit(1 as libc::c_int);
    }
    j = 0 as libc::c_int;
    while j < server.dbnum {
        let ref mut fresh4 = (*(server.db).offset(j as isize)).dict;
        *fresh4 = dictCreate(&mut dbDictType);
        let ref mut fresh5 = (*(server.db).offset(j as isize)).expires;
        *fresh5 = dictCreate(&mut dbExpiresDictType);
        (*(server.db).offset(j as isize))
            .expires_cursor = 0 as libc::c_int as libc::c_ulong;
        let ref mut fresh6 = (*(server.db).offset(j as isize)).blocking_keys;
        *fresh6 = dictCreate(&mut keylistDictType);
        let ref mut fresh7 = (*(server.db).offset(j as isize)).ready_keys;
        *fresh7 = dictCreate(&mut objectKeyPointerValueDictType);
        let ref mut fresh8 = (*(server.db).offset(j as isize)).watched_keys;
        *fresh8 = dictCreate(&mut keylistDictType);
        (*(server.db).offset(j as isize)).id = j;
        (*(server.db).offset(j as isize)).avg_ttl = 0 as libc::c_int as libc::c_longlong;
        let ref mut fresh9 = (*(server.db).offset(j as isize)).defrag_later;
        *fresh9 = listCreate();
        let ref mut fresh10 = (*(server.db).offset(j as isize)).slots_to_keys;
        *fresh10 = 0 as *mut clusterSlotToKeyMapping;
        let ref mut fresh11 = (*(*(server.db).offset(j as isize)).defrag_later).free;
        *fresh11 = core::mem::transmute::<
            Option::<unsafe extern "C" fn(sds) -> ()>,
            Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
        >(Some(sdsfree as unsafe extern "C" fn(sds) -> ()));
        j += 1;
    }
    evictionPoolAlloc();
    server.pubsub_channels = dictCreate(&mut keylistDictType);
    server.pubsub_patterns = dictCreate(&mut keylistDictType);
    server.pubsubshard_channels = dictCreate(&mut keylistDictType);
    server.cronloops = 0 as libc::c_int;
    server.in_exec = 0 as libc::c_int;
    server.busy_module_yield_flags = 0 as libc::c_int;
    server.busy_module_yield_reply = 0 as *const libc::c_char;
    server.core_propagates = 0 as libc::c_int;
    server.propagate_no_multi = 0 as libc::c_int;
    server.module_ctx_nesting = 0 as libc::c_int;
    server.client_pause_in_transaction = 0 as libc::c_int;
    server.child_pid = -(1 as libc::c_int);
    server.child_type = 0 as libc::c_int;
    server.rdb_child_type = 0 as libc::c_int;
    server.rdb_pipe_conns = 0 as *mut *mut connection;
    server.rdb_pipe_numconns = 0 as libc::c_int;
    server.rdb_pipe_numconns_writing = 0 as libc::c_int;
    server.rdb_pipe_buff = 0 as *mut libc::c_char;
    server.rdb_pipe_bufflen = 0 as libc::c_int;
    server.rdb_bgsave_scheduled = 0 as libc::c_int;
    server.child_info_pipe[0 as libc::c_int as usize] = -(1 as libc::c_int);
    server.child_info_pipe[1 as libc::c_int as usize] = -(1 as libc::c_int);
    server.child_info_nread = 0 as libc::c_int;
    server.aof_buf = sdsempty();
    server.lastsave = time(0 as *mut time_t);
    server.lastbgsave_try = 0 as libc::c_int as time_t;
    server.rdb_save_time_last = -(1 as libc::c_int) as time_t;
    server.rdb_save_time_start = -(1 as libc::c_int) as time_t;
    server.rdb_last_load_keys_expired = 0 as libc::c_int as libc::c_longlong;
    server.rdb_last_load_keys_loaded = 0 as libc::c_int as libc::c_longlong;
    server.dirty = 0 as libc::c_int as libc::c_longlong;
    resetServerStats();
    server.stat_starttime = time(0 as *mut time_t);
    server.stat_peak_memory = 0 as libc::c_int as size_t;
    server.stat_current_cow_peak = 0 as libc::c_int as size_t;
    server.stat_current_cow_bytes = 0 as libc::c_int as size_t;
    server.stat_current_cow_updated = 0 as libc::c_int as monotime;
    server.stat_current_save_keys_processed = 0 as libc::c_int as size_t;
    server.stat_current_save_keys_total = 0 as libc::c_int as size_t;
    server.stat_rdb_cow_bytes = 0 as libc::c_int as size_t;
    server.stat_aof_cow_bytes = 0 as libc::c_int as size_t;
    server.stat_module_cow_bytes = 0 as libc::c_int as size_t;
    server.stat_module_progress = 0 as libc::c_int as libc::c_double;
    let mut j_0: libc::c_int = 0 as libc::c_int;
    while j_0 < 4 as libc::c_int {
        server.stat_clients_type_memory[j_0 as usize] = 0 as libc::c_int as size_t;
        j_0 += 1;
    }
    server.stat_cluster_links_memory = 0 as libc::c_int as size_t;
    server.cron_malloc_stats.zmalloc_used = 0 as libc::c_int as size_t;
    server.cron_malloc_stats.process_rss = 0 as libc::c_int as size_t;
    server.cron_malloc_stats.allocator_allocated = 0 as libc::c_int as size_t;
    server.cron_malloc_stats.allocator_active = 0 as libc::c_int as size_t;
    server.cron_malloc_stats.allocator_resident = 0 as libc::c_int as size_t;
    server.lastbgsave_status = 0 as libc::c_int;
    server.aof_last_write_status = 0 as libc::c_int;
    server.aof_last_write_errno = 0 as libc::c_int;
    server.repl_good_slaves_count = 0 as libc::c_int;
    server.last_sig_received = 0 as libc::c_int;
    if aeCreateTimeEvent(
        server.el,
        1 as libc::c_int as libc::c_longlong,
        Some(
            serverCron
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_longlong,
                    *mut libc::c_void,
                ) -> libc::c_int,
        ),
        0 as *mut libc::c_void,
        None,
    ) == -(1 as libc::c_int) as libc::c_longlong
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            2630 as libc::c_int,
            b"Can't create event loop timers.\0" as *const u8 as *const libc::c_char,
        );
        unreachable!();
        exit(1 as libc::c_int);
    }
    if createSocketAcceptHandler(
        &mut server.ipfd,
        Some(
            acceptTcpHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) != 0 as libc::c_int
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            2637 as libc::c_int,
            b"Unrecoverable error creating TCP socket accept handler.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    if createSocketAcceptHandler(
        &mut server.tlsfd,
        Some(
            acceptTLSHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) != 0 as libc::c_int
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            2640 as libc::c_int,
            b"Unrecoverable error creating TLS socket accept handler.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    if server.sofd > 0 as libc::c_int
        && aeCreateFileEvent(
            server.el,
            server.sofd,
            1 as libc::c_int,
            Some(
                acceptUnixHandler
                    as unsafe extern "C" fn(
                        *mut aeEventLoop,
                        libc::c_int,
                        *mut libc::c_void,
                        libc::c_int,
                    ) -> (),
            ),
            0 as *mut libc::c_void,
        ) == -(1 as libc::c_int)
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            2643 as libc::c_int,
            b"Unrecoverable error creating server.sofd file event.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    if aeCreateFileEvent(
        server.el,
        server.module_pipe[0 as libc::c_int as usize],
        1 as libc::c_int,
        Some(
            modulePipeReadable
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
        0 as *mut libc::c_void,
    ) == -(1 as libc::c_int)
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            2651 as libc::c_int,
            b"Error registering the readable event for the module pipe.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    aeSetBeforeSleepProc(
        server.el,
        Some(beforeSleep as unsafe extern "C" fn(*mut aeEventLoop) -> ()),
    );
    aeSetAfterSleepProc(
        server.el,
        Some(afterSleep as unsafe extern "C" fn(*mut aeEventLoop) -> ()),
    );
    if server.arch_bits == 32 as libc::c_int
        && server.maxmemory == 0 as libc::c_int as libc::c_ulonglong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Warning: 32 bit instance detected but no memory limit set. Setting 3 GB maxmemory limit with 'noeviction' policy now.\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        server
            .maxmemory = (3072 as libc::c_longlong
            * (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_longlong)
            as libc::c_ulonglong;
        server.maxmemory_policy = (7 as libc::c_int) << 8 as libc::c_int;
    }
    if server.cluster_enabled != 0 {
        clusterInit();
    }
    scriptingInit(1 as libc::c_int);
    functionsInit();
    slowlogInit();
    latencyMonitorInit();
    ACLUpdateDefaultUserPassword(server.requirepass);
    applyWatchdogPeriod();
    if server.maxmemory_clients != 0 as libc::c_int as libc::c_long {
        initServerClientMemUsageBuckets();
    }
}
#[no_mangle]
pub unsafe extern "C" fn InitServerLast() {
    bioInit();
    initThreadedIO();
    set_jemalloc_bg_thread(server.jemalloc_bg_thread);
    server.initial_memory_usage = zmalloc_used_memory();
}
#[no_mangle]
pub unsafe extern "C" fn populateCommandLegacyRangeSpec(mut c: *mut redisCommand) {
    memset(
        &mut (*c).legacy_range_key_spec as *mut keySpec as *mut libc::c_void,
        0 as libc::c_int,
        core::mem::size_of::<keySpec>() as libc::c_ulong,
    );
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 21 as libc::c_int
        != 0
    {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
    }
    if (*c).key_specs_num == 0 as libc::c_int {
        return;
    }
    if (*c).key_specs_num == 1 as libc::c_int
        && (*((*c).key_specs).offset(0 as libc::c_int as isize)).begin_search_type
            as libc::c_uint == KSPEC_BS_INDEX as libc::c_int as libc::c_uint
        && (*((*c).key_specs).offset(0 as libc::c_int as isize)).find_keys_type
            as libc::c_uint == KSPEC_FK_RANGE as libc::c_int as libc::c_uint
    {
        (*c).legacy_range_key_spec = *((*c).key_specs).offset(0 as libc::c_int as isize);
        if (*((*c).key_specs).offset(0 as libc::c_int as isize)).flags
            as libc::c_ulonglong & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
        {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
        }
        return;
    }
    let mut firstkey: libc::c_int = 2147483647 as libc::c_int;
    let mut lastkey: libc::c_int = 0 as libc::c_int;
    let mut prev_lastkey: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*c).key_specs_num {
        if (*((*c).key_specs).offset(i as isize)).begin_search_type as libc::c_uint
            != KSPEC_BS_INDEX as libc::c_int as libc::c_uint
            || (*((*c).key_specs).offset(i as isize)).find_keys_type as libc::c_uint
                != KSPEC_FK_RANGE as libc::c_int as libc::c_uint
        {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
        } else if (*((*c).key_specs).offset(i as isize)).fk.range.keystep
            != 1 as libc::c_int
            || prev_lastkey != 0
                && prev_lastkey
                    != (*((*c).key_specs).offset(i as isize)).bs.index.pos
                        - 1 as libc::c_int
        {
            (*c)
                .flags = ((*c).flags as libc::c_ulonglong
                | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
        } else {
            if (*((*c).key_specs).offset(i as isize)).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0
            {
                (*c)
                    .flags = ((*c).flags as libc::c_ulonglong
                    | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
            }
            firstkey = if firstkey < (*((*c).key_specs).offset(i as isize)).bs.index.pos
            {
                firstkey
            } else {
                (*((*c).key_specs).offset(i as isize)).bs.index.pos
            };
            let mut lastkey_abs_index: libc::c_int = (*((*c).key_specs)
                .offset(i as isize))
                .fk
                .range
                .lastkey;
            if lastkey_abs_index >= 0 as libc::c_int {
                lastkey_abs_index += (*((*c).key_specs).offset(i as isize)).bs.index.pos;
            }
            lastkey = (if lastkey as libc::c_uint > lastkey_abs_index as libc::c_uint {
                lastkey as libc::c_uint
            } else {
                lastkey_abs_index as libc::c_uint
            }) as libc::c_int;
            prev_lastkey = lastkey;
        }
        i += 1;
    }
    if firstkey == 2147483647 as libc::c_int {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t;
        return;
    }
    if firstkey != 0 as libc::c_int {} else {
        _serverAssert(
            b"firstkey != 0\0" as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            2790 as libc::c_int,
        );
        unreachable!();
    };
    if lastkey != 0 as libc::c_int {} else {
        _serverAssert(
            b"lastkey != 0\0" as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            2791 as libc::c_int,
        );
        unreachable!();
    };
    (*c).legacy_range_key_spec.begin_search_type = KSPEC_BS_INDEX;
    (*c).legacy_range_key_spec.bs.index.pos = firstkey;
    (*c).legacy_range_key_spec.find_keys_type = KSPEC_FK_RANGE;
    (*c)
        .legacy_range_key_spec
        .fk
        .range
        .lastkey = if lastkey < 0 as libc::c_int { lastkey } else { lastkey - firstkey };
    (*c).legacy_range_key_spec.fk.range.keystep = 1 as libc::c_int;
    (*c).legacy_range_key_spec.fk.range.limit = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn catSubCommandFullname(
    mut parent_name: *const libc::c_char,
    mut sub_name: *const libc::c_char,
) -> sds {
    return sdscatfmt(
        sdsempty(),
        b"%s|%s\0" as *const u8 as *const libc::c_char,
        parent_name,
        sub_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn commandAddSubcommand(
    mut parent: *mut redisCommand,
    mut subcommand: *mut redisCommand,
    mut declared_name: *const libc::c_char,
) {
    if ((*parent).subcommands_dict).is_null() {
        (*parent).subcommands_dict = dictCreate(&mut commandTableDictType);
    }
    (*subcommand).parent = parent;
    (*subcommand).id = ACLGetCommandID((*subcommand).fullname) as libc::c_int;
    if dictAdd(
        (*parent).subcommands_dict,
        sdsnew(declared_name) as *mut libc::c_void,
        subcommand as *mut libc::c_void,
    ) == 0 as libc::c_int
    {} else {
        _serverAssert(
            b"dictAdd(parent->subcommands_dict, sdsnew(declared_name), subcommand) == DICT_OK\0"
                as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            2812 as libc::c_int,
        );
        unreachable!();
    };
}
#[no_mangle]
pub unsafe extern "C" fn setImplicitACLCategories(mut c: *mut redisCommand) {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 0 as libc::c_int
        != 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 2 as libc::c_int) as uint64_t;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 1 as libc::c_int
        != 0
        && (*c).acl_categories as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 20 as libc::c_int == 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 1 as libc::c_int) as uint64_t;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 4 as libc::c_int
        != 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | ((1 as libc::c_ulonglong) << 13 as libc::c_int
                | (1 as libc::c_ulonglong) << 17 as libc::c_int)) as uint64_t;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 5 as libc::c_int
        != 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 14 as libc::c_int
        != 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 14 as libc::c_int) as uint64_t;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 8 as libc::c_int
        != 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 16 as libc::c_int) as uint64_t;
    }
    if (*c).acl_categories as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 14 as libc::c_int == 0
    {
        (*c)
            .acl_categories = ((*c).acl_categories as libc::c_ulonglong
            | (1 as libc::c_ulonglong) << 15 as libc::c_int) as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn populateArgsStructure(
    mut args: *mut redisCommandArg,
) -> libc::c_int {
    if args.is_null() {
        return 0 as libc::c_int;
    }
    let mut count: libc::c_int = 0 as libc::c_int;
    while !((*args).name).is_null() {
        if count < 2147483647 as libc::c_int {} else {
            _serverAssert(
                b"count < INT_MAX\0" as *const u8 as *const libc::c_char,
                b"server.c\0" as *const u8 as *const libc::c_char,
                2844 as libc::c_int,
            );
            unreachable!();
        };
        (*args).num_args = populateArgsStructure((*args).subargs);
        count += 1;
        args = args.offset(1);
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn populateCommandStructure(
    mut c: *mut redisCommand,
) -> libc::c_int {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 17 as libc::c_int
        == 0 && server.sentinel_mode != 0
    {
        return -(1 as libc::c_int);
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 18 as libc::c_int
        != 0 && server.sentinel_mode == 0
    {
        return -(1 as libc::c_int);
    }
    setImplicitACLCategories(c);
    (*c).key_specs = ((*c).key_specs_static).as_mut_ptr();
    (*c).key_specs_max = 4 as libc::c_int;
    (*c).latency_histogram = 0 as *mut hdr_histogram;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 4 as libc::c_int {
        if (*((*c).key_specs).offset(i as isize)).begin_search_type as libc::c_uint
            == KSPEC_BS_INVALID as libc::c_int as libc::c_uint
        {
            break;
        }
        (*c).key_specs_num += 1;
        i += 1;
    }
    while !((*c).history).is_null()
        && !((*((*c).history).offset((*c).num_history as isize)).since).is_null()
    {
        (*c).num_history += 1;
    }
    while !((*c).tips).is_null()
        && !(*((*c).tips).offset((*c).num_tips as isize)).is_null()
    {
        (*c).num_tips += 1;
    }
    (*c).num_args = populateArgsStructure((*c).args);
    populateCommandLegacyRangeSpec(c);
    (*c).id = ACLGetCommandID((*c).fullname) as libc::c_int;
    if !((*c).subcommands).is_null() {
        let mut j: libc::c_int = 0 as libc::c_int;
        while !((*((*c).subcommands).offset(j as isize)).declared_name).is_null() {
            let mut sub: *mut redisCommand = ((*c).subcommands).offset(j as isize);
            (*sub)
                .fullname = catSubCommandFullname(
                (*c).declared_name,
                (*sub).declared_name,
            );
            if !(populateCommandStructure(sub) == -(1 as libc::c_int)) {
                commandAddSubcommand(c, sub, (*sub).declared_name);
            }
            j += 1;
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn populateCommandTable() {
    let mut j: libc::c_int = 0;
    let mut c: *mut redisCommand = 0 as *mut redisCommand;
    j = 0 as libc::c_int;
    loop {
        c = redisCommandTable.as_mut_ptr().offset(j as isize);
        if ((*c).declared_name).is_null() {
            break;
        }
        let mut retval1: libc::c_int = 0;
        let mut retval2: libc::c_int = 0;
        (*c).fullname = sdsnew((*c).declared_name);
        if !(populateCommandStructure(c) == -(1 as libc::c_int)) {
            retval1 = dictAdd(
                server.commands,
                sdsdup((*c).fullname) as *mut libc::c_void,
                c as *mut libc::c_void,
            );
            retval2 = dictAdd(
                server.orig_commands,
                sdsdup((*c).fullname) as *mut libc::c_void,
                c as *mut libc::c_void,
            );
            if retval1 == 0 as libc::c_int && retval2 == 0 as libc::c_int {} else {
                _serverAssert(
                    b"retval1 == DICT_OK && retval2 == DICT_OK\0" as *const u8
                        as *const libc::c_char,
                    b"server.c\0" as *const u8 as *const libc::c_char,
                    2936 as libc::c_int,
                );
                unreachable!();
            };
        }
        j += 1;
    };
}
#[no_mangle]
pub unsafe extern "C" fn resetCommandTableStats(mut commands: *mut dict) {
    let mut c: *mut redisCommand = 0 as *mut redisCommand;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    di = dictGetSafeIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        c = (*de).v.val as *mut redisCommand;
        (*c).microseconds = 0 as libc::c_int as libc::c_longlong;
        (*c).calls = 0 as libc::c_int as libc::c_longlong;
        (*c).rejected_calls = 0 as libc::c_int as libc::c_longlong;
        (*c).failed_calls = 0 as libc::c_int as libc::c_longlong;
        if !((*c).latency_histogram).is_null() {
            hdr_close((*c).latency_histogram);
            (*c).latency_histogram = 0 as *mut hdr_histogram;
        }
        if !((*c).subcommands_dict).is_null() {
            resetCommandTableStats((*c).subcommands_dict);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn resetErrorTableStats() {
    raxFreeWithCallback(
        server.errors,
        Some(zfree as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    server.errors = raxNew();
}
#[no_mangle]
pub unsafe extern "C" fn redisOpArrayInit(mut oa: *mut redisOpArray) {
    (*oa).ops = 0 as *mut redisOp;
    (*oa).numops = 0 as libc::c_int;
    (*oa).capacity = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisOpArrayAppend(
    mut oa: *mut redisOpArray,
    mut dbid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut target: libc::c_int,
) -> libc::c_int {
    let mut op: *mut redisOp = 0 as *mut redisOp;
    let mut prev_capacity: libc::c_int = (*oa).capacity;
    if (*oa).numops == 0 as libc::c_int {
        (*oa).capacity = 16 as libc::c_int;
    } else if (*oa).numops >= (*oa).capacity {
        (*oa).capacity *= 2 as libc::c_int;
    }
    if prev_capacity != (*oa).capacity {
        (*oa)
            .ops = zrealloc(
            (*oa).ops as *mut libc::c_void,
            (core::mem::size_of::<redisOp>() as libc::c_ulong)
                .wrapping_mul((*oa).capacity as libc::c_ulong),
        ) as *mut redisOp;
    }
    op = ((*oa).ops).offset((*oa).numops as isize);
    (*op).dbid = dbid;
    (*op).argv = argv;
    (*op).argc = argc;
    (*op).target = target;
    (*oa).numops += 1;
    return (*oa).numops;
}
#[no_mangle]
pub unsafe extern "C" fn redisOpArrayFree(mut oa: *mut redisOpArray) {
    while (*oa).numops != 0 {
        let mut j: libc::c_int = 0;
        let mut op: *mut redisOp = 0 as *mut redisOp;
        (*oa).numops -= 1;
        op = ((*oa).ops).offset((*oa).numops as isize);
        j = 0 as libc::c_int;
        while j < (*op).argc {
            decrRefCount(*((*op).argv).offset(j as isize));
            j += 1;
        }
        zfree((*op).argv as *mut libc::c_void);
    }
    zfree((*oa).ops as *mut libc::c_void);
    redisOpArrayInit(oa);
}
#[no_mangle]
pub unsafe extern "C" fn isContainerCommandBySds(mut s: sds) -> libc::c_int {
    let mut base_cmd: *mut redisCommand = dictFetchValue(
        server.commands,
        s as *const libc::c_void,
    ) as *mut redisCommand;
    let mut has_subcommands: libc::c_int = (!base_cmd.is_null()
        && !((*base_cmd).subcommands_dict).is_null()) as libc::c_int;
    return has_subcommands;
}
#[no_mangle]
pub unsafe extern "C" fn lookupSubcommand(
    mut container: *mut redisCommand,
    mut sub_name: sds,
) -> *mut redisCommand {
    return dictFetchValue((*container).subcommands_dict, sub_name as *const libc::c_void)
        as *mut redisCommand;
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandLogic(
    mut commands: *mut dict,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut strict: libc::c_int,
) -> *mut redisCommand {
    let mut base_cmd: *mut redisCommand = dictFetchValue(
        commands,
        (**argv.offset(0 as libc::c_int as isize)).ptr,
    ) as *mut redisCommand;
    let mut has_subcommands: libc::c_int = (!base_cmd.is_null()
        && !((*base_cmd).subcommands_dict).is_null()) as libc::c_int;
    if argc == 1 as libc::c_int || has_subcommands == 0 {
        if strict != 0 && argc != 1 as libc::c_int {
            return 0 as *mut redisCommand;
        }
        return base_cmd;
    } else {
        if strict != 0 && argc != 2 as libc::c_int {
            return 0 as *mut redisCommand;
        }
        return lookupSubcommand(
            base_cmd,
            (**argv.offset(1 as libc::c_int as isize)).ptr as sds,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommand(
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) -> *mut redisCommand {
    return lookupCommandLogic(server.commands, argv, argc, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandBySdsLogic(
    mut commands: *mut dict,
    mut s: sds,
) -> *mut redisCommand {
    let mut argc: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut strings: *mut sds = sdssplitlen(
        s as *const libc::c_char,
        sdslen(s) as ssize_t,
        b"|\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        &mut argc,
    );
    if strings.is_null() {
        return 0 as *mut redisCommand;
    }
    if argc > 2 as libc::c_int {
        sdsfreesplitres(strings, argc);
        return 0 as *mut redisCommand;
    }
    let vla = argc as usize;
    let mut objects: Vec::<robj> = ::std::vec::from_elem(
        robj {
            type_0_encoding_lru: [0; 4],
            refcount: 0,
            ptr: 0 as *mut libc::c_void,
        },
        vla,
    );
    let vla_0 = argc as usize;
    let mut argv: Vec::<*mut robj> = ::std::vec::from_elem(0 as *mut robj, vla_0);
    j = 0 as libc::c_int;
    while j < argc {
        (*objects.as_mut_ptr().offset(j as isize))
            .refcount = 2147483647 as libc::c_int - 1 as libc::c_int;
        let ref mut fresh12 = *objects.as_mut_ptr().offset(j as isize);
        (*fresh12).set_type_0(0 as libc::c_int as libc::c_uint);
        let ref mut fresh13 = *objects.as_mut_ptr().offset(j as isize);
        (*fresh13).set_encoding(0 as libc::c_int as libc::c_uint);
        let ref mut fresh14 = (*objects.as_mut_ptr().offset(j as isize)).ptr;
        *fresh14 = *strings.offset(j as isize) as *mut libc::c_void;
        let ref mut fresh15 = *argv.as_mut_ptr().offset(j as isize);
        *fresh15 = &mut *objects.as_mut_ptr().offset(j as isize) as *mut robj;
        j += 1;
    }
    let mut cmd: *mut redisCommand = lookupCommandLogic(
        commands,
        argv.as_mut_ptr(),
        argc,
        1 as libc::c_int,
    );
    sdsfreesplitres(strings, argc);
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandBySds(mut s: sds) -> *mut redisCommand {
    return lookupCommandBySdsLogic(server.commands, s);
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandByCStringLogic(
    mut commands: *mut dict,
    mut s: *const libc::c_char,
) -> *mut redisCommand {
    let mut cmd: *mut redisCommand = 0 as *mut redisCommand;
    let mut name: sds = sdsnew(s);
    cmd = lookupCommandBySdsLogic(commands, name);
    sdsfree(name);
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandByCString(
    mut s: *const libc::c_char,
) -> *mut redisCommand {
    return lookupCommandByCStringLogic(server.commands, s);
}
#[no_mangle]
pub unsafe extern "C" fn lookupCommandOrOriginal(
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
) -> *mut redisCommand {
    let mut cmd: *mut redisCommand = lookupCommandLogic(
        server.commands,
        argv,
        argc,
        0 as libc::c_int,
    );
    if cmd.is_null() {
        cmd = lookupCommandLogic(server.orig_commands, argv, argc, 0 as libc::c_int);
    }
    return cmd;
}
#[no_mangle]
pub unsafe extern "C" fn mustObeyClient(mut c: *mut client) -> libc::c_int {
    return ((*c).id == 18446744073709551615 as libc::c_ulong
        || (*c).flags & ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_ulong != 0)
        as libc::c_int;
}
unsafe extern "C" fn shouldPropagate(mut target: libc::c_int) -> libc::c_int {
    if server.replication_allowed == 0 || target == 0 as libc::c_int
        || server.loading != 0
    {
        return 0 as libc::c_int;
    }
    if target & 1 as libc::c_int != 0 {
        if server.aof_state != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
    }
    if target & 2 as libc::c_int != 0 {
        if (server.masterhost).is_null()
            && (!(server.repl_backlog).is_null()
                || (*server.slaves).len != 0 as libc::c_int as libc::c_ulong)
        {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn propagateNow(
    mut dbid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut target: libc::c_int,
) {
    if shouldPropagate(target) == 0 {
        return;
    }
    if !(areClientsPaused() != 0 && server.client_pause_in_transaction == 0) {} else {
        _serverAssert(
            b"!(areClientsPaused() && !server.client_pause_in_transaction)\0"
                as *const u8 as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            3144 as libc::c_int,
        );
        unreachable!();
    };
    if server.aof_state != 0 as libc::c_int && target & 1 as libc::c_int != 0 {
        feedAppendOnlyFile(dbid, argv, argc);
    }
    if target & 2 as libc::c_int != 0 {
        replicationFeedSlaves(server.slaves, dbid, argv, argc);
    }
}
#[no_mangle]
pub unsafe extern "C" fn alsoPropagate(
    mut dbid: libc::c_int,
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut target: libc::c_int,
) {
    let mut argvcopy: *mut *mut robj = 0 as *mut *mut robj;
    let mut j: libc::c_int = 0;
    if shouldPropagate(target) == 0 {
        return;
    }
    argvcopy = zmalloc(
        (core::mem::size_of::<*mut robj>() as libc::c_ulong)
            .wrapping_mul(argc as libc::c_ulong),
    ) as *mut *mut robj;
    j = 0 as libc::c_int;
    while j < argc {
        let ref mut fresh16 = *argvcopy.offset(j as isize);
        *fresh16 = *argv.offset(j as isize);
        incrRefCount(*argv.offset(j as isize));
        j += 1;
    }
    redisOpArrayAppend(&mut server.also_propagate, dbid, argvcopy, argc, target);
}
#[no_mangle]
pub unsafe extern "C" fn forceCommandPropagation(
    mut c: *mut client,
    mut flags: libc::c_int,
) {
    if (*(*c).cmd).flags as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 0 as libc::c_int
            | (1 as libc::c_ulonglong) << 16 as libc::c_int) != 0
    {} else {
        _serverAssert(
            b"c->cmd->flags & (CMD_WRITE | CMD_MAY_REPLICATE)\0" as *const u8
                as *const libc::c_char,
            b"server.c\0" as *const u8 as *const libc::c_char,
            3182 as libc::c_int,
        );
        unreachable!();
    };
    if flags & 2 as libc::c_int != 0 {
        (*c).flags |= ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong;
    }
    if flags & 1 as libc::c_int != 0 {
        (*c).flags |= ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong;
    }
}
#[no_mangle]
pub unsafe extern "C" fn preventCommandPropagation(mut c: *mut client) {
    (*c).flags
        |= ((1 as libc::c_int) << 19 as libc::c_int
            | (1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn preventCommandAOF(mut c: *mut client) {
    (*c).flags |= ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn preventCommandReplication(mut c: *mut client) {
    (*c).flags |= ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong;
}
#[no_mangle]
pub unsafe extern "C" fn slowlogPushCurrentCommand(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
    mut duration: ustime_t,
) {
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 12 as libc::c_int
        != 0
    {
        return;
    }
    let mut argv: *mut *mut robj = if !((*c).original_argv).is_null() {
        (*c).original_argv
    } else {
        (*c).argv
    };
    let mut argc: libc::c_int = if !((*c).original_argv).is_null() {
        (*c).original_argc
    } else {
        (*c).argc
    };
    slowlogPushEntryIfNeeded(c, argv, argc, duration);
}
#[no_mangle]
pub unsafe extern "C" fn updateCommandLatencyHistogram(
    mut latency_histogram: *mut *mut hdr_histogram,
    mut duration_hist: int64_t,
) {
    if duration_hist < 1 as libc::c_long {
        duration_hist = 1 as libc::c_long;
    }
    if duration_hist > 1000000000 as libc::c_long {
        duration_hist = 1000000000 as libc::c_long;
    }
    if (*latency_histogram).is_null() {
        hdr_init(
            1 as libc::c_long,
            1000000000 as libc::c_long,
            2 as libc::c_int,
            latency_histogram,
        );
    }
    hdr_record_value(*latency_histogram, duration_hist);
}
#[no_mangle]
pub unsafe extern "C" fn propagatePendingCommands() {
    if server.also_propagate.numops == 0 as libc::c_int {
        return;
    }
    let mut j: libc::c_int = 0;
    let mut rop: *mut redisOp = 0 as *mut redisOp;
    let mut multi_emitted: libc::c_int = 0 as libc::c_int;
    if server.also_propagate.numops > 1 as libc::c_int && server.propagate_no_multi == 0
    {
        let mut multi_dbid: libc::c_int = (*(server.also_propagate.ops)
            .offset(0 as libc::c_int as isize))
            .dbid;
        propagateNow(
            multi_dbid,
            &mut shared.multi,
            1 as libc::c_int,
            1 as libc::c_int | 2 as libc::c_int,
        );
        multi_emitted = 1 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < server.also_propagate.numops {
        rop = &mut *(server.also_propagate.ops).offset(j as isize) as *mut redisOp;
        if (*rop).target != 0 {} else {
            _serverAssert(
                b"rop->target\0" as *const u8 as *const libc::c_char,
                b"server.c\0" as *const u8 as *const libc::c_char,
                3257 as libc::c_int,
            );
            unreachable!();
        };
        propagateNow((*rop).dbid, (*rop).argv, (*rop).argc, (*rop).target);
        j += 1;
    }
    if multi_emitted != 0 {
        let mut exec_dbid: libc::c_int = (*(server.also_propagate.ops)
            .offset((server.also_propagate.numops - 1 as libc::c_int) as isize))
            .dbid;
        propagateNow(
            exec_dbid,
            &mut shared.exec,
            1 as libc::c_int,
            1 as libc::c_int | 2 as libc::c_int,
        );
    }
    redisOpArrayFree(&mut server.also_propagate);
}
#[no_mangle]
pub unsafe extern "C" fn incrCommandStatsOnError(
    mut cmd: *mut redisCommand,
    mut flags: libc::c_int,
) -> libc::c_int {
    static mut prev_err_count: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut res: libc::c_int = 0 as libc::c_int;
    if !cmd.is_null() {
        if server.stat_total_error_replies - prev_err_count
            > 0 as libc::c_int as libc::c_longlong
        {
            if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
                (*cmd).rejected_calls += 1;
                res = 1 as libc::c_int;
            } else if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
                (*cmd).failed_calls += 1;
                res = 1 as libc::c_int;
            }
        }
    }
    prev_err_count = server.stat_total_error_replies;
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn call(mut c: *mut client, mut flags: libc::c_int) {
    let mut dirty: libc::c_longlong = 0;
    let mut client_old_flags: uint64_t = (*c).flags;
    let mut real_cmd: *mut redisCommand = (*c).realcmd;
    (*c).flags
        &= !((1 as libc::c_int) << 14 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | ((1 as libc::c_int) << 19 as libc::c_int
                | (1 as libc::c_int) << 20 as libc::c_int)) as libc::c_ulong;
    let mut prev_core_propagates: libc::c_int = server.core_propagates;
    if server.core_propagates == 0 && flags & (1 as libc::c_int) << 4 as libc::c_int == 0
    {
        server.core_propagates = 1 as libc::c_int;
    }
    dirty = server.dirty;
    incrCommandStatsOnError(0 as *mut redisCommand, 0 as libc::c_int);
    let call_timer: libc::c_longlong = ustime();
    let fresh17 = server.fixed_time_expire;
    server.fixed_time_expire = server.fixed_time_expire + 1;
    if fresh17 == 0 as libc::c_int as libc::c_long {
        updateCachedTimeWithUs(0 as libc::c_int, call_timer);
    }
    let mut monotonic_start: monotime = 0 as libc::c_int as monotime;
    if monotonicGetType() as libc::c_uint
        == MONOTONIC_CLOCK_HW as libc::c_int as libc::c_uint
    {
        monotonic_start = getMonotonicUs.expect("non-null function pointer")();
    }
    server.in_nested_call += 1;
    ((*(*c).cmd).proc_0).expect("non-null function pointer")(c);
    server.in_nested_call -= 1;
    let mut duration: ustime_t = 0;
    if monotonicGetType() as libc::c_uint
        == MONOTONIC_CLOCK_HW as libc::c_int as libc::c_uint
    {
        duration = (getMonotonicUs.expect("non-null function pointer")())
            .wrapping_sub(monotonic_start) as ustime_t;
    } else {
        duration = ustime() - call_timer;
    }
    (*c).duration = duration as libc::c_long;
    dirty = server.dirty - dirty;
    if dirty < 0 as libc::c_int as libc::c_longlong {
        dirty = 0 as libc::c_int as libc::c_longlong;
    }
    if incrCommandStatsOnError(real_cmd, (1 as libc::c_int) << 1 as libc::c_int) == 0
        && !((*c).deferred_reply_errors).is_null()
    {
        (*real_cmd).failed_calls += 1;
    }
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 40 as libc::c_int
        != 0
    {
        (*c)
            .flags = ((*c).flags as libc::c_ulonglong
            & !((1 as libc::c_ulonglong) << 40 as libc::c_int)) as uint64_t;
        (*c).flags |= ((1 as libc::c_int) << 6 as libc::c_int) as libc::c_ulong;
    }
    if server.loading != 0
        && (*c).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0
    {
        flags
            &= !((1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int);
    }
    if (*c).flags & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0
        && !(server.script_caller).is_null()
    {
        if (*c).flags & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong != 0 {
            (*server.script_caller).flags
                |= ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong;
        }
        if (*c).flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong != 0 {
            (*server.script_caller).flags
                |= ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong;
        }
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0 {
        let mut latency_event: *mut libc::c_char = (if (*real_cmd).flags
            as libc::c_ulonglong & (1 as libc::c_ulonglong) << 14 as libc::c_int != 0
        {
            b"fast-command\0" as *const u8 as *const libc::c_char
        } else {
            b"command\0" as *const u8 as *const libc::c_char
        }) as *mut libc::c_char;
        if server.latency_monitor_threshold != 0
            && duration / 1000 as libc::c_int as libc::c_longlong
                >= server.latency_monitor_threshold
        {
            latencyAddSample(
                latency_event,
                duration / 1000 as libc::c_int as libc::c_longlong,
            );
        }
    }
    if flags & (1 as libc::c_int) << 0 as libc::c_int != 0
        && (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong == 0
    {
        slowlogPushCurrentCommand(c, real_cmd, duration);
    }
    if (*(*c).cmd).flags as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 11 as libc::c_int
            | (1 as libc::c_ulonglong) << 4 as libc::c_int) == 0
    {
        let mut argv: *mut *mut robj = if !((*c).original_argv).is_null() {
            (*c).original_argv
        } else {
            (*c).argv
        };
        let mut argc: libc::c_int = if !((*c).original_argv).is_null() {
            (*c).original_argc
        } else {
            (*c).argc
        };
        replicationFeedMonitors(c, server.monitors, (*(*c).db).id, argv, argc);
    }
    if (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong == 0 {
        freeClientOriginalArgv(c);
    }
    if flags & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        (*real_cmd).microseconds += duration;
        (*real_cmd).calls += 1;
        if server.latency_tracking_enabled != 0
            && (*c).flags & ((1 as libc::c_int) << 4 as libc::c_int) as libc::c_ulong
                == 0
        {
            updateCommandLatencyHistogram(
                &mut (*real_cmd).latency_histogram,
                (duration * 1000 as libc::c_int as libc::c_longlong) as int64_t,
            );
        }
    }
    if flags
        & ((1 as libc::c_int) << 2 as libc::c_int
            | (1 as libc::c_int) << 3 as libc::c_int) != 0
        && (*c).flags
            & ((1 as libc::c_int) << 19 as libc::c_int
                | (1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
            != ((1 as libc::c_int) << 19 as libc::c_int
                | (1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong
        && (*(*c).cmd).proc_0
            != Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 3 as libc::c_int == 0
    {
        let mut propagate_flags: libc::c_int = 0 as libc::c_int;
        if dirty != 0 {
            propagate_flags |= 1 as libc::c_int | 2 as libc::c_int;
        }
        if (*c).flags & ((1 as libc::c_int) << 15 as libc::c_int) as libc::c_ulong != 0 {
            propagate_flags |= 2 as libc::c_int;
        }
        if (*c).flags & ((1 as libc::c_int) << 14 as libc::c_int) as libc::c_ulong != 0 {
            propagate_flags |= 1 as libc::c_int;
        }
        if (*c).flags & ((1 as libc::c_int) << 20 as libc::c_int) as libc::c_ulong != 0
            || flags & (1 as libc::c_int) << 3 as libc::c_int == 0
        {
            propagate_flags &= !(2 as libc::c_int);
        }
        if (*c).flags & ((1 as libc::c_int) << 19 as libc::c_int) as libc::c_ulong != 0
            || flags & (1 as libc::c_int) << 2 as libc::c_int == 0
        {
            propagate_flags &= !(1 as libc::c_int);
        }
        if propagate_flags != 0 as libc::c_int {
            alsoPropagate((*(*c).db).id, (*c).argv, (*c).argc, propagate_flags);
        }
    }
    (*c).flags
        &= !((1 as libc::c_int) << 14 as libc::c_int
            | (1 as libc::c_int) << 15 as libc::c_int
            | ((1 as libc::c_int) << 19 as libc::c_int
                | (1 as libc::c_int) << 20 as libc::c_int)) as libc::c_ulong;
    (*c).flags
        |= client_old_flags
            & ((1 as libc::c_int) << 14 as libc::c_int
                | (1 as libc::c_int) << 15 as libc::c_int
                | ((1 as libc::c_int) << 19 as libc::c_int
                    | (1 as libc::c_int) << 20 as libc::c_int)) as libc::c_ulong;
    if (*(*c).cmd).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 1 as libc::c_int != 0
        && (*(*c).cmd).proc_0
            != Some(evalRoCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(evalShaRoCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(fcallroCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        let mut caller: *mut client = if (*c).flags
            & ((1 as libc::c_int) << 8 as libc::c_int) as libc::c_ulong != 0
            && !(server.script_caller).is_null()
        {
            server.script_caller
        } else {
            c
        };
        if (*caller).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 31 as libc::c_int != 0
            && (*caller).flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 33 as libc::c_int == 0
        {
            trackingRememberKeys(caller);
        }
    }
    server.fixed_time_expire -= 1;
    server.stat_numcommands += 1;
    let mut zmalloc_used: size_t = zmalloc_used_memory();
    if zmalloc_used > server.stat_peak_memory {
        server.stat_peak_memory = zmalloc_used;
    }
    afterCommand(c);
    if server.in_exec == 0 && server.client_pause_in_transaction != 0 {
        server.client_pause_in_transaction = 0 as libc::c_int;
    }
    server.core_propagates = prev_core_propagates;
}
#[no_mangle]
pub unsafe extern "C" fn rejectCommand(mut c: *mut client, mut reply: *mut robj) {
    flagTransaction(c);
    if !((*c).cmd).is_null() {
        (*(*c).cmd).rejected_calls += 1;
    }
    if !((*c).cmd).is_null()
        && (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        execCommandAbort(c, (*reply).ptr as sds);
    } else {
        addReplyErrorObject(c, reply);
    };
}
#[no_mangle]
pub unsafe extern "C" fn rejectCommandSds(mut c: *mut client, mut s: sds) {
    flagTransaction(c);
    if !((*c).cmd).is_null() {
        (*(*c).cmd).rejected_calls += 1;
    }
    if !((*c).cmd).is_null()
        && (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        execCommandAbort(c, s);
        sdsfree(s);
    } else {
        addReplyErrorSds(c, s);
    };
}
#[no_mangle]
pub unsafe extern "C" fn rejectCommandFormat(
    mut c: *mut client,
    mut fmt: *const libc::c_char,
    mut args: ...
) {
    let mut ap: core::ffi::VaListImpl;
    ap = args.clone();
    let mut s: sds = sdscatvprintf(sdsempty(), fmt, ap.as_va_list());
    sdsmapchars(
        s,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        b"  \0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    rejectCommandSds(c, s);
}
#[no_mangle]
pub unsafe extern "C" fn afterCommand(mut c: *mut client) {
    if server.in_nested_call == 0 {
        if server.core_propagates != 0 {
            propagatePendingCommands();
        }
        trackingHandlePendingKeyInvalidations();
    }
}
#[no_mangle]
pub unsafe extern "C" fn commandCheckExistence(
    mut c: *mut client,
    mut err: *mut sds,
) -> libc::c_int {
    if !((*c).cmd).is_null() {
        return 1 as libc::c_int;
    }
    if err.is_null() {
        return 0 as libc::c_int;
    }
    if isContainerCommandBySds(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as sds,
    ) != 0
    {
        let mut cmd: sds = sdsnew(
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
        );
        sdstoupper(cmd);
        *err = sdsnew(0 as *const libc::c_char);
        *err = sdscatprintf(
            *err,
            b"unknown subcommand '%.128s'. Try %s HELP.\0" as *const u8
                as *const libc::c_char,
            (**((*c).argv).offset(1 as libc::c_int as isize)).ptr as *mut libc::c_char,
            cmd,
        );
        sdsfree(cmd);
    } else {
        let mut args: sds = sdsempty();
        let mut i: libc::c_int = 0;
        i = 1 as libc::c_int;
        while i < (*c).argc && sdslen(args) < 128 as libc::c_int as libc::c_ulong {
            args = sdscatprintf(
                args,
                b"'%.*s' \0" as *const u8 as *const libc::c_char,
                128 as libc::c_int - sdslen(args) as libc::c_int,
                (**((*c).argv).offset(i as isize)).ptr as *mut libc::c_char,
            );
            i += 1;
        }
        *err = sdsnew(0 as *const libc::c_char);
        *err = sdscatprintf(
            *err,
            b"unknown command '%.128s', with args beginning with: %s\0" as *const u8
                as *const libc::c_char,
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *mut libc::c_char,
            args,
        );
        sdsfree(args);
    }
    sdsmapchars(
        *err,
        b"\r\n\0" as *const u8 as *const libc::c_char,
        b"  \0" as *const u8 as *const libc::c_char,
        2 as libc::c_int as size_t,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn commandCheckArity(
    mut c: *mut client,
    mut err: *mut sds,
) -> libc::c_int {
    if (*(*c).cmd).arity > 0 as libc::c_int && (*(*c).cmd).arity != (*c).argc
        || (*c).argc < -(*(*c).cmd).arity
    {
        if !err.is_null() {
            *err = sdsnew(0 as *const libc::c_char);
            *err = sdscatprintf(
                *err,
                b"wrong number of arguments for '%s' command\0" as *const u8
                    as *const libc::c_char,
                (*(*c).cmd).fullname,
            );
        }
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getCommandFlags(mut c: *mut client) -> uint64_t {
    let mut cmd_flags: uint64_t = (*(*c).cmd).flags;
    if (*(*c).cmd).proc_0
        == Some(fcallCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(fcallroCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        cmd_flags = fcallGetCommandFlags(c, cmd_flags);
    } else if (*(*c).cmd).proc_0
        == Some(evalCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(evalRoCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(evalShaCommand as unsafe extern "C" fn(*mut client) -> ())
        || (*(*c).cmd).proc_0
            == Some(evalShaRoCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        cmd_flags = evalGetCommandFlags(c, cmd_flags);
    }
    return cmd_flags;
}
#[no_mangle]
pub unsafe extern "C" fn processCommand(mut c: *mut client) -> libc::c_int {
    if scriptIsTimedout() == 0 {
        if server.in_exec == 0 {} else {
            _serverAssert(
                b"!server.in_exec\0" as *const u8 as *const libc::c_char,
                b"server.c\0" as *const u8 as *const libc::c_char,
                3673 as libc::c_int,
            );
            unreachable!();
        };
        if scriptIsRunning() == 0 {} else {
            _serverAssert(
                b"!scriptIsRunning()\0" as *const u8 as *const libc::c_char,
                b"server.c\0" as *const u8 as *const libc::c_char,
                3674 as libc::c_int,
            );
            unreachable!();
        };
    }
    moduleCallCommandFilters(c);
    if strcasecmp(
        (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
        b"host:\0" as *const u8 as *const libc::c_char,
    ) == 0
        || strcasecmp(
            (**((*c).argv).offset(0 as libc::c_int as isize)).ptr as *const libc::c_char,
            b"post\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        securityWarningCommand(c);
        return -(1 as libc::c_int);
    }
    if server.busy_module_yield_flags != 0 as libc::c_int
        && server.busy_module_yield_flags & (1 as libc::c_int) << 1 as libc::c_int == 0
    {
        (*c).bpop.timeout = 0 as libc::c_int as mstime_t;
        blockClient(c, 6 as libc::c_int);
        return 0 as libc::c_int;
    }
    (*c).realcmd = lookupCommand((*c).argv, (*c).argc);
    (*c).lastcmd = (*c).realcmd;
    (*c).cmd = (*c).lastcmd;
    let mut err: sds = 0 as *mut libc::c_char;
    if commandCheckExistence(c, &mut err) == 0 {
        rejectCommandSds(c, err);
        return 0 as libc::c_int;
    }
    if commandCheckArity(c, &mut err) == 0 {
        rejectCommandSds(c, err);
        return 0 as libc::c_int;
    }
    if (*(*c).cmd).flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 20 as libc::c_int != 0
    {
        if (*(*c).cmd).proc_0
            == Some(debugCommand as unsafe extern "C" fn(*mut client) -> ())
            && allowProtectedAction(server.enable_debug_cmd, c) == 0
            || (*(*c).cmd).proc_0
                == Some(moduleCommand as unsafe extern "C" fn(*mut client) -> ())
                && allowProtectedAction(server.enable_module_cmd, c) == 0
        {
            rejectCommandFormat(
                c,
                b"%s command not allowed. If the %s option is set to \"local\", you can run it from a local connection, otherwise you need to set this option in the configuration file, and then restart the server.\0"
                    as *const u8 as *const libc::c_char,
                if (*(*c).cmd).proc_0
                    == Some(debugCommand as unsafe extern "C" fn(*mut client) -> ())
                {
                    b"DEBUG\0" as *const u8 as *const libc::c_char
                } else {
                    b"MODULE\0" as *const u8 as *const libc::c_char
                },
                if (*(*c).cmd).proc_0
                    == Some(debugCommand as unsafe extern "C" fn(*mut client) -> ())
                {
                    b"enable-debug-command\0" as *const u8 as *const libc::c_char
                } else {
                    b"enable-module-command\0" as *const u8 as *const libc::c_char
                },
            );
            return 0 as libc::c_int;
        }
    }
    let mut cmd_flags: uint64_t = getCommandFlags(c);
    let mut is_read_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 1 as libc::c_int != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 1 as libc::c_int != 0) as libc::c_int;
    let mut is_write_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 0 as libc::c_int != 0) as libc::c_int;
    let mut is_denyoom_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 2 as libc::c_int != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 2 as libc::c_int != 0) as libc::c_int;
    let mut is_denystale_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 10 as libc::c_int == 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_inv_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 10 as libc::c_int != 0) as libc::c_int;
    let mut is_denyloading_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 9 as libc::c_int == 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_inv_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 9 as libc::c_int != 0) as libc::c_int;
    let mut is_may_replicate_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & ((1 as libc::c_ulonglong) << 0 as libc::c_int
            | (1 as libc::c_ulonglong) << 16 as libc::c_int) != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & ((1 as libc::c_ulonglong) << 0 as libc::c_int
                    | (1 as libc::c_ulonglong) << 16 as libc::c_int) != 0)
        as libc::c_int;
    let mut is_deny_async_loading_command: libc::c_int = (cmd_flags as libc::c_ulonglong
        & (1 as libc::c_ulonglong) << 23 as libc::c_int != 0
        || (*(*c).cmd).proc_0
            == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*c).mstate.cmd_flags as libc::c_ulonglong
                & (1 as libc::c_ulonglong) << 23 as libc::c_int != 0) as libc::c_int;
    let mut obey_client: libc::c_int = mustObeyClient(c);
    if authRequired(c) != 0 {
        if (*(*c).cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 15 as libc::c_int == 0
        {
            rejectCommand(c, shared.noautherr);
            return 0 as libc::c_int;
        }
    }
    if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
        && (*(*c).cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 24 as libc::c_int != 0
    {
        rejectCommandFormat(
            c,
            b"Command not allowed inside a transaction\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    let mut acl_errpos: libc::c_int = 0;
    let mut acl_retval: libc::c_int = ACLCheckAllPerm(c, &mut acl_errpos);
    if acl_retval != 0 as libc::c_int {
        addACLLogEntry(
            c,
            acl_retval,
            if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong
                != 0
            {
                2 as libc::c_int
            } else {
                0 as libc::c_int
            },
            acl_errpos,
            0 as sds,
            0 as sds,
        );
        match acl_retval {
            1 => {
                rejectCommandFormat(
                    c,
                    b"-NOPERM this user has no permissions to run the '%s' command\0"
                        as *const u8 as *const libc::c_char,
                    (*(*c).cmd).fullname,
                );
            }
            2 => {
                rejectCommandFormat(
                    c,
                    b"-NOPERM this user has no permissions to access one of the keys used as arguments\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            4 => {
                rejectCommandFormat(
                    c,
                    b"-NOPERM this user has no permissions to access one of the channels used as arguments\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            _ => {
                rejectCommandFormat(
                    c,
                    b"no permission\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        return 0 as libc::c_int;
    }
    if server.cluster_enabled != 0 && mustObeyClient(c) == 0
        && !((*(*c).cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 25 as libc::c_int == 0
            && (*(*c).cmd).key_specs_num == 0 as libc::c_int
            && (*(*c).cmd).proc_0
                != Some(execCommand as unsafe extern "C" fn(*mut client) -> ()))
    {
        let mut error_code: libc::c_int = 0;
        let mut n: *mut clusterNode = getNodeByQuery(
            c,
            (*c).cmd,
            (*c).argv,
            (*c).argc,
            &mut (*c).slot,
            &mut error_code,
        );
        if n.is_null() || n != (*server.cluster).myself {
            if (*(*c).cmd).proc_0
                == Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            {
                discardTransaction(c);
            } else {
                flagTransaction(c);
            }
            clusterRedirectClient(c, n, (*c).slot, error_code);
            (*(*c).cmd).rejected_calls += 1;
            return 0 as libc::c_int;
        }
    }
    evictClients();
    if (server.current_client).is_null() {
        return -(1 as libc::c_int);
    }
    if server.maxmemory != 0 && isInsideYieldingLongCommand() == 0 {
        let mut out_of_memory: libc::c_int = (performEvictions() == 2 as libc::c_int)
            as libc::c_int;
        trackingHandlePendingKeyInvalidations();
        if (server.current_client).is_null() {
            return -(1 as libc::c_int);
        }
        let mut reject_cmd_on_oom: libc::c_int = is_denyoom_command;
        if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
            && (*(*c).cmd).proc_0
                != Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*(*c).cmd).proc_0
                != Some(discardCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*(*c).cmd).proc_0
                != Some(quitCommand as unsafe extern "C" fn(*mut client) -> ())
            && (*(*c).cmd).proc_0
                != Some(resetCommand as unsafe extern "C" fn(*mut client) -> ())
        {
            reject_cmd_on_oom = 1 as libc::c_int;
        }
        if out_of_memory != 0 && reject_cmd_on_oom != 0 {
            rejectCommand(c, shared.oomerr);
            return 0 as libc::c_int;
        }
        server.pre_command_oom_state = out_of_memory;
    }
    if server.tracking_clients != 0 {
        trackingLimitUsedSlots();
    }
    let mut deny_write_type: libc::c_int = writeCommandsDeniedByDiskError();
    if deny_write_type != 0 as libc::c_int
        && (is_write_command != 0
            || (*(*c).cmd).proc_0
                == Some(pingCommand as unsafe extern "C" fn(*mut client) -> ()))
    {
        if obey_client != 0 {
            if server.repl_ignore_disk_write_error == 0
                && (*(*c).cmd).proc_0
                    != Some(pingCommand as unsafe extern "C" fn(*mut client) -> ())
            {
                _serverPanic(
                    b"server.c\0" as *const u8 as *const libc::c_char,
                    3878 as libc::c_int,
                    b"Replica was unable to write command to disk.\0" as *const u8
                        as *const libc::c_char,
                );
                unreachable!();
            } else {
                static mut last_log_time_ms: mstime_t = 0 as libc::c_int as mstime_t;
                let log_interval_ms: mstime_t = 10000 as libc::c_int as mstime_t;
                if server.mstime > last_log_time_ms + log_interval_ms {
                    last_log_time_ms = server.mstime;
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Replica is applying a command even though it is unable to write to disk.\0"
                                as *const u8 as *const libc::c_char,
                        );
                    }
                }
            }
        } else {
            let mut err_0: sds = writeCommandsGetDiskErrorMessage(deny_write_type);
            sdssubstr(
                err_0,
                0 as libc::c_int as size_t,
                (sdslen(err_0)).wrapping_sub(2 as libc::c_int as libc::c_ulong),
            );
            rejectCommandSds(c, err_0);
            return 0 as libc::c_int;
        }
    }
    if is_write_command != 0 && checkGoodReplicasStatus() == 0 {
        rejectCommand(c, shared.noreplicaserr);
        return 0 as libc::c_int;
    }
    if !(server.masterhost).is_null() && server.repl_slave_ro != 0 && obey_client == 0
        && is_write_command != 0
    {
        rejectCommand(c, shared.roslaveerr);
        return 0 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
        && (*c).resp == 2 as libc::c_int
        && (*(*c).cmd).proc_0
            != Some(pingCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(subscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(ssubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(unsubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(sunsubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(psubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(punsubscribeCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(quitCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(resetCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        rejectCommandFormat(
            c,
            b"Can't execute '%s': only (P|S)SUBSCRIBE / (P|S)UNSUBSCRIBE / PING / QUIT / RESET are allowed in this context\0"
                as *const u8 as *const libc::c_char,
            (*(*c).cmd).fullname,
        );
        return 0 as libc::c_int;
    }
    if !(server.masterhost).is_null()
        && server.repl_state != REPL_STATE_CONNECTED as libc::c_int
        && server.repl_serve_stale_data == 0 as libc::c_int && is_denystale_command != 0
    {
        rejectCommand(c, shared.masterdownerr);
        return 0 as libc::c_int;
    }
    if server.loading != 0 && server.async_loading == 0 && is_denyloading_command != 0 {
        rejectCommand(c, shared.loadingerr);
        return 0 as libc::c_int;
    }
    if server.async_loading != 0 && is_deny_async_loading_command != 0 {
        rejectCommand(c, shared.loadingerr);
        return 0 as libc::c_int;
    }
    if isInsideYieldingLongCommand() != 0
        && (*(*c).cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 26 as libc::c_int == 0
    {
        if server.busy_module_yield_flags != 0
            && !(server.busy_module_yield_reply).is_null()
        {
            rejectCommandFormat(
                c,
                b"-BUSY %s\0" as *const u8 as *const libc::c_char,
                server.busy_module_yield_reply,
            );
        } else if server.busy_module_yield_flags != 0 {
            rejectCommand(c, shared.slowmoduleerr);
        } else if scriptIsEval() != 0 {
            rejectCommand(c, shared.slowevalerr);
        } else {
            rejectCommand(c, shared.slowscripterr);
        }
        return 0 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0
        && (is_may_replicate_command != 0 || is_write_command != 0
            || is_read_command != 0)
    {
        rejectCommandFormat(
            c,
            b"Replica can't interact with the keyspace\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong == 0
        && (server.client_pause_type as libc::c_uint
            == CLIENT_PAUSE_ALL as libc::c_int as libc::c_uint
            || server.client_pause_type as libc::c_uint
                == CLIENT_PAUSE_WRITE as libc::c_int as libc::c_uint
                && is_may_replicate_command != 0)
    {
        (*c).bpop.timeout = 0 as libc::c_int as mstime_t;
        blockClient(c, 6 as libc::c_int);
        return 0 as libc::c_int;
    }
    if (*c).flags & ((1 as libc::c_int) << 3 as libc::c_int) as libc::c_ulong != 0
        && (*(*c).cmd).proc_0
            != Some(execCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(discardCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(multiCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(watchCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(quitCommand as unsafe extern "C" fn(*mut client) -> ())
        && (*(*c).cmd).proc_0
            != Some(resetCommand as unsafe extern "C" fn(*mut client) -> ())
    {
        queueMultiCommand(c, cmd_flags);
        addReply(c, shared.queued);
    } else {
        call(
            c,
            (1 as libc::c_int) << 0 as libc::c_int
                | (1 as libc::c_int) << 1 as libc::c_int
                | ((1 as libc::c_int) << 2 as libc::c_int
                    | (1 as libc::c_int) << 3 as libc::c_int),
        );
        (*c).woff = server.master_repl_offset;
        if (*server.ready_keys).len != 0 {
            handleClientsBlockedOnKeys();
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn incrementErrorCount(
    mut fullerr: *const libc::c_char,
    mut namelen: size_t,
) {
    let mut error: *mut redisError = raxFind(
        server.errors,
        fullerr as *mut libc::c_uchar,
        namelen,
    ) as *mut redisError;
    if error == raxNotFound as *mut redisError {
        error = zmalloc(core::mem::size_of::<redisError>() as libc::c_ulong)
            as *mut redisError;
        (*error).count = 0 as libc::c_int as libc::c_longlong;
        raxInsert(
            server.errors,
            fullerr as *mut libc::c_uchar,
            namelen,
            error as *mut libc::c_void,
            0 as *mut *mut libc::c_void,
        );
    }
    (*error).count += 1;
}
#[no_mangle]
pub unsafe extern "C" fn closeListeningSockets(mut unlink_unix_socket: libc::c_int) {
    let mut j: libc::c_int = 0;
    j = 0 as libc::c_int;
    while j < server.ipfd.count {
        close(server.ipfd.fd[j as usize]);
        j += 1;
    }
    j = 0 as libc::c_int;
    while j < server.tlsfd.count {
        close(server.tlsfd.fd[j as usize]);
        j += 1;
    }
    if server.sofd != -(1 as libc::c_int) {
        close(server.sofd);
    }
    if server.cluster_enabled != 0 {
        j = 0 as libc::c_int;
        while j < server.cfd.count {
            close(server.cfd.fd[j as usize]);
            j += 1;
        }
    }
    if unlink_unix_socket != 0 && !(server.unixsocket).is_null() {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Removing the unix socket file.\0" as *const u8 as *const libc::c_char,
            );
        }
        if unlink(server.unixsocket) != 0 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Error removing the unix socket file: %s\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn prepareForShutdown(mut flags: libc::c_int) -> libc::c_int {
    if isShutdownInitiated() != 0 {
        return -(1 as libc::c_int);
    }
    if server.loading != 0 || server.sentinel_mode != 0 {
        flags = flags & !(1 as libc::c_int) | 2 as libc::c_int;
    }
    server.shutdown_flags = flags;
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"User requested shutdown...\0" as *const u8 as *const libc::c_char,
        );
    }
    if server.supervised_mode == 2 as libc::c_int {
        redisCommunicateSystemd(b"STOPPING=1\n\0" as *const u8 as *const libc::c_char);
    }
    if flags & 4 as libc::c_int == 0 && server.shutdown_timeout != 0 as libc::c_int
        && isReadyToShutdown() == 0
    {
        server
            .shutdown_mstime = server.mstime
            + (server.shutdown_timeout * 1000 as libc::c_int) as libc::c_longlong;
        if areClientsPaused() == 0 {
            sendGetackToReplicas();
        }
        pauseClients(
            PAUSE_DURING_SHUTDOWN,
            9223372036854775807 as libc::c_longlong,
            CLIENT_PAUSE_WRITE,
        );
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Waiting for replicas before shutting down.\0" as *const u8
                    as *const libc::c_char,
            );
        }
        return -(1 as libc::c_int);
    }
    return finishShutdown();
}
#[inline]
unsafe extern "C" fn isShutdownInitiated() -> libc::c_int {
    return (server.shutdown_mstime != 0 as libc::c_int as libc::c_longlong)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isReadyToShutdown() -> libc::c_int {
    if (*server.slaves).len == 0 as libc::c_int as libc::c_ulong {
        return 1 as libc::c_int;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.slaves, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut replica: *mut client = (*ln).value as *mut client;
        if (*replica).repl_ack_off != server.master_repl_offset {
            return 0 as libc::c_int;
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn cancelShutdown() {
    core::ptr::write_volatile(
        &mut server.shutdown_asap as *mut sig_atomic_t,
        0 as libc::c_int,
    );
    server.shutdown_flags = 0 as libc::c_int;
    server.shutdown_mstime = 0 as libc::c_int as mstime_t;
    server.last_sig_received = 0 as libc::c_int;
    replyToClientsBlockedOnShutdown();
    unpauseClients(PAUSE_DURING_SHUTDOWN);
}
#[no_mangle]
pub unsafe extern "C" fn abortShutdown() -> libc::c_int {
    if isShutdownInitiated() != 0 {
        cancelShutdown();
    } else if server.shutdown_asap != 0 {
        core::ptr::write_volatile(
            &mut server.shutdown_asap as *mut sig_atomic_t,
            0 as libc::c_int,
        );
    } else {
        return -(1 as libc::c_int)
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"Shutdown manually aborted.\0" as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn finishShutdown() -> libc::c_int {
    let mut current_block: u64;
    let mut save: libc::c_int = server.shutdown_flags & 1 as libc::c_int;
    let mut nosave: libc::c_int = server.shutdown_flags & 2 as libc::c_int;
    let mut force: libc::c_int = server.shutdown_flags & 8 as libc::c_int;
    let mut replicas_iter: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut replicas_list_node: *mut listNode = 0 as *mut listNode;
    let mut num_replicas: libc::c_int = 0 as libc::c_int;
    let mut num_lagging_replicas: libc::c_int = 0 as libc::c_int;
    listRewind(server.slaves, &mut replicas_iter);
    loop {
        replicas_list_node = listNext(&mut replicas_iter);
        if replicas_list_node.is_null() {
            break;
        }
        let mut replica: *mut client = (*replicas_list_node).value as *mut client;
        num_replicas += 1;
        if (*replica).repl_ack_off != server.master_repl_offset {
            num_lagging_replicas += 1;
            let mut lag: libc::c_long = (if (*replica).replstate == 9 as libc::c_int {
                time(0 as *mut time_t) as libc::c_longlong - (*replica).repl_ack_time
            } else {
                0 as libc::c_int as libc::c_longlong
            }) as libc::c_long;
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Lagging replica %s reported offset %lld behind master, lag=%ld, state=%s.\0"
                        as *const u8 as *const libc::c_char,
                    replicationGetSlaveName(replica),
                    server.master_repl_offset - (*replica).repl_ack_off,
                    lag,
                    replstateToString((*replica).replstate),
                );
            }
        }
    }
    if num_replicas > 0 as libc::c_int {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"%d of %d replicas are in sync when shutting down.\0" as *const u8
                    as *const libc::c_char,
                num_replicas - num_lagging_replicas,
                num_replicas,
            );
        }
    }
    ldbKillForkedSessions();
    if server.child_type == 1 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"There is a child saving an .rdb. Killing it!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        killRDBChild();
        rdbRemoveTempFile(server.child_pid, 0 as libc::c_int);
    }
    if server.child_type == 4 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"There is a module fork child. Killing it!\0" as *const u8
                    as *const libc::c_char,
            );
        }
        TerminateModuleForkChild(server.child_pid, 0 as libc::c_int);
    }
    if server.child_type == 2 as libc::c_int {
        if server.aof_state == 2 as libc::c_int {
            if force != 0 {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Writing initial AOF. Exit anyway.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block = 572715077006366937;
            } else {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Writing initial AOF, can't exit.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                current_block = 2170801590148406592;
            }
        } else {
            current_block = 572715077006366937;
        }
        match current_block {
            2170801590148406592 => {}
            _ => {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"There is a child rewriting the AOF. Killing it!\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                killAppendOnlyChild();
                current_block = 12381812505308290051;
            }
        }
    } else {
        current_block = 12381812505308290051;
    }
    match current_block {
        12381812505308290051 => {
            if server.aof_state != 0 as libc::c_int {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Calling fsync() on the AOF file.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                flushAppendOnlyFile(1 as libc::c_int);
                if fdatasync(server.aof_fd) == -(1 as libc::c_int) {
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"Fail to fsync the AOF file: %s.\0" as *const u8
                                as *const libc::c_char,
                            strerror(*__errno_location()),
                        );
                    }
                }
            }
            if server.saveparamslen > 0 as libc::c_int && nosave == 0 || save != 0 {
                if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        2 as libc::c_int,
                        b"Saving the final RDB snapshot before exiting.\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                if server.supervised_mode == 2 as libc::c_int {
                    redisCommunicateSystemd(
                        b"STATUS=Saving the final RDB snapshot\n\0" as *const u8
                            as *const libc::c_char,
                    );
                }
                let mut rsi: rdbSaveInfo = rdbSaveInfo {
                    repl_stream_db: 0,
                    repl_id_is_set: 0,
                    repl_id: [0; 41],
                    repl_offset: 0,
                };
                let mut rsiptr: *mut rdbSaveInfo = 0 as *mut rdbSaveInfo;
                rsiptr = rdbPopulateSaveInfo(&mut rsi);
                if rdbSave(0 as libc::c_int, server.rdb_filename, rsiptr)
                    != 0 as libc::c_int
                {
                    if force != 0 {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Error trying to save the DB. Exit anyway.\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        current_block = 10435735846551762309;
                    } else {
                        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                3 as libc::c_int,
                                b"Error trying to save the DB, can't exit.\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        if server.supervised_mode == 2 as libc::c_int {
                            redisCommunicateSystemd(
                                b"STATUS=Error trying to save the DB, can't exit.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                        }
                        current_block = 2170801590148406592;
                    }
                } else {
                    current_block = 10435735846551762309;
                }
            } else {
                current_block = 10435735846551762309;
            }
            match current_block {
                2170801590148406592 => {}
                _ => {
                    if !(server.aof_manifest).is_null() {
                        aofManifestFree(server.aof_manifest);
                    }
                    moduleFireServerEvent(
                        5 as libc::c_int as uint64_t,
                        0 as libc::c_int,
                        0 as *mut libc::c_void,
                    );
                    if server.daemonize != 0 || !(server.pidfile).is_null() {
                        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity)
                        {
                            _serverLog(
                                2 as libc::c_int,
                                b"Removing the pid file.\0" as *const u8
                                    as *const libc::c_char,
                            );
                        }
                        unlink(server.pidfile);
                    }
                    flushSlavesOutputBuffers();
                    closeListeningSockets(1 as libc::c_int);
                    if server.cluster_enabled != 0
                        && server.cluster_config_file_lock_fd != -(1 as libc::c_int)
                    {
                        flock(
                            server.cluster_config_file_lock_fd,
                            8 as libc::c_int | 4 as libc::c_int,
                        );
                    }
                    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                        _serverLog(
                            3 as libc::c_int,
                            b"%s is now ready to exit, bye bye...\0" as *const u8
                                as *const libc::c_char,
                            if server.sentinel_mode != 0 {
                                b"Sentinel\0" as *const u8 as *const libc::c_char
                            } else {
                                b"Redis\0" as *const u8 as *const libc::c_char
                            },
                        );
                    }
                    return 0 as libc::c_int;
                }
            }
        }
        _ => {}
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Errors trying to shut down the server. Check the logs for more information.\0"
                as *const u8 as *const libc::c_char,
        );
    }
    cancelShutdown();
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn writeCommandsDeniedByDiskError() -> libc::c_int {
    if server.stop_writes_on_bgsave_err != 0 && server.saveparamslen > 0 as libc::c_int
        && server.lastbgsave_status == -(1 as libc::c_int)
    {
        return 2 as libc::c_int
    } else {
        if server.aof_state != 0 as libc::c_int {
            if server.aof_last_write_status == -(1 as libc::c_int) {
                return 1 as libc::c_int;
            }
            let mut aof_bio_fsync_status: libc::c_int = 0;
            aof_bio_fsync_status = core::intrinsics::atomic_load_relaxed(
                &mut server.aof_bio_fsync_status,
            );
            if aof_bio_fsync_status == -(1 as libc::c_int) {
                server
                    .aof_last_write_errno = core::intrinsics::atomic_load_relaxed(
                    &mut server.aof_bio_fsync_errno,
                );
                return 1 as libc::c_int;
            }
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn writeCommandsGetDiskErrorMessage(
    mut error_code: libc::c_int,
) -> sds {
    let mut ret: sds = 0 as sds;
    if error_code == 2 as libc::c_int {
        ret = sdsdup((*shared.bgsaveerr).ptr as sds);
    } else {
        ret = sdscatfmt(
            sdsempty(),
            b"-MISCONF Errors writing to the AOF file: %s\r\n\0" as *const u8
                as *const libc::c_char,
            strerror(server.aof_last_write_errno),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn pingCommand(mut c: *mut client) {
    if (*c).argc > 2 as libc::c_int {
        addReplyErrorArity(c);
        return;
    }
    if (*c).flags & ((1 as libc::c_int) << 18 as libc::c_int) as libc::c_ulong != 0
        && (*c).resp == 2 as libc::c_int
    {
        addReply(c, shared.mbulkhdr[2 as libc::c_int as usize]);
        addReplyBulkCBuffer(
            c,
            b"pong\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            4 as libc::c_int as size_t,
        );
        if (*c).argc == 1 as libc::c_int {
            addReplyBulkCBuffer(
                c,
                b"\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                0 as libc::c_int as size_t,
            );
        } else {
            addReplyBulk(c, *((*c).argv).offset(1 as libc::c_int as isize));
        }
    } else if (*c).argc == 1 as libc::c_int {
        addReply(c, shared.pong);
    } else {
        addReplyBulk(c, *((*c).argv).offset(1 as libc::c_int as isize));
    };
}
#[no_mangle]
pub unsafe extern "C" fn echoCommand(mut c: *mut client) {
    addReplyBulk(c, *((*c).argv).offset(1 as libc::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn timeCommand(mut c: *mut client) {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
    addReplyBulkLongLong(c, tv.tv_sec as libc::c_longlong);
    addReplyBulkLongLong(c, tv.tv_usec as libc::c_longlong);
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandFlags(
    mut c: *mut client,
    mut flags: uint64_t,
    mut replyFlags: *mut replyFlagNames,
) {
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut j: libc::c_int = 0 as libc::c_int;
    while !((*replyFlags.offset(j as isize)).name).is_null() {
        if flags & (*replyFlags.offset(j as isize)).flag != 0 {
            count += 1;
        }
        j += 1;
    }
    addReplySetLen(c, count as libc::c_long);
    j = 0 as libc::c_int;
    while !((*replyFlags.offset(j as isize)).name).is_null() {
        if flags & (*replyFlags.offset(j as isize)).flag != 0 {
            addReplyStatus(c, (*replyFlags.offset(j as isize)).name);
        }
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyFlagsForCommand(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    let mut flagNames: [replyFlagNames; 21] = [
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as uint64_t,
                name: b"write\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 1 as libc::c_int) as uint64_t,
                name: b"readonly\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 2 as libc::c_int) as uint64_t,
                name: b"denyoom\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 3 as libc::c_int) as uint64_t,
                name: b"module\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 4 as libc::c_int) as uint64_t,
                name: b"admin\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 5 as libc::c_int) as uint64_t,
                name: b"pubsub\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 6 as libc::c_int) as uint64_t,
                name: b"noscript\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 8 as libc::c_int) as uint64_t,
                name: b"blocking\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 9 as libc::c_int) as uint64_t,
                name: b"loading\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 10 as libc::c_int) as uint64_t,
                name: b"stale\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 11 as libc::c_int) as uint64_t,
                name: b"skip_monitor\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 12 as libc::c_int) as uint64_t,
                name: b"skip_slowlog\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 13 as libc::c_int) as uint64_t,
                name: b"asking\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 14 as libc::c_int) as uint64_t,
                name: b"fast\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 15 as libc::c_int) as uint64_t,
                name: b"no_auth\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 19 as libc::c_int) as uint64_t,
                name: b"no_mandatory_keys\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 23 as libc::c_int) as uint64_t,
                name: b"no_async_loading\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 24 as libc::c_int) as uint64_t,
                name: b"no_multi\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 25 as libc::c_int) as uint64_t,
                name: b"movablekeys\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 26 as libc::c_int) as uint64_t,
                name: b"allow_busy\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: 0 as libc::c_int as uint64_t,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    addReplyCommandFlags(c, (*cmd).flags, flagNames.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn addReplyDocFlagsForCommand(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    let mut docFlagNames: [replyFlagNames; 3] = [
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_int) << 0 as libc::c_int) as uint64_t,
                name: b"deprecated\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_int) << 1 as libc::c_int) as uint64_t,
                name: b"syscmd\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: 0 as libc::c_int as uint64_t,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    addReplyCommandFlags(c, (*cmd).doc_flags as uint64_t, docFlagNames.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn addReplyFlagsForKeyArgs(
    mut c: *mut client,
    mut flags: uint64_t,
) {
    let mut docFlagNames: [replyFlagNames; 12] = [
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 0 as libc::c_int) as uint64_t,
                name: b"RO\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 1 as libc::c_int) as uint64_t,
                name: b"RW\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 2 as libc::c_int) as uint64_t,
                name: b"OW\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 3 as libc::c_int) as uint64_t,
                name: b"RM\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 4 as libc::c_int) as uint64_t,
                name: b"access\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 5 as libc::c_int) as uint64_t,
                name: b"update\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 6 as libc::c_int) as uint64_t,
                name: b"insert\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 7 as libc::c_int) as uint64_t,
                name: b"delete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 8 as libc::c_int) as uint64_t,
                name: b"not_key\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 9 as libc::c_int) as uint64_t,
                name: b"incomplete\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_ulonglong) << 10 as libc::c_int) as uint64_t,
                name: b"variable_flags\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: 0 as libc::c_int as uint64_t,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    addReplyCommandFlags(c, flags, docFlagNames.as_mut_ptr());
}
#[no_mangle]
pub static mut ARG_TYPE_STR: [*const libc::c_char; 9] = [
    b"string\0" as *const u8 as *const libc::c_char,
    b"integer\0" as *const u8 as *const libc::c_char,
    b"double\0" as *const u8 as *const libc::c_char,
    b"key\0" as *const u8 as *const libc::c_char,
    b"pattern\0" as *const u8 as *const libc::c_char,
    b"unix-time\0" as *const u8 as *const libc::c_char,
    b"pure-token\0" as *const u8 as *const libc::c_char,
    b"oneof\0" as *const u8 as *const libc::c_char,
    b"block\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn addReplyFlagsForArg(mut c: *mut client, mut flags: uint64_t) {
    let mut argFlagNames: [replyFlagNames; 4] = [
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_int) << 0 as libc::c_int) as uint64_t,
                name: b"optional\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_int) << 1 as libc::c_int) as uint64_t,
                name: b"multiple\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: ((1 as libc::c_int) << 2 as libc::c_int) as uint64_t,
                name: b"multiple_token\0" as *const u8 as *const libc::c_char,
            };
            init
        },
        {
            let mut init = replyFlagNames {
                flag: 0 as libc::c_int as uint64_t,
                name: 0 as *const libc::c_char,
            };
            init
        },
    ];
    addReplyCommandFlags(c, flags, argFlagNames.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandArgList(
    mut c: *mut client,
    mut args: *mut redisCommandArg,
    mut num_args: libc::c_int,
) {
    addReplyArrayLen(c, num_args as libc::c_long);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < num_args {
        let mut maplen: libc::c_long = 2 as libc::c_int as libc::c_long;
        if (*args.offset(j as isize)).key_spec_index != -(1 as libc::c_int) {
            maplen += 1;
        }
        if !((*args.offset(j as isize)).token).is_null() {
            maplen += 1;
        }
        if !((*args.offset(j as isize)).summary).is_null() {
            maplen += 1;
        }
        if !((*args.offset(j as isize)).since).is_null() {
            maplen += 1;
        }
        if !((*args.offset(j as isize)).deprecated_since).is_null() {
            maplen += 1;
        }
        if (*args.offset(j as isize)).flags != 0 {
            maplen += 1;
        }
        if (*args.offset(j as isize)).type_0 as libc::c_uint
            == ARG_TYPE_ONEOF as libc::c_int as libc::c_uint
            || (*args.offset(j as isize)).type_0 as libc::c_uint
                == ARG_TYPE_BLOCK as libc::c_int as libc::c_uint
        {
            maplen += 1;
        }
        addReplyMapLen(c, maplen);
        addReplyBulkCString(c, b"name\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, (*args.offset(j as isize)).name);
        addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, ARG_TYPE_STR[(*args.offset(j as isize)).type_0 as usize]);
        if (*args.offset(j as isize)).key_spec_index != -(1 as libc::c_int) {
            addReplyBulkCString(
                c,
                b"key_spec_index\0" as *const u8 as *const libc::c_char,
            );
            addReplyLongLong(
                c,
                (*args.offset(j as isize)).key_spec_index as libc::c_longlong,
            );
        }
        if !((*args.offset(j as isize)).token).is_null() {
            addReplyBulkCString(c, b"token\0" as *const u8 as *const libc::c_char);
            addReplyBulkCString(c, (*args.offset(j as isize)).token);
        }
        if !((*args.offset(j as isize)).summary).is_null() {
            addReplyBulkCString(c, b"summary\0" as *const u8 as *const libc::c_char);
            addReplyBulkCString(c, (*args.offset(j as isize)).summary);
        }
        if !((*args.offset(j as isize)).since).is_null() {
            addReplyBulkCString(c, b"since\0" as *const u8 as *const libc::c_char);
            addReplyBulkCString(c, (*args.offset(j as isize)).since);
        }
        if !((*args.offset(j as isize)).deprecated_since).is_null() {
            addReplyBulkCString(
                c,
                b"deprecated_since\0" as *const u8 as *const libc::c_char,
            );
            addReplyBulkCString(c, (*args.offset(j as isize)).deprecated_since);
        }
        if (*args.offset(j as isize)).flags != 0 {
            addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
            addReplyFlagsForArg(c, (*args.offset(j as isize)).flags as uint64_t);
        }
        if (*args.offset(j as isize)).type_0 as libc::c_uint
            == ARG_TYPE_ONEOF as libc::c_int as libc::c_uint
            || (*args.offset(j as isize)).type_0 as libc::c_uint
                == ARG_TYPE_BLOCK as libc::c_int as libc::c_uint
        {
            addReplyBulkCString(c, b"arguments\0" as *const u8 as *const libc::c_char);
            addReplyCommandArgList(
                c,
                (*args.offset(j as isize)).subargs,
                (*args.offset(j as isize)).num_args,
            );
        }
        j += 1;
    }
}
#[no_mangle]
pub static mut RESP2_TYPE_STR: [*const libc::c_char; 7] = [
    b"simple-string\0" as *const u8 as *const libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char,
    b"integer\0" as *const u8 as *const libc::c_char,
    b"bulk-string\0" as *const u8 as *const libc::c_char,
    b"null-bulk-string\0" as *const u8 as *const libc::c_char,
    b"array\0" as *const u8 as *const libc::c_char,
    b"null-array\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub static mut RESP3_TYPE_STR: [*const libc::c_char; 10] = [
    b"simple-string\0" as *const u8 as *const libc::c_char,
    b"error\0" as *const u8 as *const libc::c_char,
    b"integer\0" as *const u8 as *const libc::c_char,
    b"double\0" as *const u8 as *const libc::c_char,
    b"bulk-string\0" as *const u8 as *const libc::c_char,
    b"array\0" as *const u8 as *const libc::c_char,
    b"map\0" as *const u8 as *const libc::c_char,
    b"set\0" as *const u8 as *const libc::c_char,
    b"bool\0" as *const u8 as *const libc::c_char,
    b"null\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandHistory(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    addReplySetLen(c, (*cmd).num_history as libc::c_long);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*cmd).num_history {
        addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
        addReplyBulkCString(c, (*((*cmd).history).offset(j as isize)).since);
        addReplyBulkCString(c, (*((*cmd).history).offset(j as isize)).changes);
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandTips(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    addReplySetLen(c, (*cmd).num_tips as libc::c_long);
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < (*cmd).num_tips {
        addReplyBulkCString(c, *((*cmd).tips).offset(j as isize));
        j += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandKeySpecs(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    addReplySetLen(c, (*cmd).key_specs_num as libc::c_long);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < (*cmd).key_specs_num {
        let mut maplen: libc::c_int = 3 as libc::c_int;
        if !((*((*cmd).key_specs).offset(i as isize)).notes).is_null() {
            maplen += 1;
        }
        addReplyMapLen(c, maplen as libc::c_long);
        if !((*((*cmd).key_specs).offset(i as isize)).notes).is_null() {
            addReplyBulkCString(c, b"notes\0" as *const u8 as *const libc::c_char);
            addReplyBulkCString(c, (*((*cmd).key_specs).offset(i as isize)).notes);
        }
        addReplyBulkCString(c, b"flags\0" as *const u8 as *const libc::c_char);
        addReplyFlagsForKeyArgs(c, (*((*cmd).key_specs).offset(i as isize)).flags);
        addReplyBulkCString(c, b"begin_search\0" as *const u8 as *const libc::c_char);
        match (*((*cmd).key_specs).offset(i as isize)).begin_search_type as libc::c_uint
        {
            1 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"unknown\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 0 as libc::c_int as libc::c_long);
            }
            2 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"index\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 1 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"index\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).bs.index.pos
                        as libc::c_longlong,
                );
            }
            3 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"keyword\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"keyword\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).bs.keyword.keyword,
                );
                addReplyBulkCString(
                    c,
                    b"startfrom\0" as *const u8 as *const libc::c_char,
                );
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).bs.keyword.startfrom
                        as libc::c_longlong,
                );
            }
            _ => {
                _serverPanic(
                    b"server.c\0" as *const u8 as *const libc::c_char,
                    4632 as libc::c_int,
                    b"Invalid begin_search key spec type %d\0" as *const u8
                        as *const libc::c_char,
                    (*((*cmd).key_specs).offset(i as isize)).begin_search_type
                        as libc::c_uint,
                );
                unreachable!();
            }
        }
        addReplyBulkCString(c, b"find_keys\0" as *const u8 as *const libc::c_char);
        match (*((*cmd).key_specs).offset(i as isize)).find_keys_type as libc::c_uint {
            1 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"unknown\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 0 as libc::c_int as libc::c_long);
            }
            2 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"range\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"lastkey\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.range.lastkey
                        as libc::c_longlong,
                );
                addReplyBulkCString(c, b"keystep\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.range.keystep
                        as libc::c_longlong,
                );
                addReplyBulkCString(c, b"limit\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.range.limit
                        as libc::c_longlong,
                );
            }
            3 => {
                addReplyMapLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulkCString(c, b"type\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"keynum\0" as *const u8 as *const libc::c_char);
                addReplyBulkCString(c, b"spec\0" as *const u8 as *const libc::c_char);
                addReplyMapLen(c, 3 as libc::c_int as libc::c_long);
                addReplyBulkCString(
                    c,
                    b"keynumidx\0" as *const u8 as *const libc::c_char,
                );
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.keynum.keynumidx
                        as libc::c_longlong,
                );
                addReplyBulkCString(
                    c,
                    b"firstkey\0" as *const u8 as *const libc::c_char,
                );
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.keynum.firstkey
                        as libc::c_longlong,
                );
                addReplyBulkCString(c, b"keystep\0" as *const u8 as *const libc::c_char);
                addReplyLongLong(
                    c,
                    (*((*cmd).key_specs).offset(i as isize)).fk.keynum.keystep
                        as libc::c_longlong,
                );
            }
            _ => {
                _serverPanic(
                    b"server.c\0" as *const u8 as *const libc::c_char,
                    4674 as libc::c_int,
                    b"Invalid find_keys key spec type %d\0" as *const u8
                        as *const libc::c_char,
                    (*((*cmd).key_specs).offset(i as isize)).begin_search_type
                        as libc::c_uint,
                );
                unreachable!();
            }
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandSubCommands(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
    mut reply_function: Option::<
        unsafe extern "C" fn(*mut client, *mut redisCommand) -> (),
    >,
    mut use_map: libc::c_int,
) {
    if ((*cmd).subcommands_dict).is_null() {
        addReplySetLen(c, 0 as libc::c_int as libc::c_long);
        return;
    }
    if use_map != 0 {
        addReplyMapLen(
            c,
            ((*(*cmd).subcommands_dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*cmd).subcommands_dict).ht_used[1 as libc::c_int as usize],
                ) as libc::c_long,
        );
    } else {
        addReplyArrayLen(
            c,
            ((*(*cmd).subcommands_dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*cmd).subcommands_dict).ht_used[1 as libc::c_int as usize],
                ) as libc::c_long,
        );
    }
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetSafeIterator((*cmd).subcommands_dict);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut sub: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if use_map != 0 {
            addReplyBulkCBuffer(
                c,
                (*sub).fullname as *const libc::c_void,
                sdslen((*sub).fullname),
            );
        }
        reply_function.expect("non-null function pointer")(c, sub);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub static mut COMMAND_GROUP_STR: [*const libc::c_char; 18] = [
    b"generic\0" as *const u8 as *const libc::c_char,
    b"string\0" as *const u8 as *const libc::c_char,
    b"list\0" as *const u8 as *const libc::c_char,
    b"set\0" as *const u8 as *const libc::c_char,
    b"sorted-set\0" as *const u8 as *const libc::c_char,
    b"hash\0" as *const u8 as *const libc::c_char,
    b"pubsub\0" as *const u8 as *const libc::c_char,
    b"transactions\0" as *const u8 as *const libc::c_char,
    b"connection\0" as *const u8 as *const libc::c_char,
    b"server\0" as *const u8 as *const libc::c_char,
    b"scripting\0" as *const u8 as *const libc::c_char,
    b"hyperloglog\0" as *const u8 as *const libc::c_char,
    b"cluster\0" as *const u8 as *const libc::c_char,
    b"sentinel\0" as *const u8 as *const libc::c_char,
    b"geo\0" as *const u8 as *const libc::c_char,
    b"stream\0" as *const u8 as *const libc::c_char,
    b"bitmap\0" as *const u8 as *const libc::c_char,
    b"module\0" as *const u8 as *const libc::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandInfo(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    if cmd.is_null() {
        addReplyNull(c);
    } else {
        let mut firstkey: libc::c_int = 0 as libc::c_int;
        let mut lastkey: libc::c_int = 0 as libc::c_int;
        let mut keystep: libc::c_int = 0 as libc::c_int;
        if (*cmd).legacy_range_key_spec.begin_search_type as libc::c_uint
            != KSPEC_BS_INVALID as libc::c_int as libc::c_uint
        {
            firstkey = (*cmd).legacy_range_key_spec.bs.index.pos;
            lastkey = (*cmd).legacy_range_key_spec.fk.range.lastkey;
            if lastkey >= 0 as libc::c_int {
                lastkey += firstkey;
            }
            keystep = (*cmd).legacy_range_key_spec.fk.range.keystep;
        }
        addReplyArrayLen(c, 10 as libc::c_int as libc::c_long);
        addReplyBulkCBuffer(
            c,
            (*cmd).fullname as *const libc::c_void,
            sdslen((*cmd).fullname),
        );
        addReplyLongLong(c, (*cmd).arity as libc::c_longlong);
        addReplyFlagsForCommand(c, cmd);
        addReplyLongLong(c, firstkey as libc::c_longlong);
        addReplyLongLong(c, lastkey as libc::c_longlong);
        addReplyLongLong(c, keystep as libc::c_longlong);
        addReplyCommandCategories(c, cmd);
        addReplyCommandTips(c, cmd);
        addReplyCommandKeySpecs(c, cmd);
        addReplyCommandSubCommands(
            c,
            cmd,
            Some(
                addReplyCommandInfo
                    as unsafe extern "C" fn(*mut client, *mut redisCommand) -> (),
            ),
            0 as libc::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn addReplyCommandDocs(
    mut c: *mut client,
    mut cmd: *mut redisCommand,
) {
    let mut maplen: libc::c_long = 1 as libc::c_int as libc::c_long;
    if !((*cmd).summary).is_null() {
        maplen += 1;
    }
    if !((*cmd).since).is_null() {
        maplen += 1;
    }
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int
        != 0
    {
        maplen += 1;
    }
    if !((*cmd).complexity).is_null() {
        maplen += 1;
    }
    if (*cmd).doc_flags != 0 {
        maplen += 1;
    }
    if !((*cmd).deprecated_since).is_null() {
        maplen += 1;
    }
    if !((*cmd).replaced_by).is_null() {
        maplen += 1;
    }
    if !((*cmd).history).is_null() {
        maplen += 1;
    }
    if !((*cmd).args).is_null() {
        maplen += 1;
    }
    if !((*cmd).subcommands_dict).is_null() {
        maplen += 1;
    }
    addReplyMapLen(c, maplen);
    if !((*cmd).summary).is_null() {
        addReplyBulkCString(c, b"summary\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, (*cmd).summary);
    }
    if !((*cmd).since).is_null() {
        addReplyBulkCString(c, b"since\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, (*cmd).since);
    }
    addReplyBulkCString(c, b"group\0" as *const u8 as *const libc::c_char);
    addReplyBulkCString(c, COMMAND_GROUP_STR[(*cmd).group as usize]);
    if !((*cmd).complexity).is_null() {
        addReplyBulkCString(c, b"complexity\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, (*cmd).complexity);
    }
    if (*cmd).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 3 as libc::c_int
        != 0
    {
        addReplyBulkCString(c, b"module\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, moduleNameFromCommand(cmd));
    }
    if (*cmd).doc_flags != 0 {
        addReplyBulkCString(c, b"doc_flags\0" as *const u8 as *const libc::c_char);
        addReplyDocFlagsForCommand(c, cmd);
    }
    if !((*cmd).deprecated_since).is_null() {
        addReplyBulkCString(
            c,
            b"deprecated_since\0" as *const u8 as *const libc::c_char,
        );
        addReplyBulkCString(c, (*cmd).deprecated_since);
    }
    if !((*cmd).replaced_by).is_null() {
        addReplyBulkCString(c, b"replaced_by\0" as *const u8 as *const libc::c_char);
        addReplyBulkCString(c, (*cmd).replaced_by);
    }
    if !((*cmd).history).is_null() {
        addReplyBulkCString(c, b"history\0" as *const u8 as *const libc::c_char);
        addReplyCommandHistory(c, cmd);
    }
    if !((*cmd).args).is_null() {
        addReplyBulkCString(c, b"arguments\0" as *const u8 as *const libc::c_char);
        addReplyCommandArgList(c, (*cmd).args, (*cmd).num_args);
    }
    if !((*cmd).subcommands_dict).is_null() {
        addReplyBulkCString(c, b"subcommands\0" as *const u8 as *const libc::c_char);
        addReplyCommandSubCommands(
            c,
            cmd,
            Some(
                addReplyCommandDocs
                    as unsafe extern "C" fn(*mut client, *mut redisCommand) -> (),
            ),
            1 as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn getKeysSubcommandImpl(
    mut c: *mut client,
    mut with_flags: libc::c_int,
) {
    let mut cmd: *mut redisCommand = lookupCommand(
        ((*c).argv).offset(2 as libc::c_int as isize),
        (*c).argc - 2 as libc::c_int,
    );
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
    let mut j: libc::c_int = 0;
    if cmd.is_null() {
        addReplyError(
            c,
            b"Invalid command specified\0" as *const u8 as *const libc::c_char,
        );
        return;
    } else {
        if doesCommandHaveKeys(cmd) == 0 {
            addReplyError(
                c,
                b"The command has no key arguments\0" as *const u8 as *const libc::c_char,
            );
            return;
        } else {
            if (*cmd).arity > 0 as libc::c_int
                && (*cmd).arity != (*c).argc - 2 as libc::c_int
                || ((*c).argc - 2 as libc::c_int) < -(*cmd).arity
            {
                addReplyError(
                    c,
                    b"Invalid number of arguments specified for command\0" as *const u8
                        as *const libc::c_char,
                );
                return;
            }
        }
    }
    if getKeysFromCommandWithSpecs(
        cmd,
        ((*c).argv).offset(2 as libc::c_int as isize),
        (*c).argc - 2 as libc::c_int,
        0 as libc::c_int,
        &mut result,
    ) == 0
    {
        if (*cmd).flags as libc::c_ulonglong
            & (1 as libc::c_ulonglong) << 19 as libc::c_int != 0
        {
            addReplyArrayLen(c, 0 as libc::c_int as libc::c_long);
        } else {
            addReplyError(
                c,
                b"Invalid arguments specified for command\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else {
        addReplyArrayLen(c, result.numkeys as libc::c_long);
        j = 0 as libc::c_int;
        while j < result.numkeys {
            if with_flags == 0 {
                addReplyBulk(
                    c,
                    *((*c).argv)
                        .offset(
                            ((*(result.keys).offset(j as isize)).pos + 2 as libc::c_int)
                                as isize,
                        ),
                );
            } else {
                addReplyArrayLen(c, 2 as libc::c_int as libc::c_long);
                addReplyBulk(
                    c,
                    *((*c).argv)
                        .offset(
                            ((*(result.keys).offset(j as isize)).pos + 2 as libc::c_int)
                                as isize,
                        ),
                );
                addReplyFlagsForKeyArgs(
                    c,
                    (*(result.keys).offset(j as isize)).flags as uint64_t,
                );
            }
            j += 1;
        }
    }
    getKeysFreeResult(&mut result);
}
#[no_mangle]
pub unsafe extern "C" fn commandGetKeysAndFlagsCommand(mut c: *mut client) {
    getKeysSubcommandImpl(c, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn getKeysSubcommand(mut c: *mut client) {
    getKeysSubcommandImpl(c, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn commandCommand(mut c: *mut client) {
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    addReplyArrayLen(
        c,
        ((*server.commands).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*server.commands).ht_used[1 as libc::c_int as usize])
            as libc::c_long,
    );
    di = dictGetIterator(server.commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        addReplyCommandInfo(c, (*de).v.val as *mut redisCommand);
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn commandCountCommand(mut c: *mut client) {
    addReplyLongLong(
        c,
        ((*server.commands).ht_used[0 as libc::c_int as usize])
            .wrapping_add((*server.commands).ht_used[1 as libc::c_int as usize])
            as libc::c_longlong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn shouldFilterFromCommandList(
    mut cmd: *mut redisCommand,
    mut filter: *mut commandListFilter,
) -> libc::c_int {
    match (*filter).type_0 as libc::c_uint {
        0 => {
            if (*filter).cache.valid == 0 {
                (*filter).cache.u.module_handle = moduleGetHandleByName((*filter).arg);
                (*filter).cache.valid = 1 as libc::c_int;
            }
            return (moduleIsModuleCommand((*filter).cache.u.module_handle, cmd) == 0)
                as libc::c_int;
        }
        1 => {
            if (*filter).cache.valid == 0 {
                (*filter)
                    .cache
                    .u
                    .aclcat = ACLGetCommandCategoryFlagByName(
                    (*filter).arg as *const libc::c_char,
                );
                (*filter).cache.valid = 1 as libc::c_int;
            }
            let mut cat: uint64_t = (*filter).cache.u.aclcat;
            if cat == 0 as libc::c_int as libc::c_ulong {
                return 1 as libc::c_int;
            }
            return ((*cmd).acl_categories & cat == 0) as libc::c_int;
        }
        2 => {
            return (stringmatchlen(
                (*filter).arg as *const libc::c_char,
                sdslen((*filter).arg) as libc::c_int,
                (*cmd).fullname as *const libc::c_char,
                sdslen((*cmd).fullname) as libc::c_int,
                1 as libc::c_int,
            ) == 0) as libc::c_int;
        }
        _ => {
            _serverPanic(
                b"server.c\0" as *const u8 as *const libc::c_char,
                4922 as libc::c_int,
                b"Invalid filter type %d\0" as *const u8 as *const libc::c_char,
                (*filter).type_0 as libc::c_uint,
            );
            unreachable!();
        }
    }
    panic!("Reached end of non-void function without returning");
}
#[no_mangle]
pub unsafe extern "C" fn commandListWithFilter(
    mut c: *mut client,
    mut commands: *mut dict,
    mut filter: commandListFilter,
    mut numcmds: *mut libc::c_int,
) {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        if shouldFilterFromCommandList(cmd, &mut filter) == 0 {
            addReplyBulkCBuffer(
                c,
                (*cmd).fullname as *const libc::c_void,
                sdslen((*cmd).fullname),
            );
            *numcmds += 1;
        }
        if !((*cmd).subcommands_dict).is_null() {
            commandListWithFilter(c, (*cmd).subcommands_dict, filter, numcmds);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn commandListWithoutFilter(
    mut c: *mut client,
    mut commands: *mut dict,
    mut numcmds: *mut libc::c_int,
) {
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = dictGetIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
        addReplyBulkCBuffer(
            c,
            (*cmd).fullname as *const libc::c_void,
            sdslen((*cmd).fullname),
        );
        *numcmds += 1;
        if !((*cmd).subcommands_dict).is_null() {
            commandListWithoutFilter(c, (*cmd).subcommands_dict, numcmds);
        }
    }
    dictReleaseIterator(di);
}
#[no_mangle]
pub unsafe extern "C" fn commandListCommand(mut c: *mut client) {
    let mut i: libc::c_int = 2 as libc::c_int;
    let mut got_filter: libc::c_int = 0 as libc::c_int;
    let mut filter: commandListFilter = {
        let mut init = commandListFilter {
            type_0: COMMAND_LIST_FILTER_MODULE,
            arg: 0 as *mut libc::c_char,
            cache: C2RustUnnamed_24 {
                valid: 0,
                u: C2RustUnnamed_25 { aclcat: 0 },
            },
        };
        init
    };
    while i < (*c).argc {
        let mut moreargs: libc::c_int = (*c).argc - 1 as libc::c_int - i;
        let mut opt: *mut libc::c_char = (**((*c).argv).offset(i as isize)).ptr
            as *mut libc::c_char;
        if strcasecmp(opt, b"filterby\0" as *const u8 as *const libc::c_char) == 0
            && moreargs == 2 as libc::c_int
        {
            let mut filtertype: *mut libc::c_char = (**((*c).argv)
                .offset((i + 1 as libc::c_int) as isize))
                .ptr as *mut libc::c_char;
            if strcasecmp(filtertype, b"module\0" as *const u8 as *const libc::c_char)
                == 0
            {
                filter.type_0 = COMMAND_LIST_FILTER_MODULE;
            } else if strcasecmp(
                filtertype,
                b"aclcat\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                filter.type_0 = COMMAND_LIST_FILTER_ACLCAT;
            } else if strcasecmp(
                filtertype,
                b"pattern\0" as *const u8 as *const libc::c_char,
            ) == 0
            {
                filter.type_0 = COMMAND_LIST_FILTER_PATTERN;
            } else {
                addReplyErrorObject(c, shared.syntaxerr);
                return;
            }
            got_filter = 1 as libc::c_int;
            filter
                .arg = (**((*c).argv).offset((i + 2 as libc::c_int) as isize)).ptr
                as sds;
            i += 2 as libc::c_int;
        } else {
            addReplyErrorObject(c, shared.syntaxerr);
            return;
        }
        i += 1;
    }
    let mut numcmds: libc::c_int = 0 as libc::c_int;
    let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
    if got_filter != 0 {
        commandListWithFilter(c, server.commands, filter, &mut numcmds);
    } else {
        commandListWithoutFilter(c, server.commands, &mut numcmds);
    }
    setDeferredArrayLen(c, replylen, numcmds as libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn commandInfoCommand(mut c: *mut client) {
    let mut i: libc::c_int = 0;
    if (*c).argc == 2 as libc::c_int {
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        addReplyArrayLen(
            c,
            ((*server.commands).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*server.commands).ht_used[1 as libc::c_int as usize])
                as libc::c_long,
        );
        di = dictGetIterator(server.commands);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            addReplyCommandInfo(c, (*de).v.val as *mut redisCommand);
        }
        dictReleaseIterator(di);
    } else {
        addReplyArrayLen(c, ((*c).argc - 2 as libc::c_int) as libc::c_long);
        i = 2 as libc::c_int;
        while i < (*c).argc {
            addReplyCommandInfo(
                c,
                lookupCommandBySds((**((*c).argv).offset(i as isize)).ptr as sds),
            );
            i += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn commandDocsCommand(mut c: *mut client) {
    let mut i: libc::c_int = 0;
    if (*c).argc == 2 as libc::c_int {
        let mut di: *mut dictIterator = 0 as *mut dictIterator;
        let mut de: *mut dictEntry = 0 as *mut dictEntry;
        addReplyMapLen(
            c,
            ((*server.commands).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*server.commands).ht_used[1 as libc::c_int as usize])
                as libc::c_long,
        );
        di = dictGetIterator(server.commands);
        loop {
            de = dictNext(di);
            if de.is_null() {
                break;
            }
            let mut cmd: *mut redisCommand = (*de).v.val as *mut redisCommand;
            addReplyBulkCBuffer(
                c,
                (*cmd).fullname as *const libc::c_void,
                sdslen((*cmd).fullname),
            );
            addReplyCommandDocs(c, cmd);
        }
        dictReleaseIterator(di);
    } else {
        let mut numcmds: libc::c_int = 0 as libc::c_int;
        let mut replylen: *mut libc::c_void = addReplyDeferredLen(c);
        i = 2 as libc::c_int;
        while i < (*c).argc {
            let mut cmd_0: *mut redisCommand = lookupCommandBySds(
                (**((*c).argv).offset(i as isize)).ptr as sds,
            );
            if !cmd_0.is_null() {
                addReplyBulkCBuffer(
                    c,
                    (*cmd_0).fullname as *const libc::c_void,
                    sdslen((*cmd_0).fullname),
                );
                addReplyCommandDocs(c, cmd_0);
                numcmds += 1;
            }
            i += 1;
        }
        setDeferredMapLen(c, replylen, numcmds as libc::c_long);
    };
}
#[no_mangle]
pub unsafe extern "C" fn commandGetKeysCommand(mut c: *mut client) {
    getKeysSubcommand(c);
}
#[no_mangle]
pub unsafe extern "C" fn commandHelpCommand(mut c: *mut client) {
    let mut help: [*const libc::c_char; 19] = [
        b"(no subcommand)\0" as *const u8 as *const libc::c_char,
        b"    Return details about all Redis commands.\0" as *const u8
            as *const libc::c_char,
        b"COUNT\0" as *const u8 as *const libc::c_char,
        b"    Return the total number of commands in this Redis server.\0" as *const u8
            as *const libc::c_char,
        b"LIST\0" as *const u8 as *const libc::c_char,
        b"    Return a list of all commands in this Redis server.\0" as *const u8
            as *const libc::c_char,
        b"INFO [<command-name> ...]\0" as *const u8 as *const libc::c_char,
        b"    Return details about multiple Redis commands.\0" as *const u8
            as *const libc::c_char,
        b"    If no command names are given, documentation details for all\0"
            as *const u8 as *const libc::c_char,
        b"    commands are returned.\0" as *const u8 as *const libc::c_char,
        b"DOCS [<command-name> ...]\0" as *const u8 as *const libc::c_char,
        b"    Return documentation details about multiple Redis commands.\0" as *const u8
            as *const libc::c_char,
        b"    If no command names are given, documentation details for all\0"
            as *const u8 as *const libc::c_char,
        b"    commands are returned.\0" as *const u8 as *const libc::c_char,
        b"GETKEYS <full-command>\0" as *const u8 as *const libc::c_char,
        b"    Return the keys from a full Redis command.\0" as *const u8
            as *const libc::c_char,
        b"GETKEYSANDFLAGS <full-command>\0" as *const u8 as *const libc::c_char,
        b"    Return the keys and the access flags from a full Redis command.\0"
            as *const u8 as *const libc::c_char,
        0 as *const libc::c_char,
    ];
    addReplyHelp(c, help.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn bytesToHumanInt(
    mut s: *mut libc::c_char,
    mut n: libc::c_ulonglong,
) {
    let mut d: libc::c_double = 0.;
    if n < 1024 as libc::c_int as libc::c_ulonglong {
        sprintf(s, b"%lluB\0" as *const u8 as *const libc::c_char, n);
    } else if n < (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulonglong {
        d = n as libc::c_double / 1024 as libc::c_int as libc::c_double;
        sprintf(s, b"%.2fK\0" as *const u8 as *const libc::c_char, d);
    } else if n
        < (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
    {
        d = n as libc::c_double
            / (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_double;
        sprintf(s, b"%.2fM\0" as *const u8 as *const libc::c_char, d);
    } else if n
        < (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
    {
        d = n as libc::c_double
            / (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double;
        sprintf(s, b"%.2fG\0" as *const u8 as *const libc::c_char, d);
    } else if n
        < (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
    {
        d = n as libc::c_double
            / (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double;
        sprintf(s, b"%.2fT\0" as *const u8 as *const libc::c_char, d);
    } else if n
        < (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong
            * 1024 as libc::c_int as libc::c_longlong) as libc::c_ulonglong
    {
        d = n as libc::c_double
            / (1024 as libc::c_longlong * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong
                * 1024 as libc::c_int as libc::c_longlong) as libc::c_double;
        sprintf(s, b"%.2fP\0" as *const u8 as *const libc::c_char, d);
    } else {
        sprintf(s, b"%lluB\0" as *const u8 as *const libc::c_char, n);
    };
}
#[no_mangle]
pub unsafe extern "C" fn fillPercentileDistributionLatencies(
    mut info: sds,
    mut histogram_name: *const libc::c_char,
    mut histogram: *mut hdr_histogram,
) -> sds {
    info = sdscatfmt(
        info,
        b"latency_percentiles_usec_%s:\0" as *const u8 as *const libc::c_char,
        histogram_name,
    );
    let mut j: libc::c_int = 0 as libc::c_int;
    while j < server.latency_tracking_info_percentiles_len {
        let mut fbuf: [libc::c_char; 128] = [0; 128];
        let mut len: size_t = sprintf(
            fbuf.as_mut_ptr(),
            b"%f\0" as *const u8 as *const libc::c_char,
            *(server.latency_tracking_info_percentiles).offset(j as isize),
        ) as size_t;
        len = trimDoubleString(fbuf.as_mut_ptr(), len) as size_t;
        info = sdscatprintf(
            info,
            b"p%s=%.3f\0" as *const u8 as *const libc::c_char,
            fbuf.as_mut_ptr(),
            hdr_value_at_percentile(
                histogram,
                *(server.latency_tracking_info_percentiles).offset(j as isize),
            ) as libc::c_double / 1000.0f32 as libc::c_double,
        );
        if j != server.latency_tracking_info_percentiles_len - 1 as libc::c_int {
            info = sdscatlen(
                info,
                b",\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
        }
        j += 1;
    }
    info = sdscatprintf(info, b"\r\n\0" as *const u8 as *const libc::c_char);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn replstateToString(
    mut replstate: libc::c_int,
) -> *const libc::c_char {
    match replstate {
        6 | 7 => return b"wait_bgsave\0" as *const u8 as *const libc::c_char,
        8 => return b"send_bulk\0" as *const u8 as *const libc::c_char,
        9 => return b"online\0" as *const u8 as *const libc::c_char,
        _ => return b"\0" as *const u8 as *const libc::c_char,
    };
}
//let iter = "b#:\n\r\0".iter().enumerate();
static mut unsafe_info_chars: [libc::c_char; 5] = {
    const BYTES: [u8; 5] = [b'#', b':', b'\n', b'\r', 0];
    let mut cell = UnsafeCell::new([0; 5]);
    let chars = unsafe { &mut *cell.get() };
    let mut i = 0;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char;
    cell.into_inner()
};

static mut unsafe_info_chars_substs: [libc::c_char; 5] = {
    const BYTES: [u8; 5] = [b'_', b'_', b'_', b'_', b'\0'];
    let mut cell = UnsafeCell::new([0; 5]);
    let chars = unsafe { &mut *cell.get() };
    let mut i = 0;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char; i += 1;
    chars[i] = BYTES[i] as libc::c_char;
    cell.into_inner()
};
/*static mut unsafe_info_chars_substs: UnsafeCell<[libc::c_char; 5]> = UnsafeCell::new(
    unsafe { core::mem::transmute::<[u8; 5], [libc::c_char; 5]>([b'_', b'_', b'_', b'_', b'\0']) }
);*/
#[no_mangle]
pub unsafe extern "C" fn getSafeInfoString(
    mut s: *const libc::c_char,
    mut len: size_t,
    mut tmp: *mut *mut libc::c_char,
) -> *const libc::c_char {
    *tmp = 0 as *mut libc::c_char;
    if (mempbrk(
        s,
        len,
        unsafe_info_chars.as_mut_ptr(),
        (core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ))
        .is_null()
    {
        return s;
    }
    *tmp = zmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    let mut new: *mut libc::c_char = *tmp;
    memcpy(new as *mut libc::c_void, s as *const libc::c_void, len);
    *new.offset(len as isize) = '\0' as i32 as libc::c_char;
    return memmapchars(
        new,
        len,
        unsafe_info_chars.as_mut_ptr(),
        unsafe_info_chars_substs.as_mut_ptr(),
        (core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
    );
}
#[no_mangle]
pub unsafe extern "C" fn genRedisInfoStringCommandStats(
    mut info: sds,
    mut commands: *mut dict,
) -> sds {
    let mut c: *mut redisCommand = 0 as *mut redisCommand;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    di = dictGetSafeIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut tmpsafe: *mut libc::c_char = 0 as *mut libc::c_char;
        c = (*de).v.val as *mut redisCommand;
        if (*c).calls != 0 || (*c).failed_calls != 0 || (*c).rejected_calls != 0 {
            info = sdscatprintf(
                info,
                b"cmdstat_%s:calls=%lld,usec=%lld,usec_per_call=%.2f,rejected_calls=%lld,failed_calls=%lld\r\n\0"
                    as *const u8 as *const libc::c_char,
                getSafeInfoString(
                    (*c).fullname as *const libc::c_char,
                    sdslen((*c).fullname),
                    &mut tmpsafe,
                ),
                (*c).calls,
                (*c).microseconds,
                (if (*c).calls == 0 as libc::c_int as libc::c_longlong {
                    0 as libc::c_int as libc::c_float
                } else {
                    (*c).microseconds as libc::c_float / (*c).calls as libc::c_float
                }) as libc::c_double,
                (*c).rejected_calls,
                (*c).failed_calls,
            );
            if !tmpsafe.is_null() {
                zfree(tmpsafe as *mut libc::c_void);
            }
        }
        if !((*c).subcommands_dict).is_null() {
            info = genRedisInfoStringCommandStats(info, (*c).subcommands_dict);
        }
    }
    dictReleaseIterator(di);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn genRedisInfoStringLatencyStats(
    mut info: sds,
    mut commands: *mut dict,
) -> sds {
    let mut c: *mut redisCommand = 0 as *mut redisCommand;
    let mut de: *mut dictEntry = 0 as *mut dictEntry;
    let mut di: *mut dictIterator = 0 as *mut dictIterator;
    di = dictGetSafeIterator(commands);
    loop {
        de = dictNext(di);
        if de.is_null() {
            break;
        }
        let mut tmpsafe: *mut libc::c_char = 0 as *mut libc::c_char;
        c = (*de).v.val as *mut redisCommand;
        if !((*c).latency_histogram).is_null() {
            info = fillPercentileDistributionLatencies(
                info,
                getSafeInfoString(
                    (*c).fullname as *const libc::c_char,
                    sdslen((*c).fullname),
                    &mut tmpsafe,
                ),
                (*c).latency_histogram,
            );
            if !tmpsafe.is_null() {
                zfree(tmpsafe as *mut libc::c_void);
            }
        }
        if !((*c).subcommands_dict).is_null() {
            info = genRedisInfoStringLatencyStats(info, (*c).subcommands_dict);
        }
    }
    dictReleaseIterator(di);
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn addInfoSectionsToDict(
    mut section_dict: *mut dict,
    mut sections: *mut *mut libc::c_char,
) {
    while !(*sections).is_null() {
        let mut section: sds = sdsnew(*sections);
        if dictAdd(section_dict, section as *mut libc::c_void, 0 as *mut libc::c_void)
            == 1 as libc::c_int
        {
            sdsfree(section);
        }
        sections = sections.offset(1);
    }
}
static mut cached_default_info_sections: *mut dict = 0 as *const dict as *mut dict;
#[no_mangle]
pub unsafe extern "C" fn releaseInfoSectionDict(mut sec: *mut dict) {
    if sec != cached_default_info_sections {
        dictRelease(sec);
    }
}
#[no_mangle]
pub unsafe extern "C" fn genInfoSectionDict(
    mut argv: *mut *mut robj,
    mut argc: libc::c_int,
    mut defaults: *mut *mut libc::c_char,
    mut out_all: *mut libc::c_int,
    mut out_everything: *mut libc::c_int,
) -> *mut dict {
    let mut default_sections: [*mut libc::c_char; 12] = [
        b"server\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"clients\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"memory\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"persistence\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"stats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"replication\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cpu\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"module_list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"errorstats\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"keyspace\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as *mut libc::c_char,
    ];
    if defaults.is_null() {
        defaults = default_sections.as_mut_ptr();
    }
    if argc == 0 as libc::c_int {
        if !cached_default_info_sections.is_null() {
            return cached_default_info_sections;
        }
        cached_default_info_sections = dictCreate(&mut stringSetDictType);
        dictExpand(cached_default_info_sections, 16 as libc::c_int as libc::c_ulong);
        addInfoSectionsToDict(cached_default_info_sections, defaults);
        return cached_default_info_sections;
    }
    let mut section_dict: *mut dict = dictCreate(&mut stringSetDictType);
    dictExpand(
        section_dict,
        (if argc < 16 as libc::c_int { argc } else { 16 as libc::c_int })
            as libc::c_ulong,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < argc {
        if strcasecmp(
            (**argv.offset(i as isize)).ptr as *const libc::c_char,
            b"default\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            addInfoSectionsToDict(section_dict, defaults);
        } else if strcasecmp(
            (**argv.offset(i as isize)).ptr as *const libc::c_char,
            b"all\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !out_all.is_null() {
                *out_all = 1 as libc::c_int;
            }
        } else if strcasecmp(
            (**argv.offset(i as isize)).ptr as *const libc::c_char,
            b"everything\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !out_everything.is_null() {
                *out_everything = 1 as libc::c_int;
            }
            if !out_all.is_null() {
                *out_all = 1 as libc::c_int;
            }
        } else {
            let mut section: sds = sdsnew(
                (**argv.offset(i as isize)).ptr as *const libc::c_char,
            );
            if dictAdd(
                section_dict,
                section as *mut libc::c_void,
                0 as *mut libc::c_void,
            ) != 0 as libc::c_int
            {
                sdsfree(section);
            }
        }
        i += 1;
    }
    return section_dict;
}
#[no_mangle]
pub unsafe extern "C" fn genRedisInfoString(
    mut section_dict: *mut dict,
    mut all_sections: libc::c_int,
    mut everything: libc::c_int,
) -> sds {
    let mut info: sds = sdsempty();
    let mut uptime: time_t = server.unixtime as libc::c_long - server.stat_starttime;
    let mut j: libc::c_int = 0;
    let mut sections: libc::c_int = 0 as libc::c_int;
    if everything != 0 {
        all_sections = 1 as libc::c_int;
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"server\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        static mut call_uname: libc::c_int = 1 as libc::c_int;
        static mut name: utsname = utsname {
            sysname: [0; 65],
            nodename: [0; 65],
            release: [0; 65],
            version: [0; 65],
            machine: [0; 65],
            domainname: [0; 65],
        };
        let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut supervised: *mut libc::c_char = 0 as *mut libc::c_char;
        if server.cluster_enabled != 0 {
            mode = b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        } else if server.sentinel_mode != 0 {
            mode = b"sentinel\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        } else {
            mode = b"standalone\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        if server.supervised != 0 {
            if server.supervised_mode == 3 as libc::c_int {
                supervised = b"upstart\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else if server.supervised_mode == 2 as libc::c_int {
                supervised = b"systemd\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            } else {
                supervised = b"unknown\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char;
            }
        } else {
            supervised = b"no\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char;
        }
        let fresh18 = sections;
        sections = sections + 1;
        if fresh18 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        if call_uname != 0 {
            uname(&mut name);
            call_uname = 0 as libc::c_int;
        }
        let mut lruclock: libc::c_uint = 0;
        lruclock = core::intrinsics::atomic_load_relaxed(&mut server.lruclock);
        info = sdscatfmt(
            info,
            b"# Server\r\nredis_version:%s\r\nredis_git_sha1:%s\r\nredis_git_dirty:%i\r\nredis_build_id:%s\r\nredis_mode:%s\r\nos:%s %s %s\r\narch_bits:%i\r\nmonotonic_clock:%s\r\nmultiplexing_api:%s\r\natomicvar_api:%s\r\ngcc_version:%i.%i.%i\r\nprocess_id:%I\r\nprocess_supervised:%s\r\nrun_id:%s\r\ntcp_port:%i\r\nserver_time_usec:%I\r\nuptime_in_seconds:%I\r\nuptime_in_days:%I\r\nhz:%i\r\nconfigured_hz:%i\r\nlru_clock:%u\r\nexecutable:%s\r\nconfig_file:%s\r\nio_threads_active:%i\r\n\0"
                as *const u8 as *const libc::c_char,
            b"7.0.8\0" as *const u8 as *const libc::c_char,
            redisGitSHA1(),
            (strtol(redisGitDirty(), 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                > 0 as libc::c_int as libc::c_long) as libc::c_int,
            redisBuildIdString(),
            mode,
            (name.sysname).as_mut_ptr(),
            (name.release).as_mut_ptr(),
            (name.machine).as_mut_ptr(),
            server.arch_bits,
            monotonicInfoString(),
            aeGetApiName(),
            b"c11-builtin\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int,
            2 as libc::c_int,
            1 as libc::c_int,
            getpid() as int64_t,
            supervised,
            (server.runid).as_mut_ptr(),
            if server.port != 0 { server.port } else { server.tls_port },
            server.ustime as int64_t,
            uptime,
            uptime / (3600 as libc::c_int * 24 as libc::c_int) as libc::c_long,
            server.hz,
            server.config_hz,
            lruclock,
            if !(server.executable).is_null() {
                server.executable as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            if !(server.configfile).is_null() {
                server.configfile as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            server.io_threads_active,
        );
        if isShutdownInitiated() != 0 {
            info = sdscatfmt(
                info,
                b"shutdown_in_milliseconds:%I\r\n\0" as *const u8 as *const libc::c_char,
                (server.shutdown_mstime - server.mstime) as int64_t,
            );
        }
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"clients\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let mut maxin: size_t = 0;
        let mut maxout: size_t = 0;
        getExpansiveClientsInfo(&mut maxin, &mut maxout);
        let fresh19 = sections;
        sections = sections + 1;
        if fresh19 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Clients\r\nconnected_clients:%lu\r\ncluster_connections:%lu\r\nmaxclients:%u\r\nclient_recent_max_input_buffer:%zu\r\nclient_recent_max_output_buffer:%zu\r\nblocked_clients:%d\r\ntracking_clients:%d\r\nclients_in_timeout_table:%llu\r\n\0"
                as *const u8 as *const libc::c_char,
            ((*server.clients).len).wrapping_sub((*server.slaves).len),
            getClusterConnectionsCount(),
            server.maxclients,
            maxin,
            maxout,
            server.blocked_clients,
            server.tracking_clients,
            raxSize(server.clients_timeout_table) as libc::c_ulonglong,
        );
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"memory\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let mut hmem: [libc::c_char; 64] = [0; 64];
        let mut peak_hmem: [libc::c_char; 64] = [0; 64];
        let mut total_system_hmem: [libc::c_char; 64] = [0; 64];
        let mut used_memory_lua_hmem: [libc::c_char; 64] = [0; 64];
        let mut used_memory_vm_total_hmem: [libc::c_char; 64] = [0; 64];
        let mut used_memory_scripts_hmem: [libc::c_char; 64] = [0; 64];
        let mut used_memory_rss_hmem: [libc::c_char; 64] = [0; 64];
        let mut maxmemory_hmem: [libc::c_char; 64] = [0; 64];
        let mut zmalloc_used: size_t = zmalloc_used_memory();
        let mut total_system_mem: size_t = server.system_memory_size;
        let mut evict_policy: *const libc::c_char = evictPolicyToString();
        let mut memory_lua: libc::c_longlong = evalMemory() as libc::c_longlong;
        let mut memory_functions: libc::c_longlong = functionsMemory()
            as libc::c_longlong;
        let mut mh: *mut redisMemOverhead = getMemoryOverheadData();
        if zmalloc_used > server.stat_peak_memory {
            server.stat_peak_memory = zmalloc_used;
        }
        bytesToHumanInt(hmem.as_mut_ptr(), zmalloc_used as libc::c_ulonglong);
        bytesToHumanInt(
            peak_hmem.as_mut_ptr(),
            server.stat_peak_memory as libc::c_ulonglong,
        );
        bytesToHumanInt(
            total_system_hmem.as_mut_ptr(),
            total_system_mem as libc::c_ulonglong,
        );
        bytesToHumanInt(used_memory_lua_hmem.as_mut_ptr(), memory_lua as libc::c_ulonglong);
        bytesToHumanInt(
            used_memory_vm_total_hmem.as_mut_ptr(),
            (memory_functions + memory_lua) as libc::c_ulonglong,
        );
        bytesToHumanInt(
            used_memory_scripts_hmem.as_mut_ptr(),
            ((*mh).lua_caches).wrapping_add((*mh).functions_caches) as libc::c_ulonglong,
        );
        bytesToHumanInt(
            used_memory_rss_hmem.as_mut_ptr(),
            server.cron_malloc_stats.process_rss as libc::c_ulonglong,
        );
        bytesToHumanInt(maxmemory_hmem.as_mut_ptr(), server.maxmemory);
        let fresh20 = sections;
        sections = sections + 1;
        if fresh20 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Memory\r\nused_memory:%zu\r\nused_memory_human:%s\r\nused_memory_rss:%zu\r\nused_memory_rss_human:%s\r\nused_memory_peak:%zu\r\nused_memory_peak_human:%s\r\nused_memory_peak_perc:%.2f%%\r\nused_memory_overhead:%zu\r\nused_memory_startup:%zu\r\nused_memory_dataset:%zu\r\nused_memory_dataset_perc:%.2f%%\r\nallocator_allocated:%zu\r\nallocator_active:%zu\r\nallocator_resident:%zu\r\ntotal_system_memory:%lu\r\ntotal_system_memory_human:%s\r\nused_memory_lua:%lld\r\nused_memory_vm_eval:%lld\r\nused_memory_lua_human:%s\r\nused_memory_scripts_eval:%lld\r\nnumber_of_cached_scripts:%lu\r\nnumber_of_functions:%lu\r\nnumber_of_libraries:%lu\r\nused_memory_vm_functions:%lld\r\nused_memory_vm_total:%lld\r\nused_memory_vm_total_human:%s\r\nused_memory_functions:%lld\r\nused_memory_scripts:%lld\r\nused_memory_scripts_human:%s\r\nmaxmemory:%lld\r\nmaxmemory_human:%s\r\nmaxmemory_policy:%s\r\nallocator_frag_ratio:%.2f\r\nallocator_frag_bytes:%zu\r\nallocator_rss_ratio:%.2f\r\nallocator_rss_bytes:%zd\r\nrss_overhead_ratio:%.2f\r\nrss_overhead_bytes:%zd\r\nmem_fragmentation_ratio:%.2f\r\nmem_fragmentation_bytes:%zd\r\nmem_not_counted_for_evict:%zu\r\nmem_replication_backlog:%zu\r\nmem_total_replication_buffers:%zu\r\nmem_clients_slaves:%zu\r\nmem_clients_normal:%zu\r\nmem_cluster_links:%zu\r\nmem_aof_buffer:%zu\r\nmem_allocator:%s\r\nactive_defrag_running:%d\r\nlazyfree_pending_objects:%zu\r\nlazyfreed_objects:%zu\r\n\0"
                as *const u8 as *const libc::c_char,
            zmalloc_used,
            hmem.as_mut_ptr(),
            server.cron_malloc_stats.process_rss,
            used_memory_rss_hmem.as_mut_ptr(),
            server.stat_peak_memory,
            peak_hmem.as_mut_ptr(),
            (*mh).peak_perc as libc::c_double,
            (*mh).overhead_total,
            (*mh).startup_allocated,
            (*mh).dataset,
            (*mh).dataset_perc as libc::c_double,
            server.cron_malloc_stats.allocator_allocated,
            server.cron_malloc_stats.allocator_active,
            server.cron_malloc_stats.allocator_resident,
            total_system_mem,
            total_system_hmem.as_mut_ptr(),
            memory_lua,
            memory_lua,
            used_memory_lua_hmem.as_mut_ptr(),
            (*mh).lua_caches as libc::c_longlong,
            ((*evalScriptsDict()).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*evalScriptsDict()).ht_used[1 as libc::c_int as usize]),
            functionsNum(),
            functionsLibNum(),
            memory_functions,
            memory_functions + memory_lua,
            used_memory_vm_total_hmem.as_mut_ptr(),
            (*mh).functions_caches as libc::c_longlong,
            (*mh).lua_caches as libc::c_longlong
                + (*mh).functions_caches as libc::c_longlong,
            used_memory_scripts_hmem.as_mut_ptr(),
            server.maxmemory,
            maxmemory_hmem.as_mut_ptr(),
            evict_policy,
            (*mh).allocator_frag as libc::c_double,
            (*mh).allocator_frag_bytes,
            (*mh).allocator_rss as libc::c_double,
            (*mh).allocator_rss_bytes,
            (*mh).rss_extra as libc::c_double,
            (*mh).rss_extra_bytes,
            (*mh).total_frag as libc::c_double,
            (*mh).total_frag_bytes,
            freeMemoryGetNotCountedMemory(),
            (*mh).repl_backlog,
            server.repl_buffer_mem,
            (*mh).clients_slaves,
            (*mh).clients_normal,
            (*mh).cluster_links,
            (*mh).aof_buffer,
            b"jemalloc-5.2.1\0" as *const u8 as *const libc::c_char,
            server.active_defrag_running,
            lazyfreeGetPendingObjectsCount(),
            lazyfreeGetFreedObjectsCount(),
        );
        freeMemoryOverheadData(mh);
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"persistence\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh21 = sections;
        sections = sections + 1;
        if fresh21 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        let mut fork_perc: libc::c_double = 0 as libc::c_int as libc::c_double;
        if server.stat_module_progress != 0. {
            fork_perc = server.stat_module_progress
                * 100 as libc::c_int as libc::c_double;
        } else if server.stat_current_save_keys_total != 0 {
            fork_perc = server.stat_current_save_keys_processed as libc::c_double
                / server.stat_current_save_keys_total as libc::c_double
                * 100 as libc::c_int as libc::c_double;
        }
        let mut aof_bio_fsync_status: libc::c_int = 0;
        aof_bio_fsync_status = core::intrinsics::atomic_load_relaxed(
            &mut server.aof_bio_fsync_status,
        );
        info = sdscatprintf(
            info,
            b"# Persistence\r\nloading:%d\r\nasync_loading:%d\r\ncurrent_cow_peak:%zu\r\ncurrent_cow_size:%zu\r\ncurrent_cow_size_age:%lu\r\ncurrent_fork_perc:%.2f\r\ncurrent_save_keys_processed:%zu\r\ncurrent_save_keys_total:%zu\r\nrdb_changes_since_last_save:%lld\r\nrdb_bgsave_in_progress:%d\r\nrdb_last_save_time:%jd\r\nrdb_last_bgsave_status:%s\r\nrdb_last_bgsave_time_sec:%jd\r\nrdb_current_bgsave_time_sec:%jd\r\nrdb_saves:%lld\r\nrdb_last_cow_size:%zu\r\nrdb_last_load_keys_expired:%lld\r\nrdb_last_load_keys_loaded:%lld\r\naof_enabled:%d\r\naof_rewrite_in_progress:%d\r\naof_rewrite_scheduled:%d\r\naof_last_rewrite_time_sec:%jd\r\naof_current_rewrite_time_sec:%jd\r\naof_last_bgrewrite_status:%s\r\naof_rewrites:%lld\r\naof_rewrites_consecutive_failures:%lld\r\naof_last_write_status:%s\r\naof_last_cow_size:%zu\r\nmodule_fork_in_progress:%d\r\nmodule_fork_last_cow_size:%zu\r\n\0"
                as *const u8 as *const libc::c_char,
            (server.loading != 0 && server.async_loading == 0) as libc::c_int,
            server.async_loading,
            server.stat_current_cow_peak,
            server.stat_current_cow_bytes,
            if server.stat_current_cow_updated != 0 {
                (elapsedMs(server.stat_current_cow_updated))
                    .wrapping_div(1000 as libc::c_int as libc::c_ulong)
            } else {
                0 as libc::c_int as libc::c_ulong
            },
            fork_perc,
            server.stat_current_save_keys_processed,
            server.stat_current_save_keys_total,
            server.dirty,
            (server.child_type == 1 as libc::c_int) as libc::c_int,
            server.lastsave,
            if server.lastbgsave_status == 0 as libc::c_int {
                b"ok\0" as *const u8 as *const libc::c_char
            } else {
                b"err\0" as *const u8 as *const libc::c_char
            },
            server.rdb_save_time_last,
            if server.child_type != 1 as libc::c_int {
                -(1 as libc::c_int) as libc::c_long
            } else {
                time(0 as *mut time_t) - server.rdb_save_time_start
            },
            server.stat_rdb_saves,
            server.stat_rdb_cow_bytes,
            server.rdb_last_load_keys_expired,
            server.rdb_last_load_keys_loaded,
            (server.aof_state != 0 as libc::c_int) as libc::c_int,
            (server.child_type == 2 as libc::c_int) as libc::c_int,
            server.aof_rewrite_scheduled,
            server.aof_rewrite_time_last,
            if server.child_type != 2 as libc::c_int {
                -(1 as libc::c_int) as libc::c_long
            } else {
                time(0 as *mut time_t) - server.aof_rewrite_time_start
            },
            if server.aof_lastbgrewrite_status == 0 as libc::c_int {
                b"ok\0" as *const u8 as *const libc::c_char
            } else {
                b"err\0" as *const u8 as *const libc::c_char
            },
            server.stat_aof_rewrites,
            server.stat_aofrw_consecutive_failures,
            if server.aof_last_write_status == 0 as libc::c_int
                && aof_bio_fsync_status == 0 as libc::c_int
            {
                b"ok\0" as *const u8 as *const libc::c_char
            } else {
                b"err\0" as *const u8 as *const libc::c_char
            },
            server.stat_aof_cow_bytes,
            (server.child_type == 4 as libc::c_int) as libc::c_int,
            server.stat_module_cow_bytes,
        );
        if server.aof_enabled != 0 {
            info = sdscatprintf(
                info,
                b"aof_current_size:%lld\r\naof_base_size:%lld\r\naof_pending_rewrite:%d\r\naof_buffer_length:%zu\r\naof_pending_bio_fsync:%llu\r\naof_delayed_fsync:%lu\r\n\0"
                    as *const u8 as *const libc::c_char,
                server.aof_current_size as libc::c_longlong,
                server.aof_rewrite_base_size as libc::c_longlong,
                server.aof_rewrite_scheduled,
                sdslen(server.aof_buf),
                bioPendingJobsOfType(1 as libc::c_int),
                server.aof_delayed_fsync,
            );
        }
        if server.loading != 0 {
            let mut perc: libc::c_double = 0 as libc::c_int as libc::c_double;
            let mut eta: time_t = 0;
            let mut elapsed: time_t = 0;
            let mut remaining_bytes: off_t = 1 as libc::c_int as off_t;
            if server.loading_total_bytes != 0 {
                perc = server.loading_loaded_bytes as libc::c_double
                    / server.loading_total_bytes as libc::c_double
                    * 100 as libc::c_int as libc::c_double;
                remaining_bytes = server.loading_total_bytes
                    - server.loading_loaded_bytes;
            } else if server.loading_rdb_used_mem != 0 {
                perc = server.loading_loaded_bytes as libc::c_double
                    / server.loading_rdb_used_mem as libc::c_double
                    * 100 as libc::c_int as libc::c_double;
                remaining_bytes = server.loading_rdb_used_mem
                    - server.loading_loaded_bytes;
                if perc > 99.99f64 {
                    perc = 99.99f64;
                }
                if remaining_bytes < 1 as libc::c_int as libc::c_long {
                    remaining_bytes = 1 as libc::c_int as off_t;
                }
            }
            elapsed = time(0 as *mut time_t) - server.loading_start_time;
            if elapsed == 0 as libc::c_int as libc::c_long {
                eta = 1 as libc::c_int as time_t;
            } else {
                eta = elapsed * remaining_bytes
                    / (server.loading_loaded_bytes + 1 as libc::c_int as libc::c_long);
            }
            info = sdscatprintf(
                info,
                b"loading_start_time:%jd\r\nloading_total_bytes:%llu\r\nloading_rdb_used_mem:%llu\r\nloading_loaded_bytes:%llu\r\nloading_loaded_perc:%.2f\r\nloading_eta_seconds:%jd\r\n\0"
                    as *const u8 as *const libc::c_char,
                server.loading_start_time,
                server.loading_total_bytes as libc::c_ulonglong,
                server.loading_rdb_used_mem as libc::c_ulonglong,
                server.loading_loaded_bytes as libc::c_ulonglong,
                perc,
                eta,
            );
        }
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"stats\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let mut stat_total_reads_processed: libc::c_longlong = 0;
        let mut stat_total_writes_processed: libc::c_longlong = 0;
        let mut stat_net_input_bytes: libc::c_longlong = 0;
        let mut stat_net_output_bytes: libc::c_longlong = 0;
        let mut stat_net_repl_input_bytes: libc::c_longlong = 0;
        let mut stat_net_repl_output_bytes: libc::c_longlong = 0;
        let mut current_eviction_exceeded_time: libc::c_longlong = if server
            .stat_last_eviction_exceeded_time != 0
        {
            elapsedUs(server.stat_last_eviction_exceeded_time) as libc::c_longlong
        } else {
            0 as libc::c_int as libc::c_longlong
        };
        let mut current_active_defrag_time: libc::c_longlong = if server
            .stat_last_active_defrag_time != 0
        {
            elapsedUs(server.stat_last_active_defrag_time) as libc::c_longlong
        } else {
            0 as libc::c_int as libc::c_longlong
        };
        stat_total_reads_processed = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_total_reads_processed,
        );
        stat_total_writes_processed = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_total_writes_processed,
        );
        stat_net_input_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_input_bytes,
        );
        stat_net_output_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_output_bytes,
        );
        stat_net_repl_input_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_repl_input_bytes,
        );
        stat_net_repl_output_bytes = core::intrinsics::atomic_load_relaxed(
            &mut server.stat_net_repl_output_bytes,
        );
        let fresh22 = sections;
        sections = sections + 1;
        if fresh22 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Stats\r\ntotal_connections_received:%lld\r\ntotal_commands_processed:%lld\r\ninstantaneous_ops_per_sec:%lld\r\ntotal_net_input_bytes:%lld\r\ntotal_net_output_bytes:%lld\r\ntotal_net_repl_input_bytes:%lld\r\ntotal_net_repl_output_bytes:%lld\r\ninstantaneous_input_kbps:%.2f\r\ninstantaneous_output_kbps:%.2f\r\ninstantaneous_input_repl_kbps:%.2f\r\ninstantaneous_output_repl_kbps:%.2f\r\nrejected_connections:%lld\r\nsync_full:%lld\r\nsync_partial_ok:%lld\r\nsync_partial_err:%lld\r\nexpired_keys:%lld\r\nexpired_stale_perc:%.2f\r\nexpired_time_cap_reached_count:%lld\r\nexpire_cycle_cpu_milliseconds:%lld\r\nevicted_keys:%lld\r\nevicted_clients:%lld\r\ntotal_eviction_exceeded_time:%lld\r\ncurrent_eviction_exceeded_time:%lld\r\nkeyspace_hits:%lld\r\nkeyspace_misses:%lld\r\npubsub_channels:%ld\r\npubsub_patterns:%lu\r\npubsubshard_channels:%lu\r\nlatest_fork_usec:%lld\r\ntotal_forks:%lld\r\nmigrate_cached_sockets:%ld\r\nslave_expires_tracked_keys:%zu\r\nactive_defrag_hits:%lld\r\nactive_defrag_misses:%lld\r\nactive_defrag_key_hits:%lld\r\nactive_defrag_key_misses:%lld\r\ntotal_active_defrag_time:%lld\r\ncurrent_active_defrag_time:%lld\r\ntracking_total_keys:%lld\r\ntracking_total_items:%lld\r\ntracking_total_prefixes:%lld\r\nunexpected_error_replies:%lld\r\ntotal_error_replies:%lld\r\ndump_payload_sanitizations:%lld\r\ntotal_reads_processed:%lld\r\ntotal_writes_processed:%lld\r\nio_threaded_reads_processed:%lld\r\nio_threaded_writes_processed:%lld\r\nreply_buffer_shrinks:%lld\r\nreply_buffer_expands:%lld\r\n\0"
                as *const u8 as *const libc::c_char,
            server.stat_numconnections,
            server.stat_numcommands,
            getInstantaneousMetric(0 as libc::c_int),
            stat_net_input_bytes + stat_net_repl_input_bytes,
            stat_net_output_bytes + stat_net_repl_output_bytes,
            stat_net_repl_input_bytes,
            stat_net_repl_output_bytes,
            (getInstantaneousMetric(1 as libc::c_int) as libc::c_float
                / 1024 as libc::c_int as libc::c_float) as libc::c_double,
            (getInstantaneousMetric(2 as libc::c_int) as libc::c_float
                / 1024 as libc::c_int as libc::c_float) as libc::c_double,
            (getInstantaneousMetric(3 as libc::c_int) as libc::c_float
                / 1024 as libc::c_int as libc::c_float) as libc::c_double,
            (getInstantaneousMetric(4 as libc::c_int) as libc::c_float
                / 1024 as libc::c_int as libc::c_float) as libc::c_double,
            server.stat_rejected_conn,
            server.stat_sync_full,
            server.stat_sync_partial_ok,
            server.stat_sync_partial_err,
            server.stat_expiredkeys,
            server.stat_expired_stale_perc * 100 as libc::c_int as libc::c_double,
            server.stat_expired_time_cap_reached_count,
            server.stat_expire_cycle_time_used / 1000 as libc::c_int as libc::c_longlong,
            server.stat_evictedkeys,
            server.stat_evictedclients,
            (server.stat_total_eviction_exceeded_time + current_eviction_exceeded_time)
                / 1000 as libc::c_int as libc::c_longlong,
            current_eviction_exceeded_time / 1000 as libc::c_int as libc::c_longlong,
            server.stat_keyspace_hits,
            server.stat_keyspace_misses,
            ((*server.pubsub_channels).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.pubsub_channels).ht_used[1 as libc::c_int as usize],
                ),
            ((*server.pubsub_patterns).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.pubsub_patterns).ht_used[1 as libc::c_int as usize],
                ),
            ((*server.pubsubshard_channels).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.pubsubshard_channels).ht_used[1 as libc::c_int as usize],
                ),
            server.stat_fork_time,
            server.stat_total_forks,
            ((*server.migrate_cached_sockets).ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*server.migrate_cached_sockets).ht_used[1 as libc::c_int as usize],
                ),
            getSlaveKeyWithExpireCount(),
            server.stat_active_defrag_hits,
            server.stat_active_defrag_misses,
            server.stat_active_defrag_key_hits,
            server.stat_active_defrag_key_misses,
            (server.stat_total_active_defrag_time + current_active_defrag_time)
                / 1000 as libc::c_int as libc::c_longlong,
            current_active_defrag_time / 1000 as libc::c_int as libc::c_longlong,
            trackingGetTotalKeys() as libc::c_ulonglong,
            trackingGetTotalItems() as libc::c_ulonglong,
            trackingGetTotalPrefixes() as libc::c_ulonglong,
            server.stat_unexpected_error_replies,
            server.stat_total_error_replies,
            server.stat_dump_payload_sanitizations,
            stat_total_reads_processed,
            stat_total_writes_processed,
            server.stat_io_reads_processed,
            server.stat_io_writes_processed,
            server.stat_reply_buffer_shrinks,
            server.stat_reply_buffer_expands,
        );
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"replication\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh23 = sections;
        sections = sections + 1;
        if fresh23 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Replication\r\nrole:%s\r\n\0" as *const u8 as *const libc::c_char,
            if (server.masterhost).is_null() {
                b"master\0" as *const u8 as *const libc::c_char
            } else {
                b"slave\0" as *const u8 as *const libc::c_char
            },
        );
        if !(server.masterhost).is_null() {
            let mut slave_repl_offset: libc::c_longlong = 1 as libc::c_int
                as libc::c_longlong;
            let mut slave_read_repl_offset: libc::c_longlong = 1 as libc::c_int
                as libc::c_longlong;
            if !(server.master).is_null() {
                slave_repl_offset = (*server.master).reploff;
                slave_read_repl_offset = (*server.master).read_reploff;
            } else if !(server.cached_master).is_null() {
                slave_repl_offset = (*server.cached_master).reploff;
                slave_read_repl_offset = (*server.cached_master).read_reploff;
            }
            info = sdscatprintf(
                info,
                b"master_host:%s\r\nmaster_port:%d\r\nmaster_link_status:%s\r\nmaster_last_io_seconds_ago:%d\r\nmaster_sync_in_progress:%d\r\nslave_read_repl_offset:%lld\r\nslave_repl_offset:%lld\r\n\0"
                    as *const u8 as *const libc::c_char,
                server.masterhost,
                server.masterport,
                if server.repl_state == REPL_STATE_CONNECTED as libc::c_int {
                    b"up\0" as *const u8 as *const libc::c_char
                } else {
                    b"down\0" as *const u8 as *const libc::c_char
                },
                if !(server.master).is_null() {
                    (server.unixtime as libc::c_long - (*server.master).lastinteraction)
                        as libc::c_int
                } else {
                    -(1 as libc::c_int)
                },
                (server.repl_state == REPL_STATE_TRANSFER as libc::c_int) as libc::c_int,
                slave_read_repl_offset,
                slave_repl_offset,
            );
            if server.repl_state == REPL_STATE_TRANSFER as libc::c_int {
                let mut perc_0: libc::c_double = 0 as libc::c_int as libc::c_double;
                if server.repl_transfer_size != 0 {
                    perc_0 = server.repl_transfer_read as libc::c_double
                        / server.repl_transfer_size as libc::c_double
                        * 100 as libc::c_int as libc::c_double;
                }
                info = sdscatprintf(
                    info,
                    b"master_sync_total_bytes:%lld\r\nmaster_sync_read_bytes:%lld\r\nmaster_sync_left_bytes:%lld\r\nmaster_sync_perc:%.2f\r\nmaster_sync_last_io_seconds_ago:%d\r\n\0"
                        as *const u8 as *const libc::c_char,
                    server.repl_transfer_size as libc::c_longlong,
                    server.repl_transfer_read as libc::c_longlong,
                    (server.repl_transfer_size - server.repl_transfer_read)
                        as libc::c_longlong,
                    perc_0,
                    (server.unixtime as libc::c_long - server.repl_transfer_lastio)
                        as libc::c_int,
                );
            }
            if server.repl_state != REPL_STATE_CONNECTED as libc::c_int {
                info = sdscatprintf(
                    info,
                    b"master_link_down_since_seconds:%jd\r\n\0" as *const u8
                        as *const libc::c_char,
                    if server.repl_down_since != 0 {
                        server.unixtime as libc::c_long - server.repl_down_since
                    } else {
                        -(1 as libc::c_int) as libc::c_long
                    },
                );
            }
            info = sdscatprintf(
                info,
                b"slave_priority:%d\r\nslave_read_only:%d\r\nreplica_announced:%d\r\n\0"
                    as *const u8 as *const libc::c_char,
                server.slave_priority,
                server.repl_slave_ro,
                server.replica_announced,
            );
        }
        info = sdscatprintf(
            info,
            b"connected_slaves:%lu\r\n\0" as *const u8 as *const libc::c_char,
            (*server.slaves).len,
        );
        if server.repl_min_slaves_to_write != 0 && server.repl_min_slaves_max_lag != 0 {
            info = sdscatprintf(
                info,
                b"min_slaves_good_slaves:%d\r\n\0" as *const u8 as *const libc::c_char,
                server.repl_good_slaves_count,
            );
        }
        if (*server.slaves).len != 0 {
            let mut slaveid: libc::c_int = 0 as libc::c_int;
            let mut ln: *mut listNode = 0 as *mut listNode;
            let mut li: listIter = listIter {
                next: 0 as *mut listNode,
                direction: 0,
            };
            listRewind(server.slaves, &mut li);
            loop {
                ln = listNext(&mut li);
                if ln.is_null() {
                    break;
                }
                let mut slave: *mut client = (*ln).value as *mut client;
                let mut ip: [libc::c_char; 46] = [0; 46];
                let mut slaveip: *mut libc::c_char = (*slave).slave_addr;
                let mut port: libc::c_int = 0;
                let mut lag: libc::c_long = 0 as libc::c_int as libc::c_long;
                if slaveip.is_null() {
                    if connPeerToString(
                        (*slave).conn,
                        ip.as_mut_ptr(),
                        core::mem::size_of::<[libc::c_char; 46]>() as libc::c_ulong,
                        &mut port,
                    ) == -(1 as libc::c_int)
                    {
                        continue;
                    }
                    slaveip = ip.as_mut_ptr();
                }
                let mut state: *const libc::c_char = replstateToString(
                    (*slave).replstate,
                );
                if *state.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32
                {
                    continue;
                }
                if (*slave).replstate == 9 as libc::c_int {
                    lag = (time(0 as *mut time_t) as libc::c_longlong
                        - (*slave).repl_ack_time) as libc::c_long;
                }
                info = sdscatprintf(
                    info,
                    b"slave%d:ip=%s,port=%d,state=%s,offset=%lld,lag=%ld\r\n\0"
                        as *const u8 as *const libc::c_char,
                    slaveid,
                    slaveip,
                    (*slave).slave_listening_port,
                    state,
                    (*slave).repl_ack_off,
                    lag,
                );
                slaveid += 1;
            }
        }
        info = sdscatprintf(
            info,
            b"master_failover_state:%s\r\nmaster_replid:%s\r\nmaster_replid2:%s\r\nmaster_repl_offset:%lld\r\nsecond_repl_offset:%lld\r\nrepl_backlog_active:%d\r\nrepl_backlog_size:%lld\r\nrepl_backlog_first_byte_offset:%lld\r\nrepl_backlog_histlen:%lld\r\n\0"
                as *const u8 as *const libc::c_char,
            getFailoverStateString(),
            (server.replid).as_mut_ptr(),
            (server.replid2).as_mut_ptr(),
            server.master_repl_offset,
            server.second_replid_offset,
            (server.repl_backlog != 0 as *mut libc::c_void as *mut replBacklog)
                as libc::c_int,
            server.repl_backlog_size,
            if !(server.repl_backlog).is_null() {
                (*server.repl_backlog).offset
            } else {
                0 as libc::c_int as libc::c_longlong
            },
            if !(server.repl_backlog).is_null() {
                (*server.repl_backlog).histlen
            } else {
                0 as libc::c_int as libc::c_longlong
            },
        );
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"cpu\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh24 = sections;
        sections = sections + 1;
        if fresh24 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        let mut self_ru: rusage = rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            c2rust_unnamed: C2RustUnnamed_39 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2RustUnnamed_38 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2RustUnnamed_37 { ru_idrss: 0 },
            c2rust_unnamed_2: C2RustUnnamed_36 { ru_isrss: 0 },
            c2rust_unnamed_3: C2RustUnnamed_35 { ru_minflt: 0 },
            c2rust_unnamed_4: C2RustUnnamed_34 { ru_majflt: 0 },
            c2rust_unnamed_5: C2RustUnnamed_33 { ru_nswap: 0 },
            c2rust_unnamed_6: C2RustUnnamed_32 { ru_inblock: 0 },
            c2rust_unnamed_7: C2RustUnnamed_31 { ru_oublock: 0 },
            c2rust_unnamed_8: C2RustUnnamed_30 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2RustUnnamed_29 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2RustUnnamed_28 { ru_nsignals: 0 },
            c2rust_unnamed_11: C2RustUnnamed_27 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2RustUnnamed_26 { ru_nivcsw: 0 },
        };
        let mut c_ru: rusage = rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            c2rust_unnamed: C2RustUnnamed_39 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2RustUnnamed_38 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2RustUnnamed_37 { ru_idrss: 0 },
            c2rust_unnamed_2: C2RustUnnamed_36 { ru_isrss: 0 },
            c2rust_unnamed_3: C2RustUnnamed_35 { ru_minflt: 0 },
            c2rust_unnamed_4: C2RustUnnamed_34 { ru_majflt: 0 },
            c2rust_unnamed_5: C2RustUnnamed_33 { ru_nswap: 0 },
            c2rust_unnamed_6: C2RustUnnamed_32 { ru_inblock: 0 },
            c2rust_unnamed_7: C2RustUnnamed_31 { ru_oublock: 0 },
            c2rust_unnamed_8: C2RustUnnamed_30 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2RustUnnamed_29 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2RustUnnamed_28 { ru_nsignals: 0 },
            c2rust_unnamed_11: C2RustUnnamed_27 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2RustUnnamed_26 { ru_nivcsw: 0 },
        };
        getrusage(RUSAGE_SELF, &mut self_ru);
        getrusage(RUSAGE_CHILDREN, &mut c_ru);
        info = sdscatprintf(
            info,
            b"# CPU\r\nused_cpu_sys:%ld.%06ld\r\nused_cpu_user:%ld.%06ld\r\nused_cpu_sys_children:%ld.%06ld\r\nused_cpu_user_children:%ld.%06ld\r\n\0"
                as *const u8 as *const libc::c_char,
            self_ru.ru_stime.tv_sec,
            self_ru.ru_stime.tv_usec,
            self_ru.ru_utime.tv_sec,
            self_ru.ru_utime.tv_usec,
            c_ru.ru_stime.tv_sec,
            c_ru.ru_stime.tv_usec,
            c_ru.ru_utime.tv_sec,
            c_ru.ru_utime.tv_usec,
        );
        let mut m_ru: rusage = rusage {
            ru_utime: timeval { tv_sec: 0, tv_usec: 0 },
            ru_stime: timeval { tv_sec: 0, tv_usec: 0 },
            c2rust_unnamed: C2RustUnnamed_39 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2RustUnnamed_38 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2RustUnnamed_37 { ru_idrss: 0 },
            c2rust_unnamed_2: C2RustUnnamed_36 { ru_isrss: 0 },
            c2rust_unnamed_3: C2RustUnnamed_35 { ru_minflt: 0 },
            c2rust_unnamed_4: C2RustUnnamed_34 { ru_majflt: 0 },
            c2rust_unnamed_5: C2RustUnnamed_33 { ru_nswap: 0 },
            c2rust_unnamed_6: C2RustUnnamed_32 { ru_inblock: 0 },
            c2rust_unnamed_7: C2RustUnnamed_31 { ru_oublock: 0 },
            c2rust_unnamed_8: C2RustUnnamed_30 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2RustUnnamed_29 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2RustUnnamed_28 { ru_nsignals: 0 },
            c2rust_unnamed_11: C2RustUnnamed_27 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2RustUnnamed_26 { ru_nivcsw: 0 },
        };
        getrusage(RUSAGE_THREAD, &mut m_ru);
        info = sdscatprintf(
            info,
            b"used_cpu_sys_main_thread:%ld.%06ld\r\nused_cpu_user_main_thread:%ld.%06ld\r\n\0"
                as *const u8 as *const libc::c_char,
            m_ru.ru_stime.tv_sec,
            m_ru.ru_stime.tv_usec,
            m_ru.ru_utime.tv_sec,
            m_ru.ru_utime.tv_usec,
        );
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"module_list\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
        || !(dictFind(
            section_dict,
            b"modules\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh25 = sections;
        sections = sections + 1;
        if fresh25 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Modules\r\n\0" as *const u8 as *const libc::c_char,
        );
        info = genModulesInfoString(info);
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"commandstats\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh26 = sections;
        sections = sections + 1;
        if fresh26 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Commandstats\r\n\0" as *const u8 as *const libc::c_char,
        );
        info = genRedisInfoStringCommandStats(info, server.commands);
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"errorstats\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh27 = sections;
        sections = sections + 1;
        if fresh27 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscat(info, b"# Errorstats\r\n\0" as *const u8 as *const libc::c_char);
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
        raxStart(&mut ri, server.errors);
        raxSeek(
            &mut ri,
            b"^\0" as *const u8 as *const libc::c_char,
            0 as *mut libc::c_uchar,
            0 as libc::c_int as size_t,
        );
        let mut e: *mut redisError = 0 as *mut redisError;
        while raxNext(&mut ri) != 0 {
            let mut tmpsafe: *mut libc::c_char = 0 as *mut libc::c_char;
            e = ri.data as *mut redisError;
            info = sdscatprintf(
                info,
                b"errorstat_%.*s:count=%lld\r\n\0" as *const u8 as *const libc::c_char,
                ri.key_len as libc::c_int,
                getSafeInfoString(ri.key as *mut libc::c_char, ri.key_len, &mut tmpsafe),
                (*e).count,
            );
            if !tmpsafe.is_null() {
                zfree(tmpsafe as *mut libc::c_void);
            }
        }
        raxStop(&mut ri);
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"latencystats\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh28 = sections;
        sections = sections + 1;
        if fresh28 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Latencystats\r\n\0" as *const u8 as *const libc::c_char,
        );
        if server.latency_tracking_enabled != 0 {
            info = genRedisInfoStringLatencyStats(info, server.commands);
        }
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"cluster\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh29 = sections;
        sections = sections + 1;
        if fresh29 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Cluster\r\ncluster_enabled:%d\r\n\0" as *const u8 as *const libc::c_char,
            server.cluster_enabled,
        );
    }
    if all_sections != 0
        || !(dictFind(
            section_dict,
            b"keyspace\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
    {
        let fresh30 = sections;
        sections = sections + 1;
        if fresh30 != 0 {
            info = sdscat(info, b"\r\n\0" as *const u8 as *const libc::c_char);
        }
        info = sdscatprintf(
            info,
            b"# Keyspace\r\n\0" as *const u8 as *const libc::c_char,
        );
        j = 0 as libc::c_int;
        while j < server.dbnum {
            let mut keys: libc::c_longlong = 0;
            let mut vkeys: libc::c_longlong = 0;
            keys = ((*(*(server.db).offset(j as isize)).dict)
                .ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*(server.db).offset(j as isize)).dict)
                        .ht_used[1 as libc::c_int as usize],
                ) as libc::c_longlong;
            vkeys = ((*(*(server.db).offset(j as isize)).expires)
                .ht_used[0 as libc::c_int as usize])
                .wrapping_add(
                    (*(*(server.db).offset(j as isize)).expires)
                        .ht_used[1 as libc::c_int as usize],
                ) as libc::c_longlong;
            if keys != 0 || vkeys != 0 {
                info = sdscatprintf(
                    info,
                    b"db%d:keys=%lld,expires=%lld,avg_ttl=%lld\r\n\0" as *const u8
                        as *const libc::c_char,
                    j,
                    keys,
                    vkeys,
                    (*(server.db).offset(j as isize)).avg_ttl,
                );
            }
            j += 1;
        }
    }
    if everything != 0
        || !(dictFind(
            section_dict,
            b"modules\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        ))
            .is_null()
        || sections
            < ((*section_dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*section_dict).ht_used[1 as libc::c_int as usize])
                as libc::c_int
        || all_sections != 0
            && ((*section_dict).ht_used[0 as libc::c_int as usize])
                .wrapping_add((*section_dict).ht_used[1 as libc::c_int as usize]) != 0
    {
        info = modulesCollectInfo(
            info,
            if everything != 0
                || !(dictFind(
                    section_dict,
                    b"modules\0" as *const u8 as *const libc::c_char
                        as *const libc::c_void,
                ))
                    .is_null()
            {
                0 as *mut dict
            } else {
                section_dict
            },
            0 as libc::c_int,
            sections,
        );
    }
    return info;
}
#[no_mangle]
pub unsafe extern "C" fn infoCommand(mut c: *mut client) {
    if server.sentinel_mode != 0 {
        sentinelInfoCommand(c);
        return;
    }
    let mut all_sections: libc::c_int = 0 as libc::c_int;
    let mut everything: libc::c_int = 0 as libc::c_int;
    let mut sections_dict: *mut dict = genInfoSectionDict(
        ((*c).argv).offset(1 as libc::c_int as isize),
        (*c).argc - 1 as libc::c_int,
        0 as *mut *mut libc::c_char,
        &mut all_sections,
        &mut everything,
    );
    let mut info: sds = genRedisInfoString(sections_dict, all_sections, everything);
    addReplyVerbatim(
        c,
        info as *const libc::c_char,
        sdslen(info),
        b"txt\0" as *const u8 as *const libc::c_char,
    );
    sdsfree(info);
    releaseInfoSectionDict(sections_dict);
}
#[no_mangle]
pub unsafe extern "C" fn monitorCommand(mut c: *mut client) {
    if (*c).flags as libc::c_ulonglong & (1 as libc::c_ulonglong) << 41 as libc::c_int
        != 0
    {
        addReplyError(
            c,
            b"MONITOR isn't allowed for DENY BLOCKING client\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*c).flags & ((1 as libc::c_int) << 0 as libc::c_int) as libc::c_ulong != 0 {
        return;
    }
    (*c).flags
        |= ((1 as libc::c_int) << 0 as libc::c_int
            | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_ulong;
    listAddNodeTail(server.monitors, c as *mut libc::c_void);
    addReply(c, shared.ok);
}
#[no_mangle]
pub unsafe extern "C" fn checkIgnoreWarning(
    mut warning: *const libc::c_char,
) -> libc::c_int {
    let mut argc: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut argv: *mut sds = sdssplitargs(server.ignore_warnings, &mut argc);
    if argv.is_null() {
        return 0 as libc::c_int;
    }
    j = 0 as libc::c_int;
    while j < argc {
        let mut flag: *mut libc::c_char = *argv.offset(j as isize);
        if strcasecmp(flag, warning) == 0 {
            break;
        }
        j += 1;
    }
    sdsfreesplitres(argv, argc);
    return (j < argc) as libc::c_int;
}
unsafe extern "C" fn THPDisable() -> libc::c_int {
    let mut ret: libc::c_int = -(22 as libc::c_int);
    if server.disable_thp == 0 {
        return ret;
    }
    ret = prctl(
        41 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn linuxMemoryWarnings() {
    let mut err_msg: sds = 0 as sds;
    if checkOvercommit(&mut err_msg) < 0 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"WARNING %s\0" as *const u8 as *const libc::c_char,
                err_msg,
            );
        }
        sdsfree(err_msg);
    }
    if checkTHPEnabled(&mut err_msg) < 0 as libc::c_int {
        server.thp_enabled = 1 as libc::c_int;
        if THPDisable() == 0 as libc::c_int {
            server.thp_enabled = 0 as libc::c_int;
        } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"WARNING %s\0" as *const u8 as *const libc::c_char,
                err_msg,
            );
        }
        sdsfree(err_msg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn createPidFile() {
    if (server.pidfile).is_null() {
        server
            .pidfile = zstrdup(
            b"/var/run/redis.pid\0" as *const u8 as *const libc::c_char,
        );
    }
    let mut fp: *mut FILE = fopen(
        server.pidfile,
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if !fp.is_null() {
        fprintf(fp, b"%d\n\0" as *const u8 as *const libc::c_char, getpid());
        fclose(fp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn daemonize() {
    let mut fd: libc::c_int = 0;
    if fork() != 0 as libc::c_int {
        exit(0 as libc::c_int);
    }
    setsid();
    fd = open(
        b"/dev/null\0" as *const u8 as *const libc::c_char,
        0o2 as libc::c_int,
        0 as libc::c_int,
    );
    if fd != -(1 as libc::c_int) {
        dup2(fd, 0 as libc::c_int);
        dup2(fd, 1 as libc::c_int);
        dup2(fd, 2 as libc::c_int);
        if fd > 2 as libc::c_int {
            close(fd);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn version() {
    printf(
        b"Redis server v=%s sha=%s:%d malloc=%s bits=%d build=%llx\n\0" as *const u8
            as *const libc::c_char,
        b"7.0.8\0" as *const u8 as *const libc::c_char,
        redisGitSHA1(),
        (atoi(redisGitDirty()) > 0 as libc::c_int) as libc::c_int,
        b"jemalloc-5.2.1\0" as *const u8 as *const libc::c_char,
        if core::mem::size_of::<libc::c_long>() as libc::c_ulong
            == 4 as libc::c_int as libc::c_ulong
        {
            32 as libc::c_int
        } else {
            64 as libc::c_int
        },
        redisBuildId() as libc::c_ulonglong,
    );
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        b"Usage: ./redis-server [/path/to/redis.conf] [options] [-]\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server - (read config from stdin)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server -v or --version\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server -h or --help\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server --test-memory <megabytes>\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server --check-system\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fprintf(stderr, b"Examples:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"       ./redis-server (run the server with default conf)\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       echo 'maxmemory 128mb' | ./redis-server -\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server /etc/redis/6379.conf\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server --port 7777\n\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server --port 7777 --replicaof 127.0.0.1 8888\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server /etc/myredis.conf --loglevel verbose -\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"       ./redis-server /etc/myredis.conf --loglevel verbose\n\n\0" as *const u8
            as *const libc::c_char,
    );
    fprintf(stderr, b"Sentinel mode:\n\0" as *const u8 as *const libc::c_char);
    fprintf(
        stderr,
        b"       ./redis-server /etc/sentinel.conf --sentinel\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn redisAsciiArt() {
    let mut ascii_logo: *const libc::c_char = b"                _._                                                  \n           _.-``__ ''-._                                             \n      _.-``    `.  `_.  ''-._           Redis %s (%s/%d) %s bit\n  .-`` .-```.  ```\\/    _.,_ ''-._                                  \n (    '      ,       .-`  | `,    )     Running in %s mode\n |`-._`-...-` __...-.``-._|'` _.-'|     Port: %d\n |    `-._   `._    /     _.-'    |     PID: %ld\n  `-._    `-._  `-./  _.-'    _.-'                                   \n |`-._`-._    `-.__.-'    _.-'_.-'|                                  \n |    `-._`-._        _.-'_.-'    |           https://redis.io       \n  `-._    `-._`-.__.-'_.-'    _.-'                                   \n |`-._`-._    `-.__.-'    _.-'_.-'|                                  \n |    `-._`-._        _.-'_.-'    |                                  \n  `-._    `-._`-.__.-'_.-'    _.-'                                   \n      `-._    `-.__.-'    _.-'                                       \n          `-._        _.-'                                           \n              `-.__.-'                                               \n\n\0"
        as *const u8 as *const libc::c_char;
    let mut buf: *mut libc::c_char = zmalloc(
        (1024 as libc::c_int * 16 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    let mut mode: *mut libc::c_char = 0 as *mut libc::c_char;
    if server.cluster_enabled != 0 {
        mode = b"cluster\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else if server.sentinel_mode != 0 {
        mode = b"sentinel\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    } else {
        mode = b"standalone\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    }
    let mut show_logo: libc::c_int = (server.syslog_enabled == 0
        && *(server.logfile).offset(0 as libc::c_int as isize) as libc::c_int
            == '\0' as i32 && isatty(fileno(stdout)) != 0
        || server.always_show_logo != 0) as libc::c_int;
    if show_logo == 0 {
        if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                2 as libc::c_int,
                b"Running mode=%s, port=%d.\0" as *const u8 as *const libc::c_char,
                mode,
                if server.port != 0 { server.port } else { server.tls_port },
            );
        }
    } else {
        snprintf(
            buf,
            (1024 as libc::c_int * 16 as libc::c_int) as libc::c_ulong,
            ascii_logo,
            b"7.0.8\0" as *const u8 as *const libc::c_char,
            redisGitSHA1(),
            (strtol(redisGitDirty(), 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                > 0 as libc::c_int as libc::c_long) as libc::c_int,
            if core::mem::size_of::<libc::c_long>() as libc::c_ulong
                == 8 as libc::c_int as libc::c_ulong
            {
                b"64\0" as *const u8 as *const libc::c_char
            } else {
                b"32\0" as *const u8 as *const libc::c_char
            },
            mode,
            if server.port != 0 { server.port } else { server.tls_port },
            getpid() as libc::c_long,
        );
        serverLogRaw(2 as libc::c_int | (1 as libc::c_int) << 10 as libc::c_int, buf);
    }
    zfree(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn changeBindAddr() -> libc::c_int {
    closeSocketListeners(&mut server.ipfd);
    closeSocketListeners(&mut server.tlsfd);
    if server.port != 0 as libc::c_int
        && listenToPort(server.port, &mut server.ipfd) != 0 as libc::c_int
        || server.tls_port != 0 as libc::c_int
            && listenToPort(server.tls_port, &mut server.tlsfd) != 0 as libc::c_int
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Failed to bind\0" as *const u8 as *const libc::c_char,
            );
        }
        closeSocketListeners(&mut server.ipfd);
        closeSocketListeners(&mut server.tlsfd);
        return -(1 as libc::c_int);
    }
    if createSocketAcceptHandler(
        &mut server.ipfd,
        Some(
            acceptTcpHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) != 0 as libc::c_int
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            6251 as libc::c_int,
            b"Unrecoverable error creating TCP socket accept handler.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    if createSocketAcceptHandler(
        &mut server.tlsfd,
        Some(
            acceptTLSHandler
                as unsafe extern "C" fn(
                    *mut aeEventLoop,
                    libc::c_int,
                    *mut libc::c_void,
                    libc::c_int,
                ) -> (),
        ),
    ) != 0 as libc::c_int
    {
        _serverPanic(
            b"server.c\0" as *const u8 as *const libc::c_char,
            6254 as libc::c_int,
            b"Unrecoverable error creating TLS socket accept handler.\0" as *const u8
                as *const libc::c_char,
        );
        unreachable!();
    }
    if server.set_proc_title != 0 {
        redisSetProcTitle(0 as *mut libc::c_char);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn changeListenPort(
    mut port: libc::c_int,
    mut sfd: *mut socketFds,
    mut accept_handler: Option::<aeFileProc>,
) -> libc::c_int {
    let mut new_sfd: socketFds = {
        let mut init = socketFds {
            fd: [0 as libc::c_int, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            count: 0,
        };
        init
    };
    closeSocketListeners(sfd);
    if port == 0 as libc::c_int {
        if server.set_proc_title != 0 {
            redisSetProcTitle(0 as *mut libc::c_char);
        }
        return 0 as libc::c_int;
    }
    if listenToPort(port, &mut new_sfd) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if createSocketAcceptHandler(&mut new_sfd, accept_handler) != 0 as libc::c_int {
        closeSocketListeners(&mut new_sfd);
        return -(1 as libc::c_int);
    }
    (*sfd).count = new_sfd.count;
    memcpy(
        ((*sfd).fd).as_mut_ptr() as *mut libc::c_void,
        (new_sfd.fd).as_mut_ptr() as *const libc::c_void,
        core::mem::size_of::<[libc::c_int; 16]>() as libc::c_ulong,
    );
    if server.set_proc_title != 0 {
        redisSetProcTitle(0 as *mut libc::c_char);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn sigShutdownHandler(mut sig: libc::c_int) {
    let mut msg: *mut libc::c_char = 0 as *mut libc::c_char;
    match sig {
        2 => {
            msg = b"Received SIGINT scheduling shutdown...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        15 => {
            msg = b"Received SIGTERM scheduling shutdown...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
        _ => {
            msg = b"Received shutdown signal, scheduling shutdown...\0" as *const u8
                as *const libc::c_char as *mut libc::c_char;
        }
    }
    if server.shutdown_asap != 0 && sig == 2 as libc::c_int {
        serverLogFromHandler(
            3 as libc::c_int,
            b"You insist... exiting now.\0" as *const u8 as *const libc::c_char,
        );
        rdbRemoveTempFile(getpid(), 1 as libc::c_int);
        exit(1 as libc::c_int);
    } else {
        if server.loading != 0 {
            msg = b"Received shutdown signal during loading, scheduling shutdown.\0"
                as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    serverLogFromHandler(3 as libc::c_int, msg);
    core::ptr::write_volatile(
        &mut server.shutdown_asap as *mut sig_atomic_t,
        1 as libc::c_int,
    );
    server.last_sig_received = sig;
}
#[no_mangle]
pub unsafe extern "C" fn setupSignalHandlers() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_12 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(
        sigShutdownHandler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    sigaction(15 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(2 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigemptyset(&mut act.sa_mask);
    act
        .sa_flags = (0x40000000 as libc::c_int as libc::c_uint
        | 0x80000000 as libc::c_uint | 4 as libc::c_int as libc::c_uint) as libc::c_int;
    act
        .__sigaction_handler
        .sa_sigaction = Some(
        sigsegvHandler
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    if server.crashlog_enabled != 0 {
        sigaction(11 as libc::c_int, &mut act, 0 as *mut sigaction);
        sigaction(7 as libc::c_int, &mut act, 0 as *mut sigaction);
        sigaction(8 as libc::c_int, &mut act, 0 as *mut sigaction);
        sigaction(4 as libc::c_int, &mut act, 0 as *mut sigaction);
        sigaction(6 as libc::c_int, &mut act, 0 as *mut sigaction);
    }
}
#[no_mangle]
pub unsafe extern "C" fn removeSignalHandlers() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_12 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut act.sa_mask);
    act
        .sa_flags = (0x40000000 as libc::c_int as libc::c_uint
        | 0x80000000 as libc::c_uint) as libc::c_int;
    act.__sigaction_handler.sa_handler = None;
    sigaction(11 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(7 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(8 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(4 as libc::c_int, &mut act, 0 as *mut sigaction);
    sigaction(6 as libc::c_int, &mut act, 0 as *mut sigaction);
}
unsafe extern "C" fn sigKillChildHandler(mut sig: libc::c_int) {
    let mut level: libc::c_int = if server.in_fork_child == 4 as libc::c_int {
        1 as libc::c_int
    } else {
        3 as libc::c_int
    };
    serverLogFromHandler(
        level,
        b"Received SIGUSR1 in child, exiting now.\0" as *const u8 as *const libc::c_char,
    );
    exitFromChild(255 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn setupChildSignalHandlers() {
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_12 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut act.sa_mask);
    act.sa_flags = 0 as libc::c_int;
    act
        .__sigaction_handler
        .sa_handler = Some(
        sigKillChildHandler as unsafe extern "C" fn(libc::c_int) -> (),
    );
    sigaction(10 as libc::c_int, &mut act, 0 as *mut sigaction);
}
#[no_mangle]
pub unsafe extern "C" fn closeChildUnusedResourceAfterFork() {
    closeListeningSockets(0 as libc::c_int);
    if server.cluster_enabled != 0
        && server.cluster_config_file_lock_fd != -(1 as libc::c_int)
    {
        close(server.cluster_config_file_lock_fd);
    }
    zfree(server.pidfile as *mut libc::c_void);
    server.pidfile = 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn redisFork(mut purpose: libc::c_int) -> libc::c_int {
    if isMutuallyExclusiveChildType(purpose) != 0 {
        if hasActiveChildProcess() != 0 {
            *__errno_location() = 17 as libc::c_int;
            return -(1 as libc::c_int);
        }
        openChildInfoPipe();
    }
    let mut childpid: libc::c_int = 0;
    let mut start: libc::c_longlong = ustime();
    childpid = fork();
    if childpid == 0 as libc::c_int {
        server.in_fork_child = purpose;
        setupChildSignalHandlers();
        setOOMScoreAdj(2 as libc::c_int);
        updateDictResizePolicy();
        dismissMemoryInChild();
        closeChildUnusedResourceAfterFork();
        if server.child_info_pipe[0 as libc::c_int as usize] != -(1 as libc::c_int) {
            close(server.child_info_pipe[0 as libc::c_int as usize]);
        }
    } else {
        if childpid == -(1 as libc::c_int) {
            let mut fork_errno: libc::c_int = *__errno_location();
            if isMutuallyExclusiveChildType(purpose) != 0 {
                closeChildInfoPipe();
            }
            *__errno_location() = fork_errno;
            return -(1 as libc::c_int);
        }
        server.stat_total_forks += 1;
        server.stat_fork_time = ustime() - start;
        server
            .stat_fork_rate = zmalloc_used_memory() as libc::c_double
            * 1000000 as libc::c_int as libc::c_double
            / server.stat_fork_time as libc::c_double
            / (1024 as libc::c_int * 1024 as libc::c_int * 1024 as libc::c_int)
                as libc::c_double;
        if server.latency_monitor_threshold != 0
            && server.stat_fork_time / 1000 as libc::c_int as libc::c_longlong
                >= server.latency_monitor_threshold
        {
            latencyAddSample(
                b"fork\0" as *const u8 as *const libc::c_char,
                server.stat_fork_time / 1000 as libc::c_int as libc::c_longlong,
            );
        }
        if isMutuallyExclusiveChildType(purpose) != 0 {
            server.child_pid = childpid;
            server.child_type = purpose;
            server.stat_current_cow_peak = 0 as libc::c_int as size_t;
            server.stat_current_cow_bytes = 0 as libc::c_int as size_t;
            server.stat_current_cow_updated = 0 as libc::c_int as monotime;
            server.stat_current_save_keys_processed = 0 as libc::c_int as size_t;
            server.stat_module_progress = 0 as libc::c_int as libc::c_double;
            server.stat_current_save_keys_total = dbTotalServerKeyCount() as size_t;
        }
        updateDictResizePolicy();
        moduleFireServerEvent(
            13 as libc::c_int as uint64_t,
            0 as libc::c_int,
            0 as *mut libc::c_void,
        );
    }
    return childpid;
}
#[no_mangle]
pub unsafe extern "C" fn sendChildCowInfo(
    mut info_type: childInfoType,
    mut pname: *mut libc::c_char,
) {
    sendChildInfoGeneric(
        info_type,
        0 as libc::c_int as size_t,
        -(1 as libc::c_int) as libc::c_double,
        pname,
    );
}
#[no_mangle]
pub unsafe extern "C" fn sendChildInfo(
    mut info_type: childInfoType,
    mut keys: size_t,
    mut pname: *mut libc::c_char,
) {
    sendChildInfoGeneric(info_type, keys, -(1 as libc::c_int) as libc::c_double, pname);
}
#[no_mangle]
pub unsafe extern "C" fn dismissMemory(
    mut ptr: *mut libc::c_void,
    mut size_hint: size_t,
) {
    if ptr.is_null() {
        return;
    }
    if size_hint != 0
        && size_hint
            <= (server.page_size).wrapping_div(2 as libc::c_int as libc::c_ulong)
    {
        return;
    }
    zmadvise_dontneed(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn dismissClientMemory(mut c: *mut client) {
    dismissMemory((*c).buf as *mut libc::c_void, (*c).buf_usable_size);
    dismissSds((*c).querybuf);
    if (*c).argc != 0
        && ((*c).argv_len_sum).wrapping_div((*c).argc as libc::c_ulong)
            >= server.page_size
    {
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < (*c).argc {
            dismissObject(*((*c).argv).offset(i as isize), 0 as libc::c_int as size_t);
            i += 1;
        }
    }
    if (*c).argc != 0 {
        dismissMemory(
            (*c).argv as *mut libc::c_void,
            ((*c).argc as libc::c_ulong)
                .wrapping_mul(core::mem::size_of::<*mut robj>() as libc::c_ulong),
        );
    }
    if (*(*c).reply).len != 0
        && ((*c).reply_bytes).wrapping_div((*(*c).reply).len as libc::c_ulonglong)
            >= server.page_size as libc::c_ulonglong
    {
        let mut li: listIter = listIter {
            next: 0 as *mut listNode,
            direction: 0,
        };
        let mut ln: *mut listNode = 0 as *mut listNode;
        listRewind((*c).reply, &mut li);
        loop {
            ln = listNext(&mut li);
            if ln.is_null() {
                break;
            }
            let mut bulk: *mut clientReplyBlock = (*ln).value as *mut clientReplyBlock;
            if !bulk.is_null() {
                dismissMemory(bulk as *mut libc::c_void, (*bulk).size);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn dismissMemoryInChild() {
    if server.thp_enabled != 0 {
        return;
    }
    let mut li: listIter = listIter {
        next: 0 as *mut listNode,
        direction: 0,
    };
    let mut ln: *mut listNode = 0 as *mut listNode;
    listRewind(server.repl_buffer_blocks, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut o: *mut replBufBlock = (*ln).value as *mut replBufBlock;
        dismissMemory(o as *mut libc::c_void, (*o).size);
    }
    listRewind(server.clients, &mut li);
    loop {
        ln = listNext(&mut li);
        if ln.is_null() {
            break;
        }
        let mut c: *mut client = (*ln).value as *mut client;
        dismissClientMemory(c);
    };
}
#[no_mangle]
pub unsafe extern "C" fn checkForSentinelMode(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut exec_name: *mut libc::c_char,
) -> libc::c_int {
    if !(strstr(exec_name, b"redis-sentinel\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        return 1 as libc::c_int;
    }
    let mut j: libc::c_int = 1 as libc::c_int;
    while j < argc {
        if strcmp(
            *argv.offset(j as isize),
            b"--sentinel\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            return 1 as libc::c_int;
        }
        j += 1;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn loadDataFromDisk() {
    let mut start: libc::c_longlong = ustime();
    if server.aof_state == 1 as libc::c_int {
        let mut ret: libc::c_int = loadAppendOnlyFiles(server.aof_manifest);
        if ret == 4 as libc::c_int || ret == 3 as libc::c_int {
            exit(1 as libc::c_int);
        }
        if ret != 1 as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"DB loaded from append only file: %.3f seconds\0" as *const u8
                        as *const libc::c_char,
                    ((ustime() - start) as libc::c_float
                        / 1000000 as libc::c_int as libc::c_float) as libc::c_double,
                );
            }
        }
    } else {
        let mut rsi: rdbSaveInfo = {
            let mut init = rdbSaveInfo {
                repl_stream_db: -(1 as libc::c_int),
                repl_id_is_set: 0 as libc::c_int,
                repl_id: unsafe { std::mem::transmute(*b"0000000000000000000000000000000000000000\0") },
                repl_offset: -(1 as libc::c_int) as libc::c_longlong,
            };
            
            unsafe {
                let ptr = init.repl_id.as_mut_ptr() as *mut u8;
                let src = b"0000000000000000000000000000000000000000\0";
                std::ptr::copy_nonoverlapping(src.as_ptr(), ptr, src.len());
            }
            
            init
        };
        *__errno_location() = 0 as libc::c_int;
        let mut rdb_flags: libc::c_int = 0 as libc::c_int;
        if iAmMaster() != 0 {
            createReplicationBacklog();
            rdb_flags |= (1 as libc::c_int) << 3 as libc::c_int;
        }
        if rdbLoad(server.rdb_filename, &mut rsi, rdb_flags) == 0 as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"DB loaded from disk: %.3f seconds\0" as *const u8
                        as *const libc::c_char,
                    ((ustime() - start) as libc::c_float
                        / 1000000 as libc::c_int as libc::c_float) as libc::c_double,
                );
            }
            if rsi.repl_id_is_set != 0
                && rsi.repl_offset != -(1 as libc::c_int) as libc::c_longlong
                && rsi.repl_stream_db != -(1 as libc::c_int)
            {
                if iAmMaster() == 0 {
                    memcpy(
                        (server.replid).as_mut_ptr() as *mut libc::c_void,
                        (rsi.repl_id).as_mut_ptr() as *const libc::c_void,
                        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                    );
                    server.master_repl_offset = rsi.repl_offset;
                    replicationCacheMasterUsingMyself();
                    selectDb(server.cached_master, rsi.repl_stream_db);
                } else {
                    memcpy(
                        (server.replid2).as_mut_ptr() as *mut libc::c_void,
                        (rsi.repl_id).as_mut_ptr() as *const libc::c_void,
                        core::mem::size_of::<[libc::c_char; 41]>() as libc::c_ulong,
                    );
                    server
                        .second_replid_offset = rsi.repl_offset
                        + 1 as libc::c_int as libc::c_longlong;
                    server.master_repl_offset += rsi.repl_offset;
                    if !(server.repl_backlog).is_null() {} else {
                        _serverAssert(
                            b"server.repl_backlog\0" as *const u8 as *const libc::c_char,
                            b"server.c\0" as *const u8 as *const libc::c_char,
                            6618 as libc::c_int,
                        );
                        unreachable!();
                    };
                    (*server.repl_backlog)
                        .offset = server.master_repl_offset
                        - (*server.repl_backlog).histlen
                        + 1 as libc::c_int as libc::c_longlong;
                    rebaseReplicationBuffer(rsi.repl_offset);
                    server.repl_no_slaves_since = time(0 as *mut time_t);
                }
            }
        } else if *__errno_location() != 2 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Fatal error loading the DB: %s. Exiting.\0" as *const u8
                        as *const libc::c_char,
                    strerror(*__errno_location()),
                );
            }
            exit(1 as libc::c_int);
        }
        if server.master_repl_offset == 0 as libc::c_int as libc::c_longlong
            && !(server.repl_backlog).is_null()
        {
            freeReplicationBacklog();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn redisOutOfMemoryHandler(mut allocation_size: size_t) {
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Out Of Memory allocating %zu bytes!\0" as *const u8 as *const libc::c_char,
            allocation_size,
        );
    }
    _serverPanic(
        b"server.c\0" as *const u8 as *const libc::c_char,
        6644 as libc::c_int,
        b"Redis aborting for OUT OF MEMORY. Allocating %zu bytes!\0" as *const u8
            as *const libc::c_char,
        allocation_size,
    );
    unreachable!();
}
unsafe extern "C" fn redisProcTitleGetVariable(
    varname: sds,
    mut arg: *mut libc::c_void,
) -> sds {
    if strcmp(
        varname as *const libc::c_char,
        b"title\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return sdsnew(arg as *const libc::c_char)
    } else if strcmp(
        varname as *const libc::c_char,
        b"listen-addr\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if server.port != 0 || server.tls_port != 0 {
            return sdscatprintf(
                sdsempty(),
                b"%s:%u\0" as *const u8 as *const libc::c_char,
                if server.bindaddr_count != 0 {
                    server.bindaddr[0 as libc::c_int as usize] as *const libc::c_char
                } else {
                    b"*\0" as *const u8 as *const libc::c_char
                },
                if server.port != 0 { server.port } else { server.tls_port },
            )
        } else {
            return sdscatprintf(
                sdsempty(),
                b"unixsocket:%s\0" as *const u8 as *const libc::c_char,
                server.unixsocket,
            )
        }
    } else if strcmp(
        varname as *const libc::c_char,
        b"server-mode\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if server.cluster_enabled != 0 {
            return sdsnew(b"[cluster]\0" as *const u8 as *const libc::c_char)
        } else if server.sentinel_mode != 0 {
            return sdsnew(b"[sentinel]\0" as *const u8 as *const libc::c_char)
        } else {
            return sdsempty()
        }
    } else if strcmp(
        varname as *const libc::c_char,
        b"config-file\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return sdsnew(
            if !(server.configfile).is_null() {
                server.configfile as *const libc::c_char
            } else {
                b"-\0" as *const u8 as *const libc::c_char
            },
        )
    } else if strcmp(
        varname as *const libc::c_char,
        b"port\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return sdscatprintf(
            sdsempty(),
            b"%u\0" as *const u8 as *const libc::c_char,
            server.port,
        )
    } else if strcmp(
        varname as *const libc::c_char,
        b"tls-port\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return sdscatprintf(
            sdsempty(),
            b"%u\0" as *const u8 as *const libc::c_char,
            server.tls_port,
        )
    } else if strcmp(
        varname as *const libc::c_char,
        b"unixsocket\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        return sdsnew(server.unixsocket)
    } else {
        return 0 as sds
    };
}
unsafe extern "C" fn expandProcTitleTemplate(
    mut template: *const libc::c_char,
    mut title: *const libc::c_char,
) -> sds {
    let mut res: sds = sdstemplate(
        template,
        Some(
            redisProcTitleGetVariable
                as unsafe extern "C" fn(sds, *mut libc::c_void) -> sds,
        ),
        title as *mut libc::c_void,
    );
    if res.is_null() {
        return 0 as sds;
    }
    return sdstrim(res, b" \0" as *const u8 as *const libc::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn validateProcTitleTemplate(
    mut template: *const libc::c_char,
) -> libc::c_int {
    let mut ok: libc::c_int = 1 as libc::c_int;
    let mut res: sds = expandProcTitleTemplate(
        template,
        b"\0" as *const u8 as *const libc::c_char,
    );
    if res.is_null() {
        return 0 as libc::c_int;
    }
    if sdslen(res) == 0 as libc::c_int as libc::c_ulong {
        ok = 0 as libc::c_int;
    }
    sdsfree(res);
    return ok;
}
#[no_mangle]
pub unsafe extern "C" fn redisSetProcTitle(mut title: *mut libc::c_char) -> libc::c_int {
    if title.is_null() {
        title = *(server.exec_argv).offset(0 as libc::c_int as isize);
    }
    let mut proc_title: sds = expandProcTitleTemplate(server.proc_title_template, title);
    if proc_title.is_null() {
        return -(1 as libc::c_int);
    }
    setproctitle(b"%s\0" as *const u8 as *const libc::c_char, proc_title);
    sdsfree(proc_title);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisSetCpuAffinity(mut cpulist: *const libc::c_char) {
    setcpuaffinity(cpulist);
}
#[no_mangle]
pub unsafe extern "C" fn redisCommunicateSystemd(
    mut sd_notify_msg: *const libc::c_char,
) -> libc::c_int {
    return 0 as libc::c_int;
}
unsafe extern "C" fn redisSupervisedUpstart() -> libc::c_int {
    let mut upstart_job: *const libc::c_char = getenv(
        b"UPSTART_JOB\0" as *const u8 as *const libc::c_char,
    );
    if upstart_job.is_null() {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"upstart supervision requested, but UPSTART_JOB not found!\0"
                    as *const u8 as *const libc::c_char,
            );
        }
        return 0 as libc::c_int;
    }
    if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            2 as libc::c_int,
            b"supervised by upstart, will stop to signal readiness.\0" as *const u8
                as *const libc::c_char,
        );
    }
    raise(19 as libc::c_int);
    unsetenv(b"UPSTART_JOB\0" as *const u8 as *const libc::c_char);
    return 1 as libc::c_int;
}
unsafe extern "C" fn redisSupervisedSystemd() -> libc::c_int {
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"systemd supervision requested or auto-detected, but Redis is compiled without libsystemd support!\0"
                as *const u8 as *const libc::c_char,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn redisIsSupervised(mut mode: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    if mode == 1 as libc::c_int {
        if !(getenv(b"UPSTART_JOB\0" as *const u8 as *const libc::c_char)).is_null() {
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Upstart supervision detected.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            mode = 3 as libc::c_int;
        } else if !(getenv(b"NOTIFY_SOCKET\0" as *const u8 as *const libc::c_char))
            .is_null()
        {
            if !((1 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    1 as libc::c_int,
                    b"Systemd supervision detected.\0" as *const u8
                        as *const libc::c_char,
                );
            }
            mode = 2 as libc::c_int;
        }
    }
    match mode {
        3 => {
            ret = redisSupervisedUpstart();
        }
        2 => {
            ret = redisSupervisedSystemd();
        }
        _ => {}
    }
    if ret != 0 {
        server.supervised_mode = mode;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn iAmMaster() -> libc::c_int {
    return (server.cluster_enabled == 0 && (server.masterhost).is_null()
        || server.cluster_enabled != 0
            && (*(*server.cluster).myself).flags & 1 as libc::c_int != 0) as libc::c_int;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut j: libc::c_int = 0;
    let mut config_from_stdin: libc::c_char = 0 as libc::c_int as libc::c_char;
    spt_init(argc, argv);
    setlocale(3 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    tzset();
    zmalloc_set_oom_handler(
        Some(redisOutOfMemoryHandler as unsafe extern "C" fn(size_t) -> ()),
    );
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    srand(
        (time(0 as *mut time_t) ^ getpid() as libc::c_long ^ tv.tv_usec) as libc::c_uint,
    );
    srandom(
        (time(0 as *mut time_t) ^ getpid() as libc::c_long ^ tv.tv_usec) as libc::c_uint,
    );
    init_genrand64(
        (tv.tv_sec as libc::c_longlong * 1000000 as libc::c_int as libc::c_longlong
            + tv.tv_usec as libc::c_longlong ^ getpid() as libc::c_longlong)
            as libc::c_ulonglong,
    );
    crc64_init();
    server.umask = umask(0o777 as libc::c_int as __mode_t);
    umask(server.umask);
    let mut hashseed: [uint8_t; 16] = [0; 16];
    getRandomBytes(
        hashseed.as_mut_ptr(),
        core::mem::size_of::<[uint8_t; 16]>() as libc::c_ulong,
    );
    dictSetHashFunctionSeed(hashseed.as_mut_ptr());
    let mut exec_name: *mut libc::c_char = strrchr(
        *argv.offset(0 as libc::c_int as isize),
        '/' as i32,
    );
    if exec_name.is_null() {
        exec_name = *argv.offset(0 as libc::c_int as isize);
    }
    server.sentinel_mode = checkForSentinelMode(argc, argv, exec_name);
    initServerConfig();
    ACLInit();
    moduleInitModulesSystem();
    tlsInit();
    server.executable = getAbsolutePath(*argv.offset(0 as libc::c_int as isize));
    server
        .exec_argv = zmalloc(
        (core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((argc + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    let ref mut fresh31 = *(server.exec_argv).offset(argc as isize);
    *fresh31 = 0 as *mut libc::c_char;
    j = 0 as libc::c_int;
    while j < argc {
        let ref mut fresh32 = *(server.exec_argv).offset(j as isize);
        *fresh32 = zstrdup(*argv.offset(j as isize));
        j += 1;
    }
    if server.sentinel_mode != 0 {
        initSentinelConfig();
        initSentinel();
    }
    if !(strstr(exec_name, b"redis-check-rdb\0" as *const u8 as *const libc::c_char))
        .is_null()
    {
        redis_check_rdb_main(argc, argv, 0 as *mut FILE);
    } else if !(strstr(
        exec_name,
        b"redis-check-aof\0" as *const u8 as *const libc::c_char,
    ))
        .is_null()
    {
        redis_check_aof_main(argc, argv);
    }
    if argc >= 2 as libc::c_int {
        j = 1 as libc::c_int;
        let mut options: sds = sdsempty();
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-v\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"--version\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            version();
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--help\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
            || strcmp(
                *argv.offset(1 as libc::c_int as isize),
                b"-h\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int
        {
            usage();
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--test-memory\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            if argc == 3 as libc::c_int {
                memtest(
                    atoi(*argv.offset(2 as libc::c_int as isize)) as size_t,
                    50 as libc::c_int,
                );
                exit(0 as libc::c_int);
            } else {
                fprintf(
                    stderr,
                    b"Please specify the amount of memory to test in megabytes.\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"Example: ./redis-server --test-memory 4096\n\n\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
        }
        if strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"--check-system\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            exit(if syscheck() != 0 { 0 as libc::c_int } else { 1 as libc::c_int });
        }
        if *(*argv.offset(1 as libc::c_int as isize)).offset(0 as libc::c_int as isize)
            as libc::c_int != '-' as i32
        {
            server.configfile = getAbsolutePath(*argv.offset(1 as libc::c_int as isize));
            zfree(
                *(server.exec_argv).offset(1 as libc::c_int as isize)
                    as *mut libc::c_void,
            );
            let ref mut fresh33 = *(server.exec_argv).offset(1 as libc::c_int as isize);
            *fresh33 = zstrdup(server.configfile);
            j = 2 as libc::c_int;
        }
        let mut argv_tmp: *mut sds = 0 as *mut sds;
        let mut argc_tmp: libc::c_int = 0;
        let mut handled_last_config_arg: libc::c_int = 1 as libc::c_int;
        while j < argc {
            if *(*argv.offset(j as isize)).offset(0 as libc::c_int as isize)
                as libc::c_int == '-' as i32
                && *(*argv.offset(j as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '\0' as i32
                && (j == 1 as libc::c_int || j == argc - 1 as libc::c_int)
            {
                config_from_stdin = 1 as libc::c_int as libc::c_char;
            } else if handled_last_config_arg != 0
                && *(*argv.offset(j as isize)).offset(0 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
                && *(*argv.offset(j as isize)).offset(1 as libc::c_int as isize)
                    as libc::c_int == '-' as i32
            {
                if sdslen(options) != 0 {
                    options = sdscat(
                        options,
                        b"\n\0" as *const u8 as *const libc::c_char,
                    );
                }
                options = sdscat(
                    options,
                    (*argv.offset(j as isize)).offset(2 as libc::c_int as isize),
                );
                options = sdscat(options, b" \0" as *const u8 as *const libc::c_char);
                argv_tmp = sdssplitargs(*argv.offset(j as isize), &mut argc_tmp);
                if argc_tmp == 1 as libc::c_int {
                    handled_last_config_arg = 0 as libc::c_int;
                    if j != argc - 1 as libc::c_int
                        && *(*argv.offset((j + 1 as libc::c_int) as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        && *(*argv.offset((j + 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        && strcasecmp(
                            *argv.offset(j as isize),
                            b"--save\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        options = sdscat(
                            options,
                            b"\"\"\0" as *const u8 as *const libc::c_char,
                        );
                        handled_last_config_arg = 1 as libc::c_int;
                    } else if j == argc - 1 as libc::c_int
                        && strcasecmp(
                            *argv.offset(j as isize),
                            b"--save\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        options = sdscat(
                            options,
                            b"\"\"\0" as *const u8 as *const libc::c_char,
                        );
                    } else if j != argc - 1 as libc::c_int
                        && *(*argv.offset((j + 1 as libc::c_int) as isize))
                            .offset(0 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        && *(*argv.offset((j + 1 as libc::c_int) as isize))
                            .offset(1 as libc::c_int as isize) as libc::c_int
                            == '-' as i32
                        && strcasecmp(
                            *argv.offset(j as isize),
                            b"--sentinel\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        options = sdscat(
                            options,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                        handled_last_config_arg = 1 as libc::c_int;
                    } else if j == argc - 1 as libc::c_int
                        && strcasecmp(
                            *argv.offset(j as isize),
                            b"--sentinel\0" as *const u8 as *const libc::c_char,
                        ) == 0
                    {
                        options = sdscat(
                            options,
                            b"\0" as *const u8 as *const libc::c_char,
                        );
                    }
                } else {
                    handled_last_config_arg = 1 as libc::c_int;
                }
                sdsfreesplitres(argv_tmp, argc_tmp);
            } else {
                options = sdscatrepr(
                    options,
                    *argv.offset(j as isize),
                    strlen(*argv.offset(j as isize)),
                );
                options = sdscat(options, b" \0" as *const u8 as *const libc::c_char);
                handled_last_config_arg = 1 as libc::c_int;
            }
            j += 1;
        }
        loadServerConfig(server.configfile, config_from_stdin, options);
        if server.sentinel_mode != 0 {
            loadSentinelConfigFromQueue();
        }
        sdsfree(options);
    }
    if server.sentinel_mode != 0 {
        sentinelCheckConfigFile();
    }
    server.supervised = redisIsSupervised(server.supervised_mode);
    let mut background: libc::c_int = (server.daemonize != 0 && server.supervised == 0)
        as libc::c_int;
    if background != 0 {
        daemonize();
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"oO0OoO0OoO0Oo Redis is starting oO0OoO0OoO0Oo\0" as *const u8
                as *const libc::c_char,
        );
    }
    if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Redis version=%s, bits=%d, commit=%s, modified=%d, pid=%d, just started\0"
                as *const u8 as *const libc::c_char,
            b"7.0.8\0" as *const u8 as *const libc::c_char,
            if core::mem::size_of::<libc::c_long>() as libc::c_ulong
                == 8 as libc::c_int as libc::c_ulong
            {
                64 as libc::c_int
            } else {
                32 as libc::c_int
            },
            redisGitSHA1(),
            (strtol(redisGitDirty(), 0 as *mut *mut libc::c_char, 10 as libc::c_int)
                > 0 as libc::c_int as libc::c_long) as libc::c_int,
            getpid(),
        );
    }
    if argc == 1 as libc::c_int {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Warning: no config file specified, using the default config. In order to specify a config file use %s /path/to/redis.conf\0"
                    as *const u8 as *const libc::c_char,
                *argv.offset(0 as libc::c_int as isize),
            );
        }
    } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
        _serverLog(
            3 as libc::c_int,
            b"Configuration loaded\0" as *const u8 as *const libc::c_char,
        );
    }
    initServer();
    if background != 0 || !(server.pidfile).is_null() {
        createPidFile();
    }
    if server.set_proc_title != 0 {
        redisSetProcTitle(0 as *mut libc::c_char);
    }
    redisAsciiArt();
    checkTcpBacklogSettings();
    if server.sentinel_mode == 0 {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"Server initialized\0" as *const u8 as *const libc::c_char,
            );
        }
        linuxMemoryWarnings();
        let mut err_msg: sds = 0 as sds;
        if checkXenClocksource(&mut err_msg) < 0 as libc::c_int {
            if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"WARNING %s\0" as *const u8 as *const libc::c_char,
                    err_msg,
                );
            }
            sdsfree(err_msg);
        }
        let mut ret: libc::c_int = 0;
        ret = checkLinuxMadvFreeForkBug(&mut err_msg);
        if ret <= 0 as libc::c_int {
            if ret < 0 as libc::c_int {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"WARNING %s\0" as *const u8 as *const libc::c_char,
                        err_msg,
                    );
                }
                sdsfree(err_msg);
            } else if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    3 as libc::c_int,
                    b"Failed to test the kernel for a bug that could lead to data corruption during background save. Your system could be affected, please report this error.\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            if checkIgnoreWarning(b"ARM64-COW-BUG\0" as *const u8 as *const libc::c_char)
                == 0
            {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"Redis will now exit to prevent data corruption. Note that it is possible to suppress this warning by setting the following config: ignore-warnings ARM64-COW-BUG\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
        }
        moduleInitModulesSystemLast();
        moduleLoadFromQueue();
        ACLLoadUsersAtStartup();
        InitServerLast();
        aofLoadManifestFromDisk();
        loadDataFromDisk();
        aofOpenIfNeededOnServerStart();
        aofDelHistoryFiles();
        if server.cluster_enabled != 0 {
            if verifyClusterConfigWithData() == -(1 as libc::c_int) {
                if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                    _serverLog(
                        3 as libc::c_int,
                        b"You can't have keys in a DB different than DB 0 when in Cluster mode. Exiting.\0"
                            as *const u8 as *const libc::c_char,
                    );
                }
                exit(1 as libc::c_int);
            }
        }
        if server.ipfd.count > 0 as libc::c_int || server.tlsfd.count > 0 as libc::c_int
        {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"Ready to accept connections\0" as *const u8 as *const libc::c_char,
                );
            }
        }
        if server.sofd > 0 as libc::c_int {
            if !((2 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
                _serverLog(
                    2 as libc::c_int,
                    b"The server is now ready to accept connections at %s\0" as *const u8
                        as *const libc::c_char,
                    server.unixsocket,
                );
            }
        }
        if server.supervised_mode == 2 as libc::c_int {
            if (server.masterhost).is_null() {
                redisCommunicateSystemd(
                    b"STATUS=Ready to accept connections\n\0" as *const u8
                        as *const libc::c_char,
                );
            } else {
                redisCommunicateSystemd(
                    b"STATUS=Ready to accept connections in read-only mode. Waiting for MASTER <-> REPLICA sync\n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            redisCommunicateSystemd(b"READY=1\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        ACLLoadUsersAtStartup();
        InitServerLast();
        sentinelIsRunning();
        if server.supervised_mode == 2 as libc::c_int {
            redisCommunicateSystemd(
                b"STATUS=Ready to accept connections\n\0" as *const u8
                    as *const libc::c_char,
            );
            redisCommunicateSystemd(b"READY=1\n\0" as *const u8 as *const libc::c_char);
        }
    }
    if server.maxmemory > 0 as libc::c_int as libc::c_ulonglong
        && server.maxmemory
            < (1024 as libc::c_int * 1024 as libc::c_int) as libc::c_ulonglong
    {
        if !((3 as libc::c_int & 0xff as libc::c_int) < server.verbosity) {
            _serverLog(
                3 as libc::c_int,
                b"WARNING: You specified a maxmemory value that is less than 1MB (current value is %llu bytes). Are you sure this is what you really want?\0"
                    as *const u8 as *const libc::c_char,
                server.maxmemory,
            );
        }
    }
    redisSetCpuAffinity(server.server_cpulist);
    setOOMScoreAdj(-(1 as libc::c_int));
    aeMain(server.el);
    aeDeleteEventLoop(server.el);
    return 0 as libc::c_int;
}
pub fn run_server() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}

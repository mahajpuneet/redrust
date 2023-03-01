#![feature(extern_types)]
#![feature(c_variadic)]
#![feature(core_intrinsics)]
#![feature(label_break_value)]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(const_cell_into_inner)]
#![feature(const_refs_to_cell)]

mod acl;
mod crc64;
mod adlist;
mod ae;
mod anet;
mod aof;
mod bio;
mod bitops;
mod blocked;
mod call_reply;
mod childinfo;
mod cli_common;
mod cluster;
mod commands;
mod config;
mod connection;
mod crc16;
mod crcspeed;
mod db;
mod debug;
mod defrag;
mod dict;
mod endianconv;
mod eval;
mod evict;
mod expire;
mod function_lua;
mod functions;
mod geo;
mod geohash;
mod geohash_helper;
mod hyperloglog;
mod intset;
mod latency;
mod lazyfree;
mod listpack;
mod localtime;
mod lolwut5;
mod lolwut6;
mod lolwut;
mod lzf_c;
mod lzf_d;
mod memtest;
mod module;
mod monotonic;
mod mt19937_64;
mod multi;
mod networking;
mod notify;
mod object;
mod pqsort;
mod pubsub;
mod quicklist;
mod rand;
mod rax;
mod rdb;
mod redis_benchmark;
mod redis_check_aof;
mod redis_check_rdb;
mod redis_cli;
mod redisassert;
mod release;
mod replication;
mod resp_parser;
mod rio;
mod script;
mod script_lua;
mod sds;
mod sentinel;
mod server;
mod setcpuaffinity;
mod setproctitle;
mod sha1;
mod sha256;
mod siphash;
mod slowlog;
mod sort;
mod sparkline;
mod syncio;
mod syscheck;
mod t_hash;
mod t_list;
mod t_set;
mod t_stream;
mod t_string;
mod t_zset;
mod timeout;
mod tls;
mod tracking;
mod util;
mod ziplist;
mod zipmap;
mod zmalloc;

fn main() {
    server::run_server(); // Call the function
}

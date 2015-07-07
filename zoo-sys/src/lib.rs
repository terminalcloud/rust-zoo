//! Bindings to the zookeeper C client library.
#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case)]
extern crate libc;

use libc::*;
use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct buffer {
    pub len: int32_t,
    pub buff: *mut c_char
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iarchive {
    pub start_record: Option<extern fn(*mut iarchive, *const c_char) -> c_int>,
    pub end_record: Option<extern fn(*mut iarchive, *const c_char, *mut int32_t) -> c_int>,
    pub start_vector: Option<extern fn(*mut iarchive, *const c_char, *mut int32_t) -> c_int>,
    pub end_vector: Option<extern fn(*mut iarchive, *const c_char) -> c_int>,
    pub deserialize_Bool: Option<extern fn(*mut iarchive,  *const c_char,  *mut int32_t) -> c_int>,
    pub deserialize_Int: Option<extern fn(*mut iarchive, *const c_char, *mut int32_t) -> c_int>,
    pub deserialize_Long: Option<extern fn(*mut iarchive, *const c_char, *mut int64_t) -> c_int>,
    pub deserialize_Buffer: Option<extern fn(*mut iarchive, *const c_char, *mut buffer) -> c_int>,
    pub deserialize_String: Option<extern fn(*mut iarchive, *const c_char, *mut *mut c_char) -> c_int>,
    pub _priv: *mut c_void
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oarchive {
    pub start_record: Option<extern fn(*mut oarchive, *const c_char) -> c_int>,
    pub end_record: Option<extern fn(*mut oarchive, *const c_char) -> c_int>,
    pub start_vector: Option<extern fn(*mut oarchive, *const c_char, *const int32_t) -> c_int>,
    pub end_vector: Option<extern fn(*mut oarchive, *const c_char) -> c_int>,
    pub serialize_Bool: Option<extern fn(*mut oarchive, *const c_char, *const int32_t) -> c_int>,
    pub serialize_Int: Option<extern fn(*mut oarchive, *const c_char, *const int32_t) -> c_int>,
    pub serialize_Long: Option<extern fn(*mut oarchive, *const c_char, *const int64_t) -> c_int>,
    pub serialize_Buffer: Option<extern fn(*mut oarchive, *const c_char, *const buffer) -> c_int>,
    pub serialize_String: Option<extern fn(*mut oarchive, *const c_char, *mut *mut c_char) -> c_int>,
    pub _priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct String_vector {
    pub count: int32_t,
    pub data: *mut *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Id {
     pub scheme: *mut c_char,
     pub id: *mut c_char
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACL {
     pub perms: int32_t,
     pub id: Id
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct Stat {
    pub czxid: int64_t,
    pub mzxid: int64_t,
    pub ctime: int64_t,
    pub mtime: int64_t,
    pub version: int32_t,
    pub cversion: int32_t,
    pub aversion: int32_t,
    pub ephemeralOwner: int64_t,
    pub dataLength: int32_t,
    pub numChildren: int32_t,
    pub pzxid: int64_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StatPersisted {
    pub czxid: int64_t,
    pub mzxid: int64_t,
    pub ctime: int64_t,
    pub mtime: int64_t,
    pub version: int32_t,
    pub cversion: int32_t,
    pub aversion: int32_t,
    pub ephemeralOwner: int64_t,
    pub pzxid: int64_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct StatPersistedV1 {
    pub czxid: int64_t,
    pub mzxid: int64_t,
    pub ctime: int64_t,
    pub mtime: int64_t,
    pub version: int32_t,
    pub cversion: int32_t,
    pub aversion: int32_t,
    pub ephemeralOwner: int64_t
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConnectRequest {
    pub protocolVersion: int32_t,
    pub lastZxidSeen: int64_t,
    pub timeOut: int32_t,
    pub sessionId: int64_t,
    pub passwd: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ConnectResponse {
    pub protocolVersion: int32_t,
    pub timeOut: int32_t,
    pub sessionId: int64_t,
    pub passwd: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetWatches {
    pub relativeZxid: int64_t,
    pub dataWatches: String_vector,
    pub existWatches: String_vector,
    pub childWatches: String_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct RequestHeader {
    pub xid: int32_t,
    pub _type: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultiHeader {
    pub _type: int32_t,
    pub done: int32_t,
    pub err: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AuthPacket {
    pub _type: int32_t,
    pub scheme: *mut c_char,
    pub auth: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ReplyHeader {
    pub xid: int32_t,
    pub zxid: int64_t,
    pub err: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetDataRequest {
    pub path: *mut c_char,
    pub watch: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetDataRequest {
    pub path: *mut c_char,
    pub data: buffer,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetDataResponse {
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetSASLRequest {
    pub token: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetSASLRequest {
    pub token: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetSASLResponse {
    pub token: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ACL_vector {
    pub count: int32_t,
    pub data: *mut ACL,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateRequest {
    pub path: *mut c_char,
    pub data: buffer,
    pub acl: ACL_vector,
    pub flags: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeleteRequest {
    pub path: *mut c_char,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetChildrenRequest {
    pub path: *mut c_char,
    pub watch: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetChildren2Request {
    pub path: *mut c_char,
    pub watch: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CheckVersionRequest {
    pub path: *mut c_char,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetMaxChildrenRequest {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetMaxChildrenResponse {
    pub max: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetMaxChildrenRequest {
    pub path: *mut c_char,
    pub max: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyncRequest {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyncResponse {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetACLRequest {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetACLRequest {
    pub path: *mut c_char,
    pub acl: ACL_vector,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetACLResponse {
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct WatcherEvent {
    pub _type: int32_t,
    pub state: int32_t,
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorResponse {
    pub err: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateResponse {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExistsRequest {
    pub path: *mut c_char,
    pub watch: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ExistsResponse {
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetDataResponse {
    pub data: buffer,
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetChildrenResponse {
    pub children: String_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetChildren2Response {
    pub children: String_vector,
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct GetACLResponse {
    pub acl: ACL_vector,
    pub stat: Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LearnerInfo {
    pub serverid: int64_t,
    pub protocolVersion: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Id_vector {
    pub count: int32_t,
    pub data: *mut Id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct QuorumPacket {
    pub _type: int32_t,
    pub zxid: int64_t,
    pub data: buffer,
    pub authinfo: Id_vector,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FileHeader {
    pub magic: int32_t,
    pub version: int32_t,
    pub dbid: int64_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TxnHeader {
    pub clientId: int64_t,
    pub cxid: int32_t,
    pub zxid: int64_t,
    pub time: int64_t,
    pub _type: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateTxnV0 {
    pub path: *mut c_char,
    pub data: buffer,
    pub acl: ACL_vector,
    pub ephemeral: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateTxn {
    pub path: *mut c_char,
    pub data: buffer,
    pub acl: ACL_vector,
    pub ephemeral: int32_t,
    pub parentCVersion: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DeleteTxn {
    pub path: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetDataTxn {
    pub path: *mut c_char,
    pub data: buffer,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CheckVersionTxn {
    pub path: *mut c_char,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetACLTxn {
    pub path: *mut c_char,
    pub acl: ACL_vector,
    pub version: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SetMaxChildrenTxn {
    pub path: *mut c_char,
    pub max: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CreateSessionTxn {
    pub timeOut: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ErrorTxn {
    pub err: int32_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Txn {
    pub _type: int32_t,
    pub data: buffer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Txn_vector {
    pub count: int32_t,
    pub data: *mut Txn,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MultiTxn {
    pub txns: Txn_vector,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ZooError {
    ZOK = 0,
    ZSYSTEMERROR = -1,
    ZRUNTIMEINCONSISTENCY = -2,
    ZDATAINCONSISTENCY = -3,
    ZCONNECTIONLOSS = -4,
    ZMARSHALLINGERROR = -5,
    ZUNIMPLEMENTED = -6,
    ZOPERATIONTIMEOUT = -7,
    ZBADARGUMENTS = -8,
    ZINVALIDSTATE = -9,
    ZAPIERROR = -100,
    ZNONODE = -101,
    ZNOAUTH = -102,
    ZBADVERSION = -103,
    ZNOCHILDRENFOREPHEMERALS = -108,
    ZNODEEXISTS = -110,
    ZNOTEMPTY = -111,
    ZSESSIONEXPIRED = -112,
    ZINVALIDCALLBACK = -113,
    ZINVALIDACL = -114,
    ZAUTHFAILED = -115,
    ZCLOSING = -116,
    ZNOTHING = -117,
    ZSESSIONMOVED = -118
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum ZooLogLevel {
    ZOO_LOG_LEVEL_ERROR = 1,
    ZOO_LOG_LEVEL_WARN = 2,
    ZOO_LOG_LEVEL_INFO = 3,
    ZOO_LOG_LEVEL_DEBUG = 4
}

pub enum zhandle_t {  }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct client_id {
    pub client_id: int64_t,
    pub passwd: [c_char; 16usize],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op {
    pub _type: c_int,
    // This represents a union. Ugh.
    pub _union: [u64; 7],
}

impl zoo_op {
    pub unsafe fn create_op(&mut self) -> *mut zoo_op_create {
        mem::transmute(self.raw_union())
    }

    pub unsafe fn delete_op(&mut self) -> *mut zoo_op_delete {
        mem::transmute(self.raw_union())
    }

    pub unsafe fn set_op(&mut self) -> *mut zoo_op_set {
        mem::transmute(self.raw_union())
    }

    pub unsafe fn check_op(&mut self) -> *mut zoo_op_check {
        mem::transmute(self.raw_union())
    }

    unsafe fn raw_union(&mut self) -> *mut u8 {
        mem::transmute(&mut self._union)
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op_create {
    pub path: *const c_char,
    pub data: *const c_char,
    pub datalen: c_int,
    pub buf: *mut c_char,
    pub buflen: c_int,
    pub acl: *const ACL_vector,
    pub flags: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op_delete {
    pub path: *const c_char,
    pub version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op_set {
    pub path: *const c_char,
    pub data: *const c_char,
    pub datalen: c_int,
    pub version: c_int,
    pub stat: *mut Stat,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op_check {
    pub path: *const c_char,
    pub version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct zoo_op_result {
    pub err: c_int,
    pub value: *mut c_char,
    pub valuelen: c_int,
    pub stat: *mut Stat,
}

pub type watcher_fn = Option<extern fn(zh: *mut zhandle_t, _type: c_int, state: c_int,
                                           path: *const c_char, watcherCtx: *mut c_void)>;

pub type void_completion_t = Option<extern fn(rc: c_int, data: *const c_void)>;

pub type stat_completion_t = Option<extern fn(rc: c_int, stat: *const Stat,
                                                  data: *const c_void)>;
pub type data_completion_t = Option<extern fn(rc: c_int, value: *const c_char, value_len: c_int,
                                                  stat: *const Stat, data: *const c_void)>;

pub type strings_completion_t = Option<extern fn(rc: c_int, strings: *const String_vector,
                                                     data: *const c_void)>;

pub type strings_stat_completion_t = Option<extern fn(rc: c_int, strings: *const String_vector,
                                                          stat: *const Stat, data: *const c_void)>;

pub type string_completion_t = Option<extern fn(rc: c_int, value: *const c_char, data: *const c_void)>;

pub type acl_completion_t = Option<extern fn(rc: c_int, acl: *mut ACL_vector, stat: *mut Stat,
                                                 data: *const c_void)>;

#[link(name = "zookeeper_mt")]
extern {
    pub static ZOO_PERM_READ: c_int;
    pub static ZOO_PERM_WRITE: c_int;
    pub static ZOO_PERM_CREATE: c_int;
    pub static ZOO_PERM_DELETE: c_int;
    pub static ZOO_PERM_ADMIN: c_int;
    pub static ZOO_PERM_ALL: c_int;
    pub static mut ZOO_ANYONE_ID_UNSAFE: Id;
    pub static mut ZOO_AUTH_IDS: Id;
    pub static mut ZOO_OPEN_ACL_UNSAFE: ACL_vector;
    pub static mut ZOO_READ_ACL_UNSAFE: ACL_vector;
    pub static mut ZOO_CREATOR_ALL_ACL: ACL_vector;
    pub static ZOOKEEPER_WRITE: c_int;
    pub static ZOOKEEPER_READ: c_int;
    pub static ZOO_EPHEMERAL: c_int;
    pub static ZOO_SEQUENCE: c_int;
    pub static ZOO_EXPIRED_SESSION_STATE: c_int;
    pub static ZOO_AUTH_FAILED_STATE: c_int;
    pub static ZOO_CONNECTING_STATE: c_int;
    pub static ZOO_ASSOCIATING_STATE: c_int;
    pub static ZOO_CONNECTED_STATE: c_int;
    pub static ZOO_CREATED_EVENT: c_int;
    pub static ZOO_DELETED_EVENT: c_int;
    pub static ZOO_CHANGED_EVENT: c_int;
    pub static ZOO_CHILD_EVENT: c_int;
    pub static ZOO_SESSION_EVENT: c_int;
    pub static ZOO_NOTWATCHING_EVENT: c_int;
}

#[link(name = "zookeeper_mt")]
extern {
    pub fn zoo_htonll(v: int64_t) -> int64_t;
    pub fn zoo_create_op_init(op: *mut zoo_op, path: *const c_char,
                              value: *const c_char, valuelen: c_int,
                              acl: *const ACL_vector, flags: c_int,
                              path_buffer: *mut c_char, path_buffer_len: c_int);
    pub fn zoo_delete_op_init(op: *mut zoo_op, path: *const c_char, version: c_int);
    pub fn zoo_set_op_init(op: *mut zoo_op, path: *const c_char,
                           buffer: *const c_char, buflen: c_int,
                           version: c_int, stat: *mut Stat);
    pub fn zoo_check_op_init(op: *mut zoo_op, path: *const c_char, version: c_int);
    pub fn zookeeper_init(host: *const c_char, _fn: watcher_fn,
                          recv_timeout: c_int, clientid: *const client_id,
                          context: *mut c_void, flags: c_int) -> *mut zhandle_t;
    pub fn zookeeper_close(zh: *mut zhandle_t) -> c_int;
    pub fn zoo_client_id(zh: *mut zhandle_t) -> *const client_id;
    pub fn zoo_recv_timeout(zh: *mut zhandle_t) -> c_int;
    pub fn zoo_get_context(zh: *mut zhandle_t) -> *const c_void;
    pub fn zoo_set_context(zh: *mut zhandle_t, context: *mut c_void);
    pub fn zoo_set_watcher(zh: *mut zhandle_t, newFn: watcher_fn) -> watcher_fn;
    pub fn zookeeper_get_connected_host(zh: *mut zhandle_t, addr: *mut sockaddr,
                                        addr_len: *mut socklen_t) -> *mut sockaddr;
    pub fn zookeeper_interest(zh: *mut zhandle_t, fd: *mut c_int,
                              interest: *mut c_int, tv: *mut timeval) -> c_int;
    pub fn zookeeper_process(zh: *mut zhandle_t, events: c_int) -> c_int;
    pub fn zoo_state(zh: *mut zhandle_t) -> c_int;
    pub fn zoo_acreate(zh: *mut zhandle_t, path: *const c_char,
                       value: *const c_char, valuelen: c_int,
                       acl: *const ACL_vector, flags: c_int,
                       completion: string_completion_t,
                       data: *const c_void) -> c_int;
    pub fn zoo_adelete(zh: *mut zhandle_t, path: *const c_char,
                       version: c_int, completion: void_completion_t,
                       data: *const c_void) -> c_int;
    pub fn zoo_aexists(zh: *mut zhandle_t, path: *const c_char,
                       watch: c_int, completion: stat_completion_t,
                       data: *const c_void) -> c_int;
    pub fn zoo_awexists(zh: *mut zhandle_t, path: *const c_char,
                        watcher: watcher_fn, watcherCtx: *mut c_void,
                        completion: stat_completion_t,
                        data: *const c_void) -> c_int;
    pub fn zoo_aget(zh: *mut zhandle_t, path: *const c_char,
                    watch: c_int, completion: data_completion_t,
                    data: *const c_void) -> c_int;
    pub fn zoo_awget(zh: *mut zhandle_t, path: *const c_char,
                     watcher: watcher_fn, watcherCtx: *mut c_void,
                     completion: data_completion_t,
                     data: *const c_void) -> c_int;
    pub fn zoo_aset(zh: *mut zhandle_t, path: *const c_char,
                    buffer: *const c_char, buflen: c_int,
                    version: c_int, completion: stat_completion_t,
                    data: *const c_void) -> c_int;
    pub fn zoo_aget_children(zh: *mut zhandle_t, path: *const c_char,
                             watch: c_int,
                             completion: strings_completion_t,
                             data: *const c_void) -> c_int;
    pub fn zoo_awget_children(zh: *mut zhandle_t, path: *const c_char,
                              watcher: watcher_fn,
                              watcherCtx: *mut c_void,
                              completion: strings_completion_t,
                              data: *const c_void) -> c_int;
    pub fn zoo_aget_children2(zh: *mut zhandle_t, path: *const c_char,
                              watch: c_int, completion: strings_stat_completion_t,
                              data: *const c_void) -> c_int;
    pub fn zoo_awget_children2(zh: *mut zhandle_t, path: *const c_char,
                               watcher: watcher_fn, watcherCtx: *mut c_void,
                               completion: strings_stat_completion_t,
                               data: *const c_void) -> c_int;
    pub fn zoo_async(zh: *mut zhandle_t, path: *const c_char,
                     completion: string_completion_t,
                     data: *const c_void) -> c_int;
    pub fn zoo_aget_acl(zh: *mut zhandle_t, path: *const c_char,
                        completion: acl_completion_t, data: *const c_void) -> c_int;
    pub fn zoo_aset_acl(zh: *mut zhandle_t, path: *const c_char,
                        version: c_int, acl: *mut ACL_vector,
                        arg1: void_completion_t, data: *const c_void) -> c_int;
    pub fn zoo_amulti(zh: *mut zhandle_t, count: c_int,
                      ops: *const zoo_op, results: *mut zoo_op_result,
                      arg1: void_completion_t, data: *const c_void) -> c_int;
    pub fn zerror(c: c_int) -> *const c_char;
    pub fn zoo_add_auth(zh: *mut zhandle_t, scheme: *const c_char,
                        cert: *const c_char, certLen: c_int,
                        completion: void_completion_t,
                        data: *const c_void) -> c_int;
    pub fn is_unrecoverable(zh: *mut zhandle_t) -> c_int;
    pub fn zoo_set_debug_level(logLevel: ZooLogLevel);
    pub fn zoo_set_log_stream(logStream: *mut FILE);
    pub fn zoo_deterministic_conn_order(yesOrNo: c_int);
    pub fn zoo_create(zh: *mut zhandle_t, path: *const c_char,
                      value: *const c_char, valuelen: c_int,
                      acl: *const ACL_vector, flags: c_int,
                      path_buffer: *mut c_char, path_buffer_len: c_int) -> c_int;
    pub fn zoo_delete(zh: *mut zhandle_t, path: *const c_char,
                      version: c_int) -> c_int;
    pub fn zoo_exists(zh: *mut zhandle_t, path: *const c_char,
                      watch: c_int, stat: *mut Stat) -> c_int;
    pub fn zoo_wexists(zh: *mut zhandle_t, path: *const c_char,
                       watcher: watcher_fn, watcherCtx: *mut c_void,
                       stat: *mut Stat) -> c_int;
    pub fn zoo_get(zh: *mut zhandle_t, path: *const c_char,
                   watch: c_int, buffer: *mut c_char,
                   buffer_len: *mut c_int, stat: *mut Stat) -> c_int;
    pub fn zoo_wget(zh: *mut zhandle_t, path: *const c_char,
                    watcher: watcher_fn, watcherCtx: *mut c_void,
                    buffer: *mut c_char, buffer_len: *mut c_int,
                    stat: *mut Stat) -> c_int;
    pub fn zoo_set(zh: *mut zhandle_t, path: *const c_char,
                   buffer: *const c_char, buflen: c_int,
                   version: c_int) -> c_int;
    pub fn zoo_set2(zh: *mut zhandle_t, path: *const c_char,
                    buffer: *const c_char, buflen: c_int,
                    version: c_int, stat: *mut Stat) -> c_int;
    pub fn zoo_get_children(zh: *mut zhandle_t, path: *const c_char,
                            watch: c_int, strings: *mut String_vector) -> c_int;
    pub fn zoo_wget_children(zh: *mut zhandle_t, path: *const c_char,
                             watcher: watcher_fn, watcherCtx: *mut c_void,
                             strings: *mut String_vector) -> c_int;
    pub fn zoo_get_children2(zh: *mut zhandle_t, path: *const c_char,
                             watch: c_int, strings: *mut String_vector,
                             stat: *mut Stat) -> c_int;
    pub fn zoo_wget_children2(zh: *mut zhandle_t, path: *const c_char,
                              watcher: watcher_fn, watcherCtx: *mut c_void,
                              strings: *mut String_vector, stat: *mut Stat) -> c_int;
    pub fn zoo_get_acl(zh: *mut zhandle_t, path: *const c_char,
                       acl: *mut ACL_vector, stat: *mut Stat) -> c_int;
    pub fn zoo_set_acl(zh: *mut zhandle_t, path: *const c_char,
                       version: c_int, acl: *const ACL_vector) -> c_int;
    pub fn zoo_multi(zh: *mut zhandle_t, count: c_int,
                     ops: *const zoo_op, results: *mut zoo_op_result) -> c_int;
}


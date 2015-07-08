//! Bindings to the zookeeper C client library.

#![allow(non_camel_case_types, raw_pointer_derive, non_snake_case)]
extern crate libc;

mod ffi;

// Functions
pub use ffi::{
    zoo_htonll,
    zoo_create_op_init,
    zoo_delete_op_init,
    zoo_check_op_init,
    zoo_set_op_init,
    zookeeper_init,
    zookeeper_close,
    zoo_client_id,
    zoo_recv_timeout,
    zoo_get_context,
    zoo_set_context,
    zoo_set_watcher,
    zookeeper_get_connected_host,
    zookeeper_interest,
    zookeeper_process,
    zoo_state,
    zoo_acreate,
    zoo_adelete,
    zoo_aexists,
    zoo_awexists,
    zoo_aget,
    zoo_awget,
    zoo_aset,
    zoo_aget_children,
    zoo_awget_children,
    zoo_aget_children2,
    zoo_awget_children2,
    zoo_async,
    zoo_aget_acl,
    zoo_aset_acl,
    zoo_amulti,
    zerror,
    zoo_add_auth,
    zoo_set_debug_level,
    zoo_set_log_stream,
    zoo_deterministic_conn_order,
    zoo_create,
    zoo_delete,
    zoo_exists,
    zoo_wexists,
    zoo_get,
    zoo_wget,
    zoo_set,
    zoo_set2,
    zoo_get_children,
    zoo_wget_children,
    zoo_get_children2,
    zoo_wget_children2,
    zoo_get_acl,
    zoo_set_acl,
    zoo_multi
};

// Type aliases
pub use ffi::{
    watcher_fn,
    void_completion_t,
    stat_completion_t,
    data_completion_t,
    strings_completion_t,
    strings_stat_completion_t,
    string_completion_t,
    acl_completion_t
};

// Types
pub use ffi::{
    zoo_op,
    zoo_op_create,
    zoo_op_delete,
    zoo_op_set,
    zoo_op_check,
    zoo_op_result,
    ACL_vector,
    ACL,
    Id,
    Stat,
    client_id,
    zhandle_t,
    ZooLogLevel,
    ZooError,
    String_vector
};

// Constants
pub use ffi::{
    ZOO_PERM_READ,
    ZOO_PERM_WRITE,
    ZOO_PERM_CREATE,
    ZOO_PERM_DELETE,
    ZOO_PERM_ADMIN,
    ZOO_PERM_ALL,
    ZOO_ANYONE_ID_UNSAFE,
    ZOO_AUTH_IDS,
    ZOO_OPEN_ACL_UNSAFE,
    ZOO_READ_ACL_UNSAFE,
    ZOO_CREATOR_ALL_ACL,
    ZOOKEEPER_WRITE,
    ZOOKEEPER_READ,
    ZOO_EPHEMERAL,
    ZOO_SEQUENCE,
    ZOO_EXPIRED_SESSION_STATE,
    ZOO_AUTH_FAILED_STATE,
    ZOO_CONNECTING_STATE,
    ZOO_ASSOCIATING_STATE,
    ZOO_CONNECTED_STATE,
    ZOO_CREATED_EVENT,
    ZOO_DELETED_EVENT,
    ZOO_CHANGED_EVENT,
    ZOO_CHILD_EVENT,
    ZOO_SESSION_EVENT,
    ZOO_NOTWATCHING_EVENT
};


#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]
#![allow(clippy::all)]

//! Rust wrapper for the `aws-c-io` library. For testing purposes only.
//! For interacting with AWS services, use the `aws-sdk-rust` crate instead.
use {
    libc::FILE,
    scratchstack_wrapper_aws_c_common::{
        aws_allocator, aws_array_list, aws_atomic_var, aws_byte_buf, aws_byte_cursor, aws_crt_statistics_category_t,
        aws_crt_statistics_handler, aws_hash_table, aws_linked_list_node, aws_ref_count, aws_shutdown_callback_options,
        aws_string, aws_task, aws_task_status, aws_thread_options,
    },
};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests;
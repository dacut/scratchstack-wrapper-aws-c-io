use {
    bindgen::builder,
    std::{
        env::var,
        fs::{copy, create_dir_all, read_dir},
        path::{Path, PathBuf},
    },
};

const LINK_LIBS: &str = "
aws-c-io
aws-c-cal
aws-c-common
s2n
crypto
ssl
";
const INCLUDE_PATH: &str = "aws/io";
const DEP_LIBRARIES: &str = r#"aws-c-common
aws-c-cal
aws-lc"#;
const FUNCTIONS: &str = r#"
aws_io_library_init
aws_io_library_clean_up
aws_io_fatal_assert_library_initialized
aws_client_bootstrap_new
aws_client_bootstrap_new
aws_client_bootstrap_release
aws_client_bootstrap_set_alpn_callback
aws_client_bootstrap_new_socket_channel
aws_server_bootstrap_new
aws_server_bootstrap_acquire
aws_server_bootstrap_release
aws_server_bootstrap_set_alpn_callback
aws_server_bootstrap_new_socket_listener
aws_server_bootstrap_destroy_socket_listener
aws_channel_task_init
aws_channel_new
aws_channel_destroy
aws_channel_shutdown
aws_channel_acquire_hold
aws_channel_release_hold
aws_channel_slot_new
aws_channel_get_event_loop
aws_channel_current_clock_time
aws_channel_fetch_local_object
aws_channel_put_local_object
aws_channel_remove_local_object
aws_channel_acquire_message_from_pool
aws_channel_schedule_task_now
aws_channel_schedule_task_now_serialized
aws_channel_schedule_task_future
aws_channel_set_statistics_handler
aws_channel_thread_is_callers_thread
aws_channel_slot_set_handler
aws_channel_slot_remove
aws_channel_slot_replace
aws_channel_slot_insert_right
aws_channel_slot_insert_end
aws_channel_slot_insert_left
aws_channel_slot_send_message
aws_channel_slot_acquire_max_message_for_write
aws_channel_slot_increment_read_window
aws_channel_slot_on_handler_shutdown_complete
aws_channel_slot_shutdown
aws_channel_slot_downstream_read_window
aws_channel_slot_upstream_message_overhead
aws_channel_handler_destroy
aws_channel_handler_process_read_message
aws_channel_handler_process_write_message
aws_channel_handler_increment_read_window
aws_channel_handler_shutdown
aws_channel_handler_initial_window_size
aws_channel_get_first_slot
aws_channel_trigger_read
aws_overlapped_init
aws_overlapped_reset
aws_overlapped_to_windows_overlapped
aws_event_loop_new_default
aws_event_loop_new_default_with_options
aws_event_loop_destroy
aws_event_loop_init_base
aws_event_loop_clean_up_base
aws_event_loop_fetch_local_object
aws_event_loop_put_local_object
aws_event_loop_remove_local_object
aws_event_loop_run
aws_event_loop_stop
aws_event_loop_register_tick_start
aws_event_loop_register_tick_end
aws_event_loop_get_load_factor
aws_event_loop_wait_for_stop_completion
aws_event_loop_schedule_task_now
aws_event_loop_schedule_task_future
aws_event_loop_cancel_task
aws_event_loop_connect_handle_to_io_completion_port
aws_event_loop_subscribe_to_io_events
aws_event_loop_unsubscribe_from_io_events
aws_event_loop_free_io_event_resources
aws_event_loop_thread_is_callers_thread
aws_event_loop_current_clock_time
aws_event_loop_group_new
aws_event_loop_group_new_pinned_to_cpu_group
aws_event_loop_group_new_default
aws_event_loop_group_new_default_pinned_to_cpu_group
aws_event_loop_group_acquire
aws_event_loop_group_release
aws_event_loop_group_get_loop_at
aws_event_loop_group_get_loop_count
aws_event_loop_group_get_next_loop
aws_host_address_copy
aws_host_address_move
aws_host_address_clean_up
aws_default_dns_resolve
aws_host_resolver_new_default
aws_host_resolver_acquire
aws_host_resolver_release
aws_host_resolver_resolve_host
aws_host_resolver_record_connection_failure
aws_host_resolver_purge_cache
aws_host_resolver_get_host_address_count
aws_host_resolver_add_host_listener
aws_host_resolver_remove_host_listener
aws_memory_pool_init
aws_memory_pool_clean_up
aws_memory_pool_acquire
aws_memory_pool_release
aws_message_pool_init
aws_message_pool_clean_up
aws_message_pool_acquire
aws_message_pool_release
aws_pipe_init
aws_pipe_clean_up_read_end
aws_pipe_clean_up_write_end
aws_pipe_get_read_end_event_loop
aws_pipe_get_write_end_event_loop
aws_pipe_write
aws_pipe_read
aws_pipe_subscribe_to_readable_events
aws_pipe_unsubscribe_from_readable_events
aws_pipe_get_unique_name
aws_pkcs11_lib_new
aws_pkcs11_lib_acquire
aws_pkcs11_lib_release
aws_retry_strategy_acquire
aws_retry_strategy_release
aws_retry_strategy_acquire_retry_token
aws_retry_strategy_schedule_retry
aws_retry_token_record_success
aws_retry_token_acquire
aws_retry_token_release
aws_retry_strategy_new_exponential_backoff
aws_retry_strategy_new_standard
aws_shared_library_init
aws_shared_library_clean_up
aws_shared_library_find_function
aws_socket_handler_new
aws_check_and_init_winsock
aws_winsock_get_connectex_fn
aws_winsock_get_acceptex_fn
aws_socket_init
aws_socket_clean_up
aws_socket_connect
aws_socket_bind
aws_socket_get_bound_address
aws_socket_listen
aws_socket_start_accept
aws_socket_stop_accept
aws_socket_close
aws_socket_shutdown_dir
aws_socket_set_options
aws_socket_assign_to_event_loop
aws_socket_get_event_loop
aws_socket_subscribe_to_readable_events
aws_socket_read
aws_socket_write
aws_socket_get_error
aws_socket_is_open
aws_crt_statistics_socket_init
aws_crt_statistics_socket_cleanup
aws_crt_statistics_socket_reset
aws_crt_statistics_tls_init
aws_crt_statistics_tls_cleanup
aws_crt_statistics_tls_reset
aws_input_stream_acquire
aws_input_stream_release
aws_input_stream_seek
aws_input_stream_read
aws_input_stream_get_status
aws_input_stream_get_length
aws_input_stream_destroy
aws_input_stream_new_from_cursor
aws_input_stream_new_from_file
aws_input_stream_new_from_open_file
aws_tls_ctx_options_init_default_client
aws_tls_ctx_options_clean_up
aws_tls_ctx_options_init_client_mtls_from_path
aws_tls_ctx_options_init_client_mtls
aws_custom_key_op_handler_acquire
aws_custom_key_op_handler_release
aws_custom_key_op_handler_perform_operation
aws_tls_ctx_options_init_client_mtls_with_custom_key_operations
aws_tls_ctx_options_init_client_mtls_with_pkcs11
aws_tls_ctx_options_set_keychain_path
aws_tls_ctx_options_init_default_server_from_path
aws_tls_ctx_options_init_default_server
aws_tls_ctx_options_init_client_mtls_from_system_path
aws_tls_ctx_options_init_default_server_from_system_path
aws_tls_ctx_options_init_client_mtls_pkcs12_from_path
aws_tls_ctx_options_init_client_mtls_pkcs12
aws_tls_ctx_options_init_server_pkcs12_from_path
aws_tls_ctx_options_init_server_pkcs12
aws_tls_ctx_options_set_alpn_list
aws_tls_ctx_options_set_verify_peer
aws_tls_ctx_options_set_minimum_tls_version
aws_tls_ctx_options_override_default_trust_store
aws_tls_ctx_options_override_default_trust_store_from_path
aws_tls_ctx_options_set_extension_data
aws_tls_connection_options_init_from_ctx
aws_tls_connection_options_clean_up
aws_tls_connection_options_copy
aws_tls_connection_options_set_callbacks
aws_tls_connection_options_set_server_name
aws_tls_connection_options_set_alpn_list
aws_tls_is_alpn_available
aws_tls_is_cipher_pref_supported
aws_tls_client_handler_new
aws_tls_server_handler_new
aws_tls_byo_crypto_set_client_setup_options
aws_tls_byo_crypto_set_server_setup_options
aws_tls_alpn_handler_new
aws_tls_client_handler_start_negotiation
aws_tls_server_ctx_new
aws_tls_client_ctx_new
aws_tls_ctx_acquire
aws_tls_ctx_release
aws_tls_handler_write
aws_tls_handler_protocol
aws_tls_handler_server_name
aws_tls_key_operation_complete
aws_tls_key_operation_complete_with_error
aws_tls_key_operation_get_input
aws_tls_key_operation_get_type
aws_tls_key_operation_get_signature_algorithm
aws_tls_key_operation_get_digest_algorithm
aws_channel_setup_client_tls
aws_tls_hash_algorithm_str
aws_tls_signature_algorithm_str
aws_tls_key_operation_type_str
"#;
const TYPES: &str = r#"
aws_io_handle
aws_io_message_type
aws_channel_on_message_write_completed_fn
aws_io_message
aws_io_clock_fn
aws_io_errors
aws_socket
aws_socket_options
aws_socket_endpoint
aws_channel_on_protocol_negotiated_fn
aws_client_bootstrap_on_channel_event_fn
aws_tls_connection_options
aws_event_loop_group
aws_client_bootstrap_shutdown_complete_fn
aws_client_bootstrap
aws_client_bootstrap_options
aws_server_bootstrap_on_accept_channel_setup_fn
aws_server_bootstrap_on_accept_channel_shutdown_fn
aws_server_bootstrap_on_server_listener_destroy_fn
aws_server_bootstrap
aws_socket_channel_bootstrap_options
aws_server_socket_channel_bootstrap_options
aws_channel_direction
aws_channel_on_setup_completed_fn
aws_channel_on_shutdown_completed_fn
aws_channel_slot
aws_channel_task_fn
aws_channel_task
aws_channel_handler_vtable
aws_channel_handler
aws_channel_options
aws_channel
aws_io_event_type
aws_event_loop_on_completion_fn
aws_win32_OVERLAPPED
aws_overlapped
aws_event_loop_on_event_fn
aws_event_loop_vtable
aws_event_loop
aws_event_loop_on_local_object_removed_fn
aws_event_loop_local_object
aws_event_loop_options
aws_new_event_loop_fn
aws_event_loop_group
aws_address_record_type
aws_get_host_address_flags
aws_host_address
aws_on_host_resolved_result_fn
aws_resolve_host_implementation_fn
aws_host_resolution_config
aws_host_listener
aws_host_listener_options
aws_host_resolver_vtable
aws_host_resolver
aws_host_resolver_default_options
aws_host_listener_resolved_address_fn
aws_host_listener_expired_address_fn
aws_host_listener_shutdown_fn
aws_host_listener_options
aws_io_log_subject
aws_memory_pool
aws_message_pool
aws_message_pool_creation_args
aws_pipe_read_end
aws_pipe_write_end
aws_pipe_on_readable_fn
aws_pipe_on_write_completed_fn
aws_pkcs11_lib
aws_pkcs11_lib_behavior
aws_pkcs11_lib_options
aws_retry_strategy_on_retry_token_acquired_fn
aws_retry_strategy_on_retry_ready_fn
aws_retry_error_type
aws_retry_strategy_vtable
aws_retry_strategy
aws_retry_token
aws_exponential_backoff_jitter_mode
aws_exponential_backoff_retry_options
aws_standard_retry_options
aws_shared_library
aws_generic_function
aws_socket_domain
aws_socket_type
aws_socket_options
aws_socket_on_connection_result_fn
aws_socket_on_accept_result_fn
aws_socket_on_write_completed_fn
aws_socket_on_write_completed_fn
aws_socket_on_readable_fn
aws_socket
aws_ms_fn_ptr
aws_crt_io_statistics_category
aws_crt_statistics_socket
aws_crt_statistics_tls
aws_stream_seek_basis
aws_stream_status
aws_input_stream_vtable
aws_input_stream
aws_tls_versions
aws_tls_cipher_pref
aws_tls_hash_algorithm
aws_tls_signature_algorithm
aws_tls_key_operation_type
aws_tls_ctx
aws_tls_on_negotiation_result_fn
aws_tls_on_data_read_fn
aws_tls_on_error_fn
aws_tls_connection_options
aws_tls_key_operation
aws_tls_ctx_options
aws_tls_negotiated_protocol_message
aws_tls_on_protocol_negotiated
aws_channel_handler
aws_tls_negotiation_status
aws_tls_handler_new_fn
aws_tls_client_handler_start_negotiation_fn
aws_tls_byo_crypto_setup_options
aws_custom_key_op_handler_vtable
aws_custom_key_op_handler
aws_tls_ctx_pkcs11_options
"#;

const VARS: &str = "
g_aws_channel_max_fragment_size
";

fn get_include_dir<P: AsRef<Path>>(dir: P) -> PathBuf {
    let mut result = PathBuf::from(dir.as_ref());

    for folder in INCLUDE_PATH.split('/') {
        result.push(folder);
    }

    result
}

fn main() {
    let root = PathBuf::from(var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set"));
    let out_dir = PathBuf::from(var("OUT_DIR").expect("OUT_DIR not set"));

    let src_include_dir = root.join("include");
    let dst_include_dir = out_dir.join("include");
    let src_lib_include_dir = get_include_dir(&src_include_dir);
    let dst_lib_include_dir = get_include_dir(&dst_include_dir);
    let src_include_dir_str = src_include_dir.to_string_lossy();
    let dst_include_dir_str = dst_include_dir.to_string_lossy();
    let src_lib_include_dir_str = src_lib_include_dir.to_string_lossy();
    let dst_lib_include_dir_str = dst_lib_include_dir.to_string_lossy();

    println!("cargo:include={dst_include_dir_str}");
    println!("cargo:rerun-if-changed=include");
    println!("cargo:rerun-if-env-changed=AWS_CRT_PREFIX");

    if let Ok(aws_crt_prefix) = var("AWS_CRT_PREFIX") {
        println!("cargo:rustc-link-search={aws_crt_prefix}/lib");
    }

    for library_name in LINK_LIBS.split('\n') {
        let library_name = library_name.trim();
        if !library_name.is_empty() {
            println!("cargo:rustc-link-lib={library_name}");
        }
    }

    // Copy include files over
    create_dir_all(&dst_lib_include_dir)
        .unwrap_or_else(|e| panic!("Unable to create directory {dst_lib_include_dir_str}: {e}"));

    let mut builder = builder()
        .clang_arg(format!("-I{src_include_dir_str}"))
        .derive_debug(true)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .allowlist_recursively(false); // Don't dive into dependent libraries.
    
    for dep in DEP_LIBRARIES.split('\n') {
        let dep = dep.trim();
        if dep.is_empty() {
            continue;
        }

        let dep = String::from(dep).replace('-', "_").to_uppercase();
        let dep_include_dir = PathBuf::from(var(format!("DEP_{dep}_INCLUDE")).unwrap_or_else(|_| panic!("DEP_{dep}_INCLUDE not set")));
        let dep_include_dir_str = dep_include_dir.to_string_lossy();
        builder = builder.clang_arg(format!("-I{dep_include_dir_str}"));
    }

    let mut n_includes = 0;

    for entry in read_dir(&src_lib_include_dir)
        .unwrap_or_else(|e| panic!("Unable to list header files in {src_lib_include_dir_str}: {e}"))
    {
        let entry =
            entry.unwrap_or_else(|e| panic!("Unable to read directory entry in {src_lib_include_dir_str}: {e}"));
        let file_name_string = entry.file_name();
        let src_path = src_lib_include_dir.join(&file_name_string);
        let src_path_str = src_path.to_string_lossy();
        let dst_path = dst_lib_include_dir.join(&file_name_string);

        if entry.file_type().unwrap_or_else(|e| panic!("Unable to read file type of {src_path_str}: {e}")).is_file() {
            // Only include header files ending with .h; ignore .inl.
            let file_name_utf8 = file_name_string.to_str().expect("Unable to convert file name to UTF-8");
            if file_name_utf8.ends_with(".h") {
                builder = builder.header(src_path_str.to_string());
                n_includes += 1;
            }

            // Copy all files to the output directory.
            copy(&src_path, &dst_path).unwrap_or_else(|e| {
                panic!(
                    "Failed to copy from {src_path_str} to {dst_path_str}: {e}",
                    dst_path_str = dst_path.to_string_lossy()
                )
            });
        }
    }

    if n_includes == 0 {
        panic!("No header files found in {src_lib_include_dir_str}");
    }

    for function_pattern in FUNCTIONS.split('\n') {
        let function_pattern = function_pattern.trim();
        if !function_pattern.is_empty() {
            builder = builder.allowlist_function(function_pattern);
        }
    }

    for type_pattern in TYPES.split('\n') {
        let type_pattern = type_pattern.trim();
        if !type_pattern.is_empty() {
            builder = builder.allowlist_type(type_pattern);
        }
    }

    for var_pattern in VARS.split('\n') {
        let var_pattern = var_pattern.trim();
        if !var_pattern.is_empty() {
            builder = builder.allowlist_var(var_pattern);
        }
    }

    let bindings_filename = out_dir.join("bindings.rs");
    let bindings = builder.generate().expect("Unable to generate bindings");
    bindings.write_to_file(&bindings_filename).unwrap_or_else(|e| {
        panic!(
            "Failed to write bindings to {bindings_filename_str}: {e}",
            bindings_filename_str = bindings_filename.to_string_lossy()
        )
    });

    if cfg!(any(target_os = "ios", target_os = "macos")) {
        println!("cargo:rustc-link-arg=-framework");
        println!("cargo:rustc-link-arg=CoreFoundation");
    }
}

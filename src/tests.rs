use crate::{aws_io_library_clean_up, aws_io_library_init};
use scratchstack_wrapper_aws_c_common::aws_default_allocator;

#[test]
fn test_init_uninit() {
    unsafe {
        aws_io_library_init(aws_default_allocator());
        aws_io_library_clean_up();
    }
}
use libc::{c_char, printf};

fn learning_libc() {
    unsafe {
        // 注意这里需要使用字节数组来表示字符串，结尾添加\0
        let message = b"Hello, world!\n\0";
        // 使用unsafe块来调用C函数
        printf(message.as_ptr() as *const i8);

        // 使用 libc 获取 hostname
        let mut name = [0u8; 256];
        let len = name.len();

        let ptr = name.as_mut_ptr() as *mut c_char;
        let result = libc::gethostname(ptr, len);
        printf(b"gethostname result: %d\n\0".as_ptr() as *const i8, result);
        printf(b"hostname: %s\n\0".as_ptr() as *const i8, ptr);

        let version = libc::gnu_get_libc_version();
        printf(b"GNU libc version: %s\n\0".as_ptr() as *const i8, version);
    }
}

#[test]
fn test_learning_libc() {
    learning_libc();
}

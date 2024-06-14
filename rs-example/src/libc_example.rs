use libc::{c_char, c_void, printf};

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

const EV_KEY: u16 = 0x01; // 键盘事件
const EV_SYN: u16 = 0x00; // 同步事件

const KEY_A: u16 = 30; // A 键
const BTN_LEFT: u16 = 272; // 鼠标左键

pub fn listen_loop() {
    // 监听循环
    unsafe {
        // 打开输入设备文件（在Linux中通常是 /dev/input/eventX）
        let fd = libc::open("/dev/input/event0\0".as_ptr() as *const i8, libc::O_RDONLY);
        if fd < 0 {
            panic!("Failed to open input device");
        }

        loop {
            let mut event: [u8; 24] = [0; 24]; // 事件数据结构的大小为24字节
            libc::read(fd, event.as_mut_ptr() as *mut c_void, core::mem::size_of_val(&event));

            let event_type = event[16] as u16 + ((event[17] as u16) << 8);
            let event_code = event[18] as u16 + ((event[19] as u16) << 8);
            let event_value =
                event[20] as i32 + ((event[21] as i32) << 8) + ((event[22] as i32) << 16) + ((event[23] as i32) << 24);

            if event_type == EV_SYN {
                // 同步事件，忽略
            } else if event_type == EV_KEY {
                // 键盘事件
                if event_value == 0 || event_value == 1 {
                    println!("Key event - Code: {}, Value: {}", event_code, event_value);
                }
            }
        }
    }
}

#[test]
fn test_learning_libc() {
    learning_libc();
}

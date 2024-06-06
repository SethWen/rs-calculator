use std::sync::{Arc, Barrier};
use std::thread;

fn learning_barrier() {
    let barrier = Arc::new(Barrier::new(3)); // 创建一个Barrier，需要等待3个线程

    let mut handles = vec![];
    for i in 0..3 {
        let c = barrier.clone();
        let handle = thread::spawn(move || {
            println!("Thread {:?} before the barrier", thread::current().id());
            c.wait(); // 等待所有线程都执行到这里
            println!("Thread {:?} after the barrier", thread::current().id());
            return i;
        });
        handles.push(handle);
    }

    for handle in handles {
        let result = handle.join().unwrap(); // 等待所有线程结束
        println!("result: {}", result);
    }
    println!("All threads have finished");
}

#[test]
fn test_learning_barrier() {
    learning_barrier();
}

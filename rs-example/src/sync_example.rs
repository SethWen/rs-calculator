use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

fn learning_barrier() {
    let barrier = Arc::new(Barrier::new(3)); // 创建一个Barrier，需要等待3个线程

    let mut handles = vec![];
    for i in 0..3 {
        let c = barrier.clone();
        let handle = thread::spawn(move || {
            println!("Thread {:?} before the barrier", thread::current().id());
            thread::sleep(Duration::from_millis(5000));
            c.wait(); // 等待所有线程都执行到这里
            println!("Thread {:?} after the barrier", thread::current().id());
            return i;
        });
        handles.push(handle);
    }

    println!("waiting...");
    for handle in handles {
        let result = handle.join().unwrap(); // 等待所有线程结束
        println!("result: {}", result);
    }
    println!("All threads have finished");
}

fn learning_coroutine() {
    let mut gen = #[coroutine]
    move |x| {
        println!("Hello");
        yield x * 2;
        println!("world!");
    };

    while let CoroutineState::Yielded(n) = Pin::new(&mut gen).resume(3) {
        println!("Got value {}", n);
        if n > 20 {
            break;
        }
    }

    if let CoroutineState::Complete(n) = Pin::new(&mut gen).resume(5) {
        println!("Complete ~");
    }
}

pub fn check_thread() {
    let thread = thread::spawn(|| {
        println!("Thread started!");
        // 阻塞线程直到收到通知
        thread::park();

        println!("Thread unparked!");
    });

    // 等待一段时间后唤醒线程
    thread::sleep(Duration::from_secs(6));
    thread.thread().unpark();

    thread.join().unwrap();
}

#[test]
fn test_learning_barrier() {
    learning_barrier();
}

#[test]
fn test_learning_coroutine() {
    learning_coroutine();
}

#[test]
fn test_check_thread() {
    check_thread();
}

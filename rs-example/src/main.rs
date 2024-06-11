#![feature(coroutines, coroutine_trait)]
#![feature(stmt_expr_attributes)]

mod fatptr_example;
mod libc_example;
mod pin_example;
mod sync_example;

fn main() {
    println!("Hello, world!");

    sync_example::check_thread();
}

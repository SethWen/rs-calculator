use std::pin::Pin;

fn learning_pin() {
    let data = Box::pin(5);
    let mut value = 8;
    let ptr = Pin::new(&mut value);
}
